use super::{ParagraphHeader, ParagraphText};
use crate::{HwpDocumentError, RecordIter};

#[derive(Debug)]
pub struct Paragraph {
    pub header: ParagraphHeader,
    pub text: ParagraphText,
}

impl<'hwp> RecordIter<'hwp> {
    pub fn paragraphs(&mut self) -> Result<Vec<Paragraph>, HwpDocumentError> {
        let mut paragraphs = vec![];

        while !self.is_empty() {
            paragraphs.push(self.paragraph()?);
        }

        Ok(paragraphs)
    }

    pub fn paragraph(&mut self) -> Result<Paragraph, HwpDocumentError> {
        let header = self.paragraph_header()?;
        let text = self.paragraph_text(header.text_size)?;

        Ok(Paragraph { header, text })
    }
}
