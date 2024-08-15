mod compatible_document;
mod document_properties;
mod id_mappings;
mod stream;

pub use compatible_document::*;
pub use document_properties::*;
pub use id_mappings::*;
pub use stream::*;

use crate::{decompress, FileHeader, HwpDocumentError, HwpRead, Version};

#[derive(Debug)]
pub struct DocInfo {
    pub document_properties: DocumentProperties,
    pub id_mappings: IdMappings,
    pub compatible_document: CompatibleDocument,
}

impl DocInfo {
    pub fn from_reader<R: HwpRead>(
        reader: &mut R,
        file_header: &FileHeader,
    ) -> Result<Self, HwpDocumentError> {
        let buf = reader.doc_info()?;
        let buf = decompress!(buf, file_header.properties.compressed);

        Ok(Self::from_vec(buf, &file_header.version)?)
    }

    pub fn from_vec(buf: Vec<u8>, version: &Version) -> Result<Self, HwpDocumentError> {
        let mut stream = DocInfoIter::new(&buf, version);

        Ok(Self {
            document_properties: stream.document_properties()?,
            id_mappings: stream.id_mappings()?,
            compatible_document: stream.compatible_document(),
        })
    }
}
