pub mod bin_data;
pub mod border_fill;
pub mod bullet;
pub mod char_shape;
pub mod distribute_doc_data;
pub mod doc_data;
pub mod face_name;
pub mod numbering;
pub mod paragraph_shape;
pub mod style;
pub mod tab_definition;

use super::{DocInfoError, DocInfoTag, RecordIter};
use crate::{u32, Version};

pub use bin_data::*;
pub use border_fill::*;
pub use bullet::*;
pub use char_shape::*;
pub use distribute_doc_data::*;
pub use doc_data::*;
pub use face_name::*;
pub use numbering::*;
pub use paragraph_shape::*;
pub use style::*;
pub use tab_definition::*;

#[derive(Debug)]
pub struct IdMappings {
    pub id_mapping_count: IdMappingCount,
    pub bin_data: Vec<BinData>,
    pub face_names: Vec<FaceName>,
    pub border_fills: Vec<BorderFill>,
    pub char_shapes: Vec<CharShape>,
    pub tab_definitions: Vec<TabDefinition>,
    pub numberings: Vec<Numbering>,
    pub bullets: Vec<Bullet>,
    pub paragraph_shapes: Vec<ParagraphShape>,
    pub styles: Vec<Style>,
    pub doc_data: Vec<DocData>,
    pub distribute_doc_data: Vec<DistributeDocData>,
}

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
    pub fn id_mappings(&mut self, version: &Version) -> Result<IdMappings, DocInfoError> {
        let record = self.expect(DocInfoTag::HWPTAG_ID_MAPPINGS)?;
        let id_mapping_count = IdMappingCount::from_buf(record.payload);

        let bin_data = self.bin_data(&id_mapping_count);
        let face_names = self.face_names(&id_mapping_count);
        let border_fills = self.border_fills(&id_mapping_count);
        let char_shapes = self.char_shapes(&id_mapping_count, version);
        let tab_definitions = self.tab_definitions(&id_mapping_count);
        let numberings = self.numberings(&id_mapping_count, version);
        let bullets = self.bullets(&id_mapping_count);
        let paragraph_shapes = self.paragraph_shapes(&id_mapping_count, version);
        let styles = self.styles(&id_mapping_count);
        let doc_data = self.doc_data();
        let distribute_doc_data = self.distribute_doc_data();

        Ok(IdMappings {
            id_mapping_count,
            bin_data,
            face_names,
            border_fills,
            char_shapes,
            tab_definitions,
            numberings,
            bullets,
            paragraph_shapes,
            styles,
            doc_data,
            distribute_doc_data,
        })
    }
}

impl IdMappingCount {
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
