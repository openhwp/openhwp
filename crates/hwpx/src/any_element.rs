use crate::error::Error;
use quick_xml::{NsReader, events::Event, name::ResolveResult};
use std::io::{BufReader, Cursor};

/// Represents any XML element with its namespace, name, attributes, and children.
#[derive(Debug)]
pub struct AnyElement {
    pub name: ElementName,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<AnyElement>,
    pub text: Option<String>,
}

/// Represents the names of XML elements with their associated namespaces.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementName {
    // "http://www.hancom.co.kr/hwpml/2011/app"
    /// `HWPApplicationSetting`
    HANCOM__APP__HWP_APPLICATION_SETTING,
    /// `CaretPosition`
    HANCOM__APP__CARET_POSITION,
    // "http://www.hancom.co.kr/hwpml/2011/version"
    /// `HCFVersion`
    HANCOM__VERSION__HCF_VERSION,
    // "urn:oasis:names:tc:opendocument:xmlns:config:1.0"
    /// `config-item-set`
    OPENDOCUMENT__CONFIG__CONFIG_ITEM_SET,
    /// `config-item`
    OPENDOCUMENT__CONFIG__CONFIG_ITEM,
    // "http://www.idpf.org/2007/opf/"
    /// `package`
    OPENDOCUMENT__OPF__PACKAGE,
    /// `metadata`
    OPENDOCUMENT__OPF__METADATA,
    /// `manifest`
    OPENDOCUMENT__OPF__MANIFEST,
    /// `spine`
    OPENDOCUMENT__OPF__SPINE,
    /// `title`
    OPENDOCUMENT__OPF__TITLE,
    /// `language`
    OPENDOCUMENT__OPF__LANGUAGE,
    /// `meta`
    OPENDOCUMENT__OPF__META,
    /// `item`
    OPENDOCUMENT__OPF__ITEM,
    /// `itemref`
    OPENDOCUMENT__OPF__ITEM_REFERENCE,
    // "http://www.hancom.co.kr/hwpml/2011/core"
    /// `fillBrush`
    HANCOM__CORE__FILL_BRUSH,
    /// `winBrush`
    HANCOM__CORE__WIN_BRUSH,
    /// `gradation`
    HANCOM__CORE__GRADATION,
    /// `imgBrush`
    HANCOM__CORE__IMAGE_BRUSH,
    /// `img`
    HANCOM__CORE__IMAGE,
    /// `intent`
    HANCOM__CORE__INDENT,
    /// `left`
    HANCOM__CORE__LEFT,
    /// `right`
    HANCOM__CORE__RIGHT,
    /// `previous`
    HANCOM__CORE__PREVIOUS,
    /// `next`
    HANCOM__CORE__NEXT,
    // "http://www.hancom.co.kr/hwpml/2011/head"
    /// `head`
    HANCOM__HEAD__HEAD,
    /// `beginNum`
    HANCOM__HEAD__BEGIN_NUMBER,
    /// `refList`
    HANCOM__HEAD__REFERENCES,
    /// `forbiddenWordList`
    HANCOM__HEAD__FORBIDDEN_WORDS,
    /// `trackchangeConfig`
    HANCOM__HEAD__TRACK_CHANGE_CONFIG,
    /// `docOption`
    HANCOM__HEAD__DOCUMENT_OPTION,
    /// `metaTag`
    HANCOM__HEAD__META_TAG,
    /// `fontfaces`
    HANCOM__HEAD__FONT_FACES,
    /// `borderFills`
    HANCOM__HEAD__BORDER_FILLS,
    /// `charProperties`
    HANCOM__HEAD__CHARACTER_PROPERTIES,
    /// `tabProperties`
    HANCOM__HEAD__TAB_PROPERTIES,
    /// `numberings`
    HANCOM__HEAD__NUMBERINGS,
    /// `bullets`
    HANCOM__HEAD__BULLETS,
    /// `paraProperties`
    HANCOM__HEAD__PARAGRAPH_PROPERTIES,
    /// `paraPr`
    HANCOM__HEAD__PARAGRAPH_PROPERTY,
    /// `styles`
    HANCOM__HEAD__STYLES,
    /// `memoProperties`
    HANCOM__HEAD__MEMO_PROPERTIES,
    /// `trackChanges`
    HANCOM__HEAD__TRACK_CHANGES,
    /// `trackChangeAuthors`
    HANCOM__HEAD__TRACK_CHANGE_AUTHORS,
    /// `charPr`
    HANCOM__HEAD__CHARACTER_PROPERTY,
    /// `fontface`
    HANCOM__HEAD__FONT_FACE,
    /// `font`
    HANCOM__HEAD__FONT,
    /// `substFont`
    HANCOM__HEAD__SUBSET_FONT,
    /// `typeInfo`
    HANCOM__HEAD__TYPE_INFO,
    /// `borderFill`
    HANCOM__HEAD__BORDER_FILL,
    /// `slash`
    HANCOM__HEAD__SLASH,
    /// `backSlash`
    HANCOM__HEAD__BACK_SLASH,
    /// `leftBorder`
    HANCOM__HEAD__LEFT_BORDER,
    /// `rightBorder`
    HANCOM__HEAD__RIGHT_BORDER,
    /// `topBorder`
    HANCOM__HEAD__TOP_BORDER,
    /// `bottomBorder`
    HANCOM__HEAD__BOTTOM_BORDER,
    /// `diagonal`
    HANCOM__HEAD__DIAGONAL,
    /// `color`
    HANCOM__HEAD__COLOR,
    /// `charShape`
    HANCOM__HEAD__CHARACTER_SHAPE,
    /// `fontRef`
    HANCOM__HEAD__FONT_REFERENCE,
    /// `ratio`
    HANCOM__HEAD__RATIO,
    /// `spacing`
    HANCOM__HEAD__SPACING,
    /// `relSz`
    HANCOM__HEAD__RELATIVE_SIZE,
    /// `offset`
    HANCOM__HEAD__OFFSET,
    /// `italic`
    HANCOM__HEAD__ITALIC,
    /// `bold`
    HANCOM__HEAD__BOLD,
    /// `underline`
    HANCOM__HEAD__UNDERLINE,
    /// `strikeout`
    HANCOM__HEAD__STRIKEOUT,
    /// `outline`
    HANCOM__HEAD__OUTLINE,
    /// `shadow`
    HANCOM__HEAD__SHADOW,
    /// `tabDef`
    HANCOM__HEAD__TAB_DEFINITION,
    /// `tabItem`
    HANCOM__HEAD__TAB_ITEM,
    /// `numbering`
    HANCOM__HEAD__NUMBERING,
    /// `paraShape`
    HANCOM__HEAD__PARAGRAPH_SHAPE,
    /// `align`
    HANCOM__HEAD__ALIGN,
    /// `heading`
    HANCOM__HEAD__HEADING,
    /// `breakSetting`
    HANCOM__HEAD__BREAK_SETTING,
    /// `margin`
    HANCOM__HEAD__MARGIN,
    /// `lineSpacing`
    HANCOM__HEAD__LINE_SPACING,
    /// `border`
    HANCOM__HEAD__BORDER,
    /// `autoSpacing`
    HANCOM__HEAD__AUTO_SPACING,
    /// `paraHead`
    HANCOM__HEAD__PARAGRAPH_HEAD,
    /// `bullet`
    HANCOM__HEAD__BULLET,
    /// `style`
    HANCOM__HEAD__STYLE,
    /// `forbiddenWord`
    HANCOM__HEAD__FORBIDDEN_WORD,
    /// `memoPr`
    HANCOM__HEAD__MEMO_PROPERTY,
    /// `trackChange`
    HANCOM__HEAD__TRACK_CHANGE,
    /// `trackChangeAuthor`
    HANCOM__HEAD__TRACK_CHANGE_AUTHOR,
    /// `compatibleDocument`
    HANCOM__HEAD__COMPATIBLE_DOCUMENT,
    /// `layoutCompatibility`
    HANCOM__HEAD__LAYOUT_COMPATIBILITY,
    /// `applyFontWeightToBold`
    HANCOM__HEAD__APPLY_FONT_WEIGHT_TO_BOLD,
    /// `useInnerUnderline`
    HANCOM__HEAD__USE_INNER_UNDERLINE,
    /// `fixedUnderlineWidth`
    HANCOM__HEAD__FIXED_UNDERLINE_WIDTH,
    /// `doNotApplyStrikeoutWithUnderline`
    HANCOM__HEAD__DO_NOT_APPLY_STRIKEOUT_WITH_UNDERLINE,
    /// `useLowercaseStrikeout`
    HANCOM__HEAD__USE_LOWERCASE_STRIKEOUT,
    /// `extendLineheightToOffset`
    HANCOM__HEAD__EXTEND_LINEHEIGHT_TO_OFFSET,
    /// `applyFontspaceToLatin`
    HANCOM__HEAD__APPLY_FONTSPACE_TO_LATIN,
    /// `treatQuotationAsLatin`
    HANCOM__HEAD__TREAT_QUOTATION_AS_LATIN,
    /// `doNotApplyDiacSymMarkOfNoneAndSix`
    HANCOM__HEAD__DO_NOT_APPLY_DIAC_SYM_MARK_OF_NONE_AND_SIX,
    /// `doNotAlignWhitespaceOnRight`
    HANCOM__HEAD__DO_NOT_ALIGN_WHITESPACE_ON_RIGHT,
    /// `doNotAdjustWordInJustify`
    HANCOM__HEAD__DO_NOT_ADJUST_WORD_IN_JUSTIFY,
    /// `baseCharUnitOnEAsian`
    HANCOM__HEAD__BASE_CHAR_UNIT_ON_E_ASIAN,
    /// `baseCharUnitOfIndentOnFirstChar`
    HANCOM__HEAD__BASE_CHAR_UNIT_OF_INDENT_ON_FIRST_CHAR,
    /// `adjustLineheightToFont`
    HANCOM__HEAD__ADJUST_LINEHEIGHT_TO_FONT,
    /// `adjustBaseInlineFixedLinespacing`
    HANCOM__HEAD__ADJUST_BASE_INLINE_FIXED_LINESPACING,
    /// `applyPrevspacingBeneathObject`
    HANCOM__HEAD__APPLY_PREVSPACING_BENEATH_OBJECT,
    /// `applyNextspacingOfLastPara`
    HANCOM__HEAD__APPLY_NEXTSPACING_OF_LAST_PARA,
    /// `applyAtLeastToPercent100Pct`
    HANCOM__HEAD__APPLY_AT_LEAST_TO_PERCENT100_PCT,
    /// `doNotApplyAutoSpaceEAsianEng`
    HANCOM__HEAD__DO_NOT_APPLY_AUTO_SPACE_E_ASIAN_ENG,
    /// `doNotApplyAutoSpaceEAsianNum`
    HANCOM__HEAD__DO_NOT_APPLY_AUTO_SPACE_E_ASIAN_NUM,
    /// `adjustParaBorderfillToSpacing`
    HANCOM__HEAD__ADJUST_PARA_BORDERFILL_TO_SPACING,
    /// `connectParaBorderfillOfEqualBorder`
    HANCOM__HEAD__CONNECT_PARA_BORDERFILL_OF_EQUAL_BORDER,
    /// `adjustParaBorderOffsetWithBorder`
    HANCOM__HEAD__ADJUST_PARA_BORDER_OFFSET_WITH_BORDER,
    /// `extendLineheightToParaBorderOffset`
    HANCOM__HEAD__EXTEND_LINEHEIGHT_TO_PARA_BORDER_OFFSET,
    /// `applyParaBorderToOutside`
    HANCOM__HEAD__APPLY_PARA_BORDER_TO_OUTSIDE,
    /// `applyMinColumnWidthTo1mm`
    HANCOM__HEAD__APPLY_MIN_COLUMN_WIDTH_TO1MM,
    /// `applyTabPosBasedOnSegment`
    HANCOM__HEAD__APPLY_TAB_POS_BASED_ON_SEGMENT,
    /// `breakTabOverline`
    HANCOM__HEAD__BREAK_TAB_OVERLINE,
    /// `adjustVertPosOfLine`
    HANCOM__HEAD__ADJUST_VERT_POS_OF_LINE,
    /// `doNotApplyWhiteSpaceHeight`
    HANCOM__HEAD__DO_NOT_APPLY_WHITE_SPACE_HEIGHT,
    /// `doNotAlignLastPeriod`
    HANCOM__HEAD__DO_NOT_ALIGN_LAST_PERIOD,
    /// `doNotAlignLastForbidden`
    HANCOM__HEAD__DO_NOT_ALIGN_LAST_FORBIDDEN,
    /// `baseLineSpacingOnLineGrid`
    HANCOM__HEAD__BASE_LINE_SPACING_ON_LINE_GRID,
    /// `applyCharSpacingToCharGrid`
    HANCOM__HEAD__APPLY_CHAR_SPACING_TO_CHAR_GRID,
    /// `doNotApplyGridInHeaderFooter`
    HANCOM__HEAD__DO_NOT_APPLY_GRID_IN_HEADER_FOOTER,
    /// `applyExtendHeaderFooterEachSection`
    HANCOM__HEAD__APPLY_EXTEND_HEADER_FOOTER_EACH_SECTION,
    /// `doNotApplyHeaderFooterAtNoSpace`
    HANCOM__HEAD__DO_NOT_APPLY_HEADER_FOOTER_AT_NO_SPACE,
    /// `doNotApplyColSeparatorAtNoGap`
    HANCOM__HEAD__DO_NOT_APPLY_COL_SEPARATOR_AT_NO_GAP,
    /// `doNotApplyLinegridAtNoLinespacing`
    HANCOM__HEAD__DO_NOT_APPLY_LINEGRID_AT_NO_LINESPACING,
    /// `doNotApplyImageEffect`
    HANCOM__HEAD__DO_NOT_APPLY_IMAGE_EFFECT,
    /// `doNotApplyShapeComment`
    HANCOM__HEAD__DO_NOT_APPLY_SHAPE_COMMENT,
    /// `doNotAdjustEmptyAnchorLine`
    HANCOM__HEAD__DO_NOT_ADJUST_EMPTY_ANCHOR_LINE,
    /// `overlapBothAllowOverlap`
    HANCOM__HEAD__OVERLAP_BOTH_ALLOW_OVERLAP,
    /// `doNotApplyVertOffsetOfForward`
    HANCOM__HEAD__DO_NOT_APPLY_VERT_OFFSET_OF_FORWARD,
    /// `extendVertLimitToPageMargins`
    HANCOM__HEAD__EXTEND_VERT_LIMIT_TO_PAGE_MARGINS,
    /// `doNotHoldAnchorOfTable`
    HANCOM__HEAD__DO_NOT_HOLD_ANCHOR_OF_TABLE,
    /// `doNotFormattingAtBeneathAnchor`
    HANCOM__HEAD__DO_NOT_FORMATTING_AT_BENEATH_ANCHOR,
    /// `adjustBaselineOfObjectToBottom`
    HANCOM__HEAD__ADJUST_BASELINE_OF_OBJECT_TO_BOTTOM,
    /// `doNotApplyExtensionCharCompose`
    HANCOM__HEAD__DO_NOT_APPLY_EXTENSION_CHAR_COMPOSE,
    /// `linkInfo`
    HANCOM__HEAD__LINK_INFO,
    /// `licenseMark`
    HANCOM__HEAD__LICENSE_MARK,
    /// `trackChangeEncryption`
    HANCOM__HEAD__TRACK_CHANGE_ENCRYPTION,
    /// `hash`
    HANCOM__HEAD__HASH,
    /// `derivationKey`
    HANCOM__HEAD__DERIVATION_KEY,
    /// `emboss`
    HANCOM__HEAD__EMBOSS,
    /// `engrave`
    HANCOM__HEAD__ENGRAVE,
    /// `superscript`
    HANCOM__HEAD__SUPERSCRIPT,
    /// `subscript`
    HANCOM__HEAD__SUBSCRIPT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Namespace {
    /// "http://www.hancom.co.kr/hwpml/2011/app"
    HancomApp,
    /// "http://www.hancom.co.kr/hwpml/2011/core"
    HancomCore,
    /// "http://www.hancom.co.kr/hwpml/2011/head"
    HancomHead,
    /// "http://www.hancom.co.kr/hwpml/2011/history"
    HancomHistory,
    /// "http://www.hancom.co.kr/schema/2011/hpf"
    HancomHpf,
    /// "http://www.hancom.co.kr/hwpml/2011/master-page"
    HancomMasterPage,
    /// "http://www.hancom.co.kr/hwpml/2016/ooxmlchart"
    HancomOoxmlChart,
    /// "http://www.hancom.co.kr/hwpml/2011/paragraph"
    HancomParagraph2011,
    /// "http://www.hancom.co.kr/hwpml/2016/paragraph"
    HancomParagraph2016,
    /// "http://www.hancom.co.kr/hwpml/2011/section"
    HancomSection,
    /// "http://www.hancom.co.kr/hwpml/2011/version"
    HancomVersion,
    /// "urn:oasis:names:tc:opendocument:xmlns:config:1.0"
    OdfConfig,
    /// "urn:oasis:names:tc:opendocument:xmlns:container"
    OdfContainer,
    /// "urn:oasis:names:tc:opendocument:xmlns:manifest:1.0"
    OdfManifest,
    /// "http://purl.org/dc/elements/1.1/"
    DoubleCore,
    /// "http://www.idpf.org/2007/ops"
    OpenDocumentOps,
    /// "http://www.idpf.org/2007/opf/"
    OpenDocumentOpf,
}

