//! [AI 생성] 구역 정의 (페이지/각주/미주/프레젠테이션 설정)
//!
//! KS X 6101:2024 - paralist.xsd

use serde::{Deserialize, Serialize};

use super::enums::{
    EndnoteNumberingType, EndnotePlacement, FillAreaType, FootnoteNumberingType, FootnotePlacement,
    GutterType, PageBorderPosition, PageBorderType, PageStartsOn, PaperOrientation, TabStopUnit,
    TextDirection, VisibilityValue,
};
use crate::core::{
    enums::{LineStyleType2, LineWidth, NumberFormatType2},
    types::{
        BorderFillIdRef, FillBrush, MasterPageIdRef, MemoShapeIdRef, OutlineShapeIdRef, RgbColor,
        SoundIdRef,
    },
};

/// [AI 생성] 시작 번호 정보
///
/// 원본: `startNum` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "startNum")]
pub struct SectionStartNumber {
    /// [AI 생성] 구역 나눔으로 새 페이지가 생길 때 페이지 번호 적용 옵션
    ///
    /// 원본: `pageStartsOn` 속성
    #[serde(rename = "@pageStartsOn", default)]
    pub page_starts_on: PageStartsOn,

    /// [AI 생성] 쪽 시작 번호
    ///
    /// 원본: `page` 속성
    #[serde(rename = "@page", default)]
    pub page: u32,

    /// [AI 생성] 그림 시작 번호
    ///
    /// 원본: `pic` 속성
    #[serde(rename = "@pic", default)]
    pub picture: u32,

    /// [AI 생성] 표 시작 번호
    ///
    /// 원본: `tbl` 속성
    #[serde(rename = "@tbl", default)]
    pub table: u32,

    /// [AI 생성] 수식 시작 번호
    ///
    /// 원본: `equation` 속성
    #[serde(rename = "@equation", default)]
    pub equation: u32,
}

/// [AI 생성] 줄맞춤 정보
///
/// 원본: `grid` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "grid")]
pub struct SectionGrid {
    /// [AI 생성] 세로로 줄맞춤을 할지 여부
    ///
    /// 원본: `lineGrid` 속성
    #[serde(rename = "@lineGrid", default)]
    pub line_grid: u32,

    /// [AI 생성] 가로로 줄맞춤을 할지 여부
    ///
    /// 원본: `charGrid` 속성
    #[serde(rename = "@charGrid", default)]
    pub character_grid: u32,

    /// [AI 생성] 원고지 형식 여부
    ///
    /// 원본: `wonggojiFormat` 속성
    #[serde(rename = "@wonggojiFormat", default)]
    pub manuscript_format: bool,
}

/// [AI 생성] 감추기/보여주기 정보
///
/// 원본: `visibility` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "visibility")]
pub struct SectionVisibility {
    /// [AI 생성] 첫 쪽에만 머리말 감추기 여부
    ///
    /// 원본: `hideFirstHeader` 속성
    #[serde(rename = "@hideFirstHeader", default)]
    pub hide_first_header: bool,

    /// [AI 생성] 첫 쪽에만 꼬리말 감추기 여부
    ///
    /// 원본: `hideFirstFooter` 속성
    #[serde(rename = "@hideFirstFooter", default)]
    pub hide_first_footer: bool,

    /// [AI 생성] 첫 쪽에만 바탕쪽 감추기 여부
    ///
    /// 원본: `hideFirstMasterPage` 속성
    #[serde(rename = "@hideFirstMasterPage", default)]
    pub hide_first_master_page: bool,

    /// [AI 생성] 테두리 가시성
    ///
    /// 원본: `border` 속성
    #[serde(rename = "@border", skip_serializing_if = "Option::is_none")]
    pub border: Option<VisibilityValue>,

    /// [AI 생성] 채우기 가시성
    ///
    /// 원본: `fill` 속성
    #[serde(rename = "@fill", skip_serializing_if = "Option::is_none")]
    pub fill: Option<VisibilityValue>,

    /// [AI 생성] 첫 쪽에만 쪽번호 감추기 여부
    ///
    /// 원본: `hideFirstPageNum` 속성
    #[serde(rename = "@hideFirstPageNum", default)]
    pub hide_first_page_number: bool,

