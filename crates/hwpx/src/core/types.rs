//! [AI 생성 문서화] 핵심 복합 타입들
//!
//! KS X 6101:2024 - core.xsd를 근거로 한 AI 생성 설명입니다. 실제 스키마(`docs/hwpx/schemas/core.xsd`)와 차이가 있으면 TODO로 남겨 보완하세요.

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use super::enums::{GradationType, HatchStyle, ImageBrushMode, ImageEffect, ValueUnit};

// ============================================================================
// 숫자 기반 Boolean 타입
// ============================================================================

/// 숫자 기반 Boolean 타입
///
/// HWP/HWPX에서는 boolean 값을 "0", "1", "2" 등의 숫자로 표현하는 경우가 있습니다.
/// "0"은 false, 그 외의 값은 true로 처리됩니다.
///
/// 스키마에서는 xs:boolean으로 정의되어 있지만 실제로는 정수 값을 사용합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NumericBool(pub bool);

impl NumericBool {
    /// 새 NumericBool 생성
    pub const fn new(value: bool) -> Self {
        Self(value)
    }

    /// 내부 boolean 값 반환
    pub const fn value(&self) -> bool {
        self.0
    }
}

impl From<bool> for NumericBool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<NumericBool> for bool {
    fn from(nb: NumericBool) -> Self {
        nb.0
    }
}

impl From<i32> for NumericBool {
    fn from(value: i32) -> Self {
        Self(value != 0)
    }
}

impl Serialize for NumericBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(if self.0 { "1" } else { "0" })
    }
}

impl<'de> Deserialize<'de> for NumericBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumericBoolVisitor;

        impl<'de> de::Visitor<'de> for NumericBoolVisitor {
            type Value = NumericBool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a numeric string like \"0\", \"1\", \"2\", or boolean")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(NumericBool(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(NumericBool(v != 0))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(NumericBool(v != 0))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "true" | "TRUE" | "True" => Ok(NumericBool(true)),
                    "false" | "FALSE" | "False" => Ok(NumericBool(false)),
                    s => {
                        // 숫자로 파싱 시도 - "0"이면 false, 그 외는 true
                        match s.parse::<i64>() {
                            Ok(n) => Ok(NumericBool(n != 0)),
                            Err(_) => Err(de::Error::invalid_value(
                                de::Unexpected::Str(s),
                                &"a numeric string or boolean",
                            )),
                        }
                    }
                }
            }
        }

        deserializer.deserialize_any(NumericBoolVisitor)
    }
}

// ============================================================================
// HWP 단위 타입
// ============================================================================

/// HWP 유닛 (1/7200 인치)
///
/// 10pt = 1000 hwpunit
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct HwpUnit(pub i32);

impl HwpUnit {
    /// 새로운 HWP 유닛 생성
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// 포인트로 변환 (10pt = 1000 hwpunit)
    pub const fn to_pt(&self) -> f64 {
        self.0 as f64 / 100.0
    }

    /// 포인트로부터 생성 (10pt = 1000 hwpunit)
    pub const fn from_pt(pt: f64) -> Self {
        Self((pt * 100.0) as i32)
    }

    /// 밀리미터로 변환
    pub const fn to_mm(&self) -> f64 {
        // 1 inch = 7200 hwpunit, 1 inch = 25.4 mm
        self.0 as f64 * 25.4 / 7200.0
    }

    /// 밀리미터로부터 생성
    pub const fn from_mm(mm: f64) -> Self {
        Self((mm * 7200.0 / 25.4) as i32)
    }

    /// 내부 값 반환
    pub const fn value(&self) -> i32 {
        self.0
    }
}

impl From<i32> for HwpUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<HwpUnit> for i32 {
    fn from(unit: HwpUnit) -> Self {
        unit.0
    }
}

// ============================================================================
// ID 참조 타입들
// ============================================================================

/// 글꼴 ID 참조
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FontIdRef(pub u32);

/// 테두리/배경 ID 참조
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BorderFillIdRef(pub u32);

/// 글자 모양 ID 참조
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CharShapeIdRef(pub u32);

/// 문단 모양 ID 참조
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParaShapeIdRef(pub u32);

