use crate::{HwpTag, RecordIter};

/// TODO: HWPTAG_DOC_DATA 분석 필요
#[derive(Debug)]
pub struct DocData;

impl<'doc_info> RecordIter<'doc_info> {
    pub fn doc_data(&mut self) -> Vec<DocData> {
        let mut doc_data = vec![];

        for record in self
            .clone()
            .take_while(|record| record.tag == HwpTag::HWPTAG_DOC_DATA)
        {
            doc_data.push(DocData::from_buf(record.payload));
            self.next();
        }

        doc_data
    }
}

impl DocData {
    #[cold]
    pub fn from_buf(_buf: &[u8]) -> Self {
        Self
    }
}
