//! 방향 관련 열거형

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 텍스트 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextDirection {
    /// 가로 (왼쪽에서 오른쪽)
    #[default]
    Horizontal,
    /// 세로 (위에서 아래)
    Vertical,
    /// 세로 (오른쪽에서 왼쪽)
    VerticalRightToLeft,
    /// 오른쪽에서 왼쪽
    RightToLeft,
}

/// 용지 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PageOrientation {
    /// 세로
    #[default]
    Portrait,
    /// 가로
    Landscape,
}
