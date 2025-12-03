//! namespace: http://www.hancom.co.kr/hwpml/2011/section
//! filename: Contents/section{[0-9]+}.xml

use crate::core::{
    GutterKind, HWPUnit, LandscapeKind, PageStartKind, TextDirectionKind, VisibilityValue,
};
use nonempty::NonEmpty;

/// ```xml
/// <sec>
///   <p>...</p>
/// </sec>
/// ```
///
/// Root Element
#[derive(Debug)]
pub struct Section {
    pub paragraphs: Vec<Paragraph>,
}

/// ```xml
/// <p
///   id="{xs:nonNegativeInteger}"
///   paraPrIDRef="{xs:nonNegativeInteger}"
///   styleIDRef="{xs:nonNegativeInteger}"
///   pageBreak="{xs:boolean; default="false"}"
///   columnBreak="{xs:boolean; default="false"}"
///   merged="{xs:boolean; default="false"}"
///   paraTcId="{xs:nonNegativeInteger}"
/// >...</p>
/// ```
///
/// 문단
#[derive(Debug)]
pub struct Paragraph {
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    pub id: u32,
    /// ```xml
    /// paraPrIDRef="{xs:nonNegativeInteger}"
    /// ```
    pub para_pr_id_ref: u32,
    /// ```xml
    /// styleIDRef="{xs:nonNegativeInteger}"
    /// ```
    pub style_id_ref: Option<u32>,
    /// ```xml
    /// pageBreak="{xs:boolean; default="false"}"
    /// ```
    pub page_break: bool,
    /// ```xml
    /// columnBreak="{xs:boolean; default="false"}"
    /// ```
    pub column_break: bool,
    /// ```xml
    /// merged="{xs:boolean; default="false"}"
    /// ```
    pub merged: bool,
    /// ```xml
    /// paraTcId="{xs:nonNegativeInteger}"
    /// ```
    pub para_tc_id: Option<u32>,
    /// ```xml
    /// <run
    ///   charPrIDRef="{xs:nonNegativeInteger}"
    ///   charTcId="{xs:nonNegativeInteger}"
    /// >...</run>
    /// ```
    pub runs: Vec<Run>,
}

/// ```xml
/// <run
///  charPrIDRef="{xs:nonNegativeInteger}"
///  charTcId="{xs:nonNegativeInteger}"
/// >...</run>
/// ```
#[derive(Debug)]
pub struct Run {
    /// ```xml
    /// charPrIDRef="{xs:nonNegativeInteger}"
    /// ```
    pub char_pr_id_ref: u32,
    /// ```xml
    /// charTcId="{xs:nonNegativeInteger}"
    /// ```
    pub char_tc_id: Option<u32>,
    /// ```xml
    /// ...
    /// ```
    pub kind: RunKind,
}