/// 탭 정의 ID 참조
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TabDefIdRef(pub u32);

/// 스타일 ID 참조
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StyleIdRef(pub u32);

/// 바이너리 아이템 ID 참조
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BinaryItemIdRef(pub String);

impl BinaryItemIdRef {
    /// 새로운 바이너리 아이템 ID 참조 생성
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 내부 값 반환
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 파일 ID 참조 (비디오 등의 파일 참조)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FileIdRef(pub String);

impl FileIdRef {
    /// 새로운 파일 ID 참조 생성
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 내부 값 반환
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 이미지 ID 참조
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ImageIdRef(pub String);

impl ImageIdRef {
    /// 새로운 이미지 ID 참조 생성
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 내부 값 반환
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 차트 ID 참조
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChartIdRef(pub String);

impl ChartIdRef {
    /// 새로운 차트 ID 참조 생성
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 내부 값 반환
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 사운드 ID 참조
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SoundIdRef(pub String);

impl SoundIdRef {
    /// 새로운 사운드 ID 참조 생성
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 내부 값 반환
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 마스터 페이지 ID 참조
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MasterPageIdRef(pub String);

impl MasterPageIdRef {
    /// 새로운 마스터 페이지 ID 참조 생성
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 내부 값 반환
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 외곽선 모양 ID 참조
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OutlineShapeIdRef(pub u32);

/// 메모 모양 ID 참조
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MemoShapeIdRef(pub u32);

/// 연결 목록 ID 참조
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LinkListIdRef(pub u32);

/// 시작 ID 참조
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BeginIdRef(pub u32);

/// 주체 ID 참조 (그룹 도형 등)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubjectIdRef(pub u32);

/// 속성 ID 참조 (일반적인 속성 참조용)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PropertyIdRef(pub u32);

/// 테두리 타입 ID 참조 (양식 컨트롤 등)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BorderTypeIdRef(pub u32);

// ============================================================================
// 색상 타입
// ============================================================================

/// RGB 색상
///
/// 원본: `RGBColorType`
///
/// 패턴: `#[0-9A-Fa-f]{6}` (예: `#FF0000`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbColor {
    /// 빨강 (0-255)
    pub r: u8,
    /// 초록 (0-255)
    pub g: u8,
    /// 파랑 (0-255)
    pub b: u8,
    /// 알파 (0-255)
    pub a: u8,
}

impl Default for RgbColor {
    fn default() -> Self {
        Self::BLACK
    }
}

impl RgbColor {
    /// 검정색
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    /// 흰색
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    /// 빨강
    pub const RED: Self = Self::rgb(255, 0, 0);
    /// 초록
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    /// 파랑
    pub const BLUE: Self = Self::rgb(0, 0, 255);

    /// 새로운 RGB 색상 생성
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// 알파를 포함한 RGBA 색 생성 (0-255 범위)
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// 검정색 (함수 버전, serde default용)
    pub const fn black() -> Self {
        Self::BLACK
    }

    /// 흰색 (함수 버전, serde default용)
    pub const fn white() -> Self {
        Self::WHITE
    }

    /// 16진수 문자열로부터 파싱
    ///
    /// HWPX 형식:
    /// - 6자리: `#RRGGBB` (alpha = 255)
    /// - 8자리: `#AARRGGBB` (ARGB 형식, alpha 먼저)
    pub fn from_hex(s: &str) -> Option<Self> {
        let s = s.strip_prefix('#').unwrap_or(s);
        if s.len() != 6 && s.len() != 8 {
            return None;
        }
        match s.len() {
            6 => {
                let r = u8::from_str_radix(&s[0..2], 16).ok()?;
                let g = u8::from_str_radix(&s[2..4], 16).ok()?;
                let b = u8::from_str_radix(&s[4..6], 16).ok()?;
                Some(Self::rgba(r, g, b, 255))
            }
            8 => {
                // ARGB 형식: #AARRGGBB
                let a = u8::from_str_radix(&s[0..2], 16).ok()?;
                let r = u8::from_str_radix(&s[2..4], 16).ok()?;
                let g = u8::from_str_radix(&s[4..6], 16).ok()?;
                let b = u8::from_str_radix(&s[6..8], 16).ok()?;
                Some(Self::rgba(r, g, b, a))
            }
            _ => None,
        }
    }

