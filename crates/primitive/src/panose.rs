//! PANOSE 1.0 글꼴 분류 시스템
//!
//! PANOSE (PANose-1)는 글꼴의 시각적 특성을 10개 숫자로 분류하는 체계입니다.
//! 이 모듈은 PANOSE 분류 체계의 각 카테고리를 Rust enum으로 정의합니다.

/// PANOSE 글꼴 분류 정보
///
/// 10개의 분류 카테고리로 글꼴의 시각적 특성을 표현합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Panose {
    /// 글꼴 계열
    pub family_type: FamilyType,
    /// 세리프 스타일
    pub serif_style: SerifStyle,
    /// 굵기
    pub weight: Weight,
    /// 비례
    pub proportion: Proportion,
    /// 대조
    pub contrast: Contrast,
    /// 스트로크 편차
    pub stroke_variation: StrokeVariation,
    /// 자획 스타일
    pub arm_style: ArmStyle,
    /// 글자형
    pub letterform: Letterform,
    /// 중간선
    pub midline: Midline,
    /// X-높이
    pub x_height: XHeight,
}

impl Panose {
    /// 10바이트 배열에서 PANOSE 정보 생성
    pub fn from_bytes(bytes: [u8; 10]) -> Self {
        Self {
            family_type: FamilyType::from(bytes[0]),
            serif_style: SerifStyle::from(bytes[1]),
            weight: Weight::from(bytes[2]),
            proportion: Proportion::from(bytes[3]),
            contrast: Contrast::from(bytes[4]),
            stroke_variation: StrokeVariation::from(bytes[5]),
            arm_style: ArmStyle::from(bytes[6]),
            letterform: Letterform::from(bytes[7]),
            midline: Midline::from(bytes[8]),
            x_height: XHeight::from(bytes[9]),
        }
    }

    /// PANOSE 정보를 10바이트 배열로 변환
    pub fn to_bytes(self) -> [u8; 10] {
        [
            self.family_type as u8,
            self.serif_style as u8,
            self.weight as u8,
            self.proportion as u8,
            self.contrast as u8,
            self.stroke_variation as u8,
            self.arm_style as u8,
            self.letterform as u8,
            self.midline as u8,
            self.x_height as u8,
        ]
    }
}

/// 글꼴 계열 (PANOSE Family Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum FamilyType {
    /// 알 수 없음
    #[default]
    Unknown = 0,
    /// 명조 (Serif)
    Myungjo = 1,
    /// 고딕 (Sans-Serif)
    Gothic = 2,
    /// 산세리프 (Monospace)
    SansSerif = 3,
    /// 필기체 (Brush Script)
    BrushScript = 4,
    /// 장식체 (Decorative)
    Decorative = 5,
    /// 비정형 명조
    NonRectMyungjo = 6,
    /// 비정형 고딕
    NonRectGothic = 7,
}

impl From<u8> for FamilyType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Myungjo,
            2 => Self::Gothic,
            3 => Self::SansSerif,
            4 => Self::BrushScript,
            5 => Self::Decorative,
            6 => Self::NonRectMyungjo,
            7 => Self::NonRectGothic,
            _ => Self::Unknown,
        }
    }
}

/// 세리프 스타일 (PANOSE Serif Style)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum SerifStyle {
    /// 임의/없음
    #[default]
    Any = 0,
    /// 해당 없음
    NoFit = 1,
    /// Cove
    Cove = 2,
    /// Obtuse Cove
    ObtuseCove = 3,
    /// Square Cove
    SquareCove = 4,
    /// Obtuse Square Cove
    ObtuseSquareCove = 5,
    /// Square
    Square = 6,
    /// Thin
    Thin = 7,
    /// Oval
    Oval = 8,
    /// Exaggerated
    Exaggerated = 9,
    /// Triangle
    Triangle = 10,
    /// Normal Sans
    NormalSans = 11,
    /// Obtuse Sans
    ObtuseSans = 12,
    /// Perpendicular Sans
    PerpendicularSans = 13,
    /// Flared
    Flared = 14,
    /// Rounded
    Rounded = 15,
}

