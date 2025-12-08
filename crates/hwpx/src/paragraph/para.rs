//! [AI 생성] 문단 요소 (runs + 줄 세그먼트)
//!
//! 런 집합과 레이아웃 결과(linesegarray)를 함께 보관하는 최소 단위 문단입니다. 수평/수직 위치, 줄바꿈 정보가 이미 계산된 상태로 저장될 수 있어 재레이아웃 시 참고됩니다. KS X 6101:2024 `paralist.xsd`.

use serde::{Deserialize, Serialize};

use super::run::Run;
use crate::core::types::{ParaShapeIdRef, StyleIdRef};

/// [AI 생성] 줄 세그먼트 배열
///
/// 문단의 줄 배치 정보를 담고 있습니다.
/// 원본: `linesegarray` 요소
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "linesegarray")]
pub struct LineSegmentArray {
    /// [AI 생성] 줄 세그먼트 목록
    #[serde(rename = "lineseg", default)]
    pub segments: Vec<LineSegment>,
}

/// [AI 생성] 줄 세그먼트
///
/// 개별 줄의 배치 정보입니다.
/// 원본: `lineseg` 요소
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "lineseg")]
pub struct LineSegment {
    /// [AI 생성] 텍스트 시작 위치
    #[serde(rename = "@textpos", default)]
    pub text_position: i32,

    /// [AI 생성] 세로 위치
    #[serde(rename = "@vertpos", default)]
    pub vertical_position: i32,

    /// [AI 생성] 세로 크기
    #[serde(rename = "@vertsize", default)]
    pub vertical_size: i32,

    /// [AI 생성] 텍스트 높이
    #[serde(rename = "@textheight", default)]
    pub text_height: i32,

    /// [AI 생성] 베이스라인
    #[serde(rename = "@baseline", default)]
    pub baseline: i32,

    /// [AI 생성] 줄 간격
    #[serde(rename = "@spacing", default)]
    pub spacing: i32,

    /// [AI 생성] 가로 위치
    #[serde(rename = "@horzpos", default)]
    pub horizontal_position: i32,

    /// [AI 생성] 가로 크기
    #[serde(rename = "@horzsize", default)]
    pub horizontal_size: i32,

    /// [AI 생성] 플래그
    #[serde(rename = "@flags", default)]
    pub flags: u32,
}

/// [AI 생성] 문단
///
/// 원본: `PType`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "p")]
pub struct Paragraph {
    /// [AI 생성] 런 목록 (`run` 요소)
    #[serde(rename = "run", default)]
    pub runs: Vec<Run>,

    /// [AI 생성] 줄 세그먼트 배열 (`linesegarray` 요소)
    #[serde(rename = "linesegarray", skip_serializing_if = "Option::is_none")]
    pub line_segments: Option<LineSegmentArray>,

    /// [AI 생성] 문단 아이디 (`id` 속성)
    #[serde(rename = "@id")]
    pub id: u32,

    /// [AI 생성] 문단 모양 아이디 참조 (`paraPrIDRef` 속성)
    #[serde(rename = "@paraPrIDRef", skip_serializing_if = "Option::is_none")]
    pub paragraph_property_id_reference: Option<ParaShapeIdRef>,

    /// [AI 생성] 스타일 아이디 참조 (`styleIDRef` 속성)
    #[serde(rename = "@styleIDRef", skip_serializing_if = "Option::is_none")]
    pub style_id_reference: Option<StyleIdRef>,

    /// [AI 생성] 페이지 나누기 (`pageBreak` 속성)
    #[serde(rename = "@pageBreak", default)]
    pub page_break: bool,

    /// [AI 생성] 단 나누기 (`columnBreak` 속성)
    #[serde(rename = "@columnBreak", default)]
    pub column_break: bool,

    /// [AI 생성] 병합됨 (`merged` 속성)
    #[serde(rename = "@merged", default)]
    pub merged: bool,

    /// [AI 생성] 문단 변경 추적 아이디 (`paraTcId` 속성)
    #[serde(rename = "@paraTcId", skip_serializing_if = "Option::is_none")]
    pub paragraph_track_change_id: Option<u32>,
}
