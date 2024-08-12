mod compatible_document;
mod document_properties;
mod id_mappings;

pub use compatible_document::*;
pub use document_properties::*;
pub use id_mappings::*;

use crate::{decompress, Compressed};
use crate::{DocInfoTag, FileHeader, HwpDocumentError, HwpRead, Record, RecordIter, Version};

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
        let buf = match file_header.properties.compressed {
            Compressed::Yes => decompress(&buf)?,
            Compressed::No => buf,
        };
        Ok(Self::from_vec(buf, &file_header.version)?)
    }

    pub fn from_vec(buf: Vec<u8>, version: &Version) -> Result<Self, HwpDocumentError> {
        let mut stream = Record::iter(&buf);

        Ok(Self {
            document_properties: stream.document_properties()?,
            id_mappings: stream.id_mappings(version)?,
            compatible_document: stream.compatible_document(),
        })
    }
}
