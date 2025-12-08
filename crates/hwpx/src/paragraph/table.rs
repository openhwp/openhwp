//! [AI 생성] 표 타입 (셀 병합/여백/셀존)
//!
//! 지면 흐름을 따라 이동하는 표 개체를 정의합니다. 셀 병합, 셀존 스타일, 캡션 위치, 텍스트 감싸기를 모두 포함해 페이지 나눔 시에도 구조가 유지되도록 설계되었습니다. KS X 6101:2024 `paralist.xsd` 기준.

use serde::{Deserialize, Serialize};

use super::enums::TablePageBreak;
use super::para_list::ParagraphList;
use super::shape_common::{
    CaptionSide, InsideMargin, OutsideMargin, ShapeNumberingType, ShapeObjectPosition,
    ShapeObjectSize, TextFlowMode, TextWrapMode,
};
use crate::core::types::{BorderFillIdRef, MetaTag};

/// [AI 생성] 표 내부 셀 영역
///
/// 원본: `cellzone` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "cellzone")]
pub struct CellZone {
    /// [AI 생성] 셀존 Row의 시작 주소 (`startRowAddr` 속성)
    #[serde(rename = "@startRowAddr", skip_serializing_if = "Option::is_none")]
    pub start_row_address: Option<u32>,

    /// [AI 생성] 셀존 Column의 시작 주소 (`startColAddr` 속성)
    #[serde(rename = "@startColAddr", skip_serializing_if = "Option::is_none")]
    pub start_column_address: Option<u32>,

    /// [AI 생성] 셀존 Row의 끝 주소 (`endRowAddr` 속성)
    #[serde(rename = "@endRowAddr", skip_serializing_if = "Option::is_none")]
    pub end_row_address: Option<u32>,

    /// [AI 생성] 셀존 Column의 끝 주소 (`endColAddr` 속성)
    #[serde(rename = "@endColAddr", skip_serializing_if = "Option::is_none")]
    pub end_column_address: Option<u32>,

    /// [AI 생성] 테두리/배경 아이디 참조 값 (`borderFillIDRef` 속성)
    #[serde(rename = "@borderFillIDRef", skip_serializing_if = "Option::is_none")]
    pub border_fill_id_ref: Option<BorderFillIdRef>,
}

/// [AI 생성] 셀존 목록
///
/// 원본: `cellzoneList` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "cellzoneList")]
pub struct CellZoneList {
    /// [AI 생성] 셀존 항목들 (`cellzone` 요소)
    #[serde(rename = "cellzone", default)]
    pub cell_zones: Vec<CellZone>,
}

/// [AI 생성] 셀 주소
///
/// 원본: `cellAddr` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "cellAddr")]
pub struct CellAddress {
    /// [AI 생성] 열 주소 (`colAddr` 속성)
    #[serde(rename = "@colAddr", skip_serializing_if = "Option::is_none")]
    pub column_address: Option<u32>,

    /// [AI 생성] 행 주소 (`rowAddr` 속성)
    #[serde(rename = "@rowAddr", skip_serializing_if = "Option::is_none")]
    pub row_address: Option<u32>,
}

/// [AI 생성] 셀 병합
///
/// 원본: `cellSpan` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "cellSpan")]
pub struct CellSpan {
    /// [AI 생성] 열 병합 수 (`colSpan` 속성)
    #[serde(rename = "@colSpan", default = "default_span")]
    pub column_span: u32,

    /// [AI 생성] 행 병합 수 (`rowSpan` 속성)
    #[serde(rename = "@rowSpan", default = "default_span")]
    pub row_span: u32,
}

fn default_span() -> u32 {
    1
}

impl Default for CellSpan {
    fn default() -> Self {
        Self {
            column_span: 1,
            row_span: 1,
        }
    }
}

/// [AI 생성] 셀 크기
///
/// 원본: `cellSz` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "cellSz")]
pub struct CellSize {
    /// [AI 생성] 셀 너비 (`width` 속성)
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// [AI 생성] 셀 높이 (`height` 속성)
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

/// [AI 생성] 셀 여백
///
/// 원본: `cellMargin` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "cellMargin")]
pub struct CellMargin {
    /// [AI 생성] 왼쪽 여백 (`left` 속성)
    #[serde(rename = "@left", default)]
    pub left: u32,

    /// [AI 생성] 오른쪽 여백 (`right` 속성)
    #[serde(rename = "@right", default)]
    pub right: u32,

    /// [AI 생성] 위쪽 여백 (`top` 속성)
    #[serde(rename = "@top", default)]
    pub top: u32,

