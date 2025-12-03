//! namespace: http://www.hancom.co.kr/hwpml/2011/head
//! filename: Contents/header.xml

use crate::{
    any_element::{AnyElement, ElementName},
    core::{
        BreakLatinWordKind, BreakNonLatinWordKind, CenterLine, FamilyType, FontKind, GradationKind,
        HWPUnit, HWPValue, HatchStyle, ImageBrushMode, ImageEffect, Language,
        LayoutCompatibilityKind, LineSpacingKind, LineType1, LineType2, LineWidth, LineWrapKind,
        MemoKind, NumberType1, ParagraphHeadingKind, ParagraphHorizontalAlignKind,
        ParagraphVerticalAlignKind, RgbColor, ShadowKind, SlashKind, StyleKind, SymbolMark,
        TabItemKind, TargetProgram, TextOffsetKind, TrackChangeKind, UnderlineKind,
    },
    error::Error,
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
    pub page: u32,
    /// ```xml
    /// footnote="{xs:positiveInteger}"
    /// ```
    ///
    /// 각주 시작 번호
    pub foot_note: u32,
    /// ```xml
    /// endnote="{xs:positiveInteger}"
    /// ```
    ///
    /// 미주 시작 번호
    pub end_note: u32,
    /// ```xml
    /// picture="{xs:positiveInteger}"
    /// ```
    ///
    /// 그림 시작 번호
    pub picture: u32,
    /// ```xml
    /// tbl="{xs:positiveInteger}"
    /// ```
    ///
    /// 표 시작 번호
    pub table: u32,
    /// ```xml
    /// equation="{xs:positiveInteger}"
    /// ```
    ///
    /// 수식 시작 번호
    pub equation: u32,
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
    /// <fontfaces itemCnt="{xs:positiveInteger > 0}">
    ///   <fontface ...>...</fontface>
    /// </fontfaces>
    /// ```
    ///
    /// 글꼴 리스트
    pub font_faces: Vec<FontFaceType>,
    /// ```xml
    /// <borderFills itemCnt="{xs:positiveInteger > 0}">
    ///   <borderFill ...>...</borderFill>
    /// </borderFills>
    /// ```
    ///
    /// 테두리/배경/채우기
    pub border_fills: Vec<BorderFillType>,
    /// ```xml
    /// <charProperties itemCnt="{xs:positiveInteger > 0}">
    ///   <charPr ...>...</charPr>
    /// </charProperties>
    /// ```
    ///
    /// 글자 모양 정보
    pub character_properties: NonEmpty<CharShapeType>,
    /// ```xml
    /// <tabProperties itemCnt="{xs:positiveInteger > 0}">
    ///   <tabPr ...>...</tabPr>
    /// </tabProperties>
    /// ```
    ///
    /// 탭 정의 정보
    pub tab_properties: Vec<TabDefType>,
    /// ```xml
    /// <numberings itemCnt="{xs:positiveInteger > 0}">
    ///   <numbering ...>...</numbering>
    /// </numberings>
    /// ```
    ///
    /// 번호 문단 정보
    pub numberings: Vec<NumberingType>,
    /// ```xml
    /// <bullets itemCnt="{xs:positiveInteger > 0}">
    ///   <bullet ...>...</bullet>
    /// </bullets>
    /// ```
    ///
    /// 글머리표 문단 정보
    pub bullets: Vec<BulletType>,
    /// ```xml
    /// <paraProperties itemCnt="{xs:positiveInteger > 0}">
    ///   <paraShape ...>...</paraShape>
    /// </paraProperties>
    /// ```
    ///
    /// 문단 모양
    pub paragraph_properties: NonEmpty<ParaShapeType>,
    /// ```xml
    /// <styles itemCnt="{xs:positiveInteger > 0}">
    ///   <style ...>...</style>
    /// </styles>
    /// ```
    ///
    /// 스타일
    pub styles: Vec<StyleType>,
    /// ```xml
    /// <memoProperties itemCnt="{xs:positiveInteger > 0}">
    ///   <memo ...>...</memo>
    /// </memoProperties>
    /// ```
    ///
    /// 메모 모양
    pub memo: Vec<MemoShapeType>,
    /// ```xml
    /// <trackChanges itemCnt="{xs:positiveInteger > 0}">
    ///   <trackChange ...>...</trackChange>
    /// </trackChanges>
    /// ```
    ///
    /// 변경 추적
    pub track_changes: Vec<TrackChange>,
    /// ```xml
    /// <trackChangeAuthors itemCnt="{xs:positiveInteger > 0}">
    ///   <trackChangeAuthor ...>...</trackChangeAuthor>
    /// </trackChangeAuthors>
    /// ```
    ///
    /// 변경 추적 작성자
    pub track_change_authors: Vec<TrackChangeAuthor>,
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
    pub language: Language,
    /// ```xml
    /// <font ...>...</font>
    /// ```
    ///
    /// 글꼴 리스트
    pub fonts: NonEmpty<Font>,
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
    pub id: u32,
    /// ```xml
    /// face="{xs:string}"
    /// ```
    ///
    /// 글꼴 이름
    pub face: String,
    /// ```xml
    /// type="{$FontKind}"
    /// ```
    ///
    /// 글꼴의 유형(rep: 대표글꼴, ttf: 트루타입글꼴, hft: 한/글 전용 글꼴)
    pub r#type: FontKind,
    /// ```xml
    /// isEmbedded="{xs:boolean; default="false"}"
    /// ```
    pub embedded: bool,
    /// ```xml
    /// binaryItemIDRef="{xs:string}"
    /// ```
    pub binary_item_id_ref: Option<String>,
    /// ```xml
    /// <substFont ...>...</substFont>
    /// ```
    pub subset: Option<SubsetFont>,
    /// ```xml
    /// <typeInfo ...>...</typeInfo>
    /// ```
    pub type_info: Option<TypeInfo>,
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
    pub face: String,
    /// ```xml
    /// type="{xs:string}"
    /// ```
    ///
    /// 글꼴의 유형
    pub r#type: FontKind,
    /// ```xml
    /// isEmbedded="{xs:boolean; default="false"}"
    /// ```
    pub embedded: bool,
    /// ```xml
    /// binaryItemIDRef="{xs:string}"
    /// ```
    pub binary_item_id_ref: Option<String>,
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
    pub family_type: FamilyType,
    /// ```xml
    /// serifStyle="{xs:string}"
    /// ```
    ///
    /// 세리프 유형
    pub serif_style: Option<String>,
    /// ```xml
    /// weight="{xs:integer}"
    /// ```
    ///
    /// 굵기
    pub weight: isize,
    /// ```xml
    /// proportion="{xs:integer}"
    /// ```
    ///
    /// 비율
    pub proportion: isize,
    /// ```xml
    /// contrast="{xs:integer}"
    /// ```
    ///
    /// 대조
    pub contrast: isize,
    /// ```xml
    /// strokeVariation="{xs:integer}"
    /// ```
    ///
    /// 스트로크 편차
    pub stroke_variation: isize,
    /// ```xml
    /// armStyle="{xs:boolean}"
    /// ```
    ///
    /// 자획모양
    pub arm_style: bool,
    /// ```xml
    /// letterForm="{xs:boolean}"
    /// ```
    ///
    /// 글자형
    pub letter_form: bool,
    /// ```xml
    /// midline="{xs:boolean}"
    /// ```
    ///
    /// 중간선
    pub midline: bool,
    /// ```xml
    /// xHeight="{xs:integer}"
    /// ```
    ///
    /// X-높이
    pub x_height: isize,
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
    pub id: u32,
    /// ```xml
    /// effect3D="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 3D효과 on/off
    pub effect_3d: bool,
    /// ```xml
    /// shadow="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 그림자 효과 on/off
    pub shadow: bool,
    /// ```xml
    /// centerLine="{$CenterLine}"
    /// ```
    ///
    /// 중심선 종류
    pub center_line: CenterLine,
    /// ```xml
    /// breakCellSeparateLine="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 자동으로 나뉜 표의 경계선 설정 여부
    pub break_cell_separate_line: bool,
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
    pub r#type: SlashKind,
    /// ```xml
    /// Crooked="{xs:boolean}"
    /// ```
    pub crooked: bool,
    /// ```xml
    /// isCounter="{xs:boolean}"
    /// ```
    pub counter: bool,
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
    pub r#type: LineType2,
    /// ```xml
    /// width="{hc:LineWidth}"
    /// ```
    ///
    /// 테두리선 굵기
    pub width: LineWidth,
    /// ```xml
    /// color="{hc:RGBColorType}"
    /// ```
    ///
    /// 테두리선 색상
    pub color: RgbColor,
}

