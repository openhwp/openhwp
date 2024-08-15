use crate::{Control, HwpDocumentError, HwpTag, HwpText, RecordIter};

#[derive(Debug, Default)]
pub struct ParagraphText {
    pub text: HwpText,
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn paragraph_text(&mut self, size: usize) -> Result<ParagraphText, HwpDocumentError> {
        let record = self.expect(HwpTag::HWPTAG_PARA_TEXT)?;
        let text = HwpText::from_buf(record.payload, size);

        Ok(ParagraphText { text })
    }
}

impl ParagraphText {
    #[inline]
    pub fn to_string(&self) -> String {
        self.text.to_string()
    }

    #[inline]
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.text.chars()
    }

    #[inline]
    pub fn controls(&self) -> impl Iterator<Item = &Control> + '_ {
        self.text.controls()
    }
}
