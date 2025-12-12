//! 섹션
//!
//! 문서의 섹션(구역)을 정의합니다. 섹션은 페이지 설정과 본문 내용을 포함합니다.

use crate::paragraph::Paragraph;
use primitive::{
    BorderFillId, Color, EndnotePlacement, FootnotePlacement, GutterPosition, HeaderFooterApplyTo,
    HwpUnit, Insets, LineNumberRestartType, LineType, NoteNumbering, NumberFormat, PageMargins,
    PageOrientation, PageStartsOn,
};

// Re-export primitive types (only simple enums that don't have structural differences)
pub use primitive::{PageBorderFillArea, PageBorderPageType, PageBorderPosition, VisibilityOption};

/// 섹션
///
/// 문서의 구역 단위입니다. 각 섹션은 독립적인 페이지 설정을 가집니다.
#[derive(Debug, Clone, Default)]
pub struct Section {
    /// 페이지 정의
    pub page: PageDefinition,

    /// 문단 목록
    pub paragraphs: Vec<Paragraph>,

    /// 단 설정
    pub columns: ColumnDefinition,

    /// 머리글
    pub headers: Vec<HeaderFooter>,

    /// 바닥글
    pub footers: Vec<HeaderFooter>,

    /// 각주 설정
    pub footnote_shape: Option<FootnoteShape>,

    /// 미주 설정
    pub endnote_shape: Option<EndnoteShape>,

    /// 페이지 테두리/배경
    pub page_border_fill: Option<PageBorderFill>,

    /// 시작 번호 정보
    pub start_number: SectionStartNumber,

    /// 섹션 확장 데이터
    pub extensions: SectionExtensions,
}

impl Section {
    /// 빈 섹션 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 문단 추가
    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph);
    }

    /// 문단 수 반환
    pub fn paragraph_count(&self) -> usize {
        self.paragraphs.len()
    }
}

/// 섹션 시작 번호 정보
///
/// 섹션에서 페이지, 그림, 표, 수식의 시작 번호를 설정합니다.
#[derive(Debug, Clone)]
pub struct SectionStartNumber {
    /// 구역 나눔으로 새 페이지가 생길 때 페이지 번호 적용 옵션
    pub page_starts_on: PageStartsOn,
    /// 쪽 시작 번호 (0이면 앞 구역에 이어서 번호를 매김, 1 이상이면 임의의 번호로 시작)
    pub page: u32,
    /// 그림 시작 번호 (0이면 앞 구역에 이어서 번호를 매김, 1 이상이면 임의의 번호로 시작)
    pub picture: u32,
    /// 표 시작 번호 (0이면 앞 구역에 이어서 번호를 매김, 1 이상이면 임의의 번호로 시작)
    pub table: u32,
    /// 수식 시작 번호 (0이면 앞 구역에 이어서 번호를 매김, 1 이상이면 임의의 번호로 시작)
    pub equation: u32,
}

impl Default for SectionStartNumber {
    fn default() -> Self {
        Self {
            page_starts_on: PageStartsOn::Both,
            page: 0,
            picture: 0,
            table: 0,
            equation: 0,
        }
    }
}

/// 페이지 정의
#[derive(Debug, Clone)]
pub struct PageDefinition {
    /// 용지 너비
    pub width: HwpUnit,
    /// 용지 높이
    pub height: HwpUnit,
    /// 여백
    pub margins: PageMargins,
    /// 용지 방향
    pub orientation: PageOrientation,
    /// 제본 여백 위치
    pub gutter_position: GutterPosition,
}

impl Default for PageDefinition {
    fn default() -> Self {
        // A4 용지 기본값 (210mm x 297mm)
        Self {
            width: HwpUnit::from_mm(210.0),
            height: HwpUnit::from_mm(297.0),
            margins: PageMargins::default(),
            orientation: PageOrientation::Portrait,
            gutter_position: GutterPosition::Left,
        }
    }
}


/// 단 정의
#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    /// 단 수
    pub count: u16,
    /// 단 방향
    pub direction: ColumnDirection,
    /// 단 간격
    pub gap: HwpUnit,
    /// 구분선 종류
    pub separator: ColumnSeparator,
    /// 구분선 두께 (0.1mm 단위, 기본값 0)
    pub separator_thickness: u8,
    /// 구분선 색상
    pub separator_color: Color,
    /// 개별 단 너비 (비어있으면 균등 분할)
    pub widths: Vec<HwpUnit>,
}

