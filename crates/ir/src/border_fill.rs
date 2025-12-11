//! 테두리/채우기
//!
//! 표, 셀, 문단 등의 테두리와 배경 채우기를 정의합니다.

use primitive::Color;
use primitive::BinaryDataId;
use primitive::{FillType, GradientType, HwpUnit, ImageEffect, ImageFillMode, LineType};

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
        self.fill = Fill::Solid(SolidFill { color });
        self
    }
}

/// 테두리
#[derive(Debug, Clone, Default)]
pub struct Border {
    /// 선 종류
    pub line_type: LineType,
    /// 선 두께
    pub width: HwpUnit,
    /// 선 색상
    pub color: Color,
}

impl Border {
    /// 테두리 없음
    pub fn none() -> Self {
        Self {
            line_type: LineType::None,
            width: HwpUnit::ZERO,
            color: Color::BLACK,
        }
    }

    /// 실선 테두리
    pub fn solid(width: HwpUnit, color: Color) -> Self {
        Self {
            line_type: LineType::Solid,
            width,
            color,
        }
    }
}

/// 채우기
#[derive(Debug, Clone, Default)]
pub enum Fill {
    /// 채우기 없음
    #[default]
    None,
    /// 단색 채우기
    Solid(SolidFill),
    /// 그라데이션 채우기
    Gradient(GradientFill),
    /// 이미지 채우기
    Image(ImageFill),
    /// 패턴 채우기
    Pattern(PatternFill),
}

impl Fill {
    /// 채우기 종류 반환
    pub fn fill_type(&self) -> FillType {
        match self {
            Fill::None => FillType::None,
            Fill::Solid(_) => FillType::Solid,
            Fill::Gradient(_) => FillType::Gradient,
            Fill::Image(_) => FillType::Image,
            Fill::Pattern(_) => FillType::Pattern,
        }
    }
}

/// 단색 채우기
#[derive(Debug, Clone)]
pub struct SolidFill {
    /// 채우기 색상
    pub color: Color,
}

impl SolidFill {
    /// 단색 채우기 생성
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

/// 그라데이션 채우기
#[derive(Debug, Clone)]
pub struct GradientFill {
    /// 그라데이션 종류
    pub gradient_type: GradientType,
    /// 그라데이션 각도 (0-360, 선형인 경우)
    pub angle: u16,
    /// 그라데이션 중심 X (원형인 경우, 0-100%)
    pub center_x: u8,
    /// 그라데이션 중심 Y (원형인 경우, 0-100%)
    pub center_y: u8,
    /// 색상 정지점들
    pub stops: Vec<GradientStop>,
    /// 번짐 정도 (0-255)
    pub blur: u8,
    /// 번짐 정도의 중심 (0-100)
    pub step_center: u8,
}

impl Default for GradientFill {
    fn default() -> Self {
        Self {
            gradient_type: GradientType::Linear,
            angle: 0,
            center_x: 50,
            center_y: 50,
            stops: vec![
                GradientStop {
                    position: 0,
                    color: Color::WHITE,
                },
                GradientStop {
                    position: 100,
                    color: Color::BLACK,
                },
            ],
            blur: 0,
            step_center: 50,
        }
    }
}

/// 그라데이션 색상 정지점
#[derive(Debug, Clone)]
pub struct GradientStop {
    /// 위치 (0-100)
    pub position: u8,
    /// 색상
    pub color: Color,
}

/// 이미지 채우기
#[derive(Debug, Clone)]
pub struct ImageFill {
    /// 바이너리 데이터 ID
    pub binary_id: BinaryDataId,
    /// 채우기 모드
    pub mode: ImageFillMode,
    /// 밝기 (-100 ~ 100)
    pub brightness: i8,
    /// 대비 (-100 ~ 100)
    pub contrast: i8,
    /// 이미지 효과
    pub effect: ImageEffect,
}

impl ImageFill {
    /// 이미지 채우기 생성
    pub fn new(binary_id: BinaryDataId) -> Self {
        Self {
            binary_id,
            mode: ImageFillMode::Tile,
            brightness: 0,
            contrast: 0,
            effect: ImageEffect::Original,
        }
    }
}

/// 패턴 채우기
#[derive(Debug, Clone)]
pub struct PatternFill {
    /// 패턴 종류
    pub pattern_type: PatternType,
    /// 전경색
    pub foreground: Color,
    /// 배경색
    pub background: Color,
}

impl Default for PatternFill {
    fn default() -> Self {
        Self {
            pattern_type: PatternType::Horizontal,
            foreground: Color::BLACK,
            background: Color::WHITE,
        }
    }
}

/// 패턴 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PatternType {
    /// 가로줄
    #[default]
    Horizontal,
    /// 세로줄
    Vertical,
    /// 왼쪽 아래 대각선
    DiagonalDown,
    /// 왼쪽 위 대각선
    DiagonalUp,
    /// 격자
    Grid,
    /// 대각선 격자
    DiagonalGrid,
}
