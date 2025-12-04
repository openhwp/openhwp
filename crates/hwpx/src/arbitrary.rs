use crate::{Arbitrary, Enumeration, Hancom, error::Error};

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
pub enum FootNoteNumberingKind {
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

impl Hancom for FootNoteNumberingKind {
    //
}

impl Arbitrary for FootNoteNumberingKind {
    const NAME: &'static str = "$FootNoteNumberingKind";
}

impl Enumeration for FootNoteNumberingKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Continuous => "CONTINUOUS",
            Self::OnSection => "ON_SECTION",
            Self::OnPage => "ON_PAGE",
        }
    }
}

impl std::str::FromStr for FootNoteNumberingKind {
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

/// 한 페이지 내에서 각주를 다단에 어떻게 위치시킬지를 표시한다
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FootNotePlaceKind {
    /// "EACH_COLUMN"
    ///
    /// 각 단마다 따로 배열
    EachColumn,
    /// "MERGED_COLUMN"
    ///
    /// 통단으로 배열
    MergedColumn,
    /// "RIGHT_MOST_COLUMN"
    ///
    /// 가장 오른쪽 단에 배열
    RightMostColumn,
}

impl Hancom for FootNotePlaceKind {
    //
}

impl Arbitrary for FootNotePlaceKind {
    const NAME: &'static str = "$FootNotePlaceKind";
}

impl Enumeration for FootNotePlaceKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::EachColumn => "EACH_COLUMN",
            Self::MergedColumn => "MERGED_COLUMN",
            Self::RightMostColumn => "RIGHT_MOST_COLUMN",
        }
    }
}

impl std::str::FromStr for FootNotePlaceKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EACH_COLUMN" => Ok(Self::EachColumn),
            "MERGED_COLUMN" => Ok(Self::MergedColumn),
            "RIGHT_MOST_COLUMN" => Ok(Self::RightMostColumn),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 번호 매기기 형식
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndNoteNumberingKind {
    /// "CONTINUOUS"
    ///
    /// 앞 구역에 이어서
    Continuous,
    /// "ON_SECTION"
    ///
    /// 현재 구역부터 새로 시작
    OnSection,
}

impl Hancom for EndNoteNumberingKind {
    //
}

impl Arbitrary for EndNoteNumberingKind {
    const NAME: &'static str = "$EndNoteNumberingKind";
}

impl Enumeration for EndNoteNumberingKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::Continuous => "CONTINUOUS",
            Self::OnSection => "ON_SECTION",
        }
    }
}

impl std::str::FromStr for EndNoteNumberingKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONTINUOUS" => Ok(Self::Continuous),
            "ON_SECTION" => Ok(Self::OnSection),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}

/// 한 페이지 내에서 미주를 다단에 어떻게 위치시킬지를 표시한다
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndNotePlaceKind {
    /// "END_OF_DOCUMENT"
    ///
    /// 문서의 마지막
    EndOfDocument,
    /// "END_OF_SECTION"
    ///
    /// 구역의 마지막
    EndOfSection,
}

impl Hancom for EndNotePlaceKind {
    //
}

impl Arbitrary for EndNotePlaceKind {
    const NAME: &'static str = "$EndNotePlaceKind";
}

impl Enumeration for EndNotePlaceKind {
    fn enumeration(&self) -> &'static str {
        match self {
            Self::EndOfDocument => "END_OF_DOCUMENT",
            Self::EndOfSection => "END_OF_SECTION",
        }
    }
}

impl std::str::FromStr for EndNotePlaceKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "END_OF_DOCUMENT" => Ok(Self::EndOfDocument),
            "END_OF_SECTION" => Ok(Self::EndOfSection),
            _ => invalid_variant!(Self::NAME, s),
        }
    }
}
