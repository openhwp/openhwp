use crate::{u32, DocInfoTag, HwpDocumentError, RecordIter};

#[derive(Debug)]
pub struct IdMappingCount {
    /// 바이너리 데이터
    pub binary_data: u32,
    /// 한글 글꼴
    pub hangul_font: u32,
    /// 영어 글꼴
    pub english_font: u32,
    /// 한자 글꼴
    pub chinese_font: u32,
    /// 일어 글꼴
    pub japanese_font: u32,
    /// 기타 글꼴
    pub etc_font: u32,
    /// 기호 글꼴
    pub symbol_font: u32,
    /// 사용자 글꼴
    pub user_font: u32,
    /// 테두리/배경
    pub border_fill: u32,
    /// 글자 모양
    pub char_shape: u32,
    /// 탭 정의
    pub tab_def: u32,
    /// 문단 번호
    pub numbering: u32,
    /// 글머리표
    pub bullet: u32,
    /// 문단 모양
    pub paragraph_shape: u32,
    /// 스타일
    pub style: u32,
    /// 메모 모양 (5.0.2.1 이상)
    pub memo_shape: u32,
    /// 변경추적 (5.0.3.2 이상)
    pub track_change: u32,
    /// 변경추적 사용자 (5.0.3.2 이상)
    pub track_change_author: u32,
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn id_mapping_count(&mut self) -> Result<IdMappingCount, HwpDocumentError> {
        let record = self.expect(DocInfoTag::HWPTAG_ID_MAPPINGS as u16)?;

        Ok(IdMappingCount::from_buf(record.payload))
    }
}

impl IdMappingCount {
    pub const fn from_buf(buf: &[u8]) -> Self {
        Self {
            binary_data: u32(buf, 0),
            hangul_font: u32(buf, 4),
            english_font: u32(buf, 8),
            chinese_font: u32(buf, 12),
            japanese_font: u32(buf, 16),
            etc_font: u32(buf, 20),
            symbol_font: u32(buf, 24),
            user_font: u32(buf, 28),
            border_fill: u32(buf, 32),
            char_shape: u32(buf, 36),
            tab_def: u32(buf, 40),
            numbering: u32(buf, 44),
            bullet: u32(buf, 48),
            paragraph_shape: u32(buf, 52),
            style: u32(buf, 56),
            memo_shape: u32(buf, 60),
            track_change: u32(buf, 64),
            track_change_author: u32(buf, 68),
        }
    }
}
