//! 명령 히스토리 (Undo/Redo)
//!
//! 편집 명령의 실행 이력을 관리하여 Undo/Redo 기능을 제공합니다.

use crate::command::{Command, CommandResult};
use crate::Document;

/// 명령 히스토리
///
/// Undo/Redo 스택을 관리합니다.
#[derive(Default)]
pub struct CommandHistory {
    /// Undo 스택
    undo_stack: Vec<Box<dyn Command>>,
    /// Redo 스택
    redo_stack: Vec<Box<dyn Command>>,
    /// 최대 히스토리 크기
    max_size: usize,
}

impl CommandHistory {
    /// 새 히스토리 생성
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: 100,
        }
    }

    /// 최대 크기 설정
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
        }
    }

    /// 명령 실행 및 히스토리에 추가
    pub fn execute(&mut self, mut command: Box<dyn Command>, doc: &mut Document) -> CommandResult<()> {
        command.execute(doc)?;
        self.undo_stack.push(command);
        self.redo_stack.clear(); // 새 명령 실행 시 Redo 스택 초기화

        // 최대 크기 초과 시 오래된 항목 제거
        while self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }

        Ok(())
    }

    /// Undo 실행
    pub fn undo(&mut self, doc: &mut Document) -> CommandResult<bool> {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo(doc)?;
            self.redo_stack.push(command);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Redo 실행
    pub fn redo(&mut self, doc: &mut Document) -> CommandResult<bool> {
        if let Some(mut command) = self.redo_stack.pop() {
            command.execute(doc)?;
            self.undo_stack.push(command);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Undo 가능 여부
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Redo 가능 여부
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Undo 스택 크기
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Redo 스택 크기
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// 히스토리 초기화
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// 마지막 명령 설명 (Undo용)
    pub fn last_undo_description(&self) -> Option<&str> {
        self.undo_stack.last().map(|cmd| cmd.description())
    }

    /// 마지막 명령 설명 (Redo용)
    pub fn last_redo_description(&self) -> Option<&str> {
        self.redo_stack.last().map(|cmd| cmd.description())
    }
}

impl std::fmt::Debug for CommandHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandHistory")
            .field("undo_count", &self.undo_stack.len())
            .field("redo_count", &self.redo_stack.len())
            .field("max_size", &self.max_size)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::InsertText;
    use crate::cursor::Position;
    use crate::{Paragraph, Run, Section};

    fn create_test_document() -> Document {
        let mut doc = Document::new();
        let mut section = Section::default();

        let mut para = Paragraph::new();
        let run = Run::with_text("Hello");
        let run_id = doc.arena.insert_run(run);
        para.add_run(run_id);

        let para_id = doc.arena.insert_paragraph(para);
        section.paragraphs.push(para_id);

        doc.add_section(section);
        doc
    }

    #[test]
    fn test_execute_and_undo() {
        let mut doc = create_test_document();
        let mut history = CommandHistory::new();

        let cmd = InsertText::new(Position::new(0, 0, 0, 5), " World");
        history.execute(Box::new(cmd), &mut doc).unwrap();

        assert_eq!(doc.to_plain_text(), "Hello World");
        assert!(history.can_undo());
        assert!(!history.can_redo());

        history.undo(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello");
        assert!(!history.can_undo());
        assert!(history.can_redo());
    }

    #[test]
    fn test_redo() {
        let mut doc = create_test_document();
        let mut history = CommandHistory::new();

        let cmd = InsertText::new(Position::new(0, 0, 0, 5), " World");
        history.execute(Box::new(cmd), &mut doc).unwrap();
        history.undo(&mut doc).unwrap();

        assert_eq!(doc.to_plain_text(), "Hello");

        history.redo(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello World");
    }

    #[test]
    fn test_redo_cleared_on_new_command() {
        let mut doc = create_test_document();
        let mut history = CommandHistory::new();

        // 첫 번째 명령
        let cmd1 = InsertText::new(Position::new(0, 0, 0, 5), " World");
        history.execute(Box::new(cmd1), &mut doc).unwrap();

        // Undo
        history.undo(&mut doc).unwrap();
        assert!(history.can_redo());

        // 새 명령 실행 -> Redo 스택 초기화
        let cmd2 = InsertText::new(Position::new(0, 0, 0, 5), "!");
        history.execute(Box::new(cmd2), &mut doc).unwrap();

        assert!(!history.can_redo());
        assert_eq!(doc.to_plain_text(), "Hello!");
    }

    #[test]
    fn test_multiple_undo_redo() {
        let mut doc = create_test_document();
        let mut history = CommandHistory::new();

        // 여러 명령 실행
        let cmd1 = InsertText::new(Position::new(0, 0, 0, 5), " World");
        history.execute(Box::new(cmd1), &mut doc).unwrap();

        let cmd2 = InsertText::new(Position::new(0, 0, 0, 11), "!");
        history.execute(Box::new(cmd2), &mut doc).unwrap();

        assert_eq!(doc.to_plain_text(), "Hello World!");
        assert_eq!(history.undo_count(), 2);

        // 두 번 Undo
        history.undo(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello World");

        history.undo(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello");

        // 한 번 Redo
        history.redo(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello World");
    }

    #[test]
    fn test_max_size() {
        let mut doc = create_test_document();
        let mut history = CommandHistory::with_max_size(3);

        for i in 0..5 {
            let cmd = InsertText::new(Position::new(0, 0, 0, 5 + i), &i.to_string());
            history.execute(Box::new(cmd), &mut doc).unwrap();
        }

        // 최대 3개만 유지
        assert_eq!(history.undo_count(), 3);
    }

    #[test]
    fn test_description() {
        let mut doc = create_test_document();
        let mut history = CommandHistory::new();

        let cmd = InsertText::new(Position::new(0, 0, 0, 5), " World");
        history.execute(Box::new(cmd), &mut doc).unwrap();

        assert_eq!(history.last_undo_description(), Some("Insert text"));
    }
}
