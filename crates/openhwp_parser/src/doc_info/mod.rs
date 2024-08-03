mod document_properties;
mod id_mappings;
mod record;
mod tag;

pub use document_properties::*;
pub use id_mappings::*;
pub use record::*;
pub use tag::*;

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

macro_rules! expect {
    ($record:ident, $tag:ident) => {
        match $record.next() {
            Some(record) if record.tag_id == DocInfoTag::$tag as u16 => record,
            Some(record) => return Err(DocInfoError::InvalidTagId(record.tag_id)),
            None => return Err(DocInfoError::EndOfRecords(stringify!($tag))),
        }
    };
}

impl DocInfo {
    pub fn from_vec(bytes: Vec<u8>, compressed: bool) -> Result<Self, DocInfoError> {
        let bytes = match compressed {
            true => decompress(&bytes)?,
            false => bytes,
        };
        let mut records = Record::iter(&bytes);

        Ok(Self {
            document_properties: DocumentProperties::from_record(&expect!(
                records,
                HWPTAG_DOCUMENT_PROPERTIES
            )),
            id_mappings: IdMappings::from_record(
                &expect!(records, HWPTAG_ID_MAPPINGS),
                &mut records,
            ),
        })
    }
}

fn decompress(bytes: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    use flate2::bufread::DeflateDecoder;
    use std::io::Read;

    let mut buf = vec![];
    DeflateDecoder::new(bytes).read_to_end(&mut buf)?;

    Ok(buf)
}
