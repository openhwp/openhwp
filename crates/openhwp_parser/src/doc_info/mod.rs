mod compatible_document;
mod document_properties;
mod id_mappings;
mod stream;

pub use compatible_document::*;
pub use document_properties::*;
pub use id_mappings::*;
pub use stream::*;

use crate::{FileHeader, HwpDocumentError, HwpRead, Version, decompress};

#[derive(Debug)]
pub struct DocInfo {
    pub document_properties: DocumentProperties,
    pub id_mappings: IdMappings,
    pub compatible_document: Option<CompatibleDocument>,
}

impl DocInfo {
    pub fn from_reader<R: HwpRead>(
        reader: &mut R,
        file_header: &FileHeader,
    ) -> Result<Self, HwpDocumentError> {
        let buf = reader.doc_info()?;
        let buf = decompress!(buf, file_header.properties.compressed);

        Ok(Self::from_buf(
            &buf,
            &file_header.version,
            file_header.properties.encrypted,
        )?)
    }

    pub fn from_buf(
        buf: &[u8],
        version: &Version,
        encrypted: bool,
    ) -> Result<Self, HwpDocumentError> {
        let mut stream = DocInfoIter::new(buf, version);

        Ok(Self {
            document_properties: stream.document_properties()?,
            id_mappings: stream.id_mappings()?,
            compatible_document: if encrypted {
                None
            } else {
                stream.compatible_document()
            },
        })
    }
}