impl From<u8> for SerifStyle {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Any,
            1 => Self::NoFit,
            2 => Self::Cove,
            3 => Self::ObtuseCove,
            4 => Self::SquareCove,
            5 => Self::ObtuseSquareCove,
            6 => Self::Square,
            7 => Self::Thin,
            8 => Self::Oval,
            9 => Self::Exaggerated,
            10 => Self::Triangle,
            11 => Self::NormalSans,
            12 => Self::ObtuseSans,
            13 => Self::PerpendicularSans,
            14 => Self::Flared,
            15 => Self::Rounded,
            _ => Self::Any,
        }
    }
}

/// 굵기 (PANOSE Weight)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Weight {
    /// 임의
    #[default]
    Any = 0,
    /// 해당 없음
    NoFit = 1,
    /// 매우 가늘게
    VeryLight = 2,
    /// 가늘게
    Light = 3,
    /// 얇게
    Thin = 4,
    /// 책
    Book = 5,
    /// 중간
    Medium = 6,
    /// 약간 굵게
    DemiBold = 7,
    /// 굵게
    Bold = 8,
    /// 아주 굵게
    Heavy = 9,
    /// 검정
    Black = 10,
    /// 북쪽 검정
    Nord = 11,
}

impl From<u8> for Weight {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Any,
            1 => Self::NoFit,
            2 => Self::VeryLight,
            3 => Self::Light,
            4 => Self::Thin,
            5 => Self::Book,
            6 => Self::Medium,
            7 => Self::DemiBold,
            8 => Self::Bold,
            9 => Self::Heavy,
            10 => Self::Black,
            11 => Self::Nord,
            _ => Self::Any,
        }
    }
}

/// 비례 (PANOSE Proportion)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Proportion {
    /// 임의
    #[default]
    Any = 0,
    /// 해당 없음
    NoFit = 1,
    /// Old Style
    OldStyle = 2,
    /// Modern
    Modern = 3,
    /// Even Width
    EvenWidth = 4,
    /// Extended
    Extended = 5,
    /// Condensed
    Condensed = 6,
    /// Very Extended
    VeryExtended = 7,
    /// Very Condensed
    VeryCondensed = 8,
    /// Monospaced
    Monospaced = 9,
}

impl From<u8> for Proportion {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Any,
            1 => Self::NoFit,
            2 => Self::OldStyle,
            3 => Self::Modern,
            4 => Self::EvenWidth,
            5 => Self::Extended,
            6 => Self::Condensed,
            7 => Self::VeryExtended,
            8 => Self::VeryCondensed,
            9 => Self::Monospaced,
            _ => Self::Any,
        }
    }
}

/// 대조 (PANOSE Contrast)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Contrast {
    /// 임의
    #[default]
    Any = 0,
    /// 해당 없음
    NoFit = 1,
    /// 없음
    None = 2,
    /// 매우 낮음
    VeryLow = 3,
    /// 낮음
    Low = 4,
    /// 약간 낮음
    MediumLow = 5,
    /// 중간
    Medium = 6,
    /// 약간 높음
    MediumHigh = 7,
    /// 높음
    High = 8,
    /// 매우 높음
    VeryHigh = 9,
}

impl From<u8> for Contrast {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Any,
            1 => Self::NoFit,
            2 => Self::None,
            3 => Self::VeryLow,
            4 => Self::Low,
            5 => Self::MediumLow,
            6 => Self::Medium,
            7 => Self::MediumHigh,
            8 => Self::High,
            9 => Self::VeryHigh,
            _ => Self::Any,
        }
    }
}

/// 스트로크 편차 (PANOSE Stroke Variation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum StrokeVariation {
    /// 임의
    #[default]
    Any = 0,
    /// 해당 없음
    NoFit = 1,
    /// No Variation
    NoVariation = 2,
    /// Gradual/Diagonal
    GradualDiagonal = 3,
    /// Gradual/Transitional
    GradualTransitional = 4,
    /// Gradual/Vertical
    GradualVertical = 5,
    /// Gradual/Horizontal
    GradualHorizontal = 6,
    /// Rapid/Vertical
    RapidVertical = 7,
    /// Rapid/Horizontal
    RapidHorizontal = 8,
    /// Instant/Vertical
    InstantVertical = 9,
    /// Instant/Horizontal
    InstantHorizontal = 10,
}

