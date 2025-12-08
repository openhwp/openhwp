//! [AI 생성 문서화] 핵심 열거형 타입들
//!
//! KS X 6101:2024 - core.xsd를 기준으로 작성된 AI 생성 문서입니다. 실제 스키마(`docs/hwpx/schemas/core.xsd`)와 다르면 TODO로 표시하거나 보완해 주세요.

use serde::{Deserialize, Serialize};

/// 번호 형식 타입 1
///
/// 원본: `NumberType1`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum NumberFormatType1 {
    /// 1, 2, 3
    #[default]
    #[serde(rename = "DIGIT")]
    Digit,
    /// 동그라미 쳐진 1, 2, 3
    #[serde(rename = "CIRCLED_DIGIT")]
    CircledDigit,
    /// I, II, III
    #[serde(rename = "ROMAN_CAPITAL")]
    RomanCapital,
    /// i, ii, iii
    #[serde(rename = "ROMAN_SMALL")]
    RomanSmall,
    /// A, B, C
    #[serde(rename = "LATIN_CAPITAL")]
    LatinCapital,
    /// a, b, c
    #[serde(rename = "LATIN_SMALL")]
    LatinSmall,
    /// 동그라미 쳐진 A, B, C
    #[serde(rename = "CIRCLED_LATIN_CAPITAL")]
    CircledLatinCapital,
    /// 동그라미 쳐진 a, b, c
    #[serde(rename = "CIRCLED_LATIN_SMALL")]
    CircledLatinSmall,
    /// 가, 나, 다
    #[serde(rename = "HANGUL_SYLLABLE")]
    HangulSyllable,
    /// 동그라미 쳐진 가, 나, 다
    #[serde(rename = "CIRCLED_HANGUL_SYLLABLE")]
    CircledHangulSyllable,
    /// ㄱ, ㄴ, ㄷ
    #[serde(rename = "HANGUL_JAMO")]
    HangulJamo,
    /// 동그라미 쳐진 ㄱ, ㄴ, ㄷ
    #[serde(rename = "CIRCLED_HANGUL_JAMO")]
    CircledHangulJamo,
    /// 일, 이, 삼
    #[serde(rename = "HANGUL_PHONETIC")]
    HangulPhonetic,
    /// 一, 二, 三
    #[serde(rename = "IDEOGRAPH")]
    Ideograph,
    /// 동그라미 쳐진 一, 二, 三
    #[serde(rename = "CIRCLED_IDEOGRAPH")]
    CircledIdeograph,
}

/// 번호 형식 타입 2 (NumberType1 확장)
///
/// 원본: `NumberType2`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum NumberFormatType2 {
    /// 1, 2, 3
    #[default]
    #[serde(rename = "DIGIT")]
    Digit,
    /// 동그라미 쳐진 1, 2, 3
    #[serde(rename = "CIRCLED_DIGIT")]
    CircledDigit,
    /// I, II, III
    #[serde(rename = "ROMAN_CAPITAL")]
    RomanCapital,
    /// i, ii, iii
    #[serde(rename = "ROMAN_SMALL")]
    RomanSmall,
    /// A, B, C
    #[serde(rename = "LATIN_CAPITAL")]
    LatinCapital,
    /// a, b, c
    #[serde(rename = "LATIN_SMALL")]
    LatinSmall,
    /// 동그라미 쳐진 A, B, C
    #[serde(rename = "CIRCLED_LATIN_CAPITAL")]
    CircledLatinCapital,
    /// 동그라미 쳐진 a, b, c
    #[serde(rename = "CIRCLED_LATIN_SMALL")]
    CircledLatinSmall,
    /// 가, 나, 다
    #[serde(rename = "HANGUL_SYLLABLE")]
    HangulSyllable,
    /// 동그라미 쳐진 가, 나, 다
    #[serde(rename = "CIRCLED_HANGUL_SYLLABLE")]
    CircledHangulSyllable,
    /// ㄱ, ㄴ, ㄷ
    #[serde(rename = "HANGUL_JAMO")]
    HangulJamo,
    /// 동그라미 쳐진 ㄱ, ㄴ, ㄷ
    #[serde(rename = "CIRCLED_HANGUL_JAMO")]
    CircledHangulJamo,
    /// 일, 이, 삼
    #[serde(rename = "HANGUL_PHONETIC")]
    HangulPhonetic,
    /// 一, 二, 三
    #[serde(rename = "IDEOGRAPH")]
    Ideograph,
    /// 동그라미 쳐진 一, 二, 三
    #[serde(rename = "CIRCLED_IDEOGRAPH")]
    CircledIdeograph,
    /// 갑, 을, 병, 정, 무, 기, 경, 신, 임, 계
    #[serde(rename = "DECAGON_CIRCLE")]
    DecagonCircle,
    /// 甲, 乙, 丙, 丁, 戊, 己, 庚, 辛, 壬, 癸
    #[serde(rename = "DECAGON_CIRCLE_HANJA")]
    DecagonCircleHanja,
    /// 4가지 문자가 차례로 반복
    #[serde(rename = "SYMBOL")]
    Symbol,
    /// 사용자 지정 문자 반복
    #[serde(rename = "USER_CHAR")]
    UserCharacter,
}