    /// 16진수 문자열로 변환
    ///
    /// - alpha = 255: `#RRGGBB`
    /// - alpha != 255: `#AARRGGBB` (ARGB 형식)
    pub fn to_hex(&self) -> String {
        match self.a {
            255 => format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b),
            a => format!("#{:02X}{:02X}{:02X}{:02X}", a, self.r, self.g, self.b),
        }
    }

    /// u32 값으로 변환 (0xRRGGBB)
    pub const fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// u32 값으로부터 생성 (0xRRGGBBAA)
    pub const fn from_u32(value: u32) -> Self {
        Self {
            r: ((value >> 16) & 0xFF) as u8,
            g: ((value >> 8) & 0xFF) as u8,
            b: (value & 0xFF) as u8,
            a: ((value >> 24) & 0xFF) as u8,
        }
    }
}

impl Serialize for RgbColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for RgbColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_hex(&s)
            .ok_or_else(|| serde::de::Error::custom(format!("invalid RGB color: {}", s)))
    }
}

/// 선택적 RGB 색상 (none 지원)
///
/// "none" 문자열을 지원하는 RGB 색상 타입.
/// `#RRGGBB` 형식 또는 "none"을 파싱합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OptionalRgbColor(pub Option<RgbColor>);

impl OptionalRgbColor {
    /// 색상 없음
    pub const NONE: Self = Self(None);

    /// 새로운 색상 생성
    pub const fn some(color: RgbColor) -> Self {
        Self(Some(color))
    }

    /// 색상 없음 생성
    pub const fn none() -> Self {
        Self(None)
    }

    /// 흰색 (기본값용)
    pub const fn white() -> Self {
        Self(Some(RgbColor::WHITE))
    }

    /// 검정색 (기본값용)
    pub const fn black() -> Self {
        Self(Some(RgbColor::BLACK))
    }

    /// 내부 값 반환
    pub const fn value(&self) -> Option<RgbColor> {
        self.0
    }

    /// 문자열로부터 파싱
    pub fn parse(s: &str) -> Option<Self> {
        if s.eq_ignore_ascii_case("none") {
            Some(Self(None))
        } else {
            RgbColor::from_hex(s).map(|c| Self(Some(c)))
        }
    }
}

impl From<RgbColor> for OptionalRgbColor {
    fn from(color: RgbColor) -> Self {
        Self(Some(color))
    }
}

impl From<Option<RgbColor>> for OptionalRgbColor {
    fn from(color: Option<RgbColor>) -> Self {
        Self(color)
    }
}

impl Serialize for OptionalRgbColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            Some(color) => serializer.serialize_str(&color.to_hex()),
            None => serializer.serialize_str("none"),
        }
    }
}

impl<'de> Deserialize<'de> for OptionalRgbColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s)
            .ok_or_else(|| serde::de::Error::custom(format!("invalid optional RGB color: {}", s)))
    }
}

// ============================================================================
// primitive::Color 변환
// ============================================================================

impl From<primitive::Color> for RgbColor {
    fn from(color: primitive::Color) -> Self {
        Self {
            r: color.red,
            g: color.green,
            b: color.blue,
            a: color.alpha,
        }
    }
}

impl From<RgbColor> for primitive::Color {
    fn from(color: RgbColor) -> Self {
        Self::argb(color.a, color.r, color.g, color.b)
    }
}

impl From<primitive::Color> for OptionalRgbColor {
    fn from(color: primitive::Color) -> Self {
        Self(Some(RgbColor::from(color)))
    }
}

impl From<Option<primitive::Color>> for OptionalRgbColor {
    fn from(color: Option<primitive::Color>) -> Self {
        Self(color.map(RgbColor::from))
    }
}

// ============================================================================
// primitive ID 변환
// ============================================================================

// 숫자 ID 변환 매크로
macro_rules! impl_id_conversion {
    ($hwpx_type:ty, $primitive_type:ty) => {
        impl From<$primitive_type> for $hwpx_type {
            fn from(id: $primitive_type) -> Self {
                Self(id.0)
            }
        }

        impl From<$hwpx_type> for $primitive_type {
            fn from(id: $hwpx_type) -> Self {
                Self(id.0)
            }
        }
    };
}

