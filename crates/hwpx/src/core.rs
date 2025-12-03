use crate::{
    Arbitrary, Core, Enumeration, Hancom,
    any_element::{AnyElement, ElementName},
    error::Error,
};

/// "AlignStyleType"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignStyle {
    /// "TOP_LEFT"
    TopLeft,
    /// "TOP"
    Top,
    /// "TOP_RIGHT"
    TopRight,
    /// "LEFT"
    Left,
    /// "CENTER"
    Center,
    /// "RIGHT"
    Right,
    /// "BOTTOM_LEFT"
    BottomLeft,
    /// "BOTTOM"
    Bottom,
    /// "BOTTOM_RIGHT"
    BottomRight,
}

impl Hancom for AlignStyle {
    //
}

impl Core for AlignStyle {
    const NAME: &'static str = "AlignStyleType";
}

impl Enumeration for AlignStyle {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::TopLeft => "TOP_LEFT",
            Self::Top => "TOP",
            Self::TopRight => "TOP_RIGHT",
            Self::Left => "LEFT",
            Self::Center => "CENTER",
            Self::Right => "RIGHT",
            Self::BottomLeft => "BOTTOM_LEFT",
            Self::Bottom => "BOTTOM",
            Self::BottomRight => "BOTTOM_RIGHT",
        }
    }
}

impl std::str::FromStr for AlignStyle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TOP_LEFT" => Ok(Self::TopLeft),
            "TOP" => Ok(Self::Top),
            "TOP_RIGHT" => Ok(Self::TopRight),
            "LEFT" => Ok(Self::Left),
            "CENTER" => Ok(Self::Center),
            "RIGHT" => Ok(Self::Right),
            "BOTTOM_LEFT" => Ok(Self::BottomLeft),
            "BOTTOM" => Ok(Self::Bottom),
            "BOTTOM_RIGHT" => Ok(Self::BottomRight),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// "ArrowSize"
#[derive(Debug)]
pub enum ArrowSize {
    /// "SMALL_SMALL"
    SmallSmall,
    /// "SMALL_MEDIUM"
    SmallMedium,
    /// "SMALL_LARGE"
    SmallLarge,
    /// "MEDIUM_SMALL"
    MediumSmall,
    /// "MEDIUM_MEDIUM"
    MediumMedium,
    /// "MEDIUM_LARGE"
    MediumLarge,
    /// "LARGE_SMALL"
    LargeSmall,
    /// "LARGE_MEDIUM"
    LargeMedium,
    /// "LARGE_LARGE"
    LargeLarge,
}

impl Hancom for ArrowSize {
    //
}

impl Core for ArrowSize {
    const NAME: &'static str = "ArrowSize";
}

impl Enumeration for ArrowSize {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::SmallSmall => "SMALL_SMALL",
            Self::SmallMedium => "SMALL_MEDIUM",
            Self::SmallLarge => "SMALL_LARGE",
            Self::MediumSmall => "MEDIUM_SMALL",
            Self::MediumMedium => "MEDIUM_MEDIUM",
            Self::MediumLarge => "MEDIUM_LARGE",
            Self::LargeSmall => "LARGE_SMALL",
            Self::LargeMedium => "LARGE_MEDIUM",
            Self::LargeLarge => "LARGE_LARGE",
        }
    }
}

impl std::str::FromStr for ArrowSize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SMALL_SMALL" => Ok(Self::SmallSmall),
            "SMALL_MEDIUM" => Ok(Self::SmallMedium),
            "SMALL_LARGE" => Ok(Self::SmallLarge),
            "MEDIUM_SMALL" => Ok(Self::MediumSmall),
            "MEDIUM_MEDIUM" => Ok(Self::MediumMedium),
            "MEDIUM_LARGE" => Ok(Self::MediumLarge),
            "LARGE_SMALL" => Ok(Self::LargeSmall),
            "LARGE_MEDIUM" => Ok(Self::LargeMedium),
            "LARGE_LARGE" => Ok(Self::LargeLarge),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// "ArrowType"
pub enum Arrow {
    /// "NORMAL"
    Normal,
    /// "ARROW"
    Arrow,
    /// "SPEAR"
    Spear,
    /// "CONCAVE_ARROW"
    ConcaveArrow,
    /// "EMPTY_DIAMOND"
    EmptyDiamond,
    /// "EMPTY_CIRCLE"
    EmptyCircle,
    /// "EMPTY_BOX"
    EmptyBox,
    /// "FILLED_DIAMOND"
    FilledDiamond,
    /// "FILLED_CIRCLE"
    FilledCircle,
    /// "FILLED_BOX"
    FilledBox,
}

impl Hancom for Arrow {
    //
}

impl Core for Arrow {
    const NAME: &'static str = "ArrowType";
}

impl Enumeration for Arrow {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Arrow => "ARROW",
            Self::Spear => "SPEAR",
            Self::ConcaveArrow => "CONCAVE_ARROW",
            Self::EmptyDiamond => "EMPTY_DIAMOND",
            Self::EmptyCircle => "EMPTY_CIRCLE",
            Self::EmptyBox => "EMPTY_BOX",
            Self::FilledDiamond => "FILLED_DIAMOND",
            Self::FilledCircle => "FILLED_CIRCLE",
            Self::FilledBox => "FILLED_BOX",
        }
    }
}

impl std::str::FromStr for Arrow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NORMAL" => Ok(Self::Normal),
            "ARROW" => Ok(Self::Arrow),
            "SPEAR" => Ok(Self::Spear),
            "CONCAVE_ARROW" => Ok(Self::ConcaveArrow),
            "EMPTY_DIAMOND" => Ok(Self::EmptyDiamond),
            "EMPTY_CIRCLE" => Ok(Self::EmptyCircle),
            "EMPTY_BOX" => Ok(Self::EmptyBox),
            "FILLED_DIAMOND" => Ok(Self::FilledDiamond),
            "FILLED_CIRCLE" => Ok(Self::FilledCircle),
            "FILLED_BOX" => Ok(Self::FilledBox),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 라틴 문자의 줄나눔 단위
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakLatinWordKind {
    /// "KEEP_WORD"
    ///
    /// 단어 단위로 줄 바꿈 안 함
    KeepWord,
    /// "HYPHENATION"
    ///
    /// 하이픈으로 줄 바꿈
    Hyphenation,
    /// "BREAK_WORD"
    ///
    /// 글자 단위로 줄 바꿈
    BreakWord,
}

impl Hancom for BreakLatinWordKind {
    //
}

impl Arbitrary for BreakLatinWordKind {
    const NAME: &'static str = "$BreakLatinWordKind";
}

impl Enumeration for BreakLatinWordKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::KeepWord => "KEEP_WORD",
            Self::Hyphenation => "HYPHENATION",
            Self::BreakWord => "BREAK_WORD",
        }
    }
}

impl std::str::FromStr for BreakLatinWordKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "KEEP_WORD" => Ok(Self::KeepWord),
            "HYPHENATION" => Ok(Self::Hyphenation),
            "BREAK_WORD" => Ok(Self::BreakWord),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 라틴 문자 이외의 문자의 줄나눔 단위
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakNonLatinWordKind {
    /// "KEEP_WORD"
    ///
    /// 단어 단위로 줄 바꿈 안 함
    KeepWord,
    /// "BREAK_WORD"
    ///
    /// 글자 단위로 줄 바꿈
    BreakWord,
}

impl Hancom for BreakNonLatinWordKind {
    //
}

impl Arbitrary for BreakNonLatinWordKind {
    const NAME: &'static str = "$BreakNonLatinWordKind";
}

impl Enumeration for BreakNonLatinWordKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::KeepWord => "KEEP_WORD",
            Self::BreakWord => "BREAK_WORD",
        }
    }
}

impl std::str::FromStr for BreakNonLatinWordKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "KEEP_WORD" => Ok(Self::KeepWord),
            "BREAK_WORD" => Ok(Self::BreakWord),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 중심선 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CenterLine {
    /// "NONE"
    None,
    /// "VERTICAL"
    Vertical,
    /// "HORIZONTAL"
    Horizontal,
    /// "CROSS"
    Cross,
}

impl Hancom for CenterLine {
    //
}

impl Arbitrary for CenterLine {
    const NAME: &'static str = "$CenterLine";
}

impl Enumeration for CenterLine {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Vertical => "VERTICAL",
            Self::Horizontal => "HORIZONTAL",
            Self::Cross => "CROSS",
        }
    }
}

impl std::str::FromStr for CenterLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Self::None),
            "VERTICAL" => Ok(Self::Vertical),
            "HORIZONTAL" => Ok(Self::Horizontal),
            "CROSS" => Ok(Self::Cross),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// "DropCapStyleType"
pub enum DropCapStyle {
    /// "None"
    None,
    /// "DoubleLine"
    DoubleLine,
    /// "TripleLine"
    TripleLine,
    /// "Margin"
    Margin,
}

impl Hancom for DropCapStyle {
    //
}

impl Core for DropCapStyle {
    const NAME: &'static str = "DropCapStyleType";
}

impl std::str::FromStr for DropCapStyle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "DoubleLine" => Ok(Self::DoubleLine),
            "TripleLine" => Ok(Self::TripleLine),
            "Margin" => Ok(Self::Margin),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamilyType {
    /// "FCAT_UNKNOWN"
    Unknown,
    /// "FCAT_MYUNGJO"
    Myungjo,
    /// "FCAT_GOTHIC"
    ///
    /// serif
    Gothic,
    /// "FCAT_SSERIF"
    ///
    /// sans-serif
    SansSerif,
    /// "FCAT_BRUSHSCRIPT"
    ///
    /// monospace
    BrushScript,
    /// "FCAT_DECORATIVE"
    ///
    /// cursive
    Decorative,
    /// "FCAT_NONRECTMJ"
    ///
    /// serif
    NonRectMj,
    /// "FCAT_NONRECTGT"
    ///
    /// sans-serif
    NonRectGt,
}