    /// [AI 생성] 첫 쪽에만 빈줄 감추기 여부
    ///
    /// 원본: `hideFirstEmptyLine` 속성
    #[serde(rename = "@hideFirstEmptyLine", default)]
    pub hide_first_empty_line: bool,

    /// [AI 생성] 줄번호 표시 여부
    ///
    /// 원본: `showLineNumber` 속성
    #[serde(rename = "@showLineNumber", default)]
    pub show_line_number: bool,
}

/// [AI 생성] 줄 번호 정보
///
/// 원본: `lineNumberShape` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "lineNumberShape")]
pub struct LineNumberShape {
    /// [AI 생성] 줄 번호 방식
    ///
    /// 원본: `restartType` 속성
    #[serde(rename = "@restartType", skip_serializing_if = "Option::is_none")]
    pub restart_type: Option<u32>,

    /// [AI 생성] 줄 번호 표시 간격
    ///
    /// 원본: `countBy` 속성
    #[serde(rename = "@countBy", skip_serializing_if = "Option::is_none")]
    pub count_by: Option<u32>,

    /// [AI 생성] 본문과의 줄 번호 위치
    ///
    /// 원본: `distance` 속성
    #[serde(rename = "@distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<u32>,

    /// [AI 생성] 줄 번호 시작 번호
    ///
    /// 원본: `startNumber` 속성
    #[serde(rename = "@startNumber", skip_serializing_if = "Option::is_none")]
    pub start_number: Option<u32>,
}

/// [AI 생성] 용지 여백
///
/// 원본: `pagePr/margin` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "margin")]
pub struct PageMargin {
    /// [AI 생성] 왼쪽 여백
    ///
    /// 원본: `left` 속성
    #[serde(rename = "@left", default)]
    pub left: u32,

    /// [AI 생성] 오른쪽 여백
    ///
    /// 원본: `right` 속성
    #[serde(rename = "@right", default)]
    pub right: u32,

    /// [AI 생성] 위쪽 여백
    ///
    /// 원본: `top` 속성
    #[serde(rename = "@top", default)]
    pub top: u32,

    /// [AI 생성] 아래쪽 여백
    ///
    /// 원본: `bottom` 속성
    #[serde(rename = "@bottom", default)]
    pub bottom: u32,

    /// [AI 생성] 머리말 여백
    ///
    /// 원본: `header` 속성
    #[serde(rename = "@header", default = "default_header_footer_margin")]
    pub header: u32,

    /// [AI 생성] 꼬리말 여백
    ///
    /// 원본: `footer` 속성
    #[serde(rename = "@footer", default = "default_header_footer_margin")]
    pub footer: u32,

    /// [AI 생성] 제본 여백
    ///
    /// 원본: `gutter` 속성
    #[serde(rename = "@gutter", default)]
    pub gutter: u32,
}

fn default_header_footer_margin() -> u32 {
    4252
}

impl Default for PageMargin {
    fn default() -> Self {
        Self {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            header: 4252,
            footer: 4252,
            gutter: 0,
        }
    }
}

/// [AI 생성] 용지 설정
///
/// 원본: `pagePr` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "pagePr")]
pub struct PageProperty {
    /// [AI 생성] 여백
    ///
    /// 원본: `margin` 요소
    #[serde(rename = "margin")]
    pub margin: PageMargin,

    /// [AI 생성] 용지 방향
    ///
    /// 원본: `landscape` 속성
    #[serde(rename = "@landscape", default)]
    pub orientation: PaperOrientation,

    /// [AI 생성] 용지 가로 크기 (HWPUNIT)
    ///
    /// 원본: `width` 속성
    #[serde(rename = "@width", default = "default_page_width")]
    pub width: u32,

    /// [AI 생성] 용지 세로 크기 (HWPUNIT)
    ///
    /// 원본: `height` 속성
    #[serde(rename = "@height", default = "default_page_height")]
    pub height: u32,

    /// [AI 생성] 제본 방법
    ///
    /// 원본: `gutterType` 속성
    #[serde(rename = "@gutterType", default)]
    pub gutter_type: GutterType,
}

fn default_page_width() -> u32 {
    59528
}

fn default_page_height() -> u32 {
    84188
}

