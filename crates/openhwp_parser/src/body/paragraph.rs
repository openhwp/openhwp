use crate::RecordIter;

#[derive(Debug)]
pub struct Paragraph {
    //
}

impl<'hwp> RecordIter<'hwp> {
    pub fn paragraphs(&mut self) -> Vec<Paragraph> {
        let mut paragraphs = vec![];

        for record in self.clone().take_while(|record| record.tag_id == 0x0102) {
            paragraphs.push(Paragraph::from_buf(record.payload));
            self.next();
        }

        paragraphs
    }
}

impl Paragraph {
    pub fn from_buf(buf: &[u8]) -> Self {
        Self {}
    }
}
