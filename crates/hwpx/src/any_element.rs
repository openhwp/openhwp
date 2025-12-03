use crate::error::Error;
use quick_xml::{NsReader, events::Event, name::ResolveResult};
use std::io::{BufReader, Cursor};

/// ```ignore
/// let (
///     begin_number,
///     references,
///     forbidden_words,
///     compatible_document,
///     track_change_config,
///     document_option,
///     meta_tag,
/// ) = children!(element;
///     one HANCOM__HEAD__BEGIN_NUMBER, BeginNumber;
///     one HANCOM__HEAD__REFERENCES, ReferenceList;
///     many HANCOM__HEAD__FORBIDDEN_WORD, ForbiddenWord;
///     opt HANCOM__HEAD__COMPATIBLE_DOCUMENT, CompatibleDocument;
///     opt HANCOM__HEAD__TRACK_CHANGE_CONFIG, TrackChangeConfig;
///     opt HANCOM__HEAD__DOCUMENT_OPTION, DocumentOption;
///     opt HANCOM__HEAD__META_TAG, MetaTag;
/// );
/// ```
///
/// `nested` 키워드는 wrapper 태그의 children을 파싱합니다:
/// ```ignore
/// let (font_faces,) = children!(element;
///     nested HANCOM__HEAD__FONT_FACES, FontFaceType;
/// );
/// // wrapper 태그 <fontfaces>의 children을 Vec<FontFaceType>으로 파싱
/// ```
///
/// `nested_nonempty` 키워드는 wrapper 태그의 children을 NonEmpty<T>로 파싱합니다:
/// ```ignore
/// let (char_properties,) = children!(element;
///     nested_nonempty HANCOM__HEAD__CHARACTER_PROPERTIES, CharShapeType;
/// );
/// // wrapper 태그의 children을 NonEmpty<CharShapeType>으로 파싱 (최소 1개 필수)
/// ```
#[macro_export]
macro_rules! children {
    (@parse $children:expr; [$($acc:expr),*] $(;)?) => {
        ($($acc),*)
    };
    (@parse $children:expr; [$($acc:expr),*]; one $variant:ident, $type:ty $(; $($rest:tt)*)?) => {{
        let mut children = $children;
        #[allow(unused)]
        let (children, value) = match children.pop() {
            Some(child) => {
                child.expect(ElementName::$variant)?;
                (children, <$type>::try_from(child)?)
            }
            None => missing_element!("{}", ElementName::$variant.display_name()),
        };
        children!(@parse children; [$($acc,)* value] $(; $($rest)*)?)
    }};
    (@parse $children:expr; [$($acc:expr),*]; opt $variant:ident, $type:ty $(; $($rest:tt)*)?) => {{
        let mut children = $children;
        #[allow(unused)]
        let (children, value) = match children.pop() {
            Some(child) if child.name == ElementName::$variant => {
                (children, Some(<$type>::try_from(child)?))
            }
            Some(child) => {
                children.push(child);
                (children, None)
            }
            None => (children, None),
        };
        children!(@parse children; [$($acc,)* value] $(; $($rest)*)?)
    }};
    (@parse $children:expr; [$($acc:expr),*]; many $variant:ident, $type:ty $(; $($rest:tt)*)?) => {{
        #[allow(unused)]
        let mut children = $children;
        let mut items = vec![];
        while let Some(child) = children.pop() {
            if child.name == ElementName::$variant {
                items.push(<$type>::try_from(child)?);
            } else {
                children.push(child);
                break;
            }
        }
        children!(@parse children; [$($acc,)* items] $(; $($rest)*)?)
    }};
    // nonempty: 같은 태그가 1개 이상 연속 (NonEmpty 반환)
    (@parse $children:expr; [$($acc:expr),*]; nonempty $variant:ident, $type:ty $(; $($rest:tt)*)?) => {{
        #[allow(unused)]
        let mut children = $children;
        let mut items = vec![];
        while let Some(child) = children.pop() {
            if child.name == ElementName::$variant {
                items.push(<$type>::try_from(child)?);
            } else {
                children.push(child);
                break;
            }
        }
        let items = match NonEmpty::from_vec(items) {
            Some(nonempty) => nonempty,
            None => missing_element!("{}", ElementName::$variant.display_name()),
        };
        children!(@parse children; [$($acc,)* items] $(; $($rest)*)?)
    }};
    // nested: wrapper 태그의 children을 파싱 (필수, Vec 반환)
    (@parse $children:expr; [$($acc:expr),*]; nested $variant:ident, $type:ty $(; $($rest:tt)*)?) => {{
        let mut children = $children;
        #[allow(unused)]
        let (children, items) = match children.pop() {
            Some(child) => {
                child.expect(ElementName::$variant)?;
                let mut items = vec![];
                for grandchild in child.children {
                    items.push(<$type>::try_from(grandchild)?);
                }
                (children, items)
            }
            None => missing_element!("{}", ElementName::$variant.display_name()),
        };
        children!(@parse children; [$($acc,)* items] $(; $($rest)*)?)
    }};
    // nested_opt: wrapper 태그의 children을 파싱 (선택적, Vec 반환)
    (@parse $children:expr; [$($acc:expr),*]; nested_opt $variant:ident, $type:ty $(; $($rest:tt)*)?) => {{
        let mut children = $children;
        #[allow(unused)]
        let (children, items) = match children.pop() {
            Some(child) if child.name == ElementName::$variant => {
                let mut items = vec![];
                for grandchild in child.children {
                    items.push(<$type>::try_from(grandchild)?);
                }
                (children, items)
            }
            Some(child) => {
                children.push(child);
                (children, vec![])
            }
            None => (children, vec![]),
        };
        children!(@parse children; [$($acc,)* items] $(; $($rest)*)?)
    }};
    // nested_nonempty: wrapper 태그의 children을 파싱 (필수, NonEmpty 반환)
    (@parse $children:expr; [$($acc:expr),*]; nested_nonempty $variant:ident, $type:ty $(; $($rest:tt)*)?) => {{
        let mut children = $children;
        #[allow(unused)]
        let (children, items) = match children.pop() {
            Some(child) => {
                child.expect(ElementName::$variant)?;
                let mut items = vec![];
                for grandchild in child.children {
                    items.push(<$type>::try_from(grandchild)?);
                }
                match NonEmpty::from_vec(items) {
                    Some(nonempty) => (children, nonempty),
                    None => missing_element!("{} (children)", ElementName::$variant.display_name()),
                }
            }
            None => missing_element!("{}", ElementName::$variant.display_name()),
        };
        children!(@parse children; [$($acc,)* items] $(; $($rest)*)?)
    }};
    // nested_opt_nonempty: wrapper 태그의 children을 파싱 (선택적, Option<NonEmpty> 반환)
    (@parse $children:expr; [$($acc:expr),*]; nested_opt_nonempty $variant:ident, $type:ty $(; $($rest:tt)*)?) => {{
        let mut children = $children;
        #[allow(unused)]
        let (children, items) = match children.pop() {
            Some(child) if child.name == ElementName::$variant => {
                let mut items = vec![];
                for grandchild in child.children {
                    items.push(<$type>::try_from(grandchild)?);
                }
                (children, NonEmpty::from_vec(items))
            }
            Some(child) => {
                children.push(child);
                (children, None)
            }
            None => (children, None),
        };
        children!(@parse children; [$($acc,)* items] $(; $($rest)*)?)
    }};
    ($element:ident; $($rest:tt)*) => {{
        let children: Vec<_> = $element.children.into_iter().rev().collect();
        children!(@parse children; []; $($rest)*)
    }};
}

