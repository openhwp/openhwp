//! 금칙 문자
//!
//! 줄바꿈/조판 시 사용 금지 문자 목록을 담습니다. KS X 6101:2024 `header.xsd` 기준.

use serde::{Deserialize, Serialize};

/// [AI 생성] 금칙 문자 목록
///
/// 원본: `ForbiddenWordListType`. 줄 첫/끝에 올 수 없는 문자 세트를 정의합니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "forbiddenWordList")]
pub struct ForbiddenWordList {
    /// [AI 생성] 금칙 문자 목록 (`forbiddenWord` 요소). 문자·문자열 모두 허용됩니다.
    #[serde(rename = "forbiddenWord")]
    pub forbidden_words: Vec<String>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성). 리스트 길이 검증용.
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}
