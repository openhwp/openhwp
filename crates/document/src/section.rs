//! 섹션
//!
//! 문서의 섹션(구역)을 정의합니다.

use ir::{BorderFillId, HwpUnit, Insets};
use primitive::{LineType, NumberFormat};

use crate::id::{HeaderFooterId, ParagraphId};

/// 섹션
#[derive(Debug, Clone, Default)]
pub struct Section {
    /// 페이지 정의
    pub page: PageDefinition,
    /// 단 정의
    pub columns: ColumnDefinition,
    /// 머리글 목록
    pub headers: Vec<HeaderFooterId>,
    /// 바닥글 목록
    pub footers: Vec<HeaderFooterId>,
    /// 각주 모양
    pub footnote_shape: Option<NoteShape>,
    /// 미주 모양
    pub endnote_shape: Option<NoteShape>,
    /// 페이지 테두리/배경
    pub page_border_fill: Option<PageBorderFill>,
    /// 문단 목록
    pub paragraphs: Vec<ParagraphId>,
    /// 시작 번호 설정
    pub start_number: Option<SectionStartNumber>,
    /// 줄 번호 모양
    pub line_number_shape: Option<LineNumberShape>,
}

/// 페이지 정의
#[derive(Debug, Clone)]
pub struct PageDefinition {
    /// 용지 너비
    pub width: HwpUnit,
    /// 용지 높이
    pub height: HwpUnit,
    /// 여백
    pub margin: PageMargin,
    /// 제본 여백
    pub gutter: HwpUnit,
    /// 용지 방향
    pub orientation: PageOrientation,
}

impl Default for PageDefinition {
    fn default() -> Self {
        Self {
            // A4 크기 (210mm x 297mm)
            width: HwpUnit::from_mm(210.0),
            height: HwpUnit::from_mm(297.0),
            margin: PageMargin::default(),
            gutter: HwpUnit::ZERO,
            orientation: PageOrientation::Portrait,
        }
    }
}

/// 페이지 여백
#[derive(Debug, Clone)]
pub struct PageMargin {
    /// 왼쪽 여백
    pub left: HwpUnit,
    /// 오른쪽 여백
    pub right: HwpUnit,
    /// 위쪽 여백
    pub top: HwpUnit,
    /// 아래쪽 여백
    pub bottom: HwpUnit,
    /// 머리말 여백
    pub header: HwpUnit,
    /// 꼬리말 여백
    pub footer: HwpUnit,
}

impl Default for PageMargin {
    fn default() -> Self {
        Self {
            left: HwpUnit::from_mm(30.0),
            right: HwpUnit::from_mm(30.0),
            top: HwpUnit::from_mm(20.0),
            bottom: HwpUnit::from_mm(15.0),
            header: HwpUnit::from_mm(15.0),
            footer: HwpUnit::from_mm(15.0),
        }
    }
}

/// 용지 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PageOrientation {
    /// 세로
    #[default]
    Portrait,
    /// 가로
    Landscape,
}

/// 단 정의
#[derive(Debug, Clone, Default)]
pub struct ColumnDefinition {
    /// 단 종류
    pub column_type: ColumnType,
    /// 단 수
    pub count: u16,
    /// 단 간격
    pub gap: HwpUnit,
    /// 구분선
    pub separator: Option<ColumnSeparator>,
    /// 각 단의 너비 (count만큼)
    pub widths: Vec<HwpUnit>,
}

/// 단 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnType {
    /// 일반
    #[default]
    Normal,
    /// 배분
    Distribute,
    /// 평행
    Parallel,
}

/// 단 구분선
#[derive(Debug, Clone)]
pub struct ColumnSeparator {
    /// 선 종류
    pub line_type: LineType,
    /// 선 두께
    pub width: HwpUnit,
    /// 선 색상
    pub color: ir::Color,
}

/// 각주/미주 모양
#[derive(Debug, Clone)]
pub struct NoteShape {
    /// 번호 형식
    pub number_format: NumberFormat,
    /// 번호 위치
    pub number_position: NoteNumberPosition,
    /// 번호 매기기 방식
    pub numbering: NoteNumbering,
    /// 구분선
    pub separator_line: Option<NoteLine>,
    /// 구분선 위치
    pub separator_position: HwpUnit,
    /// 구분선 아래 여백
    pub space_below: HwpUnit,
    /// 연속 번호 매기기
    pub continue_numbering: bool,
}

/// 각주 번호 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteNumberPosition {
    /// 위 첨자
    #[default]
    Superscript,
    /// 아래 첨자
    Subscript,
}

/// 각주 번호 매기기
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteNumbering {
    /// 문서 전체
    #[default]
    Continuous,
    /// 각 쪽마다
    PerPage,
    /// 각 섹션마다
    PerSection,
}

/// 각주 구분선
#[derive(Debug, Clone)]
pub struct NoteLine {
    /// 선 종류
    pub line_type: LineType,
    /// 선 두께
    pub width: HwpUnit,
    /// 선 색상
    pub color: ir::Color,
    /// 선 길이 (퍼센트)
    pub length: ir::Percent,
}

/// 페이지 테두리/배경
#[derive(Debug, Clone)]
pub struct PageBorderFill {
    /// 테두리/배경 ID
    pub border_fill_id: BorderFillId,
    /// 테두리 위치
    pub position: PageBorderPosition,
    /// 여백
    pub offset: Insets,
    /// 첫 페이지에만 적용
    pub first_page_only: bool,
    /// 머리말 포함
    pub include_header: bool,
    /// 꼬리말 포함
    pub include_footer: bool,
}

/// 페이지 테두리 위치 기준
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PageBorderPosition {
    /// 용지 기준
    #[default]
    Paper,
    /// 본문 기준
    Body,
}

/// 섹션 시작 번호
#[derive(Debug, Clone, Default)]
pub struct SectionStartNumber {
    /// 페이지 번호
    pub page: Option<u32>,
    /// 각주 번호
    pub footnote: Option<u32>,
    /// 미주 번호
    pub endnote: Option<u32>,
    /// 그림 번호
    pub picture: Option<u32>,
    /// 표 번호
    pub table: Option<u32>,
    /// 수식 번호
    pub equation: Option<u32>,
}

/// 줄 번호 모양
#[derive(Debug, Clone)]
pub struct LineNumberShape {
    /// 줄 번호 표시 간격
    pub interval: u16,
    /// 시작 번호
    pub start_number: u32,
    /// 본문과의 거리
    pub distance: HwpUnit,
}
