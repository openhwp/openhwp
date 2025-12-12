//! [AI 생성] 컨트롤 요소 (필드/머리말/각주 등)
//!
//! 런 내부에서 문서 구조를 제어하는 개체들입니다. 필드/책갈피/머리말·꼬리말/각주·미주/페이지 번호와 자동 번호가 포함되며, 본문과 별개로 서브 문단 목록을 가질 수 있습니다. KS X 6101:2024 `paralist.xsd`.

use serde::{Deserialize, Serialize};

use super::column::ColumnDefinition;
use super::enums::{FieldType, PageNumberPosition, PageStartsOn};
use super::para_list::ParagraphList;
use super::section_definition::AutoNumberFormat;
use crate::core::{
    enums::NumberFormatType1,
    types::{BeginIdRef, MetaTag},
};

/// [AI 생성] 필드 시작
///
/// 원본: `fieldBegin` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "fieldBegin")]
pub struct FieldBegin {
    /// [AI 생성] 매개변수 목록 (`parameters` 요소)
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterList>,

    /// [AI 생성] 서브 리스트 (`subList` 요소)
    #[serde(rename = "subList", skip_serializing_if = "Option::is_none")]
    pub sub_list: Option<ParagraphList>,

    /// [AI 생성] 메타 태그 (`metaTag` 요소)
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    /// [AI 생성] 아이디 (`id` 속성)
    #[serde(rename = "@id")]
    pub id: u32,

    /// [AI 생성] 필드 유형 (`type` 속성)
    #[serde(rename = "@type")]
    pub field_type: FieldType,

    /// [AI 생성] 이름 (`name` 속성)
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// [AI 생성] 편집 가능 여부 (`editable` 속성)
    #[serde(rename = "@editable", default = "default_true")]
    pub editable: bool,

    /// [AI 생성] 변경 여부 (`dirty` 속성)
    #[serde(rename = "@dirty", default)]
    pub dirty: bool,

    /// [AI 생성] Z 순서 (`zorder` 속성)
    #[serde(rename = "@zorder", skip_serializing_if = "Option::is_none")]
    pub z_order: Option<i32>,

    /// [AI 생성] 필드 아이디 (`fieldid` 속성)
    #[serde(rename = "@fieldid", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<u32>,
}

fn default_true() -> bool {
    true
}

/// [AI 생성] 필드 종료
///
/// 원본: `fieldEnd` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "fieldEnd")]
pub struct FieldEnd {
    /// [AI 생성] 시작 아이디 참조 (`beginIDRef` 속성)
    #[serde(rename = "@beginIDRef")]
    pub begin_id_reference: BeginIdRef,

    /// [AI 생성] 필드 아이디 (`fieldid` 속성)
    #[serde(rename = "@fieldid", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<u32>,
}

/// [AI 생성] 책갈피
///
/// 원본: `bookmark` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "bookmark")]
pub struct Bookmark {
    /// [AI 생성] 이름 (`name` 속성)
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// [AI 생성] 머리말/꼬리말 유형
///
/// 원본: `HeaderFooterType`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeaderFooter {
    /// [AI 생성] 서브 리스트 (`subList` 요소)
    #[serde(rename = "subList")]
    pub sub_list: ParagraphList,

    /// [AI 생성] 아이디 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// [AI 생성] 적용 페이지 유형 (`applyPageType` 속성)
    #[serde(rename = "@applyPageType", default)]
    pub apply_page_type: PageStartsOn,
}

/// [AI 생성] 각주/미주 유형
///
/// 원본: `NoteType`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    /// [AI 생성] 서브 리스트 (`subList` 요소)
    #[serde(rename = "subList")]
    pub sub_list: ParagraphList,

    /// [AI 생성] 인스턴스 아이디 (`instId` 속성)
    #[serde(rename = "@instId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<u32>,
}

/// [AI 생성] 자동 번호 유형
///
/// 원본: `AutoNumNewNumType.numType` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AutoNumberKind {
    /// 페이지
    #[default]
    #[serde(rename = "PAGE")]
    Page,
    /// 각주
    #[serde(rename = "FOOTNOTE")]
    Footnote,
    /// 미주
    #[serde(rename = "ENDNOTE")]
    Endnote,
    /// 그림
    #[serde(rename = "PICTURE")]
    Picture,
    /// 표
    #[serde(rename = "TABLE")]
    Table,
    /// 수식
    #[serde(rename = "EQUATION")]
    Equation,
    /// 전체 페이지
    #[serde(rename = "TOTAL_PAGE")]
    TotalPage,
}

