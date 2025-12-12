//! 트랜잭션 시스템
//!
//! 여러 명령을 하나의 원자적 단위로 묶어 실행합니다.
//! 트랜잭션 내 명령이 하나라도 실패하면 전체가 롤백됩니다.

use crate::command::{Command, CommandResult};
use crate::Document;

/// 트랜잭션
///
/// 여러 명령을 원자적으로 실행합니다.
pub struct Transaction {
    /// 트랜잭션에 포함된 명령들
    commands: Vec<Box<dyn Command>>,
    /// 실행된 명령 수 (롤백용)
    executed_count: usize,
    /// 트랜잭션 설명
    description: String,
}

impl Transaction {
    /// 새 트랜잭션 생성
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            commands: Vec::new(),
            executed_count: 0,
            description: description.into(),
        }
    }

    /// 명령 추가
    pub fn add(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    /// 명령 추가 (빌더 패턴)
    pub fn with(mut self, command: Box<dyn Command>) -> Self {
        self.commands.push(command);
        self
    }

    /// 트랜잭션 실행
    ///
    /// 모든 명령을 순차적으로 실행합니다.
    /// 중간에 실패하면 이미 실행된 명령들을 롤백합니다.
    pub fn execute(&mut self, doc: &mut Document) -> CommandResult<()> {
        self.executed_count = 0;

        for command in &mut self.commands {
            match command.execute(doc) {
                Ok(()) => {
                    self.executed_count += 1;
                }
                Err(e) => {
                    // 롤백
                    self.rollback(doc)?;
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    /// 트랜잭션 롤백
    ///
    /// 실행된 명령들을 역순으로 취소합니다.
    fn rollback(&mut self, doc: &mut Document) -> CommandResult<()> {
        for i in (0..self.executed_count).rev() {
            if let Some(command) = self.commands.get_mut(i) {
                command.undo(doc)?;
            }
        }
        self.executed_count = 0;
        Ok(())
    }

    /// 명령 수
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// 비어있는지 확인
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

impl Command for Transaction {
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()> {
        Transaction::execute(self, doc)
    }

    fn undo(&mut self, doc: &mut Document) -> CommandResult<()> {
        // 역순으로 모든 명령 취소
        for command in self.commands.iter_mut().rev() {
            command.undo(doc)?;
        }
        self.executed_count = 0;
        Ok(())
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transaction")
            .field("command_count", &self.commands.len())
            .field("executed_count", &self.executed_count)
            .field("description", &self.description)
            .finish()
    }
}

/// 트랜잭션 빌더
pub struct TransactionBuilder {
    commands: Vec<Box<dyn Command>>,
    description: String,
}

impl TransactionBuilder {
    /// 새 빌더 생성
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            commands: Vec::new(),
            description: description.into(),
        }
    }

    /// 명령 추가
    pub fn with_command(mut self, command: impl Command + 'static) -> Self {
        self.commands.push(Box::new(command));
        self
    }

    /// 트랜잭션 빌드
    pub fn build(self) -> Transaction {
        Transaction {
            commands: self.commands,
            executed_count: 0,
            description: self.description,
        }
    }

    /// 빌드 및 실행
    pub fn execute(self, doc: &mut Document) -> CommandResult<Transaction> {
        let mut transaction = self.build();
        transaction.execute(doc)?;
        Ok(transaction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::{DeleteText, InsertParagraph, InsertText};
    use crate::cursor::Position;
    use crate::{Paragraph, Run, Section};

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
    fn test_transaction_execute() {
        let mut doc = create_test_document();

        let mut transaction = Transaction::new("Replace text")
            .with(Box::new(DeleteText::new(Position::new(0, 0, 0, 6), 5))) // "World" 삭제
            .with(Box::new(InsertText::new(Position::new(0, 0, 0, 6), "Rust"))); // "Rust" 삽입

        transaction.execute(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello Rust");
    }

    #[test]
    fn test_transaction_undo() {
        let mut doc = create_test_document();

        let mut transaction = Transaction::new("Replace text")
            .with(Box::new(DeleteText::new(Position::new(0, 0, 0, 6), 5)))
            .with(Box::new(InsertText::new(Position::new(0, 0, 0, 6), "Rust")));

        transaction.execute(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello Rust");

        transaction.undo(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello World");
    }

    #[test]
    fn test_transaction_builder() {
        let mut doc = create_test_document();

        let _transaction = TransactionBuilder::new("Add paragraph and text")
            .with_command(InsertParagraph::new(0, 1))
            .execute(&mut doc)
            .unwrap();

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
    fn test_transaction_with_history() {
        use crate::history::CommandHistory;

        let mut doc = create_test_document();
        let mut history = CommandHistory::new();

        let transaction = Transaction::new("Complex operation")
            .with(Box::new(InsertText::new(Position::new(0, 0, 0, 5), "!")))
            .with(Box::new(InsertText::new(Position::new(0, 0, 0, 0), "Say: ")));

        history.execute(Box::new(transaction), &mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Say: Hello! World");

        // 전체 트랜잭션이 하나의 Undo로 취소됨
        history.undo(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello World");
    }

    #[test]
    fn test_empty_transaction() {
        let mut doc = create_test_document();
        let mut transaction = Transaction::new("Empty");

        assert!(transaction.is_empty());
        transaction.execute(&mut doc).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello World");
    }
}
