//! [AI 생성 문서화] 탭 정의
//!
//! 문단 탭 위치와 정렬, 채움 문자를 정의합니다. 자동 탭 설정과 개별 탭 아이템을 포함해 편집기 탭 스톱을 재현할 때 사용합니다. KS X 6101:2024 `header.xsd` 기반.

use serde::{Deserialize, Serialize};

use crate::core::enums::LineStyleType2;

/// [AI 생성] 탭 유형
///
/// 원본: `tabItem.type` 속성의 익명 타입. 탭 스톱 정렬 방식을 지정합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TabType {
    /// [AI 생성] 왼쪽 정렬
    #[default]
    #[serde(rename = "LEFT")]
    Left,
    /// [AI 생성] 오른쪽 정렬
    #[serde(rename = "RIGHT")]
    Right,
    /// [AI 생성] 가운데 정렬
    #[serde(rename = "CENTER")]
    Center,
    /// [AI 생성] 소수점 기준 정렬
    #[serde(rename = "DECIMAL")]
    Decimal,
}

/// [AI 생성] 탭 항목
///
/// 원본: `tabItem` 요소의 익명 타입. 단일 탭 스톱 정의.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tabItem")]
pub struct TabItem {
    /// [AI 생성] 탭 위치 hwpunit (`pos` 속성). 문단 시작점 기준 위치.
    #[serde(rename = "@pos")]
    pub position: i32,

    /// [AI 생성] 탭 종류 (`type` 속성). Left/Right/Center/Decimal.
    #[serde(rename = "@type")]
    pub tab_type: TabType,

    /// [AI 생성] 채움 종류 (`leader` 속성). 탭 사이를 채우는 점선/라인 스타일.
    #[serde(rename = "@leader")]
    pub leader: LineStyleType2,
}

/// [AI 생성] 탭 정의
///
/// 원본: `TabDefType`. 자동 탭 설정과 단일 탭 아이템을 포함합니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tabDef")]
pub struct TabDefinition {
    /// [AI 생성] 탭 항목 (`tabItem` 요소). 단일 탭 스톱. 없으면 자동 탭만 사용.
    #[serde(rename = "tabItem", skip_serializing_if = "Option::is_none")]
    pub tab_item: Option<TabItem>,

    /// [AI 생성] 탭 정의 아이디 (`id` 속성). 스타일 참조용 키.
    #[serde(rename = "@id")]
    pub id: u32,

    /// [AI 생성] 문단 왼쪽 끝 자동 탭 (`autoTabLeft` 속성). 문단 시작점에 기본 탭 스톱 추가.
    #[serde(rename = "@autoTabLeft", default)]
    pub auto_tab_left: bool,

    /// [AI 생성] 문단 오른쪽 끝 자동 탭 (`autoTabRight` 속성). 문단 끝 위치에 기본 탭 스톱 추가.
    #[serde(rename = "@autoTabRight", default)]
    pub auto_tab_right: bool,
}

/// [AI 생성] 탭 정의 목록
///
/// 원본: `tabDefs` 요소의 익명 타입. 문서 전체 탭 스타일 컬렉션입니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tabDefs")]
pub struct TabDefinitionList {
    /// [AI 생성] 탭 정의 목록 (`tabDef` 요소)
    #[serde(rename = "tabDef")]
    pub tab_definitions: Vec<TabDefinition>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성). 목록 길이 검증용.
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}