/// 선 종류 타입 1
///
/// 원본: `LineType1`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LineStyleType1 {
    /// 없음
    #[serde(rename = "NONE")]
    None,
    /// 실선
    #[default]
    #[serde(rename = "SOLID")]
    Solid,
    /// 점선
    #[serde(rename = "DOT")]
    Dot,
    /// 두꺼운 선
    #[serde(rename = "THICK")]
    Thick,
    /// 긴 점선
    #[serde(rename = "DASH")]
    Dash,
    /// -.-
    #[serde(rename = "DASH_DOT")]
    DashDot,
    /// -..-
    #[serde(rename = "DASH_DOT_DOT")]
    DashDotDot,
}

/// 선 종류 타입 2
///
/// 원본: `LineType2`
///
/// 참고: fixture 파일에서 숫자 형태("0", "1" 등)도 발견되어 alias로 지원합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LineStyleType2 {
    /// 선 없음
    #[serde(rename = "NONE", alias = "0")]
    None,
    /// 실선
    #[default]
    #[serde(rename = "SOLID", alias = "1")]
    Solid,
    /// 점선
    #[serde(rename = "DOT", alias = "2")]
    Dot,
    /// 긴 점선
    #[serde(rename = "DASH", alias = "3")]
    Dash,
    /// -.-
    #[serde(rename = "DASH_DOT", alias = "4")]
    DashDot,
    /// -..-
    #[serde(rename = "DASH_DOT_DOT", alias = "5")]
    DashDotDot,
    /// DASH보다 긴 선의 반복
    #[serde(rename = "LONG_DASH", alias = "6")]
    LongDash,
    /// DOT보다 큰 동그라미의 반복
    #[serde(rename = "CIRCLE", alias = "7")]
    Circle,
    /// 2중선(가는 선 + 가는 선)
    #[serde(rename = "DOUBLE_SLIM", alias = "8")]
    DoubleSlim,
    /// 2중선(가는 선 + 굵은 선)
    #[serde(rename = "SLIM_THICK", alias = "9")]
    SlimThick,
    /// 2중선(굵은 선 + 가는 선)
    #[serde(rename = "THICK_SLIM", alias = "10")]
    ThickSlim,
    /// 3중선(가는 선 + 굵은 선 + 가는 선)
    #[serde(rename = "SLIM_THICK_SLIM", alias = "11")]
    SlimThickSlim,
}

/// 선 종류 타입 3 (LineType2 + Wave)
///
/// 원본: `LineType3`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LineStyleType3 {
    /// 선 없음
    #[serde(rename = "NONE")]
    None,
    /// 실선
    #[default]
    #[serde(rename = "SOLID")]
    Solid,
    /// 점선
    #[serde(rename = "DOT")]
    Dot,
    /// 긴 점선
    #[serde(rename = "DASH")]
    Dash,
    /// -.-
    #[serde(rename = "DASH_DOT")]
    DashDot,
    /// -..-
    #[serde(rename = "DASH_DOT_DOT")]
    DashDotDot,
    /// DASH보다 긴 선의 반복
    #[serde(rename = "LONG_DASH")]
    LongDash,
    /// DOT보다 큰 동그라미의 반복
    #[serde(rename = "CIRCLE")]
    Circle,
    /// 2중선(가는 선 + 가는 선)
    #[serde(rename = "DOUBLE_SLIM")]
    DoubleSlim,
    /// 2중선(가는 선 + 굵은 선)
    #[serde(rename = "SLIM_THICK")]
    SlimThick,
    /// 2중선(굵은 선 + 가는 선)
    #[serde(rename = "THICK_SLIM")]
    ThickSlim,
    /// 3중선(가는 선 + 굵은 선 + 가는 선)
    #[serde(rename = "SLIM_THICK_SLIM")]
    SlimThickSlim,
    /// 물결
    #[serde(rename = "WAVE")]
    Wave,
    /// 이중 물결
    #[serde(rename = "DOUBLEWAVE")]
    DoubleWave,
}