impl_id_conversion!(FontIdRef, primitive::FontId);
impl_id_conversion!(BorderFillIdRef, primitive::BorderFillId);
impl_id_conversion!(CharShapeIdRef, primitive::CharShapeId);
impl_id_conversion!(ParaShapeIdRef, primitive::ParaShapeId);
impl_id_conversion!(TabDefIdRef, primitive::TabDefId);
impl_id_conversion!(StyleIdRef, primitive::StyleId);
impl_id_conversion!(OutlineShapeIdRef, primitive::OutlineShapeId);
impl_id_conversion!(MemoShapeIdRef, primitive::MemoShapeId);
impl_id_conversion!(LinkListIdRef, primitive::LinkListId);
impl_id_conversion!(BeginIdRef, primitive::BeginId);
impl_id_conversion!(SubjectIdRef, primitive::SubjectId);
impl_id_conversion!(PropertyIdRef, primitive::PropertyId);
impl_id_conversion!(BorderTypeIdRef, primitive::BorderTypeId);

// 문자열 ID 변환
impl From<primitive::BinaryDataId> for BinaryItemIdRef {
    fn from(id: primitive::BinaryDataId) -> Self {
        Self(id.0)
    }
}

impl From<BinaryItemIdRef> for primitive::BinaryDataId {
    fn from(id: BinaryItemIdRef) -> Self {
        Self(id.0)
    }
}

impl From<primitive::FileId> for FileIdRef {
    fn from(id: primitive::FileId) -> Self {
        Self(id.0)
    }
}

impl From<FileIdRef> for primitive::FileId {
    fn from(id: FileIdRef) -> Self {
        Self(id.0)
    }
}

impl From<primitive::ImageId> for ImageIdRef {
    fn from(id: primitive::ImageId) -> Self {
        Self(id.0)
    }
}

impl From<ImageIdRef> for primitive::ImageId {
    fn from(id: ImageIdRef) -> Self {
        Self(id.0)
    }
}

impl From<primitive::MasterPageId> for MasterPageIdRef {
    fn from(id: primitive::MasterPageId) -> Self {
        Self(id.0)
    }
}

impl From<MasterPageIdRef> for primitive::MasterPageId {
    fn from(id: MasterPageIdRef) -> Self {
        Self(id.0)
    }
}

/// HWP 값 (값, 단위)
///
/// 원본: `HWPValue`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HwpValue {
    /// 값
    ///
    /// 원본: `value` 속성
    #[serde(rename = "@value")]
    pub value: i32,

    /// 단위
    ///
    /// 원본: `unit` 속성
    #[serde(rename = "@unit", default)]
    pub unit: ValueUnit,
}

impl Default for HwpValue {
    fn default() -> Self {
        Self {
            value: 0,
            unit: ValueUnit::HwpUnit,
        }
    }
}

/// 이미지 정보
///
/// 원본: `imageType`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    /// menifest의 item 엘리먼트의 아이디 참조 값
    ///
    /// 원본: `binaryItemIDRef` 속성
    #[serde(rename = "@binaryItemIDRef")]
    pub binary_item_id_reference: BinaryItemIdRef,

    /// 밝기
    ///
    /// 원본: `bright` 속성
    #[serde(rename = "@bright", default)]
    pub brightness: i32,

    /// 명암
    ///
    /// 원본: `contrast` 속성
    #[serde(rename = "@contrast", default)]
    pub contrast: i32,

    /// 그림 효과
    ///
    /// 원본: `effect` 속성
    #[serde(rename = "@effect", default)]
    pub effect: ImageEffect,

    /// 알파
    ///
    /// 원본: `alpha` 속성
    #[serde(rename = "@alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,
}

