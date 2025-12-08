//! [AI 생성 문서화] 글자 모양
//!
//! 문서 전역 글꼴/자간/장평/색/효과(밑줄·취소선·외곽선·그림자 등)를 묶어 동일한 텍스트 렌더링을 보장합니다. 스크립트별 설정을 모두 포함하므로 호환 뷰어가 동일한 모양을 재현할 때 사용됩니다. KS X 6101:2024 `header.xsd` 기반이며 세부는 `docs/hwpx/schemas/header.xsd` 참고.

use serde::{Deserialize, Serialize};

use crate::core::{
    enums::{LineStyleType1, LineStyleType2},
    types::{BorderFillIdRef, RgbColor},
};

/// [AI 생성] 밑줄 위치
///
/// 원본: `underline.type` 속성의 익명 타입. 밑줄이 배치될 위치를 정의합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum UnderlinePosition {
    /// 없음
    #[default]
    #[serde(rename = "NONE")]
    None,
    /// 글자 아래
    #[serde(rename = "BOTTOM")]
    Bottom,
    /// 글자 중간
    #[serde(rename = "CENTER")]
    Center,
    /// 글자 위
    #[serde(rename = "TOP")]
    Top,
}

/// [AI 생성] 그림자 유형
///
/// 원본: `shadow.type` 속성의 익명 타입. 글자 그림자 효과 종류를 지정합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ShadowType {
    /// 없음
    #[default]
    #[serde(rename = "NONE")]
    None,
    /// 그림자 (drop shadow)
    #[serde(rename = "DROP")]
    Drop,
    /// 연속 그림자
    #[serde(rename = "CONTINUOUS")]
    Continuous,
}

/// [AI 생성] 강조점 유형
///
/// 원본: `symMark` 속성의 익명 타입. 문자 위·아래 강조표시를 지정합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EmphasisMarkType {
    /// 없음
    #[default]
    #[serde(rename = "NONE")]
    None,
    /// 검정 동그라미 (위쪽)
    #[serde(rename = "DOT_ABOVE")]
    DotAbove,
    /// 속 빈 동그라미 (위쪽)
    #[serde(rename = "RING_ABOVE")]
    RingAbove,
    /// 물결
    #[serde(rename = "TILDE")]
    Tilde,
    /// 갈매기형 (위)
    #[serde(rename = "CARON")]
    Caron,
    /// 측점
    #[serde(rename = "SIDE")]
    Side,
    /// 콜론
    #[serde(rename = "COLON")]
    Colon,
    /// 왼쪽 위 틱 (그레이브)
    #[serde(rename = "GRAVE_ACCENT")]
    GraveAccent,
    /// 오른쪽 위 틱 (애큐트)
    #[serde(rename = "ACUTE_ACCENT")]
    AcuteAccent,
    /// 꺾인 모자
    #[serde(rename = "CIRCUMFLEX")]
    Circumflex,
    /// 수평선
    #[serde(rename = "MACRON")]
    Macron,
    /// 갈고리
    #[serde(rename = "HOOK_ABOVE")]
    HookAbove,
    /// 아래 점
    #[serde(rename = "DOT_BELOW")]
    DotBelow,
}

/// [AI 생성] 언어별 글꼴 참조
///
/// 원본: `fontRef` 요소의 익명 타입. 한글/라틴/한자 등 각 스크립트의 글꼴 ID를 지정합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "fontRef")]
pub struct LanguageFontReference {
    /// [AI 생성] 한글 글꼴 참조 (`hangul` 속성)
    #[serde(rename = "@hangul")]
    pub hangul: u32,

    /// [AI 생성] 영문 글꼴 참조 (`latin` 속성)
    #[serde(rename = "@latin")]
    pub latin: u32,

    /// [AI 생성] 한자 글꼴 참조 (`hanja` 속성)
    #[serde(rename = "@hanja")]
    pub hanja: u32,

    /// [AI 생성] 일본어 글꼴 참조 (`japanese` 속성)
    #[serde(rename = "@japanese")]
    pub japanese: u32,

    /// [AI 생성] 기타 글꼴 참조 (`other` 속성)
    #[serde(rename = "@other")]
    pub other: u32,

    /// [AI 생성] 기호 글꼴 참조 (`symbol` 속성)
    #[serde(rename = "@symbol")]
    pub symbol: u32,

    /// [AI 생성] 사용자 글꼴 참조 (`user` 속성)
    #[serde(rename = "@user")]
    pub user: u32,
}

