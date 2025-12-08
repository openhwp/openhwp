//! [AI 생성 문서화] 글머리표
//!
//! KS X 6101:2024 - header.xsd 기반 설명입니다. 세부 규격은 `docs/hwpx/schemas/header.xsd`를 교차 확인하세요.

use serde::{Deserialize, Serialize};

use super::paragraph_head::ParagraphHead;
use crate::core::types::Image;

/// [AI 생성] 글머리표
///
/// 원본: `BulletType`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "bullet")]
pub struct Bullet {
    /// [AI 생성] 글머리표 이미지 (`img` 요소)
    #[serde(rename = "img", skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,

    /// [AI 생성] 문단 머리 정보 (`paraHead` 요소)
    #[serde(rename = "paraHead")]
    pub paragraph_head: ParagraphHead,

    /// [AI 생성] 글머리표 아이디 (`id` 속성)
    #[serde(rename = "@id")]
    pub id: u32,

    /// [AI 생성] 글머리표 문자 (`char` 속성)
    #[serde(rename = "@char")]
    pub character: String,

    /// [AI 생성] 선택된 글머리표 문자 (`checkedChar` 속성)
    #[serde(rename = "@checkedChar", skip_serializing_if = "Option::is_none")]
    pub checked_character: Option<String>,

    /// [AI 생성] 이미지 사용 여부 (`useImage` 속성)
    #[serde(rename = "@useImage")]
    pub use_image: bool,
}

/// [AI 생성] 글머리표 목록
///
/// 원본: `bullets` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "bullets")]
pub struct BulletList {
    /// [AI 생성] 글머리표 목록 (`bullet` 요소)
    #[serde(rename = "bullet")]
    pub bullets: Vec<Bullet>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성)
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}
