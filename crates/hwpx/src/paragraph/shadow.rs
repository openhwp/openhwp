//! [AI 생성] 그림자 프리셋
//!
//! 도형/워드아트/양식 컨트롤 등이 공유하는 단순 그림자 설정입니다. 위치·기울기만 지정하는 프리셋 수준이며, 세밀한 블러/거리 효과는 `effects` 모듈을 참고합니다. KS X 6101:2024 `paralist.xsd`.

use serde::{Deserialize, Serialize};

use crate::core::types::RgbColor;

/// [AI 생성] 그림자 유형
///
/// 원본: `ShadowType.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ShadowEffectType {
    /// [AI 생성] 없음
    #[default]
    #[serde(rename = "NONE")]
    None,
    /// [AI 생성] 평행 왼쪽 위
    #[serde(rename = "PARELLEL_LEFTTOP")]
    ParallelLeftTop,
    /// [AI 생성] 평행 오른쪽 위
    #[serde(rename = "PARELLEL_RIGHTTOP")]
    ParallelRightTop,
    /// [AI 생성] 평행 왼쪽 아래
    #[serde(rename = "PARELLEL_LEFTBOTTOM")]
    ParallelLeftBottom,
    /// [AI 생성] 평행 오른쪽 아래
    #[serde(rename = "PARELLEL_RIGHTBOTTOM")]
    ParallelRightBottom,
    /// [AI 생성] 기울기 왼쪽 위
    #[serde(rename = "SHEAR_LEFTTOP")]
    ShearLeftTop,
    /// [AI 생성] 기울기 오른쪽 위
    #[serde(rename = "SHEAR_RIGHTTOP")]
    ShearRightTop,
    /// [AI 생성] 기울기 왼쪽 아래
    #[serde(rename = "SHEAR_LEFTBOTTOM")]
    ShearLeftBottom,
    /// [AI 생성] 기울기 오른쪽 아래
    #[serde(rename = "SHEAR_RIGHTBOTTOM")]
    ShearRightBottom,
    /// [AI 생성] 원근 왼쪽 위
    #[serde(rename = "PERS_LEFTTOP")]
    PerspectiveLeftTop,
    /// [AI 생성] 원근 오른쪽 위
    #[serde(rename = "PERS_RIGHTTOP")]
    PerspectiveRightTop,
    /// [AI 생성] 원근 왼쪽 아래
    #[serde(rename = "PERS_LEFTBOTTOM")]
    PerspectiveLeftBottom,
    /// [AI 생성] 원근 오른쪽 아래
    #[serde(rename = "PERS_RIGHTBOTTOM")]
    PerspectiveRightBottom,
    /// [AI 생성] 축소
    #[serde(rename = "SCALE_NARROW")]
    ScaleNarrow,
    /// [AI 생성] 확대
    #[serde(rename = "SCALE_ENLARGE")]
    ScaleEnlarge,
}

/// [AI 생성] 그림자
///
/// 원본: `ShadowType`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "shadow")]
pub struct ShapeShadow {
    /// [AI 생성] 그림자 종류
    ///
    /// 원본: `type` 속성
    #[serde(rename = "@type", default)]
    pub shadow_type: ShadowEffectType,

    /// [AI 생성] 그림자 색
    ///
    /// 원본: `color` 속성
    #[serde(rename = "@color", default = "RgbColor::black")]
    pub color: RgbColor,

    /// [AI 생성] 그림자 간격 X
    ///
    /// 원본: `offsetX` 속성
    #[serde(rename = "@offsetX", skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<i32>,

    /// [AI 생성] 그림자 간격 Y
    ///
    /// 원본: `offsetY` 속성
    #[serde(rename = "@offsetY", skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<i32>,

    /// [AI 생성] 알파
    ///
    /// 원본: `alpha` 속성
    #[serde(rename = "@alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,
}

impl Default for ShapeShadow {
    fn default() -> Self {
        Self {
            shadow_type: ShadowEffectType::None,
            color: RgbColor::BLACK,
            offset_x: None,
            offset_y: None,
            alpha: None,
        }
    }
}
