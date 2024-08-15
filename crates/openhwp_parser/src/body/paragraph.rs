use super::{ParagraphHeader, ParagraphText};
use crate::{HwpDocumentError, RecordIter, Version};

#[derive(Debug)]
pub struct Paragraph {
    pub header: ParagraphHeader,
    pub text: ParagraphText,
}

impl<'hwp> RecordIter<'hwp> {
    pub fn paragraphs(&mut self, version: &Version) -> Result<Vec<Paragraph>, HwpDocumentError> {
        let mut paragraphs = vec![];

        while !self.is_empty() {
            paragraphs.push(self.paragraph(version)?);
        }

        Ok(paragraphs)
    }

    pub fn paragraph(&mut self, version: &Version) -> Result<Paragraph, HwpDocumentError> {
        let header = self.paragraph_header(version)?;
        let text = self.paragraph_text(header.text_size).unwrap_or_default();

        Ok(Paragraph { header, text })
    }
}
