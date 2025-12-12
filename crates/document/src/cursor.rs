//! 커서 시스템
//!
//! 문서 내 위치 추적 및 선택 영역을 관리합니다.

/// 문서 내 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    /// 섹션 인덱스
    pub section_index: usize,
    /// 문단 인덱스 (섹션 내)
    pub paragraph_index: usize,
    /// 런 인덱스 (문단 내)
    pub run_index: usize,
    /// 문자 오프셋 (런 내)
    pub char_offset: usize,
}

impl Position {
    /// 새 위치 생성
    pub const fn new(section: usize, paragraph: usize, run: usize, offset: usize) -> Self {
        Self {
            section_index: section,
            paragraph_index: paragraph,
            run_index: run,
            char_offset: offset,
        }
    }

    /// 문서 시작 위치
    pub fn start() -> Self {
        Self::default()
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.section_index
            .cmp(&other.section_index)
            .then(self.paragraph_index.cmp(&other.paragraph_index))
            .then(self.run_index.cmp(&other.run_index))
            .then(self.char_offset.cmp(&other.char_offset))
    }
}

/// 커서
///
/// 문서 내 현재 편집 위치를 나타냅니다.
#[derive(Debug, Clone, Default)]
pub struct Cursor {
    /// 현재 위치
    position: Position,
    /// 선택 시작 위치 (None이면 선택 없음)
    anchor: Option<Position>,
}

impl Cursor {
    /// 새 커서 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 특정 위치에 커서 생성
    pub const fn at(position: Position) -> Self {
        Self {
            position,
            anchor: None,
        }
    }

    /// 현재 위치 반환
    pub const fn position(&self) -> Position {
        self.position
    }

