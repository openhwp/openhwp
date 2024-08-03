pub mod bin_data;
pub mod face_name;

use super::{DocInfoError, DocInfoTag, RecordIter};
use crate::u32;

pub use bin_data::*;
pub use face_name::*;

#[derive(Debug)]
pub struct IdMappings {
    id_mapping_counts: IdMappingCounts,
    bin_data: Vec<BinData>,
    face_names: Vec<FaceName>,
}

#[derive(Debug)]
pub struct IdMappingCounts {
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

#[derive(Debug, Error)]
pub enum IdMappingsError {
    #[error("Bin data error: {0}")]
    BinaryData(#[from] BinDataError),
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn id_mappings(&mut self) -> Result<IdMappings, DocInfoError> {
        let record = self.expect(DocInfoTag::HWPTAG_ID_MAPPINGS)?;
        let id_mapping_counts = IdMappingCounts::from_buf(record.payload);

        let bin_data = self.bin_data(&id_mapping_counts)?;
        let face_names = self.face_names(&id_mapping_counts)?;

        Ok(IdMappings {
            id_mapping_counts,
            bin_data,
            face_names,
        })
    }
}

impl IdMappingCounts {
    pub fn from_buf(buf: &[u8]) -> Self {
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
