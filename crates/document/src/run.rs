//! 런 (Run)
//!
//! 동일한 글자 모양을 가진 텍스트 단위입니다.

use ir::CharShapeId;

use crate::id::ControlId;
use crate::run_content::RunContent;

/// 런
#[derive(Debug, Clone, Default)]
pub struct Run {
    /// 글자 모양 ID
    pub char_shape_id: Option<CharShapeId>,
    /// 내용 목록
    pub contents: Vec<RunContent>,
}

impl Run {
    /// 새 런 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 글자 모양 ID로 런 생성
    pub fn with_char_shape(char_shape_id: CharShapeId) -> Self {
        Self {
            char_shape_id: Some(char_shape_id),
            contents: Vec::new(),
        }
    }

    /// 텍스트로 런 생성
    pub fn with_text(text: impl Into<String>) -> Self {
        Self {
            char_shape_id: None,
            contents: vec![RunContent::Text(text.into())],
        }
    }

    /// 텍스트 추가
    pub fn add_text(&mut self, text: impl Into<String>) {
        self.contents.push(RunContent::Text(text.into()));
    }

    /// 컨트롤 추가
    pub fn add_control(&mut self, control_id: ControlId) {
        self.contents.push(RunContent::Control(control_id));
    }

    /// 탭 추가
    pub fn add_tab(&mut self) {
        self.contents.push(RunContent::Tab);
    }

    /// 줄바꿈 추가
    pub fn add_line_break(&mut self) {
        self.contents.push(RunContent::LineBreak);
    }

    /// 일반 텍스트 추출
    pub fn to_plain_text(&self) -> String {
        let mut text = String::new();
        for content in &self.contents {
            match content {
                RunContent::Text(s) => text.push_str(s),
                RunContent::Tab => text.push('\t'),
                RunContent::LineBreak => text.push('\n'),
                RunContent::NonBreakingSpace => text.push('\u{00A0}'),
                RunContent::FixedWidthSpace => text.push(' '),
                _ => {}
            }
        }
        text
    }

    /// 내용 수
    pub fn content_count(&self) -> usize {
        self.contents.len()
    }

    /// 텍스트 길이 (문자 수)
    pub fn text_length(&self) -> usize {
        let mut len = 0;
        for content in &self.contents {
            match content {
                RunContent::Text(s) => len += s.chars().count(),
                RunContent::Tab
                | RunContent::LineBreak
                | RunContent::Hyphen
                | RunContent::NonBreakingSpace
                | RunContent::FixedWidthSpace => len += 1,
                RunContent::Control(_) => len += 1,
                RunContent::FieldStart(_) | RunContent::FieldEnd => len += 1,
                RunContent::BookmarkStart(_) | RunContent::BookmarkEnd => {} // 길이 0
            }
        }
        len
    }
}
