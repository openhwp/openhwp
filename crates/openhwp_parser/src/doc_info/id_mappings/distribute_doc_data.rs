use crate::{HwpTag, RecordIter};

/// TODO: HWPTAG_DISTRIBUTE_DOC_DATA 분석 필요
#[derive(Debug)]
pub struct DistributeDocData;

impl<'hwp> RecordIter<'hwp> {
    pub fn distribute_doc_data(&mut self) -> Vec<DistributeDocData> {
        let mut distribute_doc_data = vec![];

        for record in self
            .clone()
            .take_while(|record| record.tag == HwpTag::HWPTAG_DISTRIBUTE_DOC_DATA)
        {
            distribute_doc_data.push(DistributeDocData::from_buf(record.payload));
            self.next();
        }

        distribute_doc_data
    }
}

impl DistributeDocData {
    #[cold]
    pub fn from_buf(_buf: &[u8]) -> Self {
        Self
    }
}
