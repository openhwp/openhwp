//! 수식 관련 타입
//!
//! 수식(Equation)의 속성을 정의합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 수식 라인 모드
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EquationLineMode {
    /// 기준선 정렬
    #[default]
    Baseline,
    /// 중앙 정렬
    Center,
    /// 하단 정렬
    Bottom,
    /// 상단 정렬
    Top,
}