/// [AI 생성] 자동 번호/새 번호 유형
///
/// 원본: `AutoNumNewNumType`
///
/// 참고: `autoNum`은 `autoNumFormat` 자식 요소를 가지지만,
/// `newNum`은 self-closing이고 속성만 가집니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoNumberNewNumber {
    /// [AI 생성] 자동 번호 형식 (`autoNumFormat` 요소, `newNum`에서는 생략 가능)
    #[serde(rename = "autoNumFormat", skip_serializing_if = "Option::is_none")]
    pub auto_number_format: Option<AutoNumberFormat>,

    /// [AI 생성] 번호 (`num` 속성)
    #[serde(rename = "@num", default = "default_one")]
    pub number: i32,

    /// [AI 생성] 번호 유형 (`numType` 속성)
    #[serde(rename = "@numType", skip_serializing_if = "Option::is_none")]
    pub number_type: Option<AutoNumberKind>,
}

fn default_one() -> i32 {
    1
}

/// [AI 생성] 페이지 번호 컨트롤
///
/// 원본: `pageNumCtrl` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "pageNumCtrl")]
pub struct PageNumberControl {
    /// [AI 생성] 페이지 시작 (`pageStartsOn` 속성)
    #[serde(rename = "@pageStartsOn", default)]
    pub page_starts_on: PageStartsOn,
}

/// [AI 생성] 페이지 숨기기
///
/// 원본: `pageHiding` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "pageHiding")]
pub struct PageHiding {
    /// [AI 생성] 머리말 숨기기 (`hideHeader` 속성)
    #[serde(rename = "@hideHeader", default)]
    pub hide_header: bool,

    /// [AI 생성] 꼬리말 숨기기 (`hideFooter` 속성)
    #[serde(rename = "@hideFooter", default)]
    pub hide_footer: bool,

    /// [AI 생성] 바탕쪽 숨기기 (`hideMasterPage` 속성)
    #[serde(rename = "@hideMasterPage", default)]
    pub hide_master_page: bool,

    /// [AI 생성] 테두리 숨기기 (`hideBorder` 속성)
    #[serde(rename = "@hideBorder", default)]
    pub hide_border: bool,

    /// [AI 생성] 채우기 숨기기 (`hideFill` 속성)
    #[serde(rename = "@hideFill", default)]
    pub hide_fill: bool,

    /// [AI 생성] 페이지 번호 숨기기 (`hidePageNum` 속성)
    #[serde(rename = "@hidePageNum", default)]
    pub hide_page_number: bool,
}

/// [AI 생성] 페이지 번호
///
/// 원본: `pageNum` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "pageNum")]
pub struct PageNumber {
    /// [AI 생성] 번호 위치 (`pos` 속성)
    #[serde(rename = "@pos", default)]
    pub position: PageNumberPosition,

    /// [AI 생성] 번호 모양 (`formatType` 속성)
    #[serde(rename = "@formatType", default)]
    pub format_type: NumberFormatType1,

    /// [AI 생성] 줄표 넣기 기능 (`sideChar` 속성)
    #[serde(rename = "@sideChar", default = "default_side_char")]
    pub side_character: String,
}

fn default_side_char() -> String {
    "-".to_string()
}

/// [AI 생성] 색인 표시
///
/// 원본: `indexmark` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "indexmark")]
pub struct IndexMark {
    /// [AI 생성] 첫 번째 키 (`firstKey` 요소)
    #[serde(rename = "firstKey", default)]
    pub first_key: String,

    /// [AI 생성] 두 번째 키 (`secondKey` 요소)
    #[serde(rename = "secondKey", default)]
    pub second_key: String,
}

/// [AI 생성] 숨은 주석
///
/// 원본: `hiddenComment` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hiddenComment")]
pub struct HiddenComment {
    /// [AI 생성] 서브 리스트 (`subList` 요소)
    #[serde(rename = "subList")]
    pub sub_list: ParagraphList,
}

/// [AI 생성] 매개변수 목록
///
/// 원본: `ParameterList`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "parameters")]
pub struct ParameterList {
    /// [AI 생성] 매개변수 항목들
    #[serde(rename = "$value", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ParameterItem>,

    /// [AI 생성] 개수 (`cnt` 속성)
    #[serde(rename = "@cnt")]
    pub count: u32,