/// [AI 생성] 자동 번호 형식
///
/// 원본: `AutoNumFormatType`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "autoNumFormat")]
pub struct AutoNumberFormat {
    /// [AI 생성] 번호 모양 종류
    ///
    /// 원본: `type` 속성
    #[serde(rename = "@type", default)]
    pub number_type: NumberFormatType2,

    /// [AI 생성] 사용자 기호
    ///
    /// 원본: `userChar` 속성
    #[serde(rename = "@userChar", skip_serializing_if = "Option::is_none")]
    pub user_character: Option<String>,

    /// [AI 생성] 앞 장식 문자
    ///
    /// 원본: `prefixChar` 속성
    #[serde(rename = "@prefixChar", skip_serializing_if = "Option::is_none")]
    pub prefix_character: Option<String>,

    /// [AI 생성] 뒤 장식 문자
    ///
    /// 원본: `suffixChar` 속성
    #[serde(rename = "@suffixChar", default = "default_suffix")]
    pub suffix_character: String,

    /// [AI 생성] 위첨자 형식 여부
    ///
    /// 원본: `supscript` 속성
    #[serde(rename = "@supscript", default)]
    pub superscript: bool,
}

fn default_suffix() -> String {
    ")".to_string()
}

/// [AI 생성] 주석 구분선
///
/// 원본: `noteLine` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "noteLine")]
pub struct NoteLine {
    /// [AI 생성] 구분선 길이
    ///
    /// 0 (구분선 없음), -1 (5 cm), -2 (2 cm), -3 (단 크기의 1/3), -4 (단 크기), 그 외 (HWPUNIT 단위의 사용자 지정 길이)
    ///
    /// 원본: `length` 속성
    #[serde(rename = "@length", default)]
    pub length: i32,

    /// [AI 생성] 구분선 종류
    ///
    /// 원본: `type` 속성
    #[serde(rename = "@type", default)]
    pub line_type: LineStyleType2,

    /// [AI 생성] 구분선 굵기 (mm 단위)
    ///
    /// 원본: `width` 속성
    #[serde(rename = "@width", default = "default_note_line_width")]
    pub width: LineWidth,

    /// [AI 생성] 구분선 색
    ///
    /// 원본: `color` 속성
    #[serde(rename = "@color", default = "RgbColor::black")]
    pub color: RgbColor,
}

fn default_note_line_width() -> LineWidth {
    LineWidth::Mm0_12
}

/// [AI 생성] 주석 간격
///
/// 원본: `noteSpacing` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "noteSpacing")]
pub struct NoteSpacing {
    /// [AI 생성] 주석 사이 여백
    ///
    /// 원본: `betweenNotes` 속성
    #[serde(rename = "@betweenNotes", default = "default_note_spacing")]
    pub between_notes: u32,

    /// [AI 생성] 구분선 아래 여백
    ///
    /// 원본: `belowLine` 속성
    #[serde(rename = "@belowLine", default = "default_line_spacing")]
    pub below_line: u32,

    /// [AI 생성] 구분선 위 여백
    ///
    /// 원본: `aboveLine` 속성
    #[serde(rename = "@aboveLine", default = "default_line_spacing")]
    pub above_line: u32,
}

fn default_note_spacing() -> u32 {
    850
}

fn default_line_spacing() -> u32 {
    567
}

impl Default for NoteSpacing {
    fn default() -> Self {
        Self {
            between_notes: 850,
            below_line: 567,
            above_line: 567,
        }
    }
}

/// [AI 생성] 각주 번호 매기기
///
/// 원본: `FootNoteShapeType/numbering` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "numbering")]
pub struct FootnoteNumbering {
    /// [AI 생성] 번호 매기기 형식
    ///
    /// 원본: `type` 속성
    #[serde(rename = "@type", default)]
    pub numbering_type: FootnoteNumberingType,

    /// [AI 생성] 시작 번호 (ON_SECTION일 때만 사용)
    ///
    /// 원본: `newNum` 속성
    #[serde(rename = "@newNum", default = "default_one")]
    pub new_number: u32,
}

fn default_one() -> u32 {
    1
}

/// [AI 생성] 각주 배치
///
/// 원본: `FootNoteShapeType/placement` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "placement")]
pub struct FootnotePlacementSettings {
    /// [AI 생성] 배치 위치
    ///
    /// 원본: `place` 속성
    #[serde(rename = "@place", default)]
    pub place: FootnotePlacement,