impl Hancom for FamilyType {
    //
}

impl Arbitrary for FamilyType {
    const NAME: &'static str = "$FamilyType";
}

impl Enumeration for FamilyType {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Unknown => "FCAT_UNKNOWN",
            Self::Myungjo => "FCAT_MYUNGJO",
            Self::Gothic => "FCAT_GOTHIC",
            Self::SansSerif => "FCAT_SSERIF",
            Self::BrushScript => "FCAT_BRUSHSCRIPT",
            Self::Decorative => "FCAT_DECORATIVE",
            Self::NonRectMj => "FCAT_NONRECTMJ",
            Self::NonRectGt => "FCAT_NONRECTGT",
        }
    }
}

impl std::str::FromStr for FamilyType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FCAT_UNKNOWN" => Ok(Self::Unknown),
            "FCAT_MYUNGJO" => Ok(Self::Myungjo),
            "FCAT_GOTHIC" => Ok(Self::Gothic),
            "FCAT_SSERIF" => Ok(Self::SansSerif),
            "FCAT_BRUSHSCRIPT" => Ok(Self::BrushScript),
            "FCAT_DECORATIVE" => Ok(Self::Decorative),
            "FCAT_NONRECTMJ" => Ok(Self::NonRectMj),
            "FCAT_NONRECTGT" => Ok(Self::NonRectGt),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontKind {
    /// 대표글꼴
    Rep,
    /// 트루타입글꼴
    Ttf,
    /// 한/글 전용 글꼴
    Hft,
}

impl Hancom for FontKind {
    //
}

impl Arbitrary for FontKind {
    const NAME: &'static str = "$FontKind";
}

impl Enumeration for FontKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Rep => "REP",
            Self::Ttf => "TTF",
            Self::Hft => "HFT",
        }
    }
}

impl std::str::FromStr for FontKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "REP" => Ok(Self::Rep),
            "TTF" => Ok(Self::Ttf),
            "HFT" => Ok(Self::Hft),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 그러데이션 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradationKind {
    /// "LINEAR"
    ///
    /// 줄무늬형
    Linear,
    /// "RADIAL"
    ///
    /// 원형
    Radial,
    /// "CONICAL"
    ///
    /// 원뿔형
    Conical,
    /// "SQUARE"
    ///
    /// 사각형
    Square,
}

impl Hancom for GradationKind {
    //
}

impl Arbitrary for GradationKind {
    const NAME: &'static str = "$GradationKind";
}

impl Enumeration for GradationKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Linear => "LINEAR",
            Self::Radial => "RADIAL",
            Self::Conical => "CONICAL",
            Self::Square => "SQUARE",
        }
    }
}

impl std::str::FromStr for GradationKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LINEAR" => Ok(Self::Linear),
            "RADIAL" => Ok(Self::Radial),
            "CONICAL" => Ok(Self::Conical),
            "SQUARE" => Ok(Self::Square),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 무늬 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HatchStyle {
    /// "HORIZONTAL"
    ///
    /// "---"
    Horizontal,
    /// "VERTICAL"
    ///
    ///  "|||"
    Vertical,
    /// "BACK_SLASH"
    ///
    ///  "\\\"
    BackSlash,
    /// "SLASH"
    ///
    ///  "///"
    Slash,
    /// "CROSS"
    ///
    ///  "+++"
    Cross,
    /// "CROSS_DIAGONAL"
    ///
    ///  "XXX"
    CrossDiagonal,
}

impl Hancom for HatchStyle {
    //
}

impl Arbitrary for HatchStyle {
    const NAME: &'static str = "$HatchStyle";
}

impl Enumeration for HatchStyle {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Horizontal => "HORIZONTAL",
            Self::Vertical => "VERTICAL",
            Self::BackSlash => "BACK_SLASH",
            Self::Slash => "SLASH",
            Self::Cross => "CROSS",
            Self::CrossDiagonal => "CROSS_DIAGONAL",
        }
    }
}

impl std::str::FromStr for HatchStyle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HORIZONTAL" => Ok(Self::Horizontal),
            "VERTICAL" => Ok(Self::Vertical),
            "BACK_SLASH" => Ok(Self::BackSlash),
            "SLASH" => Ok(Self::Slash),
            "CROSS" => Ok(Self::Cross),
            "CROSS_DIAGONAL" => Ok(Self::CrossDiagonal),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// "HWPUnit"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HWPUnit {
    /// "CHAR"
    Char,
    /// "HWPUNIT"
    HwpUnit,
}

impl Hancom for HWPUnit {
    //
}

impl Core for HWPUnit {
    const NAME: &'static str = "HWPUnit";
}

impl Enumeration for HWPUnit {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Char => "CHAR",
            Self::HwpUnit => "HWPUNIT",
        }
    }
}

impl std::str::FromStr for HWPUnit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CHAR" => Ok(Self::Char),
            "HWPUNIT" => Ok(Self::HwpUnit),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// "HWPValue"
///
/// (값, 단위)을 표현하기 위한 엘리먼트
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HWPValue {
    pub value: isize,
    pub unit: Option<HWPUnit>,
}

impl Hancom for HWPValue {
    //
}

impl Core for HWPValue {
    const NAME: &'static str = "HWPValue";
}

impl TryFrom<AnyElement> for HWPValue {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        let mut value = None;
        let mut unit = None;

        for (key, value_) in element.attributes {
            match key.as_str() {
                "value" => value = Some(value_.parse()?),
                "unit" => unit = Some(value_.parse()?),
                _ => {}
            }
        }

        let (value,) = match (value,) {
            (Some(v),) => (v,),
            _ => missing_attribute!("<HWPValue value"),
        };

        Ok(Self { value, unit })
    }
}

/// 채우기 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageBrushMode {
    /// "TILE"
    ///
    /// 바둑판식으로 모두
    Tile,
    /// "TILE_HORZ_TOP"
    ///
    /// 바둑판식으로 가로/위
    TileHorizontalTop,
    /// "TILE_HORZ_BOTTOM"
    ///
    /// 바둑판식으로 가로/아래
    TileHorizontalBottom,
    /// "TILE_VERT_LEFT"
    ///
    /// 바둑판식으로 세로/왼쪽
    TileVerticalLeft,
    /// "TILE_VERT_RIGHT"
    ///
    /// 바둑판식으로 세로/오른쪽
    TileVerticalRight,
    /// "TOTAL"
    ///
    /// 크기에 맞추어
    Total,
    /// "CENTER"
    ///
    /// 가운데로
    Center,
    /// "CENTER_TOP"
    ///
    /// 가운데 위로
    CenterTop,
    /// "CENTER_BOTTOM"
    ///
    /// 가운데 아래로
    CenterBottom,
    /// "LEFT_CENTER"
    ///
    /// 왼쪽 가운데로
    LeftCenter,
    /// "LEFT_TOP"
    ///
    /// 왼쪽 위로
    LeftTop,
    /// "LEFT_BOTTOM"
    ///
    /// 왼쪽 아래로
    LeftBottom,
    /// "RIGHT_CENTER"
    ///
    /// 오른쪽 가운데로
    RightCenter,
    /// "RIGHT_TOP"
    ///
    /// 오른쪽 위로
    RightTop,
    /// "RIGHT_BOTTOM"
    ///
    /// 오른쪽 아래로
    RightBottom,
    /// "ZOOM"
    Zoom,
}

impl Hancom for ImageBrushMode {
    //
}

impl Arbitrary for ImageBrushMode {
    const NAME: &'static str = "$ImageBrushMode";
}

impl Enumeration for ImageBrushMode {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Tile => "TILE",
            Self::TileHorizontalTop => "TILE_HORZ_TOP",
            Self::TileHorizontalBottom => "TILE_HORZ_BOTTOM",
            Self::TileVerticalLeft => "TILE_VERT_LEFT",
            Self::TileVerticalRight => "TILE_VERT_RIGHT",
            Self::Total => "TOTAL",
            Self::Center => "CENTER",
            Self::CenterTop => "CENTER_TOP",
            Self::CenterBottom => "CENTER_BOTTOM",
            Self::LeftCenter => "LEFT_CENTER",
            Self::LeftTop => "LEFT_TOP",
            Self::LeftBottom => "LEFT_BOTTOM",
            Self::RightCenter => "RIGHT_CENTER",
            Self::RightTop => "RIGHT_TOP",
            Self::RightBottom => "RIGHT_BOTTOM",
            Self::Zoom => "ZOOM",
        }
    }
}

impl std::str::FromStr for ImageBrushMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TILE" => Ok(Self::Tile),
            "TILE_HORZ_TOP" => Ok(Self::TileHorizontalTop),
            "TILE_HORZ_BOTTOM" => Ok(Self::TileHorizontalBottom),
            "TILE_VERT_LEFT" => Ok(Self::TileVerticalLeft),
            "TILE_VERT_RIGHT" => Ok(Self::TileVerticalRight),
            "TOTAL" => Ok(Self::Total),
            "CENTER" => Ok(Self::Center),
            "CENTER_TOP" => Ok(Self::CenterTop),
            "CENTER_BOTTOM" => Ok(Self::CenterBottom),
            "LEFT_CENTER" => Ok(Self::LeftCenter),
            "LEFT_TOP" => Ok(Self::LeftTop),
            "LEFT_BOTTOM" => Ok(Self::LeftBottom),
            "RIGHT_CENTER" => Ok(Self::RightCenter),
            "RIGHT_TOP" => Ok(Self::RightTop),
            "RIGHT_BOTTOM" => Ok(Self::RightBottom),
            "ZOOM" => Ok(Self::Zoom),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 그림 효과
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageEffect {
    /// "REAL_PIC"
    ///
    /// 원래 그림에서
    RealPicture,
    /// "GRAY_SCALE"
    ///
    /// 그레이 스케일로
    GrayScale,
    /// "BLACK_WHITE"
    ///
    /// 흑백으로
    BlackWhite,
}

