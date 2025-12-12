//! 페이지 설정 관련 열거형 및 타입

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::unit::HwpUnit;

/// 브레이크 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BreakType {
    /// 없음
    #[default]
    None,
    /// 페이지 나누기
    Page,
    /// 단 나누기
    Column,
    /// 섹션 나누기
    Section,
}

/// 머리글/바닥글 적용 대상
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HeaderFooterApplyTo {
    /// 양쪽 페이지
    #[default]
    Both,
    /// 짝수 페이지
    Even,
    /// 홀수 페이지
    Odd,
    /// 첫 페이지
    First,
}

/// 페이지 시작 옵션
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PageStartsOn {
    /// 양쪽 페이지
    #[default]
    Both,
    /// 짝수 페이지
    Even,
    /// 홀수 페이지
    Odd,
}

/// 줄 번호 재시작 방식
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LineNumberRestartType {
    /// 문서 전체 연속
    #[default]
    Continuous,
    /// 섹션마다 새로 시작
    RestartSection,
    /// 페이지마다 새로 시작
    RestartPage,
}

/// 페이지 번호 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PageNumberPosition {
    /// 없음
    None,
    /// 위쪽 왼쪽
    #[default]
    TopLeft,
    /// 위쪽 가운데
    TopCenter,
    /// 위쪽 오른쪽
    TopRight,
    /// 아래쪽 왼쪽
    BottomLeft,
    /// 아래쪽 가운데
    BottomCenter,
    /// 아래쪽 오른쪽
    BottomRight,
    /// 바깥쪽 위
    OutsideTop,
    /// 바깥쪽 아래
    OutsideBottom,
    /// 안쪽 위
    InsideTop,
    /// 안쪽 아래
    InsideBottom,
}

/// 제본 여백 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GutterPosition {
    /// 왼쪽
    #[default]
    Left,
    /// 오른쪽
    Right,
    /// 위쪽
    Top,
    /// 아래쪽
    Bottom,
}

impl GutterPosition {
    /// raw 값에서 생성
    pub const fn from_raw(value: u8) -> Self {
        match value & 0x03 {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::Top,
            3 => Self::Bottom,
            _ => Self::Left,
        }
    }

    /// raw 값으로 변환
    pub const fn to_raw(self) -> u8 {
        match self {
            Self::Left => 0,
            Self::Right => 1,
            Self::Top => 2,
            Self::Bottom => 3,
        }
    }
}

/// 페이지 여백
///
/// 페이지의 상하좌우 여백과 머리말/꼬리말, 제본 여백을 정의합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PageMargins {
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
    /// 제본 여백
    pub gutter: HwpUnit,
}

impl PageMargins {
    /// 새 여백 생성
    pub const fn new(
        left: HwpUnit,
        right: HwpUnit,
        top: HwpUnit,
        bottom: HwpUnit,
        header: HwpUnit,
        footer: HwpUnit,
        gutter: HwpUnit,
    ) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
            header,
            footer,
            gutter,
        }
    }

    /// 모든 방향 동일한 여백
    pub const fn uniform(margin: HwpUnit) -> Self {
        Self {
            left: margin,
            right: margin,
            top: margin,
            bottom: margin,
            header: margin,
            footer: margin,
            gutter: HwpUnit::ZERO,
        }
    }

    /// 0 여백
    pub const ZERO: Self = Self {
        left: HwpUnit::ZERO,
        right: HwpUnit::ZERO,
        top: HwpUnit::ZERO,
        bottom: HwpUnit::ZERO,
        header: HwpUnit::ZERO,
        footer: HwpUnit::ZERO,
        gutter: HwpUnit::ZERO,
    };
}

impl Default for PageMargins {
    fn default() -> Self {
        // 기본 여백 (약 30mm)
        let default_margin = HwpUnit::from_mm(30.0);
        let header_footer = HwpUnit::from_mm(15.0);
        Self {
            left: default_margin,
            right: default_margin,
            top: default_margin,
            bottom: default_margin,
            header: header_footer,
            footer: header_footer,
            gutter: HwpUnit::ZERO,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gutter_position() {
        assert_eq!(GutterPosition::from_raw(0), GutterPosition::Left);
        assert_eq!(GutterPosition::from_raw(1), GutterPosition::Right);
        assert_eq!(GutterPosition::from_raw(2), GutterPosition::Top);
        assert_eq!(GutterPosition::from_raw(3), GutterPosition::Bottom);
    }

    #[test]
    fn test_page_margins_default() {
        let margins = PageMargins::default();
        assert!(margins.left.value() > 0);
        assert!(margins.right.value() > 0);
        assert_eq!(margins.gutter, HwpUnit::ZERO);
    }
}
