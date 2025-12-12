//! 그림
//!
//! 문서 내 이미지를 정의합니다.

use crate::control::ObjectCommon;
use primitive::BinaryDataId;
use primitive::Color;
use primitive::{HwpUnit, ImageEffect, ImageFlip, Insets, LineType, Size};

/// 그림
#[derive(Debug, Clone)]
pub struct Picture {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 바이너리 데이터 ID
    pub binary_id: BinaryDataId,
    /// 원본 크기
    pub original_size: Size,
    /// 자르기
    pub crop: ImageCrop,
    /// 뒤집기
    pub flip: ImageFlip,
    /// 회전 각도 (도)
    pub rotation: f64,
    /// 이미지 효과
    pub effect: ImageEffect,
    /// 밝기 (-100 ~ 100)
    pub brightness: i8,
    /// 대비 (-100 ~ 100)
    pub contrast: i8,
    /// 투명도 (0.0 ~ 1.0)
    pub alpha: f64,
    /// 투명 색상
    pub transparent_color: Option<Color>,
    /// 테두리
    pub border: Option<PictureBorder>,
    /// 그림자
    pub shadow: Option<PictureShadow>,
    /// 안쪽 여백
    pub inside_margin: Insets,
}

impl Picture {
    /// 그림 생성
    pub fn new(binary_id: BinaryDataId) -> Self {
        Self {
            common: ObjectCommon::default(),
            binary_id,
            original_size: Size::ZERO,
            crop: ImageCrop::default(),
            flip: ImageFlip::None,
            rotation: 0.0,
            effect: ImageEffect::Original,
            brightness: 0,
            contrast: 0,
            alpha: 1.0,
            transparent_color: None,
            border: None,
            shadow: None,
            inside_margin: Insets::ZERO,
        }
    }
}

/// 이미지 자르기
#[derive(Debug, Clone, Default)]
pub struct ImageCrop {
    /// 왼쪽 자르기
    pub left: HwpUnit,
    /// 오른쪽 자르기
    pub right: HwpUnit,
    /// 위쪽 자르기
    pub top: HwpUnit,
    /// 아래쪽 자르기
    pub bottom: HwpUnit,
}

impl ImageCrop {
    /// 자르기 없음
    pub fn none() -> Self {
        Self::default()
    }

    /// 모든 방향 동일하게 자르기
    pub const fn all(value: HwpUnit) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    /// 자르기가 있는지 확인
    pub const fn has_crop(&self) -> bool {
        self.left.value() != 0
            || self.right.value() != 0
            || self.top.value() != 0
            || self.bottom.value() != 0
    }
}

/// 그림 테두리
#[derive(Debug, Clone)]
pub struct PictureBorder {
    /// 선 종류
    pub line_type: LineType,
    /// 선 두께
    pub width: HwpUnit,
    /// 선 색상
    pub color: Color,
}

/// 그림 그림자
#[derive(Debug, Clone)]
pub struct PictureShadow {
    /// 그림자 종류
    pub shadow_type: PictureShadowType,
    /// 그림자 색상
    pub color: Color,
    /// 가로 오프셋
    pub offset_x: HwpUnit,
    /// 세로 오프셋
    pub offset_y: HwpUnit,
    /// 투명도 (0.0 ~ 1.0)
    pub alpha: f64,
}

/// 그림 그림자 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PictureShadowType {
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
}
