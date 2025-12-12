//! 채우기 관련 열거형 및 구조체
//!
//! 테두리/배경 채우기를 정의합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{BinaryDataId, Color, HwpUnit, ImageEffect};

/// 채우기 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FillType {
    /// 없음
    #[default]
    None,
    /// 단색
    Solid,
    /// 그라데이션
    Gradient,
    /// 이미지
    Image,
    /// 패턴
    Pattern,
}

impl FillType {
    /// raw 값에서 생성 (HWP용, 비트 플래그)
    pub const fn from_raw_hwp(value: u32) -> Self {
        if value & 0x01 != 0 {
            Self::Solid
        } else if value & 0x02 != 0 {
            Self::Image
        } else if value & 0x04 != 0 {
            Self::Gradient
        } else {
            Self::None
        }
    }
}

/// 채우기
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    /// 채우기가 없는지 확인
    pub fn is_none(&self) -> bool {
        matches!(self, Fill::None)
    }
}

/// 단색 채우기
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SolidFill {
    /// 채우기 색상
    pub color: Color,
    /// 투명도 (0 = 투명, 255 = 불투명)
    pub alpha: u8,
}

impl SolidFill {
    /// 단색 채우기 생성
    pub fn new(color: Color) -> Self {
        Self { color, alpha: 255 }
    }

    /// 투명도 설정
    pub fn with_alpha(mut self, alpha: u8) -> Self {
        self.alpha = alpha;
        self
    }
}

impl Default for SolidFill {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            alpha: 255,
        }
    }
}

/// 그라데이션 채우기
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GradientFill {
    /// 그라데이션 종류
    pub gradient_type: GradientType,
    /// 그라데이션 각도 (0-360, 선형인 경우)
    pub angle: u16,
    /// 그라데이션 중심 X (0-100%)
    pub center_x: u8,
    /// 그라데이션 중심 Y (0-100%)
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GradientStop {
    /// 위치 (0-100)
    pub position: u8,
    /// 색상
    pub color: Color,
}

impl GradientStop {
    /// 정지점 생성
    pub fn new(position: u8, color: Color) -> Self {
        Self { position, color }
    }
}

/// 이미지 채우기
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    /// 오프셋 X
    pub offset_x: HwpUnit,
    /// 오프셋 Y
    pub offset_y: HwpUnit,
    /// 크기 (가로, 세로) - 0이면 원본 크기
    pub size: Option<(HwpUnit, HwpUnit)>,
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
            offset_x: HwpUnit::ZERO,
            offset_y: HwpUnit::ZERO,
            size: None,
        }
    }
}

/// 패턴 채우기
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

/// 그라데이션 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GradientType {
    /// 선형
    #[default]
    Linear,
    /// 원형
    Radial,
    /// 사각형
    Square,
    /// 원뿔형
    Conical,
}

impl GradientType {
    /// raw 값에서 생성 (HWP용)
    pub const fn from_raw_hwp(value: i16) -> Self {
        match value {
            1 => Self::Linear,
            2 => Self::Radial,
            3 => Self::Conical,
            4 => Self::Square,
            _ => Self::Linear,
        }
    }
}

/// 패턴 종류 (해칭)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PatternType {
    /// 없음
    None,
    /// 가로줄
    #[default]
    Horizontal,
    /// 세로줄
    Vertical,
    /// 역슬래시 (왼쪽 아래 대각선, \)
    BackSlash,
    /// 슬래시 (왼쪽 위 대각선, /)
    Slash,
    /// 십자 (격자)
    Cross,
    /// 대각선 십자 (대각선 격자)
    CrossDiagonal,
}

impl PatternType {
    /// raw 값에서 생성 (HWP용)
    pub const fn from_raw_hwp(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Horizontal,
            2 => Self::Vertical,
            3 => Self::BackSlash,
            4 => Self::Slash,
            5 => Self::Cross,
            6 => Self::CrossDiagonal,
            _ => Self::None,
        }
    }
}

/// 이미지 채우기 모드
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ImageFillMode {
    /// 바둑판식 배열 (전체)
    #[default]
    Tile,
    /// 바둑판식 배열 (가로/위)
    TileHorizontalTop,
    /// 바둑판식 배열 (가로/아래)
    TileHorizontalBottom,
    /// 바둑판식 배열 (세로/왼쪽)
    TileVerticalLeft,
    /// 바둑판식 배열 (세로/오른쪽)
    TileVerticalRight,
    /// 늘이기
    Stretch,
    /// 가운데
    Center,
    /// 가운데 위
    CenterTop,
    /// 가운데 아래
    CenterBottom,
    /// 가운데 왼쪽
    CenterLeft,
    /// 왼쪽 위
    TopLeft,
    /// 왼쪽 아래
    BottomLeft,
    /// 가운데 오른쪽
    CenterRight,
    /// 오른쪽 위
    TopRight,
    /// 오른쪽 아래
    BottomRight,
    /// 원본 크기
    Original,
}

impl ImageFillMode {
    /// raw 값에서 생성 (HWP용)
    pub const fn from_raw_hwp(value: u8) -> Self {
        match value {
            0 => Self::Tile,
            1 => Self::TileHorizontalTop,
            2 => Self::TileHorizontalBottom,
            3 => Self::TileVerticalLeft,
            4 => Self::TileVerticalRight,
            5 => Self::Stretch,
            6 => Self::Center,
            7 => Self::CenterTop,
            8 => Self::CenterBottom,
            9 => Self::CenterLeft,
            10 => Self::TopLeft,
            11 => Self::BottomLeft,
            12 => Self::CenterRight,
            13 => Self::TopRight,
            14 => Self::BottomRight,
            15 => Self::Original,
            _ => Self::Tile,
        }
    }
}

/// 채우기 영역 종류 (페이지 테두리)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FillAreaType {
    /// 종이
    #[default]
    Paper,
    /// 쪽
    Page,
    /// 테두리
    Border,
}
