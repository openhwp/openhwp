//! 테두리/채우기
//!
//! 표, 셀, 문단 등의 테두리와 배경 채우기를 정의합니다.

// Re-export primitive types for convenience
use primitive::Color;
pub use primitive::{
    Border, DiagonalType, Fill, FillType, GradientFill, GradientStop, GradientType, ImageFill,
    ImageFillMode, PatternFill, PatternType, SolidFill,
};

/// 테두리/채우기 정의
#[derive(Debug, Clone, Default)]
pub struct BorderFill {
    /// 왼쪽 테두리
    pub left: Border,
    /// 오른쪽 테두리
    pub right: Border,
    /// 위쪽 테두리
    pub top: Border,
    /// 아래쪽 테두리
    pub bottom: Border,
    /// 대각선 (왼쪽 위 → 오른쪽 아래)
    pub diagonal_down: Option<Border>,
    /// 대각선 (왼쪽 아래 → 오른쪽 위)
    pub diagonal_up: Option<Border>,
    /// 채우기
    pub fill: Fill,
    /// 3D 효과
    pub is_3d: bool,
    /// 그림자 효과
    pub has_shadow: bool,
}

impl BorderFill {
    /// 빈 테두리/채우기 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 모든 테두리를 동일하게 설정
    pub fn with_all_borders(mut self, border: Border) -> Self {
        self.left = border.clone();
        self.right = border.clone();
        self.top = border.clone();
        self.bottom = border;
        self
    }

    /// 단색 채우기 설정
    pub fn with_solid_fill(mut self, color: Color) -> Self {
        self.fill = Fill::Solid(SolidFill::new(color));
        self
    }
}
