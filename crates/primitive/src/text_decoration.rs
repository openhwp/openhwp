//! 텍스트 장식 관련 열거형

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 밑줄 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnderlineType {
    /// 없음
    #[default]
    None,
    /// 단일 밑줄
    Single,
    /// 이중 밑줄
    Double,
    /// 굵은 밑줄
    Thick,
    /// 점선 밑줄
    Dotted,
    /// 파선 밑줄
    Dash,
    /// 파선-점선 밑줄
    DashDot,
    /// 파선-점선-점선 밑줄
    DashDotDot,
    /// 물결 밑줄
    Wave,
}

/// 밑줄 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnderlinePosition {
    /// 아래
    #[default]
    Bottom,
    /// 위
    Top,
}

/// 취소선 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StrikethroughType {
    /// 없음
    #[default]
    None,
    /// 단일 취소선
    Single,
    /// 이중 취소선
    Double,
}

/// 강조점 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EmphasisType {
    /// 없음
    #[default]
    None,
    /// 점
    Dot,
    /// 원
    Circle,
    /// 속이 빈 원
    CircleOpen,
    /// 쉼표
    Comma,
    /// 콜론 (겹점)
    Colon,
    /// 체크 기호 (ˇ)
    Caron,
    /// 물결 기호 (~)
    Tilde,
}

/// 외곽선 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OutlineType {
    /// 없음
    #[default]
    None,
    /// 외곽선
    Outline,
    /// 그림자
    Shadow,
    /// 양각
    Emboss,
    /// 음각
    Engrave,
}

/// 그림자 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ShadowType {
    /// 없음
    #[default]
    None,
    /// 왼쪽 위
    TopLeft,
    /// 오른쪽 위
    TopRight,
    /// 왼쪽 아래
    BottomLeft,
    /// 오른쪽 아래
    BottomRight,
    /// 왼쪽 위 (불연속)
    TopLeftDiscrete,
    /// 왼쪽 위 (연속)
    TopLeftContinuous,
    /// 오른쪽 위 (불연속)
    TopRightDiscrete,
    /// 오른쪽 위 (연속)
    TopRightContinuous,
    /// 왼쪽 아래 (불연속)
    BottomLeftDiscrete,
    /// 왼쪽 아래 (연속)
    BottomLeftContinuous,
    /// 오른쪽 아래 (불연속)
    BottomRightDiscrete,
    /// 오른쪽 아래 (연속)
    BottomRightContinuous,
}
