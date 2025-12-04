//! namespace: http://www.hancom.co.kr/hwpml/2011/head
//! filename: Contents/header.xml

use crate::{
    any_element::{AnyElement, ElementName},
    arbitrary, core,
    error::Error,
    xs,
};
use fxhash::FxHashSet;
use nonempty::NonEmpty;

/// ```xml
/// <head>
///   <beginNum ... />
///   <refList>...</refList>
///   <forbiddenWordList>...</forbiddenWordList>
///   <compatibleDoc>...</compatibleDoc>
///   <trackchangeConfig>...</trackchangeConfig>
///   <docOption>...</docOption>
///   <metaTag ... />
/// </head>
/// ```
///
/// Root Element
#[derive(Debug)]
pub struct HWPMLHeadType {
    /// ```xml
    /// <beginNum ... />
    /// ```
    pub begin_number: BeginNumber,
    /// ```xml
    /// <refList>...</refList>
    /// ```
    pub reference: MappingTableType,
    /// ```xml
    /// <forbiddenWordList>...</forbiddenWordList>
    /// ```
    pub forbidden_words: ForbiddenWordList,
    /// ```xml
    /// <compatibleDocument>...</compatibleDocument>
    /// ```
    pub compatible_document: Option<CompatibleDocument>,
    /// ```xml
    /// <trackchangeConfig>...</trackchangeConfig>
    /// ```
    pub track_change_config: Option<TrackChangeConfig>,
    /// ```xml
    /// <docOption>...</docOption>
    /// ```
    pub document_option: Option<DocumentOption>,
    /// ```xml
    /// <metaTag ... />
    /// ```
    pub meta_tag: Option<MetaTag>,
}

impl TryFrom<AnyElement> for HWPMLHeadType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__HEAD)?;

        let (
            begin_number,
            reference,
            forbidden_words,
            compatible_document,
            track_change_config,
            document_option,
            meta_tag,
        ) = children! {
            element;
            one HANCOM__HEAD__BEGIN_NUMBER, BeginNumber;
            one HANCOM__HEAD__REFERENCES, MappingTableType;
            opt HANCOM__HEAD__FORBIDDEN_WORDS, ForbiddenWordList;
            opt HANCOM__HEAD__COMPATIBLE_DOCUMENT, CompatibleDocument;
            opt HANCOM__HEAD__TRACK_CHANGE_CONFIG, TrackChangeConfig;
            opt HANCOM__HEAD__DOCUMENT_OPTION, DocumentOption;
            opt HANCOM__HEAD__META_TAG, MetaTag;
        };

        let forbidden_words = forbidden_words.unwrap_or_default();

        Ok(Self {
            begin_number,
            reference,
            forbidden_words,
            compatible_document,
            track_change_config,
            document_option,
            meta_tag,
        })
    }
}

/// ```xml
/// <beginNum
///   page="{xs:positiveInteger}"
///   footnote="{xs:positiveInteger}"
///   endnote="{xs:positiveInteger}"
///   pic="{xs:positiveInteger}"
///   tbl="{xs:positiveInteger}"
///   equation="{xs:positiveInteger}"
/// />
/// ```
///
/// 시작 번호
#[derive(Debug)]
pub struct BeginNumber {
    /// ```xml
    /// page="{xs:positiveInteger}"
    /// ```
    ///
    /// 페이지 시작 번호
    pub page: xs::PositiveInteger32,
    /// ```xml
    /// footnote="{xs:positiveInteger}"
    /// ```
    ///
    /// 각주 시작 번호
    pub foot_note: xs::PositiveInteger32,
    /// ```xml
    /// endnote="{xs:positiveInteger}"
    /// ```
    ///
    /// 미주 시작 번호
    pub end_note: xs::PositiveInteger32,
    /// ```xml
    /// picture="{xs:positiveInteger}"
    /// ```
    ///
    /// 그림 시작 번호
    pub picture: xs::PositiveInteger32,
    /// ```xml
    /// tbl="{xs:positiveInteger}"
    /// ```
    ///
    /// 표 시작 번호
    pub table: xs::PositiveInteger32,
    /// ```xml
    /// equation="{xs:positiveInteger}"
    /// ```
    ///
    /// 수식 시작 번호
    pub equation: xs::PositiveInteger32,
}

impl TryFrom<AnyElement> for BeginNumber {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__BEGIN_NUMBER)?;

        let (page, foot_note, end_note, picture, table, equation) = attributes!(element, "beginNum";
            "page" as page => one xs::PositiveInteger32,
            "footnote" as foot_note => one xs::PositiveInteger32,
            "endnote" as end_note => one xs::PositiveInteger32,
            "pic" as picture => one xs::PositiveInteger32,
            "tbl" as table => one xs::PositiveInteger32,
            "equation" as equation => one xs::PositiveInteger32,
        );

        Ok(Self {
            page,
            foot_note,
            end_note,
            picture,
            table,
            equation,
        })
    }
}

/// ```xml
/// <refList>
///   <fontfaces ...>...</fontfaces>
///   <borderFills ...>...</borderFills>
///   <charProperties ...>...</charProperties>
///   <tabProps ...>...</tabProps>
///   <numberings ...>...</numberings>
///   <bullets ...>...</bullets>
///   <paraProperties ...>...</paraProperties>
///   <styles ...>...</styles>
///   <memoProperties ...>...</memoProperties>
///   <trackChanges ...>...</trackChanges>
///   <trackChangeAuthors ...>...</trackChangeAuthors>
/// </refList>
/// ```
///
/// 매핑 테이블. 본문에서 사용될 각종 데이터를 가지고 있는 엘리먼트.
#[derive(Debug)]
pub struct MappingTableType {
    /// ```xml
    /// <fontfaces itemCnt="{xs:positiveInteger}">
    ///   <fontface ...>...</fontface>
    /// </fontfaces>
    /// ```
    ///
    /// 글꼴 리스트
    pub font_faces: NonEmpty<FontFaceType>,
    /// ```xml
    /// <borderFills itemCnt="{xs:positiveInteger}">
    ///   <borderFill ...>...</borderFill>
    /// </borderFills>
    /// ```
    ///
    /// 테두리/배경/채우기
    pub border_fills: Vec<BorderFillType>,
    /// ```xml
    /// <charProperties itemCnt="{xs:positiveInteger}">
    ///   <charPr ...>...</charPr>
    /// </charProperties>
    /// ```
    ///
    /// 글자 모양 정보
    pub character_properties: NonEmpty<CharShapeType>,
    /// ```xml
    /// <tabProperties itemCnt="{xs:positiveInteger}">
    ///   <tabPr ...>...</tabPr>
    /// </tabProperties>
    /// ```
    ///
    /// 탭 정의 정보
    pub tab_properties: Vec<TabDefType>,
    /// ```xml
    /// <numberings itemCnt="{xs:positiveInteger}">
    ///   <numbering ...>...</numbering>
    /// </numberings>
    /// ```
    ///
    /// 번호 문단 정보
    pub numberings: Vec<NumberingType>,
    /// ```xml
    /// <bullets itemCnt="{xs:positiveInteger}">
    ///   <bullet ...>...</bullet>
    /// </bullets>
    /// ```
    ///
    /// 글머리표 문단 정보
    pub bullets: Vec<BulletType>,
    /// ```xml
    /// <paraProperties itemCnt="{xs:positiveInteger}">
    ///   <paraShape ...>...</paraShape>
    /// </paraProperties>
    /// ```
    ///
    /// 문단 모양
    pub paragraph_properties: NonEmpty<ParaShapeType>,
    /// ```xml
    /// <styles itemCnt="{xs:positiveInteger}">
    ///   <style ...>...</style>
    /// </styles>
    /// ```
    ///
    /// 스타일
    pub styles: Vec<StyleType>,
    /// ```xml
    /// <memoProperties itemCnt="{xs:positiveInteger}">
    ///   <memo ...>...</memo>
    /// </memoProperties>
    /// ```
    ///
    /// 메모 모양
    pub memo: Vec<MemoShapeType>,
    /// ```xml
    /// <trackChanges itemCnt="{xs:positiveInteger}">
    ///   <trackChange ...>...</trackChange>
    /// </trackChanges>
    /// ```
    ///
    /// 변경 추적
    pub track_changes: Vec<TrackChange>,
    /// ```xml
    /// <trackChangeAuthors itemCnt="{xs:positiveInteger}">
    ///   <trackChangeAuthor ...>...</trackChangeAuthor>
    /// </trackChangeAuthors>
    /// ```
    ///
    /// 변경 추적 작성자
    pub track_change_authors: Vec<TrackChangeAuthor>,
}

impl TryFrom<AnyElement> for MappingTableType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__REFERENCES)?;

        let (
            font_faces,
            border_fills,
            character_properties,
            tab_properties,
            numberings,
            bullets,
            paragraph_properties,
            styles,
            memo,
            track_changes,
            track_change_authors,
        ) = children! {
            element;
            nested_nonempty HANCOM__HEAD__FONT_FACES, FontFaceType;
            nested_opt HANCOM__HEAD__BORDER_FILLS, BorderFillType;
            nested_nonempty HANCOM__HEAD__CHARACTER_PROPERTIES, CharShapeType;
            nested_opt HANCOM__HEAD__TAB_PROPERTIES, TabDefType;
            nested_opt HANCOM__HEAD__NUMBERING, NumberingType;
            nested HANCOM__HEAD__BULLETS, BulletType;
            nested_nonempty HANCOM__HEAD__PARAGRAPH_PROPERTIES, ParaShapeType;
            nested HANCOM__HEAD__STYLES, StyleType;
            nested_opt HANCOM__HEAD__MEMO_PROPERTIES, MemoShapeType;
            nested_opt HANCOM__HEAD__TRACK_CHANGES, TrackChange;
            nested_opt HANCOM__HEAD__TRACK_CHANGE_AUTHORS, TrackChangeAuthor;
        };

        Ok(Self {
            font_faces,
            border_fills,
            character_properties,
            tab_properties,
            numberings,
            bullets,
            paragraph_properties,
            styles,
            memo,
            track_changes,
            track_change_authors,
        })
    }
}

/// ```xml
/// <fontface
///   lang="{xs:string}"
///   itemCnt="{xs:positiveInteger > 0}"
/// >
///   <font ...>...</font>
///   ...
/// </fontface>
/// ```
///
/// 언어별 글꼴 그룹
#[derive(Debug)]
pub struct FontFaceType {
    /// ```xml
    /// lang="{$Language}"
    /// ```
    ///
    /// 언어(한글, 영어, 한자, 일어, 기타, 심볼, 사용자)
    pub language: arbitrary::Language,
    /// ```xml
    /// <font ...>...</font>
    /// ```
    ///
    /// 글꼴 리스트
    pub fonts: NonEmpty<Font>,
}

impl TryFrom<AnyElement> for FontFaceType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__FONT_FACE)?;

        let language = attributes!(element, "fontFace";
            "lang" as language => one arbitrary::Language,
        );

        let fonts = children! {
            element;
            nonempty HANCOM__HEAD__FONT, Font
        };

        Ok(Self { language, fonts })
    }
}

/// ```xml
/// <font
///   id="{xs:nonNegativeInteger}"
///   face="{xs:string}"
///   type="{$FontKind}"
///   isEmbedded="{xs:boolean; default="false"}"
///   binaryItemIDRef="{xs:string}"
/// >
///   <substFont ...>...</substFont>
///   ...
///   <typeInfo ...>...</typeInfo>
///   ...
/// </font>
/// ```
///
/// 글꼴
#[derive(Debug)]
pub struct Font {
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 글꼴 아이디
    pub id: xs::NonNegativeInteger32,
    /// ```xml
    /// face="{xs:string}"
    /// ```
    ///
    /// 글꼴 이름
    pub face: xs::String,
    /// ```xml
    /// type="{$FontKind}"
    /// ```
    ///
    /// 글꼴의 유형(rep: 대표글꼴, ttf: 트루타입글꼴, hft: 한/글 전용 글꼴)
    pub r#type: arbitrary::FontKind,
    /// ```xml
    /// isEmbedded="{xs:boolean; default="false"}"
    /// ```
    pub embedded: xs::Boolean,
    /// ```xml
    /// binaryItemIDRef="{xs:string}"
    /// ```
    pub binary_item_id_ref: Option<xs::String>,
    /// ```xml
    /// <substFont ...>...</substFont>
    /// ```
    pub subset: Option<SubsetFont>,
    /// ```xml
    /// <typeInfo ...>...</typeInfo>
    /// ```
    pub type_info: Option<TypeInfo>,
}

impl TryFrom<AnyElement> for Font {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__FONT)?;

        let (id, face, r#type, embedded, binary_item_id_ref) = attributes!(element, "font";
            "id" as id => one xs::NonNegativeInteger32,
            "face" as face => one (string),
            "type" as r#type => one arbitrary::FontKind,
            "embedded" as embedded => default false; boolean,
            "binaryItemIDRef" as binary_item_id_ref => opt (string),
        );

        let (subset, type_info) = children!(element;
            opt HANCOM__HEAD__SUBSET_FONT, SubsetFont;
            opt HANCOM__HEAD__TYPE_INFO, TypeInfo;
        );

        Ok(Self {
            id,
            face,
            r#type,
            embedded,
            binary_item_id_ref,
            subset,
            type_info,
        })
    }
}

/// ```xml
/// <substFont
///   face="{xs:string}"
///   type="{xs:string}"
///   isEmbedded="{xs:boolean; default="false"}"
///   binaryItemIDRef="{xs:string}"
/// />
/// ```
///
/// 대체 글꼴
#[derive(Debug)]
pub struct SubsetFont {
    /// ```xml
    /// face="{xs:string}"
    /// ```
    ///
    /// 글꼴 이름
    pub face: xs::String,
    /// ```xml
    /// type="{xs:string}"
    /// ```
    ///
    /// 글꼴의 유형
    pub r#type: arbitrary::FontKind,
    /// ```xml
    /// isEmbedded="{xs:boolean; default="false"}"
    /// ```
    pub embedded: xs::Boolean,
    /// ```xml
    /// binaryItemIDRef="{xs:string}"
    /// ```
    pub binary_item_id_ref: Option<xs::String>,
}

impl TryFrom<AnyElement> for SubsetFont {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__SUBSET_FONT)?;

        let (face, r#type, embedded, binary_item_id_ref) = attributes!(element, "substFont";
            "face" as face => one (string),
            "type" as r#type => one arbitrary::FontKind,
            "embedded" as embedded => default false; boolean,
            "binaryItemIDRef" as binary_item_id_ref => opt (string),
        );

        Ok(Self {
            face,
            r#type,
            embedded,
            binary_item_id_ref,
        })
    }
}

