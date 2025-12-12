//! 편집 명령 시스템
//!
//! 문서 편집 작업을 명령 패턴으로 구현합니다.
//! 각 명령은 실행과 취소가 가능합니다.

use crate::cursor::Position;
use crate::{Document, Paragraph, ParagraphId, Run, RunId};

/// 명령 실행 결과
pub type CommandResult<T> = Result<T, CommandError>;

/// 명령 에러
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    /// 위치가 유효하지 않음
    InvalidPosition,
    /// 섹션을 찾을 수 없음
    SectionNotFound,
    /// 문단을 찾을 수 없음
    ParagraphNotFound,
    /// 런을 찾을 수 없음
    RunNotFound,
    /// 선택 영역이 없음
    NoSelection,
    /// 기타 에러
    Other(String),
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::InvalidPosition => write!(f, "Invalid position"),
            CommandError::SectionNotFound => write!(f, "Section not found"),
            CommandError::ParagraphNotFound => write!(f, "Paragraph not found"),
            CommandError::RunNotFound => write!(f, "Run not found"),
            CommandError::NoSelection => write!(f, "No selection"),
            CommandError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for CommandError {}

/// 편집 명령 trait
pub trait Command {
    /// 명령 실행
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()>;
    /// 명령 취소 (역연산)
    fn undo(&mut self, doc: &mut Document) -> CommandResult<()>;
    /// 명령 설명
    fn description(&self) -> &str;
}

/// 텍스트 삽입 명령
#[derive(Debug, Clone)]
pub struct InsertText {
    /// 삽입 위치
    pub position: Position,
    /// 삽입할 텍스트
    pub text: String,
    /// 실행 후 영향 받은 런 ID (undo용)
    affected_run: Option<RunId>,
}

impl InsertText {
    /// 새 삽입 명령 생성
    pub fn new(position: Position, text: impl Into<String>) -> Self {
        Self {
            position,
            text: text.into(),
            affected_run: None,
        }
    }
}

