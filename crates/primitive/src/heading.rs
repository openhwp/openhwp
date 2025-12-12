//! 문단 머리 관련 타입
//!
//! 문단 머리의 종류(개요, 번호, 글머리표)를 정의합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 문단 머리 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum HeadingType {
    /// 없음
    #[default]
    None = 0,
    /// 개요
    Outline = 1,
    /// 번호
    Number = 2,
    /// 글머리표
    Bullet = 3,
}

impl HeadingType {
    /// raw 값에서 생성
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Outline,
            2 => Self::Number,
            3 => Self::Bullet,
            _ => Self::None,
        }
    }

    /// raw 값으로 반환
    pub const fn as_raw(self) -> u8 {
        self as u8
    }

    /// 번호 매기기가 있는지 확인
    pub const fn has_numbering(self) -> bool {
        matches!(self, Self::Outline | Self::Number)
    }

    /// 글머리표인지 확인
    pub const fn is_bullet(self) -> bool {
        matches!(self, Self::Bullet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_type() {
        assert_eq!(HeadingType::from_raw(0), HeadingType::None);
        assert_eq!(HeadingType::from_raw(1), HeadingType::Outline);
        assert_eq!(HeadingType::from_raw(2), HeadingType::Number);
        assert_eq!(HeadingType::from_raw(3), HeadingType::Bullet);
        assert_eq!(HeadingType::from_raw(99), HeadingType::None);
    }

    #[test]
    fn test_has_numbering() {
        assert!(!HeadingType::None.has_numbering());
        assert!(HeadingType::Outline.has_numbering());
        assert!(HeadingType::Number.has_numbering());
        assert!(!HeadingType::Bullet.has_numbering());
    }

    #[test]
    fn test_is_bullet() {
        assert!(!HeadingType::None.is_bullet());
        assert!(!HeadingType::Outline.is_bullet());
        assert!(!HeadingType::Number.is_bullet());
        assert!(HeadingType::Bullet.is_bullet());
    }
}
