//! [AI 생성] 양식 컨트롤 타입
//!
//! KS X 6101:2024 - paralist.xsd

use serde::{Deserialize, Serialize};

use super::shape_common::{OutsideMargin, ShapeNumberingType, TextFlowMode, TextWrapMode};
use super::shape_common::{ShapeObjectPosition, ShapeObjectSize};
use super::table::Caption;
use crate::core::types::{BorderTypeIdRef, CharShapeIdRef, MetaTag, RgbColor};

/// [AI 생성] 양식 글자 속성
///
/// 원본: `AbstractFormObjectType/formCharPr` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "formCharPr")]
pub struct FormCharacterProperty {
    /// [AI 생성] 글자 속성 참조 값
    #[serde(rename = "@charPrIDRef", skip_serializing_if = "Option::is_none")]
    pub char_property_id_ref: Option<CharShapeIdRef>,

    /// [AI 생성] 문맥 따라가기 여부
    #[serde(rename = "@followContext", default)]
    pub follow_context: bool,

    /// [AI 생성] 자동 크기 여부
    #[serde(rename = "@autoSz", default)]
    pub auto_size: bool,

    /// [AI 생성] 자동 줄바꿈 여부
    #[serde(rename = "@wordWrap", default)]
    pub word_wrap: bool,
}

/// [AI 생성] 버튼 값
///
/// 원본: `AbstractButtonObjectType.value` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ButtonValue {
    #[default]
    /// [AI 생성] 선택 해제 상태
    #[serde(rename = "UNCHECKED")]
    Unchecked,
    /// [AI 생성] 선택됨 상태
    #[serde(rename = "CHECKED")]
    Checked,
    /// [AI 생성] 불확정(혼합) 상태
    #[serde(rename = "INDETERMINATE")]
    Indeterminate,
}

/// [AI 생성] 버튼 배경 스타일
///
/// 원본: `AbstractButtonObjectType.backStyle` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonBackStyle {
    /// [AI 생성] 배경 투명
    #[serde(rename = "TRANSPARENT")]
    Transparent,
    /// [AI 생성] 배경 불투명
    #[serde(rename = "OPAQUE")]
    Opaque,
}

/// [AI 생성] 버튼 (공통)
///
/// 원본: `AbstractButtonObjectType` (`btn`, `radioBtn`, `checkBtn` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Button {
    // AbstractShapeObjectType 요소들
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
    pub caption_element: Option<Caption>,
    /// [AI 생성] 도형 주석 (`shapeComment` 요소)
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// [AI 생성] 메타 태그 (`metaTag` 요소)
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    // AbstractFormObjectType 요소
    /// [AI 생성] 양식 글자 속성 (`formCharPr` 요소)
    #[serde(rename = "formCharPr")]
    pub form_char_property: FormCharacterProperty,

    // AbstractShapeObjectType 속성들
    /// [AI 생성] 객체 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,

    // AbstractFormObjectType 속성들
    /// [AI 생성] 컨트롤 이름 (`name` 속성)
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [AI 생성] 전경색 (`foreColor` 속성)
    #[serde(rename = "@foreColor", skip_serializing_if = "Option::is_none")]
    pub fore_color: Option<RgbColor>,
    /// [AI 생성] 배경색 (`backColor` 속성)
    #[serde(rename = "@backColor", skip_serializing_if = "Option::is_none")]
    pub back_color: Option<RgbColor>,
    /// [AI 생성] 그룹 이름 (`groupName` 속성)
    #[serde(rename = "@groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// [AI 생성] 탭 이동 허용 여부 (`tabStop` 속성)
    #[serde(rename = "@tabStop", default = "default_true")]
    pub tab_stop: bool,
    /// [AI 생성] 사용자 편집 가능 여부 (`editable` 속성)
    #[serde(rename = "@editable", default = "default_true")]
    pub editable: bool,
    /// [AI 생성] 탭 순서 (`tabOrder` 속성)
    #[serde(rename = "@tabOrder", skip_serializing_if = "Option::is_none")]
    pub tab_order: Option<i32>,
    /// [AI 생성] 사용 가능 여부 (`enabled` 속성)
    #[serde(rename = "@enabled", default = "default_true")]
    pub enabled: bool,
    /// [AI 생성] 테두리 타입 참조 (`borderTypeIDRef` 속성)
    #[serde(rename = "@borderTypeIDRef", skip_serializing_if = "Option::is_none")]
    pub border_type_id_ref: Option<BorderTypeIdRef>,
    /// [AI 생성] 프레임 출력 여부 (`drawFrame` 속성)
    #[serde(rename = "@drawFrame", default = "default_true")]
    pub draw_frame: bool,
    /// [AI 생성] 인쇄 여부 (`printable` 속성)
    #[serde(rename = "@printable", default = "default_true")]
    pub printable: bool,

    // AbstractButtonObjectType 속성들
    /// [AI 생성] 버튼 텍스트 (`caption` 속성)
    #[serde(rename = "@caption", skip_serializing_if = "Option::is_none")]
    pub caption_text: Option<String>,
    /// [AI 생성] 버튼 상태 값 (`value` 속성)
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<ButtonValue>,
    /// [AI 생성] 라디오 그룹 이름 (`radioGroupName` 속성)
    #[serde(rename = "@radioGroupName", skip_serializing_if = "Option::is_none")]
    pub radio_group_name: Option<String>,
    /// [AI 생성] 삼중 상태 사용 여부 (`triState` 속성)
    #[serde(rename = "@triState", default)]
    pub tri_state: bool,
    /// [AI 생성] 배경 스타일 (`backStyle` 속성)
    #[serde(rename = "@backStyle", skip_serializing_if = "Option::is_none")]
    pub back_style: Option<ButtonBackStyle>,
}