#[derive(Debug)]
pub enum RunKind {
    /// ```xml
    /// <secPr>...</secPr>
    /// ```
    ///
    /// 시작 번호 정보
    Section(NonEmpty<SectionDefinitionType>),
    /// ```xml
    /// <ctrl>...</secPr>
    /// ```
    Control(()),
    /// ```xml
    /// <t>...</secPr>
    /// ```
    Text(()),
    /// ```xml
    /// <tbl>...</secPr>
    /// ```
    Table(()),
    /// ```xml
    /// <pic>...</secPr>
    /// ```
    Picture(()),
    /// ```xml
    /// <ole>...</secPr>
    /// ```
    Ole(()),
    /// ```xml
    /// <container>...</secPr>
    /// ```
    Container(()),
    /// ```xml
    /// <equation>...</secPr>
    /// ```
    Equation(()),
    /// ```xml
    /// <line>...</secPr>
    /// ```
    Line(()),
    /// ```xml
    /// <ellipse>...</secPr>
    /// ```
    Ellipse(()),
    /// ```xml
    /// <arc>...</secPr>
    /// ```
    Arc(()),
    /// ```xml
    /// <polygon>...</secPr>
    /// ```
    Polygon(()),
    /// ```xml
    /// <curve,>...</secPr>
    /// ```
    Curve(()),
    /// ```xml
    /// <connectLine>...</secPr>
    /// ```
    ConnectLine(()),
    /// ```xml
    /// <textart>...</secPr>
    /// ```
    TextArt(()),
    /// ```xml
    /// <compose>...</secPr>
    /// ```
    Compose(()),
    /// ```xml
    /// <dutmal>...</secPr>
    /// ```
    Dutmal(()),
    /// ```xml
    /// <btn>...</secPr>
    /// ```
    Button(()),
    /// ```xml
    /// <radioBtn>...</secPr>
    /// ```
    RadioButton(()),
    /// ```xml
    /// <checkBtn>...</secPr>
    /// ```
    CheckButton(()),
    /// ```xml
    /// <comboBox,>...</secPr>
    /// ```
    ComboBox(()),
    /// ```xml
    /// <listBox>...</secPr>
    /// ```
    ListBox(()),
    /// ```xml
    /// <edit>...</secPr>
    /// ```
    Edit(()),
    /// ```xml
    /// <scrollBar>...</secPr>
    /// ```
    ScrollBar(()),
    /// ```xml
    /// <video>...</secPr>
    /// ```
    Video(()),
    /// ```xml
    /// <chart>...</secPr>
    /// ```
    Chart(()),
}

/// ```xml
/// <secPr
///   id="{xs:nonNegativeInteger}"
///   textDirection="{$TextDirectionKind; default="HORIZONTAL"}"
///   spaceColumns="{xs:integer; default="0"}"
///   tabStopVal="{xs:integer; default="0"}"
///   tabStopUnit="{$HWPUnit; default="HWPUNIT"}"
///   outlineShapeIDRef="{xs:nonNegativeInteger}"
///   memoShapeIDRef="{xs:nonNegativeInteger}"
///   textVerticalWidthHead="{xs:boolean; default="false"}"
///   masterPageCnt="{xs:nonNegativeInteger; default="0"}"
/// >...</secPr>
/// ```
///
/// 시작 번호 정보
#[derive(Debug)]
pub struct SectionDefinitionType {
    /// ```xml
    /// id="{xs:nonNegativeInteger}"
    /// ```
    pub id: u32,
    /// ```xml
    /// textDirection="{$TextDirectionKind; default="HORIZONTAL"}"
    /// ```
    ///
    /// 텍스트 방향
    pub text_direction: TextDirectionKind,
    /// ```xml
    /// spaceColumns="{xs:integer; default="0"}"
    /// ```
    ///
    /// 동일한 페이지에서 서로 다른 단 사이의 간격
    pub space_columns: i32,
    /// ```xml
    /// tabStopVal="{xs:integer; default="0"}"
    /// ```
    ///
    /// 기본 탭 간격
    pub tab_stop_val: i32,
    /// ```xml
    /// tabStopUnit="{$HWPUnit; default="HWPUNIT"}"
    /// ```
    ///
    /// 기본 탭 간격 단위
    pub tab_stop_unit: HWPUnit,
    /// ```xml
    /// outlineShapeIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 개요 번호 모양 아이디 참조 값
    pub outline_shape_id_ref: Option<u32>,
    /// ```xml
    /// memoShapeIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 구역 내에서 사용되는 메모의 모양을 설정하기 위한 아이디 참조 값
    pub memo_shape_id_ref: Option<u32>,
    /// ```xml
    /// textVerticalWidthHead="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 머리말/꼬리말 세로 쓰기 여부
    pub text_vertical_width_head: bool,
    /// ```xml
    /// masterPageCnt="{xs:nonNegativeInteger; default="0"}"
    /// ```
    ///
    /// 확장 바탕쪽 개수
    pub master_page_count: u32,
    /// ```xml
    /// <startNum>...</startNum>
    /// ```
    ///
    /// 시작 번호 정보
    pub start_number: Option<StartNumber>,
    /// ```xml
    /// <grid>...</grid>
    /// ```
    ///
    /// 줄 맞춤 정보
    pub grid: Option<Grid>,
    /// ```xml
    /// <visibility>...</visibility>
    /// ```
    ///
    /// 감추기/보여주기 정보
    pub visibility: Option<Visibility>,
    /// ```xml
    /// <lineNumberShape>...</lineNumberShape>
    /// ```
    ///
    /// 줄 번호 정보
    pub line_number_shape: Option<LineNumberShape>,
    /// ```xml
    /// <pagePr>...</pagePr>
    /// ```
    ///
    /// 용지 설정 정보
    pub page_properties: Option<PageProperties>,
    /// ```xml
    /// <footNotePr>...</footNotePr>
    /// ```
    ///
    /// 각주 모양
    pub footnote_properties: Option<FootnoteProperties>,
    /// ```xml
    /// <endNotePr>...</endNotePr>
    /// ```
    ///
    /// 미주 모양
    pub endnote_properties: Option<()>,
    /// ```xml
    /// <pageBorderFill>...</pageBorderFill>
    /// ```
    ///
    /// 쪽 테두리/배경 정보
    pub page_border_fill: Option<()>,
    /// ```xml
    /// <masterPage>...</masterPage>
    /// ```
    ///
    /// 바탕쪽 정보
    pub master_pages: Vec<()>,
    /// ```xml
    /// <presentation>...</presentation>
    /// ```
    ///
    /// 프레젠테이션 정보
    pub presentation: Option<()>,
}