    /// [AI 생성] 텍스트에 이어 바로 출력할지 여부
    ///
    /// 원본: `beneathText` 속성
    #[serde(rename = "@beneathText", default)]
    pub beneath_text: bool,
}

/// [AI 생성] 각주 모양
///
/// 원본: `FootNoteShapeType`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "footNotePr")]
pub struct FootnoteShape {
    /// [AI 생성] 자동 번호 형식
    ///
    /// 원본: `autoNumFormat` 요소
    #[serde(rename = "autoNumFormat")]
    pub auto_number_format: AutoNumberFormat,

    /// [AI 생성] 구분선
    ///
    /// 원본: `noteLine` 요소
    #[serde(rename = "noteLine")]
    pub note_line: NoteLine,

    /// [AI 생성] 간격
    ///
    /// 원본: `noteSpacing` 요소
    #[serde(rename = "noteSpacing")]
    pub note_spacing: NoteSpacing,

    /// [AI 생성] 번호 매기기
    ///
    /// 원본: `numbering` 요소
    #[serde(rename = "numbering")]
    pub numbering: FootnoteNumbering,

    /// [AI 생성] 배치
    ///
    /// 원본: `placement` 요소
    #[serde(rename = "placement")]
    pub placement: FootnotePlacementSettings,
}

/// [AI 생성] 미주 번호 매기기
///
/// 원본: `EndNoteShapeType/numbering` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "numbering")]
pub struct EndnoteNumbering {
    /// [AI 생성] 번호 매기기 형식
    ///
    /// 원본: `type` 속성
    #[serde(rename = "@type", default)]
    pub numbering_type: EndnoteNumberingType,

    /// [AI 생성] 시작 번호 (ON_SECTION일 때만 사용)
    ///
    /// 원본: `newNum` 속성
    #[serde(rename = "@newNum", default = "default_one")]
    pub new_number: u32,
}

/// [AI 생성] 미주 배치
///
/// 원본: `EndNoteShapeType/placement` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "placement")]
pub struct EndnotePlacementSettings {
    /// [AI 생성] 배치 위치
    ///
    /// 원본: `place` 속성
    #[serde(rename = "@place", default)]
    pub place: EndnotePlacement,

    /// [AI 생성] 텍스트에 이어 바로 출력할지 여부
    ///
    /// 원본: `beneathText` 속성
    #[serde(rename = "@beneathText", default)]
    pub beneath_text: bool,
}

/// [AI 생성] 미주 모양
///
/// 원본: `EndNoteShapeType`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "endNotePr")]
pub struct EndnoteShape {
    /// [AI 생성] 자동 번호 형식
    ///
    /// 원본: `autoNumFormat` 요소
    #[serde(rename = "autoNumFormat")]
    pub auto_number_format: AutoNumberFormat,

    /// [AI 생성] 구분선
    ///
    /// 원본: `noteLine` 요소
    #[serde(rename = "noteLine")]
    pub note_line: NoteLine,

    /// [AI 생성] 간격
    ///
    /// 원본: `noteSpacing` 요소
    #[serde(rename = "noteSpacing")]
    pub note_spacing: NoteSpacing,

    /// [AI 생성] 번호 매기기
    ///
    /// 원본: `numbering` 요소
    #[serde(rename = "numbering")]
    pub numbering: EndnoteNumbering,

    /// [AI 생성] 배치
    ///
    /// 원본: `placement` 요소
    #[serde(rename = "placement")]
    pub placement: EndnotePlacementSettings,
}

/// [AI 생성] 쪽 테두리/배경 오프셋
///
/// 원본: `pageBorderFill/offset` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "offset")]
pub struct PageBorderOffset {
    /// [AI 생성] 왼쪽 간격 (HWPUNIT)
    ///
    /// 원본: `left` 속성
    #[serde(rename = "@left", default = "default_border_offset")]
    pub left: u32,

    /// [AI 생성] 오른쪽 간격 (HWPUNIT)
    ///
    /// 원본: `right` 속성
    #[serde(rename = "@right", default = "default_border_offset")]
    pub right: u32,

    /// [AI 생성] 위쪽 간격 (HWPUNIT)
    ///
    /// 원본: `top` 속성
    #[serde(rename = "@top", default = "default_border_offset")]
    pub top: u32,