fn default_true() -> bool {
    true
}

/// [AI 생성] 목록 항목
///
/// 원본: `ListItemType`
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "listItem")]
pub struct ListItem {
    /// [AI 생성] 표시 텍스트
    #[serde(rename = "@displayText", skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,

    /// [AI 생성] 값
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// [AI 생성] 콤보박스 (드롭다운)
///
/// 원본: `ComboBoxType` (`comboBox` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "comboBox")]
pub struct ComboBox {
    // AbstractShapeObjectType 요소들
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

    // AbstractFormObjectType 요소
    /// [AI 생성] 양식 글자 속성 (`formCharPr` 요소)
    #[serde(rename = "formCharPr")]
    pub form_char_property: FormCharacterProperty,

    // ComboBoxType 전용 요소
    /// [AI 생성] 목록 항목들 (`listItem` 요소)
    #[serde(rename = "listItem", default)]
    pub list_items: Vec<ListItem>,

    // 속성들
    /// [AI 생성] 객체 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,

    // AbstractFormObjectType 속성들
    /// [AI 생성] 컨트롤 이름 (`name` 속성)
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [AI 생성] 전경색 (`foreColor` 속성)
    #[serde(rename = "@foreColor", skip_serializing_if = "Option::is_none")]
    pub fore_color: Option<RgbColor>,
    /// [AI 생성] 배경색 (`backColor` 속성)
    #[serde(rename = "@backColor", skip_serializing_if = "Option::is_none")]
    pub back_color: Option<RgbColor>,
    /// [AI 생성] 그룹 이름 (`groupName` 속성)
    #[serde(rename = "@groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// [AI 생성] 탭 이동 허용 여부 (`tabStop` 속성)
    #[serde(rename = "@tabStop", default = "default_true")]
    pub tab_stop: bool,
    /// [AI 생성] 편집 가능 여부 (`editable` 속성)
    #[serde(rename = "@editable", default = "default_true")]
    pub form_editable: bool,
    /// [AI 생성] 탭 순서 (`tabOrder` 속성)
    #[serde(rename = "@tabOrder", skip_serializing_if = "Option::is_none")]
    pub tab_order: Option<i32>,
    /// [AI 생성] 사용 가능 여부 (`enabled` 속성)
    #[serde(rename = "@enabled", default = "default_true")]
    pub enabled: bool,
    /// [AI 생성] 테두리 타입 참조 (`borderTypeIDRef` 속성)
    #[serde(rename = "@borderTypeIDRef", skip_serializing_if = "Option::is_none")]
    pub border_type_id_ref: Option<BorderTypeIdRef>,
    /// [AI 생성] 프레임 출력 여부 (`drawFrame` 속성)
    #[serde(rename = "@drawFrame", default = "default_true")]
    pub draw_frame: bool,
    /// [AI 생성] 인쇄 여부 (`printable` 속성)
    #[serde(rename = "@printable", default = "default_true")]
    pub printable: bool,