/// ```xml
/// <typeInfo
///   familyType="{$FamilyType}"
///   serifStyle="{xs:string}"
///   weight="{xs:integer}"
///   proportion="{xs:integer}"
///   contrast="{xs:integer}"
///   strokeVariation="{xs:integer}"
///   armStyle="{xs:boolean}"
///   letterForm="{xs:boolean}"
///   midline="{xs:boolean}"
///   xHeight="{xs:integer}"
/// />
/// ```
///
/// 글꼴 정보
#[derive(Debug)]
pub struct TypeInfo {
    /// ```xml
    /// familyType="{$FamilyType}"
    /// ```
    ///
    /// 글꼴 계열
    pub family_type: arbitrary::FamilyType,
    /// ```xml
    /// serifStyle="{xs:string}"
    /// ```
    ///
    /// 세리프 유형
    pub serif_style: Option<xs::String>,
    /// ```xml
    /// weight="{xs:integer}"
    /// ```
    ///
    /// 굵기
    pub weight: xs::Integer32,
    /// ```xml
    /// proportion="{xs:integer}"
    /// ```
    ///
    /// 비율
    pub proportion: xs::Integer32,
    /// ```xml
    /// contrast="{xs:integer}"
    /// ```
    ///
    /// 대조
    pub contrast: xs::Integer32,
    /// ```xml
    /// strokeVariation="{xs:integer}"
    /// ```
    ///
    /// 스트로크 편차
    pub stroke_variation: xs::Integer32,
    /// ```xml
    /// armStyle="{xs:boolean}"
    /// ```
    ///
    /// 자획모양
    pub arm_style: xs::Boolean,
    /// ```xml
    /// letterForm="{xs:boolean}"
    /// ```
    ///
    /// 글자형
    pub letter_form: xs::Boolean,
    /// ```xml
    /// midline="{xs:boolean}"
    /// ```
    ///
    /// 중간선
    pub midline: xs::Boolean,
    /// ```xml
    /// xHeight="{xs:integer}"
    /// ```
    ///
    /// X-높이
    pub x_height: xs::Integer32,
}

impl TryFrom<AnyElement> for TypeInfo {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TYPE_INFO)?;

        let (
            family_type,
            serif_style,
            weight,
            proportion,
            contrast,
            stroke_variation,
            arm_style,
            letter_form,
            midline,
            x_height,
        ) = attributes!(element, "typeInfo";
            "familyType" as family_type => one arbitrary::FamilyType,
            "serifStyle" as serif_style => opt (string),
            "weight" as weight => one xs::Integer32,
            "proportion" as proportion => one xs::Integer32,
            "contrast" as contrast => one xs::Integer32,
            "strokeVariation" as stroke_variation => one xs::Integer32,
            "armStyle" as arm_style => one (boolean),
            "letterform" as letter_form => one (boolean),
            "midline" as midline => one (boolean),
            "xHeight" as x_height => one xs::Integer32,
        );

        Ok(Self {
            family_type,
            serif_style,
            weight,
            proportion,
            contrast,
            stroke_variation,
            arm_style,
            letter_form,
            midline,
            x_height,
        })
    }
}

/// ```xml
/// <borderFill
///   id="{xs:nonNegativeInteger}"
///   effect3D="{xs:boolean; default="false"}"
///   shadow="{xs:boolean; default="false"}"
///   centerLine="{xs:boolean; default="false"}"
///   breakCellSeparateLine="{xs:boolean; default="false"}"
/// >
///   <slash ...>...</slash>
///   <backSlash ...>...</backSlash>
///   <leftBorder ...>...</leftBorder>
///   <rightBorder ...>...</rightBorder>
///   <topBorder ...>...</topBorder>
///   <bottomBorder ...>...</bottomBorder>
///   <diagonal ...>...</diagonal>
///   <fillBrush ...>...</fillBrush>
///   ...
/// </borderFill>
/// ```
///
/// 테두리/배경/채우기
#[derive(Debug)]
pub struct BorderFillType {
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 테두리/채우기 항목 아이디
    pub id: xs::NonNegativeInteger32,
    /// ```xml
    /// effect3D="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 3D효과 on/off
    pub effect_3d: xs::Boolean,
    /// ```xml
    /// shadow="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 그림자 효과 on/off
    pub shadow: xs::Boolean,
    /// ```xml
    /// centerLine="{$CenterLine}"
    /// ```
    ///
    /// 중심선 종류
    pub center_line: arbitrary::CenterLine,
    /// ```xml
    /// breakCellSeparateLine="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 자동으로 나뉜 표의 경계선 설정 여부
    pub break_cell_separate_line: xs::Boolean,
    /// ```xml
    /// <slash ...>...</slash>
    /// ```
    pub slash: Option<Slash>,
    /// ```xml
    /// <backSlash ...>...</backSlash>
    /// ```
    pub back_slash: Option<Slash>,
    /// ```xml
    /// <leftBorder ...>...</leftBorder>
    /// ```
    pub left_border: Option<BorderType>,
    /// ```xml
    /// <rightBorder ...>...</rightBorder>
    /// ```
    pub right_border: Option<BorderType>,
    /// ```xml
    /// <topBorder ...>...</topBorder>
    /// ```
    pub top_border: Option<BorderType>,
    /// ```xml
    /// <bottomBorder ...>...</bottomBorder>
    /// ```
    pub bottom_border: Option<BorderType>,
    /// ```xml
    /// <diagonal ...>...</diagonal>
    /// ```
    pub diagonal: Option<BorderType>,
    /// ```xml
    /// <fillBrush>
    ///   <!-- one of the following -->
    ///   <winBrush ... />
    ///   <gradation ... />
    ///   <imageBrush ... />
    /// </fillBrush>
    /// ```
    ///
    /// 채우기 정보
    pub fill_brush: Option<FillBrush>,
}

impl TryFrom<AnyElement> for BorderFillType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__BORDER_FILL)?;

        let (id, effect_3d, shadow, center_line, break_cell_separate_line) = attributes!(element, "borderFill";
            "id" as id => one xs::NonNegativeInteger32,
            "effect3D" as effect_3d => default false; boolean,
            "shadow" as shadow => default false; boolean,
            "centerLine" as center_line => one arbitrary::CenterLine,
            "breakCellSeparateLine" as break_cell_separate_line => default false; boolean,
        );

        let (
            slash,
            back_slash,
            left_border,
            right_border,
            top_border,
            bottom_border,
            diagonal,
            fill_brush,
        ) = children!(element;
            opt HANCOM__HEAD__SLASH, Slash;
            opt HANCOM__HEAD__BACK_SLASH, Slash;
            opt HANCOM__HEAD__LEFT_BORDER, BorderType;
            opt HANCOM__HEAD__RIGHT_BORDER, BorderType;
            opt HANCOM__HEAD__TOP_BORDER, BorderType;
            opt HANCOM__HEAD__BOTTOM_BORDER, BorderType;
            opt HANCOM__HEAD__DIAGONAL, BorderType;
            opt HANCOM__CORE__FILL_BRUSH, FillBrush;
        );

        Ok(Self {
            id,
            effect_3d,
            shadow,
            center_line,
            break_cell_separate_line,
            slash,
            back_slash,
            left_border,
            right_border,
            top_border,
            bottom_border,
            diagonal,
            fill_brush,
        })
    }
}

/// ```xml
/// <slash
///   type="{$SlashKind}"
///   Crooked="{xs:boolean}"
///   isCounter="{xs:boolean}"
/// />
/// ```
#[derive(Debug)]
pub struct Slash {
    /// ```xml
    /// type="{$SlashKind}"
    /// ```
    pub r#type: arbitrary::SlashKind,
    /// ```xml
    /// Crooked="{xs:boolean}"
    /// ```
    pub crooked: xs::Boolean,
    /// ```xml
    /// isCounter="{xs:boolean}"
    /// ```
    pub counter: xs::Boolean,
}

impl TryFrom<AnyElement> for Slash {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        let (r#type, crooked, counter) = attributes!(element, "slash";
            "type" as r#type => one arbitrary::SlashKind,
            "Crooked" as crooked => one (boolean),
            "isCounter" as counter => one (boolean),
        );

        Ok(Self {
            r#type,
            crooked,
            counter,
        })
    }
}

/// ```xml
/// <border
///   type="{hc:LineType2}"
///   width="{hc:LineWidth}"
///   color="{hc:RGBColorType}"
/// />
///
/// 테두리 형식
#[derive(Debug)]
pub struct BorderType {
    /// ```xml
    /// type="{hc:LineType2}"
    /// ```
    ///
    /// 테두리선 종류
    pub r#type: core::LineType2,
    /// ```xml
    /// width="{hc:LineWidth}"
    /// ```
    ///
    /// 테두리선 굵기
    pub width: core::LineWidth,
    /// ```xml
    /// color="{hc:RGBColorType}"
    /// ```
    ///
    /// 테두리선 색상
    pub color: core::RgbColorType,
}

impl TryFrom<AnyElement> for BorderType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        let (r#type, width, color) = attributes!(element, "border";
            "type" as r#type => one core::LineType2,
            "width" as width => one core::LineWidth,
            "color" as color => one core::RgbColorType,
        );

        Ok(Self {
            r#type,
            width,
            color,
        })
    }
}

#[derive(Debug)]
pub enum FillBrush {
    WinBrush(WinBrush),
    Gradation(Gradation),
    Image(ImageBrush),
}

impl TryFrom<AnyElement> for FillBrush {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__CORE__FILL_BRUSH)?;

        for child in element.children {
            match child.name {
                ElementName::HANCOM__CORE__WIN_BRUSH => {
                    return Ok(Self::WinBrush(child.try_into()?));
                }
                ElementName::HANCOM__CORE__GRADATION => {
                    return Ok(Self::Gradation(child.try_into()?));
                }
                ElementName::HANCOM__CORE__IMAGE_BRUSH => {
                    return Ok(Self::Image(child.try_into()?));
                }
                _ => continue,
            }
        }

        missing_element!("<fillBrush winBrush | gradation | image>")
    }
}

/// ```xml
/// <winBrush
///   faceColor="{hc:RGBColorType}"
///   hatchColor="{hc:RGBColorType}"
///   hatchStyle="{xs:string}"
///   alpha="{xs:unsignedByte}"
/// />
/// ```
///
/// 면 채우기
#[derive(Debug)]
pub struct WinBrush {
    /// ```xml
    /// faceColor="{hc:RGBColorType; default="#FFFFFF"}"
    /// ```
    ///
    /// 면 색
    pub face_color: core::RgbColorType,
    /// ```xml
    /// hatchColor="{hc:RGBColorType; default="#000000"}"
    /// ```
    ///
    /// 무늬 색
    pub hatch_color: core::RgbColorType,
    /// ```xml
    /// hatchStyle="{xs:string}"
    /// ```
    ///
    /// 무늬 종류
    pub hatch_style: Option<arbitrary::HatchStyle>,
    /// ```xml
    /// alpha="{xs:float}"
    /// ```
    pub alpha: xs::Float32,
}

impl TryFrom<AnyElement> for WinBrush {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__CORE__WIN_BRUSH)?;

        let (face_color, hatch_color, hatch_style, alpha) = attributes!(element, "winBrush";
            "faceColor" as face_color => one core::RgbColorType,
            "hatchColor" as hatch_color => one core::RgbColorType,
            "hatchStyle" as hatch_style => opt arbitrary::HatchStyle,
            "alpha" as alpha => one xs::Float32,
        );

        Ok(Self {
            face_color,
            hatch_color,
            hatch_style,
            alpha,
        })
    }
}

/// ```xml
/// <gradation
///   type="{$GradationKind}"
///   angle="{xs:integer; default="90"}"
///   centerX="{xs:integer; default="0"}"
///   centerY="{xs:integer; default="0"}"
///   step="{xs:integer; >= 0; <= 255}"
///   colorNum="{xs:nonNegativeInteger}"
///   stepCenter="{xs:integer; >= 0; <= 10}"
///   alpha="{xs:float}"
/// >
///   <color value="{hc:RGBColorType}" />
/// </gradation>
/// ```
///
/// 그러데이션 효과
#[derive(Debug)]
pub struct Gradation {
    /// ```xml
    /// type="{$GradationKind}"
    /// ```
    ///
    /// 그러데이션 유형
    pub r#type: arbitrary::GradationKind,
    /// ```xml
    /// angle="{xs:integer; default="90"}"
    /// ```
    ///
    /// 그러데이션의 기울임(시작각)
    pub angle: xs::Integer32,
    /// ```xml
    /// centerX="{xs:integer; default="0"}"
    /// ```
    ///
    /// 그러데이션의 가로중심(중심 X좌표)
    pub center_x: xs::Integer32,
    /// ```xml
    /// centerY="{xs:integer; default="0"}"
    /// ```
    ///
    /// 그러데이션의 세로중심(중심 Y좌표)
    pub center_y: xs::Integer32,
    /// ```xml
    /// step="{xs:integer; >= 0; <= 255}"
    /// ```
    ///
    /// 그러데이션 번짐정도 (0~255)
    pub step: xs::Integer8,
    /// ```xml
    /// colorNum="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 그러데이션의 개수
    pub color_number: xs::NonNegativeInteger8,
    /// ```xml
    /// stepCenter="{xs:integer; >= 0; <= 100}"
    /// ```
    ///
    /// 그러데이션 번짐정도의 중심 (0~100)
    pub step_center: xs::Integer8,
    /// ```xml
    /// alpha="{xs:float}"
    /// ```
    pub alpha: xs::Float32,
    /// ```xml
    /// <color value="{hc:RGBColorType}" />
    /// ```
    ///
    /// 그러데이션 색
    pub colors: Vec<GradationColor>,
}

impl TryFrom<AnyElement> for Gradation {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__CORE__GRADATION)?;

        let (r#type, angle, center_x, center_y, step, color_number, step_center, alpha) = attributes!(element, "gradation";
            "type" as r#type => one arbitrary::GradationKind,
            "angle" as angle => default 90,
            "centerX" as center_x => one xs::Integer32,
            "centerY" as center_y => one xs::Integer32,
            "step" as step => one xs::Integer8,
            "colorNum" as color_number => one xs::NonNegativeInteger8,
            "stepCenter" as step_center => one xs::Integer8,
            "alpha" as alpha => one xs::Float32,
        );

        let colors = children!(element;
            many HANCOM__HEAD__COLOR, GradationColor;
        );

        Ok(Self {
            r#type,
            angle,
            center_x,
            center_y,
            step,
            color_number,
            step_center,
            alpha,
            colors,
        })
    }
}

/// ```xml
/// <color value="{hc:RGBColorType}" />
/// ```
///
/// 그러데이션 색
#[derive(Debug)]
pub struct GradationColor {
    /// ```xml
    /// value="{hc:RGBColorType}"
    /// ```
    ///
    /// 색상 값
    pub value: core::RgbColorType,
}

impl TryFrom<AnyElement> for GradationColor {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__COLOR)?;

        let value = attributes!(element, "color";
            "value" as value => one core::RgbColorType,
        );

        Ok(Self { value })
    }
}

/// ```xml
/// <imgBrush
///   mode="{xs:string; default="TILE"}"
/// >
///   <img ...>...</img>
/// </imgBrush>
/// ```
///
/// 그림으로 채우기
#[derive(Debug)]
pub struct ImageBrush {
    /// ```xml
    /// mode="{xs:string; default="TILE"}"
    /// ```
    ///
    /// 채우기 유형
    pub mode: arbitrary::ImageBrushMode,
    /// ```xml
    /// <img ...>...</img>
    /// ```
    pub image: ImageType,
}

impl TryFrom<AnyElement> for ImageBrush {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__CORE__IMAGE_BRUSH)?;

        let mode = attributes!(element, "imageBrush";
            "mode" as mode => one arbitrary::ImageBrushMode,
        );

        let image = children!(element;
            one HANCOM__CORE__IMAGE, ImageType;
        );

        Ok(Self { mode, image })
    }
}

