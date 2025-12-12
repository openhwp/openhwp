//! 섹션/구역 관련 타입
//!
//! 문서의 섹션(구역) 정의에 사용되는 타입들입니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Color, HwpUnit, LineType};

/// 단 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ColumnDirection {
    /// 왼쪽에서 오른쪽
    #[default]
    LeftToRight,
    /// 오른쪽에서 왼쪽
    RightToLeft,
    /// 맞쪽 페이지 (홀수/짝수 페이지 다르게)
    FacingPages,
}

/// 단 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ColumnType {
    /// 일반
    #[default]
    Normal,
    /// 배분
    Distribute,
    /// 평행
    Parallel,
}

/// 단 구분선 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ColumnSeparatorType {
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

/// 단 구분선
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ColumnSeparator {
    /// 선 종류
    pub line_type: LineType,
    /// 선 두께 (0.1mm 단위)
    pub thickness: u8,
    /// 선 색상
    pub color: Color,
}

impl Default for ColumnSeparator {
    fn default() -> Self {
        Self {
            line_type: LineType::Solid,
            thickness: 1,
            color: Color::BLACK,
        }
    }
}

/// 페이지 테두리 위치 기준
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PageBorderPosition {
    /// 용지 기준
    #[default]
    Paper,
    /// 본문 영역 기준
    Body,
}

/// 페이지 테두리 적용 페이지 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PageBorderPageType {
    /// 모든 페이지
    #[default]
    Both,
    /// 짝수 페이지만
    Even,
    /// 홀수 페이지만
    Odd,
    /// 첫 페이지만
    First,
}

/// 페이지 테두리 채우기 영역
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PageBorderFillArea {
    /// 용지 전체
    #[default]
    Paper,
    /// 본문 영역
    Body,
    /// 꼬리말/머리말 제외
    Content,
}

/// 가시성 옵션 (HWPX 전용)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VisibilityOption {
    /// 숨김
    Hide,
    /// 표시
    Show,
    /// 첫 페이지만 숨김
    HideFirstPage,
    /// 첫 페이지만 표시
    ShowFirstPage,
}

/// 섹션 시작 번호 정보
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SectionStartNumber {
    /// 쪽 시작 번호 (0이면 앞 구역에 이어서 번호를 매김, 1 이상이면 임의의 번호로 시작)
    pub page: u32,
    /// 그림 시작 번호
    pub picture: u32,
    /// 표 시작 번호
    pub table: u32,
    /// 수식 시작 번호
    pub equation: u32,
    /// 각주 시작 번호
    pub footnote: Option<u32>,
    /// 미주 시작 번호
    pub endnote: Option<u32>,
}

/// 섹션 가시성 설정
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

/// 섹션 그리드 설정
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SectionGrid {
    /// 세로 줄맞춤 간격 (0 = 사용 안 함)
    pub line_grid: u32,
    /// 가로 글자 줄맞춤 간격 (0 = 사용 안 함)
    pub character_grid: u32,
    /// 원고지 형식 사용 여부
    pub manuscript_format: bool,
}

/// 줄 번호 모양
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LineNumberShape {
    /// 번호 매기기 방식 (연속/섹션별/페이지별)
    pub restart_type: crate::LineNumberRestartType,
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
            restart_type: crate::LineNumberRestartType::Continuous,
            count_by: 1,
            distance: HwpUnit::from_mm(5.0),
            start_number: 1,
        }
    }
}
