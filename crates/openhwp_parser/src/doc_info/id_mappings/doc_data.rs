use crate::{DocInfoIter, HwpTag};

/// TODO: HWPTAG_DOC_DATA 분석 필요
#[derive(Debug)]
pub struct DocData;

impl<'hwp> DocInfoIter<'hwp> {
    pub fn doc_data(&mut self) -> Vec<DocData> {
        let mut doc_data = vec![];

        while let Some(record) = self.next_if(|record| record.tag == HwpTag::HWPTAG_DOC_DATA) {
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
