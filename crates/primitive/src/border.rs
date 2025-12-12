//! 테두리 관련 타입
//!
//! 표, 셀, 문단 등의 테두리를 정의합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Color, HwpUnit, LineType};

/// 테두리 정의
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Border {
    /// 선 종류
    pub line_type: LineType,
    /// 선 두께
    pub width: HwpUnit,
    /// 선 색상
    pub color: Color,
}

impl Border {
    /// 테두리 없음
    pub const fn none() -> Self {
        Self {
            line_type: LineType::None,
            width: HwpUnit::ZERO,
            color: Color::BLACK,
        }
    }

    /// 실선 테두리
    pub const fn solid(width: HwpUnit, color: Color) -> Self {
        Self {
            line_type: LineType::Solid,
            width,
            color,
        }
    }

    /// 테두리가 없는지 확인
    pub const fn is_none(&self) -> bool {
        matches!(self.line_type, LineType::None) || self.width.value() == 0
    }
}

/// 대각선 종류 (표 셀)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DiagonalType {
    /// 없음
    #[default]
    None,
    /// 슬래시 (/)
    Slash,
    /// 역슬래시 (\)
    BackSlash,
    /// 교차 (X)
    Cross,
}

impl DiagonalType {
    /// raw 값에서 생성 (HWP용)
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Slash,
            2 => Self::BackSlash,
            3 => Self::Cross,
            _ => Self::None,
        }
    }
}
