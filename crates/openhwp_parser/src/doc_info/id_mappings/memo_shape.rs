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

        for record in self
            .clone()
            .take(id_mappings.memo_shape as usize)
            .take_while(|record| record.tag == HwpTag::HWPTAG_MEMO_SHAPE)
        {
            memo_shapes.push(MemoShape::from_buf(record.payload));
            self.next();
        }

        memo_shapes
    }
}