#[derive(Debug)]
pub enum FillBrush {
    WinBrush(WinBrush),
    Gradation(Gradation),
    Image(ImageBrush),
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
    pub face_color: RgbColor,
    /// ```xml
    /// hatchColor="{hc:RGBColorType; default="#000000"}"
    /// ```
    ///
    /// 무늬 색
    pub hatch_color: RgbColor,
    /// ```xml
    /// hatchStyle="{xs:string}"
    /// ```
    ///
    /// 무늬 종류
    pub hatch_style: Option<HatchStyle>,
    /// ```xml
    /// alpha="{xs:float}"
    /// ```
    pub alpha: f32,
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
    pub r#type: GradationKind,
    /// ```xml
    /// angle="{xs:integer; default="90"}"
    /// ```
    ///
    /// 그러데이션의 기울임(시작각)
    pub angle: isize,
    /// ```xml
    /// centerX="{xs:integer; default="0"}"
    /// ```
    ///
    /// 그러데이션의 가로중심(중심 X좌표)
    pub center_x: isize,
    /// ```xml
    /// centerY="{xs:integer; default="0"}"
    /// ```
    ///
    /// 그러데이션의 세로중심(중심 Y좌표)
    pub center_y: isize,
    /// ```xml
    /// step="{xs:integer; >= 0; <= 255}"
    /// ```
    ///
    /// 그러데이션 번짐정도 (0~255)
    pub step: u8,
    /// ```xml
    /// colorNum="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 그러데이션의 개수
    pub color_number: u8,
    /// ```xml
    /// stepCenter="{xs:integer; >= 0; <= 10}"
    /// ```
    ///
    /// 그러데이션 번짐정도의 중심 (0~100)
    pub step_center: usize,
    /// ```xml
    /// alpha="{xs:float}"
    /// ```
    pub alpha: f32,
    /// ```xml
    /// <color value="{hc:RGBColorType}" />
    /// ```
    ///
    /// 그러데이션 색
    pub colors: Vec<GradationColor>,
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
    pub value: RgbColor,
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
    pub mode: ImageBrushMode,
    /// ```xml
    /// <img ...>...</img>
    /// ```
    pub image: ImageType,
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
    pub binary_id_ref: String,
    /// ```xml
    /// bright="{xs:integer; default="0"}"
    /// ```
    ///
    /// 밝기
    pub bright: isize,
    /// ```xml
    /// contrast="{xs:integer; default="0"}"
    /// ```
    ///
    /// 대비
    pub contrast: isize,
    /// ```xml
    /// effect="{$ImageEffect; default="REALPIC"}"
    /// ```
    ///
    /// 효과
    pub effect: ImageEffect,
    /// ```xml
    /// alpha="{xs:float}"
    /// ```
    pub alpha: f32,
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
    pub id: u32,
    /// ```xml
    /// height="{xs:integer; default="1000"}"
    /// ```
    ///
    /// 글자 크기 (hwpunit 단위, 10 pt = 1000 hwpunit)
    pub height: usize,
    /// ```xml
    /// textColor="{hc:RGBColorType; default="#000000"}"
    /// ```
    ///
    /// 글자색
    pub text_color: RgbColor,
    /// ```xml
    /// shadeColor="{hc:RGBColorType; default="#FFFFFF"}"
    /// ```
    ///
    /// 음영색
    pub shade_color: RgbColor,
    /// ```xml
    /// useFontSpace="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 글꼴에 어울리는 빈칸
    pub use_font_space: bool,
    /// ```xml
    /// useKerning="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 커닝
    pub use_kerning: bool,
    /// ```xml
    /// symMark="{$SymbolMark; default="None"}"
    /// ```
    ///
    /// 강조점 종류
    pub symbol_mark: SymbolMark,
    /// ```xml
    /// borderFillIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 글자테두리 기능
    pub border_fill_id_ref: Option<u32>,
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
    pub italic: bool,
    /// ```xml
    /// <bold />
    /// ```
    ///
    /// 글자 속성: 진하게.
    pub bold: bool,
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
    pub emboss: bool,
    /// ```xml
    /// <engrave />
    /// ```
    ///
    /// 글자 속성: 음각.
    pub engrave: bool,
    /// ```xml
    /// <supscript />
    /// ```
    ///
    /// 글자 속성: 위첨자.
    pub superscript: bool,
    /// ```xml
    /// <subscript />
    /// ```
    ///
    /// 글자 속성: 아래첨자.
    pub subscript: bool,
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
    pub hangul: u32,
    /// ```xml
    /// latin="{xs:nonNegativeInteger}"
    /// ```
    pub latin: u32,
    /// ```xml
    /// hanja="{xs:nonNegativeInteger}"
    /// ```
    pub hanja: u32,
    /// ```xml
    /// japanese="{xs:nonNegativeInteger}"
    /// ```
    pub japanese: u32,
    /// ```xml
    /// other="{xs:nonNegativeInteger}"
    /// ```
    pub other: u32,
    /// ```xml
    /// symbol="{xs:nonNegativeInteger}"
    /// ```
    pub symbol: u32,
    /// ```xml
    /// user="{xs:nonNegativeInteger}"
    /// ```
    pub user: u32,
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
    pub hangul: u8,
    /// ```xml
    /// latin="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub latin: u8,
    /// ```xml
    /// hanja="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub hanja: u8,
    /// ```xml
    /// japanese="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub japanese: u8,
    /// ```xml
    /// other="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub other: u8,
    /// ```xml
    /// symbol="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub symbol: u8,
    /// ```xml
    /// user="{xs:positiveInteger; >= 50; <= 200; default="100"}"
    /// ```
    pub user: u8,
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
    pub hangul: i8,
    /// ```xml
    /// latin="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub latin: i8,
    /// ```xml
    /// hanja="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub hanja: i8,
    /// ```xml
    /// japanese="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub japanese: i8,
    /// ```xml
    /// other="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub other: i8,
    /// ```xml
    /// symbol="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub symbol: i8,
    /// ```xml
    /// user="{xs:integer; >= -50; <= 50; default="0"}"
    /// ```
    pub user: i8,
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
    pub hangul: u8,
    /// ```xml
    /// latin="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub latin: u8,
    /// ```xml
    /// hanja="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub hanja: u8,
    /// ```xml
    /// japanese="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub japanese: u8,
    /// ```xml
    /// other="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub other: u8,
    /// ```xml
    /// symbol="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub symbol: u8,
    /// ```xml
    /// user="{xs:positiveInteger; >= 10; <= 250; default="100"}"
    /// ```
    pub user: u8,
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
    pub hangul: i8,
    /// ```xml
    /// latin="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub latin: i8,
    /// ```xml
    /// hanja="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub hanja: i8,
    /// ```xml
    /// japanese="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub japanese: i8,
    /// ```xml
    /// other="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub other: i8,
    /// ```xml
    /// symbol="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub symbol: i8,
    /// ```xml
    /// user="{xs:integer; >= -100; <= 100; default="0"}"
    /// ```
    pub user: i8,
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
    pub r#type: UnderlineKind,
    /// ```xml
    /// shape="{hc:LineType2}"
    /// ```
    pub shape: LineType2,
    /// ```xml
    /// color="{hc:RGBColorType}"
    /// ```
    pub color: RgbColor,
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
    pub shape: LineType2,
    /// ```xml
    /// color="{hc:RGBColorType}"
    /// ```
    pub color: RgbColor,
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
    pub r#type: LineType1,
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
    pub r#type: ShadowKind,
    /// ```xml
    /// color="{hc:RGBColorType}"
    /// ```
    ///
    /// 그림자 색
    pub color: RgbColor,
    /// ```xml
    /// offsetX="{xs:integer; >= -100; <= 100}"
    /// ```
    ///
    /// 그림자 간격 X. 단위는 %.
    pub offset_x: i8,
    /// ```xml
    /// offsetY="{xs:integer; >= -100; <= 100}"
    /// ```
    ///
    /// 그림자 간격 Y. 단위는 %.
    pub offset_y: i8,
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
    pub id: u32,
    /// ```xml
    /// autoTabLeft="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 문단 왼쪽 끝 자동 탭(내어쓰기용 자동 탭)
    pub auto_tab_left: bool,
    /// ```xml
    /// autoTabRight="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 문단 오른쪽 끝 자동 탭
    pub auto_tab_right: bool,
    /// ```xml
    /// <tabItem ... />
    /// ```
    pub tab: Option<TabItem>,
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
    pub position: isize,
    /// ```xml
    /// type="{$TabItemKind}"
    /// ```
    ///
    /// 탭의 종류
    pub r#type: TabItemKind,
    /// ```xml
    /// leader="{hc:LineType2}"
    /// ```
    ///
    /// 채움 종류
    pub leader: LineType2,
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
    pub id: u32,
    /// ```xml
    /// start="{xs:integer; default="1"}"
    /// ```
    ///
    /// 시작 번호
    pub start: isize,
    /// ```xml
    /// <paraHead ...>...</paraHead>
    /// ```
    pub heads: Vec<ParagraphHeadType>,
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
    pub id: u32,
    /// ```xml
    /// tabPrIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 탭 정의 아이디 참조
    pub tab_pr_id_ref: Option<u32>,
    /// ```xml
    /// condense="{xs:integer; >= 0; <= 75; default="0"}"
    /// ```
    ///
    /// 문단 압축률
    pub condense: u8,
    /// ```xml
    /// fontLineHeight="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 글자 기준 줄 높이 사용 여부
    pub font_line_height: bool,
    /// ```xml
    /// snapToGrid="{xs:boolean; default="true"}"
    /// ```
    ///
    /// 격자에 맞춤 여부
    pub snap_to_grid: bool,
    /// ```xml
    /// suppressLineNumbers="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 줄 번호 표시 안함 여부
    pub suppress_line_numbers: bool,
    /// ```xml
    /// checked="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 체크된 문단 여부
    pub checked: bool,
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
    pub horizontal: ParagraphHorizontalAlignKind,
    /// ```xml
    /// vertical="{$ParagraphVerticalAlignKind}"
    /// ```
    ///
    /// 수직 정렬
    pub vertical: ParagraphVerticalAlignKind,
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
    pub r#type: ParagraphHeadingKind,
    /// ```xml
    /// idRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 번호 문단 모양 아이디 참조 또는 글머리표 아이디 참조
    pub id_ref: Option<u32>,
    /// ```xml
    /// level="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 레벨
    pub level: usize,
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
    pub break_latin_word: BreakLatinWordKind,
    /// ```xml
    /// breakNonLatinWord="{$BreakNonLatinWordKind}"
    /// ```
    ///
    /// 라틴 문자 이외의 문자의 줄나눔 단위
    pub break_non_latin_word: BreakNonLatinWordKind,
    /// ```xml
    /// widowOrphan="{xs:boolean}"
    /// ```
    ///
    /// 외톨이줄 보호 여부
    pub widow_orphan: bool,
    /// ```xml
    /// keepWithNext="{xs:boolean}"
    /// ```
    ///
    /// 다음 문단과 함께
    pub keep_with_next: bool,
    /// ```xml
    /// keepLines="{xs:boolean}"
    /// ```
    ///
    /// 문단 보호 여부
    pub keep_lines: bool,
    /// ```xml
    /// pageBreakBefore="{xs:boolean}"
    /// ```
    ///
    /// 문단 앞에서 항상 쪽나눔 여부
    pub page_break_before: bool,
    /// ```xml
    /// lineWrap="{$LineWrapKind}"
    /// ```
    ///
    /// 한 줄로 입력 사용 시의 형식
    pub line_wrap: LineWrapKind,
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
    pub indent: HWPValue,
    /// ```xml
    /// <left value="{xs:integer}" unit="{hc:HWPUNIT}" />
    /// ```
    ///
    /// 왼쪽 여백
    ///
    /// 단위를 표기하지 않으면 hwpunit이고 표기하면 표기한 단위로.
    pub left: HWPValue,
    /// ```xml
    /// <right value="{xs:integer}" unit="{hc:HWPUNIT}" />
    /// ```
    ///
    /// 오른쪽 여백
    pub right: HWPValue,
    /// ```xml
    /// <prev value="{xs:integer}" unit="{hc:HWPUNIT}" />
    /// ```
    ///
    /// 문단 간격 위
    pub previous: HWPValue,
    /// ```xml
    /// <next value="{xs:integer}" unit="{hc:HWPUNIT}" />
    /// ```
    ///
    /// 문단 간격 아래
    pub next: HWPValue,
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
    pub r#type: LineSpacingKind,
    /// ```xml
    /// value="{xs:integer}"
    /// ```
    ///
    /// 줄 간격 값
    ///
    /// type이 PERCENT이면 0%~500%로 제한.
    pub value: u16,
    /// ```xml
    /// unit="{hc:HWPUNIT}"
    /// ```
    ///
    /// 줄 간격 값의 단위
    pub unit: HWPUnit,
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
    pub border_fill_id_ref: Option<u32>,
    /// ```xml
    /// offsetLeft="{xs:integer; default="0"}"
    /// ```
    ///
    /// 문단 테두리 왼쪽 간격. 단위는 hwpunit.
    pub offset_left: isize,
    /// ```xml
    /// offsetRight="{xs:integer; default="0"}"
    /// ```
    ///
    /// 문단 테두리 오른쪽 간격. 단위는 hwpunit.
    pub offset_right: isize,
    /// ```xml
    /// offsetTop="{xs:integer; default="0"}"
    /// ```
    ///
    /// 문단 테두리 위쪽 간격. 단위는 hwpunit.
    pub offset_top: isize,
    /// ```xml
    /// offsetBottom="{xs:integer; default="0"}"
    /// ```
    ///
    /// 문단 테두리 아래쪽 간격. 단위는 hwpunit.
    pub offset_bottom: isize,
    /// ```xml
    /// connect="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 문단 테두리 연결 여부
    pub connect: bool,
    /// ```xml
    /// ignoreMargin="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 문단 테두리 여백 무시 여부
    pub ignore_margin: bool,
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
    pub e_asian_eng: bool,
    /// ```xml
    /// eAsianNum="{xs:boolean}"
    /// ```
    ///
    /// 한글과 숫자 간격을 자동 조절
    pub e_asian_num: bool,
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
    pub start: u32,
    /// ```xml
    /// level="{xs:positiveInteger}"
    /// ```
    ///
    /// 레벨
    pub level: usize,
    /// ```xml
    /// align="{$ParagraphHeadAlign; default="LEFT"}"
    /// ```
    ///
    /// 정렬 방식
    pub align: ParagraphHorizontalAlignKind,
    /// ```xml
    /// useInstWidth="{xs:boolean; default="true"}"
    /// ```
    ///
    /// 고정폭 사용 여부
    pub use_inset_width: bool,
    /// ```xml
    /// autoIndent="{xs:boolean; default="true"}"
    /// ```
    ///
    /// 자동 들여쓰기 여부
    pub auto_indent: bool,
    /// ```xml
    /// widthAdjust="{xs:integer; default="0"}"
    /// ```
    ///
    /// 폭 조정 값
    pub width_adjust: isize,
    /// ```xml
    /// textOffsetType="{$TextOffsetKind; default="PERCENT"}"
    /// ```
    ///
    /// 텍스트 오프셋 유형
    pub text_offset_type: TextOffsetKind,
    /// ```xml
    /// textOffset="{xs:integer; default="50"}"
    /// ```
    ///
    /// 텍스트 오프셋 값
    pub text_offset: isize,
    /// ```xml
    /// numFormat="{hc:NumberType1; default="DIGIT"}"
    /// ```
    ///
    /// 번호 형식
    pub number_format: NumberType1,
    /// ```xml
    /// charPrIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 글자 모양 아이디 참조
    pub char_pr_id_ref: Option<u32>,
    /// ```xml
    /// checkable="{xs:boolean}"
    /// ```
    ///
    /// 체크 가능 여부
    pub checkable: Option<bool>,
    /// ```xml
    /// ^1.
    /// ```
    pub text: Option<String>,
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
    pub id: u32,
    /// ```xml
    /// char="{xs:string}"
    /// ```
    ///
    /// 글머리표 문자
    pub character: String,
    /// ```xml
    /// checkedChar="{xs:string}"
    /// ```
    ///
    /// 체크된 글머리표 문자
    pub checked_character: Option<String>,
    /// ```xml
    /// useImage="{xs:boolean}"
    /// ```
    ///
    /// 이미지 사용 여부
    pub use_image: bool,
    /// ```xml
    /// <img ...>...</img>
    /// ```
    pub image: Option<ImageType>,
    /// ```xml
    /// <paraHead ...>...</paraHead>
    /// ```
    pub head: ParagraphHeadType,
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
    pub id: u32,
    /// ```xml
    /// type="{$StyleKind}"
    /// ```
    ///
    /// 스타일 종류
    pub r#type: StyleKind,
    /// ```xml
    /// name="{xs:string}"
    /// ```
    ///
    /// 로컬 스타일 이름.
    ///
    /// 한글 윈도에서는 한글 스타일 이름.
    pub name: String,
    /// ```xml
    /// engName="{xs:string}"
    /// ```
    ///
    /// 영문 스타일 이름
    pub eng_name: Option<String>,
    /// ```xml
    /// paraPrIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 문단 모양 아이디 참조.
    ///
    /// 스타일의 종류가 문단이 경우  지정해야 함.
    pub para_pr_id_ref: Option<u32>,
    /// ```xml
    /// charPrIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 글자 모양 아이디 참조
    ///
    /// 스타일의 종류가 글자인 경우  지정해야 함.
    pub char_pr_id_ref: Option<u32>,
    /// ```xml
    /// nextStyleIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 다음 스타일 아이디 참조
    ///
    /// 문단 스타일에서 사용자가 리턴키를 입력하여 다음 문단으로 이동하였을 때 적용될 문단 스타일을 지정한다.
    pub next_style_id_ref: Option<u32>,
    /// ```xml
    /// langID="{xs:unsignedShort}"
    /// ```
    ///
    /// 언어 아이디
    pub lang_id: Option<u16>,
    /// ```xml
    /// lockForm="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 양식 모드에서 Style 보호하기 여부.
    pub lock_form: bool,
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
    pub id: u32,
    /// ```xml
    /// width="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 메모 아이콘 너비. 단위는 hwpunit.
    pub width: u32,
    /// ```xml
    /// lineWidth="{hc:LineWidth}"
    /// ```
    ///
    /// 메모 아이콘 테두리 선 굵기
    pub line_width: LineWidth,
    /// ```xml
    /// lineType="{hc:LineType2}"
    /// ```
    ///
    /// 메모 아이콘 테두리 선 종류
    pub line_type: LineType2,
    /// ```xml
    /// lineColor="{hc:RGBColorType}"
    /// ```
    ///
    /// 메모 아이콘 테두리 색
    pub line_color: RgbColor,
    /// ```xml
    /// fillColor="{hc:RGBColorType}"
    /// ```
    ///
    /// 메모 아이콘 채우기 색
    pub fill_color: RgbColor,
    /// ```xml
    /// activeColor="{hc:RGBColorType}"
    /// ```
    ///
    /// 메모 아이콘 활성화 색
    pub active_color: RgbColor,
    /// ```xml
    /// memoType="{$MemoKind}"
    /// ```
    ///
    /// 메모 아이콘 종류
    pub memo_type: MemoKind,
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
    pub r#type: TrackChangeKind,
    /// ```xml
    /// date="{xs:dateTime}"
    /// ```
    ///
    /// 변경 일시
    pub date: String,
    /// ```xml
    /// authorID="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 변경 추적 작성자 아이디
    pub author_id: u32,
    /// ```xml
    /// charShapeID="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 변경된 글자 모양 아이디
    pub char_shape_id: Option<u32>,
    /// ```xml
    /// paraShapeID="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 변경된 문단 모양 아이디
    pub para_shape_id: Option<u32>,
    /// ```xml
    /// hide="{xs:boolean}"
    /// ```
    ///
    /// 변경 추적 숨김 여부
    pub hide: bool,
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 변경 추적 아이디
    pub id: u32,
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
    pub name: Option<String>,
    /// ```xml
    /// mark="{xs:boolean}"
    /// ```
    pub mark: Option<bool>,
    /// ```xml
    /// color="{hc:RGBColorType}"
    /// ```
    pub color: Option<RgbColor>,
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    pub id: u32,
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
    pub target_program: TargetProgram,
    /// ```xml
    /// <layoutCompatibility>...</layoutCompatibility>
    /// ```
    pub layout_compatibility: LayoutCompatibility,
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
    pub set: FxHashSet<LayoutCompatibilityKind>,
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

/// ```xml
/// <licenseMark
///   type="{xs:unsignedInt}"
///   flag="{xs:byte}"
///   lang="{xs:byte}"
/// />
/// ```
#[derive(Debug)]
pub struct LicenseMark {
    pub r#type: u32,
    pub flag: u8,
    pub lang: u8,
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
    pub flag: u32,
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
    pub hash: String,
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
    pub algorithm: String,
    /// ```xml
    /// size="{xs:nonNegativeInteger}"
    /// ```
    pub size: u32,
    /// ```xml
    /// count="{xs:nonNegativeInteger}"
    /// ```
    pub count: u32,
    /// ```xml
    /// salt="{xs:base64Binary}"
    /// ```
    pub salt: String,
}

/// ```xml
/// <metaTag>...</metaTag>
/// ```
#[derive(Debug)]
pub struct MetaTag {
    /// ```xml
    /// ... (xs:string)
    /// ```
    pub text: String,
}

impl TryFrom<AnyElement> for HWPMLHeadType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__HEAD)?;

        let mut begin_number = None;
        let mut reference = None;
        let mut forbidden_words = None;
        let mut compatible_document = None;
        let mut track_change_config = None;
        let mut document_option = None;
        let mut meta_tag = None;

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__BEGIN_NUMBER => begin_number = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__REFERENCES => reference = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__FORBIDDEN_WORDS => {
                    forbidden_words = Some(child.try_into()?)
                }
                ElementName::HANCOM__HEAD__COMPATIBLE_DOCUMENT => {
                    compatible_document = Some(child.try_into()?)
                }
                ElementName::HANCOM__HEAD__TRACK_CHANGE_CONFIG => {
                    track_change_config = Some(child.try_into()?)
                }
                ElementName::HANCOM__HEAD__DOCUMENT_OPTION => {
                    document_option = Some(child.try_into()?)
                }
                ElementName::HANCOM__HEAD__META_TAG => meta_tag = Some(child.try_into()?),
                _ => {}
            }
        }

        let (begin_number, reference) = match (begin_number, reference) {
            (Some(begin_number), Some(reference)) => (begin_number, reference),
            (None, _) => missing_element!("<beginNum>"),
            (_, None) => missing_element!("<refList>"),
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

impl TryFrom<AnyElement> for MappingTableType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__REFERENCES)?;

        let mut font_faces = vec![];
        let mut border_fills = vec![];
        let mut character_properties = vec![];
        let mut tab_properties = vec![];
        let mut numberings = vec![];
        let mut bullets = vec![];
        let mut paragraph_properties = vec![];
        let mut styles = vec![];
        let mut memo = vec![];
        let mut track_changes = vec![];
        let mut track_change_authors = vec![];

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__FONT_FACES => {
                    for font_face in child.children {
                        font_faces.push(font_face.try_into()?);
                    }
                }
                ElementName::HANCOM__HEAD__BORDER_FILLS => {
                    for border_fill in child.children {
                        border_fills.push(border_fill.try_into()?);
                    }
                }
                ElementName::HANCOM__HEAD__CHARACTER_PROPERTIES => {
                    for character_property in child.children {
                        character_properties.push(character_property.try_into()?);
                    }
                }
                ElementName::HANCOM__HEAD__TAB_PROPERTIES => {
                    for tab_property in child.children {
                        tab_properties.push(tab_property.try_into()?);
                    }
                }
                ElementName::HANCOM__HEAD__NUMBERING => {
                    for numbering in child.children {
                        numberings.push(numbering.try_into()?);
                    }
                }
                ElementName::HANCOM__HEAD__BULLETS => {
                    for bullet in child.children {
                        bullets.push(bullet.try_into()?);
                    }
                }
                ElementName::HANCOM__HEAD__PARAGRAPH_PROPERTIES => {
                    for paragraph_property in child.children {
                        paragraph_properties.push(paragraph_property.try_into()?);
                    }
                }
                ElementName::HANCOM__HEAD__STYLES => {
                    for style in child.children {
                        styles.push(style.try_into()?);
                    }
                }
                ElementName::HANCOM__HEAD__MEMO_PROPERTIES => {
                    for memo_shape in child.children {
                        memo.push(memo_shape.try_into()?);
                    }
                }
                ElementName::HANCOM__HEAD__TRACK_CHANGES => {
                    for track_change in child.children {
                        track_changes.push(track_change.try_into()?);
                    }
                }
                ElementName::HANCOM__HEAD__TRACK_CHANGE_AUTHORS => {
                    for track_change_author in child.children {
                        track_change_authors.push(track_change_author.try_into()?);
                    }
                }
                _ => continue,
            }
        }

        let (character_properties, paragraph_properties) = match (
            NonEmpty::from_vec(character_properties),
            NonEmpty::from_vec(paragraph_properties),
        ) {
            (Some(character_properties), Some(paragraph_properties)) => {
                (character_properties, paragraph_properties)
            }
            (None, _) => missing_element!("<charProperties>"),
            (_, None) => missing_element!("<paraProperties>"),
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

impl TryFrom<AnyElement> for BeginNumber {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__BEGIN_NUMBER)?;

        let mut page = None;
        let mut foot_note = None;
        let mut end_note = None;
        let mut picture = None;
        let mut table = None;
        let mut equation = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "page" => page = Some(value.parse()?),
                "footnote" => foot_note = Some(value.parse()?),
                "endnote" => end_note = Some(value.parse()?),
                "pic" => picture = Some(value.parse()?),
                "tbl" => table = Some(value.parse()?),
                "equation" => equation = Some(value.parse()?),
                _ => continue,
            }
        }

        let (page, foot_note, end_note, picture, table, equation) =
            match (page, foot_note, end_note, picture, table, equation) {
                (
                    Some(page),
                    Some(foot_note),
                    Some(end_note),
                    Some(picture),
                    Some(table),
                    Some(equation),
                ) => (page, foot_note, end_note, picture, table, equation),
                (None, _, _, _, _, _) => missing_attribute!("<beginNum page>"),
                (_, None, _, _, _, _) => missing_attribute!("<beginNum footnote>"),
                (_, _, None, _, _, _) => missing_attribute!("<beginNum endnote>"),
                (_, _, _, None, _, _) => missing_attribute!("<beginNum pic>"),
                (_, _, _, _, None, _) => missing_attribute!("<beginNum tbl>"),
                (_, _, _, _, _, None) => missing_attribute!("<beginNum equation>"),
            };

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

