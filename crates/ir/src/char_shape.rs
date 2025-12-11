//! 글자 모양
//!
//! 텍스트의 글꼴, 크기, 색상 등 글자 서식을 정의합니다.

use primitive::Color;
use primitive::FontId;
use primitive::{
    EmphasisType, HwpUnit, LanguageType, OutlineType, Percent, ShadowType, StrikethroughType,
    UnderlinePosition, UnderlineType,
};

/// 글자 모양 정의
#[derive(Debug, Clone)]
pub struct CharShape {
    /// 언어별 폰트 설정
    pub fonts: FontSet,

    /// 글자 크기 (HwpUnit, 기본값 1000 = 10pt)
    pub font_size: HwpUnit,

    /// 장평 (기본값 100%)
    pub char_scale: Percent,

    /// 자간 (기본값 0%)
    pub char_spacing: Percent,

    /// 글자 색상
    pub color: Color,

    /// 밑줄
    pub underline: UnderlineStyle,

    /// 취소선
    pub strikethrough: StrikethroughType,

    /// 강조점
    pub emphasis: EmphasisStyle,

    /// 외곽선
    pub outline: OutlineType,

    /// 그림자
    pub shadow: ShadowStyle,

    /// 굵게
    pub bold: bool,

    /// 기울임
    pub italic: bool,

    /// 위 첨자
    pub superscript: bool,

    /// 아래 첨자
    pub subscript: bool,

    /// 글자 배경색
    pub background_color: Option<Color>,

    /// 형광펜 색상
    pub highlight_color: Option<Color>,

    /// 양각 효과
    pub emboss: bool,

    /// 음각 효과
    pub engrave: bool,

    /// 커닝 사용 여부
    pub use_kerning: bool,

    /// 음영 색상 (배경색과 별도)
    pub shade_color: Option<Color>,

    /// 테두리/배경 참조 ID
    pub border_fill_id_ref: Option<primitive::BorderFillId>,
}

impl Default for CharShape {
    fn default() -> Self {
        Self {
            fonts: FontSet::default(),
            font_size: HwpUnit::new(1000), // 10pt
            char_scale: Percent::HUNDRED,
            char_spacing: Percent::ZERO,
            color: Color::BLACK,
            underline: UnderlineStyle::default(),
            strikethrough: StrikethroughType::None,
            emphasis: EmphasisStyle::default(),
            outline: OutlineType::None,
            shadow: ShadowStyle::default(),
            bold: false,
            italic: false,
            superscript: false,
            subscript: false,
            background_color: None,
            highlight_color: None,
            emboss: false,
            engrave: false,
            use_kerning: false,
            shade_color: None,
            border_fill_id_ref: None,
        }
    }
}

impl CharShape {
    /// 기본 글자 모양 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 글자 크기 설정 (포인트)
    pub fn with_font_size_pt(mut self, pt: f64) -> Self {
        self.font_size = HwpUnit::from_pt(pt);
        self
    }

    /// 굵게 설정
    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// 기울임 설정
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    /// 색상 설정
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

/// 언어별 폰트 설정
#[derive(Debug, Clone, Default)]
pub struct FontSet {
    /// 한글 폰트
    pub korean: Option<FontRef>,
    /// 영문 폰트
    pub english: Option<FontRef>,
    /// 한자 폰트
    pub hanja: Option<FontRef>,
    /// 일본어 폰트
    pub japanese: Option<FontRef>,
    /// 기타 폰트
    pub other: Option<FontRef>,
    /// 기호 폰트
    pub symbol: Option<FontRef>,
    /// 사용자 정의 폰트
    pub user: Option<FontRef>,
}

impl FontSet {
    /// 모든 언어에 동일한 폰트 설정
    pub fn all(font_ref: FontRef) -> Self {
        Self {
            korean: Some(font_ref.clone()),
            english: Some(font_ref.clone()),
            hanja: Some(font_ref.clone()),
            japanese: Some(font_ref.clone()),
            other: Some(font_ref.clone()),
            symbol: Some(font_ref.clone()),
            user: Some(font_ref),
        }
    }

    /// 특정 언어의 폰트 참조 가져오기
    pub fn get(&self, language: LanguageType) -> Option<&FontRef> {
        match language {
            LanguageType::Korean => self.korean.as_ref(),
            LanguageType::English => self.english.as_ref(),
            LanguageType::Hanja => self.hanja.as_ref(),
            LanguageType::Japanese => self.japanese.as_ref(),
            LanguageType::Other => self.other.as_ref(),
            LanguageType::Symbol => self.symbol.as_ref(),
            LanguageType::User => self.user.as_ref(),
        }
    }