    /// 위치 설정
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
        self.anchor = None; // 선택 해제
    }

    /// 선택 시작
    pub fn start_selection(&mut self) {
        self.anchor = Some(self.position);
    }

    /// 선택 영역 확장 (위치 이동하면서 선택)
    pub fn extend_selection(&mut self, position: Position) {
        if self.anchor.is_none() {
            self.anchor = Some(self.position);
        }
        self.position = position;
    }

    /// 선택 해제
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }

    /// 선택 영역 반환
    pub fn selection(&self) -> Option<Selection> {
        self.anchor.map(|anchor| {
            if anchor <= self.position {
                Selection {
                    start: anchor,
                    end: self.position,
                }
            } else {
                Selection {
                    start: self.position,
                    end: anchor,
                }
            }
        })
    }

    /// 선택 영역이 있는지 확인
    pub fn has_selection(&self) -> bool {
        self.anchor.is_some() && self.anchor != Some(self.position)
    }

    /// 커서를 문자 단위로 이동
    pub fn move_char(&mut self, delta: i32, doc: &crate::Document) {
        if delta > 0 {
            for _ in 0..delta {
                self.move_forward_char(doc);
            }
        } else {
            for _ in 0..(-delta) {
                self.move_backward_char(doc);
            }
        }
    }

    /// 앞으로 한 문자 이동
    fn move_forward_char(&mut self, doc: &crate::Document) {
        let Some(section_id) = doc.sections.get(self.position.section_index).copied() else {
            return;
        };
        let Some(section) = doc.arena.get_section(section_id) else {
            return;
        };
        let Some(&para_id) = section.paragraphs.get(self.position.paragraph_index) else {
            return;
        };
        let Some(para) = doc.arena.get_paragraph(para_id) else {
            return;
        };

        // 현재 런에서 이동 가능한지 확인
        if let Some(run) = para
            .runs
            .get(self.position.run_index)
            .and_then(|&run_id| doc.arena.get_run(run_id))
        {
            let run_len = run.text_length();
            if self.position.char_offset < run_len {
                self.position.char_offset += 1;
                return;
            }
        }

        // 다음 런으로 이동
        if self.position.run_index + 1 < para.runs.len() {
            self.position.run_index += 1;
            self.position.char_offset = 0;
            return;
        }

        // 다음 문단으로 이동
        if self.position.paragraph_index + 1 < section.paragraphs.len() {
            self.position.paragraph_index += 1;
            self.position.run_index = 0;
            self.position.char_offset = 0;
            return;
        }

        // 다음 섹션으로 이동
        if self.position.section_index + 1 < doc.sections.len() {
            self.position.section_index += 1;
            self.position.paragraph_index = 0;
            self.position.run_index = 0;
            self.position.char_offset = 0;
        }
    }

    /// 뒤로 한 문자 이동
    fn move_backward_char(&mut self, doc: &crate::Document) {
        // 현재 런에서 이동 가능한지 확인
        if self.position.char_offset > 0 {
            self.position.char_offset -= 1;
            return;
        }

        // 이전 런으로 이동
        if self.position.run_index > 0 {
            self.position.run_index -= 1;
            // 이전 런의 끝으로 이동
            if let Some(run_len) = doc
                .sections
                .get(self.position.section_index)
                .copied()
                .and_then(|section_id| doc.arena.get_section(section_id))
                .and_then(|section| {
                    section
                        .paragraphs
                        .get(self.position.paragraph_index)
                        .copied()
                })
                .and_then(|para_id| doc.arena.get_paragraph(para_id))
                .and_then(|para| para.runs.get(self.position.run_index).copied())
                .and_then(|run_id| doc.arena.get_run(run_id))
                .map(|run| run.text_length())
            {
                self.position.char_offset = run_len;
            }
            return;
        }

        // 이전 문단으로 이동
        if self.position.paragraph_index > 0 {
            self.position.paragraph_index -= 1;
            if let Some(para) = doc
                .sections
                .get(self.position.section_index)
                .copied()
                .and_then(|section_id| doc.arena.get_section(section_id))
                .and_then(|section| {
                    section
                        .paragraphs
                        .get(self.position.paragraph_index)
                        .copied()
                })
                .and_then(|para_id| doc.arena.get_paragraph(para_id))
            {
                self.position.run_index = para.runs.len().saturating_sub(1);
                if let Some(run_len) = para
                    .runs
                    .get(self.position.run_index)
                    .copied()
                    .and_then(|run_id| doc.arena.get_run(run_id))
                    .map(|run| run.text_length())
                {
                    self.position.char_offset = run_len;
                }
            }
            return;
        }

        // 이전 섹션으로 이동
        if self.position.section_index > 0 {
            self.position.section_index -= 1;
            if let Some(section) = doc
                .sections
                .get(self.position.section_index)
                .copied()
                .and_then(|section_id| doc.arena.get_section(section_id))
            {
                self.position.paragraph_index = section.paragraphs.len().saturating_sub(1);
                if let Some(para) = section
                    .paragraphs
                    .get(self.position.paragraph_index)
                    .copied()
                    .and_then(|para_id| doc.arena.get_paragraph(para_id))
                {
                    self.position.run_index = para.runs.len().saturating_sub(1);
                    if let Some(run_len) = para
                        .runs
                        .get(self.position.run_index)
                        .copied()
                        .and_then(|run_id| doc.arena.get_run(run_id))
                        .map(|run| run.text_length())
                    {
                        self.position.char_offset = run_len;
                    }
                }
            }
        }
    }

    /// 문단 시작으로 이동
    pub fn move_to_paragraph_start(&mut self) {
        self.position.run_index = 0;
        self.position.char_offset = 0;
    }

    /// 문단 끝으로 이동
    pub fn move_to_paragraph_end(&mut self, doc: &crate::Document) {
        if let Some(para) = doc
            .sections
            .get(self.position.section_index)
            .copied()
            .and_then(|section_id| doc.arena.get_section(section_id))
            .and_then(|section| {
                section
                    .paragraphs
                    .get(self.position.paragraph_index)
                    .copied()
            })
            .and_then(|para_id| doc.arena.get_paragraph(para_id))
        {
            self.position.run_index = para.runs.len().saturating_sub(1);
            if let Some(run_len) = para
                .runs
                .get(self.position.run_index)
                .copied()
                .and_then(|run_id| doc.arena.get_run(run_id))
                .map(|run| run.text_length())
            {
                self.position.char_offset = run_len;
            }
        }
    }

    /// 문서 시작으로 이동
    pub fn move_to_document_start(&mut self) {
        self.position = Position::start();
    }

    /// 문서 끝으로 이동
    pub fn move_to_document_end(&mut self, doc: &crate::Document) {
        if doc.sections.is_empty() {
            self.position = Position::start();
            return;
        }

        self.position.section_index = doc.sections.len() - 1;
        if let Some(section) = doc
            .sections
            .get(self.position.section_index)
            .copied()
            .and_then(|section_id| doc.arena.get_section(section_id))
        {
            self.position.paragraph_index = section.paragraphs.len().saturating_sub(1);
            self.move_to_paragraph_end(doc);
        }
    }
}

/// 선택 영역
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// 시작 위치
    pub start: Position,
    /// 끝 위치
    pub end: Position,
}

impl Selection {
    /// 새 선택 영역 생성
    pub fn new(start: Position, end: Position) -> Self {
        if start <= end {
            Self { start, end }
        } else {
            Self {
                start: end,
                end: start,
            }
        }
    }

