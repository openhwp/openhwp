//! 글맵시 관련 타입
//!
//! 글맵시(TextArt/WordArt)의 속성을 정의합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 글맵시 글꼴 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextArtFontType {
    /// TrueType Font
    #[default]
    TTF,
    /// Hancom TrueType Font
    HTF,
}

/// 글맵시 정렬
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextArtAlignment {
    /// 왼쪽 정렬
    #[default]
    Left,
    /// 가운데 정렬
    Center,
    /// 오른쪽 정렬
    Right,
    /// 양쪽 정렬
    Full,
}

/// 글맵시 추가 속성
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextArtProperties {
    /// 원시 데이터
    pub raw_data: Option<String>,
}