/// ```xml
/// <img
///   binaryIDRef="{xs:string}"
///   bright="{xs:integer; default="0"}"
///   contrast="{xs:integer; default="0"}"
///   effect="{$ImageEffect; default="REALPIC"}"
///   alpha="{xs:float}"
/// />
/// ```
///
/// 그림 정보
#[derive(Debug)]
pub struct ImageType {
    /// ```xml
    /// binaryIDRef="{xs:string}"
    /// ```
    ///
    /// manifest의 item 엘리먼트의 아이디 참조 값
    pub binary_id_ref: xs::String,
    /// ```xml
    /// bright="{xs:integer; default="0"}"
    /// ```
    ///
    /// 밝기
    pub bright: xs::Integer32,
    /// ```xml
    /// contrast="{xs:integer; default="0"}"
    /// ```
    ///
    /// 대비
    pub contrast: xs::Integer32,
    /// ```xml
    /// effect="{$ImageEffect; default="REALPIC"}"
    /// ```
    ///
    /// 효과
    pub effect: arbitrary::ImageEffect,
    /// ```xml
    /// alpha="{xs:float}"
    /// ```
    pub alpha: xs::Float32,
}

impl TryFrom<AnyElement> for ImageType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__CORE__IMAGE)?;

        let (binary_id_ref, bright, contrast, effect, alpha) = attributes!(element, "img";
            "binaryItemIDRef" as binary_id_ref => one (string),
            "bright" as bright => default 0,
            "contrast" as contrast => default 0,
            "effect" as effect => default arbitrary::ImageEffect::RealPicture,
            "alpha" as alpha => one xs::Float32,
        );

        Ok(Self {
            binary_id_ref,
            bright,
            contrast,
            effect,
            alpha,
        })
    }
}

/// ```xml
/// <charShape
///   id="{xs:nonNegativeInteger}"
///   height="{xs:integer; default="1000"}"
///   textColor="{hc:RGBColorType; default="#000000"}"
///   shadeColor="{hc:RGBColorType; default="#FFFFFF"}"
///   useFontSpace="{xs:boolean; default="false"}"
///   useKerning="{xs:boolean; default="false"}"
///   symMark="{$SymbolMark; default="None"}"
///   borderFillIDRef="{xs:nonNegativeInteger}"
/// >
///   <fontRef>...</fontRef>
///   <ratio>...</ratio>
///   <spacing>...</spacing>
///   <relSz>...</relSz>
///   <offset>...</offset>
///   <italic>...</italic>
///   <bold>...</bold>
///   <underline>...</underline>
///   <strikeout>...</strikeout>
///   <outline>...</outline>
///   <shadow>...</shadow>
///   <emboss>...</emboss>
///   <engrave>...</engrave>
///   <supscript>...</supscript>
///   <subscript>...</subscript>
/// </charShape>
/// ```
///
/// 글자 모양
#[derive(Debug)]
pub struct CharShapeType {
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 글자 모양 아이디
    pub id: xs::NonNegativeInteger32,
    /// ```xml
    /// height="{xs:integer; default="1000"}"
    /// ```
    ///
    /// 글자 크기 (hwpunit 단위, 10 pt = 1000 hwpunit)
    pub height: xs::Integer32,
    /// ```xml
    /// textColor="{hc:RGBColorType; default="#000000"}"
    /// ```
    ///
    /// 글자색
    pub text_color: core::RgbColorType,
    /// ```xml
    /// shadeColor="{hc:RGBColorType; default="#FFFFFF"}"
    /// ```
    ///
    /// 음영색
    pub shade_color: core::RgbColorType,
    /// ```xml
    /// useFontSpace="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 글꼴에 어울리는 빈칸
    pub use_font_space: xs::Boolean,
    /// ```xml
    /// useKerning="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 커닝
    pub use_kerning: xs::Boolean,
    /// ```xml
    /// symMark="{$SymbolMark; default="None"}"
    /// ```
    ///
    /// 강조점 종류
    pub symbol_mark: arbitrary::SymbolMark,
    /// ```xml
    /// borderFillIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 글자테두리 기능
    pub border_fill_id_ref: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// <fontRef
    ///   hangul="{xs:nonNegativeInteger}"
    ///   latin="{xs:nonNegativeInteger}"
    ///   hanja="{xs:nonNegativeInteger}"
    ///   japanese="{xs:nonNegativeInteger}"
    ///   other="{xs:nonNegativeInteger}"
    ///   symbol="{xs:nonNegativeInteger}"
    ///   user="{xs:nonNegativeInteger}"
    /// />
    /// ```
    ///
    /// 언어별 글꼴. 각 글꼴 타입에 맞는(한글이면 한글글꼴 타입) 참조 글꼴 ID를 기술.
    pub font_ref: FontReference,
    /// ```xml
    /// <ratio>...</ratio>
    /// ```
    ///
    /// 언어별 장평. 단위는 %.
    pub ratio: Ratio,
    /// ```xml
    /// <spacing>...</spacing>
    /// ```
    ///
    /// 언어별 자간. 단위는 %.
    pub spacing: Spacing,
    /// ```xml
    /// <relSz>...</relSz>
    /// ```
    ///
    /// 언어별 글자의 상대 크기. 단위는 %.
    pub relative_size: RelativeSize,
    /// ```xml
    /// <offset>...</offset>
    /// ```
    ///
    /// 언어별 오프셋. 단위는 %.
    pub offset: Offset,
    /// ```xml
    /// <italic />
    /// ```
    ///
    /// 글자 속성: 기울임.
    pub italic: xs::Boolean,
    /// ```xml
    /// <bold />
    /// ```
    ///
    /// 글자 속성: 진하게.
    pub bold: xs::Boolean,
    /// ```xml
    /// <underline
    ///   type="{$UnderlineKind}"
    ///   shape="{hc:LineType2}"
    ///   color="{hc:RGBColorType}"
    /// />
    /// ```
    ///
    /// 글자 속성: 밑줄.
    pub underline: Option<Underline>,
    /// ```xml
    /// <strikeout
    ///   type="{$StrikeoutKind}"
    ///   shape="{hc:LineType2}"
    ///   color="{hc:RGBColorType}"
    /// />
    /// ```
    ///
    /// 글자 속성: 취소선.
    pub strikeout: Option<Strikeout>,
    /// ```xml
    /// <outline>...</outline>
    /// ```
    ///
    /// 글자 속성: 외곽선.
    pub outline: Option<Outline>,
    /// ```xml
    /// <shadow
    ///   type="{$ShadowKind}"
    ///   color="{hc:RGBColorType}"
    ///   offsetX="{xs:integer; >= -100; <= 100}"
    ///   offsetY="{xs:integer; >= -100; <= 100}"
    /// />
    /// ```
    ///
    /// 글자 속성: 그림자.
    pub shadow: Option<Shadow>,
    /// ```xml
    /// <emboss />
    /// ```
    ///
    /// 글자 속성: 양각.
    pub emboss: xs::Boolean,
    /// ```xml
    /// <engrave />
    /// ```
    ///
    /// 글자 속성: 음각.
    pub engrave: xs::Boolean,
    /// ```xml
    /// <supscript />
    /// ```
    ///
    /// 글자 속성: 위첨자.
    pub superscript: xs::Boolean,
    /// ```xml
    /// <subscript />
    /// ```
    ///
    /// 글자 속성: 아래첨자.
    pub subscript: xs::Boolean,
}

impl TryFrom<AnyElement> for CharShapeType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__CHARACTER_PROPERTY)?;

        let (
            id,
            height,
            text_color,
            shade_color,
            use_font_space,
            use_kerning,
            symbol_mark,
            border_fill_id_ref,
        ) = attributes!(element, "charPr";
            "id" as id => one xs::NonNegativeInteger32,
            "height" as height => default 1000,
            "textColor" as text_color => default core::RgbColorType(Some((0, 0, 0))),
            "shadeColor" as shade_color => default core::RgbColorType(Some((255, 255, 255))),
            "useFontSpace" as use_font_space => default false; boolean,
            "useKerning" as use_kerning => default false; boolean,
            "symMark" as symbol_mark => default arbitrary::SymbolMark::None,
            "borderFillIDRef" as border_fill_id_ref => opt xs::NonNegativeInteger32,
        );

        let mut font_ref = None;
        let mut ratio = None;
        let mut spacing = None;
        let mut relative_size = None;
        let mut offset = None;
        let mut italic = false;
        let mut bold = false;
        let mut underline = None;
        let mut strikeout = None;
        let mut outline = None;
        let mut shadow = None;
        let mut emboss = false;
        let mut engrave = false;
        let mut superscript = false;
        let mut subscript = false;

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__FONT_REFERENCE => font_ref = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__RATIO => ratio = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__SPACING => spacing = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__RELATIVE_SIZE => relative_size = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__OFFSET => offset = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__ITALIC => italic = true,
                ElementName::HANCOM__HEAD__BOLD => bold = true,
                ElementName::HANCOM__HEAD__UNDERLINE => underline = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__STRIKEOUT => strikeout = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__OUTLINE => outline = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__SHADOW => shadow = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__EMBOSS => emboss = true,
                ElementName::HANCOM__HEAD__ENGRAVE => engrave = true,
                ElementName::HANCOM__HEAD__SUPERSCRIPT => superscript = true,
                ElementName::HANCOM__HEAD__SUBSCRIPT => subscript = true,
                _ => continue,
            }
        }

        let (font_ref, ratio, spacing, relative_size, offset) =
            match (font_ref, ratio, spacing, relative_size, offset) {
                (Some(font_ref), Some(ratio), Some(spacing), Some(relative_size), Some(offset)) => {
                    (font_ref, ratio, spacing, relative_size, offset)
                }
                (None, _, _, _, _) => missing_element!("<fontRef>"),
                (_, None, _, _, _) => missing_element!("<ratio>"),
                (_, _, None, _, _) => missing_element!("<spacing>"),
                (_, _, _, None, _) => missing_element!("<relSz>"),
                (_, _, _, _, None) => missing_element!("<offset>"),
            };

        Ok(Self {
            id,
            height,
            text_color,
            shade_color,
            use_font_space,
            use_kerning,
            symbol_mark,
            border_fill_id_ref,
            font_ref,
            ratio,
            spacing,
            relative_size,
            offset,
            italic,
            bold,
            underline,
            strikeout,
            outline,
            shadow,
            emboss,
            engrave,
            superscript,
            subscript,
        })
    }
}

/// ```xml
/// <fontRef
///   hangul="{xs:nonNegativeInteger}"
///   latin="{xs:nonNegativeInteger}"
///   hanja="{xs:nonNegativeInteger}"
///   japanese="{xs:nonNegativeInteger}"
///   other="{xs:nonNegativeInteger}"
///   symbol="{xs:nonNegativeInteger}"
///   user="{xs:nonNegativeInteger}"
/// />
/// ```
///
/// 언어별 글꼴. 각 글꼴 타입에 맞는(한글이면 한글글꼴 타입) 참조 글꼴 ID를 기술.
#[derive(Debug)]
pub struct FontReference {
    /// ```xml
    /// hangul="{xs:nonNegativeInteger}"
    /// ```
    pub hangul: xs::NonNegativeInteger32,
    /// ```xml
    /// latin="{xs:nonNegativeInteger}"
    /// ```
    pub latin: xs::NonNegativeInteger32,
    /// ```xml
    /// hanja="{xs:nonNegativeInteger}"
    /// ```
    pub hanja: xs::NonNegativeInteger32,
    /// ```xml
    /// japanese="{xs:nonNegativeInteger}"
    /// ```
    pub japanese: xs::NonNegativeInteger32,
    /// ```xml
    /// other="{xs:nonNegativeInteger}"
    /// ```
    pub other: xs::NonNegativeInteger32,
    /// ```xml
    /// symbol="{xs:nonNegativeInteger}"
    /// ```
    pub symbol: xs::NonNegativeInteger32,
    /// ```xml
    /// user="{xs:nonNegativeInteger}"
    /// ```
    pub user: xs::NonNegativeInteger32,
}

impl TryFrom<AnyElement> for FontReference {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__FONT_REFERENCE)?;

        let (hangul, latin, hanja, japanese, other, symbol, user) = attributes!(element, "fontRef";
            "hangul" as hangul => one xs::NonNegativeInteger32,
            "latin" as latin => one xs::NonNegativeInteger32,
            "hanja" as hanja => one xs::NonNegativeInteger32,
            "japanese" as japanese => one xs::NonNegativeInteger32,
            "other" as other => one xs::NonNegativeInteger32,
            "symbol" as symbol => one xs::NonNegativeInteger32,
            "user" as user => one xs::NonNegativeInteger32,
        );

        Ok(Self {
            hangul,
            latin,
            hanja,
            japanese,
            other,
            symbol,
            user,
        })
    }
}

/// ```xml
/// <ratio
///   hangul="{xs:positiveInteger; >= 50; <= 200; default="100"}"
///   latin="{xs:positiveInteger; >= 50; <= 200; default="100"}"
///   hanja="{xs:positiveInteger; >= 50; <= 200; default="100"}"
///   japanese="{xs:positiveInteger; >= 50; <= 200; default="100"}"
///   other="{xs:positiveInteger; >= 50; <= 200; default="100"}"
///   symbol="{xs:positiveInteger; >= 50; <= 200; default="100"}"
///   user="{xs:positiveInteger; >= 50; <= 200; default="100"}"
/// />
/// ```
///
/// 언어별 장평. 단위는 %.
#[derive(Debug)]
pub struct Ratio {
    /// ```xml
    /// hangul="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub hangul: xs::PositiveInteger8,
    /// ```xml
    /// latin="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub latin: xs::PositiveInteger8,
    /// ```xml
    /// hanja="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub hanja: xs::PositiveInteger8,
    /// ```xml
    /// japanese="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub japanese: xs::PositiveInteger8,
    /// ```xml
    /// other="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub other: xs::PositiveInteger8,
    /// ```xml
    /// symbol="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub symbol: xs::PositiveInteger8,
    /// ```xml
    /// user="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub user: xs::PositiveInteger8,
}

impl TryFrom<AnyElement> for Ratio {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__RATIO)?;

        let (hangul, latin, hanja, japanese, other, symbol, user) = attributes!(element, "ratio";
            "hangul" as hangul => one xs::PositiveInteger8,
            "latin" as latin => one xs::PositiveInteger8,
            "hanja" as hanja => one xs::PositiveInteger8,
            "japanese" as japanese => one xs::PositiveInteger8,
            "other" as other => one xs::PositiveInteger8,
            "symbol" as symbol => one xs::PositiveInteger8,
            "user" as user => one xs::PositiveInteger8,
        );

        Ok(Self {
            hangul,
            latin,
            hanja,
            japanese,
            other,
            symbol,
            user,
        })
    }
}

