//! [AI 생성 문서화] 매핑 테이블
//!
//! 문서에서 참조되는 글꼴/테두리/문단/스타일/바이너리 등의 리소스를 한곳에 모은 전역 테이블입니다. ID 참조로 연결되는 요소들이므로 렌더러는 이 테이블을 먼저 로드해야 합니다. KS X 6101:2024 `header.xsd` 기반, 세부는 `docs/hwpx/schemas/header.xsd` 참고.

use serde::{Deserialize, Serialize};

use super::{
    border_fill::BorderFillList, bullet::BulletList, character_shape::CharacterShapeList,
    font::FontfaceList, memo_shape::MemoShapeList, numbering::NumberingList,
    paragraph_shape::ParagraphShapeList, style::StyleList, tab_definition::TabDefinitionList,
};
use crate::core::types::BinaryItemIdRef;

/// [AI 생성] 바이너리 데이터
///
/// 원본: `binData` 요소의 익명 타입. 컨테이너 내부 바이너리 항목의 메타데이터를 담습니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "binData")]
pub struct BinaryData {
    /// [AI 생성] 절대 경로 (`absPath` 속성)
    #[serde(rename = "@absPath", skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,

    /// [AI 생성] 상대 경로 (`relPath` 속성)
    #[serde(rename = "@relPath", skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,

    /// [AI 생성] 바이너리 아이템 아이디 참조 (`binaryItemIDRef` 속성)
    #[serde(rename = "@binaryItemIDRef", skip_serializing_if = "Option::is_none")]
    pub binary_item_id_reference: Option<BinaryItemIdRef>,

    /// [AI 생성] 포맷 (`format` 속성)
    #[serde(rename = "@format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// [AI 생성] 바이너리 데이터 아이디 (`id` 속성)
    #[serde(rename = "@id")]
    pub id: u32,
}

/// [AI 생성] 바이너리 데이터 목록
///
/// 원본: `binDataList` 요소의 익명 타입. 바이너리 항목과 개수를 함께 보관합니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "binDataList")]
pub struct BinaryDataList {
    /// [AI 생성] 바이너리 데이터 목록 (`binData` 요소)
    #[serde(rename = "binData")]
    pub binary_data: Vec<BinaryData>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성). 목록 길이 검증용.
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}

/// [AI 생성] 매핑 테이블
///
/// 문서 내 폰트/테두리/문단모양/스타일 등 참조 테이블을 모읍니다. 원본: `mappingTable` 요소. 렌더링 시 ID로 참조되는 모든 리소스 풀입니다.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "mappingTable")]
pub struct MappingTable {
    /// [AI 생성] 바이너리 데이터 목록 (`binDataList` 요소)
    #[serde(rename = "binDataList", skip_serializing_if = "Option::is_none")]
    pub binary_data_list: Option<BinaryDataList>,

    /// [AI 생성] 글꼴 목록 (`fontfaces` 요소)
    #[serde(rename = "fontfaces", skip_serializing_if = "Option::is_none")]
    pub fontfaces: Option<FontfaceList>,

    /// [AI 생성] 테두리 채우기 목록 (`borderFills` 요소)
    #[serde(rename = "borderFills", skip_serializing_if = "Option::is_none")]
    pub border_fills: Option<BorderFillList>,

    /// [AI 생성] 글자 모양 목록 (`charShapes` 요소)
    #[serde(rename = "charShapes", skip_serializing_if = "Option::is_none")]
    pub character_shapes: Option<CharacterShapeList>,

    /// [AI 생성] 탭 정의 목록 (`tabDefs` 요소)
    #[serde(rename = "tabDefs", skip_serializing_if = "Option::is_none")]
    pub tab_definitions: Option<TabDefinitionList>,

    /// [AI 생성] 번호 문단 모양 목록 (`numberings` 요소)
    #[serde(rename = "numberings", skip_serializing_if = "Option::is_none")]
    pub numberings: Option<NumberingList>,

    /// [AI 생성] 글머리표 목록 (`bullets` 요소)
    #[serde(rename = "bullets", skip_serializing_if = "Option::is_none")]
    pub bullets: Option<BulletList>,

    /// [AI 생성] 문단 모양 목록 (`paraShapes` 요소)
    #[serde(rename = "paraShapes", skip_serializing_if = "Option::is_none")]
    pub paragraph_shapes: Option<ParagraphShapeList>,

    /// [AI 생성] 스타일 목록 (`styles` 요소)
    #[serde(rename = "styles", skip_serializing_if = "Option::is_none")]
    pub styles: Option<StyleList>,

    /// [AI 생성] 메모 모양 목록 (`memoShapes` 요소)
    #[serde(rename = "memoShapes", skip_serializing_if = "Option::is_none")]
    pub memo_shapes: Option<MemoShapeList>,
}
