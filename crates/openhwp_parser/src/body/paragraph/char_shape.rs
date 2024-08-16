use crate::{u32, BodyIter, HwpDocumentError, HwpTag};

#[derive(Debug)]
pub struct CharShape {
    /// 글자 모양이 바뀌는 시작 위치
    pub start_position: u32,
    /// 글자 모양 ID
    pub shape_id: u32,
}

impl<'hwp> BodyIter<'hwp> {
    pub fn char_shapes(&mut self, count: u16) -> Result<Vec<CharShape>, HwpDocumentError> {
        let record = self.expect(HwpTag::HWPTAG_PARA_CHAR_SHAPE)?;
        let mut buf = record.payload;
        let mut char_shapes = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let (char_shape, rest) = buf.split_at(8);
            char_shapes.push(CharShape::from_buf(char_shape));
            buf = rest;
        }

        Ok(char_shapes)
    }
}

impl CharShape {
    pub const fn from_buf(buf: &[u8]) -> Self {
        CharShape {
            start_position: u32(buf, 0),
            shape_id: u32(buf, 4),
        }
    }
}