/// 선 굵기
///
/// 원본: `LineWidth`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LineWidth {
    /// 0.1 mm
    #[serde(rename = "0.1 mm")]
    Mm0_1,
    /// 0.12 mm
    #[default]
    #[serde(rename = "0.12 mm")]
    Mm0_12,
    /// 0.15 mm
    #[serde(rename = "0.15 mm")]
    Mm0_15,
    /// 0.2 mm
    #[serde(rename = "0.2 mm")]
    Mm0_2,
    /// 0.25 mm
    #[serde(rename = "0.25 mm")]
    Mm0_25,
    /// 0.3 mm
    #[serde(rename = "0.3 mm")]
    Mm0_3,
    /// 0.4 mm
    #[serde(rename = "0.4 mm")]
    Mm0_4,
    /// 0.5 mm
    #[serde(rename = "0.5 mm")]
    Mm0_5,
    /// 0.6 mm
    #[serde(rename = "0.6 mm")]
    Mm0_6,
    /// 0.7 mm
    #[serde(rename = "0.7 mm")]
    Mm0_7,
    /// 1.0 mm
    #[serde(rename = "1.0 mm")]
    Mm1_0,
    /// 1.5 mm
    #[serde(rename = "1.5 mm")]
    Mm1_5,
    /// 2.0 mm
    #[serde(rename = "2.0 mm")]
    Mm2_0,
    /// 3.0 mm
    #[serde(rename = "3.0 mm")]
    Mm3_0,
    /// 4.0 mm
    #[serde(rename = "4.0 mm")]
    Mm4_0,
    /// 5.0 mm
    #[serde(rename = "5.0 mm")]
    Mm5_0,
}

/// 정렬 스타일 (9방향)
///
/// 원본: `AlignStyleType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AlignmentStyle {
    /// 왼쪽 위
    #[serde(rename = "TOP_LEFT")]
    TopLeft,
    /// 위쪽 가운데
    #[serde(rename = "TOP")]
    Top,
    /// 오른쪽 위
    #[serde(rename = "TOP_RIGHT")]
    TopRight,
    /// 왼쪽 가운데
    #[serde(rename = "LEFT")]
    Left,
    /// 가운데
    #[default]
    #[serde(rename = "CENTER")]
    Center,
    /// 오른쪽 가운데
    #[serde(rename = "RIGHT")]
    Right,
    /// 왼쪽 아래
    #[serde(rename = "BOTTOM_LEFT")]
    BottomLeft,
    /// 아래쪽 가운데
    #[serde(rename = "BOTTOM")]
    Bottom,
    /// 오른쪽 아래
    #[serde(rename = "BOTTOM_RIGHT")]
    BottomRight,
}

/// 화살표 종류
///
/// 원본: `ArrowType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ArrowStyle {
    /// 없음
    #[default]
    #[serde(rename = "NORMAL")]
    Normal,
    /// 화살표
    #[serde(rename = "ARROW")]
    Arrow,
    /// 창
    #[serde(rename = "SPEAR")]
    Spear,
    /// 오목 화살표
    #[serde(rename = "CONCAVE_ARROW")]
    ConcaveArrow,
    /// 빈 다이아몬드
    #[serde(rename = "EMPTY_DIAMOND")]
    EmptyDiamond,
    /// 빈 원
    #[serde(rename = "EMPTY_CIRCLE")]
    EmptyCircle,
    /// 빈 사각형
    #[serde(rename = "EMPTY_BOX")]
    EmptyBox,
    /// 채운 다이아몬드
    #[serde(rename = "FILLED_DIAMOND")]
    FilledDiamond,
    /// 채운 원
    #[serde(rename = "FILLED_CIRCLE")]
    FilledCircle,
    /// 채운 사각형
    #[serde(rename = "FILLED_BOX")]
    FilledBox,
}