impl Default for ColumnDefinition {
    fn default() -> Self {
        Self {
            count: 1,
            direction: ColumnDirection::LeftToRight,
            gap: HwpUnit::from_mm(8.0),
            separator: ColumnSeparator::None,
            separator_thickness: 0,
            separator_color: Color::BLACK,
            widths: Vec::new(),
        }
    }
}

/// 단 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnDirection {
    /// 왼쪽에서 오른쪽
    #[default]
    LeftToRight,
    /// 오른쪽에서 왼쪽
    RightToLeft,
    /// 맞쪽 페이지 (홀수/짝수 페이지 다르게)
    FacingPages,
}

/// 단 구분선
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnSeparator {
    /// 없음
    #[default]
    None,
    /// 실선
    Solid,
    /// 파선
    Dash,
    /// 점선
    Dot,
}

/// 머리글/바닥글
#[derive(Debug, Clone)]
pub struct HeaderFooter {
    /// 적용 대상
    pub apply_to: HeaderFooterApplyTo,
    /// 내용 (문단 목록)
    pub paragraphs: Vec<Paragraph>,
}

impl HeaderFooter {
    /// 머리글/바닥글 생성
    pub fn new(apply_to: HeaderFooterApplyTo) -> Self {
        Self {
            apply_to,
            paragraphs: Vec::new(),
        }
    }
}

/// 각주/미주 모양
#[derive(Debug, Clone)]
pub struct NoteShape {
    /// 번호 형식
    pub number_format: NumberFormat,
    /// 번호 매김 방식 (연속/섹션별/페이지별)
    pub numbering: NoteNumbering,
    /// 위 첨자 여부
    pub superscript: bool,
    /// 접두사
    pub prefix: Option<String>,
    /// 접미사
    pub suffix: Option<String>,
    /// 시작 번호
    pub start_number: u32,
    /// 사용자 기호 (번호 형식이 사용자 정의일 때)
    pub user_character: Option<String>,
    /// 구분선 길이 (HwpUnit 절대값, 또는 페이지/단 너비 기준 상대값)
    pub separator_length: HwpUnit,
    /// 구분선 위치 (HWP 전용, 여백으로부터의 거리)
    pub separator_position: Option<HwpUnit>,
    /// 구분선 종류
    pub separator_line_type: LineType,
    /// 구분선 굵기 (0.1mm 단위)
    pub separator_line_width: u8,
    /// 구분선 색상
    pub separator_line_color: Color,
    /// 본문과의 간격 (구분선 위)
    pub space_above: HwpUnit,
    /// 구분선 아래 간격
    pub space_below: HwpUnit,
    /// 각주 간 간격
    pub space_between: HwpUnit,
    /// 텍스트에 이어 바로 출력 여부
    pub beneath_text: bool,
}

impl Default for NoteShape {
    fn default() -> Self {
        Self {
            number_format: NumberFormat::Digit,
            numbering: NoteNumbering::Continuous,
            superscript: true,
            prefix: None,
            suffix: Some(")".to_string()),
            start_number: 1,
            user_character: None,
            separator_length: HwpUnit::from_mm(20.0),  // 기본값 2cm (약 1/3 페이지)
            separator_position: None,
            separator_line_type: LineType::Solid,
            separator_line_width: 1,
            separator_line_color: Color::BLACK,
            space_above: HwpUnit::from_mm(5.0),
            space_below: HwpUnit::from_mm(2.0),
            space_between: HwpUnit::from_mm(3.0),
            beneath_text: false,
        }
    }
}

/// 각주 모양 (NoteShape 확장)
#[derive(Debug, Clone)]
pub struct FootnoteShape {
    /// 기본 각주/미주 모양
    pub base: NoteShape,
    /// 각주 배치 위치
    pub placement: FootnotePlacement,
}

impl Default for FootnoteShape {
    fn default() -> Self {
        Self {
            base: NoteShape::default(),
            placement: FootnotePlacement::EachColumn,
        }
    }
}

/// 미주 모양 (NoteShape 확장)
#[derive(Debug, Clone)]
pub struct EndnoteShape {
    /// 기본 각주/미주 모양
    pub base: NoteShape,
    /// 미주 배치 위치
    pub placement: EndnotePlacement,
}

impl Default for EndnoteShape {
    fn default() -> Self {
        Self {
            base: NoteShape::default(),
            placement: EndnotePlacement::EndOfDocument,
        }
    }
}

