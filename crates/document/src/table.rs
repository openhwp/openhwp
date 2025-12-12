//! 표 (Table)
//!
//! 문서 내 표를 정의합니다.

use ir::{BorderFillId, HwpUnit, Insets};
use primitive::VerticalAlignment;

use crate::control::ObjectCommon;
use crate::id::{CellId, ParagraphId, RowId};

/// 표
#[derive(Debug, Clone)]
pub struct Table {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 행 수
    pub row_count: u16,
    /// 열 수
    pub column_count: u16,
    /// 셀 간격
    pub cell_spacing: HwpUnit,
    /// 테두리/채우기 ID
    pub border_fill_id: Option<BorderFillId>,
    /// 행 목록
    pub rows: Vec<RowId>,
    /// 표 영역 (병합 영역 정보)
    pub zones: Vec<TableZone>,
    /// 반복할 제목 행 수
    pub header_row_count: u16,
    /// 페이지 나눔 설정
    pub page_break: TablePageBreak,
    /// 머리글 반복
    pub repeat_header: bool,
    /// 자동 크기 조정 안함 (HWPX)
    pub no_adjust: bool,
    /// 잠금 여부 (HWPX)
    pub lock: bool,
    /// 안쪽 여백 (HWPX)
    pub inside_margin: Option<Insets>,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            common: ObjectCommon::default(),
            row_count: 0,
            column_count: 0,
            cell_spacing: HwpUnit::ZERO,
            border_fill_id: None,
            rows: Vec::new(),
            zones: Vec::new(),
            header_row_count: 0,
            page_break: TablePageBreak::None,
            repeat_header: false,
            no_adjust: false,
            lock: false,
            inside_margin: None,
        }
    }
}

impl Table {
    /// 새 표 생성
    pub fn new(row_count: u16, column_count: u16) -> Self {
        Self {
            row_count,
            column_count,
            ..Default::default()
        }
    }
}

/// 표 페이지 나눔 설정
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TablePageBreak {
    /// 페이지 나눔 없음
    #[default]
    None,
    /// 셀 단위로 나눔
    Cell,
    /// 무조건 나눔
    Table,
}

/// 표 행
#[derive(Debug, Clone, Default)]
pub struct TableRow {
    /// 행 높이
    pub height: HwpUnit,
    /// 셀 목록
    pub cells: Vec<CellId>,
}

impl TableRow {
    /// 새 행 생성
    pub fn new() -> Self {
        Self::default()
    }
}

/// 표 셀
#[derive(Debug, Clone)]
pub struct TableCell {
    /// 열 인덱스
    pub column: u16,
    /// 행 인덱스
    pub row: u16,
    /// 열 병합 수
    pub column_span: u16,
    /// 행 병합 수
    pub row_span: u16,
    /// 셀 너비
    pub width: HwpUnit,
    /// 셀 높이
    pub height: HwpUnit,
    /// 안쪽 여백
    pub padding: Insets,
    /// 테두리/채우기 ID
    pub border_fill_id: Option<BorderFillId>,
    /// 세로 정렬
    pub vertical_alignment: VerticalAlignment,
    /// 셀 내용 (문단 목록)
    pub paragraphs: Vec<ParagraphId>,
    /// 병합된 셀인지 (다른 셀에 병합되어 표시되지 않음)
    pub is_merged: bool,
    /// 헤더 셀 여부 (HWPX)
    pub is_header: bool,
    /// 셀 보호 (HWPX)
    pub protect: bool,
    /// 셀 이름 (HWPX)
    pub name: Option<String>,
    /// 편집 가능 여부 (HWPX)
    pub editable: bool,
    /// 여백 지정 여부 (HWPX)
    pub has_margin: bool,
    /// 변경됨 여부 (HWPX)
    pub dirty: bool,
}

impl Default for TableCell {
    fn default() -> Self {
        Self {
            column: 0,
            row: 0,
            column_span: 1,
            row_span: 1,
            width: HwpUnit::ZERO,
            height: HwpUnit::ZERO,
            padding: Insets::ZERO,
            border_fill_id: None,
            vertical_alignment: VerticalAlignment::Top,
            paragraphs: Vec::new(),
            is_merged: false,
            is_header: false,
            protect: false,
            name: None,
            editable: true,
            has_margin: false,
            dirty: false,
        }
    }
}

impl TableCell {
    /// 새 셀 생성
    pub fn new(row: u16, column: u16) -> Self {
        Self {
            column,
            row,
            ..Default::default()
        }
    }

    /// 셀이 병합 셀의 시작점인지 확인
    pub fn is_merge_origin(&self) -> bool {
        self.column_span > 1 || self.row_span > 1
    }
}

/// 표 영역 (병합 등 영역 정보)
#[derive(Debug, Clone)]
pub struct TableZone {
    /// 시작 행
    pub start_row: u16,
    /// 시작 열
    pub start_column: u16,
    /// 끝 행
    pub end_row: u16,
    /// 끝 열
    pub end_column: u16,
    /// 테두리/채우기 ID
    pub border_fill_id: Option<BorderFillId>,
}