    /// [AI 생성] 아래쪽 여백 (`bottom` 속성)
    #[serde(rename = "@bottom", default)]
    pub bottom: u32,
}

/// [AI 생성] 표 셀
///
/// 원본: `tc` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "tc")]
pub struct TableCell {
    /// [AI 생성] 문단 목록 (`subList` 요소)
    #[serde(rename = "subList")]
    pub paragraph_list: ParagraphList,

    /// [AI 생성] 셀 주소 (`cellAddr` 요소)
    #[serde(rename = "cellAddr")]
    pub cell_address: CellAddress,

    /// [AI 생성] 셀 병합 (`cellSpan` 요소)
    #[serde(rename = "cellSpan")]
    pub cell_span: CellSpan,

    /// [AI 생성] 셀 크기 (`cellSz` 요소)
    #[serde(rename = "cellSz")]
    pub cell_size: CellSize,

    /// [AI 생성] 셀 여백 (`cellMargin` 요소)
    #[serde(rename = "cellMargin")]
    pub cell_margin: CellMargin,

    /// [AI 생성] 셀 이름 (`name` 속성)
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// [AI 생성] 머리글 셀 여부 (`header` 속성)
    #[serde(rename = "@header", default)]
    pub header: bool,

    /// [AI 생성] 여백 지정 여부 (`hasMargin` 속성)
    #[serde(rename = "@hasMargin", default)]
    pub has_margin: bool,

    /// [AI 생성] 셀 보호 여부 (`protect` 속성)
    #[serde(rename = "@protect", default)]
    pub protect: bool,

    /// [AI 생성] 편집 가능 여부 (`editable` 속성)
    #[serde(rename = "@editable", default)]
    pub editable: bool,

    /// [AI 생성] 변경됨 여부 (`dirty` 속성)
    #[serde(rename = "@dirty", default)]
    pub dirty: bool,

    /// [AI 생성] 테두리/배경 아이디 참조 값 (`borderFillIDRef` 속성)
    #[serde(rename = "@borderFillIDRef", skip_serializing_if = "Option::is_none")]
    pub border_fill_id_ref: Option<BorderFillIdRef>,
}

/// [AI 생성] 표 행
///
/// 원본: `tr` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "tr")]
pub struct TableRow {
    /// [AI 생성] 셀 목록 (`tc` 요소)
    #[serde(rename = "tc")]
    pub cells: Vec<TableCell>,
}

/// [AI 생성] 라벨 정보
///
/// 원본: `label` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "label")]
pub struct TableLabel {
    /// [AI 생성] 상단 여백 (`topmargin` 속성)
    #[serde(rename = "@topmargin", skip_serializing_if = "Option::is_none")]
    pub top_margin: Option<u32>,

    /// 왼쪽 여백
    ///
    /// 원본: `leftmargin` 속성
    #[serde(rename = "@leftmargin", skip_serializing_if = "Option::is_none")]
    pub left_margin: Option<u32>,

    /// 박스 너비
    ///
    /// 원본: `boxwidth` 속성
    #[serde(rename = "@boxwidth", skip_serializing_if = "Option::is_none")]
    pub box_width: Option<u32>,

    /// 박스 길이
    ///
    /// 원본: `boxlength` 속성
    #[serde(rename = "@boxlength", skip_serializing_if = "Option::is_none")]
    pub box_length: Option<u32>,

    /// 박스 가로 여백
    ///
    /// 원본: `boxmarginhor` 속성
    #[serde(rename = "@boxmarginhor", skip_serializing_if = "Option::is_none")]
    pub box_margin_horizontal: Option<u32>,

    /// 박스 세로 여백
    ///
    /// 원본: `boxmarginver` 속성
    #[serde(rename = "@boxmarginver", skip_serializing_if = "Option::is_none")]
    pub box_margin_vertical: Option<u32>,

    /// 라벨 열 수
    ///
    /// 원본: `labelcols` 속성
    #[serde(rename = "@labelcols", skip_serializing_if = "Option::is_none")]
    pub label_columns: Option<u32>,

    /// 라벨 행 수
    ///
    /// 원본: `labelrows` 속성
    #[serde(rename = "@labelrows", skip_serializing_if = "Option::is_none")]
    pub label_rows: Option<u32>,

    /// [AI 생성] 용지 방향 (`landscape` 속성)
    #[serde(rename = "@landscape", skip_serializing_if = "Option::is_none")]
    pub landscape: Option<LabelPaperOrientation>,

    /// [AI 생성] 페이지 너비 (`pagewidth` 속성)
    #[serde(rename = "@pagewidth", skip_serializing_if = "Option::is_none")]
    pub page_width: Option<u32>,

