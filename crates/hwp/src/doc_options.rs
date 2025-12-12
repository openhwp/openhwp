//! Document options (DocOptions storage) parsing.
//!
//! The DocOptions storage contains various document-related options:
//! - _LinkDoc: Linked document paths
//! - DrmLicense: DRM packaging version
//! - DrmRootSect: Encryption algorithm
//! - CertDrmHeader: DRM packaging version for certificate

use crate::error::Result;

/// Linked document information.
///
/// Stored in the _LinkDoc stream within DocOptions storage.
#[derive(Debug, Clone, Default)]
pub struct LinkDoc {
    /// Linked document paths.
    paths: Vec<String>,
}

impl LinkDoc {
    /// Creates a new empty linked document.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the linked document paths.
    pub fn paths(&self) -> &[String] {
        &self.paths
    }

    /// Adds a linked document path.
    pub fn add_path(&mut self, path: String) {
        self.paths.push(path);
    }

    /// Parses linked document from bytes.
    ///
    /// The _LinkDoc stream contains UTF-16LE encoded paths,
    /// separated by null characters.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let mut link_doc = Self::new();

        if data.is_empty() {
            return Ok(link_doc);
        }

        // Parse UTF-16LE strings separated by null
        let mut current_path = Vec::new();
        for chunk in data.chunks(2) {
            if chunk.len() == 2 {
                let code_unit = u16::from_le_bytes([chunk[0], chunk[1]]);
                if code_unit == 0 {
                    if !current_path.is_empty() {
                        let path = String::from_utf16_lossy(&current_path);
                        if !path.is_empty() {
                            link_doc.paths.push(path);
                        }
                        current_path.clear();
                    }
                } else {
                    current_path.push(code_unit);
                }
            }
        }

        // Handle last path if not terminated
        if !current_path.is_empty() {
            let path = String::from_utf16_lossy(&current_path);
            if !path.is_empty() {
                link_doc.paths.push(path);
            }
        }

        Ok(link_doc)
    }
}

/// DRM license information.
///
/// Stored in the DrmLicense stream within DocOptions storage.
#[derive(Debug, Clone, Default)]
pub struct DrmLicense {
    /// DRM packaging version.
    version: u32,
    /// Raw license data.
    data: Vec<u8>,
}

impl DrmLicense {
    /// Creates a new DRM license.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the DRM version.
    pub const fn version(&self) -> u32 {
        self.version
    }

    /// Returns the raw license data.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Parses DRM license from bytes.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let version = if data.len() >= 4 {
            u32::from_le_bytes([data[0], data[1], data[2], data[3]])
        } else {
            0
        };

        Ok(Self {
            version,
            data: data.to_vec(),
        })
    }
}

/// Document options storage.
///
/// Contains various document-related options and DRM information.
#[derive(Debug, Clone, Default)]
pub struct DocOptions {
    /// Linked document information.
    pub link_doc: Option<LinkDoc>,
    /// DRM license information.
    pub drm_license: Option<DrmLicense>,
    /// DRM root section (encryption algorithm).
    pub drm_root_sect: Option<Vec<u8>>,
    /// Certificate DRM header.
    pub cert_drm_header: Option<Vec<u8>>,
}

impl DocOptions {
    /// Creates a new empty document options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the linked document information.
    pub fn set_link_doc(&mut self, link_doc: LinkDoc) {
        self.link_doc = Some(link_doc);
    }

    /// Sets the DRM license information.
    pub fn set_drm_license(&mut self, drm_license: DrmLicense) {
        self.drm_license = Some(drm_license);
    }

    /// Sets the DRM root section data.
    pub fn set_drm_root_sect(&mut self, data: Vec<u8>) {
        self.drm_root_sect = Some(data);
    }

    /// Sets the certificate DRM header data.
    pub fn set_cert_drm_header(&mut self, data: Vec<u8>) {
        self.cert_drm_header = Some(data);
    }

    /// Returns true if any DRM information is present.
    pub const fn has_drm(&self) -> bool {
        self.drm_license.is_some() || self.drm_root_sect.is_some() || self.cert_drm_header.is_some()
    }

    /// Returns true if the document has linked documents.
    pub fn has_links(&self) -> bool {
        self.link_doc
            .as_ref()
            .map(|l| !l.paths().is_empty())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_doc_new() {
        let link = LinkDoc::new();
        assert!(link.paths().is_empty());
    }

    #[test]
    fn test_link_doc_from_bytes() {
        // "test.hwp" in UTF-16LE followed by null
        let data = [
            0x74, 0x00, // t
            0x65, 0x00, // e
            0x73, 0x00, // s
            0x74, 0x00, // t
            0x2E, 0x00, // .
            0x68, 0x00, // h
            0x77, 0x00, // w
            0x70, 0x00, // p
            0x00, 0x00, // null terminator
        ];
        let link = LinkDoc::from_bytes(&data).unwrap();
        assert_eq!(link.paths().len(), 1);
        assert_eq!(link.paths()[0], "test.hwp");
    }

    #[test]
    fn test_drm_license_from_bytes() {
        let data = [0x01, 0x00, 0x00, 0x00, 0xAB, 0xCD];
        let license = DrmLicense::from_bytes(&data).unwrap();
        assert_eq!(license.version(), 1);
        assert_eq!(license.data().len(), 6);
    }

    #[test]
    fn test_doc_options_new() {
        let options = DocOptions::new();
        assert!(!options.has_drm());
        assert!(!options.has_links());
    }

    #[test]
    fn test_doc_options_with_drm() {
        let mut options = DocOptions::new();
        options.set_drm_license(DrmLicense::new());
        assert!(options.has_drm());
    }

    #[test]
    fn test_doc_options_with_links() {
        let mut options = DocOptions::new();
        let mut link = LinkDoc::new();
        link.add_path("test.hwp".to_string());
        options.set_link_doc(link);
        assert!(options.has_links());
    }
}
