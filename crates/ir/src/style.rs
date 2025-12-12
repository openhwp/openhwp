//! 스타일 정의 및 저장소
//!
//! 문서의 스타일(글자 모양, 문단 모양 조합)을 정의합니다.

use crate::border_fill::BorderFill;
use crate::char_shape::{CharShape, Font};
use crate::para_shape::{ParaShape, TabDef};
use primitive::{Alignment, NumberFormat};
use primitive::{BorderFillId, CharShapeId, FontId, ParaShapeId, StyleId, StyleType, TabDefId};

/// 스타일 정의
#[derive(Debug, Clone)]
pub struct Style {
    /// 스타일 이름 (한글)
    pub name: String,
    /// 영문 이름
    pub english_name: Option<String>,
    /// 스타일 종류
    pub style_type: StyleType,
    /// 문단 모양 ID (문단 스타일인 경우)
    pub para_shape_id: Option<ParaShapeId>,
    /// 글자 모양 ID
    pub char_shape_id: Option<CharShapeId>,
    /// 다음 스타일 ID
    pub next_style_id: Option<StyleId>,
}

impl Style {
    /// 문단 스타일 생성
    pub fn paragraph(
        name: impl Into<String>,
        para_shape_id: ParaShapeId,
        char_shape_id: CharShapeId,
    ) -> Self {
        Self {
            name: name.into(),
            english_name: None,
            style_type: StyleType::Paragraph,
            para_shape_id: Some(para_shape_id),
            char_shape_id: Some(char_shape_id),
            next_style_id: None,
        }
    }

    /// 글자 스타일 생성
    pub fn character(name: impl Into<String>, char_shape_id: CharShapeId) -> Self {
        Self {
            name: name.into(),
            english_name: None,
            style_type: StyleType::Character,
            para_shape_id: None,
            char_shape_id: Some(char_shape_id),
            next_style_id: None,
        }
    }
}

/// 스타일 저장소
///
/// 문서에서 사용되는 모든 스타일 관련 정의를 보관합니다.
#[derive(Debug, Clone, Default)]
pub struct StyleStore {
    /// 폰트 정의 목록
    pub fonts: Vec<Font>,
    /// 글자 모양 목록
    pub char_shapes: Vec<CharShape>,
    /// 문단 모양 목록
    pub para_shapes: Vec<ParaShape>,
    /// 탭 정의 목록
    pub tab_defs: Vec<TabDef>,
    /// 테두리/채우기 목록
    pub border_fills: Vec<BorderFill>,
    /// 스타일 목록
    pub styles: Vec<Style>,
    /// 번호 매기기 정의 목록
    pub numberings: Vec<Numbering>,
    /// 글머리 기호 정의 목록
    pub bullets: Vec<Bullet>,
}

impl StyleStore {
    /// 빈 저장소 생성
    pub fn new() -> Self {
        Self::default()
    }

    // 폰트 관련

    /// 폰트 추가 후 ID 반환
    pub fn add_font(&mut self, font: Font) -> FontId {
        let id = FontId::new(self.fonts.len() as u32);
        self.fonts.push(font);
        id
    }

    /// 폰트 가져오기
    pub fn get_font(&self, id: FontId) -> Option<&Font> {
        self.fonts.get(id.value() as usize)
    }

    // 글자 모양 관련

    /// 글자 모양 추가 후 ID 반환
    pub fn add_char_shape(&mut self, char_shape: CharShape) -> CharShapeId {
        let id = CharShapeId::new(self.char_shapes.len() as u32);
        self.char_shapes.push(char_shape);
        id
    }

    /// 글자 모양 가져오기
    pub fn get_char_shape(&self, id: CharShapeId) -> Option<&CharShape> {
        self.char_shapes.get(id.value() as usize)
    }

    // 문단 모양 관련

