//! Arena 저장소
//!
//! 문서의 모든 노드를 저장하는 중앙 저장소입니다.
//! SlotMap을 사용하여 안정적인 ID 기반 접근을 제공합니다.

use slotmap::SlotMap;

use crate::id::{CellId, ControlId, HeaderFooterId, ParagraphId, RowId, RunId, SectionId};

// 전방 선언 (실제 타입은 각 모듈에서 정의)
use crate::control::Control;
use crate::paragraph::Paragraph;
use crate::run::Run;
use crate::section::Section;
use crate::table::{TableCell, TableRow};

/// 머리글/바닥글
#[derive(Debug, Clone)]
pub struct HeaderFooter {
    /// 적용 대상
    pub apply_to: HeaderFooterApplyTo,
    /// 문단 목록
    pub paragraphs: Vec<ParagraphId>,
}

/// 머리글/바닥글 적용 대상
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HeaderFooterApplyTo {
    /// 양쪽 페이지
    #[default]
    Both,
    /// 짝수 페이지
    Even,
    /// 홀수 페이지
    Odd,
}

/// 문서 Arena 저장소
///
/// 문서의 모든 노드를 중앙에서 관리합니다.
#[derive(Debug, Default)]
pub struct DocumentArena {
    /// 섹션 저장소
    pub sections: SlotMap<SectionId, Section>,
    /// 문단 저장소
    pub paragraphs: SlotMap<ParagraphId, Paragraph>,
    /// 런 저장소
    pub runs: SlotMap<RunId, Run>,
    /// 컨트롤 저장소
    pub controls: SlotMap<ControlId, Control>,
    /// 표 행 저장소
    pub rows: SlotMap<RowId, TableRow>,
    /// 표 셀 저장소
    pub cells: SlotMap<CellId, TableCell>,
    /// 머리글/바닥글 저장소
    pub header_footers: SlotMap<HeaderFooterId, HeaderFooter>,
}

impl DocumentArena {
    /// 새 Arena 생성
    pub fn new() -> Self {
        Self::default()
    }

    // === Section API ===

    /// 섹션 추가
    pub fn insert_section(&mut self, section: Section) -> SectionId {
        self.sections.insert(section)
    }

    /// 섹션 조회
    pub fn get_section(&self, id: SectionId) -> Option<&Section> {
        self.sections.get(id)
    }

    /// 섹션 조회 (mutable)
    pub fn get_section_mut(&mut self, id: SectionId) -> Option<&mut Section> {
        self.sections.get_mut(id)
    }

    /// 섹션 삭제
    pub fn remove_section(&mut self, id: SectionId) -> Option<Section> {
        self.sections.remove(id)
    }

    // === Paragraph API ===

    /// 문단 추가
    pub fn insert_paragraph(&mut self, paragraph: Paragraph) -> ParagraphId {
        self.paragraphs.insert(paragraph)
    }

    /// 문단 조회
    pub fn get_paragraph(&self, id: ParagraphId) -> Option<&Paragraph> {
        self.paragraphs.get(id)
    }

    /// 문단 조회 (mutable)
    pub fn get_paragraph_mut(&mut self, id: ParagraphId) -> Option<&mut Paragraph> {
        self.paragraphs.get_mut(id)
    }

    /// 문단 삭제
    pub fn remove_paragraph(&mut self, id: ParagraphId) -> Option<Paragraph> {
        self.paragraphs.remove(id)
    }

    // === Run API ===

    /// 런 추가
    pub fn insert_run(&mut self, run: Run) -> RunId {
        self.runs.insert(run)
    }

    /// 런 조회
    pub fn get_run(&self, id: RunId) -> Option<&Run> {
        self.runs.get(id)
    }

    /// 런 조회 (mutable)
    pub fn get_run_mut(&mut self, id: RunId) -> Option<&mut Run> {
        self.runs.get_mut(id)
    }

    /// 런 삭제
    pub fn remove_run(&mut self, id: RunId) -> Option<Run> {
        self.runs.remove(id)
    }

    // === Control API ===

    /// 컨트롤 추가
    pub fn insert_control(&mut self, control: Control) -> ControlId {
        self.controls.insert(control)
    }

    /// 컨트롤 조회
    pub fn get_control(&self, id: ControlId) -> Option<&Control> {
        self.controls.get(id)
    }

