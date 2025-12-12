//! 기하학 관련 타입
//!
//! 도형과 변환에 사용되는 기하학적 타입들을 정의합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ArrowSize, ArrowType, HwpUnit, Point};

/// 변환 행렬 (아핀 변환)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransformMatrix {
    /// e1 요소 (scaleX)
    pub e1: f64,
    /// e2 요소 (shearX)
    pub e2: f64,
    /// e3 요소 (shearY)
    pub e3: f64,
    /// e4 요소 (scaleY)
    pub e4: f64,
    /// e5 요소 (translateX)
    pub e5: f64,
    /// e6 요소 (translateY)
    pub e6: f64,
}

impl TransformMatrix {
    /// 단위 행렬 생성
    pub const fn identity() -> Self {
        Self {
            e1: 1.0,
            e2: 0.0,
            e3: 0.0,
            e4: 1.0,
            e5: 0.0,
            e6: 0.0,
        }
    }

    /// HWP 행렬 벡터에서 변환 (6개 또는 그 이상의 값)
    pub const fn from_hwp_matrix(matrix: &[f64]) -> Option<Self> {
        if matrix.len() >= 6 {
            Some(Self {
                e1: matrix[0],
                e2: matrix[1],
                e3: matrix[2],
                e4: matrix[3],
                e5: matrix[4],
                e6: matrix[5],
            })
        } else {
            None
        }
    }

    /// HWPX Matrix 타입으로 변환
    pub const fn to_hwpx_values(&self) -> (f32, f32, f32, f32, f32, f32) {
        (
            self.e1 as f32,
            self.e2 as f32,
            self.e3 as f32,
            self.e4 as f32,
            self.e5 as f32,
            self.e6 as f32,
        )
    }
}

/// 화살표
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Arrow {
    /// 화살표 종류
    pub arrow_type: ArrowType,
    /// 화살표 크기
    pub size: ArrowSize,
    /// 채움 여부
    pub filled: bool,
}

/// 사각형 모서리 반경
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RectangleCorner {
    /// 모서리 반경
    pub corner_radius: HwpUnit,
}

/// 곡선 점
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CurvePoint {
    /// 좌표
    pub point: Point,
    /// 점 종류
    pub point_type: CurvePointKind,
}

/// 곡선 점 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CurvePointKind {
    /// 일반점
    #[default]
    Normal,
    /// 제어점 1
    Control1,
    /// 제어점 2
    Control2,
}

/// 연결점 정보
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConnectorPoint {
    /// 좌표
    pub point: Point,
    /// 대상 개체 ID 참조
    pub subject_id_ref: Option<u32>,
    /// 대상 연결 인덱스
    pub subject_index: Option<u32>,
}

/// 이미지 자르기
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