    // ComboBoxType 전용 속성들
    /// [AI 생성] 표시 행 수 (`listBoxRows`)
    #[serde(rename = "@listBoxRows", skip_serializing_if = "Option::is_none")]
    pub list_box_rows: Option<i32>,
    /// [AI 생성] 목록 폭 (`listBoxWidth`)
    #[serde(rename = "@listBoxWidth", skip_serializing_if = "Option::is_none")]
    pub list_box_width: Option<i32>,
    /// [AI 생성] 편집 가능 여부 (`editEnable`)
    #[serde(rename = "@editEnable", default)]
    pub edit_enable: bool,
    /// [AI 생성] 선택 값 (`selectedValue`)
    #[serde(rename = "@selectedValue", skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
}

/// [AI 생성] 목록 상자 (리스트박스)
///
/// 원본: `ListBoxType` (`listBox` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "listBox")]
pub struct ListBox {
    // AbstractShapeObjectType 요소들
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

    // AbstractFormObjectType 요소
    /// [AI 생성] 양식 글자 속성 (`formCharPr` 요소)
    #[serde(rename = "formCharPr")]
    pub form_char_property: FormCharacterProperty,

    // ListBoxType 전용 요소
    /// [AI 생성] 목록 항목들 (`listItem` 요소)
    #[serde(rename = "listItem", default)]
    pub list_items: Vec<ListItem>,

    // 속성들
    /// [AI 생성] 객체 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,

    // AbstractFormObjectType 속성들
    /// [AI 생성] 컨트롤 이름 (`name` 속성)
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [AI 생성] 전경색 (`foreColor` 속성)
    #[serde(rename = "@foreColor", skip_serializing_if = "Option::is_none")]
    pub fore_color: Option<RgbColor>,
    /// [AI 생성] 배경색 (`backColor` 속성)
    #[serde(rename = "@backColor", skip_serializing_if = "Option::is_none")]
    pub back_color: Option<RgbColor>,
    /// [AI 생성] 그룹 이름 (`groupName` 속성)
    #[serde(rename = "@groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// [AI 생성] 탭 이동 허용 여부 (`tabStop` 속성)
    #[serde(rename = "@tabStop", default = "default_true")]
    pub tab_stop: bool,
    /// [AI 생성] 편집 가능 여부 (`editable` 속성)
    #[serde(rename = "@editable", default = "default_true")]
    pub editable: bool,
    /// [AI 생성] 탭 순서 (`tabOrder` 속성)
    #[serde(rename = "@tabOrder", skip_serializing_if = "Option::is_none")]
    pub tab_order: Option<i32>,
    /// [AI 생성] 사용 가능 여부 (`enabled` 속성)
    #[serde(rename = "@enabled", default = "default_true")]
    pub enabled: bool,
    /// [AI 생성] 테두리 타입 참조 (`borderTypeIDRef` 속성)
    #[serde(rename = "@borderTypeIDRef", skip_serializing_if = "Option::is_none")]
    pub border_type_id_ref: Option<BorderTypeIdRef>,
    /// [AI 생성] 프레임 출력 여부 (`drawFrame` 속성)
    #[serde(rename = "@drawFrame", default = "default_true")]
    pub draw_frame: bool,
    /// [AI 생성] 인쇄 여부 (`printable` 속성)
    #[serde(rename = "@printable", default = "default_true")]
    pub printable: bool,

    // ListBoxType 전용 속성들
    /// [AI 생성] 항목 높이 (`itemHeight` 속성)
    #[serde(rename = "@itemHeight", skip_serializing_if = "Option::is_none")]
    pub item_height: Option<i32>,
    /// [AI 생성] 최상단 표시 인덱스 (`topIdx` 속성)
    #[serde(rename = "@topIdx", skip_serializing_if = "Option::is_none")]
    pub top_index: Option<u32>,
    /// [AI 생성] 선택 값 (`selectedValue` 속성)
    #[serde(rename = "@selectedValue", skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
}

/// 편집 상자 스크롤바 종류
///
/// 원본: `EditType.scrollBars` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EditScrollBars {
    #[default]
    #[serde(rename = "NONE")]
    /// [AI 생성] 스크롤바 없음
    None,
    #[serde(rename = "VERTICAL")]
    /// [AI 생성] 세로 스크롤바만 표시
    Vertical,
    #[serde(rename = "HORIZONTAL")]
    /// [AI 생성] 가로 스크롤바만 표시
    Horizontal,
    #[serde(rename = "BOTH")]
    /// [AI 생성] 가로/세로 모두 표시
    Both,
}

