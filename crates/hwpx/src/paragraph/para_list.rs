//! [AI 생성] 문단 리스트/구역 래퍼
//!
//! 단/텍스트 상자 내부에서 문단을 묶어 관리하는 컨테이너입니다. 방향(가로/세로), 줄바꿈 방식, 세로 정렬과 연결 리스트 참조를 통해 흐름을 이어 붙일 수 있습니다. KS X 6101:2024 `paralist.xsd`.

use serde::{Deserialize, Serialize};

use super::enums::{ParagraphLineWrap, ParagraphVerticalAlignment, TextDirection};
use super::para::Paragraph;
use crate::core::types::LinkListIdRef;

/// [AI 생성] 문단 리스트
///
/// 원본: `ParaListType`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "subList")]
pub struct ParagraphList {
    /// [AI 생성] 문단 목록 (`p` 요소)
    #[serde(rename = "p", default)]
    pub paragraphs: Vec<Paragraph>,

    /// [AI 생성] 아이디 (`id` 속성)
    #[serde(rename = "@id")]
    pub id: String,

    /// [AI 생성] 텍스트 방향 (`textDirection` 속성)
    #[serde(rename = "@textDirection", default)]
    pub text_direction: TextDirection,

    /// [AI 생성] 줄바꿈 (`lineWrap` 속성)
    #[serde(rename = "@lineWrap", default)]
    pub line_wrap: ParagraphLineWrap,

    /// [AI 생성] 세로 정렬 (`vertAlign` 속성)
    #[serde(rename = "@vertAlign", default)]
    pub vertical_alignment: ParagraphVerticalAlignment,

    /// [AI 생성] 연결 리스트 아이디 참조 (`linkListIDRef` 속성)
    #[serde(rename = "@linkListIDRef", skip_serializing_if = "Option::is_none")]
    pub link_list_id_reference: Option<LinkListIdRef>,

    /// [AI 생성] 연결 리스트 다음 아이디 참조 (`linkListNextIDRef` 속성)
    #[serde(rename = "@linkListNextIDRef", skip_serializing_if = "Option::is_none")]
    pub link_list_next_id_reference: Option<LinkListIdRef>,

    /// [AI 생성] 텍스트 영역의 폭 (`textWidth` 속성)
    #[serde(rename = "@textWidth", skip_serializing_if = "Option::is_none")]
    pub text_width: Option<u32>,

    /// [AI 생성] 텍스트 영역의 높이 (`textHeight` 속성)
    #[serde(rename = "@textHeight", skip_serializing_if = "Option::is_none")]
    pub text_height: Option<u32>,

    /// [AI 생성] 해당 레벨의 텍스트에 대한 참조 여부 (`hasTextRef` 속성)
    #[serde(rename = "@hasTextRef", default)]
    pub has_text_reference: bool,

    /// [AI 생성] 해당 레벨의 번호에 대한 참조 여부 (`hasNumRef` 속성)
    #[serde(rename = "@hasNumRef", default)]
    pub has_number_reference: bool,
}

/// [AI 생성] 구역
///
/// 원본: `SectionType`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "sec")]
pub struct Section {
    /// [AI 생성] 문단 목록 (`p` 요소)
    #[serde(rename = "p", default)]
    pub paragraphs: Vec<Paragraph>,
}