impl TryFrom<AnyElement> for FontFaceType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__FONT_FACE)?;

        let mut language = None;
        let mut fonts = vec![];

        for (key, value) in element.attributes {
            match key.as_str() {
                "lang" => language = Some(value.parse()?),
                _ => continue,
            }
        }

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__FONT => fonts.push(child.try_into()?),
                _ => continue,
            }
        }

        let (language, fonts) = match (language, NonEmpty::from_vec(fonts)) {
            (Some(language), Some(fonts)) => (language, fonts),
            (None, _) => missing_attribute!("<fontFace lang>"),
            (_, None) => missing_element!("<fontFace font>"),
        };

        Ok(Self { language, fonts })
    }
}

impl TryFrom<AnyElement> for Font {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__FONT)?;

        let mut id = None;
        let mut face = None;
        let mut r#type = None;
        let mut embedded = false;
        let mut binary_item_id_ref = None;
        let mut subset = None;
        let mut type_info = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "id" => id = Some(value.parse()?),
                "face" => face = Some(value),
                "type" => r#type = Some(value.parse()?),
                "embedded" => embedded = boolean!(value.as_str(), "<font embedded>"),
                "binaryItemIDRef" => binary_item_id_ref = Some(value),
                _ => continue,
            }
        }

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__SUBSET_FONT => subset = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__TYPE_INFO => type_info = Some(child.try_into()?),
                _ => continue,
            }
        }

        let (id, face, r#type) = match (id, face, r#type) {
            (Some(id), Some(face), Some(r#type)) => (id, face, r#type),
            (None, _, _) => missing_attribute!("<font id>"),
            (_, None, _) => missing_attribute!("<font face>"),
            (_, _, None) => missing_attribute!("<font type>"),
        };

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

impl TryFrom<AnyElement> for SubsetFont {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__SUBSET_FONT)?;

        let mut face = None;
        let mut r#type = None;
        let mut embedded = false;
        let mut binary_item_id_ref = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "face" => face = Some(value),
                "type" => r#type = Some(value.parse()?),
                "embedded" => embedded = boolean!(value.as_str(), "<subsetFont embedded>"),
                "binaryItemIDRef" => binary_item_id_ref = Some(value),
                _ => continue,
            }
        }

        let (face, r#type) = match (face, r#type) {
            (Some(face), Some(r#type)) => (face, r#type),
            (None, _) => missing_attribute!("<subsetFont face>"),
            (_, None) => missing_attribute!("<subsetFont type>"),
        };

        Ok(Self {
            face,
            r#type,
            embedded,
            binary_item_id_ref,
        })
    }
}

