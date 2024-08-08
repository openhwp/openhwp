use crate::{DocInfoTag, RecordIter};

/// TODO: HWPTAG_DISTRIBUTE_DOC_DATA 분석 필요
#[derive(Debug)]
pub struct DistributeDocData;

impl<'doc_info> RecordIter<'doc_info> {
    pub fn distribute_doc_data(&mut self) -> Vec<DistributeDocData> {
        let mut distribute_doc_data = vec![];

        for record in
            self.take_while(|record| record.tag_id != DocInfoTag::HWPTAG_DISTRIBUTE_DOC_DATA as u16)
        {
            distribute_doc_data.push(DistributeDocData::from_buf(record.payload));
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