impl From<u8> for StrokeVariation {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Any,
            1 => Self::NoFit,
            2 => Self::NoVariation,
            3 => Self::GradualDiagonal,
            4 => Self::GradualTransitional,
            5 => Self::GradualVertical,
            6 => Self::GradualHorizontal,
            7 => Self::RapidVertical,
            8 => Self::RapidHorizontal,
            9 => Self::InstantVertical,
            10 => Self::InstantHorizontal,
            _ => Self::Any,
        }
    }
}

/// 자획 스타일 (PANOSE Arm Style)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ArmStyle {
    /// 임의
    #[default]
    Any = 0,
    /// 해당 없음
    NoFit = 1,
    /// Straight Arms/Horizontal
    StraightArmsHorizontal = 2,
    /// Straight Arms/Wedge
    StraightArmsWedge = 3,
    /// Straight Arms/Vertical
    StraightArmsVertical = 4,
    /// Straight Arms/Single Serif
    StraightArmsSingleSerif = 5,
    /// Straight Arms/Double Serif
    StraightArmsDoubleSerif = 6,
    /// Non-Straight/Horizontal
    NonStraightHorizontal = 7,
    /// Non-Straight/Wedge
    NonStraightWedge = 8,
    /// Non-Straight/Vertical
    NonStraightVertical = 9,
    /// Non-Straight/Single Serif
    NonStraightSingleSerif = 10,
    /// Non-Straight/Double Serif
    NonStraightDoubleSerif = 11,
}

impl From<u8> for ArmStyle {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Any,
            1 => Self::NoFit,
            2 => Self::StraightArmsHorizontal,
            3 => Self::StraightArmsWedge,
            4 => Self::StraightArmsVertical,
            5 => Self::StraightArmsSingleSerif,
            6 => Self::StraightArmsDoubleSerif,
            7 => Self::NonStraightHorizontal,
            8 => Self::NonStraightWedge,
            9 => Self::NonStraightVertical,
            10 => Self::NonStraightSingleSerif,
            11 => Self::NonStraightDoubleSerif,
            _ => Self::Any,
        }
    }
}

/// 글자형 (PANOSE Letterform)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Letterform {
    /// 임의
    #[default]
    Any = 0,
    /// 해당 없음
    NoFit = 1,
    /// Normal/Contact
    NormalContact = 2,
    /// Normal/Weighted
    NormalWeighted = 3,
    /// Normal/Boxed
    NormalBoxed = 4,
    /// Normal/Flattened
    NormalFlattened = 5,
    /// Normal/Rounded
    NormalRounded = 6,
    /// Normal/Off Center
    NormalOffCenter = 7,
    /// Normal/Square
    NormalSquare = 8,
    /// Oblique/Contact
    ObliqueContact = 9,
    /// Oblique/Weighted
    ObliqueWeighted = 10,
    /// Oblique/Boxed
    ObliqueBoxed = 11,
    /// Oblique/Flattened
    ObliqueFlattened = 12,
    /// Oblique/Rounded
    ObliqueRounded = 13,
    /// Oblique/Off Center
    ObliqueOffCenter = 14,
    /// Oblique/Square
    ObliqueSquare = 15,
}

impl From<u8> for Letterform {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Any,
            1 => Self::NoFit,
            2 => Self::NormalContact,
            3 => Self::NormalWeighted,
            4 => Self::NormalBoxed,
            5 => Self::NormalFlattened,
            6 => Self::NormalRounded,
            7 => Self::NormalOffCenter,
            8 => Self::NormalSquare,
            9 => Self::ObliqueContact,
            10 => Self::ObliqueWeighted,
            11 => Self::ObliqueBoxed,
            12 => Self::ObliqueFlattened,
            13 => Self::ObliqueRounded,
            14 => Self::ObliqueOffCenter,
            15 => Self::ObliqueSquare,
            _ => Self::Any,
        }
    }
}