impl Hancom for ImageEffect {
    //
}

impl Arbitrary for ImageEffect {
    const NAME: &'static str = "$ImageEffect";
}

impl Enumeration for ImageEffect {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::RealPicture => "REAL_PIC",
            Self::GrayScale => "GRAY_SCALE",
            Self::BlackWhite => "BLACK_WHITE",
        }
    }
}

impl std::str::FromStr for ImageEffect {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "REAL_PIC" => Ok(Self::RealPicture),
            "GRAY_SCALE" => Ok(Self::GrayScale),
            "BLACK_WHITE" => Ok(Self::BlackWhite),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 언어(한글, 영어, 한자, 일어, 기타, 심볼, 사용자)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// "HANGUL"
    Hangul,
    /// "LATIN"
    Latin,
    /// "HANJA"
    Hanja,
    /// "JAPANESE"
    Japanese,
    /// "OTHER"
    Other,
    /// "SYMBOL"
    Symbol,
    /// "USER"
    User,
}

impl Hancom for Language {
    //
}

impl Arbitrary for Language {
    const NAME: &'static str = "$Language";
}

impl Enumeration for Language {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Hangul => "HANGUL",
            Self::Latin => "LATIN",
            Self::Hanja => "HANJA",
            Self::Japanese => "JAPANESE",
            Self::Other => "OTHER",
            Self::Symbol => "SYMBOL",
            Self::User => "USER",
        }
    }
}

impl std::str::FromStr for Language {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HANGUL" => Ok(Self::Hangul),
            "LATIN" => Ok(Self::Latin),
            "HANJA" => Ok(Self::Hanja),
            "JAPANESE" => Ok(Self::Japanese),
            "OTHER" => Ok(Self::Other),
            "SYMBOL" => Ok(Self::Symbol),
            "USER" => Ok(Self::User),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutCompatibilityKind {
    /// <applyFontWeightToBold />
    ApplyFontWeightToBold,
    /// <useInnerUnderline />
    UseInnerUnderline,
    /// <fixedUnderlineWidth />
    FixedUnderlineWidth,
    /// <doNotApplyStrikeoutWithUnderline />
    DoNotApplyStrikeoutWithUnderline,
    /// <useLowercaseStrikeout />
    UseLowercaseStrikeout,
    /// <extendLineheightToOffset />
    ExtendLineheightToOffset,
    /// <applyFontspaceToLatin />
    ApplyFontspaceToLatin,
    /// <treatQuotationAsLatin />
    TreatQuotationAsLatin,
    /// <doNotApplyDiacSymMarkOfNoneAndSix />
    DoNotApplyDiacSymMarkOfNoneAndSix,
    /// <doNotAlignWhitespaceOnRight />
    DoNotAlignWhitespaceOnRight,
    /// <doNotAdjustWordInJustify />
    DoNotAdjustWordInJustify,
    /// <baseCharUnitOnEAsian />
    BaseCharUnitOnEAsian,
    /// <baseCharUnitOfIndentOnFirstChar />
    BaseCharUnitOfIndentOnFirstChar,
    /// <adjustLineheightToFont />
    AdjustLineheightToFont,
    /// <adjustBaseInlineFixedLinespacing />
    AdjustBaseInlineFixedLinespacing,
    /// <applyPrevspacingBeneathObject />
    ApplyPrevspacingBeneathObject,
    /// <applyNextspacingOfLastPara />
    ApplyNextspacingOfLastPara,
    /// <applyAtLeastToPercent100Pct />
    ApplyAtLeastToPercent100Pct,
    /// <doNotApplyAutoSpaceEAsianEng />
    DoNotApplyAutoSpaceEAsianEng,
    /// <doNotApplyAutoSpaceEAsianNum />
    DoNotApplyAutoSpaceEAsianNum,
    /// <adjustParaBorderfillToSpacing />
    AdjustParaBorderfillToSpacing,
    /// <connectParaBorderfillOfEqualBorder />
    ConnectParaBorderfillOfEqualBorder,
    /// <adjustParaBorderOffsetWithBorder />
    AdjustParaBorderOffsetWithBorder,
    /// <extendLineheightToParaBorderOffset />
    ExtendLineheightToParaBorderOffset,
    /// <applyParaBorderToOutside />
    ApplyParaBorderToOutside,
    /// <applyMinColumnWidthTo1mm />
    ApplyMinColumnWidthTo1mm,
    /// <applyTabPosBasedOnSegment />
    ApplyTabPosBasedOnSegment,
    /// <breakTabOverline />
    BreakTabOverline,
    /// <adjustVertPosOfLine />
    AdjustVertPosOfLine,
    /// <doNotApplyWhiteSpaceHeight />
    DoNotApplyWhiteSpaceHeight,
    /// <doNotAlignLastPeriod />
    DoNotAlignLastPeriod,
    /// <doNotAlignLastForbidden />
    DoNotAlignLastForbidden,
    /// <baseLineSpacingOnLineGrid />
    BaseLineSpacingOnLineGrid,
    /// <applyCharSpacingToCharGrid />
    ApplyCharSpacingToCharGrid,
    /// <doNotApplyGridInHeaderFooter />
    DoNotApplyGridInHeaderFooter,
    /// <applyExtendHeaderFooterEachSection />
    ApplyExtendHeaderFooterEachSection,
    /// <doNotApplyHeaderFooterAtNoSpace />
    DoNotApplyHeaderFooterAtNoSpace,
    /// <doNotApplyColSeparatorAtNoGap />
    DoNotApplyColSeparatorAtNoGap,
    /// <doNotApplyLinegridAtNoLinespacing />
    DoNotApplyLinegridAtNoLinespacing,
    /// <doNotApplyImageEffect />
    DoNotApplyImageEffect,
    /// <doNotApplyShapeComment />
    DoNotApplyShapeComment,
    /// <doNotAdjustEmptyAnchorLine />
    DoNotAdjustEmptyAnchorLine,
    /// <overlapBothAllowOverlap />
    OverlapBothAllowOverlap,
    /// <doNotApplyVertOffsetOfForward />
    DoNotApplyVertOffsetOfForward,
    /// <extendVertLimitToPageMargins />
    ExtendVertLimitToPageMargins,
    /// <doNotHoldAnchorOfTable />
    DoNotHoldAnchorOfTable,
    /// <doNotFormattingAtBeneathAnchor />
    DoNotFormattingAtBeneathAnchor,
    /// <adjustBaselineOfObjectToBottom />
    AdjustBaselineOfObjectToBottom,
    /// <doNotApplyExtensionCharCompose />
    DoNotApplyExtensionCharCompose,
}

impl LayoutCompatibilityKind {
    pub fn from_element_name(element_name: ElementName) -> Option<Self> {
        macro_rules! mapping {
            ($($element_name:ident => $variant:ident),+ $(,)?) => {
                match element_name {
                    $(ElementName::$element_name => Some(Self::$variant),)+
                    _ => None,
                }

            }

        }

        mapping! {
            HANCOM__HEAD__APPLY_FONT_WEIGHT_TO_BOLD => ApplyFontWeightToBold,
            HANCOM__HEAD__USE_INNER_UNDERLINE => UseInnerUnderline,
            HANCOM__HEAD__FIXED_UNDERLINE_WIDTH => FixedUnderlineWidth,
            HANCOM__HEAD__DO_NOT_APPLY_STRIKEOUT_WITH_UNDERLINE => DoNotApplyStrikeoutWithUnderline,
            HANCOM__HEAD__USE_LOWERCASE_STRIKEOUT => UseLowercaseStrikeout,
            HANCOM__HEAD__EXTEND_LINEHEIGHT_TO_OFFSET => ExtendLineheightToOffset,
            HANCOM__HEAD__APPLY_FONTSPACE_TO_LATIN => ApplyFontspaceToLatin,
            HANCOM__HEAD__TREAT_QUOTATION_AS_LATIN => TreatQuotationAsLatin,
            HANCOM__HEAD__DO_NOT_APPLY_DIAC_SYM_MARK_OF_NONE_AND_SIX => DoNotApplyDiacSymMarkOfNoneAndSix,
            HANCOM__HEAD__DO_NOT_ALIGN_WHITESPACE_ON_RIGHT => DoNotAlignWhitespaceOnRight,
            HANCOM__HEAD__DO_NOT_ADJUST_WORD_IN_JUSTIFY => DoNotAdjustWordInJustify,
            HANCOM__HEAD__BASE_CHAR_UNIT_ON_E_ASIAN => BaseCharUnitOnEAsian,
            HANCOM__HEAD__BASE_CHAR_UNIT_OF_INDENT_ON_FIRST_CHAR => BaseCharUnitOfIndentOnFirstChar,
            HANCOM__HEAD__ADJUST_LINEHEIGHT_TO_FONT => AdjustLineheightToFont,
            HANCOM__HEAD__ADJUST_BASE_INLINE_FIXED_LINESPACING => AdjustBaseInlineFixedLinespacing,
            HANCOM__HEAD__APPLY_PREVSPACING_BENEATH_OBJECT => ApplyPrevspacingBeneathObject,
            HANCOM__HEAD__APPLY_NEXTSPACING_OF_LAST_PARA => ApplyNextspacingOfLastPara,
            HANCOM__HEAD__APPLY_AT_LEAST_TO_PERCENT100_PCT => ApplyAtLeastToPercent100Pct,
            HANCOM__HEAD__DO_NOT_APPLY_AUTO_SPACE_E_ASIAN_ENG => DoNotApplyAutoSpaceEAsianEng,
            HANCOM__HEAD__DO_NOT_APPLY_AUTO_SPACE_E_ASIAN_NUM => DoNotApplyAutoSpaceEAsianNum,
            HANCOM__HEAD__ADJUST_PARA_BORDERFILL_TO_SPACING => AdjustParaBorderfillToSpacing,
            HANCOM__HEAD__CONNECT_PARA_BORDERFILL_OF_EQUAL_BORDER => ConnectParaBorderfillOfEqualBorder,
            HANCOM__HEAD__ADJUST_PARA_BORDER_OFFSET_WITH_BORDER => AdjustParaBorderOffsetWithBorder,
            HANCOM__HEAD__EXTEND_LINEHEIGHT_TO_PARA_BORDER_OFFSET => ExtendLineheightToParaBorderOffset,
            HANCOM__HEAD__APPLY_PARA_BORDER_TO_OUTSIDE => ApplyParaBorderToOutside,
            HANCOM__HEAD__APPLY_MIN_COLUMN_WIDTH_TO1MM => ApplyMinColumnWidthTo1mm,
            HANCOM__HEAD__APPLY_TAB_POS_BASED_ON_SEGMENT => ApplyTabPosBasedOnSegment,
            HANCOM__HEAD__BREAK_TAB_OVERLINE => BreakTabOverline,
            HANCOM__HEAD__ADJUST_VERT_POS_OF_LINE => AdjustVertPosOfLine,
            HANCOM__HEAD__DO_NOT_APPLY_WHITE_SPACE_HEIGHT => DoNotApplyWhiteSpaceHeight,
            HANCOM__HEAD__DO_NOT_ALIGN_LAST_PERIOD => DoNotAlignLastPeriod,
            HANCOM__HEAD__DO_NOT_ALIGN_LAST_FORBIDDEN => DoNotAlignLastForbidden,
            HANCOM__HEAD__BASE_LINE_SPACING_ON_LINE_GRID => BaseLineSpacingOnLineGrid,
            HANCOM__HEAD__APPLY_CHAR_SPACING_TO_CHAR_GRID => ApplyCharSpacingToCharGrid,
            HANCOM__HEAD__DO_NOT_APPLY_GRID_IN_HEADER_FOOTER => DoNotApplyGridInHeaderFooter,
            HANCOM__HEAD__APPLY_EXTEND_HEADER_FOOTER_EACH_SECTION => ApplyExtendHeaderFooterEachSection,
            HANCOM__HEAD__DO_NOT_APPLY_HEADER_FOOTER_AT_NO_SPACE => DoNotApplyHeaderFooterAtNoSpace,
            HANCOM__HEAD__DO_NOT_APPLY_COL_SEPARATOR_AT_NO_GAP => DoNotApplyColSeparatorAtNoGap,
            HANCOM__HEAD__DO_NOT_APPLY_LINEGRID_AT_NO_LINESPACING => DoNotApplyLinegridAtNoLinespacing,
            HANCOM__HEAD__DO_NOT_APPLY_IMAGE_EFFECT => DoNotApplyImageEffect,
            HANCOM__HEAD__DO_NOT_APPLY_SHAPE_COMMENT => DoNotApplyShapeComment,
            HANCOM__HEAD__DO_NOT_ADJUST_EMPTY_ANCHOR_LINE => DoNotAdjustEmptyAnchorLine,
            HANCOM__HEAD__OVERLAP_BOTH_ALLOW_OVERLAP => OverlapBothAllowOverlap,
            HANCOM__HEAD__DO_NOT_APPLY_VERT_OFFSET_OF_FORWARD => DoNotApplyVertOffsetOfForward,
            HANCOM__HEAD__EXTEND_VERT_LIMIT_TO_PAGE_MARGINS => ExtendVertLimitToPageMargins,
            HANCOM__HEAD__DO_NOT_HOLD_ANCHOR_OF_TABLE => DoNotHoldAnchorOfTable,
            HANCOM__HEAD__DO_NOT_FORMATTING_AT_BENEATH_ANCHOR => DoNotFormattingAtBeneathAnchor,
            HANCOM__HEAD__ADJUST_BASELINE_OF_OBJECT_TO_BOTTOM => AdjustBaselineOfObjectToBottom,
            HANCOM__HEAD__DO_NOT_APPLY_EXTENSION_CHAR_COMPOSE => DoNotApplyExtensionCharCompose,
        }
    }
}

/// 줄 간격 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineSpacingKind {
    /// "PERCENT"
    ///
    /// 글자에 따라
    Percent,
    /// "FIXED"
    ///
    /// 고정 값
    Fixed,
    /// "BETWEEN_LINES"
    ///
    /// 여백만 지정
    BetweenLines,
    /// "AT_LEAST"
    ///
    /// 최소
    AtLeast,
}