macro_rules! decode {
    ($value:expr) => {
        String::from_utf8_lossy(&$value.into_inner()).into_owned()
    };
}

impl AnyElement {
    pub fn from_bytes(bytes: &[u8]) -> Result<AnyElement, Error> {
        let mut reader = NsReader::from_reader(BufReader::new(Cursor::new(bytes)));

        let mut buf = vec![];
        let mut stack: Vec<AnyElement> = vec![];
        let mut in_start_tag = false;

        loop {
            match reader.read_resolved_event_into(&mut buf)? {
                (namespace, Event::Start(event)) => {
                    in_start_tag = true;
                    stack.push((namespace, event).try_into()?);
                }
                (namespace, Event::Empty(event)) => {
                    in_start_tag = false;
                    match stack.last_mut() {
                        Some(parent) => parent.children.push((namespace, event).try_into()?),
                        None => return Ok((namespace, event).try_into()?),
                    };
                }
                (_, Event::End(_)) => {
                    in_start_tag = false;
                    match (stack.pop(), stack.last_mut()) {
                        (Some(completed), Some(parent)) => {
                            parent.children.push(completed);
                            parent.text = None;
                        }
                        (Some(completed), None) => return Ok(completed),
                        (None, _) => unknown!("End event with empty stack"),
                    };
                }
                (_, Event::Text(event)) if in_start_tag => {
                    if let Some(current) = stack.last_mut() {
                        current.text = Some(decode!(event));
                    }
                }
                (_, Event::Text(_)) => {}
                (_, Event::Decl(_)) => {}
                (_, event) => unknown!("Unexpected event: {:?}", event),
            }
            buf.clear();
        }
    }

