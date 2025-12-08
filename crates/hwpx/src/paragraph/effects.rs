//! [AI 생성] 고급 효과 (그림자/네온/반사/부드러운 가장자리)
//!
//! 기본 그림자 프리셋보다 세밀한 광원·색상·기울기 조정이 필요한 도형을 위해 쓰이는 공통 타입입니다. 색상 표현은 RGB/CMYK/스킴/시스템 네 가지 경로로 지원하며, 렌더링 엔진이 선택적으로 적용합니다. KS X 6101:2024 `paralist.xsd`.

use serde::{Deserialize, Serialize};

use crate::core::enums::AlignmentStyle;

/// [AI 생성] 기울기 (`skew` 요소)
///
/// 원본: `SkewType`
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "skew")]
pub struct Skew {
    /// [AI 생성] X축 기울기 (-90~90) (`x` 속성)
    #[serde(rename = "@x", skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,

    /// [AI 생성] Y축 기울기 (-90~90) (`y` 속성)
    #[serde(rename = "@y", skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
}

/// [AI 생성] 확대 비율 (`scale` 요소)
///
/// 원본: `ScaleType`
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "scale")]
pub struct Scale {
    /// [AI 생성] X축 확대 비율 (`x` 속성)
    #[serde(rename = "@x", skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,

    /// [AI 생성] Y축 확대 비율 (`y` 속성)
    #[serde(rename = "@y", skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
}

/// [AI 생성] 효과 색 유형 (`type` 값)
///
/// 원본: `EffectsColorType.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectsColorKind {
    /// RGB
    #[serde(rename = "RGB")]
    Rgb,
    /// CMYK
    #[serde(rename = "CMYK")]
    Cmyk,
    /// 스킴
    #[serde(rename = "SCHEME")]
    Scheme,
    /// 시스템
    #[serde(rename = "SYSTEM")]
    System,
}

/// [AI 생성] RGB 색상 값 (`rgb` 요소)
///
/// 원본: `EffectsColorType/rgb` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "rgb")]
pub struct EffectsRgbColor {
    /// [AI 생성] 빨강
    #[serde(rename = "@r")]
    pub red: u32,
    /// [AI 생성] 초록
    #[serde(rename = "@g")]
    pub green: u32,
    /// [AI 생성] 파랑
    #[serde(rename = "@b")]
    pub blue: u32,
}

/// [AI 생성] CMYK 색상 값 (`cmyk` 요소)
///
/// 원본: `EffectsColorType/cmyk` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "cmyk")]
pub struct EffectsCmykColor {
    /// [AI 생성] 시안
    #[serde(rename = "@c")]
    pub cyan: u32,
    /// [AI 생성] 마젠타
    #[serde(rename = "@m")]
    pub magenta: u32,
    /// [AI 생성] 노랑
    #[serde(rename = "@y")]
    pub yellow: u32,
    /// [AI 생성] 검정
    #[serde(rename = "@k")]
    pub black: u32,
}

/// [AI 생성] 스킴 색상 값 (`scheme` 요소)
///
/// 원본: `EffectsColorType/scheme` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "scheme")]
pub struct EffectsSchemeColor {
    /// [AI 생성] 빨강
    #[serde(rename = "@r")]
    pub red: u32,
    /// [AI 생성] 초록
    #[serde(rename = "@g")]
    pub green: u32,
    /// [AI 생성] 파랑
    #[serde(rename = "@b")]
    pub blue: u32,
}

/// [AI 생성] 시스템 색상 값 (HSL, `system` 요소)
///
/// 원본: `EffectsColorType/system` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "system")]
pub struct EffectsSystemColor {
    /// [AI 생성] 색조
    #[serde(rename = "@h")]
    pub hue: u32,
    /// [AI 생성] 채도
    #[serde(rename = "@s")]
    pub saturation: u32,
    /// [AI 생성] 명도
    #[serde(rename = "@l")]
    pub lightness: u32,
}

/// [AI 생성] 효과 색상 값 (RGB/CMYK/스킴/시스템 중 하나)
///
/// 원본: `EffectsColorType` 내 choice 요소
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EffectsColorValue {
    /// RGB
    Rgb(EffectsRgbColor),
    /// CMYK
    Cmyk(EffectsCmykColor),
    /// 스킴
    Scheme(EffectsSchemeColor),
    /// 시스템
    System(EffectsSystemColor),
}