/// ```xml
/// <startNum
///   pageStartsOn="{$PageStartKind; default="BOTH"}"
///   page="{xs:nonNegativeInteger; default="0"}"
///   pic="{xs:nonNegativeInteger; default="0"}"
///   tbl="{xs:nonNegativeInteger; default="0"}"
///   equation="{xs:nonNegativeInteger; default="0"}"
/// />
/// ```
///
/// 시작 번호 정보
#[derive(Debug)]
pub struct StartNumber {
    /// ```xml
    /// pageStartsOn="{$PageStartKind; default="BOTH"}"
    /// ```
    ///
    /// 구역 나눔으로 새 페이지가 생길 때 페이지 번호 적용 옵션
    pub page_starts_on: PageStartKind,
    /// ```xml
    /// page="{xs:nonNegativeInteger; default="0"}"
    /// ```
    ///
    /// 쪽 시작 번호
    pub page: u32,
    /// ```xml
    /// pic="{xs:nonNegativeInteger; default="0"}"
    /// ```
    ///
    /// 그림 시작 번호
    pub picture: u32,
    /// ```xml
    /// tbl="{xs:nonNegativeInteger; default="0"}"
    /// ```
    ///
    /// 표 시작 번호
    pub table: u32,
    /// ```xml
    /// equation="{xs:nonNegativeInteger; default="0"}"
    /// ```
    ///
    /// 수식 시작 번호
    pub equation: u32,
}

/// ```xml
/// <grid
///   lineGrid="{xs:nonNegativeInteger; default="0"}"
///   charGrid="{xs:nonNegativeInteger; default="0"}"
///   wonggojiFormat="{xs:boolean; default="0"}"
/// />
/// ```
///
/// 줄 맞춤 정보
#[derive(Debug)]
pub struct Grid {
    /// ```xml
    /// lineGrid="{xs:nonNegativeInteger; default="0"}"
    /// ```
    ///
    /// 줄 격자 간격
    pub line_grid: u32,
    /// ```xml
    /// charGrid="{xs:nonNegativeInteger; default="0"}"
    /// ```
    ///
    /// 글자 격자 간격
    pub char_grid: u32,
    /// ```xml
    /// wonggojiFormat="{xs:boolean; default="0"}"
    /// ```
    ///
    /// 원고지 모드 여부
    pub wonggoji_format: bool,
}

