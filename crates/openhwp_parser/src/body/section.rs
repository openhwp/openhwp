use super::Paragraph;
use crate::{decode, decompress, Compressed, HwpDocumentError, Record};

#[derive(Debug)]
pub struct Section {
    pub paragraphs: Vec<Paragraph>,
}

#[derive(Debug, Error)]
pub enum SectionError {
    #[error("Invalid section")]
    InvalidSection,
}

impl Section {
    #[inline]
    pub fn from_non_distributed(
        buf: Vec<u8>,
        compressed: Compressed,
    ) -> Result<Self, HwpDocumentError> {
        let buf = decompress!(buf, compressed);

        Ok(Self::from_buf(&buf)?)
    }

    pub fn from_distributed(
        buf: Vec<u8>,
        compressed: Compressed,
    ) -> Result<Self, HwpDocumentError> {
        let decoded = decode(buf)?;
        let buf = decompress!(decoded, compressed);

        Ok(Self::from_buf(&buf)?)
    }

    pub fn from_buf(buf: &[u8]) -> Result<Self, SectionError> {
        let mut stream = Record::iter(buf);

        Ok(Self {
            paragraphs: stream.paragraphs(),
        })
    }
}