impl Hancom for LineSpacingKind {
    //
}

impl Arbitrary for LineSpacingKind {
    const NAME: &'static str = "$LineSpacingKind";
}

impl Enumeration for LineSpacingKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Percent => "PERCENT",
            Self::Fixed => "FIXED",
            Self::BetweenLines => "BETWEEN_LINES",
            Self::AtLeast => "AT_LEAST",
        }
    }
}

impl std::str::FromStr for LineSpacingKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PERCENT" => Ok(Self::Percent),
            "FIXED" => Ok(Self::Fixed),
            "BETWEEN_LINES" => Ok(Self::BetweenLines),
            "AT_LEAST" => Ok(Self::AtLeast),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// "LineType1"
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineType1 {
    /// "NONE"
    None,
    /// "SOLID"
    ///
    /// 실선
    Solid,
    /// "DOUBLE"
    ///
    /// 점선
    Dot,
    /// "THICK"
    ///
    /// 두꺼운 선
    Thick,
    /// "DASH"
    ///
    /// 긴 점선
    Dash,
    /// "DASH_DOT"
    ///
    /// "-.-.-"
    DashDot,
    /// "DASH_DOT_DOT"
    ///
    /// "-..-..-"
    DashDotDot,
}

impl Hancom for LineType1 {
    //
}

impl Core for LineType1 {
    const NAME: &'static str = "LineType1";
}

impl Enumeration for LineType1 {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Solid => "SOLID",
            Self::Dot => "DOUBLE",
            Self::Thick => "THICK",
            Self::Dash => "DASH",
            Self::DashDot => "DASH_DOT",
            Self::DashDotDot => "DASH_DOT_DOT",
        }
    }
}

impl std::str::FromStr for LineType1 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Self::None),
            "SOLID" => Ok(Self::Solid),
            "DOUBLE" => Ok(Self::Dot),
            "THICK" => Ok(Self::Thick),
            "DASH" => Ok(Self::Dash),
            "DASH_DOT" => Ok(Self::DashDot),
            "DASH_DOT_DOT" => Ok(Self::DashDotDot),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// "LineType2"
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineType2 {
    /// "NONE"
    None,
    /// "SOLID"
    ///
    /// 실선
    Solid,
    /// "DOT"
    ///
    /// 점선
    Dot,
    /// "DASH"
    ///
    /// 긴 점선
    Dash,
    /// "DASH_DOT"
    ///
    /// "-.-.-"
    DashDot,
    /// "DASH_DOT_DOT"
    ///
    /// "-..-..-"
    DashDotDot,
    /// "LONG_DASH"
    ///
    /// DASH 보다 긴 선의 반복
    LongDash,
    /// "DOUBLE_SLIM",
    ///
    /// 2중선(가는 선 + 가는 선)
    DoubleSlim,
    /// "THICK_SLIM",
    ///
    /// 2중선(굵은 선 + 가는 선)
    ThickSlim,
}

impl Hancom for LineType2 {
    //
}

impl Core for LineType2 {
    const NAME: &'static str = "LineType2";
}

impl Enumeration for LineType2 {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Solid => "SOLID",
            Self::Dot => "DOUBLE",
            Self::Dash => "DASH",
            Self::DashDot => "DASH_DOT",
            Self::DashDotDot => "DASH_DOT_DOT",
            Self::LongDash => "LONG_DASH",
            Self::DoubleSlim => "DOUBLE_SLIM",
            Self::ThickSlim => "THICK_SLIM",
        }
    }
}

impl std::str::FromStr for LineType2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Self::None),
            "SOLID" => Ok(Self::Solid),
            "DOUBLE" => Ok(Self::Dot),
            "DASH" => Ok(Self::Dash),
            "DASH_DOT" => Ok(Self::DashDot),
            "DASH_DOT_DOT" => Ok(Self::DashDotDot),
            "LONG_DASH" => Ok(Self::LongDash),
            "DOUBLE_SLIM" => Ok(Self::DoubleSlim),
            "THICK_SLIM" => Ok(Self::ThickSlim),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// "LineType3"
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineType3 {
    /// "NONE"
    None,
    /// "DOUBLE"
    ///
    /// 점선
    Dot,
    /// "DASH"
    ///
    /// 긴 점선
    Dash,
    /// "DASH_DOT_DOT"
    ///
    /// "-..-..-"
    DashDotDot,
    /// "LONG_DASH"
    ///
    /// DASH 보다 긴 선의 반복
    LongDash,
    /// "CIRCLE"
    ///
    /// DOT 보다 더 큰 동그라미 반복
    Circle,
    /// "DOUBLE_SLIM",
    ///
    /// 2중선(가는 선 + 가는 선)
    DoubleSlim,
    /// "SLIM_THICK",
    ///
    /// 2중선(가는 선 + 굵은 선)
    SlimThick,
    /// "THICK_SLIM",
    ///
    /// 2중선(굵은 선 + 가는 선)
    ThickSlim,
    /// "THICK_SLIM",
    ///
    /// 2중선(가는 선 + 굵은 선 + 가는 선)
    SlimThickSlim,
    /// "WAVE"
    Wave,
    /// "DOUBLEWAVE"
    DoubleWave,
}

