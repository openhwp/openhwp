//! High-level HWP document API.
//!
//! This module provides the main `HwpDocument` type for reading HWP files.

use std::collections::HashMap;
use std::io::{Read, Seek};

use cfb::CompoundFile;

use crate::body::{Picture, Section};
use crate::crypto::{decrypt_distribution_stream, decrypt_password_stream};
use crate::doc_info::DocInfo;
use crate::doc_options::{DocOptions, DrmLicense, LinkDoc};
use crate::error::{Error, Result};
use crate::header::FileHeader;
use crate::preview::{PreviewImage, PreviewText};
use crate::script::{ScriptHeader, ScriptSource, ScriptVersion, Scripts};
use crate::summary::SummaryInfo;
use crate::util::decompress_stream;
use primitive::Version;

/// An HWP 5.0 document.
///
/// This is the main entry point for reading HWP files.
///
/// # Example
///
/// ```ignore
/// use hwp::HwpDocument;
///
/// let bytes = std::fs::read("document.hwp")?;
/// let doc = HwpDocument::from_bytes(&bytes)?;
///
/// // Get document information
/// println!("Version: {}", doc.header().version());
///
/// // Extract text
/// let text = doc.extract_text();
/// println!("{}", text);
/// ```
#[derive(Debug)]
pub struct HwpDocument {
    /// File header.
    header: FileHeader,
    /// Document information.
    doc_info: DocInfo,
    /// Body text sections.
    sections: Vec<Section>,
    /// Binary data (images, OLE objects, etc.) - keyed by BinData ID.
    binary_data: HashMap<u16, Vec<u8>>,
    /// Preview text (from PrvText stream).
    preview_text: Option<PreviewText>,
    /// Preview image (from PrvImage stream).
    preview_image: Option<PreviewImage>,
    /// Summary information (from \005HwpSummaryInformation stream).
    summary_info: Option<SummaryInfo>,
    /// Scripts (from Scripts storage).
    scripts: Option<Scripts>,
    /// Document options (from DocOptions storage).
    doc_options: Option<DocOptions>,
}