/// ```xml
/// <visibility
///   hideFirstHeader="{xs:boolean; default="false"}"
///   hideFirstFooter="{xs:boolean; default="false"}"
///   hideFirstMasterPage="{xs:boolean; default="false"}"
///   border="{hp:VisibilityValue}"
///   fill="{hp:VisibilityValue}"
///   hideFirstPageNumber="{xs:boolean; default="false"}"
///   hideFirstEmptyLines="{xs:boolean; default="false"}"
///   showLineNumbers="{xs:boolean; default="false"}"
/// />/ ```
///
/// 감추기/보여주기 정보
#[derive(Debug)]
pub struct Visibility {
    /// ```xml
    /// hideFirstHeader="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 첫 쪽에만 머리말 감추기 여부
    pub hide_first_header: bool,
    /// ```xml
    /// hideFirstFooter="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 첫 쪽에만 꼬리말 감추기 여부
    pub hide_first_footer: bool,
    /// ```xml
    /// hideFirstMasterPage="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 첫 쪽에만 바탕쪽 감추기 여부
    pub hide_first_master_page: bool,
    /// ```xml
    /// border="{hp:VisibilityValue}"
    /// ```
    pub border: VisibilityValue,
    /// ```xml
    /// fill="{hp:VisibilityValue}"
    /// ```
    ///
    pub fill: VisibilityValue,
    /// ```xml
    /// hideFirstPageNumber="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 첫 쪽에만 쪽번호 감추기 여부
    pub hide_first_page_number: bool,
    /// ```xml
    /// hideFirstEmptyLines="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 첫 쪽에만 번줄 감추기 여부
    pub hide_first_empty_lines: bool,
    /// ```xml
    /// showLineNumbers="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 줄 번호 표시 여부
    pub show_line_numbers: bool,
}

/// ```xml
/// <lineNumberShape
///   restartType="{xs:unsignedInt}"
///   countBy="{xs:unsignedInt}"
///   distance="{xs:unsignedInt}"
///   startNumber="{xs:unsignedInt}"
/// />
/// ```
///
/// 줄 번호 정보
#[derive(Debug)]
pub struct LineNumberShape {
    /// ```xml
    /// restartType="{xs:unsignedInt}"
    /// ```
    ///
    /// 줄 번호 방식
    pub restart_type: u32,
    /// ```xml
    /// countBy="{xs:unsignedInt}"
    /// ```
    ///
    /// 줄 번호 표시 간격
    pub count_by: u32,
    /// ```xml
    /// distance="{xs:unsignedInt}"
    /// ```
    ///
    /// 본문과의 줄 번호 위치
    pub distance: u32,
    /// ```xml
    /// startNumber="{xs:unsignedInt}"
    /// ```
    ///
    /// 줄 번호 시작 번호
    pub start_number: u32,
}

/// ```xml
/// <pagePr
///   landscape="{$LandscapeKind; default="NARROWLY"}"
///   width="{xs:positiveInteger; default="59528"}"
///   height="{xs:positiveInteger; default="84188"}"
///   gutterType="{$GutterTypeKind; default="LEFT_ONLY"}"
/// />
/// ```
///
/// 용지 설정 정보
#[derive(Debug)]
pub struct PageProperties {
    /// ```xml
    /// landscape="{$LandscapeKind; default="NARROWLY"}"
    /// ```
    ///
    /// 용지 방향
    pub landscape: LandscapeKind,
    /// ```xml
    /// width="{xs:positiveInteger; default="59528"}"
    /// ```
    ///
    /// 용지 가로 크기. 단위는 HWPUNIT.
    pub width: u32,
    /// ```xml
    /// height="{xs:positiveInteger; default="84188"}"
    /// ```
    ///
    /// 용지 세로 크기. 단위는 HWPUNIT.
    pub height: u32,
    /// ```xml
    /// gutterType="{$GutterTypeKind; default="LEFT_ONLY"}"
    /// ```
    ///
    /// 제책 방법
    pub gutter_type: GutterKind,
}

/// ```xml
/// <footNotePr>
///   <numbering>...</numbering>
///   <placement>...</placement>
/// </footNotePr>
/// ```
///
/// 각주 모양
#[derive(Debug)]
pub struct FootnoteProperties {
    /// ```xml
    /// <numbering>...</numbering>
    /// ```
    pub numbering: (),
    /// ```xml
    /// <placement>...</placement>
    /// ```
    pub placement: (),
}
