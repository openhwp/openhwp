//! namespace: http://www.hancom.co.kr/hwpml/2011/section
//! filename: Contents/section{[0-9]+}.xml

use crate::{
    arbitrary::{
        EndNoteNumberingKind, EndNotePlaceKind, FootNoteNumberingKind, FootNotePlaceKind,
        GutterKind, LandscapeKind, NoteLineLength, PageStartKind, TextDirectionKind, VisibilityValue,
    },
    core::{HWPUnit, LineType2, LineWidth, NumberType2, RgbColorType},
    xs,
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
    pub id: xs::NonNegativeInteger32,
    /// ```xml
    /// paraPrIDRef="{xs:nonNegativeInteger}"
    /// ```
    pub para_pr_id_ref: xs::NonNegativeInteger32,
    /// ```xml
    /// styleIDRef="{xs:nonNegativeInteger}"
    /// ```
    pub style_id_ref: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// pageBreak="{xs:boolean; default="false"}"
    /// ```
    pub page_break: xs::Boolean,
    /// ```xml
    /// columnBreak="{xs:boolean; default="false"}"
    /// ```
    pub column_break: xs::Boolean,
    /// ```xml
    /// merged="{xs:boolean; default="false"}"
    /// ```
    pub merged: xs::Boolean,
    /// ```xml
    /// paraTcId="{xs:nonNegativeInteger}"
    /// ```
    pub para_tc_id: Option<xs::NonNegativeInteger32>,
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
    pub char_pr_id_ref: xs::NonNegativeInteger32,
    /// ```xml
    /// charTcId="{xs:nonNegativeInteger}"
    /// ```
    pub char_tc_id: Option<xs::NonNegativeInteger32>,
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
    pub id: xs::NonNegativeInteger32,
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
    pub space_columns: xs::Integer32,
    /// ```xml
    /// tabStopVal="{xs:integer; default="0"}"
    /// ```
    ///
    /// 기본 탭 간격
    pub tab_stop_val: xs::Integer32,
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
    pub outline_shape_id_ref: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// memoShapeIDRef="{xs:nonNegativeInteger}"
    /// ```
    ///
    /// 구역 내에서 사용되는 메모의 모양을 설정하기 위한 아이디 참조 값
    pub memo_shape_id_ref: Option<xs::NonNegativeInteger32>,
    /// ```xml
    /// textVerticalWidthHead="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 머리말/꼬리말 세로 쓰기 여부
    pub text_vertical_width_head: xs::Boolean,
    /// ```xml
    /// masterPageCnt="{xs:nonNegativeInteger; default="0"}"
    /// ```
    ///
    /// 확장 바탕쪽 개수
    pub master_page_count: xs::NonNegativeInteger32,
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
    pub footnote_properties: Option<FootNoteShapeType>,
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
    pub page: xs::NonNegativeInteger32,
    /// ```xml
    /// pic="{xs:nonNegativeInteger; default="0"}"
    /// ```
    ///
    /// 그림 시작 번호
    pub picture: xs::NonNegativeInteger32,
    /// ```xml
    /// tbl="{xs:nonNegativeInteger; default="0"}"
    /// ```
    ///
    /// 표 시작 번호
    pub table: xs::NonNegativeInteger32,
    /// ```xml
    /// equation="{xs:nonNegativeInteger; default="0"}"
    /// ```
    ///
    /// 수식 시작 번호
    pub equation: xs::NonNegativeInteger32,
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
    pub line_grid: xs::NonNegativeInteger32,
    /// ```xml
    /// charGrid="{xs:nonNegativeInteger; default="0"}"
    /// ```
    ///
    /// 글자 격자 간격
    pub char_grid: xs::NonNegativeInteger32,
    /// ```xml
    /// wonggojiFormat="{xs:boolean; default="0"}"
    /// ```
    ///
    /// 원고지 모드 여부
    pub wonggoji_format: xs::Boolean,
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
    pub hide_first_header: xs::Boolean,
    /// ```xml
    /// hideFirstFooter="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 첫 쪽에만 꼬리말 감추기 여부
    pub hide_first_footer: xs::Boolean,
    /// ```xml
    /// hideFirstMasterPage="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 첫 쪽에만 바탕쪽 감추기 여부
    pub hide_first_master_page: xs::Boolean,
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
    pub hide_first_page_number: xs::Boolean,
    /// ```xml
    /// hideFirstEmptyLines="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 첫 쪽에만 번줄 감추기 여부
    pub hide_first_empty_lines: xs::Boolean,
    /// ```xml
    /// showLineNumbers="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 줄 번호 표시 여부
    pub show_line_numbers: xs::Boolean,
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
    pub restart_type: xs::NonNegativeInteger32,
    /// ```xml
    /// countBy="{xs:unsignedInt}"
    /// ```
    ///
    /// 줄 번호 표시 간격
    pub count_by: xs::NonNegativeInteger32,
    /// ```xml
    /// distance="{xs:unsignedInt}"
    /// ```
    ///
    /// 본문과의 줄 번호 위치
    pub distance: xs::NonNegativeInteger32,
    /// ```xml
    /// startNumber="{xs:unsignedInt}"
    /// ```
    ///
    /// 줄 번호 시작 번호
    pub start_number: xs::NonNegativeInteger32,
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
    pub width: xs::PositiveInteger32,
    /// ```xml
    /// height="{xs:positiveInteger; default="84188"}"
    /// ```
    ///
    /// 용지 세로 크기. 단위는 HWPUNIT.
    pub height: xs::PositiveInteger32,
    /// ```xml
    /// gutterType="{$GutterTypeKind; default="LEFT_ONLY"}"
    /// ```
    ///
    /// 제책 방법
    pub gutter_type: GutterKind,
}