impl Hancom for LineType3 {
    //
}

impl Core for LineType3 {
    const NAME: &'static str = "LineType3";
}

impl Enumeration for LineType3 {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Dot => "DOUBLE",
            Self::Dash => "DASH",
            Self::DashDotDot => "DASH_DOT_DOT",
            Self::LongDash => "LONG_DASH",
            Self::Circle => "CIRCLE",
            Self::DoubleSlim => "DOUBLE_SLIM",
            Self::SlimThick => "SLIM_THICK",
            Self::ThickSlim => "THICK_SLIM",
            Self::SlimThickSlim => "SLIM_THICK_SLIM",
            Self::Wave => "WAVE",
            Self::DoubleWave => "DOUBLEWAVE",
        }
    }
}

impl std::str::FromStr for LineType3 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Self::None),
            "DOUBLE" => Ok(Self::Dot),
            "DASH" => Ok(Self::Dash),
            "DASH_DOT_DOT" => Ok(Self::DashDotDot),
            "LONG_DASH" => Ok(Self::LongDash),
            "CIRCLE" => Ok(Self::Circle),
            "DOUBLE_SLIM" => Ok(Self::DoubleSlim),
            "SLIM_THICK" => Ok(Self::SlimThick),
            "THICK_SLIM" => Ok(Self::ThickSlim),
            "SLIM_THICK_SLIM" => Ok(Self::SlimThickSlim),
            "WAVE" => Ok(Self::Wave),
            "DOUBLEWAVE" => Ok(Self::DoubleWave),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// "LineWidth"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineWidth {
    /// "0.1 mm"
    _0_1,
    /// "0.12 mm"
    _0_12,
    /// "0.15 mm"
    _0_15,
    /// "0.2 mm"
    _0_2,
    /// "0.25 mm"
    _0_25,
    /// "0.3 mm"
    _0_3,
    /// "0.4 mm"
    _0_4,
    /// "0.5 mm"
    _0_5,
    /// "0.6 mm"
    _0_6,
    /// "0.7 mm"
    _0_7,
    /// "1.0 mm"
    _1_0,
    /// "1.5 mm"
    _1_5,
    /// "2.0 mm"
    _2_0,
    /// "3.0 mm"
    _3_0,
    /// "4.0 mm"
    _4_0,
    /// "5.0 mm"
    _5_0,
}

impl Hancom for LineWidth {
    //
}

impl Core for LineWidth {
    const NAME: &'static str = "LineWidth";
}

impl Enumeration for LineWidth {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::_0_1 => "0.1 mm",
            Self::_0_12 => "0.12 mm",
            Self::_0_15 => "0.15 mm",
            Self::_0_2 => "0.2 mm",
            Self::_0_25 => "0.25 mm",
            Self::_0_3 => "0.3 mm",
            Self::_0_4 => "0.4 mm",
            Self::_0_5 => "0.5 mm",
            Self::_0_6 => "0.6 mm",
            Self::_0_7 => "0.7 mm",
            Self::_1_0 => "1.0 mm",
            Self::_1_5 => "1.5 mm",
            Self::_2_0 => "2.0 mm",
            Self::_3_0 => "3.0 mm",
            Self::_4_0 => "4.0 mm",
            Self::_5_0 => "5.0 mm",
        }
    }
}

impl std::str::FromStr for LineWidth {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "0.1 mm" => Ok(Self::_0_1),
            "0.12 mm" => Ok(Self::_0_12),
            "0.15 mm" => Ok(Self::_0_15),
            "0.2 mm" => Ok(Self::_0_2),
            "0.25 mm" => Ok(Self::_0_25),
            "0.3 mm" => Ok(Self::_0_3),
            "0.4 mm" => Ok(Self::_0_4),
            "0.5 mm" => Ok(Self::_0_5),
            "0.6 mm" => Ok(Self::_0_6),
            "0.7 mm" => Ok(Self::_0_7),
            "1.0 mm" => Ok(Self::_1_0),
            "1.5 mm" => Ok(Self::_1_5),
            "2.0 mm" => Ok(Self::_2_0),
            "3.0 mm" => Ok(Self::_3_0),
            "4.0 mm" => Ok(Self::_4_0),
            "5.0 mm" => Ok(Self::_5_0),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 한 줄로 입력 사용 시의 형식
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineWrapKind {
    /// "BREAK"
    ///
    /// 일반적인 줄 바꿈
    BREAK,
    /// "SQUEEZE"
    ///
    /// 자간을 조정하여 한 줄을 유지
    SQUEEZE,
    /// "KEEP"
    ///
    /// 줄 바꿈 안 함
    KEEP,
}

impl Hancom for LineWrapKind {
    //
}

impl Arbitrary for LineWrapKind {
    const NAME: &'static str = "$LineWrapKind";
}

impl Enumeration for LineWrapKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::BREAK => "BREAK",
            Self::SQUEEZE => "SQUEEZE",
            Self::KEEP => "KEEP",
        }
    }
}

impl std::str::FromStr for LineWrapKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BREAK" => Ok(Self::BREAK),
            "SQUEEZE" => Ok(Self::SQUEEZE),
            "KEEP" => Ok(Self::KEEP),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    /// "application/xml"
    Xml,
    /// "application/javascript"
    JavaScript,
    /// "image/jpg" or "image/jpeg"
    Jpeg,
    /// "image/png"
    Png,
    /// "image/gif"
    Gif,
    /// "text/plain"
    PlainText,
    /// "application/rdf+xml"
    Rdf,
    /// "application/hwpml-package"
    HwpmlPackage,
}

impl Hancom for MediaType {
    //
}

impl Arbitrary for MediaType {
    const NAME: &'static str = "$MediaType";
}

impl Enumeration for MediaType {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Xml => "application/xml",
            Self::JavaScript => "application/javascript",
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Gif => "image/gif",
            Self::PlainText => "text/plain",
            Self::Rdf => "application/rdf+xml",
            Self::HwpmlPackage => "application/hwpml-package",
        }
    }
}

impl std::str::FromStr for MediaType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "application/xml" => Ok(MediaType::Xml),
            "application/javascript" => Ok(MediaType::JavaScript),
            "image/jpg" | "image/jpeg" => Ok(MediaType::Jpeg),
            "image/png" => Ok(MediaType::Png),
            "image/gif" => Ok(MediaType::Gif),
            "text/plain" => Ok(MediaType::PlainText),
            "application/rdf+xml" => Ok(MediaType::Rdf),
            "application/hwpml-package" => Ok(MediaType::HwpmlPackage),
            _ => unknown!(
                "Unknown media type: {}
            ",
                s
            ),
        }
    }
}

/// 메모 변경 추적을 위한 속성
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoKind {
    /// "NORMAL"
    Normal,
    /// "USER_INSERT"
    UserInsert,
    /// "USER_DELETE"
    UserDelete,
    /// "USER_UPDATE"
    UserUpdate,
}

impl Hancom for MemoKind {
    //
}

impl Arbitrary for MemoKind {
    const NAME: &'static str = "$MemoKind";
}

impl Enumeration for MemoKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::UserInsert => "USER_INSERT",
            Self::UserDelete => "USER_DELETE",
            Self::UserUpdate => "USER_UPDATE",
        }
    }
}

impl std::str::FromStr for MemoKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NORMAL" => Ok(Self::Normal),
            "USER_INSERT" => Ok(Self::UserInsert),
            "USER_DELETE" => Ok(Self::UserDelete),
            "USER_UPDATE" => Ok(Self::UserUpdate),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdRef(pub String);

/// "NumberType1"
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberType1 {
    /// "DIGIT"
    ///
    /// "1", "2", "3", ...
    Digit,
    /// "CIRCLED_DIGIT"
    ///
    /// "①", "②", "③", ...
    Circled,
    /// "ROMAN_CAPITAL"
    ///
    /// "I", "II", "III", ...
    RomanCapital,
    /// "ROMAN_SMALL"
    ///
    /// "i", "ii", "iii", ...
    RomanSmall,
    /// "LATIN_CAPITAL"
    ///
    /// "A", "B", "C", ...
    LatinCapital,
    /// "LATIN_SMALL"
    ///
    /// "a", "b", "c", ...
    LatinSmall,
    /// "CIRCLED_LATIN_CAPITAL"
    ///
    /// "Ⓐ", "Ⓑ", "Ⓒ", ...
    CircledLatinCapital,
    /// "CIRCLED_LATIN_SMALL"
    ///
    /// "ⓐ", "ⓑ", "ⓒ", ...
    CircledLatinSmall,
    /// "HANGUL_SYLLABLE"
    ///
    /// "가", "나", "다", ...
    HangulSyllable,
    /// "CIRCLED_HANGUL_SYLLABLE"
    ///
    /// "Ⓐ", "Ⓑ", "Ⓒ", ...
    CircledHangulSyllable,
    /// "HANGUL_JAMO"
    ///
    /// "ㄱ", "ㄴ", "ㄷ", ...
    HangulJamo,
    /// "CIRCLED_HANGUL_JAMO"
    ///
    /// "Ⓐ", "Ⓑ", "Ⓒ", ...
    CircledHangulJamo,
    /// "HANGUL_PHONETIC"
    ///
    /// "ᄀ", "ᄂ", "ᄃ", ...
    HangulPhonetic,
    /// "IDEOGRAPH"
    ///
    /// "一", "二", "三", ...
    Ideograph,
    /// "CIRCLED_IDEOGRAPH"
    ///
    /// "㊀", "㊁", "㊂", ...
    CircledIdeograph,
}

