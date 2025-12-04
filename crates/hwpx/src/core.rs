use crate::{
    Core, Enumeration, Hancom,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdRef(pub String);

impl std::str::FromStr for IdRef {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

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

#[macro_export]
macro_rules! boolean {
    ($value:expr, $attribute:expr) => {
        match $value {
            "true" | "1" | "yes" => true.into(),
            "false" | "0" | "no" => false.into(),
            _ => unknown!("Invalid value for <{}>", $attribute),
        }
    };
}