/// ```xml
/// <autoNumFormat
///   type="{hc:NumberType2; default="DIGIT"}"
///   userChar="{xs:string}"
///   prefixChar="{xs:string}"
///   suffixChar="{xs:string}"
///   supscript="{xs:boolean; default="false"}"
/// />
/// ```
#[derive(Debug)]
pub struct AutoNumberFormat {
    /// ```xml
    /// type="{hc:NumberType2; default="DIGIT"}"
    /// ```
    ///
    /// 번호 모양 종류
    pub number_type: NumberType2,
    /// ```xml
    /// userChar="{xs:string}"
    /// ```
    ///
    /// 사용자 기호
    pub user_char: Option<xs::String>,
    /// ```xml
    /// prefixChar="{xs:string}"
    /// ```
    ///
    /// 앞 장식 문자
    pub prefix_char: Option<xs::String>,
    /// ```xml
    /// suffixChar="{xs:string}"
    /// ```
    ///
    /// 뒤 장식 문자
    pub suffix_char: Option<xs::String>,
    /// ```xml
    /// supscript="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 각주/미주 내용 중 번호 코드의 모양을 위첨자 형식으로 할지 여부
    pub superscript: xs::Boolean,
}

/// ```xml
/// <noteLine
///   length="{xs:integer; default="0"}"
///   type="{hc:LineType2; default="SOLID"}"
///   width="{hc:LineWidth; default="0.12 mm"}"
///   color="{hc:RGBColorType; default="#000000"}"
/// />
/// ```
#[derive(Debug)]
pub struct NoteLine {
    /// ```xml
    /// length="{$NoteLineLength; default="0"}"
    /// ```
    ///
    /// 구분선 길이, 0(구분선 없음), -1 (5 cm), -2 (2 cm), -3 (단 크기의 1/3), -4 (단 크기), 그 외 (HWPUNIT 단위의 사용자 지정 길이)
    pub length: NoteLineLength,
    /// ```xml
    /// type="{hc:LineType2; default="SOLID"}"
    /// ```
    ///
    /// 구분선 종류.
    pub line_type: LineType2,
    /// ```xml
    /// width="{hc:LineWidth; default="0.12 mm"}"
    /// ```
    ///
    /// 구분선 굵기. 단위는 mm.
    pub width: LineWidth,
    /// ```xml
    /// color="{hc:RGBColorType; default="#000000"}"
    /// ```
    ///
    /// 구분선 색.
    pub color: RgbColorType,
}

/// ```xml
/// <noteSpacing
///   betweenNotes="{xs:nonNegativeInteger; default="850"}"
///   belowLine="{xs:nonNegativeInteger; default="567"}"
///   aboveLine="{xs:nonNegativeInteger; default="567"}"
/// />
/// ```
#[derive(Debug)]
pub struct NoteSpacing {
    /// ```xml
    /// betweenNotes="{xs:nonNegativeInteger; default="850"}"
    /// ```
    ///
    /// 주석 사이 여백
    pub between_notes: xs::NonNegativeInteger32,
    /// ```xml
    /// belowLine="{xs:nonNegativeInteger; default="567"}"
    /// ```
    ///
    /// 구분선 아래 여백.
    pub below_line: xs::NonNegativeInteger32,
    /// ```xml
    /// aboveLine="{xs:nonNegativeInteger; default="567"}"
    /// ```
    ///
    /// 구분선 위 여백.
    pub above_line: xs::NonNegativeInteger32,
}

