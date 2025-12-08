//! [AI 생성 문서화] 글꼴 정보
//!
//! KS X 6101:2024 - header.xsd 기반 설명입니다. 실제 스키마는 `docs/hwpx/schemas/header.xsd`를 교차 확인하세요.

use serde::{Deserialize, Serialize};

use crate::core::types::BinaryItemIdRef;

/// [AI 생성] 글꼴 언어 종류
///
/// 원본: `lang` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontLanguage {
    /// [AI 생성] 한글
    #[serde(rename = "HANGUL")]
    Hangul,
    /// [AI 생성] 영어
    #[serde(rename = "LATIN")]
    Latin,
    /// [AI 생성] 한자
    #[serde(rename = "HANJA")]
    Hanja,
    /// [AI 생성] 일본어
    #[serde(rename = "JAPANESE")]
    Japanese,
    /// [AI 생성] 기타
    #[serde(rename = "OTHER")]
    Other,
    /// [AI 생성] 심볼
    #[serde(rename = "SYMBOL")]
    Symbol,
    /// [AI 생성] 사용자
    #[serde(rename = "USER")]
    User,
}

/// [AI 생성] 글꼴 유형
///
/// 원본: `type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontType {
    /// [AI 생성] 대표 글꼴
    #[serde(rename = "REP")]
    Representative,
    /// [AI 생성] 트루타입 글꼴
    #[serde(rename = "TTF")]
    TrueType,
    /// [AI 생성] 한글 전용 글꼴
    #[serde(rename = "HFT")]
    HangeulFont,
}

/// [AI 생성] 글꼴 계열
///
/// 원본: `familyType` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontFamilyType {
    /// [AI 생성] 알 수 없음
    #[serde(rename = "FCAT_UNKNOWN")]
    Unknown,
    /// [AI 생성] 명조 (serif)
    #[serde(rename = "FCAT_MYUNGJO")]
    Myungjo,
    /// [AI 생성] 고딕 (sans-serif)
    #[serde(rename = "FCAT_GOTHIC")]
    Gothic,
    /// [AI 생성] 세리프 (monospace)
    #[serde(rename = "FCAT_SSERIF")]
    SansSerif,
    /// [AI 생성] 필기체 (cursive)
    #[serde(rename = "FCAT_BRUSHSCRIPT")]
    BrushScript,
    /// [AI 생성] 장식체 (cursive)
    #[serde(rename = "FCAT_DECORATIVE")]
    Decorative,
    /// [AI 생성] 비정형 명조 (serif)
    #[serde(rename = "FCAT_NONRECTMJ")]
    NonRectMyungjo,
    /// [AI 생성] 비정형 고딕 (sans-serif)
    #[serde(rename = "FCAT_NONRECTGT")]
    NonRectGothic,
}

/// [AI 생성] PANOSE 세리프 유형
///
/// PANOSE 분류 체계의 세리프 스타일
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum PanoseSerifStyle {
    /// 임의/없음
    #[default]
    #[serde(rename = "Any", alias = "0")]
    Any = 0,
    /// 해당 없음
    #[serde(rename = "NoFit", alias = "1")]
    NoFit = 1,
    /// Cove
    #[serde(rename = "Cove", alias = "2")]
    Cove = 2,
    /// Obtuse Cove
    #[serde(rename = "ObtuseCove", alias = "3")]
    ObtuseCove = 3,
    /// Square Cove
    #[serde(rename = "SquareCove", alias = "4")]
    SquareCove = 4,
    /// Obtuse Square Cove
    #[serde(rename = "ObtuseSquareCove", alias = "5")]
    ObtuseSquareCove = 5,
    /// Square
    #[serde(rename = "Square", alias = "6")]
    Square = 6,
    /// Thin
    #[serde(rename = "Thin", alias = "7")]
    Thin = 7,
    /// Oval
    #[serde(rename = "Oval", alias = "8")]
    Oval = 8,
    /// Exaggerated
    #[serde(rename = "Exaggerated", alias = "9")]
    Exaggerated = 9,
    /// Triangle
    #[serde(rename = "Triangle", alias = "10")]
    Triangle = 10,
    /// Normal Sans
    #[serde(rename = "NormalSans", alias = "11")]
    NormalSans = 11,
    /// Obtuse Sans
    #[serde(rename = "ObtuseSans", alias = "12")]
    ObtuseSans = 12,
    /// Perpendicular Sans
    #[serde(rename = "PerpendicularSans", alias = "13")]
    PerpendicularSans = 13,
    /// Flared
    #[serde(rename = "Flared", alias = "14")]
    Flared = 14,
    /// Rounded
    #[serde(rename = "Rounded", alias = "15")]
    Rounded = 15,
}