    /// 선택 영역이 비어있는지 확인
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// 위치가 선택 영역에 포함되는지 확인
    pub fn contains(&self, position: Position) -> bool {
        position >= self.start && position <= self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Document, Paragraph, Run, Section};

    fn create_test_document() -> Document {
        let mut doc = Document::new();

        // 섹션 추가
        let mut section = Section::default();

        // 첫 번째 문단
        let mut para1 = Paragraph::new();
        let run1 = Run::with_text("Hello ");
        let run1_id = doc.arena.insert_run(run1);
        para1.add_run(run1_id);

        let run2 = Run::with_text("World");
        let run2_id = doc.arena.insert_run(run2);
        para1.add_run(run2_id);

        let para1_id = doc.arena.insert_paragraph(para1);
        section.paragraphs.push(para1_id);

        // 두 번째 문단
        let mut para2 = Paragraph::new();
        let run3 = Run::with_text("Second paragraph");
        let run3_id = doc.arena.insert_run(run3);
        para2.add_run(run3_id);

        let para2_id = doc.arena.insert_paragraph(para2);
        section.paragraphs.push(para2_id);

        doc.add_section(section);
        doc
    }

    #[test]
    fn test_cursor_creation() {
        let cursor = Cursor::new();
        assert_eq!(cursor.position(), Position::start());
        assert!(!cursor.has_selection());
    }

    #[test]
    fn test_cursor_at_position() {
        let pos = Position::new(0, 1, 0, 5);
        let cursor = Cursor::at(pos);
        assert_eq!(cursor.position(), pos);
    }

    #[test]
    fn test_cursor_selection() {
        let mut cursor = Cursor::new();
        cursor.start_selection();
        cursor.extend_selection(Position::new(0, 0, 1, 3));

        assert!(cursor.has_selection());

        let selection = cursor.selection().unwrap();
        assert_eq!(selection.start, Position::start());
        assert_eq!(selection.end, Position::new(0, 0, 1, 3));
    }

    #[test]
    fn test_cursor_move_forward() {
        let doc = create_test_document();
        let mut cursor = Cursor::new();

        // "Hello " 내에서 이동
        cursor.move_char(3, &doc);
        assert_eq!(cursor.position().char_offset, 3);

        // 다음 런으로 이동
        cursor.move_char(4, &doc); // "llo " + 1 = 다음 런
        assert_eq!(cursor.position().run_index, 1);
    }

    #[test]
    fn test_cursor_move_backward() {
        let doc = create_test_document();
        let mut cursor = Cursor::at(Position::new(0, 0, 1, 2));

        cursor.move_char(-3, &doc);
        assert_eq!(cursor.position().run_index, 0);
    }

    #[test]
    fn test_cursor_paragraph_navigation() {
        let doc = create_test_document();
        let mut cursor = Cursor::new();

        cursor.move_to_paragraph_end(&doc);
        assert_eq!(cursor.position().run_index, 1); // 두 번째 런
        assert_eq!(cursor.position().char_offset, 5); // "World" 끝

        cursor.move_to_paragraph_start();
        assert_eq!(cursor.position().run_index, 0);
        assert_eq!(cursor.position().char_offset, 0);
    }

    #[test]
    fn test_cursor_document_navigation() {
        let doc = create_test_document();
        let mut cursor = Cursor::new();

        cursor.move_to_document_end(&doc);
        assert_eq!(cursor.position().paragraph_index, 1);

        cursor.move_to_document_start();
        assert_eq!(cursor.position(), Position::start());
    }

    #[test]
    fn test_selection_contains() {
        let selection = Selection::new(Position::new(0, 0, 0, 2), Position::new(0, 0, 1, 3));

        assert!(selection.contains(Position::new(0, 0, 0, 5)));
        assert!(selection.contains(Position::new(0, 0, 1, 0)));
        assert!(!selection.contains(Position::new(0, 0, 0, 1)));
        assert!(!selection.contains(Position::new(0, 0, 1, 5)));
    }

    #[test]
    fn test_position_ordering() {
        let p1 = Position::new(0, 0, 0, 0);
        let p2 = Position::new(0, 0, 0, 5);
        let p3 = Position::new(0, 0, 1, 0);
        let p4 = Position::new(0, 1, 0, 0);
        let p5 = Position::new(1, 0, 0, 0);

        assert!(p1 < p2);
        assert!(p2 < p3);
        assert!(p3 < p4);
        assert!(p4 < p5);
    }
}