/// attributes 매크로: XML element의 attributes를 파싱합니다.
///
/// ## 키워드
/// - `one T`: 필수 attribute, `.parse::<T>()` 호출
/// - `one (string)`: 필수 String attribute (parse 없이 그대로 사용)
/// - `one (boolean)`: 필수 boolean attribute ("true"/"1"/"false"/"0")
/// - `opt T`: 선택적 attribute, `.parse::<T>()` 호출 (Option<T> 반환)
/// - `opt (string)`: 선택적 String attribute (Option<String> 반환)
/// - `default $val`: 기본값이 있는 attribute, `.parse()` 호출
/// - `default $val; boolean`: 기본값이 있는 boolean attribute ("true"/"1"/"false"/"0")
///
/// ## 사용 예시
/// ```ignore
/// let (id, face, r#type, embedded, binary_item_id_ref) = attributes!(element, "font";
///     "id" as id => one xs::PositiveInteger32,
///     "face" as face => one (string),
///     "type" as r#type => one FontKind,
///     "embedded" as embedded => default false; boolean,
///     "binaryItemIDRef" as binary_item_id_ref => opt (string),
/// );
/// ```
#[macro_export]
macro_rules! attributes {
    // 내부: 변수 선언 생성
    (@vars; $(,)?) => {};
    (@vars; $name:literal as $var:ident => one (string) $(, $($rest:tt)*)?) => {
        let mut $var = None;
        attributes!(@vars; $($($rest)*)?);
    };
    (@vars; $name:literal as $var:ident => one (boolean) $(, $($rest:tt)*)?) => {
        let mut $var = None;
        attributes!(@vars; $($($rest)*)?);
    };
    (@vars; $name:literal as $var:ident => one $type:ty $(, $($rest:tt)*)?) => {
        let mut $var = None;
        attributes!(@vars; $($($rest)*)?);
    };
    (@vars; $name:literal as $var:ident => opt (string) $(, $($rest:tt)*)?) => {
        let mut $var = None;
        attributes!(@vars; $($($rest)*)?);
    };
    (@vars; $name:literal as $var:ident => opt (boolean) $(, $($rest:tt)*)?) => {
        let mut $var = None;
        attributes!(@vars; $($($rest)*)?);
    };
    (@vars; $name:literal as $var:ident => opt $type:ty $(, $($rest:tt)*)?) => {
        let mut $var = None;
        attributes!(@vars; $($($rest)*)?);
    };
    (@vars; $name:literal as $var:ident => default $default:expr; $parse:ident $(, $($rest:tt)*)?) => {
        let mut $var = $default;
        attributes!(@vars; $($($rest)*)?);
    };
    (@vars; $name:literal as $var:ident => default $default:expr $(, $($rest:tt)*)?) => {
        let mut $var = $default;
        attributes!(@vars; $($($rest)*)?);
    };

    // 내부: match arm 생성
    (@arms $key:ident, $value:ident, $tag:literal; $(,)?) => {};
    (@arms $key:ident, $value:ident, $tag:literal; $name:literal as $var:ident => one (string) $(, $($rest:tt)*)?) => {
        if $key == $name {
            $var = Some($value.clone());
        }
        attributes!(@arms $key, $value, $tag; $($($rest)*)?);
    };
    (@arms $key:ident, $value:ident, $tag:literal; $name:literal as $var:ident => one (boolean) $(, $($rest:tt)*)?) => {
        if $key == $name {
            $var = boolean!($value.as_str(), concat!("<", $tag, " ", $name, ">"));
        }
        attributes!(@arms $key, $value, $tag; $($($rest)*)?);
    };
    (@arms $key:ident, $value:ident, $tag:literal; $name:literal as $var:ident => one $type:ty $(, $($rest:tt)*)?) => {
        if $key == $name {
            $var = Some($value.parse::<$type>()?);
        }
        attributes!(@arms $key, $value, $tag; $($($rest)*)?);
    };
    (@arms $key:ident, $value:ident, $tag:literal; $name:literal as $var:ident => opt (string) $(, $($rest:tt)*)?) => {
        if $key == $name {
            $var = Some($value.clone());
        }
        attributes!(@arms $key, $value, $tag; $($($rest)*)?);
    };
    (@arms $key:ident, $value:ident, $tag:literal; $name:literal as $var:ident => opt (boolean) $(, $($rest:tt)*)?) => {
        if $key == $name {
            $var = boolean!($value.as_str(), concat!("<", $tag, " ", $name, ">"));
        }
        attributes!(@arms $key, $value, $tag; $($($rest)*)?);
    };
    (@arms $key:ident, $value:ident, $tag:literal; $name:literal as $var:ident => opt $type:ty $(, $($rest:tt)*)?) => {
        if $key == $name {
            $var = Some($value.parse::<$type>()?);
        }
        attributes!(@arms $key, $value, $tag; $($($rest)*)?);
    };
    (@arms $key:ident, $value:ident, $tag:literal; $name:literal as $var:ident => default $default:expr; boolean $(, $($rest:tt)*)?) => {
        if $key == $name {
            $var = boolean!($value.as_str(), concat!("<", $tag, " ", $name, ">"));
        }
        attributes!(@arms $key, $value, $tag; $($($rest)*)?);
    };
    (@arms $key:ident, $value:ident, $tag:literal; $name:literal as $var:ident => default $default:expr $(, $($rest:tt)*)?) => {
        if $key == $name {
            $var = $value.parse()?;
        }
        attributes!(@arms $key, $value, $tag; $($($rest)*)?);
    };

    // 내부: 필수 검증 및 결과 생성
    (@result $tag:literal; [] $(,)?) => {
        ()
    };
    (@result $tag:literal; [$($acc:tt)*]; $name:literal as $var:ident => one (string) $(, $($rest:tt)*)?) => {
        attributes!(@result $tag; [$($acc)* $var.ok_or_else(|| Error::MissingAttribute(concat!("<", $tag, " ", $name, ">").to_string()))?,]; $($($rest)*)?)
    };
    (@result $tag:literal; [$($acc:tt)*]; $name:literal as $var:ident => one (boolean) $(, $($rest:tt)*)?) => {
        attributes!(@result $tag; [$($acc)* $var.ok_or_else(|| Error::MissingAttribute(concat!("<", $tag, " ", $name, ">").to_string()))?,]; $($($rest)*)?)
    };
    (@result $tag:literal; [$($acc:tt)*]; $name:literal as $var:ident => one $type:ty $(, $($rest:tt)*)?) => {
        attributes!(@result $tag; [$($acc)* $var.ok_or_else(|| Error::MissingAttribute(concat!("<", $tag, " ", $name, ">").to_string()))?,]; $($($rest)*)?)
    };
    (@result $tag:literal; [$($acc:tt)*]; $name:literal as $var:ident => opt (string) $(, $($rest:tt)*)?) => {
        attributes!(@result $tag; [$($acc)* $var,]; $($($rest)*)?)
    };
    (@result $tag:literal; [$($acc:tt)*]; $name:literal as $var:ident => opt (boolean) $(, $($rest:tt)*)?) => {
        attributes!(@result $tag; [$($acc)* $var,]; $($($rest)*)?)
    };
    (@result $tag:literal; [$($acc:tt)*]; $name:literal as $var:ident => opt $type:ty $(, $($rest:tt)*)?) => {
        attributes!(@result $tag; [$($acc)* $var,]; $($($rest)*)?)
    };
    (@result $tag:literal; [$($acc:tt)*]; $name:literal as $var:ident => default $default:expr; $parse:ident $(, $($rest:tt)*)?) => {
        attributes!(@result $tag; [$($acc)* $var,]; $($($rest)*)?)
    };
    (@result $tag:literal; [$($acc:tt)*]; $name:literal as $var:ident => default $default:expr $(, $($rest:tt)*)?) => {
        attributes!(@result $tag; [$($acc)* $var,]; $($($rest)*)?)
    };
    (@result $tag:literal; [$($acc:expr,)+] $(;)?) => {
        ($($acc),+)
    };

    // 진입점
    ($element:expr, $tag:literal; $($specs:tt)*) => {{
        attributes!(@vars; $($specs)*);
        for (__key, __value) in &$element.attributes {
            attributes!(@arms __key, __value, $tag; $($specs)*);
        }
        attributes!(@result $tag; []; $($specs)*)
    }};
}

