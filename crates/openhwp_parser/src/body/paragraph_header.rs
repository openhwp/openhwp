use crate::{u32, HwpDocumentError, HwpTag, RecordIter};

#[derive(Debug)]
pub struct ParagraphHeader {
    pub text_size: usize,
}

impl<'hwp> RecordIter<'hwp> {
    pub fn paragraph_header(&mut self) -> Result<ParagraphHeader, HwpDocumentError> {
        let record = self.expect(HwpTag::HWPTAG_PARA_HEADER)?;
        let header = ParagraphHeader::from_buf(record.payload);

        Ok(header)
    }
}

impl ParagraphHeader {
    pub const fn from_buf(buf: &[u8]) -> Self {
        let text_size = match u32(buf, 0) {
            size if size & 0x80000000 == 0x80000000 => size & 0x7fffffff,
            size => size,
        } as usize;

        Self { text_size }
    }
}