/// [AI 생성] PANOSE 굵기
///
/// PANOSE 분류 체계의 굵기 (Weight)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum PanoseWeight {
    /// 임의
    #[default]
    #[serde(rename = "Any", alias = "0")]
    Any = 0,
    /// 해당 없음
    #[serde(rename = "NoFit", alias = "1")]
    NoFit = 1,
    /// 매우 가늘게
    #[serde(rename = "VeryLight", alias = "2")]
    VeryLight = 2,
    /// 가늘게
    #[serde(rename = "Light", alias = "3")]
    Light = 3,
    /// 얇게
    #[serde(rename = "Thin", alias = "4")]
    Thin = 4,
    /// 책
    #[serde(rename = "Book", alias = "5")]
    Book = 5,
    /// 중간
    #[serde(rename = "Medium", alias = "6")]
    Medium = 6,
    /// 약간 굵게
    #[serde(rename = "DemiBold", alias = "7")]
    DemiBold = 7,
    /// 굵게
    #[serde(rename = "Bold", alias = "8")]
    Bold = 8,
    /// 아주 굵게
    #[serde(rename = "Heavy", alias = "9")]
    Heavy = 9,
    /// 검정
    #[serde(rename = "Black", alias = "10")]
    Black = 10,
    /// 북쪽 검정
    #[serde(rename = "Nord", alias = "11")]
    Nord = 11,
}

/// [AI 생성] PANOSE 비례
///
/// PANOSE 분류 체계의 비례 (Proportion)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum PanoseProportion {
    /// 임의
    #[default]
    #[serde(rename = "Any", alias = "0")]
    Any = 0,
    /// 해당 없음
    #[serde(rename = "NoFit", alias = "1")]
    NoFit = 1,
    /// Old Style
    #[serde(rename = "OldStyle", alias = "2")]
    OldStyle = 2,
    /// Modern
    #[serde(rename = "Modern", alias = "3")]
    Modern = 3,
    /// Even Width
    #[serde(rename = "EvenWidth", alias = "4")]
    EvenWidth = 4,
    /// Extended
    #[serde(rename = "Extended", alias = "5")]
    Extended = 5,
    /// Condensed
    #[serde(rename = "Condensed", alias = "6")]
    Condensed = 6,
    /// Very Extended
    #[serde(rename = "VeryExtended", alias = "7")]
    VeryExtended = 7,
    /// Very Condensed
    #[serde(rename = "VeryCondensed", alias = "8")]
    VeryCondensed = 8,
    /// Monospaced
    #[serde(rename = "Monospaced", alias = "9")]
    Monospaced = 9,
}

/// [AI 생성] PANOSE 대조
///
/// PANOSE 분류 체계의 대조 (Contrast)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum PanoseContrast {
    /// 임의
    #[default]
    #[serde(rename = "Any", alias = "0")]
    Any = 0,
    /// 해당 없음
    #[serde(rename = "NoFit", alias = "1")]
    NoFit = 1,
    /// 없음
    #[serde(rename = "None", alias = "2")]
    None = 2,
    /// 매우 낮음
    #[serde(rename = "VeryLow", alias = "3")]
    VeryLow = 3,
    /// 낮음
    #[serde(rename = "Low", alias = "4")]
    Low = 4,
    /// 약간 낮음
    #[serde(rename = "MediumLow", alias = "5")]
    MediumLow = 5,
    /// 중간
    #[serde(rename = "Medium", alias = "6")]
    Medium = 6,
    /// 약간 높음
    #[serde(rename = "MediumHigh", alias = "7")]
    MediumHigh = 7,
    /// 높음
    #[serde(rename = "High", alias = "8")]
    High = 8,
    /// 매우 높음
    #[serde(rename = "VeryHigh", alias = "9")]
    VeryHigh = 9,
}