    /// [AI 생성] 아래쪽 간격 (HWPUNIT)
    ///
    /// 원본: `bottom` 속성
    #[serde(rename = "@bottom", default = "default_border_offset")]
    pub bottom: u32,
}

fn default_border_offset() -> u32 {
    1417
}

impl Default for PageBorderOffset {
    fn default() -> Self {
        Self {
            left: 1417,
            right: 1417,
            top: 1417,
            bottom: 1417,
        }
    }
}

/// [AI 생성] 쪽 테두리/배경 정보
///
/// 원본: `pageBorderFill` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "pageBorderFill")]
pub struct PageBorderFill {
    /// [AI 생성] 오프셋
    ///
    /// 원본: `offset` 요소
    #[serde(rename = "offset")]
    pub offset: PageBorderOffset,

    /// [AI 생성] 종류
    ///
    /// 원본: `type` 속성
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub page_type: Option<PageBorderType>,

    /// [AI 생성] 테두리/배경 아이디 참조 값
    ///
    /// 원본: `borderFillIDRef` 속성
    #[serde(rename = "@borderFillIDRef", skip_serializing_if = "Option::is_none")]
    pub border_fill_id_reference: Option<BorderFillIdRef>,

    /// [AI 생성] 쪽 테두리 위치 기준
    ///
    /// 원본: `textBorder` 속성
    #[serde(rename = "@textBorder", skip_serializing_if = "Option::is_none")]
    pub text_border: Option<PageBorderPosition>,

    /// [AI 생성] 머리말 포함 여부
    ///
    /// 원본: `headerInside` 속성
    #[serde(rename = "@headerInside", default)]
    pub header_inside: bool,

    /// [AI 생성] 꼬리말 포함 여부
    ///
    /// 원본: `footerInside` 속성
    #[serde(rename = "@footerInside", default)]
    pub footer_inside: bool,

    /// [AI 생성] 채울 영역
    ///
    /// 원본: `fillArea` 속성
    #[serde(rename = "@fillArea", skip_serializing_if = "Option::is_none")]
    pub fill_area: Option<FillAreaType>,
}

/// [AI 생성] 바탕쪽 참조
///
/// 원본: `masterPage` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "masterPage")]
pub struct MasterPageReference {
    /// [AI 생성] 바탕쪽 아이디 참조
    ///
    /// 원본: `idRef` 속성
    #[serde(rename = "@idRef")]
    pub id_reference: MasterPageIdRef,
}

/// [AI 생성] 프레젠테이션 정보
///
/// 원본: `presentation` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "presentation")]
pub struct Presentation {
    /// [AI 생성] 채우기 정보
    ///
    /// 원본: `fillBrush` 요소
    #[serde(rename = "fillBrush", skip_serializing_if = "Option::is_none")]
    pub fill_brush: Option<FillBrush>,

    /// [AI 생성] 화면 전환 효과
    ///
    /// 원본: `effect` 속성
    #[serde(rename = "@effect", skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,

    /// [AI 생성] 소리 아이디 참조
    ///
    /// 원본: `soundIDRef` 속성
    #[serde(rename = "@soundIDRef", skip_serializing_if = "Option::is_none")]
    pub sound_id_reference: Option<SoundIdRef>,

    /// [AI 생성] 텍스트 반전
    ///
    /// 원본: `invertText` 속성
    #[serde(rename = "@invertText", default)]
    pub invert_text: bool,

    /// [AI 생성] 자동 표시
    ///
    /// 원본: `autoshow` 속성
    #[serde(rename = "@autoshow", default)]
    pub auto_show: bool,

    /// [AI 생성] 표시 시간
    ///
    /// 원본: `showtime` 속성
    #[serde(rename = "@showtime", skip_serializing_if = "Option::is_none")]
    pub show_time: Option<u32>,

    /// [AI 생성] 적용 범위
    ///
    /// 원본: `applyto` 속성
    #[serde(rename = "@applyto", skip_serializing_if = "Option::is_none")]
    pub apply_to: Option<String>,
}

/// [AI 생성] 구역 정의
///
/// 원본: `SectionDefinitionType`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "secPr")]
pub struct SectionDefinition {
    /// [AI 생성] 시작 번호 정보
    ///
    /// 원본: `startNum` 요소
    #[serde(rename = "startNum", skip_serializing_if = "Option::is_none")]
    pub start_number: Option<SectionStartNumber>,