    pub fn expect(&self, expected: ElementName) -> Result<(), Error> {
        if self.name == expected {
            Ok(())
        } else {
            Err(Error::UnexpectedElement {
                expected,
                found: self.name,
            })
        }
    }
}

impl<'a> TryFrom<(ResolveResult<'a>, quick_xml::events::BytesStart<'a>)> for AnyElement {
    type Error = Error;

    fn try_from(
        (namespace, event): (ResolveResult<'a>, quick_xml::events::BytesStart<'a>),
    ) -> Result<Self, Self::Error> {
        let name = ElementName::from_event(namespace, &event)?;

        let mut attributes = vec![];
        for attribute in event.attributes() {
            let attribute = attribute?;
            let (local_name, namespace) = attribute.key.decompose();
            if namespace.is_some() {
                continue;
            }
            let name = decode!(local_name);
            let value = attribute.unescape_value()?.into_owned();
            attributes.push((name, value));
        }

        Ok(Self {
            name,
            attributes,
            children: vec![],
            text: None,
        })
    }
}

impl ElementName {
    fn from_event<'a>(
        namespace: ResolveResult<'a>,
        event: &quick_xml::events::BytesStart<'a>,
    ) -> Result<Self, Error> {
        let namespace = match namespace {
            ResolveResult::Bound(p) => match Namespace::from_bytes(p.into_inner()) {
                Some(ns) => ns,
                None => unknown!("Unknown namespace in element name: {}", decode!(p)),
            },
            _ => unknown!("Unbound namespace in element name"),
        };
        let local_name = event.local_name();

        macro_rules! element_names {
            (
                $(
                    ($ns:ident, $local:expr) => $variant:ident,
                )+
            ) => {
                match (namespace, local_name.into_inner()) {
                    $(
                        (Namespace::$ns, $local) => Ok(ElementName::$variant),
                    )+
                    _ => unknown!(
                        "Unknown element name: {{{:?}}}{}",
                        namespace,
                        decode!(local_name)
                    ),
                }
            };
        }

        element_names! {
            (HancomApp, b"HWPApplicationSetting") => HANCOM__APP__HWP_APPLICATION_SETTING,
            (HancomApp, b"CaretPosition") => HANCOM__APP__CARET_POSITION,
            (HancomVersion, b"HCFVersion") => HANCOM__VERSION__HCF_VERSION,
            (OdfConfig, b"config-item-set") => OPENDOCUMENT__CONFIG__CONFIG_ITEM_SET,
            (OdfConfig, b"config-item") => OPENDOCUMENT__CONFIG__CONFIG_ITEM,
            (OpenDocumentOpf, b"package") => OPENDOCUMENT__OPF__PACKAGE,
            (OpenDocumentOpf, b"metadata") => OPENDOCUMENT__OPF__METADATA,
            (OpenDocumentOpf, b"manifest") => OPENDOCUMENT__OPF__MANIFEST,
            (OpenDocumentOpf, b"spine") => OPENDOCUMENT__OPF__SPINE,
            (OpenDocumentOpf, b"title") => OPENDOCUMENT__OPF__TITLE,
            (OpenDocumentOpf, b"language") => OPENDOCUMENT__OPF__LANGUAGE,
            (OpenDocumentOpf, b"meta") => OPENDOCUMENT__OPF__META,
            (OpenDocumentOpf, b"item") => OPENDOCUMENT__OPF__ITEM,
            (OpenDocumentOpf, b"itemref") => OPENDOCUMENT__OPF__ITEM_REFERENCE,
            (HancomCore, b"fillBrush") => HANCOM__CORE__FILL_BRUSH,
            (HancomCore, b"winBrush") => HANCOM__CORE__WIN_BRUSH,
            (HancomCore, b"gradation") => HANCOM__CORE__GRADATION,
            (HancomCore, b"imgBrush") => HANCOM__CORE__IMAGE_BRUSH,
            (HancomCore, b"img") => HANCOM__CORE__IMAGE,
            (HancomCore, b"intent") => HANCOM__CORE__INDENT,
            (HancomCore, b"left") => HANCOM__CORE__LEFT,
            (HancomCore, b"right") => HANCOM__CORE__RIGHT,
            (HancomCore, b"prev") => HANCOM__CORE__PREVIOUS,
            (HancomCore, b"next") => HANCOM__CORE__NEXT,
            (HancomHead, b"head") => HANCOM__HEAD__HEAD,
            (HancomHead, b"beginNum") => HANCOM__HEAD__BEGIN_NUMBER,
            (HancomHead, b"refList") => HANCOM__HEAD__REFERENCES,
            (HancomHead, b"forbiddenWordList") => HANCOM__HEAD__FORBIDDEN_WORDS,
            (HancomHead, b"trackchangeConfig") => HANCOM__HEAD__TRACK_CHANGE_CONFIG,
            (HancomHead, b"docOption") => HANCOM__HEAD__DOCUMENT_OPTION,
            (HancomHead, b"metaTag") => HANCOM__HEAD__META_TAG,
            (HancomHead, b"fontfaces") => HANCOM__HEAD__FONT_FACES,
            (HancomHead, b"borderFills") => HANCOM__HEAD__BORDER_FILLS,
            (HancomHead, b"charProperties") => HANCOM__HEAD__CHARACTER_PROPERTIES,
            (HancomHead, b"tabProperties") => HANCOM__HEAD__TAB_PROPERTIES,
            (HancomHead, b"numberings") => HANCOM__HEAD__NUMBERINGS,
            (HancomHead, b"bullets") => HANCOM__HEAD__BULLETS,
            (HancomHead, b"paraProperties") => HANCOM__HEAD__PARAGRAPH_PROPERTIES,
            (HancomHead, b"paraPr") => HANCOM__HEAD__PARAGRAPH_PROPERTY,
            (HancomHead, b"styles") => HANCOM__HEAD__STYLES,
            (HancomHead, b"memoProperties") => HANCOM__HEAD__MEMO_PROPERTIES,
            (HancomHead, b"trackChanges") => HANCOM__HEAD__TRACK_CHANGES,
            (HancomHead, b"trackChangeAuthors") => HANCOM__HEAD__TRACK_CHANGE_AUTHORS,
            (HancomHead, b"charPr") => HANCOM__HEAD__CHARACTER_PROPERTY,
            (HancomHead, b"fontface") => HANCOM__HEAD__FONT_FACE,
            (HancomHead, b"font") => HANCOM__HEAD__FONT,
            (HancomHead, b"substFont") => HANCOM__HEAD__SUBSET_FONT,
            (HancomHead, b"typeInfo") => HANCOM__HEAD__TYPE_INFO,
            (HancomHead, b"borderFill") => HANCOM__HEAD__BORDER_FILL,
            (HancomHead, b"slash") => HANCOM__HEAD__SLASH,
            (HancomHead, b"backSlash") => HANCOM__HEAD__BACK_SLASH,
            (HancomHead, b"leftBorder") => HANCOM__HEAD__LEFT_BORDER,
            (HancomHead, b"rightBorder") => HANCOM__HEAD__RIGHT_BORDER,
            (HancomHead, b"topBorder") => HANCOM__HEAD__TOP_BORDER,
            (HancomHead, b"bottomBorder") => HANCOM__HEAD__BOTTOM_BORDER,
            (HancomHead, b"diagonal") => HANCOM__HEAD__DIAGONAL,
            (HancomHead, b"color") => HANCOM__HEAD__COLOR,
            (HancomHead, b"charShape") => HANCOM__HEAD__CHARACTER_SHAPE,
            (HancomHead, b"fontRef") => HANCOM__HEAD__FONT_REFERENCE,
            (HancomHead, b"ratio") => HANCOM__HEAD__RATIO,
            (HancomHead, b"spacing") => HANCOM__HEAD__SPACING,
            (HancomHead, b"relSz") => HANCOM__HEAD__RELATIVE_SIZE,
            (HancomHead, b"offset") => HANCOM__HEAD__OFFSET,
            (HancomHead, b"italic") => HANCOM__HEAD__ITALIC,
            (HancomHead, b"bold") => HANCOM__HEAD__BOLD,
            (HancomHead, b"underline") => HANCOM__HEAD__UNDERLINE,
            (HancomHead, b"strikeout") => HANCOM__HEAD__STRIKEOUT,
            (HancomHead, b"outline") => HANCOM__HEAD__OUTLINE,
            (HancomHead, b"shadow") => HANCOM__HEAD__SHADOW,
            (HancomHead, b"tabDef") => HANCOM__HEAD__TAB_DEFINITION,
            (HancomHead, b"tabItem") => HANCOM__HEAD__TAB_ITEM,
            (HancomHead, b"numbering") => HANCOM__HEAD__NUMBERING,
            (HancomHead, b"paraShape") => HANCOM__HEAD__PARAGRAPH_SHAPE,
            (HancomHead, b"align") => HANCOM__HEAD__ALIGN,
            (HancomHead, b"heading") => HANCOM__HEAD__HEADING,
            (HancomHead, b"breakSetting") => HANCOM__HEAD__BREAK_SETTING,
            (HancomHead, b"margin") => HANCOM__HEAD__MARGIN,
            (HancomHead, b"lineSpacing") => HANCOM__HEAD__LINE_SPACING,
            (HancomHead, b"border") => HANCOM__HEAD__BORDER,
            (HancomHead, b"autoSpacing") => HANCOM__HEAD__AUTO_SPACING,
            (HancomHead, b"paraHead") => HANCOM__HEAD__PARAGRAPH_HEAD,
            (HancomHead, b"bullet") => HANCOM__HEAD__BULLET,
            (HancomHead, b"style") => HANCOM__HEAD__STYLE,
            (HancomHead, b"forbiddenWord") => HANCOM__HEAD__FORBIDDEN_WORD,
            (HancomHead, b"memoPr") => HANCOM__HEAD__MEMO_PROPERTY,
            (HancomHead, b"trackChange") => HANCOM__HEAD__TRACK_CHANGE,
            (HancomHead, b"trackChangeAuthor") => HANCOM__HEAD__TRACK_CHANGE_AUTHOR,
            (HancomHead, b"compatibleDocument") => HANCOM__HEAD__COMPATIBLE_DOCUMENT,
            (HancomHead, b"layoutCompatibility") => HANCOM__HEAD__LAYOUT_COMPATIBILITY,
            (HancomHead, b"applyFontWeightToBold") => HANCOM__HEAD__APPLY_FONT_WEIGHT_TO_BOLD,
            (HancomHead, b"useInnerUnderline") => HANCOM__HEAD__USE_INNER_UNDERLINE,
            (HancomHead, b"fixedUnderlineWidth") => HANCOM__HEAD__FIXED_UNDERLINE_WIDTH,
            (HancomHead, b"doNotApplyStrikeoutWithUnderline") => HANCOM__HEAD__DO_NOT_APPLY_STRIKEOUT_WITH_UNDERLINE,
            (HancomHead, b"useLowercaseStrikeout") => HANCOM__HEAD__USE_LOWERCASE_STRIKEOUT,
            (HancomHead, b"extendLineheightToOffset") => HANCOM__HEAD__EXTEND_LINEHEIGHT_TO_OFFSET,
            (HancomHead, b"applyFontspaceToLatin") => HANCOM__HEAD__APPLY_FONTSPACE_TO_LATIN,
            (HancomHead, b"treatQuotationAsLatin") => HANCOM__HEAD__TREAT_QUOTATION_AS_LATIN,
            (HancomHead, b"doNotApplyDiacSymMarkOfNoneAndSix") => HANCOM__HEAD__DO_NOT_APPLY_DIAC_SYM_MARK_OF_NONE_AND_SIX,
            (HancomHead, b"doNotAlignWhitespaceOnRight") => HANCOM__HEAD__DO_NOT_ALIGN_WHITESPACE_ON_RIGHT,
            (HancomHead, b"doNotAdjustWordInJustify") => HANCOM__HEAD__DO_NOT_ADJUST_WORD_IN_JUSTIFY,
            (HancomHead, b"baseCharUnitOnEAsian") => HANCOM__HEAD__BASE_CHAR_UNIT_ON_E_ASIAN,
            (HancomHead, b"baseCharUnitOfIndentOnFirstChar") => HANCOM__HEAD__BASE_CHAR_UNIT_OF_INDENT_ON_FIRST_CHAR,
            (HancomHead, b"adjustLineheightToFont") => HANCOM__HEAD__ADJUST_LINEHEIGHT_TO_FONT,
            (HancomHead, b"adjustBaseInlineFixedLinespacing") => HANCOM__HEAD__ADJUST_BASE_INLINE_FIXED_LINESPACING,
            (HancomHead, b"applyPrevspacingBeneathObject") => HANCOM__HEAD__APPLY_PREVSPACING_BENEATH_OBJECT,
            (HancomHead, b"applyNextspacingOfLastPara") => HANCOM__HEAD__APPLY_NEXTSPACING_OF_LAST_PARA,
            (HancomHead, b"applyAtLeastToPercent100Pct") => HANCOM__HEAD__APPLY_AT_LEAST_TO_PERCENT100_PCT,
            (HancomHead, b"doNotApplyAutoSpaceEAsianEng") => HANCOM__HEAD__DO_NOT_APPLY_AUTO_SPACE_E_ASIAN_ENG,
            (HancomHead, b"doNotApplyAutoSpaceEAsianNum") => HANCOM__HEAD__DO_NOT_APPLY_AUTO_SPACE_E_ASIAN_NUM,
            (HancomHead, b"adjustParaBorderfillToSpacing") => HANCOM__HEAD__ADJUST_PARA_BORDERFILL_TO_SPACING,
            (HancomHead, b"connectParaBorderfillOfEqualBorder") => HANCOM__HEAD__CONNECT_PARA_BORDERFILL_OF_EQUAL_BORDER,
            (HancomHead, b"adjustParaBorderOffsetWithBorder") => HANCOM__HEAD__ADJUST_PARA_BORDER_OFFSET_WITH_BORDER,
            (HancomHead, b"extendLineheightToParaBorderOffset") => HANCOM__HEAD__EXTEND_LINEHEIGHT_TO_PARA_BORDER_OFFSET,
            (HancomHead, b"applyParaBorderToOutside") => HANCOM__HEAD__APPLY_PARA_BORDER_TO_OUTSIDE,
            (HancomHead, b"applyMinColumnWidthTo1mm") => HANCOM__HEAD__APPLY_MIN_COLUMN_WIDTH_TO1MM,
            (HancomHead, b"applyTabPosBasedOnSegment") => HANCOM__HEAD__APPLY_TAB_POS_BASED_ON_SEGMENT,
            (HancomHead, b"breakTabOverline") => HANCOM__HEAD__BREAK_TAB_OVERLINE,
            (HancomHead, b"adjustVertPosOfLine") => HANCOM__HEAD__ADJUST_VERT_POS_OF_LINE,
            (HancomHead, b"doNotApplyWhiteSpaceHeight") => HANCOM__HEAD__DO_NOT_APPLY_WHITE_SPACE_HEIGHT,
            (HancomHead, b"doNotAlignLastPeriod") => HANCOM__HEAD__DO_NOT_ALIGN_LAST_PERIOD,
            (HancomHead, b"doNotAlignLastForbidden") => HANCOM__HEAD__DO_NOT_ALIGN_LAST_FORBIDDEN,
            (HancomHead, b"baseLineSpacingOnLineGrid") => HANCOM__HEAD__BASE_LINE_SPACING_ON_LINE_GRID,
            (HancomHead, b"applyCharSpacingToCharGrid") => HANCOM__HEAD__APPLY_CHAR_SPACING_TO_CHAR_GRID,
            (HancomHead, b"doNotApplyGridInHeaderFooter") => HANCOM__HEAD__DO_NOT_APPLY_GRID_IN_HEADER_FOOTER,
            (HancomHead, b"applyExtendHeaderFooterEachSection") => HANCOM__HEAD__APPLY_EXTEND_HEADER_FOOTER_EACH_SECTION,
            (HancomHead, b"doNotApplyHeaderFooterAtNoSpace") => HANCOM__HEAD__DO_NOT_APPLY_HEADER_FOOTER_AT_NO_SPACE,
            (HancomHead, b"doNotApplyColSeparatorAtNoGap") => HANCOM__HEAD__DO_NOT_APPLY_COL_SEPARATOR_AT_NO_GAP,
            (HancomHead, b"doNotApplyLinegridAtNoLinespacing") => HANCOM__HEAD__DO_NOT_APPLY_LINEGRID_AT_NO_LINESPACING,
            (HancomHead, b"doNotApplyImageEffect") => HANCOM__HEAD__DO_NOT_APPLY_IMAGE_EFFECT,
            (HancomHead, b"doNotApplyShapeComment") => HANCOM__HEAD__DO_NOT_APPLY_SHAPE_COMMENT,
            (HancomHead, b"doNotAdjustEmptyAnchorLine") => HANCOM__HEAD__DO_NOT_ADJUST_EMPTY_ANCHOR_LINE,
            (HancomHead, b"overlapBothAllowOverlap") => HANCOM__HEAD__OVERLAP_BOTH_ALLOW_OVERLAP,
            (HancomHead, b"doNotApplyVertOffsetOfForward") => HANCOM__HEAD__DO_NOT_APPLY_VERT_OFFSET_OF_FORWARD,
            (HancomHead, b"extendVertLimitToPageMargins") => HANCOM__HEAD__EXTEND_VERT_LIMIT_TO_PAGE_MARGINS,
            (HancomHead, b"doNotHoldAnchorOfTable") => HANCOM__HEAD__DO_NOT_HOLD_ANCHOR_OF_TABLE,
            (HancomHead, b"doNotFormattingAtBeneathAnchor") => HANCOM__HEAD__DO_NOT_FORMATTING_AT_BENEATH_ANCHOR,
            (HancomHead, b"adjustBaselineOfObjectToBottom") => HANCOM__HEAD__ADJUST_BASELINE_OF_OBJECT_TO_BOTTOM,
            (HancomHead, b"doNotApplyExtensionCharCompose") => HANCOM__HEAD__DO_NOT_APPLY_EXTENSION_CHAR_COMPOSE,
            (HancomHead, b"linkInfo") => HANCOM__HEAD__LINK_INFO,
            (HancomHead, b"licenseMark") => HANCOM__HEAD__LICENSE_MARK,
            (HancomHead, b"trackChangeEncryption") => HANCOM__HEAD__TRACK_CHANGE_ENCRYPTION,
            (HancomHead, b"hash") => HANCOM__HEAD__HASH,
            (HancomHead, b"derivationKey") => HANCOM__HEAD__DERIVATION_KEY,
            (HancomHead, b"emboss") => HANCOM__HEAD__EMBOSS,
            (HancomHead, b"engrave") => HANCOM__HEAD__ENGRAVE,
            (HancomHead, b"supscript") => HANCOM__HEAD__SUPERSCRIPT,
            (HancomHead, b"subscript") => HANCOM__HEAD__SUBSCRIPT,
        }
    }
}