/// [AI 생성] 언어별 장평
///
/// 원본: `ratio` 요소의 익명 타입. 스크립트별 가로 세로 비율을 지정합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ratio")]
pub struct LanguageRatio {
    /// [AI 생성] 한글 장평 (50-200%) (`hangul` 속성)
    #[serde(rename = "@hangul", default = "default_ratio")]
    pub hangul: u8,

    /// [AI 생성] 영문 장평 (50-200%) (`latin` 속성)
    #[serde(rename = "@latin", default = "default_ratio")]
    pub latin: u8,

    /// [AI 생성] 한자 장평 (50-200%) (`hanja` 속성)
    #[serde(rename = "@hanja", default = "default_ratio")]
    pub hanja: u8,

    /// [AI 생성] 일본어 장평 (50-200%) (`japanese` 속성)
    #[serde(rename = "@japanese", default = "default_ratio")]
    pub japanese: u8,

    /// [AI 생성] 기타 장평 (50-200%) (`other` 속성)
    #[serde(rename = "@other", default = "default_ratio")]
    pub other: u8,

    /// [AI 생성] 기호 장평 (50-200%) (`symbol` 속성)
    #[serde(rename = "@symbol", default = "default_ratio")]
    pub symbol: u8,

    /// [AI 생성] 사용자 장평 (50-200%) (`user` 속성)
    #[serde(rename = "@user", default = "default_ratio")]
    pub user: u8,
}

fn default_ratio() -> u8 {
    100
}

impl Default for LanguageRatio {
    fn default() -> Self {
        Self {
            hangul: 100,
            latin: 100,
            hanja: 100,
            japanese: 100,
            other: 100,
            symbol: 100,
            user: 100,
        }
    }
}

/// [AI 생성] 언어별 자간
///
/// 원본: `spacing` 요소의 익명 타입. 스크립트별 자간(%)을 지정합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "spacing")]
pub struct LanguageSpacing {
    /// [AI 생성] 한글 자간 (-50 ~ 50%) (`hangul` 속성)
    #[serde(rename = "@hangul", default)]
    pub hangul: i8,

    /// [AI 생성] 영문 자간 (-50 ~ 50%) (`latin` 속성)
    #[serde(rename = "@latin", default)]
    pub latin: i8,

    /// [AI 생성] 한자 자간 (-50 ~ 50%) (`hanja` 속성)
    #[serde(rename = "@hanja", default)]
    pub hanja: i8,

    /// [AI 생성] 일본어 자간 (-50 ~ 50%) (`japanese` 속성)
    #[serde(rename = "@japanese", default)]
    pub japanese: i8,

    /// [AI 생성] 기타 자간 (-50 ~ 50%) (`other` 속성)
    #[serde(rename = "@other", default)]
    pub other: i8,

    /// [AI 생성] 기호 자간 (-50 ~ 50%) (`symbol` 속성)
    #[serde(rename = "@symbol", default)]
    pub symbol: i8,

    /// [AI 생성] 사용자 자간 (-50 ~ 50%) (`user` 속성)
    #[serde(rename = "@user", default)]
    pub user: i8,
}

impl Default for LanguageSpacing {
    fn default() -> Self {
        Self {
            hangul: 0,
            latin: 0,
            hanja: 0,
            japanese: 0,
            other: 0,
            symbol: 0,
            user: 0,
        }
    }
}

/// [AI 생성] 언어별 상대 크기
///
/// 원본: `relSz` 요소의 익명 타입. 스크립트별 글자 크기 비율(%)을 지정합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "relSz")]
pub struct LanguageRelativeSize {
    /// [AI 생성] 한글 상대 크기 (10-250%) (`hangul` 속성)
    #[serde(rename = "@hangul", default = "default_ratio")]
    pub hangul: u8,

    /// [AI 생성] 영문 상대 크기 (10-250%) (`latin` 속성)
    #[serde(rename = "@latin", default = "default_ratio")]
    pub latin: u8,

    /// [AI 생성] 한자 상대 크기 (10-250%) (`hanja` 속성)
    #[serde(rename = "@hanja", default = "default_ratio")]
    pub hanja: u8,

    /// [AI 생성] 일본어 상대 크기 (10-250%) (`japanese` 속성)
    #[serde(rename = "@japanese", default = "default_ratio")]
    pub japanese: u8,

    /// [AI 생성] 기타 상대 크기 (10-250%) (`other` 속성)
    #[serde(rename = "@other", default = "default_ratio")]
    pub other: u8,

    /// [AI 생성] 기호 상대 크기 (10-250%) (`symbol` 속성)
    #[serde(rename = "@symbol", default = "default_ratio")]
    pub symbol: u8,

    /// [AI 생성] 사용자 상대 크기 (10-250%) (`user` 속성)
    #[serde(rename = "@user", default = "default_ratio")]
    pub user: u8,
}