/// 행렬 정보
///
/// 원본: `MatrixType`
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Matrix {
    /// 원본: `e1` 속성
    #[serde(rename = "@e1", skip_serializing_if = "Option::is_none")]
    pub e1: Option<f32>,

    /// 원본: `e2` 속성
    #[serde(rename = "@e2", skip_serializing_if = "Option::is_none")]
    pub e2: Option<f32>,

    /// 원본: `e3` 속성
    #[serde(rename = "@e3", skip_serializing_if = "Option::is_none")]
    pub e3: Option<f32>,

    /// 원본: `e4` 속성
    #[serde(rename = "@e4", skip_serializing_if = "Option::is_none")]
    pub e4: Option<f32>,

    /// 원본: `e5` 속성
    #[serde(rename = "@e5", skip_serializing_if = "Option::is_none")]
    pub e5: Option<f32>,

    /// 원본: `e6` 속성
    #[serde(rename = "@e6", skip_serializing_if = "Option::is_none")]
    pub e6: Option<f32>,
}

/// 점 정보
///
/// 원본: `PointType`
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Point {
    /// X 좌표
    ///
    /// 원본: `x` 속성
    #[serde(rename = "@x", skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,

    /// Y 좌표
    ///
    /// 원본: `y` 속성
    #[serde(rename = "@y", skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,
}

/// 면 채우기
///
/// 원본: `winBrush` 요소
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "winBrush")]
pub struct WindowsBrush {
    /// 면 색
    ///
    /// 원본: `faceColor` 속성
    /// "none" 값을 지원함
    #[serde(rename = "@faceColor", default = "OptionalRgbColor::white")]
    pub face_color: OptionalRgbColor,

    /// 무늬 색
    ///
    /// 원본: `hatchColor` 속성
    /// "none" 값을 지원함
    #[serde(rename = "@hatchColor", default = "OptionalRgbColor::black")]
    pub hatch_color: OptionalRgbColor,

    /// 무늬 종류
    ///
    /// 원본: `hatchStyle` 속성
    #[serde(rename = "@hatchStyle", skip_serializing_if = "Option::is_none")]
    pub hatch_style: Option<HatchStyle>,

    /// 알파
    ///
    /// 원본: `alpha` 속성
    #[serde(rename = "@alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,
}

impl Default for WindowsBrush {
    fn default() -> Self {
        Self {
            face_color: OptionalRgbColor::white(),
            hatch_color: OptionalRgbColor::black(),
            hatch_style: None,
            alpha: None,
        }
    }
}

/// 그라데이션 색
///
/// 원본: `color` 요소 (gradation 내)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "color")]
pub struct GradationColor {
    /// 색상 값
    ///
    /// 원본: `value` 속성
    #[serde(rename = "@value")]
    pub value: RgbColor,
}

/// 그라데이션 번짐 정도 (0~255)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GradationStep(u8);

impl Default for GradationStep {
    fn default() -> Self {
        Self(255)
    }
}

impl GradationStep {
    /// 새로운 그라데이션 번짐 정도 생성 (0~255)
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    /// 값 반환
    pub const fn value(&self) -> u8 {
        self.0
    }
}

/// 그라데이션 번짐 정도 중심 (0~100)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GradationStepCenter(u8);

impl Default for GradationStepCenter {
    fn default() -> Self {
        Self(50)
    }
}

impl GradationStepCenter {
    /// 새로운 그라데이션 번짐 정도 중심 생성 (0~100)
    pub const fn new(value: u8) -> Option<Self> {
        if value <= 100 {
            Some(Self(value))
        } else {
            None
        }
    }

    /// 값 반환
    pub const fn value(&self) -> u8 {
        self.0
    }
}

/// 그라데이션 효과
///
/// 원본: `gradation` 요소
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "gradation")]
pub struct Gradation {
    /// 그라데이션 색 목록
    ///
    /// 원본: `color` 요소
    #[serde(rename = "color", default)]
    pub colors: Vec<GradationColor>,

    /// 그라데이션 유형
    ///
    /// 원본: `type` 속성
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub gradation_type: Option<GradationType>,

    /// 그라데이션의 기울임(시작각)
    ///
    /// 원본: `angle` 속성
    #[serde(rename = "@angle", default = "default_gradation_angle")]
    pub angle: i32,

    /// 그라데이션의 가로중심(중심 X좌표)
    ///
    /// 원본: `centerX` 속성
    #[serde(rename = "@centerX", default)]
    pub center_x: i32,

