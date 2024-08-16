use crate::{u32, BodyIter, HwpTag};

#[non_exhaustive]
#[derive(Debug)]
pub struct ParagraphRangeTag {
    /// 영역 시작
    pub start_position: u32,
    /// 영역 끝
    pub end_position: u32,
    /// 태그(종류 + 데이터)
    /// : 상위 8비트가 종류를 하위 24비트가 종류별로 다른 설명을 부여할 수 있는 임의의 데이터를 나타낸다.
    pub property: u32,
}

impl<'hwp> BodyIter<'hwp> {
    pub fn paragraph_range_tags(&mut self, count: u16) -> Vec<ParagraphRangeTag> {
        let record = match self.expect(HwpTag::HWPTAG_PARA_RANGE_TAG) {
            Ok(record) => record,
            Err(_) => return vec![],
        };
        let mut buf = record.payload;
        let mut paragraph_range_tags = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let (paragraph_range_tag, rest) = buf.split_at(12);
            paragraph_range_tags.push(ParagraphRangeTag::from_buf(paragraph_range_tag));
            buf = rest;
        }

        paragraph_range_tags
    }
}

impl ParagraphRangeTag {
    pub fn from_buf(buf: &[u8]) -> Self {
        ParagraphRangeTag {
            start_position: u32(buf, 0),
            end_position: u32(buf, 4),
            property: u32(buf, 8),
        }
    }
}