    /// [AI 생성] 줄맞춤 정보
    ///
    /// 원본: `grid` 요소
    #[serde(rename = "grid", skip_serializing_if = "Option::is_none")]
    pub grid: Option<SectionGrid>,

    /// [AI 생성] 감추기/보여주기 정보
    ///
    /// 원본: `visibility` 요소
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<SectionVisibility>,

    /// [AI 생성] 줄 번호 정보
    ///
    /// 원본: `lineNumberShape` 요소
    #[serde(rename = "lineNumberShape", skip_serializing_if = "Option::is_none")]
    pub line_number_shape: Option<LineNumberShape>,

    /// [AI 생성] 용지 설정
    ///
    /// 원본: `pagePr` 요소
    #[serde(rename = "pagePr", skip_serializing_if = "Option::is_none")]
    pub page_property: Option<PageProperty>,

    /// [AI 생성] 각주 모양
    ///
    /// 원본: `footNotePr` 요소
    #[serde(rename = "footNotePr", skip_serializing_if = "Option::is_none")]
    pub footnote_shape: Option<FootnoteShape>,

    /// [AI 생성] 미주 모양
    ///
    /// 원본: `endNotePr` 요소
    #[serde(rename = "endNotePr", skip_serializing_if = "Option::is_none")]
    pub endnote_shape: Option<EndnoteShape>,

    /// [AI 생성] 쪽 테두리/배경 정보
    ///
    /// 원본: `pageBorderFill` 요소
    #[serde(
        rename = "pageBorderFill",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub page_border_fills: Vec<PageBorderFill>,

    /// [AI 생성] 바탕쪽 정보
    ///
    /// 원본: `masterPage` 요소
    #[serde(rename = "masterPage", default, skip_serializing_if = "Vec::is_empty")]
    pub master_pages: Vec<MasterPageReference>,

    /// [AI 생성] 프레젠테이션 정보
    ///
    /// 원본: `presentation` 요소
    #[serde(rename = "presentation", skip_serializing_if = "Option::is_none")]
    pub presentation: Option<Presentation>,

    /// [AI 생성] 구역 아이디
    ///
    /// 원본: `id` 속성
    #[serde(rename = "@id")]
    pub id: String,

    /// [AI 생성] 텍스트 방향
    ///
    /// 원본: `textDirection` 속성
    #[serde(rename = "@textDirection", default)]
    pub text_direction: TextDirection,

    /// [AI 생성] 동일한 페이지에서 서로 다른 단 사이의 간격
    ///
    /// 원본: `spaceColumns` 속성
    #[serde(rename = "@spaceColumns", default)]
    pub space_columns: i32,

    /// [AI 생성] 기본 탭 간격
    ///
    /// 원본: `tabStopVal` 속성
    #[serde(rename = "@tabStopVal", default)]
    pub tab_stop_value: i32,

    /// [AI 생성] 기본 탭 간격 단위
    ///
    /// 원본: `tabStopUnit` 속성
    #[serde(rename = "@tabStopUnit", default)]
    pub tab_stop_unit: TabStopUnit,

    /// [AI 생성] 개요 번호 모양 아이디 참조 값
    ///
    /// 원본: `outlineShapeIDRef` 속성
    #[serde(rename = "@outlineShapeIDRef", skip_serializing_if = "Option::is_none")]
    pub outline_shape_id_reference: Option<OutlineShapeIdRef>,

    /// [AI 생성] 메모 모양 아이디 참조 값
    ///
    /// 원본: `memoShapeIDRef` 속성
    #[serde(rename = "@memoShapeIDRef", skip_serializing_if = "Option::is_none")]
    pub memo_shape_id_reference: Option<MemoShapeIdRef>,

    /// [AI 생성] 머리말/꼬리말 세로 쓰기 여부
    ///
    /// 원본: `textVerticalWidthHead` 속성
    #[serde(rename = "@textVerticalWidthHead", default)]
    pub text_vertical_width_head: bool,

    /// [AI 생성] 확장 바탕쪽 개수
    ///
    /// 원본: `masterPageCnt` 속성
    #[serde(rename = "@masterPageCnt", default)]
    pub master_page_count: u32,
}
