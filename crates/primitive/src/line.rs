//! 선 스타일 관련 열거형

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 선 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LineType {
    /// 없음
    None,
    /// 실선
    #[default]
    Solid,
    /// 파선
    Dash,
    /// 점선
    Dot,
    /// 파선-점선
    DashDot,
    /// 파선-점선-점선
    DashDotDot,
    /// 긴 파선
    LongDash,
    /// 이중선
    Double,
    /// 삼중선
    Triple,
    /// 물결
    Wave,
    /// 이중 물결
    DoubleWave,
    /// 두꺼운 삼중선
    ThickThinLarge,
    /// 얇은 삼중선
    ThinThickLarge,
    /// 원형
    Circle,
}

/// 선 끝 모양
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineCap {
    /// 평평
    #[default]
    Flat,
    /// 둥근
    Round,
    /// 사각
    Square,
}

/// 선 외곽선 스타일 (HWPX 전용)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineOutlineStyle {
    /// 표준
    #[default]
    Normal,
    /// 바깥쪽
    Outer,
    /// 안쪽
    Inner,
}

/// 줄 나눔 방식 (텍스트 박스 내부)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineWrap {
    /// 줄 바꿈 (자동 줄 바꿈)
    #[default]
    Break,
    /// 줄 바꿈 없이 축소
    Squeeze,
    /// 줄 바꿈 없이 유지
    Keep,
}