/// [AI 생성] PANOSE 스트로크 편차
///
/// PANOSE 분류 체계의 스트로크 편차 (Stroke Variation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum PanoseStrokeVariation {
    /// 임의
    #[default]
    #[serde(rename = "Any", alias = "0")]
    Any = 0,
    /// 해당 없음
    #[serde(rename = "NoFit", alias = "1")]
    NoFit = 1,
    /// No Variation
    #[serde(rename = "NoVariation", alias = "2")]
    NoVariation = 2,
    /// Gradual/Diagonal
    #[serde(rename = "GradualDiagonal", alias = "3")]
    GradualDiagonal = 3,
    /// Gradual/Transitional
    #[serde(rename = "GradualTransitional", alias = "4")]
    GradualTransitional = 4,
    /// Gradual/Vertical
    #[serde(rename = "GradualVertical", alias = "5")]
    GradualVertical = 5,
    /// Gradual/Horizontal
    #[serde(rename = "GradualHorizontal", alias = "6")]
    GradualHorizontal = 6,
    /// Rapid/Vertical
    #[serde(rename = "RapidVertical", alias = "7")]
    RapidVertical = 7,
    /// Rapid/Horizontal
    #[serde(rename = "RapidHorizontal", alias = "8")]
    RapidHorizontal = 8,
    /// Instant/Vertical
    #[serde(rename = "InstantVertical", alias = "9")]
    InstantVertical = 9,
    /// Instant/Horizontal
    #[serde(rename = "InstantHorizontal", alias = "10")]
    InstantHorizontal = 10,
}

/// [AI 생성] PANOSE X-높이
///
/// PANOSE 분류 체계의 X-높이 (X-Height)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum PanoseXHeight {
    /// 임의
    #[default]
    #[serde(rename = "Any", alias = "0")]
    Any = 0,
    /// 해당 없음
    #[serde(rename = "NoFit", alias = "1")]
    NoFit = 1,
    /// Constant/Small
    #[serde(rename = "ConstantSmall", alias = "2")]
    ConstantSmall = 2,
    /// Constant/Standard
    #[serde(rename = "ConstantStandard", alias = "3")]
    ConstantStandard = 3,
    /// Constant/Large
    #[serde(rename = "ConstantLarge", alias = "4")]
    ConstantLarge = 4,
    /// Ducking/Small
    #[serde(rename = "DuckingSmall", alias = "5")]
    DuckingSmall = 5,
    /// Ducking/Standard
    #[serde(rename = "DuckingStandard", alias = "6")]
    DuckingStandard = 6,
    /// Ducking/Large
    #[serde(rename = "DuckingLarge", alias = "7")]
    DuckingLarge = 7,
}

/// [AI 생성] PANOSE 자획 유형
///
/// PANOSE 분류 체계의 자획 유형 (Arm Style)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum PanoseArmStyle {
    /// 임의
    #[default]
    #[serde(rename = "Any", alias = "0")]
    Any = 0,
    /// 해당 없음
    #[serde(rename = "NoFit", alias = "1")]
    NoFit = 1,
    /// Straight Arms/Horizontal
    #[serde(rename = "StraightArmsHorizontal", alias = "2")]
    StraightArmsHorizontal = 2,
    /// Straight Arms/Wedge
    #[serde(rename = "StraightArmsWedge", alias = "3")]
    StraightArmsWedge = 3,
    /// Straight Arms/Vertical
    #[serde(rename = "StraightArmsVertical", alias = "4")]
    StraightArmsVertical = 4,
    /// Straight Arms/Single Serif
    #[serde(rename = "StraightArmsSingleSerif", alias = "5")]
    StraightArmsSingleSerif = 5,
    /// Straight Arms/Double Serif
    #[serde(rename = "StraightArmsDoubleSerif", alias = "6")]
    StraightArmsDoubleSerif = 6,
    /// Non-Straight/Horizontal
    #[serde(rename = "NonStraightHorizontal", alias = "7")]
    NonStraightHorizontal = 7,
    /// Non-Straight/Wedge
    #[serde(rename = "NonStraightWedge", alias = "8")]
    NonStraightWedge = 8,
    /// Non-Straight/Vertical
    #[serde(rename = "NonStraightVertical", alias = "9")]
    NonStraightVertical = 9,
    /// Non-Straight/Single Serif
    #[serde(rename = "NonStraightSingleSerif", alias = "10")]
    NonStraightSingleSerif = 10,
    /// Non-Straight/Double Serif
    #[serde(rename = "NonStraightDoubleSerif", alias = "11")]
    NonStraightDoubleSerif = 11,
}