/// 편집 상자 탭키 동작
///
/// 원본: `EditType.tabKeyBehavior` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditTabKeyBehavior {
    #[serde(rename = "NEXT_OBJECT")]
    /// [AI 생성] 탭키로 다음 개체로 이동
    NextObject,
    #[serde(rename = "INSERT_TAB")]
    /// [AI 생성] 탭 문자를 입력
    InsertTab,
}

/// 편집 상자 정렬
///
/// 원본: `EditType.alignText` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EditTextAlignment {
    #[default]
    #[serde(rename = "LEFT")]
    /// [AI 생성] 좌측 정렬
    Left,
    #[serde(rename = "CENTER")]
    /// [AI 생성] 중앙 정렬
    Center,
    #[serde(rename = "RIGHT")]
    /// [AI 생성] 우측 정렬
    Right,
}

/// 편집 상자
///
/// 원본: `EditType` (`edit` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "edit")]
pub struct Edit {
    // AbstractShapeObjectType 요소들
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

    // AbstractFormObjectType 요소
    /// [AI 생성] 양식 글자 속성 (`formCharPr` 요소)
    #[serde(rename = "formCharPr")]
    pub form_char_property: FormCharacterProperty,

    // EditType 전용 요소
    /// [AI 생성] 편집 상자 내용 (`text` 요소)
    #[serde(rename = "text")]
    pub text: String,

    // 속성들
    /// [AI 생성] 객체 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,

    // AbstractFormObjectType 속성들
    /// [AI 생성] 컨트롤 이름 (`name` 속성)
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [AI 생성] 전경색 (`foreColor` 속성)
    #[serde(rename = "@foreColor", skip_serializing_if = "Option::is_none")]
    pub fore_color: Option<RgbColor>,
    /// [AI 생성] 배경색 (`backColor` 속성)
    #[serde(rename = "@backColor", skip_serializing_if = "Option::is_none")]
    pub back_color: Option<RgbColor>,
    /// [AI 생성] 그룹 이름 (`groupName` 속성)
    #[serde(rename = "@groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// [AI 생성] 탭 이동 허용 여부 (`tabStop` 속성)
    #[serde(rename = "@tabStop", default = "default_true")]
    pub tab_stop: bool,
    /// [AI 생성] 편집 가능 여부 (`editable` 속성)
    #[serde(rename = "@editable", default = "default_true")]
    pub form_editable: bool,
    /// [AI 생성] 탭 순서 (`tabOrder` 속성)
    #[serde(rename = "@tabOrder", skip_serializing_if = "Option::is_none")]
    pub tab_order: Option<i32>,
    /// [AI 생성] 사용 가능 여부 (`enabled` 속성)
    #[serde(rename = "@enabled", default = "default_true")]
    pub enabled: bool,
    /// [AI 생성] 테두리 타입 참조 (`borderTypeIDRef` 속성)
    #[serde(rename = "@borderTypeIDRef", skip_serializing_if = "Option::is_none")]
    pub border_type_id_ref: Option<BorderTypeIdRef>,
    /// [AI 생성] 프레임 출력 여부 (`drawFrame` 속성)
    #[serde(rename = "@drawFrame", default = "default_true")]
    pub draw_frame: bool,
    /// [AI 생성] 인쇄 여부 (`printable` 속성)
    #[serde(rename = "@printable", default = "default_true")]
    pub printable: bool,

    // EditType 전용 속성들
    /// [AI 생성] 다중 줄 입력 여부 (`multiLine` 속성)
    #[serde(rename = "@multiLine", default)]
    pub multi_line: bool,
    /// [AI 생성] 비밀번호 마스킹 문자 (`passwordChar` 속성)
    #[serde(rename = "@passwordChar", default = "default_password_char")]
    pub password_char: String,
    /// [AI 생성] 최대 길이 (`maxLength` 속성)
    #[serde(rename = "@maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    /// [AI 생성] 스크롤바 표시 설정 (`scrollBars` 속성)
    #[serde(rename = "@scrollBars", default)]
    pub scroll_bars: EditScrollBars,
    /// [AI 생성] 탭키 동작 (`tabKeyBehavior` 속성)
    #[serde(rename = "@tabKeyBehavior", skip_serializing_if = "Option::is_none")]
    pub tab_key_behavior: Option<EditTabKeyBehavior>,
    /// [AI 생성] 숫자 입력만 허용 (`numOnly` 속성)
    #[serde(rename = "@numOnly", default)]
    pub number_only: bool,
    /// [AI 생성] 읽기 전용 여부 (`readOnly` 속성)
    #[serde(rename = "@readOnly", default)]
    pub read_only: bool,
    /// [AI 생성] 텍스트 정렬 (`alignText` 속성)
    #[serde(rename = "@alignText", default)]
    pub align_text: EditTextAlignment,
}

