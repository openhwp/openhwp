//! 텍스트 정렬 관련 열거형

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 텍스트 정렬
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Alignment {
    /// 왼쪽 정렬
    #[default]
    Left,
    /// 가운데 정렬
    Center,
    /// 오른쪽 정렬
    Right,
    /// 양쪽 정렬
    Justify,
    /// 배분 정렬 (글자 간격 균등)
    Distribute,
    /// 나눔 정렬 (단어 간격 균등, HWP 전용)
    Divide,
}

/// 수직 정렬
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VerticalAlignment {
    /// 위쪽
    #[default]
    Top,
    /// 가운데
    Middle,
    /// 아래쪽
    Bottom,
    /// 기준선 (글자 기준선 정렬)
    Baseline,
}

/// 개체 수직 오프셋 타입 (정렬 방식)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalOffsetType {
    /// 위쪽
    #[default]
    Top,
    /// 가운데
    Center,
    /// 아래쪽
    Bottom,
    /// 안쪽 (양쪽 페이지 기준)
    Inside,
    /// 바깥쪽 (양쪽 페이지 기준)
    Outside,
}

/// 개체 수평 오프셋 타입 (정렬 방식)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HorizontalOffsetType {
    /// 왼쪽
    #[default]
    Left,
    /// 가운데
    Center,
    /// 오른쪽
    Right,
    /// 안쪽 (양쪽 페이지 기준)
    Inside,
    /// 바깥쪽 (양쪽 페이지 기준)
    Outside,
}