/// 화살표 크기
///
/// 원본: `ArrowSize`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ArrowSize {
    /// 작은-작은
    #[default]
    #[serde(rename = "SMALL_SMALL")]
    SmallSmall,
    /// 작은-보통
    #[serde(rename = "SMALL_MEDIUM")]
    SmallMedium,
    /// 작은-큰
    #[serde(rename = "SMALL_LARGE")]
    SmallLarge,
    /// 보통-작은
    #[serde(rename = "MEDIUM_SMALL")]
    MediumSmall,
    /// 보통-보통
    #[serde(rename = "MEDIUM_MEDIUM")]
    MediumMedium,
    /// 보통-큰
    #[serde(rename = "MEDIUM_LARGE")]
    MediumLarge,
    /// 큰-작은
    #[serde(rename = "LARGE_SMALL")]
    LargeSmall,
    /// 큰-보통
    #[serde(rename = "LARGE_MEDIUM")]
    LargeMedium,
    /// 큰-큰
    #[serde(rename = "LARGE_LARGE")]
    LargeLarge,
}

/// 변경 추적 종류
///
/// 원본: `TrackChangeType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TrackChangeType {
    /// 알 수 없음
    #[default]
    #[serde(rename = "UnKown")]
    Unknown,
    /// 삽입
    #[serde(rename = "Insert")]
    Insert,
    /// 삭제
    #[serde(rename = "Delete")]
    Delete,
    /// 글자 모양
    #[serde(rename = "CharShape")]
    CharacterShape,
    /// 문단 모양
    #[serde(rename = "ParaShape")]
    ParagraphShape,
}

/// 덧대글 스타일
///
/// 원본: `DropCapStyleType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DropCapStyle {
    /// 없음
    #[default]
    #[serde(rename = "None")]
    None,
    /// 이중선
    #[serde(rename = "DoubleLine")]
    DoubleLine,
    /// 삼중선
    #[serde(rename = "TripleLine")]
    TripleLine,
    /// 여백
    #[serde(rename = "Margin")]
    Margin,
}

/// 테두리선 종류 (BorderAttributeGroup용)
///
/// 원본: `type` 속성의 익명 타입 (BorderAttributeGroup)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BorderLineStyle {
    /// 실선
    #[default]
    #[serde(rename = "Solid")]
    Solid,
    /// 긴 점선
    #[serde(rename = "Dash")]
    Dash,
    /// 점선
    #[serde(rename = "Dot")]
    Dot,
    /// -.-
    #[serde(rename = "DashDot")]
    DashDot,
    /// -..-
    #[serde(rename = "DashDotDot")]
    DashDotDot,
    /// Dash보다 긴 선의 반복
    #[serde(rename = "LongDash")]
    LongDash,
    /// Dot보다 큰 동그라미의 반복
    #[serde(rename = "Circle")]
    Circle,
    /// 2중선
    #[serde(rename = "DoubleSlim")]
    DoubleSlim,
    /// 가는 선 + 굵은 선 2중선
    #[serde(rename = "SlimThick")]
    SlimThick,
    /// 굵은 선 + 가는 선 2중선
    #[serde(rename = "ThickSlim")]
    ThickSlim,
    /// 가는 선 + 굵은 선 + 가는 선 3중선
    #[serde(rename = "SlimThickSlim")]
    SlimThickSlim,
    /// 선 없음
    #[serde(rename = "None")]
    None,
}

/// 테두리선 굵기 (BorderAttributeGroup용)
///
/// 원본: `width` 속성의 익명 타입 (BorderAttributeGroup)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BorderLineWidth {
    /// 0.1mm
    #[serde(rename = "0.1mm")]
    Mm0_1,
    /// 0.12mm
    #[default]
    #[serde(rename = "0.12mm")]
    Mm0_12,
    /// 0.15mm
    #[serde(rename = "0.15mm")]
    Mm0_15,
    /// 0.2mm
    #[serde(rename = "0.2mm")]
    Mm0_2,
    /// 0.25mm
    #[serde(rename = "0.25mm")]
    Mm0_25,
    /// 0.3mm
    #[serde(rename = "0.3mm")]
    Mm0_3,
    /// 0.4mm
    #[serde(rename = "0.4mm")]
    Mm0_4,
    /// 0.5mm
    #[serde(rename = "0.5mm")]
    Mm0_5,
    /// 0.6mm
    #[serde(rename = "0.6mm")]
    Mm0_6,
    /// 0.7mm
    #[serde(rename = "0.7mm")]
    Mm0_7,
    /// 1.0mm
    #[serde(rename = "1.0mm")]
    Mm1_0,
    /// 1.5mm
    #[serde(rename = "1.5mm")]
    Mm1_5,
    /// 2.0mm
    #[serde(rename = "2.0mm")]
    Mm2_0,
    /// 3.0mm
    #[serde(rename = "3.0mm")]
    Mm3_0,
    /// 4.0mm
    #[serde(rename = "4.0mm")]
    Mm4_0,
    /// 5.0mm
    #[serde(rename = "5.0mm")]
    Mm5_0,
}