fn default_password_char() -> String {
    "*".to_string()
}

/// 스크롤바 종류
///
/// 원본: `ScrollBarType.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollBarType {
    /// [AI 생성] 가로 스크롤바
    #[serde(rename = "HORIZONTAL")]
    Horizontal,
    /// [AI 생성] 세로 스크롤바
    #[serde(rename = "VERTICAL")]
    Vertical,
}

/// 스크롤바
///
/// 원본: `ScrollBarType` (`scrollBar` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "scrollBar")]
pub struct ScrollBar {
    // AbstractShapeObjectType 요소들
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

    // AbstractFormObjectType 요소
    /// [AI 생성] 양식 글자 속성 (`formCharPr` 요소)
    #[serde(rename = "formCharPr")]
    pub form_char_property: FormCharacterProperty,

    // 속성들
    /// [AI 생성] 객체 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,

    // AbstractFormObjectType 속성들
    /// [AI 생성] 컨트롤 이름 (`name` 속성)
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [AI 생성] 전경색 (`foreColor` 속성)
    #[serde(rename = "@foreColor", skip_serializing_if = "Option::is_none")]
    pub fore_color: Option<RgbColor>,
    /// [AI 생성] 배경색 (`backColor` 속성)
    #[serde(rename = "@backColor", skip_serializing_if = "Option::is_none")]
    pub back_color: Option<RgbColor>,
    /// [AI 생성] 그룹 이름 (`groupName` 속성)
    #[serde(rename = "@groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// [AI 생성] 탭 이동 허용 여부 (`tabStop` 속성)
    #[serde(rename = "@tabStop", default = "default_true")]
    pub tab_stop: bool,
    /// [AI 생성] 편집 가능 여부 (`editable` 속성)
    #[serde(rename = "@editable", default = "default_true")]
    pub editable: bool,
    /// [AI 생성] 탭 순서 (`tabOrder` 속성)
    #[serde(rename = "@tabOrder", skip_serializing_if = "Option::is_none")]
    pub tab_order: Option<i32>,
    /// [AI 생성] 사용 가능 여부 (`enabled` 속성)
    #[serde(rename = "@enabled", default = "default_true")]
    pub enabled: bool,
    /// [AI 생성] 테두리 타입 참조 (`borderTypeIDRef` 속성)
    #[serde(rename = "@borderTypeIDRef", skip_serializing_if = "Option::is_none")]
    pub border_type_id_ref: Option<BorderTypeIdRef>,
    /// [AI 생성] 프레임 출력 여부 (`drawFrame` 속성)
    #[serde(rename = "@drawFrame", default = "default_true")]
    pub draw_frame: bool,
    /// [AI 생성] 인쇄 여부 (`printable` 속성)
    #[serde(rename = "@printable", default = "default_true")]
    pub printable: bool,

    // ScrollBarType 전용 속성들
    /// [AI 생성] 반복 지연 시간 (`delay` 속성)
    #[serde(rename = "@delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<u32>,
    /// [AI 생성] 큰 증감값 (`largeChange` 속성)
    #[serde(rename = "@largeChange", skip_serializing_if = "Option::is_none")]
    pub large_change: Option<u32>,
    /// [AI 생성] 작은 증감값 (`smallChange` 속성)
    #[serde(rename = "@smallChange", skip_serializing_if = "Option::is_none")]
    pub small_change: Option<u32>,
    /// [AI 생성] 최소값 (`min` 속성)
    #[serde(rename = "@min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    /// [AI 생성] 최대값 (`max` 속성)
    #[serde(rename = "@max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// [AI 생성] 페이지 단위 (`page` 속성)
    #[serde(rename = "@page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// [AI 생성] 현재 값 (`value` 속성)
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
    /// [AI 생성] 스크롤바 방향 (`type` 속성)
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub bar_type: Option<ScrollBarType>,
}