impl Default for LanguageRelativeSize {
    fn default() -> Self {
        Self {
            hangul: 100,
            latin: 100,
            hanja: 100,
            japanese: 100,
            other: 100,
            symbol: 100,
            user: 100,
        }
    }
}

/// [AI 생성] 언어별 오프셋
///
/// 원본: `offset` 요소의 익명 타입. 스크립트별 수직 오프셋(%)을 지정합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "offset")]
pub struct LanguageOffset {
    /// [AI 생성] 한글 오프셋 (-100 ~ 100%) (`hangul` 속성)
    #[serde(rename = "@hangul", default)]
    pub hangul: i8,

    /// [AI 생성] 영문 오프셋 (-100 ~ 100%) (`latin` 속성)
    #[serde(rename = "@latin", default)]
    pub latin: i8,

    /// [AI 생성] 한자 오프셋 (-100 ~ 100%) (`hanja` 속성)
    #[serde(rename = "@hanja", default)]
    pub hanja: i8,

    /// [AI 생성] 일본어 오프셋 (-100 ~ 50%) (`japanese` 속성)
    #[serde(rename = "@japanese", default)]
    pub japanese: i8,

    /// [AI 생성] 기타 오프셋 (-100 ~ 50%) (`other` 속성)
    #[serde(rename = "@other", default)]
    pub other: i8,

    /// [AI 생성] 기호 오프셋 (-100 ~ 50%) (`symbol` 속성)
    #[serde(rename = "@symbol", default)]
    pub symbol: i8,

    /// [AI 생성] 사용자 오프셋 (-50 ~ 50%) (`user` 속성)
    #[serde(rename = "@user", default)]
    pub user: i8,
}

impl Default for LanguageOffset {
    fn default() -> Self {
        Self {
            hangul: 0,
            latin: 0,
            hanja: 0,
            japanese: 0,
            other: 0,
            symbol: 0,
            user: 0,
        }
    }
}

/// [AI 생성] 밑줄 정보
///
/// 원본: `underline` 요소의 익명 타입.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "underline")]
pub struct CharacterUnderline {
    /// [AI 생성] 밑줄 위치 (`type` 속성)
    #[serde(rename = "@type")]
    pub position: UnderlinePosition,

    /// [AI 생성] 밑줄 모양 (`shape` 속성)
    #[serde(rename = "@shape")]
    pub shape: LineStyleType2,

    /// [AI 생성] 밑줄 색상 (`color` 속성)
    #[serde(rename = "@color")]
    pub color: RgbColor,
}

/// [AI 생성] 취소선 정보
///
/// 원본: `strikeout` 요소의 익명 타입.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "strikeout")]
pub struct CharacterStrikeout {
    /// [AI 생성] 취소선 모양 (`shape` 속성)
    #[serde(rename = "@shape")]
    pub shape: LineStyleType2,

    /// [AI 생성] 취소선 색상 (`color` 속성)
    #[serde(rename = "@color")]
    pub color: RgbColor,
}

/// [AI 생성] 외곽선 정보
///
/// 원본: `outline` 요소의 익명 타입.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "outline")]
pub struct CharacterOutline {
    /// [AI 생성] 외곽선 종류 (`type` 속성)
    #[serde(rename = "@type")]
    pub outline_type: LineStyleType1,
}

/// [AI 생성] 그림자 정보
///
/// 원본: `shadow` 요소의 익명 타입.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "shadow")]
pub struct CharacterShadow {
    /// [AI 생성] 그림자 종류 (`type` 속성)
    #[serde(rename = "@type")]
    pub shadow_type: ShadowType,

    /// [AI 생성] 그림자 색상 (`color` 속성)
    #[serde(rename = "@color")]
    pub color: RgbColor,

    /// [AI 생성] 그림자 간격 X (hwpunit) (`offsetX` 속성)
    #[serde(rename = "@offsetX")]
    pub offset_x: i8,

    /// [AI 생성] 그림자 간격 Y (hwpunit) (`offsetY` 속성)
    #[serde(rename = "@offsetY")]
    pub offset_y: i8,
}

/// [AI 생성] 글자 모양
///
/// 원본: `CharShapeType`. 스크립트별 글꼴/크기/자간과 강조(볼드/이탤릭/밑줄/취소선/외곽선/그림자/양각·음각·첨자)를 한 세트로 정의합니다.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "charShape")]
pub struct CharacterShape {
    /// [AI 생성] 언어별 글꼴 참조 (`fontRef` 요소)
    #[serde(rename = "fontRef")]
    pub font_reference: LanguageFontReference,

