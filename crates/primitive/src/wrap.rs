//! 텍스트 감싸기 관련 열거형

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 텍스트 감싸기 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextWrapType {
    /// 어울림 (글자처럼 취급)
    #[default]
    Inline,
    /// 자리 차지
    Square,
    /// 빈 공간 채우기
    Tight,
    /// 글 뒤로
    Behind,
    /// 글 앞으로
    InFront,
}

/// 텍스트 감싸기 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextWrapSide {
    /// 양쪽
    #[default]
    Both,
    /// 왼쪽만
    Left,
    /// 오른쪽만
    Right,
    /// 큰 쪽만
    Largest,
}