/// 페이지 테두리/배경
#[derive(Debug, Clone)]
pub struct PageBorderFill {
    /// 테두리/채우기 ID
    pub border_fill_id: BorderFillId,
    /// 테두리 위치 기준
    pub position: PageBorderPosition,
    /// 여백
    pub offsets: Insets,
    /// 첫 페이지만 적용
    pub first_page_only: bool,
    /// 머리글 영역 포함
    pub header_inside: bool,
    /// 바닥글 영역 포함
    pub footer_inside: bool,
    /// 텍스트 뒤 채우기 (HWP 전용)
    pub fill_behind: bool,
    /// 적용 페이지 종류 (HWPX)
    pub page_type: PageBorderPageType,
    /// 채우기 영역 (HWPX)
    pub fill_area: PageBorderFillArea,
}

impl Default for PageBorderFill {
    fn default() -> Self {
        Self {
            border_fill_id: BorderFillId::default(),
            position: PageBorderPosition::Paper,
            offsets: Insets::default(),
            first_page_only: false,
            header_inside: false,
            footer_inside: false,
            fill_behind: false,
            page_type: PageBorderPageType::Both,
            fill_area: PageBorderFillArea::Paper,
        }
    }
}

// PageBorderPosition, PageBorderPageType, PageBorderFillArea re-exported from primitive

/// 섹션 확장 데이터
#[derive(Debug, Clone, Default)]
pub struct SectionExtensions {
    /// HWPX 마스터 페이지 참조
    pub master_page_ids: Vec<String>,
    /// HWPX 프레젠테이션 설정
    pub presentation: Option<SectionPresentation>,
    /// 섹션 가시성 설정
    pub visibility: SectionVisibility,
    /// 섹션 그리드 설정
    pub grid: SectionGrid,
    /// 줄 번호 모양 설정
    pub line_number_shape: Option<LineNumberShape>,
}

/// 섹션 가시성 설정
///
/// 이 섹션에서 특정 요소의 표시 여부를 제어합니다.
#[derive(Debug, Clone, Default)]
pub struct SectionVisibility {
    /// 머리글 숨김 (HWP)
    pub hide_header: bool,
    /// 바닥글 숨김 (HWP)
    pub hide_footer: bool,
    /// 마스터페이지 숨김 (HWP)
    pub hide_master_page: bool,
    /// 테두리 숨김 (HWP)
    pub hide_border: bool,
    /// 배경 숨김 (HWP)
    pub hide_background: bool,
    /// 쪽 번호 숨김 (HWP)
    pub hide_page_number: bool,
    /// 첫 페이지 머리말 숨김 (HWPX)
    pub hide_first_header: bool,
    /// 첫 페이지 꼬리말 숨김 (HWPX)
    pub hide_first_footer: bool,
    /// 첫 페이지 바탕쪽 숨김 (HWPX)
    pub hide_first_master_page: bool,
    /// 첫 페이지 번호 숨김 (HWPX)
    pub hide_first_page_number: bool,
    /// 첫 페이지 빈 줄 숨김 (HWPX)
    pub hide_first_empty_line: bool,
    /// 줄 번호 표시 (HWPX)
    pub show_line_number: bool,
    /// 테두리 표시 (HWPX visibility enum)
    pub border_visibility: Option<VisibilityOption>,
    /// 채우기 표시 (HWPX visibility enum)
    pub fill_visibility: Option<VisibilityOption>,
}

// VisibilityOption re-exported from primitive

/// 섹션 그리드 설정
///
/// 줄맞춤 및 원고지 형식 설정
#[derive(Debug, Clone, Default)]
pub struct SectionGrid {
    /// 세로 줄맞춤 간격 (0 = 사용 안 함)
    pub line_grid: u32,
    /// 가로 글자 줄맞춤 간격 (0 = 사용 안 함)
    pub character_grid: u32,
    /// 원고지 형식 사용 여부
    pub manuscript_format: bool,
}

/// 섹션 프레젠테이션 설정 (HWPX 전용)
#[derive(Debug, Clone)]
pub struct SectionPresentation {
    /// 슬라이드 번호
    pub slide_number: Option<u32>,
    /// 전환 효과
    pub transition: Option<String>,
}

/// 줄 번호 모양
///
/// 섹션의 줄 번호 표시 설정을 정의합니다.
#[derive(Debug, Clone)]
pub struct LineNumberShape {
    /// 번호 매기기 방식 (연속/섹션별/페이지별)
    pub restart_type: LineNumberRestartType,
    /// 표시 간격 (몇 줄마다 번호를 표시할지)
    pub count_by: u32,
    /// 본문과의 거리 (HWPUNIT)
    pub distance: HwpUnit,
    /// 시작 번호
    pub start_number: u32,
}

impl Default for LineNumberShape {
    fn default() -> Self {
        Self {
            restart_type: LineNumberRestartType::Continuous,
            count_by: 1,
            distance: HwpUnit::from_mm(5.0),
            start_number: 1,
        }
    }
}