impl TryFrom<AnyElement> for TypeInfo {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TYPE_INFO)?;

        let mut family_type = None;
        let mut serif_style = None;
        let mut weight = None;
        let mut proportion = None;
        let mut contrast = None;
        let mut stroke_variation = None;
        let mut arm_style = None;
        let mut letter_form = None;
        let mut midline = None;
        let mut x_height = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "familyType" => family_type = Some(value.parse()?),
                "serifStyle" => serif_style = Some(value),
                "weight" => weight = Some(value.parse()?),
                "proportion" => proportion = Some(value.parse()?),
                "contrast" => contrast = Some(value.parse()?),
                "strokeVariation" => stroke_variation = Some(value.parse()?),
                "armStyle" => arm_style = boolean!(value.as_str(), "<typeInfo armStyle>"),
                "letterform" => letter_form = boolean!(value.as_str(), "<typeInfo letterform>"),
                "midline" => midline = boolean!(value.as_str(), "<typeInfo midline>"),
                "xHeight" => x_height = Some(value.parse()?),
                _ => continue,
            }
        }

        let (
            family_type,
            weight,
            proportion,
            contrast,
            stroke_variation,
            arm_style,
            letter_form,
            midline,
            x_height,
        ) = match (
            family_type,
            weight,
            proportion,
            contrast,
            stroke_variation,
            arm_style,
            letter_form,
            midline,
            x_height,
        ) {
            (
                Some(family_type),
                Some(weight),
                Some(proportion),
                Some(contrast),
                Some(stroke_variation),
                Some(arm_style),
                Some(letter_form),
                Some(midline),
                Some(x_height),
            ) => (
                family_type,
                weight,
                proportion,
                contrast,
                stroke_variation,
                arm_style,
                letter_form,
                midline,
                x_height,
            ),
            (None, _, _, _, _, _, _, _, _) => missing_attribute!("<typeInfo familyType>"),
            (_, None, _, _, _, _, _, _, _) => missing_attribute!("<typeInfo weight>"),
            (_, _, None, _, _, _, _, _, _) => missing_attribute!("<typeInfo proportion>"),
            (_, _, _, None, _, _, _, _, _) => missing_attribute!("<typeInfo contrast>"),
            (_, _, _, _, None, _, _, _, _) => missing_attribute!("<typeInfo strokeVariation>"),
            (_, _, _, _, _, None, _, _, _) => missing_attribute!("<typeInfo armStyle>"),
            (_, _, _, _, _, _, None, _, _) => missing_attribute!("<typeInfo letterForm>"),
            (_, _, _, _, _, _, _, None, _) => missing_attribute!("<typeInfo midline>"),
            (_, _, _, _, _, _, _, _, None) => missing_attribute!("<typeInfo xHeight>"),
        };

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

impl TryFrom<AnyElement> for BorderFillType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__BORDER_FILL)?;

        let mut id = None;
        let mut effect_3d = false;
        let mut shadow = false;
        let mut center_line = None;
        let mut break_cell_separate_line = false;
        let mut slash = None;
        let mut back_slash = None;
        let mut left_border = None;
        let mut right_border = None;
        let mut top_border = None;
        let mut bottom_border = None;
        let mut diagonal = None;
        let mut fill_brush = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "id" => id = Some(value.parse()?),
                "effect3D" => effect_3d = boolean!(value.as_str(), "<borderFill effect3D>"),
                "shadow" => shadow = boolean!(value.as_str(), "<borderFill shadow>"),
                "centerLine" => center_line = Some(value.parse()?),
                "breakCellSeparateLine" => {
                    break_cell_separate_line =
                        boolean!(value.as_str(), "<borderFill breakCellSeparateLine>")
                }
                _ => continue,
            }
        }

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__SLASH => slash = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__BACK_SLASH => back_slash = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__LEFT_BORDER => left_border = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__RIGHT_BORDER => right_border = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__TOP_BORDER => top_border = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__BOTTOM_BORDER => bottom_border = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__DIAGONAL => diagonal = Some(child.try_into()?),
                ElementName::HANCOM__CORE__FILL_BRUSH => fill_brush = Some(child.try_into()?),
                _ => continue,
            }
        }

        let (id, center_line) = match (id, center_line) {
            (Some(id), Some(center_line)) => (id, center_line),
            (None, _) => missing_attribute!("<borderFill id>"),
            (_, None) => missing_attribute!("<borderFill centerLine>"),
        };

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

impl TryFrom<AnyElement> for Slash {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        let mut r#type = None;
        let mut crooked = None;
        let mut counter = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "type" => r#type = Some(value.parse()?),
                "Crooked" => crooked = boolean!(value.as_str(), "<slash crooked>"),
                "isCounter" => counter = boolean!(value.as_str(), "<slash counter>"),
                _ => continue,
            }
        }

        let (r#type, crooked, counter) = match (r#type, crooked, counter) {
            (Some(r#type), Some(crooked), Some(counter)) => (r#type, crooked, counter),
            (None, _, _) => missing_attribute!("<slash type>"),
            (_, None, _) => missing_attribute!("<slash crooked>"),
            (_, _, None) => missing_attribute!("<slash counter>"),
        };

        Ok(Self {
            r#type,
            crooked,
            counter,
        })
    }
}

