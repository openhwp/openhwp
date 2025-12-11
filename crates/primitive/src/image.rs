//! 이미지 효과 관련 열거형

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 이미지 효과
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ImageEffect {
    /// 원본
    #[default]
    Original,
    /// 흑백
    Grayscale,
    /// 흑백 2색
    BlackWhite,
    /// 패턴
    Pattern,
}

/// 이미지 뒤집기
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageFlip {
    /// 없음
    #[default]
    None,
    /// 가로 뒤집기
    Horizontal,
    /// 세로 뒤집기
    Vertical,
    /// 양쪽 뒤집기
    Both,
}