macro_rules! decode {
    ($value:expr) => {
        String::from_utf8_lossy(&$value.into_inner()).into_owned()
    };
}

/// Represents any XML element with its namespace, name, attributes, and children.
#[derive(Debug)]
pub struct AnyElement {
    pub name: ElementName,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<AnyElement>,
    pub text: Option<String>,
}

macro_rules! element_names {
    (
        $(
            $variant:ident => ($ns:ident, $local:expr),
        )+
    ) => {
        /// Represents the names of XML elements with their associated namespaces.
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ElementName {
            $(
                #[doc = concat!("`<", $local, ">`")]
                $variant,
            )+
        }

        impl ElementName {
            pub fn display_name(&self) -> &'static str {
                match self {
                    $(
                        ElementName::$variant => $local,
                    )+
                }
            }

            fn from_event<'a>(namespace: ResolveResult<'a>, event: &quick_xml::events::BytesStart<'a>) -> Result<Self, Error> {
                let namespace = match namespace {
                    ResolveResult::Bound(p) => match Namespace::from_bytes(p.into_inner()) {
                        Some(ns) => ns,
                        None => unknown!("Unknown namespace in element name: {}", decode!(p)),
                    },
                    _ => unknown!("Unbound namespace in element name"),
                };
                let local_name = decode!(event.local_name());

                match (namespace, local_name.as_str()) {
                    $(
                        (Namespace::$ns, $local) => Ok(ElementName::$variant),
                    )+
                    _ => unknown!("Unknown element name: {{{:?}}}{}", namespace, local_name),

                }
            }
        }
    };
}