/// [AI 생성] 효과 색 (`effectsColor` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectsColor {
    /// [AI 생성] 색상 값 (RGB/CMYK/스킴/시스템 중 하나)
    ///
    /// 원본: `EffectsColorType` choice 요소
    #[serde(flatten)]
    pub value: Option<EffectsColorValue>,

    /// [AI 생성] 색상 유형 (`type` 속성)
    ///
    /// 원본: `EffectsColorType.type` 속성 (RGB/CMYK/SCHEME/SYSTEM)
    #[serde(rename = "@type")]
    pub color_type: EffectsColorKind,

    /// [AI 생성] 스킴 인덱스 (`schemeIdx` 속성)
    #[serde(rename = "@schemeIdx", skip_serializing_if = "Option::is_none")]
    pub scheme_index: Option<i32>,

    /// [AI 생성] 시스템 인덱스 (`systemIdx` 속성)
    #[serde(rename = "@systemIdx", skip_serializing_if = "Option::is_none")]
    pub system_index: Option<i32>,

    /// [AI 생성] 프리셋 인덱스 (`presetIdx` 속성)
    #[serde(rename = "@presetIdx", skip_serializing_if = "Option::is_none")]
    pub preset_index: Option<i32>,
}

/// [AI 생성] 네온 효과 유형
///
/// 원본: `GlowEffectsColorType/effect.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GlowEffectKind {
    /// 알파값 설정 (절대)
    #[serde(rename = "ALPHA")]
    Alpha,
    /// 알파값 가중 곱
    #[serde(rename = "ALPHA_MOD")]
    AlphaModulate,
    /// 알파값 오프셋
    #[serde(rename = "ALPHA_OFF")]
    AlphaOffset,
    /// 빨강 채널 설정
    #[serde(rename = "RED")]
    Red,
    /// 빨강 채널 가중 곱
    #[serde(rename = "RED_MOD")]
    RedModulate,
    /// 빨강 채널 오프셋
    #[serde(rename = "RED_OFF")]
    RedOffset,
    /// 초록 채널 설정
    #[serde(rename = "GREEN")]
    Green,
    /// 초록 채널 가중 곱
    #[serde(rename = "GREEN_MOD")]
    GreenModulate,
    /// 초록 채널 오프셋
    #[serde(rename = "GREEN_OFF")]
    GreenOffset,
    /// 파랑 채널 설정
    #[serde(rename = "BLUE")]
    Blue,
    /// 파랑 채널 가중 곱
    #[serde(rename = "BLUE_MOD")]
    BlueModulate,
    /// 파랑 채널 오프셋
    #[serde(rename = "BLUE_OFF")]
    BlueOffset,
    /// 색상 각도 설정
    #[serde(rename = "HUE")]
    Hue,
    /// 색상 각도 가중 곱
    #[serde(rename = "HUE_MOD")]
    HueModulate,
    /// 색상 각도 오프셋
    #[serde(rename = "HUE_OFF")]
    HueOffset,
    /// 채도 설정
    #[serde(rename = "SAT")]
    Saturation,
    /// 채도 가중 곱
    #[serde(rename = "SAT_MOD")]
    SaturationModulate,
    /// 채도 오프셋
    #[serde(rename = "SAT_OFF")]
    SaturationOffset,
    /// 휘도 설정
    #[serde(rename = "LUM")]
    Luminance,
    /// 휘도 가중 곱
    #[serde(rename = "LUM_MOD")]
    LuminanceModulate,
    /// 휘도 오프셋
    #[serde(rename = "LUM_OFF")]
    LuminanceOffset,
    /// 색상 어둡게 (쉐이드)
    #[serde(rename = "SHADE")]
    Shade,
    /// 색상 밝게 (틴트)
    #[serde(rename = "TINT")]
    Tint,
    /// 회색조 변환
    #[serde(rename = "GRAY")]
    Gray,
    /// 보색 변환
    #[serde(rename = "COMP")]
    Complement,
    /// 감마 적용
    #[serde(rename = "GAMMA")]
    Gamma,
    /// 역감마 적용
    #[serde(rename = "INV_GAMMA")]
    InverseGamma,
    /// 역상 변환
    #[serde(rename = "INV")]
    Inverse,
}