impl Command for InsertText {
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()> {
        let section_id = doc
            .sections
            .get(self.position.section_index)
            .copied()
            .ok_or(CommandError::SectionNotFound)?;

        let section = doc
            .arena
            .get_section(section_id)
            .ok_or(CommandError::SectionNotFound)?;

        let para_id = section
            .paragraphs
            .get(self.position.paragraph_index)
            .copied()
            .ok_or(CommandError::ParagraphNotFound)?;

        let para = doc
            .arena
            .get_paragraph(para_id)
            .ok_or(CommandError::ParagraphNotFound)?;

        // 런이 없으면 새로 생성
        if para.runs.is_empty() {
            let run = Run::with_text(&self.text);
            let run_id = doc.arena.insert_run(run);
            let para = doc
                .arena
                .get_paragraph_mut(para_id)
                .ok_or(CommandError::ParagraphNotFound)?;
            para.runs.push(run_id);
            self.affected_run = Some(run_id);
            return Ok(());
        }

        let run_id = para
            .runs
            .get(self.position.run_index)
            .copied()
            .ok_or(CommandError::RunNotFound)?;

        self.affected_run = Some(run_id);

        // 런에 텍스트 삽입
        let run = doc
            .arena
            .get_run_mut(run_id)
            .ok_or(CommandError::RunNotFound)?;

        insert_text_at_offset(run, self.position.char_offset, &self.text);
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> CommandResult<()> {
        let run_id = self.affected_run.ok_or(CommandError::RunNotFound)?;

        let run = doc
            .arena
            .get_run_mut(run_id)
            .ok_or(CommandError::RunNotFound)?;

        delete_text_at_offset(run, self.position.char_offset, self.text.len());
        Ok(())
    }

    fn description(&self) -> &str {
        "Insert text"
    }
}

/// 텍스트 삭제 명령
#[derive(Debug, Clone)]
pub struct DeleteText {
    /// 삭제 시작 위치
    pub start: Position,
    /// 삭제할 길이
    pub length: usize,
    /// 삭제된 텍스트 (undo용)
    deleted_text: Option<String>,
}

impl DeleteText {
    /// 새 삭제 명령 생성
    pub const fn new(start: Position, length: usize) -> Self {
        Self {
            start,
            length,
            deleted_text: None,
        }
    }
}

impl Command for DeleteText {
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()> {
        let section_id = doc
            .sections
            .get(self.start.section_index)
            .copied()
            .ok_or(CommandError::SectionNotFound)?;

        let section = doc
            .arena
            .get_section(section_id)
            .ok_or(CommandError::SectionNotFound)?;

        let para_id = section
            .paragraphs
            .get(self.start.paragraph_index)
            .copied()
            .ok_or(CommandError::ParagraphNotFound)?;

        let para = doc
            .arena
            .get_paragraph(para_id)
            .ok_or(CommandError::ParagraphNotFound)?;

        let run_id = para
            .runs
            .get(self.start.run_index)
            .copied()
            .ok_or(CommandError::RunNotFound)?;

        let run = doc
            .arena
            .get_run_mut(run_id)
            .ok_or(CommandError::RunNotFound)?;

        // 삭제할 텍스트 저장
        self.deleted_text = Some(extract_text_at_offset(
            run,
            self.start.char_offset,
            self.length,
        ));

        // 텍스트 삭제
        delete_text_at_offset(run, self.start.char_offset, self.length);
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> CommandResult<()> {
        let deleted = self
            .deleted_text
            .as_ref()
            .ok_or(CommandError::Other("No deleted text stored".into()))?;

        let section_id = doc
            .sections
            .get(self.start.section_index)
            .copied()
            .ok_or(CommandError::SectionNotFound)?;

        let section = doc
            .arena
            .get_section(section_id)
            .ok_or(CommandError::SectionNotFound)?;

        let para_id = section
            .paragraphs
            .get(self.start.paragraph_index)
            .copied()
            .ok_or(CommandError::ParagraphNotFound)?;

        let para = doc
            .arena
            .get_paragraph(para_id)
            .ok_or(CommandError::ParagraphNotFound)?;

        let run_id = para
            .runs
            .get(self.start.run_index)
            .copied()
            .ok_or(CommandError::RunNotFound)?;

        let run = doc
            .arena
            .get_run_mut(run_id)
            .ok_or(CommandError::RunNotFound)?;

        insert_text_at_offset(run, self.start.char_offset, deleted);
        Ok(())
    }

    fn description(&self) -> &str {
        "Delete text"
    }
}

/// 문단 삽입 명령
#[derive(Debug, Clone)]
pub struct InsertParagraph {
    /// 삽입할 위치 (섹션 인덱스, 문단 인덱스)
    pub section_index: usize,
    pub paragraph_index: usize,
    /// 삽입된 문단 ID (undo용)
    inserted_para_id: Option<ParagraphId>,
}

impl InsertParagraph {
    /// 새 문단 삽입 명령 생성
    pub const fn new(section_index: usize, paragraph_index: usize) -> Self {
        Self {
            section_index,
            paragraph_index,
            inserted_para_id: None,
        }
    }
}

impl Command for InsertParagraph {
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()> {
        let section_id = doc
            .sections
            .get(self.section_index)
            .copied()
            .ok_or(CommandError::SectionNotFound)?;

        // 새 문단 생성
        let para = Paragraph::new();
        let para_id = doc.arena.insert_paragraph(para);
        self.inserted_para_id = Some(para_id);

        // 섹션에 문단 추가
        let section = doc
            .arena
            .get_section_mut(section_id)
            .ok_or(CommandError::SectionNotFound)?;

        let idx = self.paragraph_index.min(section.paragraphs.len());
        section.paragraphs.insert(idx, para_id);

        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> CommandResult<()> {
        let para_id = self
            .inserted_para_id
            .ok_or(CommandError::ParagraphNotFound)?;

        let section_id = doc
            .sections
            .get(self.section_index)
            .copied()
            .ok_or(CommandError::SectionNotFound)?;

        let section = doc
            .arena
            .get_section_mut(section_id)
            .ok_or(CommandError::SectionNotFound)?;

        // 문단 제거
        section.paragraphs.retain(|&id| id != para_id);
        doc.arena.remove_paragraph(para_id);

        Ok(())
    }

    fn description(&self) -> &str {
        "Insert paragraph"
    }
}

/// 문단 삭제 명령
#[derive(Debug, Clone)]
pub struct DeleteParagraph {
    /// 삭제할 위치 (섹션 인덱스, 문단 인덱스)
    pub section_index: usize,
    pub paragraph_index: usize,
    /// 삭제된 문단 (undo용)
    deleted_para: Option<Paragraph>,
    deleted_para_id: Option<ParagraphId>,
}

impl DeleteParagraph {
    /// 새 문단 삭제 명령 생성
    pub const fn new(section_index: usize, paragraph_index: usize) -> Self {
        Self {
            section_index,
            paragraph_index,
            deleted_para: None,
            deleted_para_id: None,
        }
    }
}

impl Command for DeleteParagraph {
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()> {
        let section_id = doc
            .sections
            .get(self.section_index)
            .copied()
            .ok_or(CommandError::SectionNotFound)?;

        let section = doc
            .arena
            .get_section(section_id)
            .ok_or(CommandError::SectionNotFound)?;

        let para_id = section
            .paragraphs
            .get(self.paragraph_index)
            .copied()
            .ok_or(CommandError::ParagraphNotFound)?;

        // 문단 저장 (undo용)
        if let Some(para) = doc.arena.get_paragraph(para_id) {
            self.deleted_para = Some(para.clone());
        }
        self.deleted_para_id = Some(para_id);

        // 섹션에서 문단 제거
        let section = doc
            .arena
            .get_section_mut(section_id)
            .ok_or(CommandError::SectionNotFound)?;
        section.paragraphs.remove(self.paragraph_index);

        // Arena에서 문단 제거
        doc.arena.remove_paragraph(para_id);

        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> CommandResult<()> {
        let para = self
            .deleted_para
            .take()
            .ok_or(CommandError::ParagraphNotFound)?;

        let section_id = doc
            .sections
            .get(self.section_index)
            .copied()
            .ok_or(CommandError::SectionNotFound)?;

        // 문단 복원
        let para_id = doc.arena.insert_paragraph(para);

        let section = doc
            .arena
            .get_section_mut(section_id)
            .ok_or(CommandError::SectionNotFound)?;

        let idx = self.paragraph_index.min(section.paragraphs.len());
        section.paragraphs.insert(idx, para_id);

        Ok(())
    }

    fn description(&self) -> &str {
        "Delete paragraph"
    }
}

// 헬퍼 함수들

/// 런의 특정 오프셋에 텍스트 삽입
fn insert_text_at_offset(run: &mut Run, offset: usize, text: &str) {
    use crate::run_content::RunContent;

    let mut current_offset = 0;

    for content in &mut run.contents {
        if let RunContent::Text(s) = content {
            let len = s.chars().count();
            if offset >= current_offset && offset <= current_offset + len {
                let byte_offset = s
                    .char_indices()
                    .nth(offset - current_offset)
                    .map(|(i, _)| i)
                    .unwrap_or(s.len());
                s.insert_str(byte_offset, text);
                return;
            }
            current_offset += len;
        } else {
            current_offset += 1;
        }
    }

    // 오프셋이 끝에 있으면 새 텍스트 콘텐츠 추가
    run.contents.push(RunContent::Text(text.to_string()));
}

/// 런의 특정 오프셋에서 텍스트 삭제
fn delete_text_at_offset(run: &mut Run, offset: usize, length: usize) {
    use crate::run_content::RunContent;

    let mut current_offset = 0;
    let mut to_remove = length;

    for content in &mut run.contents {
        if to_remove == 0 {
            break;
        }

        if let RunContent::Text(s) = content {
            let len = s.chars().count();
            if offset < current_offset + len {
                let start = offset.saturating_sub(current_offset);
                let end = (start + to_remove).min(len);
                let remove_count = end - start;

                let byte_start = s
                    .char_indices()
                    .nth(start)
                    .map(|(i, _)| i)
                    .unwrap_or(s.len());
                let byte_end = s.char_indices().nth(end).map(|(i, _)| i).unwrap_or(s.len());

                s.drain(byte_start..byte_end);
                to_remove -= remove_count;
            }
            current_offset += len;
        } else {
            current_offset += 1;
        }
    }
}

/// 런의 특정 오프셋에서 텍스트 추출
fn extract_text_at_offset(run: &Run, offset: usize, length: usize) -> String {
    use crate::run_content::RunContent;

    let mut result = String::new();
    let mut current_offset = 0;
    let mut remaining = length;

    for content in &run.contents {
        if remaining == 0 {
            break;
        }

        if let RunContent::Text(s) = content {
            let len = s.chars().count();
            if offset < current_offset + len {
                let start = offset.saturating_sub(current_offset);
                let end = (start + remaining).min(len);

                let extracted: String = s.chars().skip(start).take(end - start).collect();
                result.push_str(&extracted);
                remaining -= end - start;
            }
            current_offset += len;
        } else {
            current_offset += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Section;

    fn create_test_document() -> Document {
        let mut doc = Document::new();
        let mut section = Section::default();

        let mut para = Paragraph::new();
        let run = Run::with_text("Hello World");
        let run_id = doc.arena.insert_run(run);
        para.add_run(run_id);

        let para_id = doc.arena.insert_paragraph(para);
        section.paragraphs.push(para_id);

        doc.add_section(section);
        doc
    }

    #[test]
    fn test_insert_text() {
        let mut doc = create_test_document();
        let mut cmd = InsertText::new(Position::new(0, 0, 0, 5), " Beautiful");

        cmd.execute(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello Beautiful World");

        cmd.undo(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello World");
    }

    #[test]
    fn test_delete_text() {
        let mut doc = create_test_document();
        let mut cmd = DeleteText::new(Position::new(0, 0, 0, 5), 6); // " World"

        cmd.execute(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello");

        cmd.undo(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello World");
    }

    #[test]
    fn test_insert_paragraph() {
        let mut doc = create_test_document();
        let mut cmd = InsertParagraph::new(0, 1);

        cmd.execute(&mut doc).unwrap();
        assert_eq!(
            doc.arena
                .get_section(doc.sections[0])
                .unwrap()
                .paragraphs
                .len(),
            2
        );

        cmd.undo(&mut doc).unwrap();
        assert_eq!(
            doc.arena
                .get_section(doc.sections[0])
                .unwrap()
                .paragraphs
                .len(),
            1
        );
    }

    #[test]
    fn test_delete_paragraph() {
        let mut doc = create_test_document();

        // 먼저 두 번째 문단 추가
        let mut insert_cmd = InsertParagraph::new(0, 1);
        insert_cmd.execute(&mut doc).unwrap();

        // 두 번째 문단 삭제
        let mut delete_cmd = DeleteParagraph::new(0, 1);
        delete_cmd.execute(&mut doc).unwrap();
        assert_eq!(
            doc.arena
                .get_section(doc.sections[0])
                .unwrap()
                .paragraphs
                .len(),
            1
        );

        delete_cmd.undo(&mut doc).unwrap();
        assert_eq!(
            doc.arena
                .get_section(doc.sections[0])
                .unwrap()
                .paragraphs
                .len(),
            2
        );
    }

    #[test]
    fn test_insert_into_empty_paragraph() {
        let mut doc = Document::new();
        let mut section = Section::default();
        let para = Paragraph::new();
        let para_id = doc.arena.insert_paragraph(para);
        section.paragraphs.push(para_id);
        doc.add_section(section);

        let mut cmd = InsertText::new(Position::new(0, 0, 0, 0), "New text");
        cmd.execute(&mut doc).unwrap();

        assert_eq!(doc.to_plain_text(), "New text");
    }
}