impl TryFrom<AnyElement> for BorderType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        let mut r#type = None;
        let mut width = None;
        let mut color = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "type" => r#type = Some(value.parse()?),
                "width" => width = Some(value.parse()?),
                "color" => color = Some(value.parse()?),
                _ => continue,
            }
        }

        let (r#type, width, color) = match (r#type, width, color) {
            (Some(r#type), Some(width), Some(color)) => (r#type, width, color),
            (None, _, _) => missing_attribute!("<border type>"),
            (_, None, _) => missing_attribute!("<border width>"),
            (_, _, None) => missing_attribute!("<border color>"),
        };

        Ok(Self {
            r#type,
            width,
            color,
        })
    }
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

impl TryFrom<AnyElement> for WinBrush {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__CORE__WIN_BRUSH)?;

        let mut face_color = None;
        let mut hatch_color = None;
        let mut hatch_style = None;
        let mut alpha = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "faceColor" => face_color = Some(value.parse()?),
                "hatchColor" => hatch_color = Some(value.parse()?),
                "hatchStyle" => hatch_style = Some(value.parse()?),
                "alpha" => alpha = Some(value.parse()?),
                _ => continue,
            }
        }

        let (face_color, hatch_color, alpha) = match (face_color, hatch_color, alpha) {
            (Some(face_color), Some(hatch_color), Some(alpha)) => (face_color, hatch_color, alpha),
            (None, _, _) => missing_attribute!("<winBrush faceColor>"),
            (_, None, _) => missing_attribute!("<winBrush hatchColor>"),
            (_, _, None) => missing_attribute!("<winBrush alpha>"),
        };

        Ok(Self {
            face_color,
            hatch_color,
            hatch_style,
            alpha,
        })
    }
}

impl TryFrom<AnyElement> for Gradation {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__CORE__GRADATION)?;

        let mut r#type = None;
        let mut angle = 90;
        let mut center_x = None;
        let mut center_y = None;
        let mut step = None;
        let mut color_number = None;
        let mut step_center = None;
        let mut alpha = None;
        let mut colors = vec![];

        for (key, value) in element.attributes {
            match key.as_str() {
                "type" => r#type = Some(value.parse()?),
                "angle" => angle = value.parse()?,
                "centerX" => center_x = Some(value.parse()?),
                "centerY" => center_y = Some(value.parse()?),
                "step" => step = Some(value.parse()?),
                "colorNum" => color_number = Some(value.parse()?),
                "stepCenter" => step_center = Some(value.parse()?),
                "alpha" => alpha = Some(value.parse()?),
                _ => continue,
            }
        }

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__COLOR => colors.push(child.try_into()?),
                _ => continue,
            }
        }

        let (r#type, center_x, center_y, step, color_number, step_center, alpha) = match (
            r#type,
            center_x,
            center_y,
            step,
            color_number,
            step_center,
            alpha,
        ) {
            (
                Some(r#type),
                Some(center_x),
                Some(center_y),
                Some(step),
                Some(color_number),
                Some(step_center),
                Some(alpha),
            ) => (
                r#type,
                center_x,
                center_y,
                step,
                color_number,
                step_center,
                alpha,
            ),
            (None, _, _, _, _, _, _) => missing_attribute!("<gradation type>"),
            (_, None, _, _, _, _, _) => missing_attribute!("<gradation centerX>"),
            (_, _, None, _, _, _, _) => missing_attribute!("<gradation centerY>"),
            (_, _, _, None, _, _, _) => missing_attribute!("<gradation step>"),
            (_, _, _, _, None, _, _) => missing_attribute!("<gradation colorNum>"),
            (_, _, _, _, _, None, _) => missing_attribute!("<gradation stepCenter>"),
            (_, _, _, _, _, _, None) => missing_attribute!("<gradation alpha>"),
        };

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

impl TryFrom<AnyElement> for GradationColor {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__COLOR)?;

        let mut value = None;

        for (key, value_str) in element.attributes {
            match key.as_str() {
                "value" => value = Some(value_str.parse()?),
                _ => continue,
            }
        }

        let (value,) = match (value,) {
            (Some(value),) => (value,),
            (None,) => missing_attribute!("<color value>"),
        };

        Ok(Self { value })
    }
}

impl TryFrom<AnyElement> for ImageBrush {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__CORE__IMAGE_BRUSH)?;

        let mut mode = None;
        let mut image = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "mode" => mode = Some(value.parse()?),
                _ => continue,
            }
        }

        for child in element.children {
            match child.name {
                ElementName::HANCOM__CORE__IMAGE => image = Some(child.try_into()?),
                _ => continue,
            }
        }

        let (mode, image) = match (mode, image) {
            (Some(mode), Some(image)) => (mode, image),
            (None, _) => missing_attribute!("<imageBrush mode>"),
            (_, None) => missing_element!("<imageBrush image>"),
        };

        Ok(Self { mode, image })
    }
}

impl TryFrom<AnyElement> for ImageType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__CORE__IMAGE)?;

        let mut binary_id_ref = None;
        let mut bright = 0;
        let mut contrast = 0;
        let mut effect = ImageEffect::RealPicture;
        let mut alpha = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "binaryItemIDRef" => binary_id_ref = Some(value),
                "bright" => bright = value.parse()?,
                "contrast" => contrast = value.parse()?,
                "effect" => effect = value.parse()?,
                "alpha" => alpha = Some(value.parse()?),
                _ => continue,
            }
        }

        let (binary_id_ref, alpha) = match (binary_id_ref, alpha) {
            (Some(binary_id_ref), Some(alpha)) => (binary_id_ref, alpha),
            (None, _) => missing_attribute!("<img binaryItemIDRef>"),
            (_, None) => missing_attribute!("<img alpha>"),
        };

        Ok(Self {
            binary_id_ref,
            bright,
            contrast,
            effect,
            alpha,
        })
    }
}

impl TryFrom<AnyElement> for CharShapeType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__CHARACTER_PROPERTY)?;

        let mut id = None;
        let mut height = 1000;
        let mut text_color = RgbColor(Some((0, 0, 0)));
        let mut shade_color = RgbColor(Some((255, 255, 255)));
        let mut use_font_space = false;
        let mut use_kerning = false;
        let mut symbol_mark = SymbolMark::None;
        let mut border_fill_id_ref = None;
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

        for (key, value) in element.attributes {
            match key.as_str() {
                "id" => id = Some(value.parse()?),
                "height" => height = value.parse()?,
                "textColor" => text_color = value.parse()?,
                "shadeColor" => shade_color = value.parse()?,
                "useFontSpace" => {
                    use_font_space = boolean!(value.as_str(), "<charPr useFontSpace>")
                }
                "useKerning" => use_kerning = boolean!(value.as_str(), "<charPr useKerning>"),
                "symMark" => symbol_mark = value.parse()?,
                "borderFillIDRef" => border_fill_id_ref = Some(value.parse()?),
                _ => continue,
            }
        }

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

        let (id, font_ref, ratio, spacing, relative_size, offset) =
            match (id, font_ref, ratio, spacing, relative_size, offset) {
                (
                    Some(id),
                    Some(font_ref),
                    Some(ratio),
                    Some(spacing),
                    Some(relative_size),
                    Some(offset),
                ) => (id, font_ref, ratio, spacing, relative_size, offset),
                (None, _, _, _, _, _) => missing_attribute!("<charPr id>"),
                (_, None, _, _, _, _) => missing_element!("<fontRef>"),
                (_, _, None, _, _, _) => missing_element!("<ratio>"),
                (_, _, _, None, _, _) => missing_element!("<spacing>"),
                (_, _, _, _, None, _) => missing_element!("<relSz>"),
                (_, _, _, _, _, None) => missing_element!("<offset>"),
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

impl TryFrom<AnyElement> for FontReference {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__FONT_REFERENCE)?;

        let mut hangul = None;
        let mut latin = None;
        let mut hanja = None;
        let mut japanese = None;
        let mut other = None;
        let mut symbol = None;
        let mut user = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "hangul" => hangul = Some(value.parse()?),
                "latin" => latin = Some(value.parse()?),
                "hanja" => hanja = Some(value.parse()?),
                "japanese" => japanese = Some(value.parse()?),
                "other" => other = Some(value.parse()?),
                "symbol" => symbol = Some(value.parse()?),
                "user" => user = Some(value.parse()?),
                _ => continue,
            }
        }

        let (hangul, latin, hanja, japanese, other, symbol, user) =
            match (hangul, latin, hanja, japanese, other, symbol, user) {
                (
                    Some(hangul),
                    Some(latin),
                    Some(hanja),
                    Some(japanese),
                    Some(other),
                    Some(symbol),
                    Some(user),
                ) => (hangul, latin, hanja, japanese, other, symbol, user),
                (None, _, _, _, _, _, _) => missing_attribute!("<fontRef hangul>"),
                (_, None, _, _, _, _, _) => missing_attribute!("<fontRef latin>"),
                (_, _, None, _, _, _, _) => missing_attribute!("<fontRef hanja>"),
                (_, _, _, None, _, _, _) => missing_attribute!("<fontRef japanese>"),
                (_, _, _, _, None, _, _) => missing_attribute!("<fontRef other>"),
                (_, _, _, _, _, None, _) => missing_attribute!("<fontRef symbol>"),
                (_, _, _, _, _, _, None) => missing_attribute!("<fontRef user>"),
            };

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

impl TryFrom<AnyElement> for Ratio {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__RATIO)?;

        let mut hangul = None;
        let mut latin = None;
        let mut hanja = None;
        let mut japanese = None;
        let mut other = None;
        let mut symbol = None;
        let mut user = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "hangul" => hangul = Some(value.parse()?),
                "latin" => latin = Some(value.parse()?),
                "hanja" => hanja = Some(value.parse()?),
                "japanese" => japanese = Some(value.parse()?),
                "other" => other = Some(value.parse()?),
                "symbol" => symbol = Some(value.parse()?),
                "user" => user = Some(value.parse()?),
                _ => continue,
            }
        }

        let (hangul, latin, hanja, japanese, other, symbol, user) =
            match (hangul, latin, hanja, japanese, other, symbol, user) {
                (
                    Some(hangul),
                    Some(latin),
                    Some(hanja),
                    Some(japanese),
                    Some(other),
                    Some(symbol),
                    Some(user),
                ) => (hangul, latin, hanja, japanese, other, symbol, user),
                (None, _, _, _, _, _, _) => missing_attribute!("<ratio hangul>"),
                (_, None, _, _, _, _, _) => missing_attribute!("<ratio latin>"),
                (_, _, None, _, _, _, _) => missing_attribute!("<ratio hanja>"),
                (_, _, _, None, _, _, _) => missing_attribute!("<ratio japanese>"),
                (_, _, _, _, None, _, _) => missing_attribute!("<ratio other>"),
                (_, _, _, _, _, None, _) => missing_attribute!("<ratio symbol>"),
                (_, _, _, _, _, _, None) => missing_attribute!("<ratio user>"),
            };

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