impl HwpDocument {
    /// Parses an HWP document from bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - The raw bytes of the HWP file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file is not a valid HWP 5.0 document
    /// - The document is encrypted (use `from_bytes_with_password` instead)
    /// - Any parsing error occurs
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        Self::from_bytes_internal(data, None)
    }

    /// Parses an encrypted HWP document with a password.
    ///
    /// # Arguments
    ///
    /// * `data` - The raw bytes of the HWP file
    /// * `password` - The document password
    pub fn from_bytes_with_password(data: &[u8], password: &str) -> Result<Self> {
        Self::from_bytes_internal(data, Some(password))
    }

    fn from_bytes_internal(data: &[u8], password: Option<&str>) -> Result<Self> {
        // Open as compound file
        let cursor = std::io::Cursor::new(data);
        let mut cfb = CompoundFile::open(cursor)?;

        // Read and parse file header
        let header = Self::read_file_header(&mut cfb)?;

        // Check for encryption
        if header.properties().is_encrypted() && password.is_none() {
            return Err(Error::EncryptedDocument);
        }

        // Check for distribution document
        let distribution_data = if header.is_distribution() {
            Some(Self::read_distribution_data(&mut cfb)?)
        } else {
            None
        };

        // Read and parse DocInfo
        let doc_info =
            Self::read_doc_info(&mut cfb, &header, password, distribution_data.as_deref())?;

        // Read sections
        let section_count = doc_info
            .document_properties
            .as_ref()
            .map(|p| p.section_count() as usize)
            .unwrap_or(1);

        let mut sections = Vec::with_capacity(section_count);
        for i in 0..section_count {
            match Self::read_section(&mut cfb, &header, i, password, distribution_data.as_deref()) {
                Ok(section) => sections.push(section),
                // Some documents may have fewer sections than declared
                Err(_) => break,
            }
        }

        // Read binary data (images, etc.)
        let binary_data = Self::read_binary_data(&mut cfb, &doc_info, &header)?;

        // Read preview text (optional)
        let preview_text = Self::read_preview_text(&mut cfb).ok();

        // Read preview image (optional)
        let preview_image = Self::read_preview_image(&mut cfb).ok();

        // Read summary information (optional)
        let summary_info = Self::read_summary_info(&mut cfb).ok();

        // Read scripts (optional)
        let scripts = Self::read_scripts(&mut cfb).ok();

        // Read document options (optional)
        let doc_options = Self::read_doc_options(&mut cfb).ok();

        Ok(Self {
            header,
            doc_info,
            sections,
            binary_data,
            preview_text,
            preview_image,
            summary_info,
            scripts,
            doc_options,
        })
    }

    fn read_file_header<R: Read + Seek>(cfb: &mut CompoundFile<R>) -> Result<FileHeader> {
        let mut stream = cfb.open_stream("/FileHeader")?;
        let mut data = vec![0u8; FileHeader::SIZE];
        stream.read_exact(&mut data)?;
        FileHeader::from_bytes(&data)
    }

    fn read_distribution_data<R: Read + Seek>(cfb: &mut CompoundFile<R>) -> Result<Vec<u8>> {
        // Distribution data is stored in DocInfo stream as HWPTAG_DISTRIBUTE_DOC_DATA
        // For now, we'll read it from DocInfo and extract it
        // This is a simplified implementation
        let mut stream = cfb.open_stream("/DocInfo")?;
        let mut data = Vec::new();
        stream.read_to_end(&mut data)?;
        Ok(data)
    }

    fn read_doc_info<R: Read + Seek>(
        cfb: &mut CompoundFile<R>,
        header: &FileHeader,
        password: Option<&str>,
        distribution_data: Option<&[u8]>,
    ) -> Result<DocInfo> {
        let mut stream = cfb.open_stream("/DocInfo")?;
        let mut data = Vec::new();
        stream.read_to_end(&mut data)?;

        // Decompress if needed
        let decompressed = if header.properties().is_compressed() {
            decompress_stream(&data)?
        } else {
            data
        };

        // Decrypt if encrypted
        let decrypted = if header.is_encrypted() {
            if let Some(pwd) = password {
                let version = header.encryption_version();
                decrypt_password_stream(&decompressed, version, pwd)?
            } else {
                return Err(Error::EncryptedDocument);
            }
        } else if let Some(dist_data) = distribution_data {
            // Distribution document decryption
            decrypt_distribution_stream(&decompressed, dist_data)?
        } else {
            decompressed
        };

        DocInfo::from_bytes(&decrypted)
    }

    fn read_section<R: Read + Seek>(
        cfb: &mut CompoundFile<R>,
        header: &FileHeader,
        index: usize,
        password: Option<&str>,
        distribution_data: Option<&[u8]>,
    ) -> Result<Section> {
        let stream_name = if header.is_distribution() {
            format!("/ViewText/Section{}", index)
        } else {
            format!("/BodyText/Section{}", index)
        };

        let mut stream = cfb
            .open_stream(&stream_name)
            .map_err(|_| Error::MissingStream {
                name: stream_name.clone(),
            })?;
        let mut data = Vec::new();
        stream.read_to_end(&mut data)?;

        // Decompress if needed
        let decompressed = if header.properties().is_compressed() {
            decompress_stream(&data)?
        } else {
            data
        };

        // Decrypt if encrypted
        let decrypted = if header.is_encrypted() {
            if let Some(pwd) = password {
                let version = header.encryption_version();
                decrypt_password_stream(&decompressed, version, pwd)?
            } else {
                return Err(Error::EncryptedDocument);
            }
        } else if let Some(dist_data) = distribution_data {
            decrypt_distribution_stream(&decompressed, dist_data)?
        } else {
            decompressed
        };

        Section::from_bytes(&decrypted)
    }

    /// Reads preview text from PrvText stream.
    fn read_preview_text<R: Read + Seek>(cfb: &mut CompoundFile<R>) -> Result<PreviewText> {
        let mut stream = cfb.open_stream("/PrvText")?;
        let mut data = Vec::new();
        stream.read_to_end(&mut data)?;
        PreviewText::from_bytes(&data)
    }

    /// Reads preview image from PrvImage stream.
    fn read_preview_image<R: Read + Seek>(cfb: &mut CompoundFile<R>) -> Result<PreviewImage> {
        let mut stream = cfb.open_stream("/PrvImage")?;
        let mut data = Vec::new();
        stream.read_to_end(&mut data)?;
        Ok(PreviewImage::from_bytes(data))
    }

    /// Reads summary information from \005HwpSummaryInformation stream.
    fn read_summary_info<R: Read + Seek>(cfb: &mut CompoundFile<R>) -> Result<SummaryInfo> {
        // OLE property stream name starts with \005
        let mut stream = cfb.open_stream("/\x05HwpSummaryInformation")?;
        let mut data = Vec::new();
        stream.read_to_end(&mut data)?;
        SummaryInfo::from_bytes(&data)
    }

    /// Reads scripts from Scripts storage.
    fn read_scripts<R: Read + Seek>(cfb: &mut CompoundFile<R>) -> Result<Scripts> {
        let mut scripts = Scripts::default();

        // Read JScriptVersion
        if let Ok(mut stream) = cfb.open_stream("/Scripts/JScriptVersion") {
            let mut data = Vec::new();
            stream.read_to_end(&mut data)?;
            scripts.version = ScriptVersion::from_bytes(&data)?;
        }

        // Read DefaultJScript
        if let Ok(mut stream) = cfb.open_stream("/Scripts/DefaultJScript") {
            let mut data = Vec::new();
            stream.read_to_end(&mut data)?;
            scripts.header = ScriptHeader::from_bytes(&data)?;
        }

        // Read script sources (JScript0, JScript1, etc.)
        let max_scripts = scripts.header.script_count.min(100);
        for i in 0..max_scripts {
            let stream_name = format!("/Scripts/JScript{}", i);
            let Ok(mut stream) = cfb.open_stream(&stream_name) else {
                continue;
            };

            let mut data = Vec::new();
            if stream.read_to_end(&mut data).is_ok() && !data.is_empty() {
                if let Ok(source) = ScriptSource::from_bytes(&format!("JScript{}", i), &data) {
                    scripts.sources.push(source);
                }
            }
        }

        Ok(scripts)
    }

    /// Reads document options from DocOptions storage.
    fn read_doc_options<R: Read + Seek>(cfb: &mut CompoundFile<R>) -> Result<DocOptions> {
        let mut doc_options = DocOptions::new();

        // Read _LinkDoc
        if let Ok(mut stream) = cfb.open_stream("/DocOptions/_LinkDoc") {
            let mut data = Vec::new();
            stream.read_to_end(&mut data)?;
            doc_options.set_link_doc(LinkDoc::from_bytes(&data)?);
        }

        // Read DrmLicense
        if let Ok(mut stream) = cfb.open_stream("/DocOptions/DrmLicense") {
            let mut data = Vec::new();
            stream.read_to_end(&mut data)?;
            doc_options.set_drm_license(DrmLicense::from_bytes(&data)?);
        }

        // Read DrmRootSect
        if let Ok(mut stream) = cfb.open_stream("/DocOptions/DrmRootSect") {
            let mut data = Vec::new();
            stream.read_to_end(&mut data)?;
            doc_options.set_drm_root_sect(data);
        }

        // Read CertDrmHeader
        if let Ok(mut stream) = cfb.open_stream("/DocOptions/CertDrmHeader") {
            let mut data = Vec::new();
            stream.read_to_end(&mut data)?;
            doc_options.set_cert_drm_header(data);
        }

        Ok(doc_options)
    }

    /// Reads binary data from the BinData storage.
    fn read_binary_data<R: Read + Seek>(
        cfb: &mut CompoundFile<R>,
        doc_info: &DocInfo,
        header: &FileHeader,
    ) -> Result<HashMap<u16, Vec<u8>>> {
        let mut result = HashMap::new();

        // Get binary data info from doc_info
        let binary_items = &doc_info.binary_data;

        for (index, bin_info) in binary_items.iter().enumerate() {
            // BinData IDs are 1-based in the storage
            let bin_id = (index + 1) as u16;

            // Get the stream name from BinaryData
            if let Some(stream_name) = bin_info.stream_name() {
                let full_path = format!("/BinData/{}", stream_name);

                // Try to read the stream
                if let Ok(mut stream) = cfb.open_stream(&full_path) {
                    let mut data = Vec::new();
                    stream.read_to_end(&mut data)?;
                    if !data.is_empty() {
                        // Decompress if needed (BinData follows storage compression setting)
                        let final_data = if header.properties().is_compressed() {
                            // Try decompression, fall back to raw data if it fails
                            decompress_stream(&data).unwrap_or(data)
                        } else {
                            data
                        };
                        result.insert(bin_id, final_data);
                    }
                }
            }
        }

        Ok(result)
    }

    /// Returns the file header.
    pub const fn header(&self) -> &FileHeader {
        &self.header
    }

    /// Returns the document information.
    pub const fn doc_info(&self) -> &DocInfo {
        &self.doc_info
    }

    /// Returns the sections.
    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    /// Returns the HWP version.
    pub const fn version(&self) -> Version {
        self.header.version()
    }

    /// Returns true if this is a distribution document.
    pub const fn is_distribution_document(&self) -> bool {
        self.header.is_distribution()
    }

    /// Returns true if this is an encrypted document.
    pub const fn is_encrypted(&self) -> bool {
        self.header.is_encrypted()
    }

    /// Extracts all plain text from the document.
    ///
    /// Returns all text content with paragraphs separated by newlines.
    pub fn extract_text(&self) -> String {
        self.sections
            .iter()
            .map(|s| s.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Returns the number of sections.
    pub const fn section_count(&self) -> usize {
        self.sections.len()
    }

    /// Returns the total number of paragraphs across all sections.
    pub fn paragraph_count(&self) -> usize {
        self.sections.iter().map(|s| s.paragraph_count()).sum()
    }

    // === Binary Data API ===

    /// Returns binary data by its ID.
    ///
    /// Binary data IDs are 1-based and correspond to items in the DocInfo BinaryData list.
    /// Returns the raw binary content (e.g., image bytes) if available.
    pub fn get_binary_data(&self, id: u16) -> Option<&[u8]> {
        self.binary_data.get(&id).map(|v| v.as_slice())
    }

    /// Returns all binary data IDs available in this document.
    pub fn binary_data_ids(&self) -> Vec<u16> {
        self.binary_data.keys().copied().collect()
    }

    /// Returns the number of binary data items.
    pub fn binary_data_count(&self) -> usize {
        self.binary_data.len()
    }

    /// Returns the image data for a picture control.
    ///
    /// This is a convenience method that looks up the binary data
    /// using the picture's binary_data_id.
    pub fn get_picture_data(&self, picture: &Picture) -> Option<&[u8]> {
        self.get_binary_data(picture.binary_data_id())
    }

    // === Preview API ===

    /// Returns the preview text if available.
    ///
    /// The preview text is a plain text representation of the document
    /// stored in the PrvText stream.
    pub fn preview_text(&self) -> Option<&PreviewText> {
        self.preview_text.as_ref()
    }

    /// Returns the preview image if available.
    ///
    /// The preview image is a thumbnail of the document
    /// stored in the PrvImage stream (PNG, GIF, or BMP format).
    pub fn preview_image(&self) -> Option<&PreviewImage> {
        self.preview_image.as_ref()
    }

    // === Summary Information API ===

    /// Returns the summary information if available.
    ///
    /// Contains metadata like title, subject, author, keywords, etc.
    pub fn summary_info(&self) -> Option<&SummaryInfo> {
        self.summary_info.as_ref()
    }

    /// Returns the document title from summary info.
    pub fn title(&self) -> Option<&str> {
        self.summary_info.as_ref().and_then(|s| s.title.as_deref())
    }

    /// Returns the document author from summary info.
    pub fn author(&self) -> Option<&str> {
        self.summary_info.as_ref().and_then(|s| s.author.as_deref())
    }

    /// Returns the document subject from summary info.
    pub fn subject(&self) -> Option<&str> {
        self.summary_info
            .as_ref()
            .and_then(|s| s.subject.as_deref())
    }

    /// Returns the document keywords from summary info.
    pub fn keywords(&self) -> Option<&str> {
        self.summary_info
            .as_ref()
            .and_then(|s| s.keywords.as_deref())
    }

    // === Scripts API ===

    /// Returns the scripts if available.
    ///
    /// HWP documents can contain JavaScript code for forms and automation.
    pub fn scripts(&self) -> Option<&Scripts> {
        self.scripts.as_ref()
    }

    /// Returns true if the document contains scripts.
    pub fn has_scripts(&self) -> bool {
        self.scripts
            .as_ref()
            .map(|s| !s.is_empty())
            .unwrap_or(false)
    }

    // === Document Options API ===

    /// Returns the document options if available.
    ///
    /// Contains linked document paths, DRM information, etc.
    pub fn doc_options(&self) -> Option<&DocOptions> {
        self.doc_options.as_ref()
    }

    /// Returns true if the document has DRM protection.
    pub fn has_drm(&self) -> bool {
        self.doc_options
            .as_ref()
            .map(|o| o.has_drm())
            .unwrap_or(false)
    }

    /// Returns true if the document has linked documents.
    pub fn has_linked_documents(&self) -> bool {
        self.doc_options
            .as_ref()
            .map(|o| o.has_links())
            .unwrap_or(false)
    }

    // === Style Information API ===

    /// Returns the font face names.
    pub fn font_faces(&self) -> &[crate::doc_info::FaceName] {
        &self.doc_info.face_names
    }

    /// Returns the character shapes.
    pub fn character_shapes(&self) -> &[crate::doc_info::CharacterShape] {
        &self.doc_info.character_shapes
    }

    /// Returns the paragraph shapes.
    pub fn paragraph_shapes(&self) -> &[crate::doc_info::ParagraphShape] {
        &self.doc_info.paragraph_shapes
    }

    /// Returns the border fills.
    pub fn border_fills(&self) -> &[crate::doc_info::BorderFill] {
        &self.doc_info.border_fills
    }

    /// Returns the styles.
    pub fn styles(&self) -> &[crate::doc_info::Style] {
        &self.doc_info.styles
    }

    /// Returns the tab definitions.
    pub fn tab_definitions(&self) -> &[crate::doc_info::TabDefinition] {
        &self.doc_info.tab_definitions
    }

    /// Returns the numbering definitions.
    pub fn numberings(&self) -> &[crate::doc_info::Numbering] {
        &self.doc_info.numberings
    }

    /// Returns the bullet definitions.
    pub fn bullets(&self) -> &[crate::doc_info::Bullet] {
        &self.doc_info.bullets
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_document_api() {
        // This would require sample files to test properly
        // For now, just ensure the module compiles
    }
}