/// [AI 생성] 네온 효과 색 조정
///
/// 원본: `GlowEffectsColorType/effect` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "effect")]
pub struct GlowColorEffect {
    /// [AI 생성] 효과 유형
    #[serde(rename = "@type")]
    pub effect_type: GlowEffectKind,

    /// [AI 생성] 값
    #[serde(rename = "@value")]
    pub value: String,
}

/// [AI 생성] 네온 효과 색
///
/// 원본: `GlowEffectsColorType`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "effectsColor")]
pub struct GlowEffectsColor {
    /// [AI 생성] 색상 값 (RGB, CMYK, Scheme, System 중 하나)
    #[serde(flatten)]
    pub value: Option<EffectsColorValue>,

    /// [AI 생성] 효과 조정 (`effect` 요소)
    #[serde(rename = "effect", skip_serializing_if = "Option::is_none")]
    pub effect: Option<GlowColorEffect>,

    /// [AI 생성] 색 유형 (`type` 속성)
    #[serde(rename = "@type")]
    pub color_type: EffectsColorKind,

    /// [AI 생성] 스킴 인덱스 (`schemeIdx` 속성)
    #[serde(rename = "@schemeIdx", skip_serializing_if = "Option::is_none")]
    pub scheme_index: Option<i32>,

    /// [AI 생성] 시스템 인덱스 (`systemIdx` 속성)
    #[serde(rename = "@systemIdx", skip_serializing_if = "Option::is_none")]
    pub system_index: Option<i32>,

    /// [AI 생성] 프리셋 인덱스 (`presetIdx` 속성)
    #[serde(rename = "@presetIdx", skip_serializing_if = "Option::is_none")]
    pub preset_index: Option<i32>,
}

/// [AI 생성] 그림자 효과 스타일
///
/// 원본: `EffectsType/shadow.style` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AdvancedShadowStyle {
    /// 바깥쪽
    #[default]
    #[serde(rename = "OUTSIDE")]
    Outside,
    /// 안쪽
    #[serde(rename = "INSIDE")]
    Inside,
}

/// [AI 생성] 고급 그림자 효과
///
/// 원본: `EffectsType/shadow` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "shadow")]
pub struct AdvancedShadowEffect {
    /// [AI 생성] 기울기 각도 (`skew` 요소)
    #[serde(rename = "skew")]
    pub skew: Skew,

    /// [AI 생성] 확대 비율 (`scale` 요소)
    #[serde(rename = "scale")]
    pub scale: Scale,

    /// [AI 생성] 효과 색 (`effectsColor` 요소)
    #[serde(rename = "effectsColor")]
    pub effects_color: EffectsColor,

    /// [AI 생성] 그림자 스타일 (`style` 속성)
    #[serde(rename = "@style", skip_serializing_if = "Option::is_none")]
    pub style: Option<AdvancedShadowStyle>,

    /// [AI 생성] 투명도 (0~1) (`alpha` 속성)
    #[serde(rename = "@alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,

    /// [AI 생성] 흐릿한 정도 (HWPUNIT) (`radius` 속성)
    #[serde(rename = "@radius", skip_serializing_if = "Option::is_none")]
    pub radius: Option<f32>,

    /// [AI 생성] 그림자 생성 각도 (0~360) (`direction` 속성)
    #[serde(rename = "@direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<i32>,

    /// [AI 생성] 거리 (HWPUNIT) (`distance` 속성)
    #[serde(rename = "@distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<f32>,

    /// [AI 생성] 그림자 표현 방식 (`alignStyle` 속성)
    #[serde(rename = "@alignStyle", skip_serializing_if = "Option::is_none")]
    pub align_style: Option<AlignmentStyle>,

    /// [AI 생성] 도형과 함께 그림자 회전 여부 (`rotationStyle` 속성)
    #[serde(rename = "@rotationStyle", skip_serializing_if = "Option::is_none")]
    pub rotation_style: Option<bool>,
}

/// [AI 생성] 네온 효과
///
/// 원본: `EffectsType/glow` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "glow")]
pub struct GlowEffect {
    /// [AI 생성] 효과 색
    ///
    /// 원본: `effectsColor` 요소
    #[serde(rename = "effectsColor")]
    pub effects_color: GlowEffectsColor,

    /// [AI 생성] 투명도 (0~1)
    ///
    /// 원본: `alpha` 속성
    #[serde(rename = "@alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,