    /// 컨트롤 조회 (mutable)
    pub fn get_control_mut(&mut self, id: ControlId) -> Option<&mut Control> {
        self.controls.get_mut(id)
    }

    /// 컨트롤 삭제
    pub fn remove_control(&mut self, id: ControlId) -> Option<Control> {
        self.controls.remove(id)
    }

    // === Row API ===

    /// 표 행 추가
    pub fn insert_row(&mut self, row: TableRow) -> RowId {
        self.rows.insert(row)
    }

    /// 표 행 조회
    pub fn get_row(&self, id: RowId) -> Option<&TableRow> {
        self.rows.get(id)
    }

    /// 표 행 조회 (mutable)
    pub fn get_row_mut(&mut self, id: RowId) -> Option<&mut TableRow> {
        self.rows.get_mut(id)
    }

    /// 표 행 삭제
    pub fn remove_row(&mut self, id: RowId) -> Option<TableRow> {
        self.rows.remove(id)
    }

    // === Cell API ===

    /// 표 셀 추가
    pub fn insert_cell(&mut self, cell: TableCell) -> CellId {
        self.cells.insert(cell)
    }

    /// 표 셀 조회
    pub fn get_cell(&self, id: CellId) -> Option<&TableCell> {
        self.cells.get(id)
    }

    /// 표 셀 조회 (mutable)
    pub fn get_cell_mut(&mut self, id: CellId) -> Option<&mut TableCell> {
        self.cells.get_mut(id)
    }

    /// 표 셀 삭제
    pub fn remove_cell(&mut self, id: CellId) -> Option<TableCell> {
        self.cells.remove(id)
    }

    // === HeaderFooter API ===

    /// 머리글/바닥글 추가
    pub fn insert_header_footer(&mut self, hf: HeaderFooter) -> HeaderFooterId {
        self.header_footers.insert(hf)
    }

    /// 머리글/바닥글 조회
    pub fn get_header_footer(&self, id: HeaderFooterId) -> Option<&HeaderFooter> {
        self.header_footers.get(id)
    }

    /// 머리글/바닥글 조회 (mutable)
    pub fn get_header_footer_mut(&mut self, id: HeaderFooterId) -> Option<&mut HeaderFooter> {
        self.header_footers.get_mut(id)
    }

    /// 머리글/바닥글 삭제
    pub fn remove_header_footer(&mut self, id: HeaderFooterId) -> Option<HeaderFooter> {
        self.header_footers.remove(id)
    }

    // === Utility ===

    /// 전체 노드 수
    pub fn total_count(&self) -> usize {
        self.sections.len()
            + self.paragraphs.len()
            + self.runs.len()
            + self.controls.len()
            + self.rows.len()
            + self.cells.len()
            + self.header_footers.len()
    }

    /// Arena 비우기
    pub fn clear(&mut self) {
        self.sections.clear();
        self.paragraphs.clear();
        self.runs.clear();
        self.controls.clear();
        self.rows.clear();
        self.cells.clear();
        self.header_footers.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_creation() {
        let arena = DocumentArena::new();
        assert_eq!(arena.total_count(), 0);
    }

    #[test]
    fn test_section_crud() {
        let mut arena = DocumentArena::new();

        // Insert
        let section = Section::default();
        let id = arena.insert_section(section);

        // Get
        assert!(arena.get_section(id).is_some());
        assert!(arena.get_section_mut(id).is_some());

        // Remove
        let removed = arena.remove_section(id);
        assert!(removed.is_some());
        assert!(arena.get_section(id).is_none());
    }

    #[test]
    fn test_paragraph_crud() {
        let mut arena = DocumentArena::new();

        let paragraph = Paragraph::default();
        let id = arena.insert_paragraph(paragraph);

        assert!(arena.get_paragraph(id).is_some());

        let removed = arena.remove_paragraph(id);
        assert!(removed.is_some());
        assert!(arena.get_paragraph(id).is_none());
    }

    #[test]
    fn test_nonexistent_id() {
        let arena = DocumentArena::new();
        let fake_id = SectionId::default();
        assert!(arena.get_section(fake_id).is_none());
    }

    #[test]
    fn test_clear() {
        let mut arena = DocumentArena::new();
        arena.insert_section(Section::default());
        arena.insert_paragraph(Paragraph::default());

        assert!(arena.total_count() > 0);

        arena.clear();
        assert_eq!(arena.total_count(), 0);
    }
}