/// ```xml
/// <spacing
///   hangul="{xs:integer; >= -50; <= 50; default="0"}"
///   latin="{xs:integer; >= -50; <= 50; default="0"}"
///   hanja="{xs:integer; >= -50; <= 50; default="0"}"
///   japanese="{xs:integer; >= -50; <= 50; default="0"}"
///   other="{xs:integer; >= -50; <= 50; default="0"}"
///   symbol="{xs:integer; >= -50; <= 50; default="0"}"
///   user="{xs:integer; >= -50; <= 50; default="0"}"
/// />
///
/// 언어별 자간. 단위는 %.
#[derive(Debug)]
pub struct Spacing {
    /// ```xml
    /// hangul="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub hangul: xs::Integer8,
    /// ```xml
    /// latin="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub latin: xs::Integer8,
    /// ```xml
    /// hanja="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub hanja: xs::Integer8,
    /// ```xml
    /// japanese="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub japanese: xs::Integer8,
    /// ```xml
    /// other="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub other: xs::Integer8,
    /// ```xml
    /// symbol="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub symbol: xs::Integer8,
    /// ```xml
    /// user="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub user: xs::Integer8,
}

impl TryFrom<AnyElement> for Spacing {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__SPACING)?;

        let (hangul, latin, hanja, japanese, other, symbol, user) = attributes!(element, "spacing";
            "hangul" as hangul => one xs::Integer8,
            "latin" as latin => one xs::Integer8,
            "hanja" as hanja => one xs::Integer8,
            "japanese" as japanese => one xs::Integer8,
            "other" as other => one xs::Integer8,
            "symbol" as symbol => one xs::Integer8,
            "user" as user => one xs::Integer8,
        );

        Ok(Self {
            hangul,
            latin,
            hanja,
            japanese,
            other,
            symbol,
            user,
        })
    }
}

/// ```xml
/// <relSz
///   hangul="{xs:positiveInteger; >= 10; <= 250; default="100"}"
///   latin="{xs:positiveInteger; >= 10; <= 250; default="100"}"
///   hanja="{xs:positiveInteger; >= 10; <= 250; default="100"}"
///   japanese="{xs:positiveInteger; >= 10; <= 250; default="100"}"
///   other="{xs:positiveInteger; >= 10; <= 250; default="100"}"
///   symbol="{xs:positiveInteger; >= 10; <= 250; default="100"}"
///   user="{xs:positiveInteger; >= 10; <= 250; default="100"}"
/// />
///
/// 언어별 글자의 상대 크기. 단위는 %.
#[derive(Debug)]
pub struct RelativeSize {
    /// ```xml
    /// hangul="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub hangul: xs::PositiveInteger8,
    /// ```xml
    /// latin="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub latin: xs::PositiveInteger8,
    /// ```xml
    /// hanja="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub hanja: xs::PositiveInteger8,
    /// ```xml
    /// japanese="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub japanese: xs::PositiveInteger8,
    /// ```xml
    /// other="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub other: xs::PositiveInteger8,
    /// ```xml
    /// symbol="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub symbol: xs::PositiveInteger8,
    /// ```xml
    /// user="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub user: xs::PositiveInteger8,
}

impl TryFrom<AnyElement> for RelativeSize {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__RELATIVE_SIZE)?;

        let (hangul, latin, hanja, japanese, other, symbol, user) = attributes!(element, "relSz";
            "hangul" as hangul => one xs::PositiveInteger8,
            "latin" as latin => one xs::PositiveInteger8,
            "hanja" as hanja => one xs::PositiveInteger8,
            "japanese" as japanese => one xs::PositiveInteger8,
            "other" as other => one xs::PositiveInteger8,
            "symbol" as symbol => one xs::PositiveInteger8,
            "user" as user => one xs::PositiveInteger8,
        );

        Ok(Self {
            hangul,
            latin,
            hanja,
            japanese,
            other,
            symbol,
            user,
        })
    }
}

/// ```xml
/// <offset
///   hangul="{xs:integer; >= -100; <= 100; default="0"}"
///   latin="{xs:integer; >= -100; <= 100; default="0"}"
///   hanja="{xs:integer; >= -100; <= 100; default="0"}"
///   japanese="{xs:integer; >= -100; <= 100; default="0"}"
///   other="{xs:integer; >= -100; <= 100; default="0"}"
///   symbol="{xs:integer; >= -100; <= 100; default="0"}"
///   user="{xs:integer; >= -100; <= 100; default="0"}"
/// />
///
/// 언어별 오프셋. 단위는 %.
#[derive(Debug)]
pub struct Offset {
    /// ```xml
    /// hangul="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub hangul: xs::Integer8,
    /// ```xml
    /// latin="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub latin: xs::Integer8,
    /// ```xml
    /// hanja="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub hanja: xs::Integer8,
    /// ```xml
    /// japanese="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub japanese: xs::Integer8,
    /// ```xml
    /// other="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub other: xs::Integer8,
    /// ```xml
    /// symbol="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub symbol: xs::Integer8,
    /// ```xml
    /// user="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub user: xs::Integer8,
}

impl TryFrom<AnyElement> for Offset {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__OFFSET)?;

        let (hangul, latin, hanja, japanese, other, symbol, user) = attributes!(element, "offset";
            "hangul" as hangul => one xs::Integer8,
            "latin" as latin => one xs::Integer8,
            "hanja" as hanja => one xs::Integer8,
            "japanese" as japanese => one xs::Integer8,
            "other" as other => one xs::Integer8,
            "symbol" as symbol => one xs::Integer8,
            "user" as user => one xs::Integer8,
        );

        Ok(Self {
            hangul,
            latin,
            hanja,
            japanese,
            other,
            symbol,
            user,
        })
    }
}

/// ```xml
/// <underline
///   type="{$UnderlineKind}"
///   shape="{hc:LineType2}"
///   color="{hc:RGBColorType}"
/// />
///
/// 글자 속성: 밑줄.
#[derive(Debug)]
pub struct Underline {
    /// ```xml
    /// type="{$UnderlineKind}"
    /// ```
    pub r#type: arbitrary::UnderlineKind,
    /// ```xml
    /// shape="{hc:LineType2}"
    /// ```
    pub shape: core::LineType2,
    /// ```xml
    /// color="{hc:RGBColorType}"
    /// ```
    pub color: core::RgbColorType,
}

impl TryFrom<AnyElement> for Underline {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__UNDERLINE)?;

        let (r#type, shape, color) = attributes!(element, "underline";
            "type" as r#type => one arbitrary::UnderlineKind,
            "shape" as shape => one core::LineType2,
            "color" as color => one core::RgbColorType,
        );

        Ok(Self {
            r#type,
            shape,
            color,
        })
    }
}

/// ```xml
/// <strikeout
///   shape="{hc:LineType2}"
///   color="{hc:RGBColorType}"
/// />
///
/// 글자 속성: 취소선.
#[derive(Debug)]
pub struct Strikeout {
    /// ```xml
    /// shape="{hc:LineType2}"
    /// ```
    pub shape: core::LineType2,
    /// ```xml
    /// color="{hc:RGBColorType}"
    /// ```
    pub color: core::RgbColorType,
}

impl TryFrom<AnyElement> for Strikeout {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__STRIKEOUT)?;

        let (shape, color) = attributes!(element, "strikeout";
            "shape" as shape => one core::LineType2,
            "color" as color => one core::RgbColorType,
        );

        Ok(Self { shape, color })
    }
}

/// ```xml
/// <outline
///   type="{hc:LineType1}"
/// />
///
/// 글자 속성: 외곽선.
#[derive(Debug)]
pub struct Outline {
    /// ```xml
    /// type="{hc:LineType1}"
    /// ```
    pub r#type: core::LineType1,
}

impl TryFrom<AnyElement> for Outline {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__OUTLINE)?;

        let r#type = attributes!(element, "outline";
            "type" as r#type => one core::LineType1,
        );

        Ok(Self { r#type })
    }
}

/// ```xml
/// <shadow
///   type="{$ShadowKind}"
///   color="{hc:RGBColorType}"
///   offsetX="{xs:integer; >= -100; <= 100}"
///   offsetY="{xs:integer; >= -100; <= 100}"
/// />
///
/// 글자 속성: 그림자.
#[derive(Debug)]
pub struct Shadow {
    /// ```xml
    /// type="{$ShadowKind}"
    /// ```
    ///
    /// 그림자 종류
    pub r#type: arbitrary::ShadowKind,
    /// ```xml
    /// color="{hc:RGBColorType}"
    /// ```
    ///
    /// 그림자 색
    pub color: core::RgbColorType,
    /// ```xml
    /// offsetX="{xs:integer; >= -100; <= 100}"
    /// ```
    ///
    /// 그림자 간격 X. 단위는 %.
    pub offset_x: xs::Integer8,
    /// ```xml
    /// offsetY="{xs:integer; >= -100; <= 100}"
    /// ```
    ///
    /// 그림자 간격 Y. 단위는 %.
    pub offset_y: xs::Integer8,
}

impl TryFrom<AnyElement> for Shadow {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__SHADOW)?;

        let (r#type, color, offset_x, offset_y) = attributes!(element, "shadow";
            "type" as r#type => one arbitrary::ShadowKind,
            "color" as color => one core::RgbColorType,
            "offsetX" as offset_x => one xs::Integer8,
            "offsetY" as offset_y => one xs::Integer8,
        );

        Ok(Self {
            r#type,
            color,
            offset_x,
            offset_y,
        })
    }
}

/// ```xml
/// <tabDef
///   id="{xs:nonNegativeInteger}"
///   autoTabLeft="{xs:boolean; default="false"}"
///   autoTabRight="{xs:boolean; default="false"}"
/// >
///   <tabItem ... />
/// </tabDef>
///
/// 탭 정의 정보
#[derive(Debug)]
pub struct TabDefType {
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    pub id: xs::NonNegativeInteger32,
    /// ```xml
    /// autoTabLeft="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 문단 왼쪽 끝 자동 탭(내어쓰기용 자동 탭)
    pub auto_tab_left: xs::Boolean,
    /// ```xml
    /// autoTabRight="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 문단 오른쪽 끝 자동 탭
    pub auto_tab_right: xs::Boolean,
    /// ```xml
    /// <tabItem ... />
    /// ```
    pub tab: Option<TabItem>,
}

impl TryFrom<AnyElement> for TabDefType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TAB_DEFINITION)?;

        let (id, auto_tab_left, auto_tab_right) = attributes!(element, "tabDef";
            "id" as id => one xs::NonNegativeInteger32,
            "autoTabLeft" as auto_tab_left => default false; boolean,
            "autoTabRight" as auto_tab_right => default false; boolean,
        );

        let tab = children!(element;
            opt HANCOM__HEAD__TAB_ITEM, TabItem;
        );

        Ok(Self {
            id,
            auto_tab_left,
            auto_tab_right,
            tab,
        })
    }
}

/// ```xml
/// <tabItem
///   pos="{xs:integer}"
///   type="{$TabItemKind}"
///   leader="{hc:LineType2}"
/// />
#[derive(Debug)]
pub struct TabItem {
    /// ```xml
    /// pos="{xs:integer}"
    /// ```
    ///
    /// 탭 위치. 단위는 hwpunit.
    pub position: xs::Integer32,
    /// ```xml
    /// type="{$TabItemKind}"
    /// ```
    ///
    /// 탭의 종류
    pub r#type: arbitrary::TabItemKind,
    /// ```xml
    /// leader="{hc:LineType2}"
    /// ```
    ///
    /// 채움 종류
    pub leader: core::LineType2,
}

impl TryFrom<AnyElement> for TabItem {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TAB_ITEM)?;

        let (position, r#type, leader) = attributes!(element, "tabItem";
            "pos" as position => one xs::Integer32,
            "type" as r#type => one arbitrary::TabItemKind,
            "leader" as leader => one core::LineType2,
        );

        Ok(Self {
            position,
            r#type,
            leader,
        })
    }
}

/// ```xml
/// <numbering
///   id="{xs:nonNegativeInteger}"
///   start="{xs:integer; default="1"}"
/// >
///   <paraHead ...>...</paraHead>
/// </numbering>
/// ```
///
/// 번호 문단 모양 정보
#[derive(Debug)]
pub struct NumberingType {
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 번호 문단 모양 아이디
    pub id: xs::NonNegativeInteger32,
    /// ```xml
    /// start="{xs:integer; default="1"}"
    /// ```
    ///
    /// 시작 번호
    pub start: xs::Integer32,
    /// ```xml
    /// <paraHead ...>...</paraHead>
    /// ```
    pub heads: Vec<ParagraphHeadType>,
}

impl TryFrom<AnyElement> for NumberingType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__NUMBERING)?;

        let (id, start) = attributes!(element, "numbering";
            "id" as id => one xs::NonNegativeInteger32,
            "start" as start => default 1,
        );

        let heads = children!(element;
            many HANCOM__HEAD__PARAGRAPH_HEAD, ParagraphHeadType;
        );

        Ok(Self { id, start, heads })
    }
}

/// ```xml
/// <paraShape
///   id="{xs:nonNegativeInteger}"
///   tabPrIDRef="{xs:nonNegativeInteger}"
///   condense="{xs:integer; >= 0; <= 75; default="0"}"
///   fontLineHeight="{xs:boolean; default="false"}"
///   snapToGrid="{xs:boolean; default="true"}"
///   suppressLineNumbers="{xs:boolean; default="false"}"
///   checked="{xs:boolean; default="false"}"
/// >
///   <align ...>...</align>
///   <heading ...>...</heading>
///   <breakSetting ...>...</breakSetting>
///   <margin ...>...</margin>
///   <lineSpacing ...>...</lineSpacing>
///   <border ...>...</border>
///   <autoSpacing ...>...</autoSpacing>
/// </paraShape>
/// ```
#[derive(Debug)]
pub struct ParaShapeType {
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 문단 모양 아이디
    pub id: xs::NonNegativeInteger32,
    /// ```xml
    /// tabPrIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 탭 정의 아이디 참조
    pub tab_pr_id_ref: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// condense="{xs:integer; >= 0; <= 75; default="0"}"
    /// ```
    ///
    /// 문단 압축률
    pub condense: xs::Integer8,
    /// ```xml
    /// fontLineHeight="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 글자 기준 줄 높이 사용 여부
    pub font_line_height: xs::Boolean,
    /// ```xml
    /// snapToGrid="{xs:boolean; default="true"}"
    /// ```
    ///
    /// 격자에 맞춤 여부
    pub snap_to_grid: xs::Boolean,
    /// ```xml
    /// suppressLineNumbers="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 줄 번호 표시 안함 여부
    pub suppress_line_numbers: xs::Boolean,
    /// ```xml
    /// checked="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 체크된 문단 여부
    pub checked: xs::Boolean,
    /// ```xml
    /// <align ...>...</align>
    /// ```
    ///
    /// 문단 내 정렬
    pub align: ParagraphAlignType,
    /// ```xml
    /// <heading ...>...</heading>
    /// ```
    ///
    /// 문단 머리 번호/글머리표
    pub heading: ParagraphHeading,
    /// ```xml
    /// <breakSetting ...>...</breakSetting>
    /// ```
    ///
    /// 문단 줄나눔 설정
    pub break_setting: ParagraphBreakSetting,
    /// ```xml
    /// <margin ...>...</margin>
    /// ```
    ///
    /// 문단 여백
    pub margin: ParagraphMargin,
    /// ```xml
    /// <lineSpacing ...>...</lineSpacing>
    /// ```
    ///
    /// 줄 간격
    pub line_spacing: ParagraphLineSpacing,
    /// ```xml
    /// <border ...>...</border>
    /// ```
    ///
    /// 문단 테두리
    pub border: ParagraphBorder,
    /// ```xml
    /// <autoSpacing ...>...</autoSpacing>
    /// ```
    ///
    /// 문단 자동 간격 조정 설정
    pub auto_spacing: ParagraphAutoSpacing,
}