element_names! {
    HANCOM__APP__HWP_APPLICATION_SETTING => (HancomApp, "HWPApplicationSetting"),
    HANCOM__APP__CARET_POSITION => (HancomApp, "CaretPosition"),
    HANCOM__VERSION__HCF_VERSION => (HancomVersion, "HCFVersion"),
    OPENDOCUMENT__CONFIG__CONFIG_ITEM_SET => (OpenDocumentConfig, "config-item-set"),
    OPENDOCUMENT__CONFIG__CONFIG_ITEM => (OpenDocumentConfig, "config-item"),
    OPENDOCUMENT__OPF__PACKAGE => (OpenDocumentOpf, "package"),
    OPENDOCUMENT__OPF__METADATA => (OpenDocumentOpf, "metadata"),
    OPENDOCUMENT__OPF__MANIFEST => (OpenDocumentOpf, "manifest"),
    OPENDOCUMENT__OPF__SPINE => (OpenDocumentOpf, "spine"),
    OPENDOCUMENT__OPF__TITLE => (OpenDocumentOpf, "title"),
    OPENDOCUMENT__OPF__LANGUAGE => (OpenDocumentOpf, "language"),
    OPENDOCUMENT__OPF__META => (OpenDocumentOpf, "meta"),
    OPENDOCUMENT__OPF__ITEM => (OpenDocumentOpf, "item"),
    OPENDOCUMENT__OPF__ITEM_REFERENCE => (OpenDocumentOpf, "itemref"),
    HANCOM__CORE__FILL_BRUSH => (HancomCore, "fillBrush"),
    HANCOM__CORE__WIN_BRUSH => (HancomCore, "winBrush"),
    HANCOM__CORE__GRADATION => (HancomCore, "gradation"),
    HANCOM__CORE__IMAGE_BRUSH => (HancomCore, "imgBrush"),
    HANCOM__CORE__IMAGE => (HancomCore, "img"),
    HANCOM__CORE__INDENT => (HancomCore, "intent"),
    HANCOM__CORE__LEFT => (HancomCore, "left"),
    HANCOM__CORE__RIGHT => (HancomCore, "right"),
    HANCOM__CORE__PREVIOUS => (HancomCore, "prev"),
    HANCOM__CORE__NEXT => (HancomCore, "next"),
    HANCOM__HEAD__HEAD => (HancomHead, "head"),
    HANCOM__HEAD__BEGIN_NUMBER => (HancomHead, "beginNum"),
    HANCOM__HEAD__REFERENCES => (HancomHead, "refList"),
    HANCOM__HEAD__FORBIDDEN_WORDS => (HancomHead, "forbiddenWordList"),
    HANCOM__HEAD__TRACK_CHANGE_CONFIG => (HancomHead, "trackchangeConfig"),
    HANCOM__HEAD__DOCUMENT_OPTION => (HancomHead, "docOption"),
    HANCOM__HEAD__META_TAG => (HancomHead, "metaTag"),
    HANCOM__HEAD__FONT_FACES => (HancomHead, "fontfaces"),
    HANCOM__HEAD__BORDER_FILLS => (HancomHead, "borderFills"),
    HANCOM__HEAD__CHARACTER_PROPERTIES => (HancomHead, "charProperties"),
    HANCOM__HEAD__TAB_PROPERTIES => (HancomHead, "tabProperties"),
    HANCOM__HEAD__NUMBERINGS => (HancomHead, "numberings"),
    HANCOM__HEAD__BULLETS => (HancomHead, "bullets"),
    HANCOM__HEAD__PARAGRAPH_PROPERTIES => (HancomHead, "paraProperties"),
    HANCOM__HEAD__PARAGRAPH_PROPERTY => (HancomHead, "paraPr"),
    HANCOM__HEAD__STYLES => (HancomHead, "styles"),
    HANCOM__HEAD__MEMO_PROPERTIES => (HancomHead, "memoProperties"),
    HANCOM__HEAD__TRACK_CHANGES => (HancomHead, "trackChanges"),
    HANCOM__HEAD__TRACK_CHANGE_AUTHORS => (HancomHead, "trackChangeAuthors"),
    HANCOM__HEAD__CHARACTER_PROPERTY => (HancomHead, "charPr"),
    HANCOM__HEAD__FONT_FACE => (HancomHead, "fontface"),
    HANCOM__HEAD__FONT => (HancomHead, "font"),
    HANCOM__HEAD__SUBSET_FONT => (HancomHead, "substFont"),
    HANCOM__HEAD__TYPE_INFO => (HancomHead, "typeInfo"),
    HANCOM__HEAD__BORDER_FILL => (HancomHead, "borderFill"),
    HANCOM__HEAD__SLASH => (HancomHead, "slash"),
    HANCOM__HEAD__BACK_SLASH => (HancomHead, "backSlash"),
    HANCOM__HEAD__LEFT_BORDER => (HancomHead, "leftBorder"),
    HANCOM__HEAD__RIGHT_BORDER => (HancomHead, "rightBorder"),
    HANCOM__HEAD__TOP_BORDER => (HancomHead, "topBorder"),
    HANCOM__HEAD__BOTTOM_BORDER => (HancomHead, "bottomBorder"),
    HANCOM__HEAD__DIAGONAL => (HancomHead, "diagonal"),
    HANCOM__HEAD__COLOR => (HancomHead, "color"),
    HANCOM__HEAD__CHARACTER_SHAPE => (HancomHead, "charShape"),
    HANCOM__HEAD__FONT_REFERENCE => (HancomHead, "fontRef"),
    HANCOM__HEAD__RATIO => (HancomHead, "ratio"),
    HANCOM__HEAD__SPACING => (HancomHead, "spacing"),
    HANCOM__HEAD__RELATIVE_SIZE => (HancomHead, "relSz"),
    HANCOM__HEAD__OFFSET => (HancomHead, "offset"),
    HANCOM__HEAD__ITALIC => (HancomHead, "italic"),
    HANCOM__HEAD__BOLD => (HancomHead, "bold"),
    HANCOM__HEAD__UNDERLINE => (HancomHead, "underline"),
    HANCOM__HEAD__STRIKEOUT => (HancomHead, "strikeout"),
    HANCOM__HEAD__OUTLINE => (HancomHead, "outline"),
    HANCOM__HEAD__SHADOW => (HancomHead, "shadow"),
    HANCOM__HEAD__TAB_DEFINITION => (HancomHead, "tabDef"),
    HANCOM__HEAD__TAB_ITEM => (HancomHead, "tabItem"),
    HANCOM__HEAD__NUMBERING => (HancomHead, "numbering"),
    HANCOM__HEAD__PARAGRAPH_SHAPE => (HancomHead, "paraShape"),
    HANCOM__HEAD__ALIGN => (HancomHead, "align"),
    HANCOM__HEAD__HEADING => (HancomHead, "heading"),
    HANCOM__HEAD__BREAK_SETTING => (HancomHead, "breakSetting"),
    HANCOM__HEAD__MARGIN => (HancomHead, "margin"),
    HANCOM__HEAD__LINE_SPACING => (HancomHead, "lineSpacing"),
    HANCOM__HEAD__BORDER => (HancomHead, "border"),
    HANCOM__HEAD__AUTO_SPACING => (HancomHead, "autoSpacing"),
    HANCOM__HEAD__PARAGRAPH_HEAD => (HancomHead, "paraHead"),
    HANCOM__HEAD__BULLET => (HancomHead, "bullet"),
    HANCOM__HEAD__STYLE => (HancomHead, "style"),
    HANCOM__HEAD__FORBIDDEN_WORD => (HancomHead, "forbiddenWord"),
    HANCOM__HEAD__MEMO_PROPERTY => (HancomHead, "memoPr"),
    HANCOM__HEAD__TRACK_CHANGE => (HancomHead, "trackChange"),
    HANCOM__HEAD__TRACK_CHANGE_AUTHOR => (HancomHead, "trackChangeAuthor"),
    HANCOM__HEAD__COMPATIBLE_DOCUMENT => (HancomHead, "compatibleDocument"),
    HANCOM__HEAD__LAYOUT_COMPATIBILITY => (HancomHead, "layoutCompatibility"),
    HANCOM__HEAD__APPLY_FONT_WEIGHT_TO_BOLD => (HancomHead, "applyFontWeightToBold"),
    HANCOM__HEAD__USE_INNER_UNDERLINE => (HancomHead, "useInnerUnderline"),
    HANCOM__HEAD__FIXED_UNDERLINE_WIDTH => (HancomHead, "fixedUnderlineWidth"),
    HANCOM__HEAD__DO_NOT_APPLY_STRIKEOUT_WITH_UNDERLINE => (HancomHead, "doNotApplyStrikeoutWithUnderline"),
    HANCOM__HEAD__USE_LOWERCASE_STRIKEOUT => (HancomHead, "useLowercaseStrikeout"),
    HANCOM__HEAD__EXTEND_LINEHEIGHT_TO_OFFSET => (HancomHead, "extendLineheightToOffset"),
    HANCOM__HEAD__APPLY_FONTSPACE_TO_LATIN => (HancomHead, "applyFontspaceToLatin"),
    HANCOM__HEAD__TREAT_QUOTATION_AS_LATIN => (HancomHead, "treatQuotationAsLatin"),
    HANCOM__HEAD__DO_NOT_APPLY_DIAC_SYM_MARK_OF_NONE_AND_SIX => (HancomHead, "doNotApplyDiacSymMarkOfNoneAndSix"),
    HANCOM__HEAD__DO_NOT_ALIGN_WHITESPACE_ON_RIGHT => (HancomHead, "doNotAlignWhitespaceOnRight"),
    HANCOM__HEAD__DO_NOT_ADJUST_WORD_IN_JUSTIFY => (HancomHead, "doNotAdjustWordInJustify"),
    HANCOM__HEAD__BASE_CHAR_UNIT_ON_E_ASIAN => (HancomHead, "baseCharUnitOnEAsian"),
    HANCOM__HEAD__BASE_CHAR_UNIT_OF_INDENT_ON_FIRST_CHAR => (HancomHead, "baseCharUnitOfIndentOnFirstChar"),
    HANCOM__HEAD__ADJUST_LINEHEIGHT_TO_FONT => (HancomHead, "adjustLineheightToFont"),
    HANCOM__HEAD__ADJUST_BASE_INLINE_FIXED_LINESPACING => (HancomHead, "adjustBaseInlineFixedLinespacing"),
    HANCOM__HEAD__APPLY_PREVSPACING_BENEATH_OBJECT => (HancomHead, "applyPrevspacingBeneathObject"),
    HANCOM__HEAD__APPLY_NEXTSPACING_OF_LAST_PARA => (HancomHead, "applyNextspacingOfLastPara"),
    HANCOM__HEAD__APPLY_AT_LEAST_TO_PERCENT100_PCT => (HancomHead, "applyAtLeastToPercent100Pct"),
    HANCOM__HEAD__DO_NOT_APPLY_AUTO_SPACE_E_ASIAN_ENG => (HancomHead, "doNotApplyAutoSpaceEAsianEng"),
    HANCOM__HEAD__DO_NOT_APPLY_AUTO_SPACE_E_ASIAN_NUM => (HancomHead, "doNotApplyAutoSpaceEAsianNum"),
    HANCOM__HEAD__ADJUST_PARA_BORDERFILL_TO_SPACING => (HancomHead, "adjustParaBorderfillToSpacing"),
    HANCOM__HEAD__CONNECT_PARA_BORDERFILL_OF_EQUAL_BORDER => (HancomHead, "connectParaBorderfillOfEqualBorder"),
    HANCOM__HEAD__ADJUST_PARA_BORDER_OFFSET_WITH_BORDER => (HancomHead, "adjustParaBorderOffsetWithBorder"),
    HANCOM__HEAD__EXTEND_LINEHEIGHT_TO_PARA_BORDER_OFFSET => (HancomHead, "extendLineheightToParaBorderOffset"),
    HANCOM__HEAD__APPLY_PARA_BORDER_TO_OUTSIDE => (HancomHead, "applyParaBorderToOutside"),
    HANCOM__HEAD__APPLY_MIN_COLUMN_WIDTH_TO1MM => (HancomHead, "applyMinColumnWidthTo1mm"),
    HANCOM__HEAD__APPLY_TAB_POS_BASED_ON_SEGMENT => (HancomHead, "applyTabPosBasedOnSegment"),
    HANCOM__HEAD__BREAK_TAB_OVERLINE => (HancomHead, "breakTabOverline"),
    HANCOM__HEAD__ADJUST_VERT_POS_OF_LINE => (HancomHead, "adjustVertPosOfLine"),
    HANCOM__HEAD__DO_NOT_APPLY_WHITE_SPACE_HEIGHT => (HancomHead, "doNotApplyWhiteSpaceHeight"),
    HANCOM__HEAD__DO_NOT_ALIGN_LAST_PERIOD => (HancomHead, "doNotAlignLastPeriod"),
    HANCOM__HEAD__DO_NOT_ALIGN_LAST_FORBIDDEN => (HancomHead, "doNotAlignLastForbidden"),
    HANCOM__HEAD__BASE_LINE_SPACING_ON_LINE_GRID => (HancomHead, "baseLineSpacingOnLineGrid"),
    HANCOM__HEAD__APPLY_CHAR_SPACING_TO_CHAR_GRID => (HancomHead, "applyCharSpacingToCharGrid"),
    HANCOM__HEAD__DO_NOT_APPLY_GRID_IN_HEADER_FOOTER => (HancomHead, "doNotApplyGridInHeaderFooter"),
    HANCOM__HEAD__APPLY_EXTEND_HEADER_FOOTER_EACH_SECTION => (HancomHead, "applyExtendHeaderFooterEachSection"),
    HANCOM__HEAD__DO_NOT_APPLY_HEADER_FOOTER_AT_NO_SPACE => (HancomHead, "doNotApplyHeaderFooterAtNoSpace"),
    HANCOM__HEAD__DO_NOT_APPLY_COL_SEPARATOR_AT_NO_GAP => (HancomHead, "doNotApplyColSeparatorAtNoGap"),
    HANCOM__HEAD__DO_NOT_APPLY_LINEGRID_AT_NO_LINESPACING => (HancomHead, "doNotApplyLinegridAtNoLinespacing"),
    HANCOM__HEAD__DO_NOT_APPLY_IMAGE_EFFECT => (HancomHead, "doNotApplyImageEffect"),
    HANCOM__HEAD__DO_NOT_APPLY_SHAPE_COMMENT => (HancomHead, "doNotApplyShapeComment"),
    HANCOM__HEAD__DO_NOT_ADJUST_EMPTY_ANCHOR_LINE => (HancomHead, "doNotAdjustEmptyAnchorLine"),
    HANCOM__HEAD__OVERLAP_BOTH_ALLOW_OVERLAP => (HancomHead, "overlapBothAllowOverlap"),
    HANCOM__HEAD__DO_NOT_APPLY_VERT_OFFSET_OF_FORWARD => (HancomHead, "doNotApplyVertOffsetOfForward"),
    HANCOM__HEAD__EXTEND_VERT_LIMIT_TO_PAGE_MARGINS => (HancomHead, "extendVertLimitToPageMargins"),
    HANCOM__HEAD__DO_NOT_HOLD_ANCHOR_OF_TABLE => (HancomHead, "doNotHoldAnchorOfTable"),
    HANCOM__HEAD__DO_NOT_FORMATTING_AT_BENEATH_ANCHOR => (HancomHead, "doNotFormattingAtBeneathAnchor"),
    HANCOM__HEAD__ADJUST_BASELINE_OF_OBJECT_TO_BOTTOM => (HancomHead, "adjustBaselineOfObjectToBottom"),
    HANCOM__HEAD__DO_NOT_APPLY_EXTENSION_CHAR_COMPOSE => (HancomHead, "doNotApplyExtensionCharCompose"),
    HANCOM__HEAD__LINK_INFO => (HancomHead, "linkInfo"),
    HANCOM__HEAD__LICENSE_MARK => (HancomHead, "licenseMark"),
    HANCOM__HEAD__TRACK_CHANGE_ENCRYPTION => (HancomHead, "trackChangeEncryption"),
    HANCOM__HEAD__HASH => (HancomHead, "hash"),
    HANCOM__HEAD__DERIVATION_KEY => (HancomHead, "derivationKey"),
    HANCOM__HEAD__EMBOSS => (HancomHead, "emboss"),
    HANCOM__HEAD__ENGRAVE => (HancomHead, "engrave"),
    HANCOM__HEAD__SUPERSCRIPT => (HancomHead, "supscript"),
    HANCOM__HEAD__SUBSCRIPT => (HancomHead, "subscript"),
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
    OpenDocumentConfig,
    /// "urn:oasis:names:tc:opendocument:xmlns:container"
    OpenDocumentContainer,
    /// "urn:oasis:names:tc:opendocument:xmlns:manifest:1.0"
    OpenDocumentManifest,
    /// "http://purl.org/dc/elements/1.1/"
    DoubleCore,
    /// "http://www.idpf.org/2007/ops"
    OpenDocumentOps,
    /// "http://www.idpf.org/2007/opf/"
    OpenDocumentOpf,
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
            b"urn:oasis:names:tc:opendocument:xmlns:config:1.0" => OpenDocumentConfig,
            b"urn:oasis:names:tc:opendocument:xmlns:container" => OpenDocumentContainer,
            b"urn:oasis:names:tc:opendocument:xmlns:manifest:1.0" => OpenDocumentManifest,
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
