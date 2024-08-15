use crate::{HwpTag, RecordIter};

/// TODO: HWPTAG_FORBIDDEN_CHAR 분석 필요
#[derive(Debug)]
pub struct ForbiddenChar;

impl<'doc_info> RecordIter<'doc_info> {
    pub fn forbidden_chars(&mut self) -> Vec<ForbiddenChar> {
        let mut forbidden_chars = vec![];

        for record in self
            .clone()
            .take_while(|record| record.tag == HwpTag::HWPTAG_FORBIDDEN_CHAR)
        {
            forbidden_chars.push(ForbiddenChar::from_buf(record.payload));
            self.next();
        }

        forbidden_chars
    }
}

impl ForbiddenChar {
    #[cold]
    pub fn from_buf(_buf: &[u8]) -> Self {
        Self
    }
}