impl TryFrom<AnyElement> for ParaShapeType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__PARAGRAPH_PROPERTY)?;

        let (
            id,
            tab_pr_id_ref,
            condense,
            font_line_height,
            snap_to_grid,
            suppress_line_numbers,
            checked,
        ) = attributes!(element, "paraPr";
            "id" as id => one xs::NonNegativeInteger32,
            "tabPrIDRef" as tab_pr_id_ref => opt xs::NonNegativeInteger32,
            "condense" as condense => default 0,
            "fontLineHeight" as font_line_height => default false; boolean,
            "snapToGrid" as snap_to_grid => default true; boolean,
            "suppressLineNumbers" as suppress_line_numbers => default false; boolean,
            "checked" as checked => default false; boolean,
        );

        let (align, heading, break_setting, auto_spacing, margin, line_spacing, border) = children!(element;
            one HANCOM__HEAD__ALIGN, ParagraphAlignType;
            one HANCOM__HEAD__HEADING, ParagraphHeading;
            one HANCOM__HEAD__BREAK_SETTING, ParagraphBreakSetting;
            one HANCOM__HEAD__AUTO_SPACING, ParagraphAutoSpacing;
            one HANCOM__HEAD__MARGIN, ParagraphMargin;
            one HANCOM__HEAD__LINE_SPACING, ParagraphLineSpacing;
            one HANCOM__HEAD__BORDER, ParagraphBorder
        );

        Ok(Self {
            id,
            tab_pr_id_ref,
            condense,
            font_line_height,
            snap_to_grid,
            suppress_line_numbers,
            checked,
            align,
            heading,
            break_setting,
            margin,
            line_spacing,
            border,
            auto_spacing,
        })
    }
}

/// ```xml
/// <align
///   horizontal="{$ParagraphHorizontalAlignKind}"
///   vertical="{$ParagraphVerticalAlignKind}"
/// />
/// ```
///
/// 문단 내 정렬
#[derive(Debug)]
pub struct ParagraphAlignType {
    /// ```xml
    /// horizontal="{$ParagraphHorizontalAlignKind}"
    /// ```
    ///
    /// 수평 정렬
    pub horizontal: arbitrary::ParagraphHorizontalAlignKind,
    /// ```xml
    /// vertical="{$ParagraphVerticalAlignKind}"
    /// ```
    ///
    /// 수직 정렬
    pub vertical: arbitrary::ParagraphVerticalAlignKind,
}

impl TryFrom<AnyElement> for ParagraphAlignType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__ALIGN)?;

        let (horizontal, vertical) = attributes!(element, "align";
            "horizontal" as horizontal => one arbitrary::ParagraphHorizontalAlignKind,
            "vertical" as vertical => one arbitrary::ParagraphVerticalAlignKind,
        );

        Ok(Self {
            horizontal,
            vertical,
        })
    }
}

/// ```xml
/// <heading
///   type="{$ParagraphHeadingKind}"
///   idRef="{xs:nonNegativeInteger}"
///   level="{xs:nonNegativeInteger}"
/// />
/// ```
///
/// 문단 머리 번호/글머리표
#[derive(Debug)]
pub struct ParagraphHeading {
    /// ```xml
    /// type="{$ParagraphHeadingKind}"
    /// ```
    ///
    /// 문단 머리 유형
    pub r#type: arbitrary::ParagraphHeadingKind,
    /// ```xml
    /// idRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 번호 문단 모양 아이디 참조 또는 글머리표 아이디 참조
    pub id_ref: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// level="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 레벨
    pub level: xs::NonNegativeInteger32,
}

impl TryFrom<AnyElement> for ParagraphHeading {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__HEADING)?;

        let (r#type, id_ref, level) = attributes!(element, "heading";
            "type" as r#type => one arbitrary::ParagraphHeadingKind,
            "idRef" as id_ref => opt xs::NonNegativeInteger32,
            "level" as level => one xs::NonNegativeInteger32,
        );

        Ok(Self {
            r#type,
            id_ref,
            level,
        })
    }
}

/// ```xml
/// <breakSetting
///   breakLatinWord="{$BreakLatinWordKind}"
///   breakNonLatinWord="{$BreakNonLatinWordKind}"
///   widowOrphan="{xs:boolean}"
///   keepWithNext="{xs:boolean}"
///   keepLines="{xs:boolean}"
///   pageBreakBefore="{xs:boolean}"
///   lineWrap="{$LineWrapKind}"
/// />
/// ```
///
/// 문단 줄나눔 설정
#[derive(Debug)]
pub struct ParagraphBreakSetting {
    /// ```xml
    /// breakLatinWord="{$BreakLatinWordKind}"
    /// ```
    ///
    /// 라틴 문자의 줄나눔 단위
    pub break_latin_word: arbitrary::BreakLatinWordKind,
    /// ```xml
    /// breakNonLatinWord="{$BreakNonLatinWordKind}"
    /// ```
    ///
    /// 라틴 문자 이외의 문자의 줄나눔 단위
    pub break_non_latin_word: arbitrary::BreakNonLatinWordKind,
    /// ```xml
    /// widowOrphan="{xs:boolean}"
    /// ```
    ///
    /// 외톨이줄 보호 여부
    pub widow_orphan: xs::Boolean,
    /// ```xml
    /// keepWithNext="{xs:boolean}"
    /// ```
    ///
    /// 다음 문단과 함께
    pub keep_with_next: xs::Boolean,
    /// ```xml
    /// keepLines="{xs:boolean}"
    /// ```
    ///
    /// 문단 보호 여부
    pub keep_lines: xs::Boolean,
    /// ```xml
    /// pageBreakBefore="{xs:boolean}"
    /// ```
    ///
    /// 문단 앞에서 항상 쪽나눔 여부
    pub page_break_before: xs::Boolean,
    /// ```xml
    /// lineWrap="{$LineWrapKind}"
    /// ```
    ///
    /// 한 줄로 입력 사용 시의 형식
    pub line_wrap: arbitrary::LineWrapKind,
}

impl TryFrom<AnyElement> for ParagraphBreakSetting {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__BREAK_SETTING)?;

        let (
            break_latin_word,
            break_non_latin_word,
            widow_orphan,
            keep_with_next,
            keep_lines,
            page_break_before,
            line_wrap,
        ) = attributes!(element, "breakSetting";
            "breakLatinWord" as break_latin_word => one arbitrary::BreakLatinWordKind,
            "breakNonLatinWord" as break_non_latin_word => one arbitrary::BreakNonLatinWordKind,
            "widowOrphan" as widow_orphan => one (boolean),
            "keepWithNext" as keep_with_next => one (boolean),
            "keepLines" as keep_lines => one (boolean),
            "pageBreakBefore" as page_break_before => one (boolean),
            "lineWrap" as line_wrap => one arbitrary::LineWrapKind,
        );

        Ok(Self {
            break_latin_word,
            break_non_latin_word,
            widow_orphan,
            keep_with_next,
            keep_lines,
            page_break_before,
            line_wrap,
        })
    }
}

/// ```xml
/// <margin>
///   <intent ... />
///   <left ... />
///   <right ... />
///   <prev ... />
///   <next ... />
/// </margin>
///
/// 문단 여백
#[derive(Debug)]
pub struct ParagraphMargin {
    /// ```xml
    /// <intent value="{xs:integer}" unit="{hc:HWPUNIT}" />
    /// ```
    ///
    /// 들여쓰기/내어쓰기
    ///
    /// n이 0보다 크면 들여쓰기 n.
    ///
    /// n이 0이면 : 보통.
    ///
    /// n이 0보다 작으면 내어쓰기 n.
    pub indent: core::HWPValue,
    /// ```xml
    /// <left value="{xs:integer}" unit="{hc:HWPUNIT}" />
    /// ```
    ///
    /// 왼쪽 여백
    ///
    /// 단위를 표기하지 않으면 hwpunit이고 표기하면 표기한 단위로.
    pub left: core::HWPValue,
    /// ```xml
    /// <right value="{xs:integer}" unit="{hc:HWPUNIT}" />
    /// ```
    ///
    /// 오른쪽 여백
    pub right: core::HWPValue,
    /// ```xml
    /// <prev value="{xs:integer}" unit="{hc:HWPUNIT}" />
    /// ```
    ///
    /// 문단 간격 위
    pub previous: core::HWPValue,
    /// ```xml
    /// <next value="{xs:integer}" unit="{hc:HWPUNIT}" />
    /// ```
    ///
    /// 문단 간격 아래
    pub next: core::HWPValue,
}

impl TryFrom<AnyElement> for ParagraphMargin {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__MARGIN)?;

        let (indent, left, right, previous, next) = children!(element;
            one HANCOM__CORE__INDENT, core::HWPValue;
            one HANCOM__CORE__LEFT, core::HWPValue;
            one HANCOM__CORE__RIGHT, core::HWPValue;
            one HANCOM__CORE__PREVIOUS, core::HWPValue;
            one HANCOM__CORE__NEXT, core::HWPValue;
        );

        Ok(Self {
            indent,
            left,
            right,
            previous,
            next,
        })
    }
}

/// ```xml
/// <lineSpacing
///   type="{$LineSpacingKind}"
///   value="{xs:integer}"
///   unit="{hc:HWPUNIT}"
/// />
/// ```
///
/// 줄 간격
#[derive(Debug)]
pub struct ParagraphLineSpacing {
    /// ```xml
    /// type="{$LineSpacingKind}"
    /// ```
    ///
    /// 줄 간격 종류
    pub r#type: arbitrary::LineSpacingKind,
    /// ```xml
    /// value="{xs:integer}"
    /// ```
    ///
    /// 줄 간격 값
    ///
    /// type이 PERCENT이면 0%~500%로 제한.
    pub value: xs::Integer16,
    /// ```xml
    /// unit="{hc:HWPUNIT}"
    /// ```
    ///
    /// 줄 간격 값의 단위
    pub unit: core::HWPUnit,
}

impl TryFrom<AnyElement> for ParagraphLineSpacing {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__LINE_SPACING)?;

        let (r#type, value, unit) = attributes!(element, "lineSpacing";
            "type" as r#type => one arbitrary::LineSpacingKind,
            "value" as value => one xs::Integer16,
            "unit" as unit => one core::HWPUnit,
        );

        Ok(Self {
            r#type,
            value,
            unit,
        })
    }
}

/// ```xml
/// <border
///   borderFillIDRef="{xs:nonNegativeInteger}"
///   offsetLeft="{xs:integer; default="0"}"
///   offsetRight="{xs:integer; default="0"}"
///   offsetTop="{xs:integer; default="0"}"
///   offsetBottom="{xs:integer; default="0"}"
///   connect="{xs:boolean; default="false"}"
///   ignoreMargin="{xs:boolean; default="false"}"
/// />
/// ```
///
/// 문단 테두리
#[derive(Debug)]
pub struct ParagraphBorder {
    /// ```xml
    /// borderFillIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 테두리/배경 모양 아이디 참조
    pub border_fill_id_ref: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// offsetLeft="{xs:integer; default="0"}"
    /// ```
    ///
    /// 문단 테두리 왼쪽 간격. 단위는 hwpunit.
    pub offset_left: xs::Integer32,
    /// ```xml
    /// offsetRight="{xs:integer; default="0"}"
    /// ```
    ///
    /// 문단 테두리 오른쪽 간격. 단위는 hwpunit.
    pub offset_right: xs::Integer32,
    /// ```xml
    /// offsetTop="{xs:integer; default="0"}"
    /// ```
    ///
    /// 문단 테두리 위쪽 간격. 단위는 hwpunit.
    pub offset_top: xs::Integer32,
    /// ```xml
    /// offsetBottom="{xs:integer; default="0"}"
    /// ```
    ///
    /// 문단 테두리 아래쪽 간격. 단위는 hwpunit.
    pub offset_bottom: xs::Integer32,
    /// ```xml
    /// connect="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 문단 테두리 연결 여부
    pub connect: xs::Boolean,
    /// ```xml
    /// ignoreMargin="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 문단 테두리 여백 무시 여부
    pub ignore_margin: xs::Boolean,
}

impl TryFrom<AnyElement> for ParagraphBorder {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__BORDER)?;

        let (
            border_fill_id_ref,
            offset_left,
            offset_right,
            offset_top,
            offset_bottom,
            connect,
            ignore_margin,
        ) = attributes!(element, "border";
            "borderFillIDRef" as border_fill_id_ref => opt xs::NonNegativeInteger32,
            "offsetLeft" as offset_left => default 0,
            "offsetRight" as offset_right => default 0,
            "offsetTop" as offset_top => default 0,
            "offsetBottom" as offset_bottom => default 0,
            "connect" as connect => default false; boolean,
            "ignoreMargin" as ignore_margin => default false; boolean,
        );

        Ok(Self {
            border_fill_id_ref,
            offset_left,
            offset_right,
            offset_top,
            offset_bottom,
            connect,
            ignore_margin,
        })
    }
}

/// ```xml
/// <autoSpacing
///   eAsianEng="{xs:boolean}"
///   eAsianNum="{xs:boolean}"
/// />
///```
///
/// 문단 자동 간격 조절 설정
#[derive(Debug)]
pub struct ParagraphAutoSpacing {
    /// ```xml
    /// eAsianEng="{xs:boolean}"
    /// ```
    ///
    /// 한글과 영어 간격을 자동 조절
    pub e_asian_eng: xs::Boolean,
    /// ```xml
    /// eAsianNum="{xs:boolean}"
    /// ```
    ///
    /// 한글과 숫자 간격을 자동 조절
    pub e_asian_num: xs::Boolean,
}

impl TryFrom<AnyElement> for ParagraphAutoSpacing {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__AUTO_SPACING)?;

        let (e_asian_eng, e_asian_num) = attributes!(element, "autoSpacing";
            "eAsianEng" as e_asian_eng => one (boolean),
            "eAsianNum" as e_asian_num => one (boolean),
        );

        Ok(Self {
            e_asian_eng,
            e_asian_num,
        })
    }
}

