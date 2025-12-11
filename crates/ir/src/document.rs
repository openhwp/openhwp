//! 문서 최상위 구조
//!
//! IR 문서의 루트 타입을 정의합니다.

use crate::binary::BinaryDataStore;
use crate::extensions::Extensions;
use crate::metadata::Metadata;
use crate::section::Section;
use crate::style::StyleStore;

/// IR 문서
///
/// HWP와 HWPX 문서를 통합 표현하는 최상위 타입입니다.
#[derive(Debug, Clone, Default)]
pub struct Document {
    /// 문서 메타데이터 (제목, 저자 등)
    pub metadata: Metadata,

    /// 문서 설정
    pub settings: DocumentSettings,

    /// 스타일 정의 (글자 모양, 문단 모양, 스타일 등)
    pub styles: StyleStore,

    /// 본문 섹션들
    pub sections: Vec<Section>,

    /// 바이너리 데이터 저장소 (이미지, OLE 등)
    pub binary_data: BinaryDataStore,

    /// 형식별 확장 데이터
    pub extensions: Extensions,
}

impl Document {
    /// 빈 문서 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 섹션 추가
    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }

    /// 섹션 수 반환
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }

    /// 전체 텍스트 추출
    pub fn to_plain_text(&self) -> String {
        let mut text = String::new();
        for section in &self.sections {
            for para in &section.paragraphs {
                if !text.is_empty() {
                    text.push('\n');
                }
                text.push_str(&para.to_plain_text());
            }
        }
        text
    }
}

/// 문서 설정
#[derive(Debug, Clone, Default)]
pub struct DocumentSettings {
    /// 시작 페이지 번호
    pub starting_page_number: u32,

    /// 시작 각주 번호
    pub starting_footnote_number: u32,

    /// 시작 미주 번호
    pub starting_endnote_number: u32,

    /// 캐럿 위치 (마지막 편집 위치)
    pub caret_position: Option<CaretPosition>,

    /// 호환 문서 대상
    pub compatible_document: Option<CompatibleDocument>,

    /// 대표 언어 (HWP language 필드, HWPX 미지원)
    pub representative_language: Option<u16>,
}

/// 캐럿 위치
#[derive(Debug, Clone, Default)]
pub struct CaretPosition {
    /// 섹션 인덱스
    pub section: u32,
    /// 문단 인덱스
    pub paragraph: u32,
    /// 문자 위치
    pub position: u32,
}

/// 호환 문서 대상
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompatibleDocument {
    /// 기본 (현재 버전)
    #[default]
    Current,
    /// 한글 2007
    Hwp2007,
    /// 한글 97
    Hwp97,
    /// MS Word
    MsWord,
}
