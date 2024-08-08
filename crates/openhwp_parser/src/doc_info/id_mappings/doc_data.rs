use crate::{DocInfoTag, RecordIter};

/// TODO: HWPTAG_DOC_DATA 분석 필요
#[derive(Debug)]
pub struct DocData;

impl<'doc_info> RecordIter<'doc_info> {
    pub fn doc_data(&mut self) -> Vec<DocData> {
        let mut doc_data = vec![];

        for record in self.take_while(|record| record.tag_id != DocInfoTag::HWPTAG_DOC_DATA as u16)
        {
            doc_data.push(DocData::from_buf(record.payload));
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