    /// 특정 언어의 폰트 설정
    pub fn set(&mut self, language: LanguageType, font_ref: FontRef) {
        match language {
            LanguageType::Korean => self.korean = Some(font_ref),
            LanguageType::English => self.english = Some(font_ref),
            LanguageType::Hanja => self.hanja = Some(font_ref),
            LanguageType::Japanese => self.japanese = Some(font_ref),
            LanguageType::Other => self.other = Some(font_ref),
            LanguageType::Symbol => self.symbol = Some(font_ref),
            LanguageType::User => self.user = Some(font_ref),
        }
    }
}

/// 폰트 참조
#[derive(Debug, Clone)]
pub struct FontRef {
    /// 폰트 ID (스타일 저장소 내 인덱스)
    pub id: FontId,
    /// 폰트 너비 비율 (기본값 100%)
    pub width_ratio: Percent,
    /// 폰트 간격 (기본값 0%)
    pub spacing: Percent,
    /// 폰트 오프셋 (기본값 0%)
    pub offset: Percent,
    /// 상대 크기 (기본값 100%)
    pub relative_size: Percent,
}

impl FontRef {
    /// 폰트 참조 생성
    pub fn new(id: FontId) -> Self {
        Self {
            id,
            width_ratio: Percent::HUNDRED,
            spacing: Percent::ZERO,
            offset: Percent::ZERO,
            relative_size: Percent::HUNDRED,
        }
    }
}

/// 밑줄 스타일
#[derive(Debug, Clone, Default)]
pub struct UnderlineStyle {
    /// 밑줄 종류
    pub line_type: UnderlineType,
    /// 밑줄 위치
    pub position: UnderlinePosition,
    /// 밑줄 색상 (None이면 글자 색상 사용)
    pub color: Option<Color>,
}

impl UnderlineStyle {
    /// 밑줄 없음
    pub fn none() -> Self {
        Self::default()
    }

    /// 단일 밑줄
    pub fn single() -> Self {
        Self {
            line_type: UnderlineType::Single,
            position: UnderlinePosition::Bottom,
            color: None,
        }
    }
}

/// 강조점 스타일
#[derive(Debug, Clone, Default)]
pub struct EmphasisStyle {
    /// 강조점 종류
    pub emphasis_type: EmphasisType,
    /// 강조점 색상 (None이면 글자 색상 사용)
    pub color: Option<Color>,
}

/// 그림자 스타일
#[derive(Debug, Clone, Default)]
pub struct ShadowStyle {
    /// 그림자 종류
    pub shadow_type: ShadowType,
    /// 그림자 색상
    pub color: Option<Color>,
    /// 가로 오프셋
    pub offset_x: HwpUnit,
    /// 세로 오프셋
    pub offset_y: HwpUnit,
}

/// 폰트 정의
#[derive(Debug, Clone)]
pub struct Font {
    /// 폰트 이름
    pub name: String,
    /// 대체 폰트 이름 (간단한 형태, HWP용)
    pub alternate_name: Option<String>,
    /// 폰트 종류
    pub font_type: FontType,
    /// 폰트 패밀리
    pub family: FontFamily,
    /// PANOSE 글꼴 분류 정보
    pub panose: Option<primitive::Panose>,
    /// 대체 폰트 상세 정보 (HWPX용)
    pub substitute_font: Option<SubstituteFont>,
    /// 기본 폰트 이름 (HWP용)
    pub default_font_name: Option<String>,
    /// 임베디드 폰트 여부
    pub is_embedded: bool,
    /// 바이너리 항목 참조 (임베디드 폰트 데이터)
    pub binary_item_id_ref: Option<primitive::BinaryDataId>,
}

impl Font {
    /// 폰트 생성
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            alternate_name: None,
            font_type: FontType::default(),
            family: FontFamily::default(),
            panose: None,
            substitute_font: None,
            default_font_name: None,
            is_embedded: false,
            binary_item_id_ref: None,
        }
    }
}

/// 대체 폰트 정보 (HWPX용)
#[derive(Debug, Clone)]
pub struct SubstituteFont {
    /// 대체 폰트 이름
    pub face: String,
    /// 대체 폰트 종류
    pub font_type: FontType,
    /// 임베디드 여부
    pub is_embedded: bool,
    /// 바이너리 항목 참조
    pub binary_item_id_ref: Option<primitive::BinaryDataId>,
}

/// 폰트 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontType {
    /// 대표 글꼴
    #[default]
    Representative,
    /// 트루타입
    TrueType,
    /// 한글 전용
    HangeulOnly,
}

/// 폰트 패밀리
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontFamily {
    /// 알 수 없음
    #[default]
    Unknown,
    /// 명조
    Serif,
    /// 고딕
    SansSerif,
    /// 장식체
    Decorative,
    /// 필기체
    Script,
    /// 고정폭
    Monospace,
}