/// ```xml
/// <paraHead
///   start="{xs:unsignedInt; default="1"}"
///   level="{xs:positiveInteger}"
///   align="{$ParagraphHeadAlign; default="LEFT"}"
///   useInstWidth="{xs:boolean; default="true"}"
///   autoIndent="{xs:boolean; default="true"}"
///   widthAdjust="{xs:integer; default="0"}"
///   textOffsetType="{$TextOffsetKind; default="PERCENT"}"
///   textOffset="{xs:integer; default="50"}"
///   numFormat="{hc:NumberType1; default="DIGIT"}"
///   charPrIDRef="{xs:nonNegativeInteger}"
///   checkable="{xs:boolean}"
/// >
///   ^1.
/// </paraHead>
/// ```
///
/// 각 번호 문단 머리의 정보.
///
/// 문자열 내 특정 문자에 제어코드(^)를 붙임으로써 한글에서 표시되는 번호 문단 머리의 포맷을 제어한다.
///
/// ^n: 레벨 경로를 표시한다. (예: 1.1.1.1.1.1.1)
///
/// ^N: 레벨 경로를 표시하며 마지막에 마침표를 하나 더 찍는다. (예: 1.1.1.1.1.1.1.)
///
/// ^레벨번호(1~7): 해당 레벨에 해당하는 숫자 또는 문자 또는 기호를 표시한다.
#[derive(Debug)]
pub struct ParagraphHeadType {
    /// ```xml
    /// start="{xs:unsignedInt; default="1"}"
    /// ```
    ///
    /// 시작 번호
    pub start: xs::UnsignedInt32,
    /// ```xml
    /// level="{xs:positiveInteger}"
    /// ```
    ///
    /// 레벨
    pub level: xs::PositiveInteger32,
    /// ```xml
    /// align="{$ParagraphHeadAlign; default="LEFT"}"
    /// ```
    ///
    /// 정렬 방식
    pub align: arbitrary::ParagraphHorizontalAlignKind,
    /// ```xml
    /// useInstWidth="{xs:boolean; default="true"}"
    /// ```
    ///
    /// 고정폭 사용 여부
    pub use_inset_width: xs::Boolean,
    /// ```xml
    /// autoIndent="{xs:boolean; default="true"}"
    /// ```
    ///
    /// 자동 들여쓰기 여부
    pub auto_indent: xs::Boolean,
    /// ```xml
    /// widthAdjust="{xs:integer; default="0"}"
    /// ```
    ///
    /// 폭 조정 값
    pub width_adjust: xs::Integer32,
    /// ```xml
    /// textOffsetType="{$TextOffsetKind; default="PERCENT"}"
    /// ```
    ///
    /// 텍스트 오프셋 유형
    pub text_offset_type: arbitrary::TextOffsetKind,
    /// ```xml
    /// textOffset="{xs:integer; default="50"}"
    /// ```
    ///
    /// 텍스트 오프셋 값
    pub text_offset: xs::Integer32,
    /// ```xml
    /// numFormat="{hc:NumberType1; default="DIGIT"}"
    /// ```
    ///
    /// 번호 형식
    pub number_format: core::NumberType1,
    /// ```xml
    /// charPrIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 글자 모양 아이디 참조
    pub char_pr_id_ref: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// checkable="{xs:boolean}"
    /// ```
    ///
    /// 체크 가능 여부
    pub checkable: Option<xs::Boolean>,
    /// ```xml
    /// ^1.
    /// ```
    pub text: Option<xs::String>,
}

impl TryFrom<AnyElement> for ParagraphHeadType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__PARAGRAPH_HEAD)?;

        let (
            start,
            level,
            align,
            use_inset_width,
            auto_indent,
            width_adjust,
            text_offset_type,
            text_offset,
            number_format,
            char_pr_id_ref,
            checkable,
        ) = attributes!(element, "paraHead";
            "start" as start => default 1u32,
            "level" as level => one xs::PositiveInteger32,
            "align" as align => default arbitrary::ParagraphHorizontalAlignKind::Left,
            "useInsetWidth" as use_inset_width => default true; boolean,
            "autoIndent" as auto_indent => default true; boolean,
            "widthAdjust" as width_adjust => default 0,
            "textOffsetType" as text_offset_type => default arbitrary::TextOffsetKind::Percent,
            "textOffset" as text_offset => default 50,
            "numFormat" as number_format => default core::NumberType1::Digit,
            "charPrIDRef" as char_pr_id_ref => opt xs::NonNegativeInteger32,
            "checkable" as checkable => opt (boolean),
        );

        Ok(Self {
            start,
            level,
            align,
            use_inset_width,
            auto_indent,
            width_adjust,
            text_offset_type,
            text_offset,
            number_format,
            char_pr_id_ref,
            checkable,
            text: element.text,
        })
    }
}

/// ```xml
/// <bullet
///   id="{xs:nonNegativeInteger}"
///   char="{xs:string}"
///   checkedChar="{xs:string}"
///   useImage="{xs:boolean}"
/// >
///   <img ...>...</img>
///   <paraHead ...>...</paraHead>
/// </bullet>
/// ```
///
/// 글머리표 문단 모양 정보
#[derive(Debug)]
pub struct BulletType {
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 글머리표 아이디
    pub id: xs::NonNegativeInteger32,
    /// ```xml
    /// char="{xs:string}"
    /// ```
    ///
    /// 글머리표 문자
    pub character: xs::String,
    /// ```xml
    /// checkedChar="{xs:string}"
    /// ```
    ///
    /// 체크된 글머리표 문자
    pub checked_character: Option<xs::String>,
    /// ```xml
    /// useImage="{xs:boolean}"
    /// ```
    ///
    /// 이미지 사용 여부
    pub use_image: xs::Boolean,
    /// ```xml
    /// <img ...>...</img>
    /// ```
    pub image: Option<ImageType>,
    /// ```xml
    /// <paraHead ...>...</paraHead>
    /// ```
    pub head: ParagraphHeadType,
}

impl TryFrom<AnyElement> for BulletType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__BULLET)?;

        let (id, character, checked_character, use_image) = attributes!(element, "bullet";
            "id" as id => one xs::NonNegativeInteger32,
            "char" as character => one (string),
            "checkedChar" as checked_character => opt (string),
            "useImage" as use_image => one (boolean),
        );

        let (image, head) = children!(element;
            opt HANCOM__CORE__IMAGE, ImageType;
            one HANCOM__HEAD__PARAGRAPH_HEAD, ParagraphHeadType
        );

        Ok(Self {
            id,
            character,
            checked_character,
            use_image,
            image,
            head,
        })
    }
}

/// ```xml
/// <style
///   id="{xs:nonNegativeInteger}"
///   type="{$StyleKind}"
///   name="{xs:string}"
///   engName="{xs:string}"
///   paraPrIDRef="{xs:nonNegativeInteger}"
///   charPrIDRef="{xs:nonNegativeInteger}"
///   nextStyleIDRef="{xs:nonNegativeInteger}"
///   langID="{xs:unsignedShort}"
///   lockForm="{xs:boolean; default="false"}"
/// />
///```
///
/// 스타일 정보
#[derive(Debug)]
pub struct StyleType {
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 스타일 아이디
    pub id: xs::NonNegativeInteger32,
    /// ```xml
    /// type="{$StyleKind}"
    /// ```
    ///
    /// 스타일 종류
    pub r#type: arbitrary::StyleKind,
    /// ```xml
    /// name="{xs:string}"
    /// ```
    ///
    /// 로컬 스타일 이름.
    ///
    /// 한글 윈도에서는 한글 스타일 이름.
    pub name: xs::String,
    /// ```xml
    /// engName="{xs:string}"
    /// ```
    ///
    /// 영문 스타일 이름
    pub eng_name: Option<xs::String>,
    /// ```xml
    /// paraPrIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 문단 모양 아이디 참조.
    ///
    /// 스타일의 종류가 문단이 경우  지정해야 함.
    pub para_pr_id_ref: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// charPrIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 글자 모양 아이디 참조
    ///
    /// 스타일의 종류가 글자인 경우  지정해야 함.
    pub char_pr_id_ref: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// nextStyleIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 다음 스타일 아이디 참조
    ///
    /// 문단 스타일에서 사용자가 리턴키를 입력하여 다음 문단으로 이동하였을 때 적용될 문단 스타일을 지정한다.
    pub next_style_id_ref: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// langID="{xs:unsignedShort}"
    /// ```
    ///
    /// 언어 아이디
    pub lang_id: Option<xs::UnsignedShort>,
    /// ```xml
    /// lockForm="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 양식 모드에서 Style 보호하기 여부.
    pub lock_form: xs::Boolean,
}

impl TryFrom<AnyElement> for StyleType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__STYLE)?;

        let (
            id,
            r#type,
            name,
            eng_name,
            para_pr_id_ref,
            char_pr_id_ref,
            next_style_id_ref,
            lang_id,
            lock_form,
        ) = attributes!(element, "style";
            "id" as id => one xs::NonNegativeInteger32,
            "type" as r#type => one arbitrary::StyleKind,
            "name" as name => one (string),
            "engName" as eng_name => opt (string),
            "paraPrIDRef" as para_pr_id_ref => opt xs::NonNegativeInteger32,
            "charPrIDRef" as char_pr_id_ref => opt xs::NonNegativeInteger32,
            "nextStyleIDRef" as next_style_id_ref => opt xs::NonNegativeInteger32,
            "langID" as lang_id => opt xs::UnsignedShort,
            "lockForm" as lock_form => default false; boolean,
        );

        Ok(Self {
            id,
            r#type,
            name,
            eng_name,
            para_pr_id_ref,
            char_pr_id_ref,
            next_style_id_ref,
            lang_id,
            lock_form,
        })
    }
}

/// ```xml
/// <memoShape
///   id="{xs:nonNegativeInteger}"
///   width="{xs:nonNegativeInteger}"
///   lineWidth="{hc:LineWidth}"
///   lineType="{hc:LineType2}"
///   lineColor="{hc:RGBColorType}"
///   fillColor="{hc:RGBColorType}"
///   activeColor="{hc:RGBColorType}"
///   memoType="{$MemoKind}"
/// />
///```
///
/// 메모 모양 정보
#[derive(Debug)]
pub struct MemoShapeType {
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 메모 모양 아이디
    pub id: xs::NonNegativeInteger32,
    /// ```xml
    /// width="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 메모 아이콘 너비. 단위는 hwpunit.
    pub width: xs::NonNegativeInteger32,
    /// ```xml
    /// lineWidth="{hc:LineWidth}"
    /// ```
    ///
    /// 메모 아이콘 테두리 선 굵기
    pub line_width: core::LineWidth,
    /// ```xml
    /// lineType="{hc:LineType2}"
    /// ```
    ///
    /// 메모 아이콘 테두리 선 종류
    pub line_type: core::LineType2,
    /// ```xml
    /// lineColor="{hc:RGBColorType}"
    /// ```
    ///
    /// 메모 아이콘 테두리 색
    pub line_color: core::RgbColorType,
    /// ```xml
    /// fillColor="{hc:RGBColorType}"
    /// ```
    ///
    /// 메모 아이콘 채우기 색
    pub fill_color: core::RgbColorType,
    /// ```xml
    /// activeColor="{hc:RGBColorType}"
    /// ```
    ///
    /// 메모 아이콘 활성화 색
    pub active_color: core::RgbColorType,
    /// ```xml
    /// memoType="{$MemoKind}"
    /// ```
    ///
    /// 메모 아이콘 종류
    pub memo_type: arbitrary::MemoKind,
}

impl TryFrom<AnyElement> for MemoShapeType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__MEMO_PROPERTY)?;

        let (id, width, line_width, line_type, line_color, fill_color, active_color, memo_type) = attributes!(element, "memoShape";
            "id" as id => one xs::NonNegativeInteger32,
            "width" as width => one xs::NonNegativeInteger32,
            "lineWidth" as line_width => one core::LineWidth,
            "lineType" as line_type => one core::LineType2,
            "lineColor" as line_color => one core::RgbColorType,
            "fillColor" as fill_color => one core::RgbColorType,
            "activeColor" as active_color => one core::RgbColorType,
            "memoType" as memo_type => one arbitrary::MemoKind,
        );

        Ok(Self {
            id,
            width,
            line_width,
            line_type,
            line_color,
            fill_color,
            active_color,
            memo_type,
        })
    }
}

/// ```xml
/// <trackChange
///   type="{$TrackChangeKind}"
///   date="{xs:dateTime}"
///   authorID="{xs:nonNegativeInteger}"
///   charShapeID="{xs:nonNegativeInteger}"
///   paraShapeID="{xs:nonNegativeInteger}"
///   hide="{xs:boolean}"
///   id="{xs:nonNegativeInteger}"
/// />
/// ```
///
/// 변경 추적 정보
#[derive(Debug)]
pub struct TrackChange {
    /// ```xml
    /// type="{$TrackChangeKind}"
    /// ```
    ///
    /// 변경 추적 종류
    pub r#type: core::TrackChangeKind,
    /// ```xml
    /// date="{xs:dateTime}"
    /// ```
    ///
    /// 변경 일시
    pub date: xs::DateTime,
    /// ```xml
    /// authorID="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 변경 추적 작성자 아이디
    pub author_id: xs::NonNegativeInteger32,
    /// ```xml
    /// charShapeID="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 변경된 글자 모양 아이디
    pub char_shape_id: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// paraShapeID="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 변경된 문단 모양 아이디
    pub para_shape_id: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// hide="{xs:boolean}"
    /// ```
    ///
    /// 변경 추적 숨김 여부
    pub hide: xs::Boolean,
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 변경 추적 아이디
    pub id: xs::NonNegativeInteger32,
}

impl TryFrom<AnyElement> for TrackChange {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TRACK_CHANGE)?;

        let (r#type, date, author_id, char_shape_id, para_shape_id, hide, id) = attributes!(element, "trackChange";
            "type" as r#type => one core::TrackChangeKind,
            "date" as date => one (string),
            "authorID" as author_id => one xs::NonNegativeInteger32,
            "charPrIDRef" as char_shape_id => opt xs::NonNegativeInteger32,
            "paraPrIDRef" as para_shape_id => opt xs::NonNegativeInteger32,
            "hide" as hide => one (boolean),
            "id" as id => one xs::NonNegativeInteger32,
        );

        Ok(Self {
            r#type,
            date,
            author_id,
            char_shape_id,
            para_shape_id,
            hide,
            id,
        })
    }
}

/// ```xml
/// <trackChangeAuthor
///   name="{xs:string}"
///   mark="{xs:boolean}"
///   color="{hc:RGBColorType}"
///   id="{xs:nonNegativeInteger}"
/// />
/// ```
///
/// 변경 추적 작성자
#[derive(Debug)]
pub struct TrackChangeAuthor {
    /// ```xml
    /// name="{xs:string}"
    /// ```
    pub name: Option<xs::String>,
    /// ```xml
    /// mark="{xs:boolean}"
    /// ```
    pub mark: Option<xs::Boolean>,
    /// ```xml
    /// color="{hc:RGBColorType}"
    /// ```
    pub color: Option<core::RgbColorType>,
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    pub id: xs::NonNegativeInteger32,
}

impl TryFrom<AnyElement> for TrackChangeAuthor {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TRACK_CHANGE_AUTHOR)?;

        let (name, mark, color, id) = attributes!(element, "trackChangeAuthor";
            "name" as name => opt (string),
            "mark" as mark => opt (boolean),
            "color" as color => opt core::RgbColorType,
            "id" as id => one xs::NonNegativeInteger32,
        );

        Ok(Self {
            name,
            mark,
            color,
            id,
        })
    }
}

/// ```xml
/// <forbiddenWordList>
///   <forbiddenWord>...</forbiddenWord>
/// </forbiddenWordList>
/// ```
#[derive(Debug, Default)]
pub struct ForbiddenWordList {
    /// ```xml
    /// <forbiddenWord>...</forbiddenWord>
    /// ```
    pub words: Vec<String>,
}

impl TryFrom<AnyElement> for ForbiddenWordList {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__FORBIDDEN_WORDS)?;

        let mut words = vec![];

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__FORBIDDEN_WORD => match child.text {
                    Some(word) => words.push(word),
                    None => continue,
                },
                _ => continue,
            }
        }

        Ok(Self { words })
    }
}