    /// [AI 생성] 페이지 높이 (`pageheight` 속성)
    #[serde(rename = "@pageheight", skip_serializing_if = "Option::is_none")]
    pub page_height: Option<u32>,
}

/// [AI 생성] 라벨 용지 방향
///
/// 원본: `label.landscape` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LabelPaperOrientation {
    /// 가로
    #[serde(rename = "WIDELY")]
    Widely,
    /// 세로
    #[default]
    #[serde(rename = "NARROWLY")]
    Narrowly,
}

/// [AI 생성] 캡션
///
/// 원본: `AbstractShapeObjectType/caption` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "caption")]
pub struct Caption {
    /// [AI 생성] 문단 목록 (`subList` 요소)
    #[serde(rename = "subList")]
    pub paragraph_list: ParagraphList,

    /// [AI 생성] 캡션 위치 (`side` 속성)
    #[serde(rename = "@side", default)]
    pub side: CaptionSide,

    /// [AI 생성] 전체 크기 여부 (`fullSz` 속성)
    #[serde(rename = "@fullSz", default)]
    pub full_size: bool,

    /// [AI 생성] 너비 (`width` 속성)
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,

    /// [AI 생성] 개체와의 간격 (`gap` 속성)
    #[serde(rename = "@gap", default = "default_caption_gap")]
    pub gap: i32,

    /// [AI 생성] 마지막 너비 (`lastWidth` 속성)
    #[serde(rename = "@lastWidth", skip_serializing_if = "Option::is_none")]
    pub last_width: Option<u32>,
}

fn default_caption_gap() -> i32 {
    850
}

/// [AI 생성] 표
///
/// AbstractShapeObjectType을 확장한 표 타입
///
/// 원본: `TableType` (`tbl` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "tbl")]
pub struct Table {
    // AbstractShapeObjectType 속성들
    /// [AI 생성] 크기 (`sz` 요소)
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,

    /// [AI 생성] 위치 (`pos` 요소)
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,

    /// [AI 생성] 바깥 여백 (`outMargin` 요소)
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,

    /// [AI 생성] 캡션 (`caption` 요소)
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,

    /// [AI 생성] 도형 주석 (`shapeComment` 요소)
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,

    /// [AI 생성] 메타 태그 (`metaTag` 요소)
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    // TableType 전용 요소들
    /// [AI 생성] 안쪽 여백 (`inMargin` 요소)
    #[serde(rename = "inMargin", skip_serializing_if = "Option::is_none")]
    pub inside_margin: Option<InsideMargin>,

    /// [AI 생성] 셀존 목록 (`cellzoneList` 요소)
    #[serde(rename = "cellzoneList", skip_serializing_if = "Option::is_none")]
    pub cell_zone_list: Option<CellZoneList>,

    /// [AI 생성] 행 목록 (`tr` 요소)
    #[serde(rename = "tr", default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<TableRow>,

    /// [AI 생성] 라벨 정보 (`label` 요소)
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<TableLabel>,

    // AbstractShapeObjectType 속성들
    /// [AI 생성] 아이디 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// [AI 생성] Z 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,

    /// [AI 생성] 번호 매기기 종류 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,

    /// [AI 생성] 텍스트 배치 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,

    /// [AI 생성] 텍스트 흐름 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,

    /// [AI 생성] 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,

    // TableType 전용 속성들
    /// [AI 생성] 쪽 나눔 설정 (`pageBreak` 속성)
    #[serde(rename = "@pageBreak", default)]
    pub page_break: TablePageBreak,

    /// [AI 생성] 머리글 반복 여부 (`repeatHeader` 속성)
    #[serde(rename = "@repeatHeader", default)]
    pub repeat_header: bool,

    /// [AI 생성] 자동 조정 안함 여부 (`noAdjust` 속성)
    #[serde(rename = "@noAdjust", default)]
    pub no_adjust: bool,

    /// [AI 생성] 행 수 (`rowCnt` 속성)
    #[serde(rename = "@rowCnt", skip_serializing_if = "Option::is_none")]
    pub row_count: Option<u32>,

    /// [AI 생성] 열 수 (`colCnt` 속성)
    #[serde(rename = "@colCnt", skip_serializing_if = "Option::is_none")]
    pub column_count: Option<u32>,

    /// [AI 생성] 셀 간격 (`cellSpacing` 속성)
    #[serde(rename = "@cellSpacing", default)]
    pub cell_spacing: u32,

    /// [AI 생성] 테두리/배경 아이디 참조 값 (`borderFillIDRef` 속성)
    #[serde(rename = "@borderFillIDRef", skip_serializing_if = "Option::is_none")]
    pub border_fill_id_ref: Option<BorderFillIdRef>,
}