/// [AI 생성] PANOSE 글자형
///
/// PANOSE 분류 체계의 글자형 (Letterform)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum PanoseLetterform {
    /// 임의
    #[default]
    #[serde(rename = "Any", alias = "0")]
    Any = 0,
    /// 해당 없음
    #[serde(rename = "NoFit", alias = "1")]
    NoFit = 1,
    /// Normal/Contact
    #[serde(rename = "NormalContact", alias = "2")]
    NormalContact = 2,
    /// Normal/Weighted
    #[serde(rename = "NormalWeighted", alias = "3")]
    NormalWeighted = 3,
    /// Normal/Boxed
    #[serde(rename = "NormalBoxed", alias = "4")]
    NormalBoxed = 4,
    /// Normal/Flattened
    #[serde(rename = "NormalFlattened", alias = "5")]
    NormalFlattened = 5,
    /// Normal/Rounded
    #[serde(rename = "NormalRounded", alias = "6")]
    NormalRounded = 6,
    /// Normal/Off Center
    #[serde(rename = "NormalOffCenter", alias = "7")]
    NormalOffCenter = 7,
    /// Normal/Square
    #[serde(rename = "NormalSquare", alias = "8")]
    NormalSquare = 8,
    /// Oblique/Contact
    #[serde(rename = "ObliqueContact", alias = "9")]
    ObliqueContact = 9,
    /// Oblique/Weighted
    #[serde(rename = "ObliqueWeighted", alias = "10")]
    ObliqueWeighted = 10,
    /// Oblique/Boxed
    #[serde(rename = "ObliqueBoxed", alias = "11")]
    ObliqueBoxed = 11,
    /// Oblique/Flattened
    #[serde(rename = "ObliqueFlattened", alias = "12")]
    ObliqueFlattened = 12,
    /// Oblique/Rounded
    #[serde(rename = "ObliqueRounded", alias = "13")]
    ObliqueRounded = 13,
    /// Oblique/Off Center
    #[serde(rename = "ObliqueOffCenter", alias = "14")]
    ObliqueOffCenter = 14,
    /// Oblique/Square
    #[serde(rename = "ObliqueSquare", alias = "15")]
    ObliqueSquare = 15,
}

/// [AI 생성] PANOSE 중간선
///
/// PANOSE 분류 체계의 중간선 (Midline)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum PanoseMidline {
    /// 임의
    #[default]
    #[serde(rename = "Any", alias = "0")]
    Any = 0,
    /// 해당 없음
    #[serde(rename = "NoFit", alias = "1")]
    NoFit = 1,
    /// Standard/Trimmed
    #[serde(rename = "StandardTrimmed", alias = "2")]
    StandardTrimmed = 2,
    /// Standard/Pointed
    #[serde(rename = "StandardPointed", alias = "3")]
    StandardPointed = 3,
    /// Standard/Serifed
    #[serde(rename = "StandardSerifed", alias = "4")]
    StandardSerifed = 4,
    /// High/Trimmed
    #[serde(rename = "HighTrimmed", alias = "5")]
    HighTrimmed = 5,
    /// High/Pointed
    #[serde(rename = "HighPointed", alias = "6")]
    HighPointed = 6,
    /// High/Serifed
    #[serde(rename = "HighSerifed", alias = "7")]
    HighSerifed = 7,
    /// Constant/Trimmed
    #[serde(rename = "ConstantTrimmed", alias = "8")]
    ConstantTrimmed = 8,
    /// Constant/Pointed
    #[serde(rename = "ConstantPointed", alias = "9")]
    ConstantPointed = 9,
    /// Constant/Serifed
    #[serde(rename = "ConstantSerifed", alias = "10")]
    ConstantSerifed = 10,
    /// Low/Trimmed
    #[serde(rename = "LowTrimmed", alias = "11")]
    LowTrimmed = 11,
    /// Low/Pointed
    #[serde(rename = "LowPointed", alias = "12")]
    LowPointed = 12,
    /// Low/Serifed
    #[serde(rename = "LowSerifed", alias = "13")]
    LowSerifed = 13,
}

/// [AI 생성] 글꼴 타입 정보 (PANOSE)
///
/// PANOSE (Parametric description of the Appearance of Printed Normal Operating System Environment)
/// 글꼴 분류 체계를 표현합니다.
///
/// 원본: `typeInfo` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "typeInfo")]
pub struct FontTypeInfo {
    /// [AI 생성] 글꼴 계열 (PANOSE Family Kind) (`familyType` 속성)
    #[serde(rename = "@familyType")]
    pub family_type: FontFamilyType,

    /// [AI 생성] 세리프 유형 (PANOSE Serif Style) (`serifStyle` 속성)
    #[serde(rename = "@serifStyle", skip_serializing_if = "Option::is_none")]
    pub serif_style: Option<PanoseSerifStyle>,

    /// [AI 생성] 굵기 (PANOSE Weight) (`weight` 속성)
    #[serde(rename = "@weight")]
    pub weight: PanoseWeight,