impl TryFrom<AnyElement> for Spacing {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__SPACING)?;

        let mut hangul = None;
        let mut latin = None;
        let mut hanja = None;
        let mut japanese = None;
        let mut other = None;
        let mut symbol = None;
        let mut user = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "hangul" => hangul = Some(value.parse()?),
                "latin" => latin = Some(value.parse()?),
                "hanja" => hanja = Some(value.parse()?),
                "japanese" => japanese = Some(value.parse()?),
                "other" => other = Some(value.parse()?),
                "symbol" => symbol = Some(value.parse()?),
                "user" => user = Some(value.parse()?),
                _ => continue,
            }
        }

        let (hangul, latin, hanja, japanese, other, symbol, user) =
            match (hangul, latin, hanja, japanese, other, symbol, user) {
                (
                    Some(hangul),
                    Some(latin),
                    Some(hanja),
                    Some(japanese),
                    Some(other),
                    Some(symbol),
                    Some(user),
                ) => (hangul, latin, hanja, japanese, other, symbol, user),
                (None, _, _, _, _, _, _) => missing_attribute!("<spacing hangul>"),
                (_, None, _, _, _, _, _) => missing_attribute!("<spacing latin>"),
                (_, _, None, _, _, _, _) => missing_attribute!("<spacing hanja>"),
                (_, _, _, None, _, _, _) => missing_attribute!("<spacing japanese>"),
                (_, _, _, _, None, _, _) => missing_attribute!("<spacing other>"),
                (_, _, _, _, _, None, _) => missing_attribute!("<spacing symbol>"),
                (_, _, _, _, _, _, None) => missing_attribute!("<spacing user>"),
            };

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

impl TryFrom<AnyElement> for RelativeSize {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__RELATIVE_SIZE)?;

        let mut hangul = None;
        let mut latin = None;
        let mut hanja = None;
        let mut japanese = None;
        let mut other = None;
        let mut symbol = None;
        let mut user = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "hangul" => hangul = Some(value.parse()?),
                "latin" => latin = Some(value.parse()?),
                "hanja" => hanja = Some(value.parse()?),
                "japanese" => japanese = Some(value.parse()?),
                "other" => other = Some(value.parse()?),
                "symbol" => symbol = Some(value.parse()?),
                "user" => user = Some(value.parse()?),
                _ => continue,
            }
        }

        let (hangul, latin, hanja, japanese, other, symbol, user) =
            match (hangul, latin, hanja, japanese, other, symbol, user) {
                (
                    Some(hangul),
                    Some(latin),
                    Some(hanja),
                    Some(japanese),
                    Some(other),
                    Some(symbol),
                    Some(user),
                ) => (hangul, latin, hanja, japanese, other, symbol, user),
                (None, _, _, _, _, _, _) => missing_attribute!("<relSz hangul>"),
                (_, None, _, _, _, _, _) => missing_attribute!("<relSz latin>"),
                (_, _, None, _, _, _, _) => missing_attribute!("<relSz hanja>"),
                (_, _, _, None, _, _, _) => missing_attribute!("<relSz japanese>"),
                (_, _, _, _, None, _, _) => missing_attribute!("<relSz other>"),
                (_, _, _, _, _, None, _) => missing_attribute!("<relSz symbol>"),
                (_, _, _, _, _, _, None) => missing_attribute!("<relSz user>"),
            };

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

impl TryFrom<AnyElement> for Offset {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__OFFSET)?;

        let mut hangul = None;
        let mut latin = None;
        let mut hanja = None;
        let mut japanese = None;
        let mut other = None;
        let mut symbol = None;
        let mut user = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "hangul" => hangul = Some(value.parse()?),
                "latin" => latin = Some(value.parse()?),
                "hanja" => hanja = Some(value.parse()?),
                "japanese" => japanese = Some(value.parse()?),
                "other" => other = Some(value.parse()?),
                "symbol" => symbol = Some(value.parse()?),
                "user" => user = Some(value.parse()?),
                _ => continue,
            }
        }

        let (hangul, latin, hanja, japanese, other, symbol, user) =
            match (hangul, latin, hanja, japanese, other, symbol, user) {
                (
                    Some(hangul),
                    Some(latin),
                    Some(hanja),
                    Some(japanese),
                    Some(other),
                    Some(symbol),
                    Some(user),
                ) => (hangul, latin, hanja, japanese, other, symbol, user),
                (None, _, _, _, _, _, _) => missing_attribute!("<offset hangul>"),
                (_, None, _, _, _, _, _) => missing_attribute!("<offset latin>"),
                (_, _, None, _, _, _, _) => missing_attribute!("<offset hanja>"),
                (_, _, _, None, _, _, _) => missing_attribute!("<offset japanese>"),
                (_, _, _, _, None, _, _) => missing_attribute!("<offset other>"),
                (_, _, _, _, _, None, _) => missing_attribute!("<offset symbol>"),
                (_, _, _, _, _, _, None) => missing_attribute!("<offset user>"),
            };

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

impl TryFrom<AnyElement> for Underline {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__UNDERLINE)?;

        let mut r#type = None;
        let mut shape = None;
        let mut color = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "type" => r#type = Some(value.parse()?),
                "shape" => shape = Some(value.parse()?),
                "color" => color = Some(value.parse()?),
                _ => continue,
            }
        }

        let (r#type, shape, color) = match (r#type, shape, color) {
            (Some(r#type), Some(shape), Some(color)) => (r#type, shape, color),
            (None, _, _) => missing_attribute!("<underline type>"),
            (_, None, _) => missing_attribute!("<underline shape>"),
            (_, _, None) => missing_attribute!("<underline color>"),
        };

        Ok(Self {
            r#type,
            shape,
            color,
        })
    }
}

impl TryFrom<AnyElement> for Strikeout {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__STRIKEOUT)?;

        let mut shape = None;
        let mut color = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "shape" => shape = Some(value.parse()?),
                "color" => color = Some(value.parse()?),
                _ => continue,
            }
        }

        let (shape, color) = match (shape, color) {
            (Some(shape), Some(color)) => (shape, color),
            (None, _) => missing_attribute!("<strikeout shape>"),
            (_, None) => missing_attribute!("<strikeout color>"),
        };

        Ok(Self { shape, color })
    }
}

impl TryFrom<AnyElement> for Outline {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__OUTLINE)?;

        let mut r#type = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "type" => r#type = Some(value.parse()?),
                _ => continue,
            }
        }

        let (r#type,) = match (r#type,) {
            (Some(r#type),) => (r#type,),
            (None,) => missing_attribute!("<outline type>"),
        };

        Ok(Self { r#type })
    }
}

impl TryFrom<AnyElement> for Shadow {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__SHADOW)?;

        let mut r#type = None;
        let mut color = None;
        let mut offset_x = None;
        let mut offset_y = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "type" => r#type = Some(value.parse()?),
                "color" => color = Some(value.parse()?),
                "offsetX" => offset_x = Some(value.parse()?),
                "offsetY" => offset_y = Some(value.parse()?),
                _ => continue,
            }
        }

        let (r#type, color, offset_x, offset_y) = match (r#type, color, offset_x, offset_y) {
            (Some(r#type), Some(color), Some(offset_x), Some(offset_y)) => {
                (r#type, color, offset_x, offset_y)
            }
            (None, _, _, _) => missing_attribute!("<shadow type>"),
            (_, None, _, _) => missing_attribute!("<shadow color>"),
            (_, _, None, _) => missing_attribute!("<shadow offsetX>"),
            (_, _, _, None) => missing_attribute!("<shadow offsetY>"),
        };

        Ok(Self {
            r#type,
            color,
            offset_x,
            offset_y,
        })
    }
}

impl TryFrom<AnyElement> for TabDefType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TAB_DEFINITION)?;

        let mut id = None;
        let mut auto_tab_left = false;
        let mut auto_tab_right = false;
        let mut tab = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "id" => id = Some(value.parse()?),
                "autoTabLeft" => auto_tab_left = boolean!(value.as_str(), "<tabDef autoTabLeft>"),
                "autoTabRight" => {
                    auto_tab_right = boolean!(value.as_str(), "<tabDef autoTabRight>")
                }
                _ => continue,
            }
        }

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__TAB_ITEM => tab = Some(child.try_into()?),
                _ => continue,
            }
        }

        let id = match id {
            Some(id) => id,
            None => missing_attribute!("<tabDef id>"),
        };

        Ok(Self {
            id,
            auto_tab_left,
            auto_tab_right,
            tab,
        })
    }
}

impl TryFrom<AnyElement> for TabItem {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TAB_ITEM)?;

        let mut position = None;
        let mut r#type = None;
        let mut leader = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "pos" => position = Some(value.parse()?),
                "type" => r#type = Some(value.parse()?),
                "leader" => leader = Some(value.parse()?),
                _ => continue,
            }
        }

        let (position, r#type, leader) = match (position, r#type, leader) {
            (Some(position), Some(r#type), Some(leader)) => (position, r#type, leader),
            (None, _, _) => missing_attribute!("<tabItem position>"),
            (_, None, _) => missing_attribute!("<tabItem type>"),
            (_, _, None) => missing_attribute!("<tabItem leader>"),
        };

        Ok(Self {
            position,
            r#type,
            leader,
        })
    }
}

impl TryFrom<AnyElement> for NumberingType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__NUMBERING)?;

        let mut id = None;
        let mut start = 1;
        let mut heads = vec![];

        for (key, value) in element.attributes {
            match key.as_str() {
                "id" => id = Some(value.parse()?),
                "start" => start = value.parse()?,
                _ => continue,
            }
        }

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__PARAGRAPH_HEAD => heads.push(child.try_into()?),
                _ => continue,
            }
        }

        let (id,) = match (id,) {
            (Some(id),) => (id,),
            (None,) => missing_attribute!("<numbering id>"),
        };

        Ok(Self { id, start, heads })
    }
}

impl TryFrom<AnyElement> for ParagraphHeadType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__PARAGRAPH_HEAD)?;

        let mut start = 1;
        let mut level = None;
        let mut align = ParagraphHorizontalAlignKind::Left;
        let mut use_inset_width = true;
        let mut auto_indent = true;
        let mut width_adjust = 0;
        let mut text_offset_type = TextOffsetKind::Percent;
        let mut text_offset = 50;
        let mut number_format = NumberType1::Digit;
        let mut char_pr_id_ref = None;
        let mut checkable = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "start" => start = value.parse()?,
                "level" => level = Some(value.parse()?),
                "align" => align = value.parse()?,
                "useInsetWidth" => {
                    use_inset_width = boolean!(value.as_str(), "<paraHead useInsetWidth>")
                }
                "autoIndent" => auto_indent = boolean!(value.as_str(), "<paraHead autoIndent>"),
                "widthAdjust" => width_adjust = value.parse()?,
                "textOffsetType" => text_offset_type = value.parse()?,
                "textOffset" => text_offset = value.parse()?,
                "numFormat" => number_format = value.parse()?,
                "charPrIDRef" => char_pr_id_ref = Some(value.parse()?),
                "checkable" => checkable = boolean!(value.as_str(), "<paraHead checkable>"),
                _ => continue,
            }
        }

        let level = match level {
            Some(level) => level,
            None => missing_attribute!("<paraHead level>"),
        };

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