impl Namespace {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        macro_rules! namespaces {
            (
                $(
                    $uri:expr => $variant:ident,
                )+
            ) => {
                match bytes {
                    $(
                        $uri => Some(Namespace::$variant),
                    )+
                    _ => None,
                }
            };
        }

        namespaces! {
            b"http://www.hancom.co.kr/hwpml/2011/app" => HancomApp,
            b"http://www.hancom.co.kr/hwpml/2011/core" => HancomCore,
            b"http://www.hancom.co.kr/hwpml/2011/head" => HancomHead,
            b"http://www.hancom.co.kr/hwpml/2011/history" => HancomHistory,
            b"http://www.hancom.co.kr/schema/2011/hpf" => HancomHpf,
            b"http://www.hancom.co.kr/hwpml/2011/master-page" => HancomMasterPage,
            b"http://www.hancom.co.kr/hwpml/2016/ooxmlchart" => HancomOoxmlChart,
            b"http://www.hancom.co.kr/hwpml/2011/paragraph" => HancomParagraph2011,
            b"http://www.hancom.co.kr/hwpml/2016/paragraph" => HancomParagraph2016,
            b"http://www.hancom.co.kr/hwpml/2011/section" => HancomSection,
            b"http://www.hancom.co.kr/hwpml/2011/version" => HancomVersion,
            b"urn:oasis:names:tc:opendocument:xmlns:config:1.0" => OdfConfig,
            b"urn:oasis:names:tc:opendocument:xmlns:container" => OdfContainer,
            b"urn:oasis:names:tc:opendocument:xmlns:manifest:1.0" => OdfManifest,
            b"http://purl.org/dc/elements/1.1/" => DoubleCore,
            b"http://www.idpf.org/2007/ops" => OpenDocumentOpf,
            b"http://www.idpf.org/2007/opf/" => OpenDocumentOpf,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_xml() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ha:HWPApplicationSetting xmlns:ha="http://www.hancom.co.kr/hwpml/2011/app"
  xmlns:config="urn:oasis:names:tc:opendocument:xmlns:config:1.0">
  <ha:CaretPosition listIDRef="0" paraIDRef="6" pos="18" />
  <config:config-item-set name="PrintInfo">
    <config:config-item name="PrintAutoFootNote" type="boolean">false</config:config-item>
    <config:config-item name="PrintAutoHeadNote" type="boolean">false</config:config-item>
    <config:config-item name="PrintMethod" type="short">0</config:config-item>
    <config:config-item name="OverlapSize" type="short">0</config:config-item>
    <config:config-item name="PrintCropMark" type="short">0</config:config-item>
    <config:config-item name="BinderHoleType" type="short">0</config:config-item>
    <config:config-item name="ZoomX" type="short">100</config:config-item>
    <config:config-item name="ZoomY" type="short">100</config:config-item>
  </config:config-item-set>
</ha:HWPApplicationSetting>
"#;
        let element = AnyElement::from_bytes(XML)?;

        insta::assert_debug_snapshot!(element);

        Ok(())
    }

    #[test]
    fn version() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hv:HCFVersion xmlns:hv="http://www.hancom.co.kr/hwpml/2011/version"
  tagetApplication="WORDPROCESSOR" major="5" minor="1" micro="0" buildNumber="1" os="1"
  xmlVersion="1.2" application="Hancom Office Hangul" appVersion="10, 0, 0, 9139 WIN32LEWindows_8" />"#;
        let element = AnyElement::from_bytes(XML)?;

        insta::assert_debug_snapshot!(element);

        Ok(())
    }
}