    /// [AI 생성] 비례 (PANOSE Proportion) (`proportion` 속성)
    #[serde(rename = "@proportion")]
    pub proportion: PanoseProportion,

    /// [AI 생성] 대조 (PANOSE Contrast) (`contrast` 속성)
    #[serde(rename = "@contrast")]
    pub contrast: PanoseContrast,

    /// [AI 생성] 스트로크 편차 (PANOSE Stroke Variation) (`strokeVariation` 속성)
    #[serde(rename = "@strokeVariation")]
    pub stroke_variation: PanoseStrokeVariation,

    /// [AI 생성] 자획 유형 (PANOSE Arm Style) (`armStyle` 속성)
    #[serde(rename = "@armStyle")]
    pub arm_style: PanoseArmStyle,

    /// [AI 생성] 글자형 (PANOSE Letterform) (`letterform` 속성)
    #[serde(rename = "@letterform")]
    pub letterform: PanoseLetterform,

    /// [AI 생성] 중간선 (PANOSE Midline) (`midline` 속성)
    #[serde(rename = "@midline")]
    pub midline: PanoseMidline,

    /// [AI 생성] X-높이 (PANOSE X-Height) (`xHeight` 속성)
    #[serde(rename = "@xHeight")]
    pub x_height: PanoseXHeight,
}

/// [AI 생성] 대체 글꼴
///
/// 원본: `substFont` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "substFont")]
pub struct SubstituteFont {
    /// [AI 생성] 글꼴 이름 (`face` 속성)
    #[serde(rename = "@face")]
    pub face: String,

    /// [AI 생성] 글꼴 유형 (`type` 속성)
    #[serde(rename = "@type")]
    pub font_type: FontType,

    /// [AI 생성] 내장 여부 (`isEmbedded` 속성)
    #[serde(rename = "@isEmbedded", default)]
    pub is_embedded: bool,

    /// [AI 생성] 바이너리 아이템 아이디 참조 (`binaryItemIDRef` 속성)
    #[serde(rename = "@binaryItemIDRef", skip_serializing_if = "Option::is_none")]
    pub binary_item_id_reference: Option<BinaryItemIdRef>,
}

/// [AI 생성] 글꼴
///
/// 원본: `font` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "font")]
pub struct Font {
    /// [AI 생성] 대체 글꼴 (`substFont` 요소)
    #[serde(rename = "substFont", skip_serializing_if = "Option::is_none")]
    pub substitute_font: Option<SubstituteFont>,

    /// [AI 생성] 글꼴 정보 (`typeInfo` 요소)
    #[serde(rename = "typeInfo", skip_serializing_if = "Option::is_none")]
    pub type_info: Option<FontTypeInfo>,

    /// [AI 생성] 글꼴 아이디 (`id` 속성)
    #[serde(rename = "@id")]
    pub id: u32,

    /// [AI 생성] 글꼴 이름 (`face` 속성)
    #[serde(rename = "@face")]
    pub face: String,

    /// [AI 생성] 글꼴 유형 (`type` 속성)
    #[serde(rename = "@type")]
    pub font_type: FontType,

    /// [AI 생성] 내장 여부 (`isEmbedded` 속성)
    #[serde(rename = "@isEmbedded", default)]
    pub is_embedded: bool,

    /// [AI 생성] 바이너리 아이템 아이디 참조 (`binaryItemIDRef` 속성)
    #[serde(rename = "@binaryItemIDRef", skip_serializing_if = "Option::is_none")]
    pub binary_item_id_reference: Option<BinaryItemIdRef>,
}

/// [AI 생성] 언어별 글꼴 그룹
///
/// 원본: `FontfaceType`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "fontface")]
pub struct Fontface {
    /// [AI 생성] 글꼴 목록 (`font` 요소)
    #[serde(rename = "font")]
    pub fonts: Vec<Font>,

    /// [AI 생성] 언어 (`lang` 속성)
    #[serde(rename = "@lang")]
    pub language: FontLanguage,

    /// [AI 생성] 글꼴 개수 (`fontCnt` 속성)
    #[serde(rename = "@fontCnt")]
    pub font_count: u32,
}

/// [AI 생성] 글꼴 목록
///
/// 원본: `fontfaces` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "fontfaces")]
pub struct FontfaceList {
    /// [AI 생성] 언어별 글꼴 그룹 목록 (`fontface` 요소)
    #[serde(rename = "fontface")]
    pub fontfaces: Vec<Fontface>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성)
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}