/// ```xml
/// <compatibleDocument
///   targetProgram="{$TargetProgram}"
/// >
///   <layoutCompatibility>...</layoutCompatibility>
/// </compatibleDocument>
/// ```
///
/// 문서 호환성 정보
#[derive(Debug)]
pub struct CompatibleDocument {
    /// ```xml
    /// targetProgram="{$TargetProgram}"
    /// ```
    pub target_program: arbitrary::TargetProgram,
    /// ```xml
    /// <layoutCompatibility>...</layoutCompatibility>
    /// ```
    pub layout_compatibility: LayoutCompatibility,
}

impl TryFrom<AnyElement> for CompatibleDocument {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__COMPATIBLE_DOCUMENT)?;

        let target_program = attributes!(element, "compatibleDocument";
            "targetProgram" as target_program => one arbitrary::TargetProgram,
        );

        let layout_compatibility = children! {element;
            opt HANCOM__HEAD__LAYOUT_COMPATIBILITY, LayoutCompatibility;
        };

        let layout_compatibility = layout_compatibility.unwrap_or_default();

        Ok(Self {
            target_program,
            layout_compatibility,
        })
    }
}

/// ```xml
/// <layoutCompatibility>
///   <applyFontWeightToBold />
///   <useInnerUnderline />
///   <fixedUnderlineWidth />
///   <doNotApplyStrikeoutWithUnderline />
///   <useLowercaseStrikeout />
///   <extendLineheightToOffset />
///   <applyFontspaceToLatin />
///   <treatQuotationAsLatin />
///   <doNotApplyDiacSymMarkOfNoneAndSix />
///   <doNotAlignWhitespaceOnRight />
///   <doNotAdjustWordInJustify />
///   <baseCharUnitOnEAsian />
///   <baseCharUnitOfIndentOnFirstChar />
///   <adjustLineheightToFont />
///   <adjustBaseInlineFixedLinespacing />
///   <applyPrevspacingBeneathObject />
///   <applyNextspacingOfLastPara />
///   <applyAtLeastToPercent100Pct />
///   <doNotApplyAutoSpaceEAsianEng />
///   <doNotApplyAutoSpaceEAsianNum />
///   <adjustParaBorderfillToSpacing />
///   <connectParaBorderfillOfEqualBorder />
///   <adjustParaBorderOffsetWithBorder />
///   <extendLineheightToParaBorderOffset />
///   <applyParaBorderToOutside />
///   <applyMinColumnWidthTo1mm />
///   <applyTabPosBasedOnSegment />
///   <breakTabOverline />
///   <adjustVertPosOfLine />
///   <doNotApplyWhiteSpaceHeight />
///   <doNotAlignLastPeriod />
///   <doNotAlignLastForbidden />
///   <baseLineSpacingOnLineGrid />
///   <applyCharSpacingToCharGrid />
///   <doNotApplyGridInHeaderFooter />
///   <applyExtendHeaderFooterEachSection />
///   <doNotApplyHeaderFooterAtNoSpace />
///   <doNotApplyColSeparatorAtNoGap />
///   <doNotApplyLinegridAtNoLinespacing />
///   <doNotApplyImageEffect />
///   <doNotApplyShapeComment />
///   <doNotAdjustEmptyAnchorLine />
///   <overlapBothAllowOverlap />
///   <doNotApplyVertOffsetOfForward />
///   <extendVertLimitToPageMargins />
///   <doNotHoldAnchorOfTable />
///   <doNotFormattingAtBeneathAnchor />
///   <adjustBaselineOfObjectToBottom />
///   <doNotApplyExtensionCharCompose />
/// </layoutCompatibility>
/// ```
#[derive(Debug, Default)]
pub struct LayoutCompatibility {
    pub set: FxHashSet<core::LayoutCompatibilityKind>,
}

impl TryFrom<AnyElement> for LayoutCompatibility {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__LAYOUT_COMPATIBILITY)?;

        let set = element
            .children
            .into_iter()
            .filter_map(|child| core::LayoutCompatibilityKind::from_element_name(child.name))
            .collect();

        Ok(Self { set })
    }
}

/// ```xml
/// <docOption>
///   <linkInfo
///     path="{xs:string}"
///     pageInherit="{xs:boolean; default="false"}"
///     footnoteInherit="{xs:boolean; default="false"}"
///   />
///   <licenseMark
///     type="{xs:unsignedInt}"
///     flag="{xs:byte}"
///     lang="{xs:byte}"
///   />
/// </docOption>
/// ```
#[derive(Debug)]
pub struct DocumentOption {
    /// ```xml
    /// <linkInfo
    ///   path="{xs:string}"
    ///   pageInherit="{xs:boolean; default="false"}"
    ///   footnoteInherit="{xs:boolean; default="false"}"
    /// />
    /// ```
    pub link_info: Option<LinkInfo>,
    /// ```xml
    /// <licenseMark
    ///   type="{xs:unsignedInt}"
    ///   flag="{xs:byte}"
    ///   lang="{xs:byte}"
    /// />
    /// ```
    pub license_mark: Option<LicenseMark>,
}

impl TryFrom<AnyElement> for DocumentOption {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__DOCUMENT_OPTION)?;

        let (link_info, license_mark) = children!(element;
            opt HANCOM__HEAD__LINK_INFO, LinkInfo;
            opt HANCOM__HEAD__LICENSE_MARK, LicenseMark;
        );

        Ok(Self {
            link_info,
            license_mark,
        })
    }
}

/// ```xml
/// <linkInfo
///   path="{xs:string}"
///   pageInherit="{xs:boolean; default="false"}"
///   footnoteInherit="{xs:boolean; default="false"}"
/// />
/// ```
#[derive(Debug)]
pub struct LinkInfo {
    pub path: String,
    pub page_inherit: bool,
    pub footnote_inherit: bool,
}

impl TryFrom<AnyElement> for LinkInfo {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__LINK_INFO)?;

        let (path, page_inherit, footnote_inherit) = attributes!(element, "linkInfo";
            "path" as path => one (string),
            "pageInherit" as page_inherit => default false; boolean,
            "footnoteInherit" as footnote_inherit => default false; boolean,
        );

        Ok(Self {
            path,
            page_inherit,
            footnote_inherit,
        })
    }
}

/// ```xml
/// <licenseMark
///   type="{xs:unsignedInt}"
///   flag="{xs:byte}"
///   lang="{xs:byte}"
/// />
/// ```
#[derive(Debug)]
pub struct LicenseMark {
    /// ```xml
    /// type="{xs:unsignedInt}"
    /// ```
    pub r#type: xs::UnsignedInt32,
    /// ```xml
    /// flag="{xs:byte}"
    /// ```
    pub flag: xs::Byte,
    /// ```xml
    /// lang="{xs:byte}"
    /// ```
    pub lang: xs::Byte,
}

impl TryFrom<AnyElement> for LicenseMark {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__LICENSE_MARK)?;

        let (r#type, flag, lang) = attributes!(element, "licenseMark";
            "type" as r#type => one xs::UnsignedInt32,
            "flag" as flag => one xs::Byte,
            "lang" as lang => one xs::Byte,
        );

        Ok(Self { r#type, flag, lang })
    }
}

/// ```xml
/// <trackchangeConfig flag="{xs:nonNegativeInteger}">
///   <trackChangeEncryption>
///     <derivationKey
///       algorithm="{xs:string}"
///       size="{xs:nonNegativeInteger}"
///       count="{xs:nonNegativeInteger}"
///       salt="{xs:base64Binary}"
///     />
///     <hash>...</hash>
///   </trackChangeEncryption>
/// </trackchangeConfig>
/// ```
#[derive(Debug)]
pub struct TrackChangeConfig {
    /// ```xml
    /// flag="{xs:nonNegativeInteger}"
    /// ```
    pub flag: xs::NonNegativeInteger32,
    /// ```xml
    /// <trackChangeEncryption>
    ///   <derivationKey
    ///     algorithm="{xs:string}"
    ///     size="{xs:nonNegativeInteger}"
    ///     count="{xs:nonNegativeInteger}"
    ///     salt="{xs:base64Binary}"
    ///   />
    ///   <hash>...</hash>
    /// </trackChangeEncryption>
    /// ```
    pub track_change_encryption: Option<TrackChangeEncryption>,
}

impl TryFrom<AnyElement> for TrackChangeConfig {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TRACK_CHANGE_CONFIG)?;

        let flag = attributes!(element, "trackchangeConfig";
            "flag" as flag => one xs::NonNegativeInteger32,
        );

        let track_change_encryption = children!(element;
            opt HANCOM__HEAD__TRACK_CHANGE_ENCRYPTION, TrackChangeEncryption;
        );

        Ok(Self {
            flag,
            track_change_encryption,
        })
    }
}

/// ```xml
/// <trackChangeEncryption>
///   <derivationKey
///     algorithm="{xs:string}"
///     size="{xs:nonNegativeInteger}"
///     count="{xs:nonNegativeInteger}"
///     salt="{xs:base64Binary}"
///   />
///   <hash>...</hash>
/// </trackChangeEncryption>
/// ```
#[derive(Debug)]
pub struct TrackChangeEncryption {
    /// ```xml
    /// <derivationKey
    ///   algorithm="{xs:string}"
    ///   size="{xs:nonNegativeInteger}"
    ///   count="{xs:nonNegativeInteger}"
    ///   salt="{xs:base64Binary}"
    /// />
    pub derivation_key: DerivationKey,
    /// ```xml
    /// <hash>...</hash>
    pub hash: xs::String,
}

impl TryFrom<AnyElement> for TrackChangeEncryption {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TRACK_CHANGE_ENCRYPTION)?;

        let mut derivation_key = None;
        let mut hash = None;

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__DERIVATION_KEY => {
                    derivation_key = Some(child.try_into()?)
                }
                ElementName::HANCOM__HEAD__HASH => match child.text {
                    Some(text) => hash = Some(text),
                    None => continue,
                },
                _ => continue,
            }
        }

        let (derivation_key, hash) = match (derivation_key, hash) {
            (Some(derivation_key), Some(hash)) => (derivation_key, hash),
            (None, _) => missing_element!("<derivationKey>"),
            (_, None) => missing_element!("<hash>"),
        };

        Ok(Self {
            derivation_key,
            hash,
        })
    }
}

/// ```xml
/// <derivationKey
///   algorithm="{xs:string}"
///   size="{xs:nonNegativeInteger}"
///   count="{xs:nonNegativeInteger}"
///   salt="{xs:base64Binary}"
/// />
/// ```
#[derive(Debug)]
pub struct DerivationKey {
    /// ```xml
    /// algorithm="{xs:string}"
    /// ```
    pub algorithm: xs::String,
    /// ```xml
    /// size="{xs:nonNegativeInteger}"
    /// ```
    pub size: xs::NonNegativeInteger32,
    /// ```xml
    /// count="{xs:nonNegativeInteger}"
    /// ```
    pub count: xs::NonNegativeInteger32,
    /// ```xml
    /// salt="{xs:base64Binary}"
    /// ```
    pub salt: xs::Base64Binary,
}

impl TryFrom<AnyElement> for DerivationKey {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__DERIVATION_KEY)?;

        let (algorithm, size, count, salt) = attributes!(element, "derivationKey";
            "algorithm" as algorithm => one (string),
            "size" as size => one xs::NonNegativeInteger32,
            "count" as count => one xs::NonNegativeInteger32,
            "salt" as salt => one (string),
        );

        Ok(Self {
            algorithm,
            size,
            count,
            salt,
        })
    }
}

/// ```xml
/// <metaTag>...</metaTag>
/// ```
#[derive(Debug)]
pub struct MetaTag {
    /// ```xml
    /// ... (xs:string)
    /// ```
    pub text: xs::String,
}

