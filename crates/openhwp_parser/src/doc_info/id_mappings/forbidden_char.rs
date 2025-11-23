use crate::{DocInfoIter, HwpTag};

/// TODO: HWPTAG_FORBIDDEN_CHAR 분석 필요
#[derive(Debug)]
pub struct ForbiddenChar;

impl<'hwp> DocInfoIter<'hwp> {
    pub fn forbidden_chars(&mut self) -> Vec<ForbiddenChar> {
        let mut forbidden_chars = vec![];

        while let Some(record) = self.next_if(|record| record.tag == HwpTag::HWPTAG_FORBIDDEN_CHAR)
        {
            forbidden_chars.push(ForbiddenChar::from_buf(record.payload));
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
