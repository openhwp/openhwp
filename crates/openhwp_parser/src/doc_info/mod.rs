mod document_properties;
mod id_mappings;
mod record;
mod tag;

pub use document_properties::*;
pub use id_mappings::*;
pub use record::*;
pub use tag::*;

use crate::Version;

#[derive(Debug)]
pub struct DocInfo {
    pub document_properties: DocumentProperties,
    pub id_mappings: IdMappings,
}

#[derive(Debug, Error)]
pub enum DocInfoError {
    #[error("Decompression error: {0}")]
    Decompression(#[from] std::io::Error),
    #[error("Invalid tag id: {0}")]
    InvalidTagId(u16),
    #[error("End of records: {0}")]
    EndOfRecords(&'static str),
}

impl DocInfo {
    pub fn from_vec(
        buf: Vec<u8>,
        compressed: bool,
        version: &Version,
    ) -> Result<Self, DocInfoError> {
        let buf = match compressed {
            true => decompress(&buf)?,
            false => buf,
        };
        let mut stream = Record::iter(&buf);

        Ok(Self {
            document_properties: stream.document_properties()?,
            id_mappings: stream.id_mappings(version)?,
        })
    }
}

fn decompress(source: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    use flate2::bufread::DeflateDecoder;
    use std::io::Read;

    let mut buf = vec![];
    DeflateDecoder::new(source).read_to_end(&mut buf)?;

    Ok(buf)
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn expect(&mut self, tag: DocInfoTag) -> Result<Record, DocInfoError> {
        match self.next() {
            Some(record) if record.tag_id == tag as u16 => Ok(record),
            Some(record) => Err(DocInfoError::InvalidTagId(record.tag_id)),
            None => Err(DocInfoError::EndOfRecords(stringify!($tag))),
        }
    }
}
