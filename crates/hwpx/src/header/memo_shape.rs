//! [AI 생성 문서화] 메모 모양
//!
//! 본문 주석(메모) 박스의 외형과 추적 유형을 정의합니다. 메모의 색·선·두께와 변경 추적 타입별 색상을 재현할 때 사용합니다. KS X 6101:2024 `header.xsd` 기반 설명이며 세부 스키마는 `docs/hwpx/schemas/header.xsd` 참고.

use serde::{Deserialize, Serialize};

use crate::core::{enums::LineStyleType2, types::RgbColor};

/// [AI 생성] 메모 유형 (변경 추적용)
///
/// 원본: `MemoShapeType.memoType` 속성의 익명 타입. 메모가 어떤 변경 흐름에서 생성됐는지 표시합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MemoTrackingType {
    /// [AI 생성] 일반
    #[default]
    #[serde(rename = "NOMAL")]
    Normal,
    /// [AI 생성] 사용자 삽입
    #[serde(rename = "USER_INSERT")]
    UserInsert,
    /// [AI 생성] 사용자 삭제
    #[serde(rename = "USER_DELETE")]
    UserDelete,
    /// [AI 생성] 사용자 수정
    #[serde(rename = "USER_UPDATE")]
    UserUpdate,
}

/// [AI 생성] 메모 모양
///
/// 원본: `MemoShapeType`. 메모 박스 외형과 변경 추적 색상을 정의합니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "memoShape")]
pub struct MemoShape {
    /// [AI 생성] 메모 모양 아이디 (`id` 속성). 참조용 키.
    #[serde(rename = "@id")]
    pub id: u32,

    /// [AI 생성] 메모 표시 너비 (`width` 속성). hwpunit.
    #[serde(rename = "@width")]
    pub width: u32,

    /// [AI 생성] 메모 라인 두께 (`lineWidth` 속성). 선택적 폭.
    #[serde(rename = "@lineWidth", skip_serializing_if = "Option::is_none")]
    pub line_width: Option<String>,

    /// [AI 생성] 메모 선 종류 (`lineType` 속성). 테두리 라인 스타일.
    #[serde(rename = "@lineType")]
    pub line_type: LineStyleType2,

    /// [AI 생성] 메모 선 색 (`lineColor` 속성)
    #[serde(rename = "@lineColor")]
    pub line_color: RgbColor,

    /// [AI 생성] 메모 색 (`fillColor` 속성). 기본 배경색.
    #[serde(rename = "@fillColor")]
    pub fill_color: RgbColor,

    /// [AI 생성] 메모 활성 시 색 (`activeColor` 속성). 포커스 시 색상.
    #[serde(rename = "@activeColor")]
    pub active_color: RgbColor,

    /// [AI 생성] 메모 변경 추적 유형 (`memoType` 속성). 삽입/삭제/수정 등 구분.
    #[serde(rename = "@memoType", skip_serializing_if = "Option::is_none")]
    pub memo_type: Option<MemoTrackingType>,
}

/// [AI 생성] 메모 모양 목록
///
/// 원본: `memoShapes` 요소의 익명 타입. 문서 전체 메모 스타일 집합입니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "memoShapes")]
pub struct MemoShapeList {
    /// [AI 생성] 메모 모양 목록 (`memoShape` 요소)
    #[serde(rename = "memoShape")]
    pub memo_shapes: Vec<MemoShape>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성). 목록 길이 검증용.
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}