    /// 문단 모양 추가 후 ID 반환
    pub fn add_para_shape(&mut self, para_shape: ParaShape) -> ParaShapeId {
        let id = ParaShapeId::new(self.para_shapes.len() as u32);
        self.para_shapes.push(para_shape);
        id
    }

    /// 문단 모양 가져오기
    pub fn get_para_shape(&self, id: ParaShapeId) -> Option<&ParaShape> {
        self.para_shapes.get(id.value() as usize)
    }

    // 탭 정의 관련

    /// 탭 정의 추가 후 ID 반환
    pub fn add_tab_def(&mut self, tab_def: TabDef) -> TabDefId {
        let id = TabDefId::new(self.tab_defs.len() as u32);
        self.tab_defs.push(tab_def);
        id
    }

    /// 탭 정의 가져오기
    pub fn get_tab_def(&self, id: TabDefId) -> Option<&TabDef> {
        self.tab_defs.get(id.value() as usize)
    }

    // 테두리/채우기 관련

    /// 테두리/채우기 추가 후 ID 반환
    pub fn add_border_fill(&mut self, border_fill: BorderFill) -> BorderFillId {
        let id = BorderFillId::new(self.border_fills.len() as u32);
        self.border_fills.push(border_fill);
        id
    }

    /// 테두리/채우기 가져오기
    pub fn get_border_fill(&self, id: BorderFillId) -> Option<&BorderFill> {
        self.border_fills.get(id.value() as usize)
    }

    // 스타일 관련

    /// 스타일 추가 후 ID 반환
    pub fn add_style(&mut self, style: Style) -> StyleId {
        let id = StyleId::new(self.styles.len() as u32);
        self.styles.push(style);
        id
    }

    /// 스타일 가져오기
    pub fn get_style(&self, id: StyleId) -> Option<&Style> {
        self.styles.get(id.value() as usize)
    }

    /// 이름으로 스타일 찾기
    pub fn find_style_by_name(&self, name: &str) -> Option<(StyleId, &Style)> {
        self.styles
            .iter()
            .enumerate()
            .find(|(_, s)| s.name == name)
            .map(|(i, s)| (StyleId::new(i as u32), s))
    }
}

/// 번호 매기기 정의
#[derive(Debug, Clone)]
pub struct Numbering {
    /// 번호 매기기 이름
    pub name: Option<String>,
    /// 수준별 설정 (최대 7수준)
    pub levels: Vec<NumberingLevel>,
    /// 시작 번호
    pub start_number: u32,
}

impl Default for Numbering {
    fn default() -> Self {
        Self {
            name: None,
            levels: Vec::new(),
            start_number: 1,
        }
    }
}

/// 번호 매기기 수준
#[derive(Debug, Clone)]
pub struct NumberingLevel {
    /// 수준 (0부터 시작)
    pub level: u8,
    /// 번호 형식 문자열 (예: "1.", "가.", "(1)")
    pub format: String,
    /// 글자 모양 ID
    pub char_shape_id: Option<CharShapeId>,
    /// 텍스트 오프셋
    pub text_offset: i32,
    /// 번호 너비
    pub number_width: i32,
    /// 시작 번호 (per-level)
    pub start_number: u32,
    /// 정렬 방식
    pub alignment: Alignment,
    /// 실제 인스턴스 너비 사용 여부
    pub use_instance_width: bool,
    /// 자동 들여쓰기 여부
    pub auto_indent: bool,
    /// 번호 형식 (숫자, 로마자, 한글 등)
    pub number_format: NumberFormat,
}

/// 글머리 기호 정의
#[derive(Debug, Clone)]
pub struct Bullet {
    /// 기호 문자
    pub char: char,
    /// 글자 모양 ID
    pub char_shape_id: Option<CharShapeId>,
    /// 체크 박스 여부
    pub is_checkbox: bool,
}

impl Bullet {
    /// 글머리 기호 생성
    pub const fn new(char: char) -> Self {
        Self {
            char,
            char_shape_id: None,
            is_checkbox: false,
        }
    }
}