    /// 그라데이션의 세로중심(중심 Y좌표)
    ///
    /// 원본: `centerY` 속성
    #[serde(rename = "@centerY", default)]
    pub center_y: i32,

    /// 그라데이션 번짐정도 (0~255)
    ///
    /// 원본: `step` 속성
    #[serde(rename = "@step", default)]
    pub step: GradationStep,

    /// 그라데이션의 색수
    ///
    /// 원본: `colorNum` 속성
    #[serde(rename = "@colorNum", default = "default_gradation_color_count")]
    pub color_count: u32,

    /// 그라데이션 번짐정도의 중심 (0~100)
    ///
    /// 원본: `stepCenter` 속성
    #[serde(rename = "@stepCenter", default)]
    pub step_center: GradationStepCenter,

    /// 알파
    ///
    /// 원본: `alpha` 속성
    #[serde(rename = "@alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,
}

fn default_gradation_angle() -> i32 {
    90
}

fn default_gradation_color_count() -> u32 {
    2
}

impl Default for Gradation {
    fn default() -> Self {
        Self {
            colors: Vec::new(),
            gradation_type: None,
            angle: 90,
            center_x: 0,
            center_y: 0,
            step: GradationStep::default(),
            color_count: 2,
            step_center: GradationStepCenter::default(),
            alpha: None,
        }
    }
}

/// 그림으로 채우기
///
/// 원본: `imgBrush` 요소
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "imgBrush")]
pub struct ImageBrush {
    /// 이미지 정보
    ///
    /// 원본: `img` 요소
    #[serde(rename = "img")]
    pub image: Image,

    /// 채우기 유형
    ///
    /// 원본: `mode` 속성
    #[serde(rename = "@mode", default)]
    pub mode: ImageBrushMode,
}

/// 채우기 정보
///
/// 원본: `FillBrushType`
///
/// HWPX 파일에서 fillBrush는 winBrush, gradation, imgBrush를 조합하여 사용할 수 있습니다.
/// 예를 들어 배경색(winBrush)과 이미지(imgBrush)를 동시에 지정할 수 있습니다.
///
/// 일반적으로는 winBrush, gradation, imgBrush 중 하나만 사용되지만,
/// 실제 HWPX 파일에서는 winBrush와 imgBrush가 동시에 존재할 수 있습니다.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "fillBrush")]
pub struct FillBrush {
    /// 면 채우기 (윈도우 브러시)
    ///
    /// 원본: `winBrush` 요소
    #[serde(rename = "winBrush", skip_serializing_if = "Option::is_none")]
    pub windows_brush: Option<WindowsBrush>,

    /// 그라데이션 효과
    ///
    /// 원본: `gradation` 요소
    #[serde(rename = "gradation", skip_serializing_if = "Option::is_none")]
    pub gradation: Option<Gradation>,

    /// 그림으로 채우기
    ///
    /// 원본: `imgBrush` 요소
    #[serde(rename = "imgBrush", skip_serializing_if = "Option::is_none")]
    pub image_brush: Option<ImageBrush>,
}

/// 키 암호화 유도 키 정보
///
/// 원본: `derivationKey` 요소
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "derivationKey")]
pub struct DerivationKey {
    /// 알고리즘
    ///
    /// 원본: `algorithm` 속성
    #[serde(rename = "@algorithm", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// 크기
    ///
    /// 원본: `size` 속성
    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    /// 반복 횟수
    ///
    /// 원본: `count` 속성
    #[serde(rename = "@count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// 솔트 (Base64 인코딩)
    ///
    /// 원본: `salt` 속성
    #[serde(rename = "@salt", skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
}

/// 키 암호화 정보
///
/// 원본: `KeyEncryptionType`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyEncryption {
    /// 유도 키 정보
    ///
    /// 원본: `derivationKey` 요소
    #[serde(rename = "derivationKey")]
    pub derivation_key: DerivationKey,

    /// 해시 (Base64 인코딩)
    ///
    /// 원본: `hash` 요소
    #[serde(rename = "hash")]
    pub hash: String,
}

/// 메타 태그
///
/// 원본: `MetaTagType`
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MetaTag {
    /// 내용
    #[serde(rename = "$text", default)]
    pub content: String,
}