impl TryFrom<AnyElement> for BulletType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__BULLET)?;

        let mut id = None;
        let mut character = None;
        let mut checked_character = None;
        let mut use_image = None;
        let mut image = None;
        let mut head = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "id" => id = Some(value.parse()?),
                "char" => character = Some(value),
                "checkedChar" => checked_character = Some(value),
                "useImage" => use_image = Some(boolean!(value.as_str(), "<bullet useImage>")),
                _ => continue,
            }
        }

        for child in element.children {
            match child.name {
                ElementName::HANCOM__CORE__IMAGE => image = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__PARAGRAPH_HEAD => head = Some(child.try_into()?),
                _ => continue,
            }
        }

        let (id, character, use_image, head) = match (id, character, use_image, head) {
            (Some(id), Some(character), Some(use_image), Some(head)) => {
                (id, character, use_image, head)
            }
            (None, _, _, _) => missing_attribute!("<bullet id>"),
            (_, None, _, _) => missing_attribute!("<bullet char>"),
            (_, _, None, _) => missing_attribute!("<bullet useImage>"),
            (_, _, _, None) => missing_element!("<paraHead>"),
        };

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

impl TryFrom<AnyElement> for ParaShapeType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__PARAGRAPH_PROPERTY)?;

        let mut id = None;
        let mut tab_pr_id_ref = None;
        let mut condense = 0;
        let mut font_line_height = false;
        let mut snap_to_grid = true;
        let mut suppress_line_numbers = false;
        let mut checked = false;
        let mut align = None;
        let mut heading = None;
        let mut break_setting = None;
        let mut margin = None;
        let mut line_spacing = None;
        let mut border = None;
        let mut auto_spacing = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "id" => id = Some(value.parse()?),
                "tabPrIDRef" => tab_pr_id_ref = Some(value.parse()?),
                "condense" => condense = value.parse()?,
                "fontLineHeight" => {
                    font_line_height = boolean!(value.as_str(), "<paraShape fontLineHeight>")
                }
                "snapToGrid" => snap_to_grid = boolean!(value.as_str(), "<paraShape snapToGrid>"),
                "suppressLineNumbers" => {
                    suppress_line_numbers =
                        boolean!(value.as_str(), "<paraShape suppressLineNumbers>")
                }
                "checked" => checked = boolean!(value.as_str(), "<paraShape checked>"),
                _ => continue,
            }
        }

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__ALIGN => align = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__HEADING => heading = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__BREAK_SETTING => break_setting = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__MARGIN => margin = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__LINE_SPACING => line_spacing = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__BORDER => border = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__AUTO_SPACING => auto_spacing = Some(child.try_into()?),
                _ => continue,
            }
        }

        let (id, align, heading, break_setting, margin, line_spacing, border, auto_spacing) = match (
            id,
            align,
            heading,
            break_setting,
            margin,
            line_spacing,
            border,
            auto_spacing,
        ) {
            (
                Some(id),
                Some(align),
                Some(heading),
                Some(break_setting),
                Some(margin),
                Some(line_spacing),
                Some(border),
                Some(auto_spacing),
            ) => (
                id,
                align,
                heading,
                break_setting,
                margin,
                line_spacing,
                border,
                auto_spacing,
            ),
            (None, _, _, _, _, _, _, _) => missing_attribute!("<paraPr id>"),
            (_, None, _, _, _, _, _, _) => missing_element!("<align>"),
            (_, _, None, _, _, _, _, _) => missing_element!("<heading>"),
            (_, _, _, None, _, _, _, _) => missing_element!("<breakSetting>"),
            (_, _, _, _, None, _, _, _) => missing_element!("<margin>"),
            (_, _, _, _, _, None, _, _) => missing_element!("<lineSpacing>"),
            (_, _, _, _, _, _, None, _) => missing_element!("<border>"),
            (_, _, _, _, _, _, _, None) => missing_element!("<autoSpacing>"),
        };

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

impl TryFrom<AnyElement> for ParagraphAlignType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__ALIGN)?;

        let mut horizontal = None;
        let mut vertical = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "horizontal" => horizontal = Some(value.parse()?),
                "vertical" => vertical = Some(value.parse()?),
                _ => continue,
            }
        }

        let (horizontal, vertical) = match (horizontal, vertical) {
            (Some(horizontal), Some(vertical)) => (horizontal, vertical),
            (None, _) => missing_attribute!("<align horizontal>"),
            (_, None) => missing_attribute!("<align vertical>"),
        };

        Ok(Self {
            horizontal,
            vertical,
        })
    }
}

impl TryFrom<AnyElement> for ParagraphHeading {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__HEADING)?;

        let mut r#type = None;
        let mut id_ref = None;
        let mut level = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "type" => r#type = Some(value.parse()?),
                "idRef" => id_ref = Some(value.parse()?),
                "level" => level = Some(value.parse()?),
                _ => continue,
            }
        }

        let (r#type, level) = match (r#type, level) {
            (Some(r#type), Some(level)) => (r#type, level),
            (None, _) => missing_attribute!("<heading type>"),
            (_, None) => missing_attribute!("<heading level>"),
        };

        Ok(Self {
            r#type,
            id_ref,
            level,
        })
    }
}

impl TryFrom<AnyElement> for ParagraphBreakSetting {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__BREAK_SETTING)?;

        let mut break_latin_word = None;
        let mut break_non_latin_word = None;
        let mut widow_orphan = None;
        let mut keep_with_next = None;
        let mut keep_lines = None;
        let mut page_break_before = None;
        let mut line_wrap = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "breakLatinWord" => break_latin_word = Some(value.parse()?),
                "breakNonLatinWord" => break_non_latin_word = Some(value.parse()?),
                "widowOrphan" => {
                    widow_orphan = boolean!(value.as_str(), "<breakSetting widowOrphan>")
                }
                "keepWithNext" => {
                    keep_with_next = boolean!(value.as_str(), "<breakSetting keepWithNext>")
                }
                "keepLines" => keep_lines = boolean!(value.as_str(), "<breakSetting keepLines>"),
                "pageBreakBefore" => {
                    page_break_before = boolean!(value.as_str(), "<breakSetting pageBreakBefore>")
                }
                "lineWrap" => line_wrap = Some(value.parse()?),
                _ => continue,
            }
        }

        let (
            break_latin_word,
            break_non_latin_word,
            widow_orphan,
            keep_with_next,
            keep_lines,
            page_break_before,
            line_wrap,
        ) = match (
            break_latin_word,
            break_non_latin_word,
            widow_orphan,
            keep_with_next,
            keep_lines,
            page_break_before,
            line_wrap,
        ) {
            (
                Some(break_latin_word),
                Some(break_non_latin_word),
                Some(widow_orphan),
                Some(keep_with_next),
                Some(keep_lines),
                Some(page_break_before),
                Some(line_wrap),
            ) => (
                break_latin_word,
                break_non_latin_word,
                widow_orphan,
                keep_with_next,
                keep_lines,
                page_break_before,
                line_wrap,
            ),
            (None, _, _, _, _, _, _) => missing_attribute!("<breakSetting breakLatinWord>"),
            (_, None, _, _, _, _, _) => {
                missing_attribute!("<breakSetting breakNonLatinWord>")
            }
            (_, _, None, _, _, _, _) => missing_attribute!("<breakSetting widowOrphan>"),
            (_, _, _, None, _, _, _) => missing_attribute!("<breakSetting keepWithNext>"),
            (_, _, _, _, None, _, _) => missing_attribute!("<breakSetting keepLines>"),
            (_, _, _, _, _, None, _) => {
                missing_attribute!("<breakSetting pageBreakBefore>")
            }
            (_, _, _, _, _, _, None) => missing_attribute!("<breakSetting lineWrap>"),
        };

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

impl TryFrom<AnyElement> for ParagraphMargin {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__MARGIN)?;

        let mut indent = None;
        let mut left = None;
        let mut right = None;
        let mut previous = None;
        let mut next = None;

        for child in element.children {
            match child.name {
                ElementName::HANCOM__CORE__INDENT => indent = Some(child.try_into()?),
                ElementName::HANCOM__CORE__LEFT => left = Some(child.try_into()?),
                ElementName::HANCOM__CORE__RIGHT => right = Some(child.try_into()?),
                ElementName::HANCOM__CORE__PREVIOUS => previous = Some(child.try_into()?),
                ElementName::HANCOM__CORE__NEXT => next = Some(child.try_into()?),
                _ => continue,
            }
        }

        let (indent, left, right, previous, next) = match (indent, left, right, previous, next) {
            (Some(indent), Some(left), Some(right), Some(previous), Some(next)) => {
                (indent, left, right, previous, next)
            }
            (None, _, _, _, _) => missing_element!("<margin indent>"),
            (_, None, _, _, _) => missing_element!("<margin left>"),
            (_, _, None, _, _) => missing_element!("<margin right>"),
            (_, _, _, None, _) => missing_element!("<margin previous>"),
            (_, _, _, _, None) => missing_element!("<margin next>"),
        };

        Ok(Self {
            indent,
            left,
            right,
            previous,
            next,
        })
    }
}

impl TryFrom<AnyElement> for ParagraphLineSpacing {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__LINE_SPACING)?;

        let mut r#type = None;
        let mut value = None;
        let mut unit = None;

        for (key, value_) in element.attributes {
            match key.as_str() {
                "type" => r#type = Some(value_.parse()?),
                "value" => value = Some(value_.parse()?),
                "unit" => unit = Some(value_.parse()?),
                _ => continue,
            }
        }

        let (r#type, value, unit) = match (r#type, value, unit) {
            (Some(r#type), Some(value), Some(unit)) => (r#type, value, unit),
            (None, _, _) => missing_attribute!("<lineSpacing type>"),
            (_, None, _) => missing_attribute!("<lineSpacing value>"),
            (_, _, None) => missing_attribute!("<lineSpacing unit>"),
        };

        Ok(Self {
            r#type,
            value,
            unit,
        })
    }
}

impl TryFrom<AnyElement> for ParagraphBorder {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__BORDER)?;

        let mut border_fill_id_ref = None;
        let mut offset_left = 0;
        let mut offset_right = 0;
        let mut offset_top = 0;
        let mut offset_bottom = 0;
        let mut connect = false;
        let mut ignore_margin = false;

        for (key, value) in element.attributes {
            match key.as_str() {
                "borderFillIDRef" => border_fill_id_ref = Some(value.parse()?),
                "offsetLeft" => offset_left = value.parse()?,
                "offsetRight" => offset_right = value.parse()?,
                "offsetTop" => offset_top = value.parse()?,
                "offsetBottom" => offset_bottom = value.parse()?,
                "connect" => connect = boolean!(value.as_str(), "<border connect>"),
                "ignoreMargin" => ignore_margin = boolean!(value.as_str(), "<border ignoreMargin>"),
                _ => continue,
            }
        }

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

