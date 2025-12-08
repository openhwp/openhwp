//! [AI 생성 문서화] 번호 문단 모양
//!
//! 여러 수준의 번호 목록을 정의하는 스타일 모음입니다. 각 레벨별 머리표 형식(번호/문자/기호)과 시작값을 포함해 렌더러가 동일한 다단계 목록을 재현할 때 사용합니다. KS X 6101:2024 `header.xsd` 기반.

use serde::{Deserialize, Serialize};

use super::paragraph_head::ParagraphHead;

/// [AI 생성] 번호 문단 모양
///
/// 원본: `NumberingType`. 다단계 번호 스타일 한 세트.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "numbering")]
pub struct Numbering {
    /// [AI 생성] 문단 머리 목록 (`paraHead` 요소). 레벨별 머리표 정의.
    #[serde(rename = "paraHead")]
    pub paragraph_heads: Vec<ParagraphHead>,

    /// [AI 생성] 번호 문단 모양 아이디 (`id` 속성). 스타일 참조용 키.
    #[serde(rename = "@id")]
    pub id: u32,

    /// [AI 생성] 시작 번호 (`start` 속성). 1 기반 기본값.
    #[serde(rename = "@start", default = "default_start")]
    pub start: i32,
}

fn default_start() -> i32 {
    1
}

/// [AI 생성] 번호 문단 모양 목록
///
/// 원본: `numberings` 요소의 익명 타입
/// 원본: `numberings` 요소의 익명 타입. 문서 전역 번호 목록 스타일 풀입니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "numberings")]
pub struct NumberingList {
    /// [AI 생성] 번호 문단 모양 목록 (`numbering` 요소)
    #[serde(rename = "numbering")]
    pub numberings: Vec<Numbering>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성). 목록 길이 검증용.
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}