/// 중간선 (PANOSE Midline)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Midline {
    /// 임의
    #[default]
    Any = 0,
    /// 해당 없음
    NoFit = 1,
    /// Standard/Trimmed
    StandardTrimmed = 2,
    /// Standard/Pointed
    StandardPointed = 3,
    /// Standard/Serifed
    StandardSerifed = 4,
    /// High/Trimmed
    HighTrimmed = 5,
    /// High/Pointed
    HighPointed = 6,
    /// High/Serifed
    HighSerifed = 7,
    /// Constant/Trimmed
    ConstantTrimmed = 8,
    /// Constant/Pointed
    ConstantPointed = 9,
    /// Constant/Serifed
    ConstantSerifed = 10,
    /// Low/Trimmed
    LowTrimmed = 11,
    /// Low/Pointed
    LowPointed = 12,
    /// Low/Serifed
    LowSerifed = 13,
}

impl From<u8> for Midline {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Any,
            1 => Self::NoFit,
            2 => Self::StandardTrimmed,
            3 => Self::StandardPointed,
            4 => Self::StandardSerifed,
            5 => Self::HighTrimmed,
            6 => Self::HighPointed,
            7 => Self::HighSerifed,
            8 => Self::ConstantTrimmed,
            9 => Self::ConstantPointed,
            10 => Self::ConstantSerifed,
            11 => Self::LowTrimmed,
            12 => Self::LowPointed,
            13 => Self::LowSerifed,
            _ => Self::Any,
        }
    }
}

/// X-높이 (PANOSE X-Height)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum XHeight {
    /// 임의
    #[default]
    Any = 0,
    /// 해당 없음
    NoFit = 1,
    /// Constant/Small
    ConstantSmall = 2,
    /// Constant/Standard
    ConstantStandard = 3,
    /// Constant/Large
    ConstantLarge = 4,
    /// Ducking/Small
    DuckingSmall = 5,
    /// Ducking/Standard
    DuckingStandard = 6,
    /// Ducking/Large
    DuckingLarge = 7,
}

impl From<u8> for XHeight {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Any,
            1 => Self::NoFit,
            2 => Self::ConstantSmall,
            3 => Self::ConstantStandard,
            4 => Self::ConstantLarge,
            5 => Self::DuckingSmall,
            6 => Self::DuckingStandard,
            7 => Self::DuckingLarge,
            _ => Self::Any,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panose_from_bytes() {
        let bytes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 1];
        let panose = Panose::from_bytes(bytes);

        assert_eq!(panose.family_type, FamilyType::Myungjo);
        assert_eq!(panose.serif_style, SerifStyle::Cove);
        assert_eq!(panose.weight, Weight::Light);
        assert_eq!(panose.proportion, Proportion::EvenWidth);
        assert_eq!(panose.contrast, Contrast::MediumLow);
        assert_eq!(panose.stroke_variation, StrokeVariation::GradualHorizontal);
        assert_eq!(panose.arm_style, ArmStyle::NonStraightHorizontal);
        assert_eq!(panose.letterform, Letterform::NormalSquare);
        assert_eq!(panose.midline, Midline::ConstantPointed);
        assert_eq!(panose.x_height, XHeight::NoFit);
    }

    #[test]
    fn test_panose_to_bytes() {
        let panose = Panose {
            family_type: FamilyType::Gothic,
            serif_style: SerifStyle::NormalSans,
            weight: Weight::Bold,
            proportion: Proportion::Monospaced,
            contrast: Contrast::None,
            stroke_variation: StrokeVariation::NoVariation,
            arm_style: ArmStyle::StraightArmsHorizontal,
            letterform: Letterform::NormalContact,
            midline: Midline::StandardTrimmed,
            x_height: XHeight::ConstantStandard,
        };

        let bytes = panose.to_bytes();
        assert_eq!(bytes, [2, 11, 8, 9, 2, 2, 2, 2, 2, 3]);
    }

    #[test]
    fn test_panose_roundtrip() {
        let original = [0, 5, 6, 7, 8, 9, 10, 11, 12, 7];
        let panose = Panose::from_bytes(original);
        let result = panose.to_bytes();
        assert_eq!(original, result);
    }

    #[test]
    fn test_default_panose() {
        let panose = Panose::default();
        assert_eq!(panose.to_bytes(), [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