impl Hancom for NumberType1 {
    //
}

impl Core for NumberType1 {
    const NAME: &'static str = "NumberType1";
}

impl Enumeration for NumberType1 {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Digit => "DIGIT",
            Self::Circled => "CIRCLED_DIGIT",
            Self::RomanCapital => "ROMAN_CAPITAL",
            Self::RomanSmall => "ROMAN_SMALL",
            Self::LatinCapital => "LATIN_CAPITAL",
            Self::LatinSmall => "LATIN_SMALL",
            Self::CircledLatinCapital => "CIRCLED_LATIN_CAPITAL",
            Self::CircledLatinSmall => "CIRCLED_LATIN_SMALL",
            Self::HangulSyllable => "HANGUL_SYLLABLE",
            Self::CircledHangulSyllable => "CIRCLED_HANGUL_SYLLABLE",
            Self::HangulJamo => "HANGUL_JAMO",
            Self::CircledHangulJamo => "CIRCLED_HANGUL_JAMO",
            Self::HangulPhonetic => "HANGUL_PHONETIC",
            Self::Ideograph => "IDEOGRAPH",
            Self::CircledIdeograph => "CIRCLED_IDEOGRAPH",
        }
    }
}

impl std::str::FromStr for NumberType1 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DIGIT" => Ok(Self::Digit),
            "CIRCLED_DIGIT" => Ok(Self::Circled),
            "ROMAN_CAPITAL" => Ok(Self::RomanCapital),
            "ROMAN_SMALL" => Ok(Self::RomanSmall),
            "LATIN_CAPITAL" => Ok(Self::LatinCapital),
            "LATIN_SMALL" => Ok(Self::LatinSmall),
            "CIRCLED_LATIN_CAPITAL" => Ok(Self::CircledLatinCapital),
            "CIRCLED_LATIN_SMALL" => Ok(Self::CircledLatinSmall),
            "HANGUL_SYLLABLE" => Ok(Self::HangulSyllable),
            "CIRCLED_HANGUL_SYLLABLE" => Ok(Self::CircledHangulSyllable),
            "HANGUL_JAMO" => Ok(Self::HangulJamo),
            "CIRCLED_HANGUL_JAMO" => Ok(Self::CircledHangulJamo),
            "HANGUL_PHONETIC" => Ok(Self::HangulPhonetic),
            "IDEOGRAPH" => Ok(Self::Ideograph),
            "CIRCLED_IDEOGRAPH" => Ok(Self::CircledIdeograph),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// "NumberType2"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberType2 {
    /// "DIGIT"
    ///
    /// "1", "2", "3", ...
    Digit,
    /// "CIRCLED_DIGIT"
    ///
    /// "①", "②", "③", ...
    Circled,
    /// "ROMAN_CAPITAL"
    ///
    /// "I", "II", "III", ...
    RomanCapital,
    /// "ROMAN_SMALL"
    ///
    /// "i", "ii", "iii", ...
    RomanSmall,
    /// "LATIN_CAPITAL"
    ///
    /// "A", "B", "C", ...
    LatinCapital,
    /// "LATIN_SMALL"
    ///
    /// "a", "b", "c", ...
    LatinSmall,
    /// "CIRCLED_LATIN_CAPITAL"
    ///
    /// "Ⓐ", "Ⓑ", "Ⓒ", ...
    CircledLatinCapital,
    /// "CIRCLED_LATIN_SMALL"
    ///
    /// "ⓐ", "ⓑ", "ⓒ", ...
    CircledLatinSmall,
    /// "HANGUL_SYLLABLE"
    ///
    /// "가", "나", "다", ...
    HangulSyllable,
    /// "CIRCLED_HANGUL_SYLLABLE"
    ///
    /// "Ⓐ", "Ⓑ", "Ⓒ", ...
    CircledHangulSyllable,
    /// "HANGUL_JAMO"
    ///
    /// "ㄱ", "ㄴ", "ㄷ", ...
    HangulJamo,
    /// "CIRCLED_HANGUL_JAMO"
    ///
    /// "Ⓐ", "Ⓑ", "Ⓒ", ...
    CircledHangulJamo,
    /// "HANGUL_PHONETIC"
    ///
    /// "ᄀ", "ᄂ", "ᄃ", ...
    HangulPhonetic,
    /// "IDEOGRAPH"
    ///
    /// "一", "二", "三", ...
    Ideograph,
    /// "CIRCLED_IDEOGRAPH"
    ///
    /// "㊀", "㊁", "㊂", ...
    CircledIdeograph,
    /// "DECAGON_CIRCLE"
    ///
    /// "갑", "을", "병", ...
    DecagonCircle,
    /// "DECAGON_CIRCLE_HANJA"
    ///
    /// "甲", "乙", "丙", ...
    DecagonCircleHanja,
    /// "SYMBOL"
    ///
    /// 4가지 심볼을 차례로 반복
    Symbol,
    /// "USER_CHAR"
    ///
    /// 사용자 정의 문자 반복
    UserChar,
}

impl Hancom for NumberType2 {
    //
}

impl Core for NumberType2 {
    const NAME: &'static str = "NumberType2";
}

impl Enumeration for NumberType2 {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Digit => "DIGIT",
            Self::Circled => "CIRCLED_DIGIT",
            Self::RomanCapital => "ROMAN_CAPITAL",
            Self::RomanSmall => "ROMAN_SMALL",
            Self::LatinCapital => "LATIN_CAPITAL",
            Self::LatinSmall => "LATIN_SMALL",
            Self::CircledLatinCapital => "CIRCLED_LATIN_CAPITAL",
            Self::CircledLatinSmall => "CIRCLED_LATIN_SMALL",
            Self::HangulSyllable => "HANGUL_SYLLABLE",
            Self::CircledHangulSyllable => "CIRCLED_HANGUL_SYLLABLE",
            Self::HangulJamo => "HANGUL_JAMO",
            Self::CircledHangulJamo => "CIRCLED_HANGUL_JAMO",
            Self::HangulPhonetic => "HANGUL_PHONETIC",
            Self::Ideograph => "IDEOGRAPH",
            Self::CircledIdeograph => "CIRCLED_IDEOGRAPH",
            Self::DecagonCircle => "DECAGON_CIRCLE",
            Self::DecagonCircleHanja => "DECAGON_CIRCLE_HANJA",
            Self::Symbol => "SYMBOL",
            Self::UserChar => "USER_CHAR",
        }
    }
}

impl From<NumberType1> for NumberType2 {
    fn from(value: NumberType1) -> Self {
        match value {
            NumberType1::Digit => Self::Digit,
            NumberType1::Circled => Self::Circled,
            NumberType1::RomanCapital => Self::RomanCapital,
            NumberType1::RomanSmall => Self::RomanSmall,
            NumberType1::LatinCapital => Self::LatinCapital,
            NumberType1::LatinSmall => Self::LatinSmall,
            NumberType1::CircledLatinCapital => Self::CircledLatinCapital,
            NumberType1::CircledLatinSmall => Self::CircledLatinSmall,
            NumberType1::HangulSyllable => Self::HangulSyllable,
            NumberType1::CircledHangulSyllable => Self::CircledHangulSyllable,
            NumberType1::HangulJamo => Self::HangulJamo,
            NumberType1::CircledHangulJamo => Self::CircledHangulJamo,
            NumberType1::HangulPhonetic => Self::HangulPhonetic,
            NumberType1::Ideograph => Self::Ideograph,
            NumberType1::CircledIdeograph => Self::CircledIdeograph,
        }
    }
}

impl std::str::FromStr for NumberType2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DIGIT" => Ok(Self::Digit),
            "CIRCLED_DIGIT" => Ok(Self::Circled),
            "ROMAN_CAPITAL" => Ok(Self::RomanCapital),
            "ROMAN_SMALL" => Ok(Self::RomanSmall),
            "LATIN_CAPITAL" => Ok(Self::LatinCapital),
            "LATIN_SMALL" => Ok(Self::LatinSmall),
            "CIRCLED_LATIN_CAPITAL" => Ok(Self::CircledLatinCapital),
            "CIRCLED_LATIN_SMALL" => Ok(Self::CircledLatinSmall),
            "HANGUL_SYLLABLE" => Ok(Self::HangulSyllable),
            "CIRCLED_HANGUL_SYLLABLE" => Ok(Self::CircledHangulSyllable),
            "HANGUL_JAMO" => Ok(Self::HangulJamo),
            "CIRCLED_HANGUL_JAMO" => Ok(Self::CircledHangulJamo),
            "HANGUL_PHONETIC" => Ok(Self::HangulPhonetic),
            "IDEOGRAPH" => Ok(Self::Ideograph),
            "CIRCLED_IDEOGRAPH" => Ok(Self::CircledIdeograph),
            "DECAGON_CIRCLE" => Ok(Self::DecagonCircle),
            "DECAGON_CIRCLE_HANJA" => Ok(Self::DecagonCircleHanja),
            "SYMBOL" => Ok(Self::Symbol),
            "USER_CHAR" => Ok(Self::UserChar),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 문단 머리 모양 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParagraphHeadingKind {
    /// "NONE"
    ///
    /// 없음
    None,
    /// "OUTLINE"
    ///
    /// 개요
    Outline,
    /// "NUMBER"
    ///
    /// 번호
    Number,
    /// "BULLET"
    ///
    /// 글머리표
    Bullet,
}

