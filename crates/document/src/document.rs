//! 문서 (Document)
//!
//! 리치 텍스트 에디터용 문서 최상위 구조입니다.

use ir::{BinaryDataStore, Extensions, Metadata, StyleStore};

use crate::arena::DocumentArena;
use crate::id::SectionId;
use crate::paragraph::Paragraph;
use crate::run::Run;

/// 문서
///
/// IR Document를 기반으로 한 편집 가능한 문서 모델입니다.
#[derive(Debug, Default)]
pub struct Document {
    /// 문서 메타데이터 (제목, 저자 등)
    pub metadata: Metadata,

    /// 문서 설정
    pub settings: DocumentSettings,

    /// 스타일 정의 (글자 모양, 문단 모양, 스타일 등)
    pub styles: StyleStore,

    /// 섹션 ID 목록 (순서 유지)
    pub sections: Vec<SectionId>,

    /// 바이너리 데이터 저장소 (이미지, OLE 등)
    pub binary_data: BinaryDataStore,

    /// 형식별 확장 데이터
    pub extensions: Extensions,

    /// 노드 저장소 (Arena)
    pub arena: DocumentArena,
}

impl Document {
    /// 빈 문서 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 섹션 추가
    pub fn add_section(&mut self, section: crate::Section) -> SectionId {
        let id = self.arena.insert_section(section);
        self.sections.push(id);
        id
    }

    /// 섹션 수 반환
    pub const fn section_count(&self) -> usize {
        self.sections.len()
    }

    /// 섹션 조회
    pub fn get_section(&self, id: SectionId) -> Option<&crate::Section> {
        self.arena.get_section(id)
    }

    /// 섹션 조회 (mutable)
    pub fn get_section_mut(&mut self, id: SectionId) -> Option<&mut crate::Section> {
        self.arena.get_section_mut(id)
    }

    /// 전체 텍스트 추출
    pub fn to_plain_text(&self) -> String {
        let mut text = String::new();
        for &section_id in &self.sections {
            if let Some(section) = self.arena.get_section(section_id) {
                for &para_id in &section.paragraphs {
                    if let Some(para) = self.arena.get_paragraph(para_id) {
                        if !text.is_empty() {
                            text.push('\n');
                        }
                        text.push_str(&self.paragraph_to_plain_text(para));
                    }
                }
            }
        }
        text
    }

    /// 문단을 일반 텍스트로 변환
    fn paragraph_to_plain_text(&self, para: &Paragraph) -> String {
        let mut text = String::new();
        for &run_id in &para.runs {
            if let Some(run) = self.arena.get_run(run_id) {
                text.push_str(&run.to_plain_text());
            }
        }
        text
    }

    /// 문단에 텍스트 런 추가 (편의 메서드)
    pub fn add_text_to_paragraph(&mut self, para_id: crate::ParagraphId, text: &str) {
        let run = Run::with_text(text);
        let run_id = self.arena.insert_run(run);
        if let Some(para) = self.arena.get_paragraph_mut(para_id) {
            para.add_run(run_id);
        }
    }
}

/// 문서 설정
#[derive(Debug, Clone, Default)]
pub struct DocumentSettings {
    /// 시작 페이지 번호
    pub starting_page_number: u32,

    /// 시작 각주 번호
    pub starting_footnote_number: u32,

    /// 시작 미주 번호
    pub starting_endnote_number: u32,

    /// 캐럿 위치 (마지막 편집 위치)
    pub caret_position: Option<CaretPosition>,
}

/// 캐럿 위치
#[derive(Debug, Clone, Default)]
pub struct CaretPosition {
    /// 섹션 인덱스
    pub section: u32,
    /// 문단 인덱스
    pub paragraph: u32,
    /// 문자 위치
    pub position: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Section;

    #[test]
    fn test_document_creation() {
        let doc = Document::new();
        assert_eq!(doc.section_count(), 0);
    }

    #[test]
    fn test_add_section() {
        let mut doc = Document::new();
        let section = Section::default();
        let id = doc.add_section(section);

        assert_eq!(doc.section_count(), 1);
        assert!(doc.get_section(id).is_some());
    }

    #[test]
    fn test_plain_text_extraction() {
        let mut doc = Document::new();

        // 섹션 추가
        let mut section = Section::default();

        // 문단 추가
        let para = Paragraph::new();
        let para_id = doc.arena.insert_paragraph(para);
        section.paragraphs.push(para_id);

        // 런 추가
        let run = Run::with_text("Hello, World!");
        let run_id = doc.arena.insert_run(run);
        doc.arena
            .get_paragraph_mut(para_id)
            .unwrap()
            .add_run(run_id);

        // 섹션을 문서에 추가
        doc.add_section(section);

        assert_eq!(doc.to_plain_text(), "Hello, World!");
    }
}
