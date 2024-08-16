use crate::{u32, BodyIter, HwpDocumentError, HwpTag};

#[derive(Debug)]
pub struct LineSegment {
    /// 텍스트 시작 위치
    pub start_position: u32,
    /// 줄의 세로 위치
    pub vertical_position: i32,
    /// 줄의 높이
    pub line_height: i32,
    /// 텍스트 부분의 높이
    pub text_height: i32,
    /// 줄의 세로 위치에서 베이스라인까지 거리
    pub baseline_gap: i32,
    /// 줄간격
    pub line_spacing: i32,
    /// 컬럼에서의 시작 위치
    pub start_position_in_column: i32,
    /// 세그먼트의 폭
    pub width: i32,
    /// 페이지의 첫 줄인지 여부
    pub is_first_line_in_page: bool,
    /// 컬럼의 첫 줄인지 여부
    pub is_first_line_in_column: bool,
    /// 텍스트가 배열되지 않은 빈 세그먼트인지 여부
    pub is_empty: bool,
    /// 줄의 첫 세그먼트인지 여부
    pub is_first: bool,
    /// 줄의 마지막 세그먼트인지 여부
    pub is_last: bool,
    /// 줄의 마지막에 auto-hyphenation이 수행되었는지 여부.
    pub auto_hyphenated: bool,
    /// indentation 적용
    pub indented: bool,
    /// 문단 머리 모양 적용
    pub use_heading: bool,
}

impl<'hwp> BodyIter<'hwp> {
    pub fn line_segments(&mut self, count: u16) -> Result<Vec<LineSegment>, HwpDocumentError> {
        let record = self.expect(HwpTag::HWPTAG_PARA_LINE_SEG)?;
        let mut buf = record.payload;
        let mut line_segments = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let (line_segment, rest) = buf.split_at(8);
            line_segments.push(LineSegment::from_buf(line_segment));
            buf = rest;
        }

        Ok(line_segments)
    }
}

impl LineSegment {
    pub const fn from_buf(buf: &[u8]) -> Self {
        LineSegment {
            start_position: u32(buf, 0),
            vertical_position: u32(buf, 4) as i32,
            line_height: u32(buf, 8) as i32,
            text_height: u32(buf, 12) as i32,
            baseline_gap: u32(buf, 16) as i32,
            line_spacing: u32(buf, 20) as i32,
            start_position_in_column: u32(buf, 24) as i32,
            width: u32(buf, 28) as i32,
            is_first_line_in_page: buf[32] & 0b0000_0001 != 0,
            is_first_line_in_column: buf[32] & 0b0000_0010 != 0,
            is_empty: buf[34] & 0b0000_0001 != 0,
            is_first: buf[34] & 0b0000_0010 != 0,
            is_last: buf[34] & 0b0000_0100 != 0,
            auto_hyphenated: buf[34] & 0b0000_1000 != 0,
            indented: buf[34] & 0b0001_0000 != 0,
            use_heading: buf[34] & 0b0010_0000 != 0,
        }
    }
}
