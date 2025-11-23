use super::IdMappingCount;
use crate::{DocInfoIter, HwpTag};

#[derive(Debug, Clone)]
pub struct MemoShape {
    pub raw: Vec<u8>,
}

impl MemoShape {
    #[inline]
    fn from_buf(buf: &[u8]) -> Self {
        Self { raw: buf.to_vec() }
    }
}

impl<'hwp> DocInfoIter<'hwp> {
    pub fn memo_shapes(&mut self, id_mappings: &IdMappingCount) -> Vec<MemoShape> {
        let mut memo_shapes = Vec::with_capacity(id_mappings.memo_shape as usize);

        for _ in 0..id_mappings.memo_shape {
            match self.next_if(|record| record.tag == HwpTag::HWPTAG_MEMO_SHAPE) {
                Some(record) => memo_shapes.push(MemoShape::from_buf(record.payload)),
                None => break,
            }
        }

        memo_shapes
    }
}
