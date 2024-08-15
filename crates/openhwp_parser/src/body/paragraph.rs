use crate::RecordIter;

#[derive(Debug)]
pub struct Paragraph {
    //
}

impl<'hwp> RecordIter<'hwp> {
    pub fn paragraphs(&mut self) -> Vec<Paragraph> {
        let paragraphs = vec![];

        paragraphs
    }
}

impl Paragraph {
    pub fn from_buf(buf: &[u8]) -> Self {
        Self {}
    }
}