impl Hancom for ParagraphHeadingKind {
    //
}

impl Arbitrary for ParagraphHeadingKind {
    const NAME: &'static str = "$ParagraphHeadingKind";
}

impl Enumeration for ParagraphHeadingKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Outline => "OUTLINE",
            Self::Number => "NUMBER",
            Self::Bullet => "BULLET",
        }
    }
}

impl std::str::FromStr for ParagraphHeadingKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Self::None),
            "OUTLINE" => Ok(Self::Outline),
            "NUMBER" => Ok(Self::Number),
            "BULLET" => Ok(Self::Bullet),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 정렬 방식
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParagraphHorizontalAlignKind {
    /// "JUSTIFY"
    ///
    /// 양쪽 정렬
    Justify,
    /// "LEFT"
    ///
    /// 왼쪽 정렬
    Left,
    /// "RIGHT"
    ///
    /// 오른쪽 정렬
    Right,
    /// "CENTER"
    ///
    /// 가운데 정렬
    Center,
    /// "DISTRIBUTE"
    ///
    /// 배분 정렬
    Distribute,
    /// "DISTRIBUTE_SPACE"
    ///
    /// 나눔 정렬(공백에만 배분)
    DistributeSpace,
}

impl Hancom for ParagraphHorizontalAlignKind {
    //
}

impl Arbitrary for ParagraphHorizontalAlignKind {
    const NAME: &'static str = "$ParagraphHorizontalAlignKind";
}

impl Enumeration for ParagraphHorizontalAlignKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Justify => "JUSTIFY",
            Self::Left => "LEFT",
            Self::Right => "RIGHT",
            Self::Center => "CENTER",
            Self::Distribute => "DISTRIBUTE",
            Self::DistributeSpace => "DISTRIBUTE_SPACE",
        }
    }
}

impl std::str::FromStr for ParagraphHorizontalAlignKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JUSTIFY" => Ok(Self::Justify),
            "LEFT" => Ok(Self::Left),
            "RIGHT" => Ok(Self::Right),
            "CENTER" => Ok(Self::Center),
            "DISTRIBUTE" => Ok(Self::Distribute),
            "DISTRIBUTE_SPACE" => Ok(Self::DistributeSpace),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 세로 정렬
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParagraphVerticalAlignKind {
    /// "BASELINE"
    ///
    /// 글꼴 기준
    Baseline,
    /// "TOP"
    ///
    /// 위쪽
    Top,
    /// "CENTER"
    ///
    /// 가운데
    Center,
    /// "BOTTOM"
    ///
    /// 아래
    Bottom,
}

impl Hancom for ParagraphVerticalAlignKind {
    //
}

impl Arbitrary for ParagraphVerticalAlignKind {
    const NAME: &'static str = "$ParagraphVerticalAlignKind";
}

impl Enumeration for ParagraphVerticalAlignKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Baseline => "BASELINE",
            Self::Top => "TOP",
            Self::Center => "CENTER",
            Self::Bottom => "BOTTOM",
        }
    }
}

impl std::str::FromStr for ParagraphVerticalAlignKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BASELINE" => Ok(Self::Baseline),
            "TOP" => Ok(Self::Top),
            "CENTER" => Ok(Self::Center),
            "BOTTOM" => Ok(Self::Bottom),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// "RGBColorType"
///
/// "#RRGGBB" 형식의 문자열 또는 "none"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColorType(pub Option<(u8, u8, u8)>);

impl Hancom for RgbColorType {
    //
}

impl Core for RgbColorType {
    const NAME: &'static str = "RGBColorType";
}

impl std::str::FromStr for RgbColorType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "none" {
            return Ok(Self(None));
        }

        if let Some(stripped) = s.strip_prefix('#') {
            if stripped.len() == 6 {
                let r = u8::from_str_radix(&stripped[0..2], 16)?;
                let g = u8::from_str_radix(&stripped[2..4], 16)?;
                let b = u8::from_str_radix(&stripped[4..6], 16)?;
                return Ok(Self(Some((r, g, b))));
            }
        }

        invalid_variant!(Self::NAME, s)
    }
}

/// 그림자 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadowKind {
    /// "NONE"
    None,
    /// "DROP"
    Drop,
    /// "CONTINUOUS"
    Continuous,
}

impl Hancom for ShadowKind {
    //
}

impl Arbitrary for ShadowKind {
    const NAME: &'static str = "$ShadowKind";
}

impl Enumeration for ShadowKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Drop => "DROP",
            Self::Continuous => "CONTINUOUS",
        }
    }
}

impl std::str::FromStr for ShadowKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Self::None),
            "DROP" => Ok(Self::Drop),
            "CONTINUOUS" => Ok(Self::Continuous),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlashKind {
    /// "NONE"
    ///
    /// 없음
    None,
    /// "CENTER"
    ///
    /// 중심선 하나
    Center,
    /// "CENTER_BELOW"
    ///
    /// 중심선 + 중심선 아래의 사선
    CenterBelow,
    /// "CENTER_ABOVE"
    ///
    /// 중심선 + 중심선 위의 사선
    CenterAbove,
    /// "ALL"
    ///
    /// 중심선 + 중심선 아래의 사선 + 중심선 위의 사선
    All,
}

impl Hancom for SlashKind {
    //
}

impl Arbitrary for SlashKind {
    const NAME: &'static str = "$SlashKind";
}

impl Enumeration for SlashKind {
    fn enumeration(&self) -> &'static str {
        match self {
            SlashKind::None => "NONE",
            SlashKind::Center => "CENTER",
            SlashKind::CenterBelow => "CENTER_BELOW",
            SlashKind::CenterAbove => "CENTER_ABOVE",
            SlashKind::All => "ALL",
        }
    }
}

impl std::str::FromStr for SlashKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(SlashKind::None),
            "CENTER" => Ok(SlashKind::Center),
            "CENTER_BELOW" => Ok(SlashKind::CenterBelow),
            "CENTER_ABOVE" => Ok(SlashKind::CenterAbove),
            "ALL" => Ok(SlashKind::All),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 스타일 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleKind {
    /// "PARA"
    ///
    /// 문단 스타일
    Para,
    /// "CHAR"
    ///
    /// 글자 스타일
    Char,
}

impl Hancom for StyleKind {
    //
}

impl Arbitrary for StyleKind {
    const NAME: &'static str = "$StyleKind";
}

impl Enumeration for StyleKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Para => "PARA",
            Self::Char => "CHAR",
        }
    }
}

impl std::str::FromStr for StyleKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PARA" => Ok(Self::Para),
            "CHAR" => Ok(Self::Char),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 강조점 종류
#[derive(Debug)]
pub enum SymbolMark {
    /// "NONE"
    None,
    /// "DOT_ABOVE"
    DotAbove,
    /// "RING_ABOVE"
    RingAbove,
    /// "TILDE"
    Tilde,
    /// "CARON"
    Caron,
    /// "SIDE"
    Side,
    /// "COLON"
    Colon,
    /// "GRAVE_ACCENT"
    GraveAccent,
    /// "ACUTE_ACCENT"
    AcuteAccent,
    /// "CIRCUMFLEX"
    Circumflex,
    /// "MACRON"
    Macron,
    /// "HOOK_ABOVE"
    HookAbove,
    /// "DOT_BELOW"
    DotBelow,
}

impl Hancom for SymbolMark {
    //
}

impl Arbitrary for SymbolMark {
    const NAME: &'static str = "$SymbolMark";
}

impl Enumeration for SymbolMark {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::DotAbove => "DOT_ABOVE",
            Self::RingAbove => "RING_ABOVE",
            Self::Tilde => "TILDE",
            Self::Caron => "CARON",
            Self::Side => "SIDE",
            Self::Colon => "COLON",
            Self::GraveAccent => "GRAVE_ACCENT",
            Self::AcuteAccent => "ACUTE_ACCENT",
            Self::Circumflex => "CIRCUMFLEX",
            Self::Macron => "MACRON",
            Self::HookAbove => "HOOK_ABOVE",
            Self::DotBelow => "DOT_BELOW",
        }
    }
}

impl std::str::FromStr for SymbolMark {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Self::None),
            "DOT_ABOVE" => Ok(Self::DotAbove),
            "RING_ABOVE" => Ok(Self::RingAbove),
            "TILDE" => Ok(Self::Tilde),
            "CARON" => Ok(Self::Caron),
            "SIDE" => Ok(Self::Side),
            "COLON" => Ok(Self::Colon),
            "GRAVE_ACCENT" => Ok(Self::GraveAccent),
            "ACUTE_ACCENT" => Ok(Self::AcuteAccent),
            "CIRCUMFLEX" => Ok(Self::Circumflex),
            "MACRON" => Ok(Self::Macron),
            "HOOK_ABOVE" => Ok(Self::HookAbove),
            "DOT_BELOW" => Ok(Self::DotBelow),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 탭의 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabItemKind {
    /// "LEFT"
    Left,
    /// "RIGHT"
    Right,
    /// "CENTER"
    Center,
    /// "DECIMAL"
    Decimal,
}

impl Hancom for TabItemKind {
    //
}

impl Arbitrary for TabItemKind {
    const NAME: &'static str = "$TabItemKind";
}

