//! 문단
//!
//! 문서의 문단을 정의합니다.

use ir::{ParaShapeId, StyleId};

use crate::id::RunId;

/// 문단
#[derive(Debug, Clone, Default)]
pub struct Paragraph {
    /// 문단 모양 ID
    pub para_shape_id: Option<ParaShapeId>,
    /// 스타일 ID
    pub style_id: Option<StyleId>,
    /// 런 목록
    pub runs: Vec<RunId>,
    /// 나눔 종류
    pub break_type: BreakType,
    /// 인스턴스 ID (고유 식별자)
    pub instance_id: Option<u32>,
    /// 범위 태그
    pub range_tags: Vec<RangeTag>,
}

impl Paragraph {
    /// 새 문단 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 런 추가
    pub fn add_run(&mut self, run_id: RunId) {
        self.runs.push(run_id);
    }

    /// 런 수
    pub fn run_count(&self) -> usize {
        self.runs.len()
    }
}

/// 나눔 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BreakType {
    /// 나눔 없음
    #[default]
    None,
    /// 페이지 나눔
    Page,
    /// 단 나눔
    Column,
    /// 섹션 나눔
    Section,
}

/// 범위 태그
#[derive(Debug, Clone)]
pub struct RangeTag {
    /// 시작 위치 (문자 인덱스)
    pub start: u32,
    /// 끝 위치 (문자 인덱스)
    pub end: u32,
    /// 태그 종류
    pub tag_type: RangeTagType,
    /// 태그 데이터
    pub data: Option<String>,
    /// 변경 추적 정보
    pub track_change_info: Option<TrackChangeInfo>,
}

/// 범위 태그 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeTagType {
    /// 책갈피
    Bookmark,
    /// 하이퍼링크
    Hyperlink,
    /// 변경 추적 - 삽입
    TrackChangeInsert,
    /// 변경 추적 - 삭제
    TrackChangeDelete,
    /// 형광펜
    Highlight,
    /// 기타
    Other(u8),
}

/// 변경 추적 정보
#[derive(Debug, Clone)]
pub struct TrackChangeInfo {
    /// 변경 추적 ID
    pub track_change_id: u32,
    /// 태그 ID
    pub tag_id: Option<u32>,
    /// 문단 끝 여부
    pub paragraph_end: bool,
}