/// HWPUNIT 또는 CHAR 단위
///
/// 원본: `unit` 속성의 익명 타입 (HWPValue)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ValueUnit {
    /// 글자 단위
    #[serde(rename = "CHAR")]
    Character,
    /// HWPUNIT 단위
    #[default]
    #[serde(rename = "HWPUNIT")]
    HwpUnit,
}

/// 그림 효과
///
/// 원본: `effect` 속성의 익명 타입 (imageType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ImageEffect {
    /// 원래 그림에서
    #[default]
    #[serde(rename = "REAL_PIC")]
    RealPicture,
    /// 그레이스케일로
    #[serde(rename = "GRAY_SCALE")]
    GrayScale,
    /// 흑백으로
    #[serde(rename = "BLACK_WHITE")]
    BlackWhite,
}

/// 무늬 종류
///
/// 원본: `hatchStyle` 속성의 익명 타입 (winBrush)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HatchStyle {
    /// - - - -
    #[serde(rename = "HORIZONTAL")]
    Horizontal,
    /// |||||
    #[serde(rename = "VERTICAL")]
    Vertical,
    /// \\\\\\
    #[serde(rename = "BACK_SLASH")]
    BackSlash,
    /// //////
    #[serde(rename = "SLASH")]
    Slash,
    /// +++++
    #[serde(rename = "CROSS")]
    Cross,
    /// xxxxx
    #[serde(rename = "CROSS_DIAGONAL")]
    CrossDiagonal,
}

/// 그라데이션 유형
///
/// 원본: `type` 속성의 익명 타입 (gradation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GradationType {
    /// 줄무늬형
    #[serde(rename = "LINEAR")]
    Linear,
    /// 원형
    #[serde(rename = "RADIAL")]
    Radial,
    /// 원뿔형
    #[serde(rename = "CONICAL")]
    Conical,
    /// 사각형
    #[serde(rename = "SQUARE")]
    Square,
}

/// 이미지 브러시 모드
///
/// 원본: `mode` 속성의 익명 타입 (imgBrush)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ImageBrushMode {
    /// 바둑판식으로-모두
    #[default]
    #[serde(rename = "TILE")]
    Tile,
    /// 바둑판식으로-가로/위
    #[serde(rename = "TILE_HORZ_TOP")]
    TileHorizontalTop,
    /// 바둑판식으로-가로/아래
    #[serde(rename = "TILE_HORZ_BOTTOM")]
    TileHorizontalBottom,
    /// 바둑판식으로-세로/왼쪽
    #[serde(rename = "TILE_VERT_LEFT")]
    TileVerticalLeft,
    /// 바둑판식으로-세로/오른쪽
    #[serde(rename = "TILE_VERT_RIGHT")]
    TileVerticalRight,
    /// 크기에 맞추어
    #[serde(rename = "TOTAL")]
    Total,
    /// 가운데로
    #[serde(rename = "CENTER")]
    Center,
    /// 가운데 위로
    #[serde(rename = "CENTER_TOP")]
    CenterTop,
    /// 가운데 아래로
    #[serde(rename = "CENTER_BOTTOM")]
    CenterBottom,
    /// 왼쪽 가운데로
    #[serde(rename = "LEFT_CENTER")]
    LeftCenter,
    /// 왼쪽 위로
    #[serde(rename = "LEFT_TOP")]
    LeftTop,
    /// 왼쪽 아래로
    #[serde(rename = "LEFT_BOTTOM")]
    LeftBottom,
    /// 오른쪽 가운데로
    #[serde(rename = "RIGHT_CENTER")]
    RightCenter,
    /// 오른쪽 위로
    #[serde(rename = "RIGHT_TOP")]
    RightTop,
    /// 오른쪽 아래로
    #[serde(rename = "RIGHT_BOTTOM")]
    RightBottom,
    /// 확대
    #[serde(rename = "ZOOM")]
    Zoom,
}