    /// [AI 생성] 네온 크기 (HWPUNIT)
    ///
    /// 원본: `radius` 속성
    #[serde(rename = "@radius", skip_serializing_if = "Option::is_none")]
    pub radius: Option<f32>,
}

/// [AI 생성] 부드러운 가장자리 효과
///
/// 원본: `EffectsType/softEdge` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename = "softEdge")]
pub struct SoftEdgeEffect {
    /// [AI 생성] 부드러운 가장자리 크기 (HWPUNIT) (`radius` 속성)
    #[serde(rename = "@radius", skip_serializing_if = "Option::is_none")]
    pub radius: Option<f32>,
}

/// [AI 생성] 반사 투명도
///
/// 원본: `EffectsType/reflection/alpha` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "alpha")]
pub struct ReflectionAlpha {
    /// [AI 생성] 시작 위치 투명도 (0~1) (`start` 속성)
    #[serde(rename = "@start", skip_serializing_if = "Option::is_none")]
    pub start: Option<f32>,

    /// [AI 생성] 끝 위치 투명도 (0~1) (`end` 속성)
    #[serde(rename = "@end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f32>,
}

/// [AI 생성] 반사 위치
///
/// 원본: `EffectsType/reflection/pos` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "pos")]
pub struct ReflectionPosition {
    /// [AI 생성] 시작 위치 (`start` 속성)
    #[serde(rename = "@start", skip_serializing_if = "Option::is_none")]
    pub start: Option<f32>,

    /// [AI 생성] 끝 위치 (`end` 속성)
    #[serde(rename = "@end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f32>,
}

/// [AI 생성] 반사 효과
///
/// 원본: `EffectsType/reflection` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "reflection")]
pub struct ReflectionEffect {
    /// [AI 생성] 기울기 (`skew` 요소)
    #[serde(rename = "skew")]
    pub skew: Skew,

    /// [AI 생성] 확대 비율 (`scale` 요소)
    #[serde(rename = "scale")]
    pub scale: Scale,

    /// [AI 생성] 투명도 (`alpha` 요소)
    #[serde(rename = "alpha")]
    pub alpha: ReflectionAlpha,

    /// [AI 생성] 위치 (`pos` 요소)
    #[serde(rename = "pos")]
    pub position: ReflectionPosition,

    /// [AI 생성] 그림자 표현 방식 (`alignStyle` 속성)
    #[serde(rename = "@alignStyle", skip_serializing_if = "Option::is_none")]
    pub align_style: Option<AlignmentStyle>,

    /// [AI 생성] 흐릿한 정도 (HWPUNIT) (`radius` 속성)
    #[serde(rename = "@radius", skip_serializing_if = "Option::is_none")]
    pub radius: Option<f32>,

    /// [AI 생성] 그림자 방향 각도 (0~360) (`direction` 속성)
    #[serde(rename = "@direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<i32>,

    /// [AI 생성] 거리 (HWPUNIT) (`distance` 속성)
    #[serde(rename = "@distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<f32>,

    /// [AI 생성] 도형과 함께 회전 여부 (`rotationStyle` 속성)
    #[serde(rename = "@rotationStyle", skip_serializing_if = "Option::is_none")]
    pub rotation_style: Option<bool>,

    /// [AI 생성] 오프셋 방향 (0~360) (`fadeDirection` 속성)
    #[serde(rename = "@fadeDirection", skip_serializing_if = "Option::is_none")]
    pub fade_direction: Option<i32>,
}

/// [AI 생성] 효과
///
/// 원본: `EffectsType`
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "effects")]
pub struct Effects {
    /// [AI 생성] 고급 그림자 효과 (`shadow` 요소)
    #[serde(rename = "shadow", skip_serializing_if = "Option::is_none")]
    pub shadow: Option<AdvancedShadowEffect>,

    /// [AI 생성] 네온 효과 (`glow` 요소)
    #[serde(rename = "glow", skip_serializing_if = "Option::is_none")]
    pub glow: Option<GlowEffect>,

    /// [AI 생성] 부드러운 가장자리 효과 (`softEdge` 요소)
    #[serde(rename = "softEdge", skip_serializing_if = "Option::is_none")]
    pub soft_edge: Option<SoftEdgeEffect>,

    /// [AI 생성] 반사 효과 (`reflection` 요소)
    #[serde(rename = "reflection", skip_serializing_if = "Option::is_none")]
    pub reflection: Option<ReflectionEffect>,
}