    /// [AI 생성] 이름 (`name` 속성)
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// [AI 생성] 매개변수 항목
///
/// 원본: `ParameterList` 내 choice 요소들
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParameterItem {
    /// [AI 생성] 불리언 매개변수
    #[serde(rename = "booleanParam")]
    Boolean(BooleanParameter),
    /// [AI 생성] 정수 매개변수
    #[serde(rename = "integerParam")]
    Integer(IntegerParameter),
    /// [AI 생성] 실수 매개변수
    #[serde(rename = "floatParam")]
    Float(FloatParameter),
    /// [AI 생성] 문자열 매개변수
    #[serde(rename = "stringParam")]
    String(StringParameter),
    /// [AI 생성] 리스트 매개변수
    #[serde(rename = "listParam")]
    List(Box<ParameterList>),
}

/// [AI 생성] 불리언 매개변수
///
/// 원본: `booleanParam` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "booleanParam")]
pub struct BooleanParameter {
    /// [AI 생성] 값
    #[serde(rename = "$text")]
    pub value: bool,

    /// [AI 생성] 이름
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// [AI 생성] 정수 매개변수
///
/// 원본: `integerParam` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "integerParam")]
pub struct IntegerParameter {
    /// [AI 생성] 값
    #[serde(rename = "$text")]
    pub value: i64,

    /// [AI 생성] 이름
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// [AI 생성] 실수 매개변수
///
/// 원본: `floatParam` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "floatParam")]
pub struct FloatParameter {
    /// [AI 생성] 값
    #[serde(rename = "$text")]
    pub value: f32,

    /// [AI 생성] 이름
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// [AI 생성] 문자열 매개변수
///
/// 원본: `stringParam` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "stringParam")]
pub struct StringParameter {
    /// [AI 생성] 값
    #[serde(rename = "$text")]
    pub value: String,

    /// [AI 생성] 이름
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// [AI 생성] 변경 추적 태그
///
/// 원본: `TrackChangeTag`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TrackChangeTag {
    /// [AI 생성] 문단 끝 (`paraend` 속성)
    #[serde(rename = "@paraend", skip_serializing_if = "Option::is_none")]
    pub paragraph_end: Option<bool>,

    /// [AI 생성] 변경 추적 아이디 (`TcId` 속성)
    #[serde(rename = "@TcId", skip_serializing_if = "Option::is_none")]
    pub track_change_id: Option<u32>,

    /// [AI 생성] 아이디 (`Id` 속성)
    #[serde(rename = "@Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

/// [AI 생성] 컨트롤
///
/// 원본: `ctrl` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ctrl")]
pub struct Control {
    /// [AI 생성] 컨트롤 내용
    #[serde(rename = "$value", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ControlItem>,
}

/// [AI 생성] 컨트롤 항목
///
/// 원본: `ctrl` 요소 내 choice 요소들
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ControlItem {
    /// [AI 생성] 단 정의
    #[serde(rename = "colPr")]
    ColumnDefinition(ColumnDefinition),
    /// [AI 생성] 필드 시작
    #[serde(rename = "fieldBegin")]
    FieldBegin(FieldBegin),
    /// [AI 생성] 필드 종료
    #[serde(rename = "fieldEnd")]
    FieldEnd(FieldEnd),
    /// [AI 생성] 책갈피
    #[serde(rename = "bookmark")]
    Bookmark(Bookmark),
    /// [AI 생성] 머리말
    #[serde(rename = "header")]
    Header(HeaderFooter),
    /// [AI 생성] 꼬리말
    #[serde(rename = "footer")]
    Footer(HeaderFooter),
    /// [AI 생성] 각주
    #[serde(rename = "footNote")]
    Footnote(Note),
    /// [AI 생성] 미주
    #[serde(rename = "endNote")]
    Endnote(Note),
    /// [AI 생성] 자동 번호
    #[serde(rename = "autoNum")]
    AutoNumber(AutoNumberNewNumber),
    /// [AI 생성] 새 번호
    #[serde(rename = "newNum")]
    NewNumber(AutoNumberNewNumber),
    /// [AI 생성] 페이지 번호 컨트롤
    #[serde(rename = "pageNumCtrl")]
    PageNumberControl(PageNumberControl),
    /// [AI 생성] 페이지 숨기기
    #[serde(rename = "pageHiding")]
    PageHiding(PageHiding),
    /// [AI 생성] 페이지 번호
    #[serde(rename = "pageNum")]
    PageNumber(PageNumber),
    /// [AI 생성] 색인 표시
    #[serde(rename = "indexmark")]
    IndexMark(IndexMark),
    /// [AI 생성] 숨은 주석
    #[serde(rename = "hiddenComment")]
    HiddenComment(HiddenComment),
}
