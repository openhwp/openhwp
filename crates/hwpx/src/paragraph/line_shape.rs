//! [AI 생성] 선/외곽선 스타일 정의
//!
//! 도형·커넥터·워드아트 등에서 공통으로 쓰이는 선 모양 속성입니다. 텍스트 감싸기 계산에 쓰일 굵기, 마커(화살표) 스타일, 내/외곽선 여부 등을 정의합니다. KS X 6101:2024 `paralist.xsd` 기준.

use serde::{Deserialize, Serialize};

use crate::core::{
    enums::{ArrowSize, ArrowStyle, LineStyleType2},
    types::RgbColor,
};

/// [AI 생성] 선 끝 모양
///
/// 원본: `LineShapeType.endCap` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LineEndCapStyle {
    /// 둥근 끝
    #[serde(rename = "ROUND")]
    Round,
    /// 평평한 끝
    #[default]
    #[serde(rename = "FLAT")]
    Flat,
}

/// [AI 생성] 외곽선 스타일
///
/// 원본: `LineShapeType.outlineStyle` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OutlineStyle {
    /// 표준
    #[default]
    #[serde(rename = "NORMAL")]
    Normal,
    /// 바깥쪽
    #[serde(rename = "OUTER")]
    Outer,
    /// 안쪽
    #[serde(rename = "INNER")]
    Inner,
}

/// [AI 생성] 선 모양
///
/// 원본: `LineShapeType`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineShape {
    /// [AI 생성] 색상 (`color` 속성)
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<RgbColor>,

    /// [AI 생성] 굵기 (`width` 속성)
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// [AI 생성] 스타일 (`style` 속성)
    #[serde(rename = "@style", default)]
    pub style: LineStyleType2,

    /// [AI 생성] 선 끝 모양 (`endCap` 속성)
    #[serde(rename = "@endCap", default)]
    pub end_cap: LineEndCapStyle,

    /// [AI 생성] 머리 스타일 (`headStyle` 속성)
    #[serde(rename = "@headStyle", default)]
    pub head_style: ArrowStyle,

    /// [AI 생성] 꼬리 스타일 (`tailStyle` 속성)
    #[serde(rename = "@tailStyle", default)]
    pub tail_style: ArrowStyle,

    /// [AI 생성] 머리 채움 (`headfill` 속성)
    #[serde(rename = "@headfill", default)]
    pub head_fill: bool,

    /// [AI 생성] 꼬리 채움 (`tailfill` 속성)
    #[serde(rename = "@tailfill", default)]
    pub tail_fill: bool,

    /// [AI 생성] 머리 크기 (`headSz` 속성)
    #[serde(rename = "@headSz", default)]
    pub head_size: ArrowSize,

    /// [AI 생성] 꼬리 크기 (`tailSz` 속성)
    #[serde(rename = "@tailSz", default)]
    pub tail_size: ArrowSize,

    /// [AI 생성] 외곽선 스타일 (`outlineStyle` 속성)
    #[serde(rename = "@outlineStyle", default)]
    pub outline_style: OutlineStyle,

    /// [AI 생성] 알파 값 (`alpha` 속성)
    #[serde(rename = "@alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,
}

impl Default for LineShape {
    fn default() -> Self {
        Self {
            color: None,
            width: None,
            style: LineStyleType2::Solid,
            end_cap: LineEndCapStyle::Flat,
            head_style: ArrowStyle::Normal,
            tail_style: ArrowStyle::Normal,
            head_fill: false,
            tail_fill: false,
            head_size: ArrowSize::SmallSmall,
            tail_size: ArrowSize::SmallSmall,
            outline_style: OutlineStyle::Normal,
            alpha: None,
        }
    }
}