impl TryFrom<AnyElement> for ParagraphAutoSpacing {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__AUTO_SPACING)?;

        let mut e_asian_eng = None;
        let mut e_asian_num = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "eAsianEng" => e_asian_eng = boolean!(value.as_str(), "<autoSpacing eAsianEng>"),
                "eAsianNum" => e_asian_num = boolean!(value.as_str(), "<autoSpacing eAsianNum>"),
                _ => continue,
            }
        }

        let (e_asian_eng, e_asian_num) = match (e_asian_eng, e_asian_num) {
            (Some(e_asian_eng), Some(e_asian_num)) => (e_asian_eng, e_asian_num),
            (None, _) => missing_attribute!("<autoSpacing eAsianEng>"),
            (_, None) => missing_attribute!("<autoSpacing eAsianNum>"),
        };

        Ok(Self {
            e_asian_eng,
            e_asian_num,
        })
    }
}

impl TryFrom<AnyElement> for StyleType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__STYLE)?;

        let mut id = None;
        let mut r#type = None;
        let mut name = None;
        let mut eng_name = None;
        let mut para_pr_id_ref = None;
        let mut char_pr_id_ref = None;
        let mut next_style_id_ref = None;
        let mut lang_id = None;
        let mut lock_form = false;

        for (key, value) in element.attributes {
            match key.as_str() {
                "id" => id = Some(value.parse()?),
                "type" => r#type = Some(value.parse()?),
                "name" => name = Some(value),
                "engName" => eng_name = Some(value),
                "paraPrIDRef" => para_pr_id_ref = Some(value.parse()?),
                "charPrIDRef" => char_pr_id_ref = Some(value.parse()?),
                "nextStyleIDRef" => next_style_id_ref = Some(value.parse()?),
                "langID" => lang_id = Some(value.parse()?),
                "lockForm" => lock_form = boolean!(value.as_str(), "<style lockForm>"),
                _ => continue,
            }
        }

        let (id, r#type, name) = match (id, r#type, name) {
            (Some(id), Some(r#type), Some(name)) => (id, r#type, name),
            (None, _, _) => missing_attribute!("<style id>"),
            (_, None, _) => missing_attribute!("<style type>"),
            (_, _, None) => missing_attribute!("<style name>"),
        };

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

impl TryFrom<AnyElement> for MemoShapeType {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__MEMO_PROPERTY)?;

        let mut id = None;
        let mut width = None;
        let mut line_width = None;
        let mut line_type = None;
        let mut line_color = None;
        let mut fill_color = None;
        let mut active_color = None;
        let mut memo_type = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "id" => id = Some(value.parse()?),
                "width" => width = Some(value.parse()?),
                "lineWidth" => line_width = Some(value.parse()?),
                "lineType" => line_type = Some(value.parse()?),
                "lineColor" => line_color = Some(value.parse()?),
                "fillColor" => fill_color = Some(value.parse()?),
                "activeColor" => active_color = Some(value.parse()?),
                "memoType" => memo_type = Some(value.parse()?),
                _ => continue,
            }
        }

        let (id, width, line_width, line_type, line_color, fill_color, active_color, memo_type) =
            match (
                id,
                width,
                line_width,
                line_type,
                line_color,
                fill_color,
                active_color,
                memo_type,
            ) {
                (
                    Some(id),
                    Some(width),
                    Some(line_width),
                    Some(line_type),
                    Some(line_color),
                    Some(fill_color),
                    Some(active_color),
                    Some(memo_type),
                ) => (
                    id,
                    width,
                    line_width,
                    line_type,
                    line_color,
                    fill_color,
                    active_color,
                    memo_type,
                ),
                (None, _, _, _, _, _, _, _) => missing_attribute!("<memoShape id>"),
                (_, None, _, _, _, _, _, _) => missing_attribute!("<memoShape width>"),
                (_, _, None, _, _, _, _, _) => missing_attribute!("<memoShape lineWidth>"),
                (_, _, _, None, _, _, _, _) => missing_attribute!("<memoShape lineType>"),
                (_, _, _, _, None, _, _, _) => missing_attribute!("<memoShape lineColor>"),
                (_, _, _, _, _, None, _, _) => missing_attribute!("<memoShape fillColor>"),
                (_, _, _, _, _, _, None, _) => missing_attribute!("<memoShape activeColor>"),
                (_, _, _, _, _, _, _, None) => missing_attribute!("<memoShape memoType>"),
            };

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

impl TryFrom<AnyElement> for TrackChange {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TRACK_CHANGE)?;

        let mut r#type = None;
        let mut date = None;
        let mut author_id = None;
        let mut char_shape_id = None;
        let mut para_shape_id = None;
        let mut hide = None;
        let mut id = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "type" => r#type = Some(value.parse()?),
                "date" => date = Some(value),
                "authorID" => author_id = Some(value.parse()?),
                "charPrIDRef" => char_shape_id = Some(value.parse()?),
                "paraPrIDRef" => para_shape_id = Some(value.parse()?),
                "hide" => hide = Some(boolean!(value.as_str(), "<trackChange hide>")),
                "id" => id = Some(value.parse()?),
                _ => continue,
            }
        }

        let (r#type, date, author_id, hide, id) = match (r#type, date, author_id, hide, id) {
            (Some(r#type), Some(date), Some(author_id), Some(hide), Some(id)) => {
                (r#type, date, author_id, hide, id)
            }
            (None, _, _, _, _) => missing_attribute!("<trackChange type>"),
            (_, None, _, _, _) => missing_attribute!("<trackChange date>"),
            (_, _, None, _, _) => missing_attribute!("<trackChange authorID>"),
            (_, _, _, None, _) => missing_attribute!("<trackChange hide>"),
            (_, _, _, _, None) => missing_attribute!("<trackChange id>"),
        };

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

impl TryFrom<AnyElement> for TrackChangeAuthor {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TRACK_CHANGE_AUTHOR)?;

        let mut name = None;
        let mut mark = None;
        let mut color = None;
        let mut id = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "name" => name = Some(value),
                "mark" => mark = boolean!(value.as_str(), "<trackChangeAuthor mark>"),
                "color" => color = Some(value.parse()?),
                "id" => id = Some(value.parse()?),
                _ => continue,
            }
        }

        let (id,) = match (id,) {
            (Some(id),) => (id,),
            (None,) => missing_attribute!("<trackChangeAuthor id>"),
        };

        Ok(Self {
            name,
            mark,
            color,
            id,
        })
    }
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

impl TryFrom<AnyElement> for TrackChangeConfig {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__TRACK_CHANGE_CONFIG)?;

        let mut flag = None;
        let mut track_change_encryption = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "flag" => flag = Some(value.parse()?),
                _ => continue,
            }
        }

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__TRACK_CHANGE_ENCRYPTION => {
                    track_change_encryption = Some(child.try_into()?)
                }
                _ => continue,
            }
        }

        let (flag,) = match (flag,) {
            (Some(flag),) => (flag,),
            (None,) => missing_attribute!("<trackchangeConfig flag>"),
        };

        Ok(Self {
            flag,
            track_change_encryption,
        })
    }
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

impl TryFrom<AnyElement> for DerivationKey {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__DERIVATION_KEY)?;

        let mut algorithm = None;
        let mut size = None;
        let mut count = None;
        let mut salt = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "algorithm" => algorithm = Some(value),
                "size" => size = Some(value.parse()?),
                "count" => count = Some(value.parse()?),
                "salt" => salt = Some(value),
                _ => continue,
            }
        }

        let (algorithm, size, count, salt) = match (algorithm, size, count, salt) {
            (Some(algorithm), Some(size), Some(count), Some(salt)) => {
                (algorithm, size, count, salt)
            }
            (None, _, _, _) => missing_attribute!("<derivationKey algorithm>"),
            (_, None, _, _) => missing_attribute!("<derivationKey size>"),
            (_, _, None, _) => missing_attribute!("<derivationKey count>"),
            (_, _, _, None) => missing_attribute!("<derivationKey salt>"),
        };

        Ok(Self {
            algorithm,
            size,
            count,
            salt,
        })
    }
}

impl TryFrom<AnyElement> for DocumentOption {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__DOCUMENT_OPTION)?;

        let mut link_info = None;
        let mut license_mark = None;

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__LINK_INFO => link_info = Some(child.try_into()?),
                ElementName::HANCOM__HEAD__LICENSE_MARK => license_mark = Some(child.try_into()?),
                _ => {}
            }
        }

        Ok(Self {
            link_info,
            license_mark,
        })
    }
}

impl TryFrom<AnyElement> for LinkInfo {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__LINK_INFO)?;

        let mut path = None;
        let mut page_inherit = false;
        let mut footnote_inherit = false;

        for (key, value) in element.attributes {
            match key.as_str() {
                "path" => path = Some(value),
                "pageInherit" => page_inherit = boolean!(value.as_str(), "<linkInfo pageInherit>"),
                "footnoteInherit" => {
                    footnote_inherit = boolean!(value.as_str(), "<linkInfo footnoteInherit>")
                }
                _ => continue,
            }
        }

        let path = match path {
            Some(path) => path,
            None => missing_attribute!("<linkInfo path>"),
        };

        Ok(Self {
            path,
            page_inherit,
            footnote_inherit,
        })
    }
}

impl TryFrom<AnyElement> for LicenseMark {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__LICENSE_MARK)?;

        let mut r#type = None;
        let mut flag = None;
        let mut lang = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "type" => r#type = Some(value.parse()?),
                "flag" => flag = Some(value.parse()?),
                "lang" => lang = Some(value.parse()?),
                _ => continue,
            }
        }

        let (r#type, flag, lang) = match (r#type, flag, lang) {
            (Some(r#type), Some(flag), Some(lang)) => (r#type, flag, lang),
            (None, _, _) => missing_attribute!("<licenseMark type>"),
            (_, None, _) => missing_attribute!("<licenseMark flag>"),
            (_, _, None) => missing_attribute!("<licenseMark lang>"),
        };

        Ok(Self { r#type, flag, lang })
    }
}

impl TryFrom<AnyElement> for CompatibleDocument {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__COMPATIBLE_DOCUMENT)?;

        let mut target_program = None;
        let mut layout_compatibility = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "targetProgram" => target_program = Some(value.parse()?),
                _ => continue,
            }
        }

        for child in element.children {
            match child.name {
                ElementName::HANCOM__HEAD__LAYOUT_COMPATIBILITY => {
                    layout_compatibility = Some(child.try_into()?)
                }
                _ => continue,
            }
        }

        let target_program = match target_program {
            Some(target_program) => target_program,
            None => missing_attribute!("<compatibleDocument targetProgram>"),
        };

        let layout_compatibility = layout_compatibility.unwrap_or_default();

        Ok(Self {
            target_program,
            layout_compatibility,
        })
    }
}

impl TryFrom<AnyElement> for LayoutCompatibility {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__HEAD__LAYOUT_COMPATIBILITY)?;

        let set = element
            .children
            .into_iter()
            .filter_map(|child| LayoutCompatibilityKind::from_element_name(child.name))
            .collect();

        Ok(Self { set })
    }
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
  <hh:tabItem pos="72" type="CENTER" leader="DASH" />
  <hh:tabItem pos="108" type="RIGHT" leader="DASH" />
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
