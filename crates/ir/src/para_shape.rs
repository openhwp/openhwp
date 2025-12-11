//! 문단 모양
//!
//! 문단의 정렬, 들여쓰기, 줄간격 등 문단 서식을 정의합니다.

use primitive::{BorderFillId, TabDefId};
use primitive::{Alignment, HeadingType, HwpUnit, LineBreakKorean, LineBreakLatin, Percent, TabLeader, TabType, VerticalAlignment};

/// 문단 모양 정의
#[derive(Debug, Clone)]
pub struct ParaShape {
    /// 정렬
    pub alignment: Alignment,

    /// 왼쪽 여백
    pub margin_left: HwpUnit,

    /// 오른쪽 여백
    pub margin_right: HwpUnit,

    /// 첫 줄 들여쓰기 (양수: 들여쓰기, 음수: 내어쓰기)
    pub first_line_indent: HwpUnit,

    /// 문단 앞 간격
    pub space_before: HwpUnit,

    /// 문단 뒤 간격
    pub space_after: HwpUnit,

    /// 줄 간격
    pub line_spacing: LineSpacing,

    /// 탭 정의 ID
    pub tab_def_id: Option<TabDefId>,

    /// 테두리/배경 ID
    pub border_fill_id: Option<BorderFillId>,

    /// 한글 줄 나눔
    pub line_break_korean: LineBreakKorean,

    /// 영어 줄 나눔
    pub line_break_latin: LineBreakLatin,

    /// 줄 끝 공백 포함
    pub snap_to_grid: bool,

    /// 한 줄로 입력
    pub suppress_line_numbers: bool,

    /// 외톨이줄 방지
    pub widow_orphan_control: bool,

    /// 다음 문단과 함께
    pub keep_with_next: bool,

    /// 문단 보호
    pub keep_lines: bool,

    /// 문단 앞에서 페이지 나눔
    pub page_break_before: bool,

    /// 세로 정렬 (글자 기준)
    pub vertical_alignment: VerticalAlignment,

    /// 자동 줄 간격에서 글꼴 기준 비율
    pub auto_line_height_ratio: Percent,

    /// 문단 머리 기호/번호
    pub numbering: Option<ParagraphNumbering>,

    /// 문단 테두리 설정
    pub border: Option<ParagraphBorder>,

    /// 한글/영문 자동 간격
    pub auto_spacing_east_asian_english: bool,

    /// 한글/숫자 자동 간격
    pub auto_spacing_east_asian_number: bool,
}

impl Default for ParaShape {
    fn default() -> Self {
        Self {
            alignment: Alignment::Justify,
            margin_left: HwpUnit::ZERO,
            margin_right: HwpUnit::ZERO,
            first_line_indent: HwpUnit::ZERO,
            space_before: HwpUnit::ZERO,
            space_after: HwpUnit::ZERO,
            line_spacing: LineSpacing::default(),
            tab_def_id: None,
            border_fill_id: None,
            line_break_korean: LineBreakKorean::Word,
            line_break_latin: LineBreakLatin::Word,
            snap_to_grid: false,
            suppress_line_numbers: false,
            widow_orphan_control: false,
            keep_with_next: false,
            keep_lines: false,
            page_break_before: false,
            vertical_alignment: VerticalAlignment::default(),
            auto_line_height_ratio: Percent::new(100.0),
            numbering: None,
            border: None,
            auto_spacing_east_asian_english: false,
            auto_spacing_east_asian_number: false,
        }
    }
}

impl ParaShape {
    /// 기본 문단 모양 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 정렬 설정
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// 왼쪽 여백 설정
    pub fn with_margin_left(mut self, margin: HwpUnit) -> Self {
        self.margin_left = margin;
        self
    }

    /// 오른쪽 여백 설정
    pub fn with_margin_right(mut self, margin: HwpUnit) -> Self {
        self.margin_right = margin;
        self
    }

    /// 첫 줄 들여쓰기 설정
    pub fn with_first_line_indent(mut self, indent: HwpUnit) -> Self {
        self.first_line_indent = indent;
        self
    }
}

/// 줄 간격
#[derive(Debug, Clone)]
pub struct LineSpacing {
    /// 줄 간격 종류
    pub spacing_type: LineSpacingType,
    /// 줄 간격 값
    pub value: LineSpacingValue,
}

impl Default for LineSpacing {
    fn default() -> Self {
        Self {
            spacing_type: LineSpacingType::Percent,
            value: LineSpacingValue::Percent(Percent::new(160.0)),
        }
    }
}

impl LineSpacing {
    /// 비율 줄 간격 (예: 160%)
    pub fn percent(value: f64) -> Self {
        Self {
            spacing_type: LineSpacingType::Percent,
            value: LineSpacingValue::Percent(Percent::new(value)),
        }
    }

    /// 고정 줄 간격
    pub fn fixed(value: HwpUnit) -> Self {
        Self {
            spacing_type: LineSpacingType::Fixed,
            value: LineSpacingValue::Fixed(value),
        }
    }

    /// 최소 줄 간격
    pub fn at_least(value: HwpUnit) -> Self {
        Self {
            spacing_type: LineSpacingType::AtLeast,
            value: LineSpacingValue::Fixed(value),
        }
    }
}

/// 줄 간격 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineSpacingType {
    /// 비율 (글자 크기 대비)
    #[default]
    Percent,
    /// 고정값
    Fixed,
    /// 최소값
    AtLeast,
    /// 글꼴 기준 비율
    FontBased,
}

/// 줄 간격 값
#[derive(Debug, Clone)]
pub enum LineSpacingValue {
    /// 비율
    Percent(Percent),
    /// 고정값
    Fixed(HwpUnit),
}

impl Default for LineSpacingValue {
    fn default() -> Self {
        Self::Percent(Percent::new(160.0))
    }
}

/// 문단 번호/기호
#[derive(Debug, Clone)]
pub struct ParagraphNumbering {
    /// 문단 머리 종류 (None/Outline/Number/Bullet)
    pub heading_type: HeadingType,
    /// 번호 매기기 정의 ID
    pub numbering_id: Option<u32>,
    /// 글머리 기호 정의 ID
    pub bullet_id: Option<u32>,
    /// 수준 (1부터 시작, 개요 수준)
    pub level: u8,
}

/// 탭 정의
#[derive(Debug, Clone)]
pub struct TabDef {
    /// 탭 위치들
    pub tabs: Vec<Tab>,
    /// 자동 탭 간격 (없으면 비활성)
    pub auto_tab_interval: Option<HwpUnit>,
}

impl Default for TabDef {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            auto_tab_interval: Some(HwpUnit::new(800)), // 8pt (1pt = 100 HwpUnit)
        }
    }
}

/// 탭
#[derive(Debug, Clone)]
pub struct Tab {
    /// 탭 위치
    pub position: HwpUnit,
    /// 탭 종류
    pub tab_type: TabType,
    /// 채움선 종류
    pub leader: TabLeader,
}

/// 문단 테두리 설정
#[derive(Debug, Clone)]
pub struct ParagraphBorder {
    /// 테두리/배경 참조
    pub border_fill_id_ref: BorderFillId,
    /// 왼쪽 간격
    pub offset_left: HwpUnit,
    /// 오른쪽 간격
    pub offset_right: HwpUnit,
    /// 위쪽 간격
    pub offset_top: HwpUnit,
    /// 아래쪽 간격
    pub offset_bottom: HwpUnit,
    /// 테두리 연결
    pub connect: bool,
    /// 여백 무시
    pub ignore_margin: bool,
}
