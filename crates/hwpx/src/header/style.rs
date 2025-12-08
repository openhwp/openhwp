//! [AI 생성 문서화] 스타일
//!
//! KS X 6101:2024 - header.xsd 기반 설명입니다. 실제 스키마는 `docs/hwpx/schemas/header.xsd`를 확인하세요.

use serde::{Deserialize, Serialize};

use crate::core::types::{CharShapeIdRef, ParaShapeIdRef, StyleIdRef};

/// [AI 생성] 스타일 종류
///
/// 원본: `StyleType.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum StyleKind {
    /// [AI 생성] 문단 스타일
    #[default]
    #[serde(rename = "PARA")]
    Paragraph,
    /// [AI 생성] 글자 스타일
    #[serde(rename = "CHAR")]
    Character,
}

/// [AI 생성] 스타일
///
/// 원본: `StyleType`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "style")]
pub struct Style {
    /// [AI 생성] 스타일 아이디 (`id` 속성)
    #[serde(rename = "@id")]
    pub id: u32,

    /// [AI 생성] 스타일 종류 (`type` 속성)
    #[serde(rename = "@type")]
    pub style_type: StyleKind,

    /// [AI 생성] 로컬 스타일 이름 (`name` 속성)
    #[serde(rename = "@name")]
    pub name: String,

    /// [AI 생성] 영문 스타일 이름 (`engName` 속성)
    #[serde(rename = "@engName", skip_serializing_if = "Option::is_none")]
    pub english_name: Option<String>,

    /// [AI 생성] 문단 모양 아이디 참조 (`paraPrIDRef` 속성)
    #[serde(rename = "@paraPrIDRef", skip_serializing_if = "Option::is_none")]
    pub paragraph_shape_id_reference: Option<ParaShapeIdRef>,

    /// [AI 생성] 글자 모양 아이디 참조 (`charPrIDRef` 속성)
    #[serde(rename = "@charPrIDRef", skip_serializing_if = "Option::is_none")]
    pub character_shape_id_reference: Option<CharShapeIdRef>,

    /// [AI 생성] 다음 스타일 아이디 참조 (`nextStyleIDRef` 속성)
    /// 문단 스타일에서 리턴 시 적용될 다음 스타일
    #[serde(rename = "@nextStyleIDRef", skip_serializing_if = "Option::is_none")]
    pub next_style_id_reference: Option<StyleIdRef>,

    /// [AI 생성] 언어 아이디 (`langID` 속성)
    #[serde(rename = "@langID", skip_serializing_if = "Option::is_none")]
    pub language_id: Option<u16>,

    /// [AI 생성] 양식 모드에서 스타일 보호 여부 (`lockForm` 속성)
    #[serde(rename = "@lockForm", default)]
    pub lock_form: bool,
}

/// [AI 생성] 스타일 목록
///
/// 원본: `styles` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "styles")]
pub struct StyleList {
    /// [AI 생성] 스타일 목록 (`style` 요소)
    #[serde(rename = "style")]
    pub styles: Vec<Style>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성)
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}