/// ```xml
/// <footNotePr>
///   <autoNumFormat>...</autoNumFormat>
///   <noteLine>...</noteLine>
///   <noteSpacing>...</noteSpacing>
///   <numbering>...</numbering>
///   <placement>...</placement>
/// </footNotePr>
/// ```
///
/// 각주 모양
#[derive(Debug)]
pub struct FootNoteShapeType {
    /// ```xml
    /// <autoNumFormat>...</autoNumFormat>
    /// ```
    pub auto_number_format: AutoNumberFormat,
    /// ```xml
    /// <noteLine>...</noteLine>
    /// ```
    pub note_line: NoteLine,
    /// ```xml
    /// <noteSpacing>...</noteSpacing>
    /// ```
    pub note_spacing: NoteSpacing,
    /// ```xml
    /// <numbering>...</numbering>
    /// ```
    ///
    /// 각주 번호 매기기
    pub numbering: FootNoteNumbering,
    /// ```xml
    /// <placement>...</placement>
    /// ```
    ///
    /// 각주 위치
    pub placement: FootNotePlacement,
}

/// ```xml
/// <numbering
///   type="{$FootNoteNumberingKind; default="CONTINUOUS"}"
///   newNum="{xs:positiveInteger; default="1"}"
/// />
/// ```
#[derive(Debug)]
pub struct FootNoteNumbering {
    /// ```xml
    /// type="{$FootNoteNumberingKind; default="CONTINUOUS"}"
    /// ```
    ///
    /// 각주 번호 매기기 방식
    pub r#type: FootNoteNumberingKind,
    /// ```xml
    /// newNum="{xs:positiveInteger; default="1"}"
    /// ```
    ///
    /// 새 번호
    pub new_number: xs::PositiveInteger32,
}

/// ```xml
/// <placement
///   place="{$FootNotePlaceKind; default="EACH_COLUMN"}"
///   beneathText="{xs:boolean; default="false"}"
/// />
/// ```
#[derive(Debug)]
pub struct FootNotePlacement {
    /// ```xml
    /// place="{$FootNotePlaceKind; default="EACH_COLUMN"}"
    /// ```
    ///
    /// 한 페이지 내에서 각주를 다단에 어떻게 위치시킬지를 표시한다
    pub place: FootNotePlaceKind,
    /// ```xml
    /// beneathText="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 텍스트에 이어 바로 출력할지 여부
    pub beneath_text: xs::Boolean,
}

/// ```xml
/// <endNotePr>
///   <autoNumFormat>...</autoNumFormat>
///   <noteLine>...</noteLine>
///   <noteSpacing>...</noteSpacing>
///   <numbering>...</numbering>
///   <placement>...</placement>
/// </endNotePr>
/// ```
///
/// 미주 모양
#[derive(Debug)]
pub struct EndNoteShapeType {
    /// ```xml
    /// <autoNumFormat>...</autoNumFormat>
    /// ```
    pub auto_number_format: AutoNumberFormat,
    /// ```xml
    /// <noteLine>...</noteLine>
    /// ```
    pub note_line: NoteLine,
    /// ```xml
    /// <noteSpacing>...</noteSpacing>
    /// ```
    pub note_spacing: NoteSpacing,
    /// ```xml
    /// <numbering>...</numbering>
    /// ```
    ///
    /// 각주 번호 매기기
    pub numbering: EndNoteNumbering,
    /// ```xml
    /// <placement>...</placement>
    /// ```
    ///
    /// 각주 위치
    pub placement: EndNotePlacement,
}

/// ```xml
/// <numbering
///   type="{$EndNoteNumberingKind; default="CONTINUOUS"}"
///   newNum="{xs:positiveInteger; default="1"}"
/// />
/// ```
#[derive(Debug)]
pub struct EndNoteNumbering {
    /// ```xml
    /// type="{$EndNoteNumberingKind; default="CONTINUOUS"}"
    /// ```
    ///
    /// 각주 번호 매기기 방식
    pub r#type: EndNoteNumberingKind,
    /// ```xml
    /// newNum="{xs:positiveInteger; default="1"}"
    /// ```
    ///
    /// 시작 번호. type 이 ON_SECTION일 때만 사용한다.
    pub new_number: xs::PositiveInteger32,
}

/// ```xml
/// <placement
///   place="{$EndNotePlaceKind; default="END_OF_DOCUMENT"}"
///   beneathText="{xs:boolean; default="false"}"
/// />
/// ```
#[derive(Debug)]
pub struct EndNotePlacement {
    /// ```xml
    /// place="{$EndNotePlaceKind; default="END_OF_DOCUMENT"}"
    /// ```
    ///
    /// 한 페이지 내에서 미주를 다단에 어떻게 위치시킬지를 표시한다
    pub place: EndNotePlaceKind,
    /// ```xml
    /// beneathText="{xs:boolean; default="false"}"
    /// ```
    ///
    /// 텍스트에 이어 바로 출력할지 여부
    pub beneath_text: xs::Boolean,
}