impl TryFrom<AnyElement> for MetaTag {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__META_TAG)?;

        let text = match element.text {
            Some(text) => text,
            None => missing_element!("<metaTag> text"),
        };

        Ok(Self { text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn begin_number() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:beginNum xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head" page="1" footnote="1" endnote="1" pic="1" tbl="1" equation="1" />"#;
        let element = AnyElement::from_bytes(XML)?;
        let begin_number = BeginNumber::try_from(element)?;

        insta::assert_debug_snapshot!(begin_number);

        Ok(())
    }

    #[test]
    fn font_face() -> Result<(), Error> {
        const XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:fontface lang="HANGUL" fontCnt="6" xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head">
  <hh:font id="0" face="돋움체" type="TTF" isEmbedded="0">
    <hh:typeInfo familyType="FCAT_GOTHIC" weight="6" proportion="9" contrast="0"
    strokeVariation="1" armStyle="1" letterform="1" midline="1" xHeight="1" />
  </hh:font>
</hh:fontface>"#;
        let element = AnyElement::from_bytes(XML.as_bytes())?;
        let font_face = FontFaceType::try_from(element)?;

        insta::assert_debug_snapshot!(font_face);

        Ok(())
    }

    #[test]
    fn border_fill() -> Result<(), Error> {
        const XML: &[u8] = br##"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:borderFill xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head" id="1" threeD="0" shadow="0" centerLine="NONE" breakCellSeparateLine="0">
  <hh:slash type="NONE" Crooked="0" isCounter="0" />
  <hh:backSlash type="NONE" Crooked="0" isCounter="0" />
  <hh:leftBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:rightBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:topBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:bottomBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:diagonal type="SOLID" width="0.1 mm" color="#000000" />
</hh:borderFill>"##;
        let element = AnyElement::from_bytes(XML)?;
        let border_fill = BorderFillType::try_from(element)?;

        insta::assert_debug_snapshot!(border_fill);

        Ok(())
    }

    #[test]
    fn border_fill_win_brush() -> Result<(), Error> {
        const XML: &[u8] = br##"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:borderFill
  xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head"
  xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core"
  id="3" threeD="0" shadow="0" centerLine="NONE" breakCellSeparateLine="0">
  <hh:slash type="NONE" Crooked="0" isCounter="0" />
  <hh:backSlash type="NONE" Crooked="0" isCounter="0" />
  <hh:leftBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:rightBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:topBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:bottomBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:diagonal type="SOLID" width="0.1 mm" color="#000000" />
  <hc:fillBrush>
    <hc:winBrush faceColor="none" hatchColor="#000000" alpha="0" />
  </hc:fillBrush>
</hh:borderFill>"##;
        let element = AnyElement::from_bytes(XML)?;
        let border_fill = BorderFillType::try_from(element)?;

        insta::assert_debug_snapshot!(border_fill);

        Ok(())
    }

    #[test]
    fn border_fill_gradation() -> Result<(), Error> {
        const XML: &[u8] = br##"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:borderFill
  xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head"
  xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core"
  id="3" threeD="0" shadow="0" centerLine="NONE" breakCellSeparateLine="0">
  <hh:slash type="NONE" Crooked="0" isCounter="0" />
  <hh:backSlash type="NONE" Crooked="0" isCounter="0" />
  <hh:leftBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:rightBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:topBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:bottomBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:diagonal type="SOLID" width="0.1 mm" color="#000000" />
  <hc:fillBrush>
    <hc:gradation type="LINEAR" angle="90" centerX="0" centerY="0" step="10" colorNum="2" stepCenter="0" alpha="0">
      <hh:color value="#FFFFFF" />
      <hh:color value="#000000" />
    </hc:gradation>
  </hc:fillBrush>
</hh:borderFill>"##;
        let element = AnyElement::from_bytes(XML)?;
        let border_fill = BorderFillType::try_from(element)?;

        insta::assert_debug_snapshot!(border_fill);

        Ok(())
    }

    #[test]
    fn border_fill_image_brush() -> Result<(), Error> {
        const XML: &[u8] = br##"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:borderFill
  xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head"
  xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core"
  id="3" threeD="0" shadow="0" centerLine="NONE" breakCellSeparateLine="0">
  <hh:slash type="NONE" Crooked="0" isCounter="0" />
  <hh:backSlash type="NONE" Crooked="0" isCounter="0" />
  <hh:leftBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:rightBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:topBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:bottomBorder type="NONE" width="0.1 mm" color="#000000" />
  <hh:diagonal type="SOLID" width="0.1 mm" color="#000000" />
  <hc:fillBrush>
    <hc:imgBrush mode="TILE">
      <hc:img binaryItemIDRef="BINARY_ID_1" bright="0" contrast="0" effect="REAL_PIC" alpha="0" />
    </hc:imgBrush>
  </hc:fillBrush>
</hh:borderFill>"##;
        let element = AnyElement::from_bytes(XML)?;
        let border_fill = BorderFillType::try_from(element)?;

        insta::assert_debug_snapshot!(border_fill);

        Ok(())
    }

    #[test]
    fn char_shape() -> Result<(), Error> {
        const XML: &str = r##"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:charPr
  xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head" 
  id="0" height="1000" textColor="#000000" shadeColor="none" useFontSpace="0"
  useKerning="0" symMark="NONE" borderFillIDRef="2">
  <hh:fontRef hangul="4" latin="4" hanja="4" japanese="4" other="4" symbol="4" user="4" />
  <hh:ratio hangul="100" latin="100" hanja="100" japanese="100" other="100" symbol="100"
    user="100" />
  <hh:spacing hangul="0" latin="0" hanja="0" japanese="0" other="0" symbol="0" user="0" />
  <hh:relSz hangul="100" latin="100" hanja="100" japanese="100" other="100" symbol="100"
    user="100" />
  <hh:offset hangul="0" latin="0" hanja="0" japanese="0" other="0" symbol="0" user="0" />
  <hh:italic />
  <hh:bold />
  <hh:underline type="NONE" shape="NONE" color="#000000" />
  <hh:strikeout shape="NONE" color="#000000" />
  <hh:outline type="NONE" />
  <hh:shadow type="NONE" color="#000000" offsetX="0" offsetY="0" />
  <hh:emboss />
  <hh:engrave />
  <hh:supscript />
  <hh:subscript />
</hh:charPr>"##;
        let element = AnyElement::from_bytes(XML.as_bytes())?;
        let char_shape = CharShapeType::try_from(element)?;

        insta::assert_debug_snapshot!(char_shape);

        Ok(())
    }

    #[test]
    fn tab_definition() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:tabDef xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head" id="1" autoTabLeft="0" autoTabRight="0">
  <hh:tabItem pos="36" type="LEFT" leader="NONE" />
</hh:tabDef>"#;
        let element = AnyElement::from_bytes(XML)?;
        let tab_definition = TabDefType::try_from(element)?;

        insta::assert_debug_snapshot!(tab_definition);

        Ok(())
    }

    #[test]
    fn numbering_type() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:numbering
  xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head"
  id="1" start="0">
  <hh:paraHead start="1" level="1" align="LEFT" useInstWidth="1" autoIndent="1"
    widthAdjust="0" textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT"
    charPrIDRef="4294967295" checkable="0">^1.</hh:paraHead>
  <hh:paraHead start="1" level="2" align="LEFT" useInstWidth="1" autoIndent="1"
    widthAdjust="0" textOffsetType="PERCENT" textOffset="50" numFormat="HANGUL_SYLLABLE"
    charPrIDRef="4294967295" checkable="0">^2.</hh:paraHead>
  <hh:paraHead start="1" level="3" align="LEFT" useInstWidth="1" autoIndent="1"
    widthAdjust="0" textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT"
    charPrIDRef="4294967295" checkable="0">^3)</hh:paraHead>
  <hh:paraHead start="1" level="4" align="LEFT" useInstWidth="1" autoIndent="1"
    widthAdjust="0" textOffsetType="PERCENT" textOffset="50" numFormat="HANGUL_SYLLABLE"
    charPrIDRef="4294967295" checkable="0">^4)</hh:paraHead>
  <hh:paraHead start="1" level="5" align="LEFT" useInstWidth="1" autoIndent="1"
    widthAdjust="0" textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT"
    charPrIDRef="4294967295" checkable="0">(^5)</hh:paraHead>
  <hh:paraHead start="1" level="6" align="LEFT" useInstWidth="1" autoIndent="1"
    widthAdjust="0" textOffsetType="PERCENT" textOffset="50" numFormat="HANGUL_SYLLABLE"
    charPrIDRef="4294967295" checkable="0">(^6)</hh:paraHead>
  <hh:paraHead start="1" level="7" align="LEFT" useInstWidth="1" autoIndent="1"
    widthAdjust="0" textOffsetType="PERCENT" textOffset="50" numFormat="CIRCLED_DIGIT"
    charPrIDRef="4294967295" checkable="1">^7</hh:paraHead>
  <hh:paraHead start="1" level="8" align="LEFT" useInstWidth="0" autoIndent="1"
    widthAdjust="0" textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT"
    charPrIDRef="4294967295" checkable="0" />
  <hh:paraHead start="1" level="9" align="LEFT" useInstWidth="0" autoIndent="1"
    widthAdjust="0" textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT"
    charPrIDRef="4294967295" checkable="0" />
  <hh:paraHead start="1" level="10" align="LEFT" useInstWidth="0" autoIndent="1"
    widthAdjust="0" textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT"
    charPrIDRef="4294967295" checkable="0" />
</hh:numbering>"#;
        let element = AnyElement::from_bytes(XML)?;
        let numbering_type = NumberingType::try_from(element)?;

        insta::assert_debug_snapshot!(numbering_type);

        Ok(())
    }

    #[test]
    fn bullet_type() -> Result<(), Error> {
        const XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:bullet
  xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head"
  xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core"
  id="1" char="●" checkedChar="■" useImage="0">
  <hc:img
    xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core"
    binaryItemIDRef="BINARY_ID_1" bright="0" contrast="0" effect="REAL_PIC"
    alpha="0" />
  <hh:paraHead start="1" level="1" align="LEFT" useInstWidth="1" autoIndent="1"
    widthAdjust="0" textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT"
    charPrIDRef="4294967295" checkable="0">●</hh:paraHead>
</hh:bullet>"#;
        let element = AnyElement::from_bytes(XML.as_bytes())?;
        let bullet_type = BulletType::try_from(element)?;

        insta::assert_debug_snapshot!(bullet_type);

        Ok(())
    }

    #[test]
    fn paragraph_shape() -> Result<(), Error> {
        const XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:paraPr
  xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head"
  xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core"
  id="0" tabPrIDRef="0" condense="0" fontLineHeight="0" snapToGrid="1"
  suppressLineNumbers="0" checked="0">
  <hh:align horizontal="JUSTIFY" vertical="BASELINE" />
  <hh:heading type="NONE" idRef="0" level="0" />
  <hh:breakSetting breakLatinWord="KEEP_WORD" breakNonLatinWord="KEEP_WORD" widowOrphan="0"
    keepWithNext="0" keepLines="0" pageBreakBefore="0" lineWrap="BREAK" />
  <hh:autoSpacing eAsianEng="0" eAsianNum="0" />
  <hh:margin>
    <hc:intent value="0" unit="HWPUNIT" />
    <hc:left value="0" unit="HWPUNIT" />
    <hc:right value="0" unit="HWPUNIT" />
    <hc:prev value="0" unit="HWPUNIT" />
    <hc:next value="0" unit="HWPUNIT" />
  </hh:margin>
  <hh:lineSpacing type="PERCENT" value="160" unit="HWPUNIT" />
  <hh:border borderFillIDRef="2" offsetLeft="0" offsetRight="0" offsetTop="0" offsetBottom="0"
    connect="0" ignoreMargin="0" />
</hh:paraPr>"#;
        let element = AnyElement::from_bytes(XML.as_bytes())?;
        let paragraph_shape = ParaShapeType::try_from(element)?;

        insta::assert_debug_snapshot!(paragraph_shape);

        Ok(())
    }

    #[test]
    fn style_type() -> Result<(), Error> {
        const XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:style
  xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head"
  id="0" type="PARA" name="바탕글" engName="Normal" paraPrIDRef="0" charPrIDRef="0"
  nextStyleIDRef="0" langID="1042" lockForm="0" />"#;
        let element = AnyElement::from_bytes(XML.as_bytes())?;
        let style_type = StyleType::try_from(element)?;

        insta::assert_debug_snapshot!(style_type);

        Ok(())
    }

    #[test]
    fn memo_shape_type() -> Result<(), Error> {
        const XML: &[u8] = br##"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:memoPr
  xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head"
  id="1" width="2000" lineWidth="0.25 mm" lineType="SOLID" lineColor="#FF0000"
  fillColor="#FFFF00" activeColor="#00FF00" memoType="NORMAL" />"##;
        let element = AnyElement::from_bytes(XML)?;
        let memo_properties = MemoShapeType::try_from(element)?;

        insta::assert_debug_snapshot!(memo_properties);

        Ok(())
    }

    #[test]
    fn track_change() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:trackChange
  xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head"
  type="Insert" date="2023-10-01T12:34:56Z" authorID="2"
  charPrIDRef="5" paraPrIDRef="3" hide="false" id="10" />"#;
        let element = AnyElement::from_bytes(XML)?;
        let track_change = TrackChange::try_from(element)?;

        insta::assert_debug_snapshot!(track_change);

        Ok(())
    }

    #[test]
    fn track_change_author() -> Result<(), Error> {
        const XML: &[u8] = br##"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:trackChangeAuthor
  xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head"
  name="Alice" mark="true" color="#FF0000" id="1" />"##;
        let element = AnyElement::from_bytes(XML)?;
        let track_change_author = TrackChangeAuthor::try_from(element)?;

        insta::assert_debug_snapshot!(track_change_author);

        Ok(())
    }

    #[test]
    fn forbidden_words() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:forbiddenWordList xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head">
  <hh:forbiddenWord>foo</hh:forbiddenWord>
  <hh:forbiddenWord>bar</hh:forbiddenWord>
  <hh:forbiddenWord>baz</hh:forbiddenWord>
</hh:forbiddenWordList>"#;
        let element = AnyElement::from_bytes(XML)?;
        let forbidden_words = ForbiddenWordList::try_from(element)?;

        insta::assert_debug_snapshot!(forbidden_words);

        Ok(())
    }

    #[test]
    fn compatible_document() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:compatibleDocument xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head" targetProgram="HWP201X">
  <hh:layoutCompatibility>
    <hh:applyFontWeightToBold />
    <hh:useInnerUnderline />
    <hh:fixedUnderlineWidth />
    <hh:doNotApplyStrikeoutWithUnderline />
    <hh:useLowercaseStrikeout />
    <hh:extendLineheightToOffset />
    <hh:applyFontspaceToLatin />
    <hh:treatQuotationAsLatin />
    <hh:doNotApplyDiacSymMarkOfNoneAndSix />
    <hh:doNotAlignWhitespaceOnRight />
    <hh:doNotAdjustWordInJustify />
    <hh:baseCharUnitOnEAsian />
    <hh:baseCharUnitOfIndentOnFirstChar />
    <hh:adjustLineheightToFont />
    <hh:adjustBaseInlineFixedLinespacing />
    <hh:applyPrevspacingBeneathObject />
    <hh:applyNextspacingOfLastPara />
    <hh:applyAtLeastToPercent100Pct />
    <hh:doNotApplyAutoSpaceEAsianEng />
    <hh:doNotApplyAutoSpaceEAsianNum />
    <hh:adjustParaBorderfillToSpacing />
    <hh:connectParaBorderfillOfEqualBorder />
    <hh:adjustParaBorderOffsetWithBorder />
    <hh:extendLineheightToParaBorderOffset />
    <hh:applyParaBorderToOutside />
    <hh:applyMinColumnWidthTo1mm />
    <hh:applyTabPosBasedOnSegment />
    <hh:breakTabOverline />
    <hh:adjustVertPosOfLine />
    <hh:doNotApplyWhiteSpaceHeight />
    <hh:doNotAlignLastPeriod />
    <hh:doNotAlignLastForbidden />
    <hh:baseLineSpacingOnLineGrid />
    <hh:applyCharSpacingToCharGrid />
    <hh:doNotApplyGridInHeaderFooter />
    <hh:applyExtendHeaderFooterEachSection />
    <hh:doNotApplyHeaderFooterAtNoSpace />
    <hh:doNotApplyColSeparatorAtNoGap />
    <hh:doNotApplyLinegridAtNoLinespacing />
    <hh:doNotApplyImageEffect />
    <hh:doNotApplyShapeComment />
    <hh:doNotAdjustEmptyAnchorLine />
    <hh:overlapBothAllowOverlap />
    <hh:doNotApplyVertOffsetOfForward />
    <hh:extendVertLimitToPageMargins />
    <hh:doNotHoldAnchorOfTable />
    <hh:doNotFormattingAtBeneathAnchor />
    <hh:adjustBaselineOfObjectToBottom />
    <hh:doNotApplyExtensionCharCompose />
  </hh:layoutCompatibility>
</hh:compatibleDocument>"#;
        let element = AnyElement::from_bytes(XML)?;
        let compatible_document = CompatibleDocument::try_from(element)?;

        insta::assert_debug_snapshot!(compatible_document);

        Ok(())
    }

    #[test]
    fn track_change_config() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:trackchangeConfig xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head" flag="3">
  <hh:trackChangeEncryption>
    <hh:derivationKey algorithm="SHA-256" size="32" count="1000" salt="c29tZXNhbHQ=" />
    <hh:hash>dGVzdGhhc2g=</hh:hash>
  </hh:trackChangeEncryption>
</hh:trackchangeConfig>"#;
        let element = AnyElement::from_bytes(XML)?;
        let track_change_config = TrackChangeConfig::try_from(element)?;

        insta::assert_debug_snapshot!(track_change_config);

        Ok(())
    }

    #[test]
    fn document_option() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:docOption xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head">
  <hh:linkInfo path="C:\Documents\" pageInherit="true" footnoteInherit="false" />
  <hh:licenseMark type="2" flag="1" lang="0" />
</hh:docOption>"#;
        let element = AnyElement::from_bytes(XML)?;
        let document_option = DocumentOption::try_from(element)?;

        insta::assert_debug_snapshot!(document_option);

        Ok(())
    }

    #[test]
    fn meta_tag() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hh:metaTag xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head">This is a meta tag.</hh:metaTag>"#;
        let element = AnyElement::from_bytes(XML)?;
        let meta_tag = MetaTag::try_from(element)?;

        insta::assert_debug_snapshot!(meta_tag);

        Ok(())
    }
}