impl Enumeration for TabItemKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Left => "LEFT",
            Self::Right => "RIGHT",
            Self::Center => "CENTER",
            Self::Decimal => "DECIMAL",
        }
    }
}

impl std::str::FromStr for TabItemKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LEFT" => Ok(Self::Left),
            "RIGHT" => Ok(Self::Right),
            "CENTER" => Ok(Self::Center),
            "DECIMAL" => Ok(Self::Decimal),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetProgram {
    /// "HWP201X"
    Hwp201X,
    /// "HWP200X"
    Hwp200X,
    /// "MS_WORD"
    MsWord,
}

impl Hancom for TargetProgram {
    //
}

impl Arbitrary for TargetProgram {
    const NAME: &'static str = "TargetProgram";
}

impl Enumeration for TargetProgram {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Hwp201X => "HWP201X",
            Self::Hwp200X => "HWP200X",
            Self::MsWord => "MS_WORD",
        }
    }
}

impl std::str::FromStr for TargetProgram {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HWP201X" => Ok(Self::Hwp201X),
            "HWP200X" => Ok(Self::Hwp200X),
            "MS_WORD" => Ok(Self::MsWord),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 텍스트 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirectionKind {
    /// "HORIZONTAL"
    Horizontal,
    /// "VERTICAL"
    Vertical,
    /// "VERTICALALL"
    ///
    /// 세로 영문 세움
    VerticalAll,
}

impl Hancom for TextDirectionKind {
    //
}

impl Arbitrary for TextDirectionKind {
    const NAME: &'static str = "$TextDirectionKind";
}

impl Enumeration for TextDirectionKind {
    fn enumeration(&self) -> &'static str {
        match self {
            TextDirectionKind::Horizontal => "HORIZONTAL",
            TextDirectionKind::Vertical => "VERTICAL",
            TextDirectionKind::VerticalAll => "VERTICALALL",
        }
    }
}

impl std::str::FromStr for TextDirectionKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HORIZONTAL" => Ok(Self::Horizontal),
            "VERTICAL" => Ok(Self::Vertical),
            "VERTICALALL" => Ok(Self::VerticalAll),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 수준별 본문과의 거리 단위 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextOffsetKind {
    /// "PERCENT"
    Percent,
    /// "HWPUNIT"
    HwpUnit,
}

impl Hancom for TextOffsetKind {
    //
}

impl Arbitrary for TextOffsetKind {
    const NAME: &'static str = "$TextOffsetKind";
}

impl Enumeration for TextOffsetKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Percent => "PERCENT",
            Self::HwpUnit => "HWPUNIT",
        }
    }
}

impl std::str::FromStr for TextOffsetKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PERCENT" => Ok(Self::Percent),
            "HWPUNIT" => Ok(Self::HwpUnit),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// TrackChangeType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackChangeKind {
    /// "UnKnown" | "UnKown"
    Unknown,
    /// "Insert"
    Insert,
    /// "Delete"
    Delete,
    /// "CharShape"
    CharShape,
    /// "ParaShape"
    ParagraphShape,
}

impl Hancom for TrackChangeKind {
    //
}

impl Core for TrackChangeKind {
    const NAME: &'static str = "TrackChangeType";
}

impl Enumeration for TrackChangeKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Unknown => "UnKnown",
            Self::Insert => "Insert",
            Self::Delete => "Delete",
            Self::CharShape => "CharShape",
            Self::ParagraphShape => "ParaShape",
        }
    }
}

impl std::str::FromStr for TrackChangeKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UnKnown" | "UnKown" => Ok(TrackChangeKind::Unknown),
            "Insert" => Ok(TrackChangeKind::Insert),
            "Delete" => Ok(TrackChangeKind::Delete),
            "CharShape" => Ok(TrackChangeKind::CharShape),
            "ParaShape" => Ok(TrackChangeKind::ParagraphShape),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 밑줄 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnderlineKind {
    /// "NONE"
    None,
    /// "BOTTOM"
    Bottom,
    /// "CENTER"
    Center,
    /// "TOP"
    Top,
}

impl Hancom for UnderlineKind {
    //
}

impl Arbitrary for UnderlineKind {
    const NAME: &'static str = "$UnderlineKind";
}

impl Enumeration for UnderlineKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Bottom => "BOTTOM",
            Self::Center => "CENTER",
            Self::Top => "TOP",
        }
    }
}

impl std::str::FromStr for UnderlineKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Self::None),
            "BOTTOM" => Ok(Self::Bottom),
            "CENTER" => Ok(Self::Center),
            "TOP" => Ok(Self::Top),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 구역 나눔으로 새 페이지가 생길 때 페이지 번호 적용 옵션
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageStartKind {
    /// "BOTH"
    Both,
    /// "EVEN"
    Even,
    /// "ODD"
    Odd,
}

impl Hancom for PageStartKind {
    //
}

impl Arbitrary for PageStartKind {
    const NAME: &'static str = "$PageStartKind";
}

impl Enumeration for PageStartKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Both => "BOTH",
            Self::Even => "EVEN",
            Self::Odd => "ODD",
        }
    }
}

impl std::str::FromStr for PageStartKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BOTH" => Ok(Self::Both),
            "EVEN" => Ok(Self::Even),
            "ODD" => Ok(Self::Odd),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityValue {
    /// "HIDE_FIRST"
    HideFirst,
    /// "SHOW_FIRST"
    ShowFirst,
    /// "SHOW_ALL"
    ShowAll,
}

impl Hancom for VisibilityValue {
    //
}

impl Arbitrary for VisibilityValue {
    const NAME: &'static str = "$VisibilityValue";
}

impl Enumeration for VisibilityValue {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::HideFirst => "HIDE_FIRST",
            Self::ShowFirst => "SHOW_FIRST",
            Self::ShowAll => "SHOW_ALL",
        }
    }
}

impl std::str::FromStr for VisibilityValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HIDE_FIRST" => Ok(Self::HideFirst),
            "SHOW_FIRST" => Ok(Self::ShowFirst),
            "SHOW_ALL" => Ok(Self::ShowAll),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 용지 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LandscapeKind {
    /// "WIDELY"
    Widely,
    /// "NARROWLY"
    Narrowly,
}

impl Hancom for LandscapeKind {
    //
}

impl Arbitrary for LandscapeKind {
    const NAME: &'static str = "$LandscapeKind";
}

impl Enumeration for LandscapeKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Widely => "WIDELY",
            Self::Narrowly => "NARROWLY",
        }
    }
}

impl std::str::FromStr for LandscapeKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WIDELY" => Ok(Self::Widely),
            "NARROWLY" => Ok(Self::Narrowly),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 제책 방법
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GutterKind {
    /// "LEFT_ONLY"
    LeftOnly,
    /// "LEFT_RIGHT"
    LeftRight,
    /// "TOP_BOTTOM"
    TopBottom,
}

impl Hancom for GutterKind {
    //
}

impl Arbitrary for GutterKind {
    const NAME: &'static str = "$GutterKind";
}

impl Enumeration for GutterKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::LeftOnly => "LEFT_ONLY",
            Self::LeftRight => "LEFT_RIGHT",
            Self::TopBottom => "TOP_BOTTOM",
        }
    }
}

impl std::str::FromStr for GutterKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LEFT_ONLY" => Ok(Self::LeftOnly),
            "LEFT_RIGHT" => Ok(Self::LeftRight),
            "TOP_BOTTOM" => Ok(Self::TopBottom),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 구분선 길이, 0(구분선 없음), -1 (5 cm), -2 (2 cm), -3 (단 크기의 1/3), -4 (단 크기), 그 외 (HWPUNIT 단위의 사용자 지정 길이)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteLineLength {
    /// "0"
    ///
    /// 구분선 없음
    None,
    /// "-1"
    ///
    /// 5 cm
    _5cm,
    /// "-2"
    ///
    /// 2 cm
    _2cm,
    /// "-3"
    ///
    /// 단 크기의 1/3
    OneThird,
    /// "-4"
    ///
    /// 단 크기
    Full,
    ///
    Custom(u32),
}

impl Hancom for NoteLineLength {
    //
}

impl Arbitrary for NoteLineLength {
    const NAME: &'static str = "$NoteLineLength";
}

impl std::str::FromStr for NoteLineLength {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::None),
            "-1" => Ok(Self::_5cm),
            "-2" => Ok(Self::_2cm),
            "-3" => Ok(Self::OneThird),
            "-4" => Ok(Self::Full),
            _ => Ok(Self::Custom(s.parse()?)),
        }
    }
}

/// 번호 매기기 형식
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberingKind {
    /// "CONTINUOUS"
    ///
    /// 앞 구역에 이어서
    Continuous,
    /// "ON_SECTION"
    ///
    /// 현재 구역부터 새로 시작
    OnSection,
    /// "ON_PAGE"
    ///
    /// 쪽마다 새로 시작. 각주 전용.
    OnPage,
}

impl Hancom for NumberingKind {
    //
}

impl Arbitrary for NumberingKind {
    const NAME: &'static str = "$NumberingKind";
}

impl Enumeration for NumberingKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Continuous => "CONTINUOUS",
            Self::OnSection => "ON_SECTION",
            Self::OnPage => "ON_PAGE",
        }
    }
}

impl std::str::FromStr for NumberingKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONTINUOUS" => Ok(Self::Continuous),
            "ON_SECTION" => Ok(Self::OnSection),
            "ON_PAGE" => Ok(Self::OnPage),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

#[macro_export]
macro_rules! boolean {
    ($value:expr, $attribute:expr) => {
        match $value {
            "true" | "1" => true.into(),
            "false" | "0" => false.into(),
            _ => unknown!("Invalid value for <{}>", $attribute),
        }
    };
}
