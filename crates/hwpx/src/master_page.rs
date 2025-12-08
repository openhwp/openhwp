//! [AI 생성 문서화] 바탕쪽 정보
//!
//! 양쪽/짝수/홀수/마지막 쪽 등에 반복 적용되는 마스터 페이지 정의입니다. 배경 문단 목록과 적용 범위를 묶어 지면마다 자동 배치합니다. KS X 6101:2024 `masterpage.xsd` 기준.

use serde::{Deserialize, Serialize};

/// [AI 생성] 바탕쪽 적용 범위
///
/// 원본: `type` 속성의 익명 타입. 바탕쪽이 어느 페이지 유형에 적용될지 지정합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MasterPageApplicationType {
    /// 양쪽
    #[default]
    #[serde(rename = "BOTH")]
    Both,
    /// 짝수쪽
    #[serde(rename = "EVEN")]
    Even,
    /// 홀수쪽
    #[serde(rename = "ODD")]
    Odd,
    /// 마지막 쪽
    #[serde(rename = "LAST_PAGE")]
    LastPage,
    /// 임의의 쪽
    #[serde(rename = "OPTIONAL_PAGE")]
    OptionalPage,
}

/// [AI 생성] 바탕쪽 루트 요소
///
/// 원본: `masterPage` 요소. 문단 목록과 적용 범위를 묶어 특정 페이지 유형에 반복 적용됩니다.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "masterPage")]
pub struct MasterPage {
    /// [AI 생성] 문단 목록 (`subList` 요소). 바탕쪽에 고정 배치되는 문단 집합.
    #[serde(rename = "subList")]
    pub paragraph_list: crate::paragraph::ParagraphList,

    /// [AI 생성] 아이디 (`id` 속성). 마스터 페이지 식별자.
    #[serde(rename = "@id")]
    pub id: String,

    /// [AI 생성] 적용 범위 (`type` 속성). 양쪽/짝수/홀수/마지막/임의 쪽 선택.
    #[serde(rename = "@type", default)]
    pub application_type: MasterPageApplicationType,

    /// [AI 생성] 임의의 쪽 번호 (`pageNumber` 속성, OPTIONAL_PAGE일 때). 특정 쪽에만 적용.
    #[serde(rename = "@pageNumber", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<u32>,

    /// [AI 생성] 쪽 복사 여부 (`pageDuplicate` 속성). 앞뒤 면 동일 배치 여부.
    #[serde(rename = "@pageDuplicate", default)]
    pub page_duplicate: bool,

    /// [AI 생성] 쪽 앞면 여부 (`pageFront` 속성). 책 제본 기준 앞면 플래그.
    #[serde(rename = "@pageFront", default)]
    pub page_front: bool,
}
