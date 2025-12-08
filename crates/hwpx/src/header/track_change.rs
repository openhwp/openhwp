//! 변경 추적
//!
//! KS X 6101:2024 - header.xsd

use serde::{Deserialize, Serialize};

use crate::core::{enums::TrackChangeType, types::RgbColor};

/// [AI 생성] 변경 추적 정보
///
/// 원본: `TrackChange`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "trackChange")]
pub struct TrackChange {
    /// [AI 생성] 변경 추적 유형 (`type` 속성)
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub change_type: Option<TrackChangeType>,

    /// [AI 생성] 날짜 (`date` 속성)
    #[serde(rename = "@date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    /// [AI 생성] 작성자 아이디 (`authorID` 속성)
    #[serde(rename = "@authorID", skip_serializing_if = "Option::is_none")]
    pub author_id: Option<u32>,

    /// [AI 생성] 글자 모양 아이디 (`charShapeID` 속성)
    #[serde(rename = "@charShapeID", skip_serializing_if = "Option::is_none")]
    pub character_shape_id: Option<u32>,

    /// [AI 생성] 문단 모양 아이디 (`paraShapeID` 속성)
    #[serde(rename = "@paraShapeID", skip_serializing_if = "Option::is_none")]
    pub paragraph_shape_id: Option<u32>,

    /// [AI 생성] 숨김 여부 (`hide` 속성)
    #[serde(rename = "@hide")]
    pub hide: bool,

    /// [AI 생성] 변경 추적 아이디 (`id` 속성)
    #[serde(rename = "@id")]
    pub id: u32,
}

/// [AI 생성] 변경 추적 사용자
///
/// 원본: `TrackChangeAuthor`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "trackChangeAuthor")]
pub struct TrackChangeAuthor {
    /// [AI 생성] 사용자 이름 (`name` 속성)
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// [AI 생성] 표시 여부 (`mark` 속성)
    #[serde(rename = "@mark", skip_serializing_if = "Option::is_none")]
    pub mark: Option<bool>,

    /// [AI 생성] 색상 (`color` 속성)
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<RgbColor>,

    /// [AI 생성] 작성자 아이디 (`id` 속성)
    #[serde(rename = "@id")]
    pub id: u32,
}

/// [AI 생성] 변경 추적 목록
///
/// 원본: `trackChanges` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "trackChanges")]
pub struct TrackChangeList {
    /// [AI 생성] 변경 추적 목록 (`trackChange` 요소)
    #[serde(rename = "trackChange")]
    pub track_changes: Vec<TrackChange>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성)
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}

/// [AI 생성] 변경 추적 사용자 목록
///
/// 원본: `trackChangeAuthors` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "trackChangeAuthors")]
pub struct TrackChangeAuthorList {
    /// [AI 생성] 변경 추적 사용자 목록 (`trackChangeAuthor` 요소)
    #[serde(rename = "trackChangeAuthor")]
    pub authors: Vec<TrackChangeAuthor>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성)
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}

/// [AI 생성] 변경 추적 설정
///
/// 원본: 문서 헤더의 변경 추적 관련 설정
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "trackChangeConfig")]
pub struct TrackChangeConfig {
    /// [AI 생성] 변경 추적 사용자 목록 (`trackChangeAuthors` 요소)
    #[serde(rename = "trackChangeAuthors", skip_serializing_if = "Option::is_none")]
    pub authors: Option<TrackChangeAuthorList>,

    /// [AI 생성] 변경 추적 목록 (`trackChanges` 요소)
    #[serde(rename = "trackChanges", skip_serializing_if = "Option::is_none")]
    pub changes: Option<TrackChangeList>,
}
