use super::{Record, RecordIter};
use crate::u32;

#[derive(Debug)]
pub struct IdMappings {
    /// 바이너리 데이터
    binary_data: u32,
    /// 한글 글꼴
    hangul_font: u32,
    /// 영어 글꼴
    english_font: u32,
    /// 한자 글꼴
    chinese_font: u32,
    /// 일어 글꼴
    japanese_font: u32,
    /// 기타 글꼴
    etc_font: u32,
    /// 기호 글꼴
    symbol_font: u32,
    /// 사용자 글꼴
    user_font: u32,
    /// 테두리/배경
    border_fill: u32,
    /// 글자 모양
    char_shape: u32,
    /// 탭 정의
    tab_def: u32,
    /// 문단 번호
    numbering: u32,
    /// 글머리표
    bullet: u32,
    /// 문단 모양
    paragraph_shape: u32,
    /// 스타일
    style: u32,
    /// 메모 모양 (5.0.2.1 이상)
    memo_shape: u32,
    /// 변경추적 (5.0.3.2 이상)
    track_change: u32,
    /// 변경추적 사용자 (5.0.3.2 이상)
    track_change_author: u32,
}

impl IdMappings {
    pub fn from_record(record: &Record, records: &mut RecordIter) -> Self {
        let buf = &record.payload;

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