    /// [AI 생성] 언어별 장평 (`ratio` 요소)
    #[serde(rename = "ratio")]
    pub ratio: LanguageRatio,

    /// [AI 생성] 언어별 자간 (`spacing` 요소)
    #[serde(rename = "spacing")]
    pub spacing: LanguageSpacing,

    /// [AI 생성] 언어별 상대 크기 (`relSz` 요소)
    #[serde(rename = "relSz")]
    pub relative_size: LanguageRelativeSize,

    /// [AI 생성] 언어별 오프셋 (`offset` 요소)
    #[serde(rename = "offset")]
    pub offset: LanguageOffset,

    /// [AI 생성] 기울임 (`italic` 요소). 이탤릭 토글.
    #[serde(rename = "italic", skip_serializing_if = "Option::is_none")]
    pub italic: Option<()>,

    /// [AI 생성] 진하게 (`bold` 요소). 볼드 토글.
    #[serde(rename = "bold", skip_serializing_if = "Option::is_none")]
    pub bold: Option<()>,

    /// [AI 생성] 밑줄 (`underline` 요소)
    #[serde(rename = "underline", skip_serializing_if = "Option::is_none")]
    pub underline: Option<CharacterUnderline>,

    /// [AI 생성] 취소선 (`strikeout` 요소)
    #[serde(rename = "strikeout", skip_serializing_if = "Option::is_none")]
    pub strikeout: Option<CharacterStrikeout>,

    /// [AI 생성] 외곽선 (`outline` 요소)
    #[serde(rename = "outline", skip_serializing_if = "Option::is_none")]
    pub outline: Option<CharacterOutline>,

    /// [AI 생성] 그림자 (`shadow` 요소)
    #[serde(rename = "shadow", skip_serializing_if = "Option::is_none")]
    pub shadow: Option<CharacterShadow>,

    /// [AI 생성] 양각 (`emboss` 요소)
    #[serde(rename = "emboss", skip_serializing_if = "Option::is_none")]
    pub emboss: Option<()>,

    /// [AI 생성] 음각 (`engrave` 요소)
    #[serde(rename = "engrave", skip_serializing_if = "Option::is_none")]
    pub engrave: Option<()>,

    /// [AI 생성] 위 첨자 (`supscript` 요소)
    #[serde(rename = "supscript", skip_serializing_if = "Option::is_none")]
    pub superscript: Option<()>,

    /// [AI 생성] 아래 첨자 (`subscript` 요소)
    #[serde(rename = "subscript", skip_serializing_if = "Option::is_none")]
    pub subscript: Option<()>,

    /// [AI 생성] 글자 모양 아이디 (`id` 속성). 참조용 키.
    #[serde(rename = "@id")]
    pub id: u32,

    /// [AI 생성] 글자 크기 hwpunit (10pt=1000) (`height` 속성)
    #[serde(rename = "@height", default = "default_character_height")]
    pub height: i32,

    /// [AI 생성] 글자색 (`textColor` 속성)
    #[serde(rename = "@textColor", default = "RgbColor::black")]
    pub text_color: RgbColor,

    /// [AI 생성] 음영 색 (`shadeColor` 속성)
    #[serde(rename = "@shadeColor", default = "RgbColor::white")]
    pub shade_color: RgbColor,

    /// [AI 생성] 글꼴에 어울리는 빈칸 사용 여부 (`useFontSpace` 속성). 폰트 정의된 공간 폭 사용.
    #[serde(rename = "@useFontSpace", default)]
    pub use_font_space: bool,

    /// [AI 생성] 커닝 사용 여부 (`useKerning` 속성). 글꼴 커닝 적용.
    #[serde(rename = "@useKerning", default)]
    pub use_kerning: bool,

    /// [AI 생성] 강조점 종류 (`symMark` 속성)
    #[serde(rename = "@symMark", default)]
    pub emphasis_mark: EmphasisMarkType,

    /// [AI 생성] 글자 테두리 참조 (`borderFillIDRef` 속성)
    #[serde(rename = "@borderFillIDRef", skip_serializing_if = "Option::is_none")]
    pub border_fill_id_reference: Option<BorderFillIdRef>,
}

fn default_character_height() -> i32 {
    1000
}

/// [AI 생성] 글자 모양 목록
///
/// 원본: `charShapes` 요소의 익명 타입
/// 원본: `charShapes` 요소의 익명 타입. 전역 글자 스타일 풀입니다.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "charShapes")]
pub struct CharacterShapeList {
    /// [AI 생성] 글자 모양 목록 (`charShape` 요소)
    #[serde(rename = "charShape")]
    pub character_shapes: Vec<CharacterShape>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성). 목록 길이 검증용.
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}
