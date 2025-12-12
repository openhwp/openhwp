# office-core 크레이트 상세 명세

이 문서는 `office-core` 크레이트의 구현 상세를 정의합니다.

> **상위 문서**: [DESIGN.md](./DESIGN.md)

---

## 1. 개요

### 1.1 목적

`office-core` 크레이트는 **에디터의 핵심 로직을 통합**합니다.

- 문서 모델, 레이아웃, 렌더링 연결
- 입력 이벤트 처리
- 뷰포트 및 스크롤 관리
- 커서 및 선택 영역 관리
- IME 상태 관리

### 1.2 설계 원칙

1. **플랫폼 독립**: 순수 Rust로 구현, 플랫폼 API 직접 호출 없음
2. **결정론적**: 동일 입력에 동일 출력, 테스트 가능
3. **단방향 데이터 흐름**: 이벤트 → 상태 변경 → 렌더 요청

---

## 2. 파일 구조

```
crates/office-core/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── office.rs           # OfficeCore
    ├── state.rs            # OfficeState
    ├── event.rs            # InputEvent enum
    ├── result.rs           # UpdateResult
    │
    ├── handler/            # 이벤트 핸들러
    │   ├── mod.rs
    │   ├── keyboard.rs     # 키보드 이벤트
    │   ├── mouse.rs        # 마우스 이벤트
    │   ├── ime.rs          # IME 이벤트
    │   └── clipboard.rs    # 클립보드 이벤트
    │
    ├── cursor.rs           # 커서 관리
    ├── viewport.rs         # 뷰포트 관리
    ├── scroll.rs           # 스크롤 관리
    └── render.rs           # 렌더링 명령 생성
```

---

## 3. 핵심 타입

### 3.1 OfficeCore

에디터의 메인 구조체입니다.

```rust
// src/office.rs

use document::{Document, Position, Selection, SelectionSet, CommandHistory};
use layout::{LayoutEngine, TextMeasurer};
use render_api::{RenderCommands, Rect};

use crate::event::InputEvent;
use crate::result::UpdateResult;
use crate::viewport::Viewport;
use crate::cursor::CursorState;

/// 에디터 코어
///
/// 플랫폼 독립적인 에디터 로직을 제공합니다.
pub struct OfficeCore {
    /// 문서 모델
    document: Document,

    /// 선택 영역
    selection: SelectionSet,

    /// 명령 히스토리 (Undo/Redo)
    history: CommandHistory,

    /// 레이아웃 엔진
    layout: LayoutEngine,

    /// 뷰포트
    viewport: Viewport,

    /// IME 상태
    ime: ImeState,

    /// 커서 상태 (깜빡임 등)
    cursor: CursorState,

    /// 마우스 드래그 상태
    drag_state: Option<DragState>,

    /// 문서 수정 여부
    modified: bool,
}

impl OfficeCore {
    /// 새 에디터 생성
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            selection: SelectionSet::default(),
            history: CommandHistory::new(),
            layout: LayoutEngine::new(),
            viewport: Viewport::new(800.0, 600.0),
            ime: ImeState::default(),
            cursor: CursorState::new(),
            drag_state: None,
            modified: false,
        }
    }

    /// 문서 로드
    pub fn load_document(&mut self, document: Document) {
        self.document = document;
        self.selection = SelectionSet::default();
        self.history.clear();
        self.modified = false;

        // 전체 레이아웃 무효화
        for i in 0..self.document.block_count() {
            if let Some(block) = self.document.block_mut(i) {
                block.mark_layout_dirty();
            }
        }
    }

    /// 문서 참조
    pub fn document(&self) -> &Document {
        &self.document
    }

    /// 선택 영역 참조
    pub fn selection(&self) -> &SelectionSet {
        &self.selection
    }

    /// 뷰포트 참조
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    /// 수정 여부
    pub fn is_modified(&self) -> bool {
        self.modified || self.history.is_modified()
    }

    // === 이벤트 처리 ===

    /// 입력 이벤트 처리
    ///
    /// # Arguments
    /// * `event` - 입력 이벤트
    ///
    /// # Returns
    /// 업데이트 결과 (렌더링 필요 여부 등)
    pub fn handle_event(&mut self, event: InputEvent) -> UpdateResult {
        match event {
            // 키보드
            InputEvent::KeyDown(key) => self.handle_key_down(key),
            InputEvent::KeyUp(key) => self.handle_key_up(key),

            // 마우스
            InputEvent::MouseDown(mouse) => self.handle_mouse_down(mouse),
            InputEvent::MouseUp(mouse) => self.handle_mouse_up(mouse),
            InputEvent::MouseMove(mouse) => self.handle_mouse_move(mouse),
            InputEvent::MouseWheel(wheel) => self.handle_mouse_wheel(wheel),

            // IME
            InputEvent::ImeStart => self.handle_ime_start(),
            InputEvent::ImeUpdate(text) => self.handle_ime_update(text),
            InputEvent::ImeCommit(text) => self.handle_ime_commit(text),

            // 클립보드
            InputEvent::Copy => self.handle_copy(),
            InputEvent::Cut => self.handle_cut(),
            InputEvent::Paste(data) => self.handle_paste(data),

            // 뷰포트
            InputEvent::Resize { width, height } => {
                self.viewport.resize(width, height);
                UpdateResult::layout_needed()
            }

            InputEvent::Focus => {
                self.cursor.show();
                UpdateResult::cursor_changed()
            }

            InputEvent::Blur => {
                self.cursor.hide();
                UpdateResult::cursor_changed()
            }
        }
    }

    // === 레이아웃 ===

    /// 레이아웃 업데이트
    pub fn update_layout(&mut self, measurer: &dyn TextMeasurer) {
        self.layout.update(
            &self.document,
            &self.viewport.rect(),
            measurer,
        );

        // dirty 플래그 클리어
        for i in 0..self.document.block_count() {
            if let Some(block) = self.document.block_mut(i) {
                if !block.is_layout_dirty() {
                    continue;
                }
                // 레이아웃 완료 후 클리어는 LayoutEngine에서 처리
            }
        }
    }

    // === 렌더링 ===

    /// 렌더링 명령 생성
    pub fn render(&self) -> RenderCommands {
        crate::render::render_document(
            &self.layout,
            &self.document,
            &self.viewport,
            &self.selection,
            &self.ime,
            &self.cursor,
        )
    }

    // === 커서 관리 ===

    /// 커서 업데이트 (깜빡임 등)
    ///
    /// # Arguments
    /// * `elapsed_ms` - 경과 시간 (밀리초)
    ///
    /// # Returns
    /// 렌더링 필요 여부
    pub fn update_cursor(&mut self, elapsed_ms: u64) -> bool {
        self.cursor.update(elapsed_ms)
    }

    /// 커서가 보이도록 스크롤
    pub fn scroll_to_cursor(&mut self) -> bool {
        let cursor_pos = self.selection.primary().cursor_position();

        if let Some(cursor_rect) = self.layout.position_to_rect(cursor_pos) {
            self.viewport.scroll_to_visible(cursor_rect)
        } else {
            false
        }
    }

    // === Undo/Redo ===

    /// Undo 가능 여부
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    /// Redo 가능 여부
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// Undo 실행
    pub fn undo(&mut self) -> UpdateResult {
        if self.history.undo(&mut self.document) {
            UpdateResult::document_changed()
        } else {
            UpdateResult::none()
        }
    }

    /// Redo 실행
    pub fn redo(&mut self) -> UpdateResult {
        if self.history.redo(&mut self.document) {
            UpdateResult::document_changed()
        } else {
            UpdateResult::none()
        }
    }

    // === Hit Test ===

    /// 화면 좌표 → 문서 위치
    pub fn hit_test(&self, x: f32, y: f32) -> Position {
        let point = layout::Point::new(
            x + self.viewport.scroll_x,
            y + self.viewport.scroll_y,
        );
        self.layout.hit_test(point).position
    }

    // === IME ===

    /// IME 입력 위치 (숨겨진 textarea 배치용)
    pub fn ime_rect(&self) -> Option<Rect> {
        let pos = self.selection.primary().cursor_position();
        self.layout.position_to_rect(pos).map(|r| {
            Rect::new(
                r.x - self.viewport.scroll_x,
                r.y - self.viewport.scroll_y,
                r.width,
                r.height,
            )
        })
    }
}

/// IME 상태
#[derive(Debug, Clone, Default)]
pub struct ImeState {
    /// 조합 중인 텍스트
    pub composing: Option<ComposingText>,
}

/// 조합 중인 텍스트
#[derive(Debug, Clone)]
pub struct ComposingText {
    /// 조합 시작 위치
    pub position: Position,
    /// 조합 중인 텍스트
    pub text: String,
}

/// 드래그 상태
#[derive(Debug, Clone)]
struct DragState {
    /// 드래그 시작 위치
    start: Position,
    /// 드래그 종류
    kind: DragKind,
}

#[derive(Debug, Clone, Copy)]
enum DragKind {
    /// 텍스트 선택
    Selection,
    /// 블록 이동 (추후)
    Block,
}

impl Default for OfficeCore {
    fn default() -> Self {
        Self::new()
    }
}
```

### 3.2 InputEvent

입력 이벤트 열거형입니다.

```rust
// src/event.rs

/// 입력 이벤트
#[derive(Debug, Clone)]
pub enum InputEvent {
    // === 키보드 ===
    /// 키 누름
    KeyDown(KeyEvent),
    /// 키 뗌
    KeyUp(KeyEvent),

    // === 마우스 ===
    /// 마우스 버튼 누름
    MouseDown(MouseEvent),
    /// 마우스 버튼 뗌
    MouseUp(MouseEvent),
    /// 마우스 이동
    MouseMove(MouseEvent),
    /// 마우스 휠
    MouseWheel(WheelEvent),

    // === IME ===
    /// IME 조합 시작
    ImeStart,
    /// IME 조합 중 (텍스트 업데이트)
    ImeUpdate(String),
    /// IME 조합 확정
    ImeCommit(String),

    // === 클립보드 ===
    /// 복사
    Copy,
    /// 잘라내기
    Cut,
    /// 붙여넣기
    Paste(PasteData),

    // === 뷰포트 ===
    /// 크기 변경
    Resize { width: f32, height: f32 },
    /// 포커스 획득
    Focus,
    /// 포커스 상실
    Blur,
}

/// 키보드 이벤트
#[derive(Debug, Clone)]
pub struct KeyEvent {
    /// 키 코드
    pub key: Key,
    /// 수정자 키
    pub modifiers: Modifiers,
}

/// 키 코드
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    // 문자
    Char(char),

    // 기능 키
    Enter,
    Tab,
    Backspace,
    Delete,
    Escape,

    // 방향 키
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,

    // 기타
    Insert,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // 알 수 없는 키
    Unknown(String),
}

/// 수정자 키
#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,  // Cmd on macOS, Win on Windows
}

impl Modifiers {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn ctrl() -> Self {
        Self { ctrl: true, ..Self::default() }
    }

    pub fn shift() -> Self {
        Self { shift: true, ..Self::default() }
    }

    pub fn ctrl_shift() -> Self {
        Self { ctrl: true, shift: true, ..Self::default() }
    }

    /// 플랫폼별 "명령" 키 (Ctrl 또는 Cmd)
    pub fn command(&self) -> bool {
        #[cfg(target_os = "macos")]
        { self.meta }
        #[cfg(not(target_os = "macos"))]
        { self.ctrl }
    }
}

/// 마우스 이벤트
#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// X 좌표 (뷰포트 기준)
    pub x: f32,
    /// Y 좌표 (뷰포트 기준)
    pub y: f32,
    /// 버튼
    pub button: MouseButton,
    /// 수정자 키
    pub modifiers: Modifiers,
    /// 클릭 횟수 (더블/트리플 클릭)
    pub click_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    None,
}

/// 휠 이벤트
#[derive(Debug, Clone)]
pub struct WheelEvent {
    /// X 이동량
    pub delta_x: f32,
    /// Y 이동량
    pub delta_y: f32,
    /// 수정자 키
    pub modifiers: Modifiers,
}

/// 붙여넣기 데이터
#[derive(Debug, Clone)]
pub struct PasteData {
    /// 내부 포맷 (있으면)
    pub internal: Option<document::ClipboardData>,
    /// HTML
    pub html: Option<String>,
    /// 평문
    pub plain_text: Option<String>,
}
```

### 3.3 UpdateResult

이벤트 처리 결과입니다.

```rust
// src/result.rs

use std::ops::Range;

/// 이벤트 처리 결과
#[derive(Debug, Clone, Default)]
pub struct UpdateResult {
    /// 문서 변경됨
    pub document_changed: bool,
    /// 선택 변경됨
    pub selection_changed: bool,
    /// 레이아웃 재계산 필요
    pub layout_needed: bool,
    /// 렌더링 필요
    pub render_needed: bool,
    /// 커서 변경됨 (깜빡임 리셋)
    pub cursor_changed: bool,
    /// 커서가 보이도록 스크롤 필요
    pub scroll_to_cursor: bool,
    /// dirty 블록 범위
    pub dirty_range: Option<Range<usize>>,
    /// 클립보드 데이터 (복사 시)
    pub clipboard_data: Option<document::ClipboardData>,
}

impl UpdateResult {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn document_changed() -> Self {
        Self {
            document_changed: true,
            layout_needed: true,
            render_needed: true,
            cursor_changed: true,
            scroll_to_cursor: true,
            ..Self::default()
        }
    }

    pub fn selection_changed() -> Self {
        Self {
            selection_changed: true,
            render_needed: true,
            cursor_changed: true,
            scroll_to_cursor: true,
            ..Self::default()
        }
    }

    pub fn cursor_changed() -> Self {
        Self {
            cursor_changed: true,
            render_needed: true,
            ..Self::default()
        }
    }

    pub fn layout_needed() -> Self {
        Self {
            layout_needed: true,
            render_needed: true,
            ..Self::default()
        }
    }

    pub fn with_dirty_range(mut self, range: Range<usize>) -> Self {
        self.dirty_range = Some(range);
        self
    }

    pub fn with_clipboard(mut self, data: document::ClipboardData) -> Self {
        self.clipboard_data = Some(data);
        self
    }

    /// 두 결과 병합
    pub fn merge(self, other: Self) -> Self {
        Self {
            document_changed: self.document_changed || other.document_changed,
            selection_changed: self.selection_changed || other.selection_changed,
            layout_needed: self.layout_needed || other.layout_needed,
            render_needed: self.render_needed || other.render_needed,
            cursor_changed: self.cursor_changed || other.cursor_changed,
            scroll_to_cursor: self.scroll_to_cursor || other.scroll_to_cursor,
            dirty_range: merge_ranges(self.dirty_range, other.dirty_range),
            clipboard_data: self.clipboard_data.or(other.clipboard_data),
        }
    }
}

fn merge_ranges(a: Option<Range<usize>>, b: Option<Range<usize>>) -> Option<Range<usize>> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.start.min(b.start)..a.end.max(b.end)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}
```

---

## 4. 이벤트 핸들러

### 4.1 키보드 핸들러

```rust
// src/handler/keyboard.rs

use document::{Command, InsertTextCommand, DeleteCommand, DeleteDirection, Position};
use crate::{OfficeCore, UpdateResult, KeyEvent, Key, Modifiers};

impl OfficeCore {
    pub(crate) fn handle_key_down(&mut self, event: KeyEvent) -> UpdateResult {
        // IME 조합 중이면 일부 키만 처리
        if self.ime.composing.is_some() {
            return match event.key {
                Key::Escape => {
                    // 조합 취소
                    self.ime.composing = None;
                    UpdateResult::cursor_changed()
                }
                _ => UpdateResult::none(),
            };
        }

        match (&event.key, &event.modifiers) {
            // === 문자 입력 ===
            (Key::Char(c), m) if !m.ctrl && !m.alt && !m.meta => {
                self.insert_char(*c)
            }

            // === 삭제 ===
            (Key::Backspace, m) if !m.ctrl => {
                self.delete_backward()
            }
            (Key::Backspace, m) if m.ctrl => {
                self.delete_word_backward()
            }
            (Key::Delete, m) if !m.ctrl => {
                self.delete_forward()
            }
            (Key::Delete, m) if m.ctrl => {
                self.delete_word_forward()
            }

            // === 줄바꿈 ===
            (Key::Enter, _) => {
                self.insert_paragraph_break()
            }

            // === 탭 ===
            (Key::Tab, m) if !m.shift => {
                self.insert_char('\t')
            }

            // === 커서 이동 ===
            (Key::Left, m) => self.move_cursor_left(m.shift, m.ctrl),
            (Key::Right, m) => self.move_cursor_right(m.shift, m.ctrl),
            (Key::Up, m) => self.move_cursor_up(m.shift),
            (Key::Down, m) => self.move_cursor_down(m.shift),
            (Key::Home, m) => self.move_cursor_home(m.shift, m.ctrl),
            (Key::End, m) => self.move_cursor_end(m.shift, m.ctrl),

            // === 단축키 ===
            (Key::Char('a'), m) if m.command() => {
                self.select_all()
            }
            (Key::Char('z'), m) if m.command() && !m.shift => {
                self.undo()
            }
            (Key::Char('z'), m) if m.command() && m.shift => {
                self.redo()
            }
            (Key::Char('y'), m) if m.command() => {
                self.redo()
            }

            // === 서식 단축키 ===
            (Key::Char('b'), m) if m.command() => {
                self.toggle_bold()
            }
            (Key::Char('i'), m) if m.command() => {
                self.toggle_italic()
            }
            (Key::Char('u'), m) if m.command() => {
                self.toggle_underline()
            }

            _ => UpdateResult::none(),
        }
    }

    pub(crate) fn handle_key_up(&mut self, _event: KeyEvent) -> UpdateResult {
        UpdateResult::none()
    }

    // === 텍스트 입력 ===

    fn insert_char(&mut self, c: char) -> UpdateResult {
        // 선택 영역 있으면 먼저 삭제
        if !self.selection.primary().is_collapsed() {
            self.delete_selection();
        }

        let pos = self.selection.primary().cursor_position();
        let cmd = InsertTextCommand::new(pos, c.to_string());

        let range = cmd.execute(&mut self.document);
        self.history.push(Box::new(cmd));

        // 커서 이동
        let new_offset = pos.offset + c.len_utf8();
        self.selection.primary_mut().move_to(Position::new(pos.block_index, new_offset));

        UpdateResult::document_changed().with_dirty_range(range)
    }

    fn insert_paragraph_break(&mut self) -> UpdateResult {
        // 선택 영역 있으면 먼저 삭제
        if !self.selection.primary().is_collapsed() {
            self.delete_selection();
        }

        let pos = self.selection.primary().cursor_position();

        // 문단 분리 명령
        let cmd = document::SplitParagraphCommand::new(pos);
        let range = cmd.execute(&mut self.document);
        self.history.push(Box::new(cmd));

        // 커서를 새 문단 시작으로 이동
        self.selection.primary_mut().move_to(Position::new(pos.block_index + 1, 0));

        UpdateResult::document_changed().with_dirty_range(range)
    }

    // === 삭제 ===

    fn delete_backward(&mut self) -> UpdateResult {
        if !self.selection.primary().is_collapsed() {
            return self.delete_selection();
        }

        let pos = self.selection.primary().cursor_position();

        // 문단 시작이면 이전 문단과 병합
        if pos.offset == 0 {
            if pos.block_index == 0 {
                return UpdateResult::none();
            }
            return self.merge_with_previous_paragraph();
        }

        let cmd = DeleteCommand::backward(pos);
        let range = cmd.execute(&mut self.document);
        self.history.push(Box::new(cmd));

        // 커서 이동
        let text = self.document.block(pos.block_index)
            .and_then(|b| b.as_paragraph())
            .map(|p| p.text())
            .unwrap_or("");
        let new_offset = prev_grapheme_boundary(text, pos.offset);
        self.selection.primary_mut().move_to(Position::new(pos.block_index, new_offset));

        UpdateResult::document_changed().with_dirty_range(range)
    }

    fn delete_forward(&mut self) -> UpdateResult {
        if !self.selection.primary().is_collapsed() {
            return self.delete_selection();
        }

        let pos = self.selection.primary().cursor_position();

        let text_len = self.document.block(pos.block_index)
            .and_then(|b| b.as_paragraph())
            .map(|p| p.text_len())
            .unwrap_or(0);

        // 문단 끝이면 다음 문단과 병합
        if pos.offset >= text_len {
            if pos.block_index >= self.document.block_count() - 1 {
                return UpdateResult::none();
            }
            return self.merge_with_next_paragraph();
        }

        let cmd = DeleteCommand::forward(pos);
        let range = cmd.execute(&mut self.document);
        self.history.push(Box::new(cmd));

        UpdateResult::document_changed().with_dirty_range(range)
    }

    fn delete_word_backward(&mut self) -> UpdateResult {
        let pos = self.selection.primary().cursor_position();
        let cmd = DeleteCommand::backward_word(pos);
        let range = cmd.execute(&mut self.document);
        self.history.push(Box::new(cmd));

        UpdateResult::document_changed().with_dirty_range(range)
    }

    fn delete_word_forward(&mut self) -> UpdateResult {
        let pos = self.selection.primary().cursor_position();
        let cmd = DeleteCommand::forward_word(pos);
        let range = cmd.execute(&mut self.document);
        self.history.push(Box::new(cmd));

        UpdateResult::document_changed().with_dirty_range(range)
    }

    fn delete_selection(&mut self) -> UpdateResult {
        let selection = self.selection.primary();
        if selection.is_collapsed() {
            return UpdateResult::none();
        }

        let (start, end) = selection.ordered();
        let cmd = document::DeleteRangeCommand::new(start, end);
        let range = cmd.execute(&mut self.document);
        self.history.push(Box::new(cmd));

        // 커서를 시작 위치로
        self.selection.primary_mut().move_to(start);

        UpdateResult::document_changed().with_dirty_range(range)
    }

    fn merge_with_previous_paragraph(&mut self) -> UpdateResult {
        let pos = self.selection.primary().cursor_position();
        let cmd = document::MergeParagraphCommand::new(pos.block_index - 1);
        let range = cmd.execute(&mut self.document);
        self.history.push(Box::new(cmd));

        // 이전 문단 끝으로 커서 이동
        let prev_len = self.document.block(pos.block_index - 1)
            .and_then(|b| b.as_paragraph())
            .map(|p| p.text_len())
            .unwrap_or(0);
        self.selection.primary_mut().move_to(Position::new(pos.block_index - 1, prev_len));

        UpdateResult::document_changed().with_dirty_range(range)
    }

    fn merge_with_next_paragraph(&mut self) -> UpdateResult {
        let pos = self.selection.primary().cursor_position();
        let cmd = document::MergeParagraphCommand::new(pos.block_index);
        let range = cmd.execute(&mut self.document);
        self.history.push(Box::new(cmd));

        UpdateResult::document_changed().with_dirty_range(range)
    }

    // === 커서 이동 ===

    fn move_cursor_left(&mut self, extend: bool, word: bool) -> UpdateResult {
        let pos = self.selection.primary().cursor_position();

        let new_pos = if word {
            self.find_word_boundary_left(pos)
        } else {
            self.find_char_boundary_left(pos)
        };

        if extend {
            self.selection.primary_mut().extend_to(new_pos);
        } else {
            self.selection.primary_mut().move_to(new_pos);
        }

        UpdateResult::selection_changed()
    }

    fn move_cursor_right(&mut self, extend: bool, word: bool) -> UpdateResult {
        let pos = self.selection.primary().cursor_position();

        let new_pos = if word {
            self.find_word_boundary_right(pos)
        } else {
            self.find_char_boundary_right(pos)
        };

        if extend {
            self.selection.primary_mut().extend_to(new_pos);
        } else {
            self.selection.primary_mut().move_to(new_pos);
        }

        UpdateResult::selection_changed()
    }

    fn move_cursor_up(&mut self, extend: bool) -> UpdateResult {
        // TODO: 시각적 줄 기준 이동 (레이아웃 필요)
        let pos = self.selection.primary().cursor_position();

        if pos.block_index == 0 {
            let new_pos = Position::new(0, 0);
            if extend {
                self.selection.primary_mut().extend_to(new_pos);
            } else {
                self.selection.primary_mut().move_to(new_pos);
            }
        } else {
            let new_pos = Position::new(pos.block_index - 1, 0);
            if extend {
                self.selection.primary_mut().extend_to(new_pos);
            } else {
                self.selection.primary_mut().move_to(new_pos);
            }
        }

        UpdateResult::selection_changed()
    }

    fn move_cursor_down(&mut self, extend: bool) -> UpdateResult {
        let pos = self.selection.primary().cursor_position();

        if pos.block_index >= self.document.block_count() - 1 {
            let text_len = self.document.block(pos.block_index)
                .and_then(|b| b.as_paragraph())
                .map(|p| p.text_len())
                .unwrap_or(0);
            let new_pos = Position::new(pos.block_index, text_len);
            if extend {
                self.selection.primary_mut().extend_to(new_pos);
            } else {
                self.selection.primary_mut().move_to(new_pos);
            }
        } else {
            let new_pos = Position::new(pos.block_index + 1, 0);
            if extend {
                self.selection.primary_mut().extend_to(new_pos);
            } else {
                self.selection.primary_mut().move_to(new_pos);
            }
        }

        UpdateResult::selection_changed()
    }

    fn move_cursor_home(&mut self, extend: bool, document_start: bool) -> UpdateResult {
        let new_pos = if document_start {
            Position::start()
        } else {
            let pos = self.selection.primary().cursor_position();
            Position::new(pos.block_index, 0)
        };

        if extend {
            self.selection.primary_mut().extend_to(new_pos);
        } else {
            self.selection.primary_mut().move_to(new_pos);
        }

        UpdateResult::selection_changed()
    }

    fn move_cursor_end(&mut self, extend: bool, document_end: bool) -> UpdateResult {
        let new_pos = if document_end {
            let last_block = self.document.block_count() - 1;
            let text_len = self.document.block(last_block)
                .and_then(|b| b.as_paragraph())
                .map(|p| p.text_len())
                .unwrap_or(0);
            Position::new(last_block, text_len)
        } else {
            let pos = self.selection.primary().cursor_position();
            let text_len = self.document.block(pos.block_index)
                .and_then(|b| b.as_paragraph())
                .map(|p| p.text_len())
                .unwrap_or(0);
            Position::new(pos.block_index, text_len)
        };

        if extend {
            self.selection.primary_mut().extend_to(new_pos);
        } else {
            self.selection.primary_mut().move_to(new_pos);
        }

        UpdateResult::selection_changed()
    }

    fn select_all(&mut self) -> UpdateResult {
        let last_block = self.document.block_count() - 1;
        let text_len = self.document.block(last_block)
            .and_then(|b| b.as_paragraph())
            .map(|p| p.text_len())
            .unwrap_or(0);

        let selection = document::Selection::range(
            Position::start(),
            Position::new(last_block, text_len),
        );
        *self.selection.primary_mut() = selection;

        UpdateResult::selection_changed()
    }

    // === 서식 ===

    fn toggle_bold(&mut self) -> UpdateResult {
        // TODO: 굵게 토글
        UpdateResult::none()
    }

    fn toggle_italic(&mut self) -> UpdateResult {
        // TODO: 기울임 토글
        UpdateResult::none()
    }

    fn toggle_underline(&mut self) -> UpdateResult {
        // TODO: 밑줄 토글
        UpdateResult::none()
    }

    // === 유틸리티 ===

    fn find_char_boundary_left(&self, pos: Position) -> Position {
        if pos.offset == 0 {
            // 이전 블록 끝으로
            if pos.block_index > 0 {
                let prev_len = self.document.block(pos.block_index - 1)
                    .and_then(|b| b.as_paragraph())
                    .map(|p| p.text_len())
                    .unwrap_or(0);
                return Position::new(pos.block_index - 1, prev_len);
            }
            return pos;
        }

        let text = self.document.block(pos.block_index)
            .and_then(|b| b.as_paragraph())
            .map(|p| p.text())
            .unwrap_or("");

        let new_offset = prev_grapheme_boundary(text, pos.offset);
        Position::new(pos.block_index, new_offset)
    }

    fn find_char_boundary_right(&self, pos: Position) -> Position {
        let text_len = self.document.block(pos.block_index)
            .and_then(|b| b.as_paragraph())
            .map(|p| p.text_len())
            .unwrap_or(0);

        if pos.offset >= text_len {
            // 다음 블록 시작으로
            if pos.block_index < self.document.block_count() - 1 {
                return Position::new(pos.block_index + 1, 0);
            }
            return Position::new(pos.block_index, text_len);
        }

        let text = self.document.block(pos.block_index)
            .and_then(|b| b.as_paragraph())
            .map(|p| p.text())
            .unwrap_or("");

        let new_offset = next_grapheme_boundary(text, pos.offset);
        Position::new(pos.block_index, new_offset)
    }

    fn find_word_boundary_left(&self, pos: Position) -> Position {
        // TODO: 단어 경계 검색
        self.find_char_boundary_left(pos)
    }

    fn find_word_boundary_right(&self, pos: Position) -> Position {
        // TODO: 단어 경계 검색
        self.find_char_boundary_right(pos)
    }
}

// Unicode segmentation 유틸리티
fn prev_grapheme_boundary(text: &str, offset: usize) -> usize {
    use unicode_segmentation::UnicodeSegmentation;

    let mut last = 0;
    for (i, _) in text.grapheme_indices(true) {
        if i >= offset {
            break;
        }
        last = i;
    }
    last
}

fn next_grapheme_boundary(text: &str, offset: usize) -> usize {
    use unicode_segmentation::UnicodeSegmentation;

    for (i, g) in text.grapheme_indices(true) {
        if i >= offset {
            return i + g.len();
        }
    }
    text.len()
}
```

### 4.2 마우스 핸들러

```rust
// src/handler/mouse.rs

use document::{Position, Selection};
use crate::{OfficeCore, UpdateResult, MouseEvent, MouseButton, WheelEvent};

impl OfficeCore {
    pub(crate) fn handle_mouse_down(&mut self, event: MouseEvent) -> UpdateResult {
        if event.button != MouseButton::Left {
            return UpdateResult::none();
        }

        let pos = self.hit_test(event.x, event.y);

        match event.click_count {
            1 => {
                // 단일 클릭: 커서 이동 또는 선택 시작
                if event.modifiers.shift {
                    // Shift+클릭: 선택 확장
                    self.selection.primary_mut().extend_to(pos);
                } else {
                    // 클릭: 커서 이동
                    self.selection.primary_mut().move_to(pos);
                }

                // 드래그 시작
                self.drag_state = Some(DragState {
                    start: pos,
                    kind: DragKind::Selection,
                });
            }
            2 => {
                // 더블 클릭: 단어 선택
                let word_range = self.find_word_at(pos);
                *self.selection.primary_mut() = Selection::range(word_range.0, word_range.1);
            }
            3 => {
                // 트리플 클릭: 문단 선택
                let para_start = Position::new(pos.block_index, 0);
                let text_len = self.document.block(pos.block_index)
                    .and_then(|b| b.as_paragraph())
                    .map(|p| p.text_len())
                    .unwrap_or(0);
                let para_end = Position::new(pos.block_index, text_len);
                *self.selection.primary_mut() = Selection::range(para_start, para_end);
            }
            _ => {}
        }

        UpdateResult::selection_changed()
    }

    pub(crate) fn handle_mouse_up(&mut self, _event: MouseEvent) -> UpdateResult {
        self.drag_state = None;
        UpdateResult::none()
    }

    pub(crate) fn handle_mouse_move(&mut self, event: MouseEvent) -> UpdateResult {
        // 드래그 중이면 선택 확장
        if let Some(drag) = &self.drag_state {
            if matches!(drag.kind, DragKind::Selection) {
                let pos = self.hit_test(event.x, event.y);
                self.selection.primary_mut().extend_to(pos);
                return UpdateResult::selection_changed();
            }
        }

        UpdateResult::none()
    }

    pub(crate) fn handle_mouse_wheel(&mut self, event: WheelEvent) -> UpdateResult {
        // 스크롤
        let scroll_amount = if event.modifiers.shift {
            // 가로 스크롤
            self.viewport.scroll_x += event.delta_y;
            true
        } else {
            // 세로 스크롤
            self.viewport.scroll_y += event.delta_y;
            true
        };

        if scroll_amount {
            // 스크롤 범위 제한
            self.viewport.clamp_scroll(self.layout.total_height());
            UpdateResult {
                render_needed: true,
                ..UpdateResult::none()
            }
        } else {
            UpdateResult::none()
        }
    }

    fn find_word_at(&self, pos: Position) -> (Position, Position) {
        let text = self.document.block(pos.block_index)
            .and_then(|b| b.as_paragraph())
            .map(|p| p.text())
            .unwrap_or("");

        // 간단한 구현: 공백 기준
        let start = text[..pos.offset]
            .rfind(char::is_whitespace)
            .map(|i| i + 1)
            .unwrap_or(0);

        let end = text[pos.offset..]
            .find(char::is_whitespace)
            .map(|i| pos.offset + i)
            .unwrap_or(text.len());

        (
            Position::new(pos.block_index, start),
            Position::new(pos.block_index, end),
        )
    }
}
```

### 4.3 IME 핸들러

```rust
// src/handler/ime.rs

use document::Position;
use crate::{OfficeCore, UpdateResult, ComposingText};

impl OfficeCore {
    pub(crate) fn handle_ime_start(&mut self) -> UpdateResult {
        let pos = self.selection.primary().cursor_position();

        self.ime.composing = Some(ComposingText {
            position: pos,
            text: String::new(),
        });

        UpdateResult::cursor_changed()
    }

    pub(crate) fn handle_ime_update(&mut self, text: String) -> UpdateResult {
        if let Some(ref mut composing) = self.ime.composing {
            composing.text = text;
        }

        UpdateResult {
            render_needed: true,
            ..UpdateResult::none()
        }
    }

    pub(crate) fn handle_ime_commit(&mut self, text: String) -> UpdateResult {
        // 조합 상태 클리어
        let position = self.ime.composing.take()
            .map(|c| c.position)
            .unwrap_or_else(|| self.selection.primary().cursor_position());

        // 텍스트 삽입
        let cmd = document::InsertTextCommand::new(position, text.clone());
        let range = cmd.execute(&mut self.document);
        self.history.push(Box::new(cmd));

        // 커서 이동
        let new_offset = position.offset + text.len();
        self.selection.primary_mut().move_to(Position::new(position.block_index, new_offset));

        UpdateResult::document_changed().with_dirty_range(range)
    }
}
```

### 4.4 클립보드 핸들러

```rust
// src/handler/clipboard.rs

use document::ClipboardData;
use crate::{OfficeCore, UpdateResult, PasteData};

impl OfficeCore {
    pub(crate) fn handle_copy(&mut self) -> UpdateResult {
        let selection = self.selection.primary();
        if selection.is_collapsed() {
            return UpdateResult::none();
        }

        let clipboard_data = ClipboardData::from_selection(&self.document, selection);
        UpdateResult::none().with_clipboard(clipboard_data)
    }

    pub(crate) fn handle_cut(&mut self) -> UpdateResult {
        let copy_result = self.handle_copy();
        let delete_result = self.delete_selection();

        copy_result.merge(delete_result)
    }

    pub(crate) fn handle_paste(&mut self, data: PasteData) -> UpdateResult {
        // 선택 영역 삭제
        if !self.selection.primary().is_collapsed() {
            self.delete_selection();
        }

        // 붙여넣기 우선순위: 내부 > HTML > Plain Text
        if let Some(internal) = data.internal {
            return self.paste_internal(internal);
        }

        if let Some(html) = data.html {
            return self.paste_html(&html);
        }

        if let Some(text) = data.plain_text {
            return self.paste_plain_text(&text);
        }

        UpdateResult::none()
    }

    fn paste_internal(&mut self, data: ClipboardData) -> UpdateResult {
        // 스타일 병합
        let (char_map, para_map) = self.document.styles_mut().merge(&data.styles);

        // 바이너리 병합
        for (id, bytes) in data.binaries {
            self.document.binaries_mut().insert(id, bytes);
        }

        // 블록 삽입
        let pos = self.selection.primary().cursor_position();
        let mut insert_pos = pos;

        for block in data.blocks {
            // TODO: 스타일 ID 재매핑
            // TODO: 블록 삽입
        }

        UpdateResult::document_changed()
    }

    fn paste_html(&mut self, html: &str) -> UpdateResult {
        // TODO: HTML 파싱 및 변환
        // 일단 plain text로 fallback
        let text = strip_html_tags(html);
        self.paste_plain_text(&text)
    }

    fn paste_plain_text(&mut self, text: &str) -> UpdateResult {
        let pos = self.selection.primary().cursor_position();

        // 줄바꿈으로 분리
        let lines: Vec<&str> = text.lines().collect();

        if lines.is_empty() {
            return UpdateResult::none();
        }

        if lines.len() == 1 {
            // 단일 줄: 현재 문단에 삽입
            let cmd = document::InsertTextCommand::new(pos, lines[0].to_string());
            let range = cmd.execute(&mut self.document);
            self.history.push(Box::new(cmd));

            let new_offset = pos.offset + lines[0].len();
            self.selection.primary_mut().move_to(document::Position::new(pos.block_index, new_offset));

            return UpdateResult::document_changed().with_dirty_range(range);
        }

        // 여러 줄: 문단 분리 필요
        // TODO: 배치 명령으로 구현
        let mut current_pos = pos;

        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                // 새 문단 생성
                let split_cmd = document::SplitParagraphCommand::new(current_pos);
                split_cmd.execute(&mut self.document);
                current_pos = document::Position::new(current_pos.block_index + 1, 0);
            }

            if !line.is_empty() {
                let insert_cmd = document::InsertTextCommand::new(current_pos, line.to_string());
                insert_cmd.execute(&mut self.document);
                current_pos = document::Position::new(current_pos.block_index, line.len());
            }
        }

        self.selection.primary_mut().move_to(current_pos);
        UpdateResult::document_changed()
    }
}

fn strip_html_tags(html: &str) -> String {
    // 간단한 HTML 태그 제거
    let mut result = String::new();
    let mut in_tag = false;

    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }

    // HTML 엔티티 디코딩
    result
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&nbsp;", " ")
}
```

---

## 5. 뷰포트 및 커서

### 5.1 Viewport

```rust
// src/viewport.rs

use render_api::Rect;

/// 뷰포트 (보이는 영역)
#[derive(Debug, Clone)]
pub struct Viewport {
    /// 너비
    pub width: f32,
    /// 높이
    pub height: f32,
    /// 가로 스크롤 오프셋
    pub scroll_x: f32,
    /// 세로 스크롤 오프셋
    pub scroll_y: f32,
}

impl Viewport {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            scroll_x: 0.0,
            scroll_y: 0.0,
        }
    }

    /// 뷰포트 사각형
    pub fn rect(&self) -> Rect {
        Rect::new(self.scroll_x, self.scroll_y, self.width, self.height)
    }

    /// 크기 변경
    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    /// 특정 영역이 보이도록 스크롤
    ///
    /// 반환: 스크롤 변경 여부
    pub fn scroll_to_visible(&mut self, rect: Rect) -> bool {
        let mut changed = false;

        // 수직 스크롤
        if rect.y < self.scroll_y {
            self.scroll_y = rect.y - 10.0;  // 여백
            changed = true;
        } else if rect.y + rect.height > self.scroll_y + self.height {
            self.scroll_y = rect.y + rect.height - self.height + 10.0;
            changed = true;
        }

        // 수평 스크롤
        if rect.x < self.scroll_x {
            self.scroll_x = rect.x - 10.0;
            changed = true;
        } else if rect.x + rect.width > self.scroll_x + self.width {
            self.scroll_x = rect.x + rect.width - self.width + 10.0;
            changed = true;
        }

        if changed {
            self.clamp_scroll(f32::MAX);  // 범위 제한은 전체 높이 알 때
        }

        changed
    }

    /// 스크롤 범위 제한
    pub fn clamp_scroll(&mut self, total_height: f32) {
        self.scroll_x = self.scroll_x.max(0.0);
        self.scroll_y = self.scroll_y.max(0.0);

        let max_y = (total_height - self.height).max(0.0);
        self.scroll_y = self.scroll_y.min(max_y);
    }

    /// 특정 좌표가 뷰포트 내에 있는지
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.scroll_x
            && x < self.scroll_x + self.width
            && y >= self.scroll_y
            && y < self.scroll_y + self.height
    }
}
```

### 5.2 CursorState

```rust
// src/cursor.rs

/// 커서 상태
#[derive(Debug, Clone)]
pub struct CursorState {
    /// 표시 여부
    visible: bool,
    /// 깜빡임 상태 (true = 보임)
    blink_on: bool,
    /// 마지막 깜빡임 전환 시간
    last_blink_ms: u64,
    /// 깜빡임 간격 (밀리초)
    blink_interval_ms: u64,
}

impl CursorState {
    pub fn new() -> Self {
        Self {
            visible: true,
            blink_on: true,
            last_blink_ms: 0,
            blink_interval_ms: 530,  // 표준 깜빡임 속도
        }
    }

    /// 커서 표시
    pub fn show(&mut self) {
        self.visible = true;
        self.blink_on = true;
        self.last_blink_ms = 0;
    }

    /// 커서 숨김
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// 커서 상태 업데이트
    ///
    /// # Arguments
    /// * `elapsed_ms` - 경과 시간 (밀리초)
    ///
    /// # Returns
    /// 렌더링 필요 여부
    pub fn update(&mut self, elapsed_ms: u64) -> bool {
        if !self.visible {
            return false;
        }

        self.last_blink_ms += elapsed_ms;

        if self.last_blink_ms >= self.blink_interval_ms {
            self.blink_on = !self.blink_on;
            self.last_blink_ms = 0;
            true
        } else {
            false
        }
    }

    /// 깜빡임 리셋 (입력 시)
    pub fn reset_blink(&mut self) {
        self.blink_on = true;
        self.last_blink_ms = 0;
    }

    /// 현재 보이는지
    pub fn is_visible(&self) -> bool {
        self.visible && self.blink_on
    }
}

impl Default for CursorState {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## 6. 렌더링 명령 생성

```rust
// src/render.rs

use document::{Document, Selection, SelectionSet, Position};
use layout::{LayoutEngine, BlockLayout, ParagraphLayout, LineLayout, RunLayout};
use render_api::*;

use crate::viewport::Viewport;
use crate::cursor::CursorState;
use crate::ImeState;

/// 문서 렌더링 명령 생성
pub fn render_document(
    layout: &LayoutEngine,
    doc: &Document,
    viewport: &Viewport,
    selection: &SelectionSet,
    ime: &ImeState,
    cursor: &CursorState,
) -> RenderCommands {
    let mut commands = RenderCommands::new();

    // 배경
    commands.push(RenderCommand::FillRect {
        rect: Rect::new(0.0, 0.0, viewport.width, viewport.height),
        color: Color::WHITE,
    });

    // 뷰포트 변환
    commands.push(RenderCommand::Save);
    commands.push(RenderCommand::Translate {
        x: -viewport.scroll_x,
        y: -viewport.scroll_y,
    });

    // 보이는 블록 렌더링
    let visible = layout.visible_blocks(&viewport.rect());

    for block_idx in visible {
        let block_y = layout.block_y(block_idx);

        if let Some(block_layout) = layout.block_layout(block_idx) {
            commands.push(RenderCommand::Save);
            commands.push(RenderCommand::Translate { x: 0.0, y: block_y });

            match block_layout {
                BlockLayout::Paragraph(para) => {
                    render_paragraph(
                        &mut commands,
                        para,
                        doc,
                        block_idx,
                        selection.primary(),
                        ime,
                    );
                }
                BlockLayout::Table(table) => {
                    // TODO: 표 렌더링
                }
                BlockLayout::Image(image) => {
                    commands.push(RenderCommand::DrawImage {
                        image_id: ImageId(image.binary_id.0),
                        rect: image.bounds.clone(),
                    });
                }
                BlockLayout::Shape(shape) => {
                    // TODO: 도형 렌더링
                }
            }

            commands.push(RenderCommand::Restore);
        }
    }

    // 커서 렌더링
    if cursor.is_visible() {
        if let Some(cursor_rect) = layout.position_to_rect(selection.primary().cursor_position()) {
            let screen_rect = Rect::new(
                cursor_rect.x,
                cursor_rect.y,
                2.0,
                cursor_rect.height,
            );

            commands.push(RenderCommand::FillRect {
                rect: screen_rect,
                color: Color::BLACK,
            });
        }
    }

    // IME 조합 중 텍스트
    if let Some(ref composing) = ime.composing {
        render_composing_text(&mut commands, layout, composing, doc);
    }

    commands.push(RenderCommand::Restore);
    commands
}

fn render_paragraph(
    commands: &mut RenderCommands,
    layout: &ParagraphLayout,
    doc: &Document,
    block_idx: usize,
    selection: &Selection,
    ime: &ImeState,
) {
    for line in &layout.lines {
        // 선택 영역 하이라이트
        if !selection.is_collapsed() {
            if let Some(highlight) = calculate_selection_highlight(
                selection,
                block_idx,
                line,
                layout,
            ) {
                commands.push(RenderCommand::FillRect {
                    rect: highlight,
                    color: Color::rgba(0, 120, 215, 80),
                });
            }
        }

        // Run 렌더링
        for run in &line.runs {
            render_run(commands, run, line, doc);
        }
    }
}

fn render_run(
    commands: &mut RenderCommands,
    run: &RunLayout,
    line: &LineLayout,
    doc: &Document,
) {
    let style = doc.styles().get_char_style(run.style_id)
        .cloned()
        .unwrap_or_default();

    let text_style = TextStyle::from_char_style(&style, doc.styles());

    commands.push(RenderCommand::DrawText {
        method: TextRenderMethod::String(StringText {
            text: run.text.clone(),
            position: Point::new(
                line.bounds.x + run.bounds.x,
                line.baseline,
            ),
            style: text_style,
        }),
    });
}

fn calculate_selection_highlight(
    selection: &Selection,
    block_idx: usize,
    line: &LineLayout,
    _layout: &ParagraphLayout,
) -> Option<Rect> {
    let (start, end) = selection.ordered();

    // 이 블록이 선택 범위에 포함되는지
    if block_idx < start.block_index || block_idx > end.block_index {
        return None;
    }

    // 이 줄이 선택 범위에 포함되는지
    let line_start = line.text_range.start;
    let line_end = line.text_range.end;

    let sel_start_in_line = if block_idx == start.block_index {
        start.offset.max(line_start)
    } else {
        line_start
    };

    let sel_end_in_line = if block_idx == end.block_index {
        end.offset.min(line_end)
    } else {
        line_end
    };

    if sel_start_in_line >= sel_end_in_line {
        return None;
    }

    // 간단한 구현: 전체 줄 하이라이트
    // TODO: 정확한 문자 위치 계산
    Some(Rect::new(
        line.bounds.x,
        line.bounds.y,
        line.bounds.width,
        line.bounds.height,
    ))
}

fn render_composing_text(
    commands: &mut RenderCommands,
    layout: &LayoutEngine,
    composing: &crate::ComposingText,
    doc: &Document,
) {
    if let Some(cursor_rect) = layout.position_to_rect(composing.position) {
        // 조합 중 배경
        let style = doc.styles().get_char_style(
            doc.block(composing.position.block_index)
                .and_then(|b| b.as_paragraph())
                .map(|p| p.char_style_at(composing.position.offset))
                .unwrap_or_default()
        ).cloned().unwrap_or_default();

        let text_style = TextStyle::from_char_style(&style, doc.styles());

        // 조합 중 텍스트 배경 (밑줄 효과)
        commands.push(RenderCommand::DrawLine {
            start: Point::new(cursor_rect.x, cursor_rect.y + cursor_rect.height),
            end: Point::new(cursor_rect.x + 50.0, cursor_rect.y + cursor_rect.height),  // 대략적 너비
            color: Color::BLACK,
            width: 1.0,
        });

        // 조합 중 텍스트
        commands.push(RenderCommand::DrawText {
            method: TextRenderMethod::String(StringText {
                text: composing.text.clone(),
                position: Point::new(cursor_rect.x, cursor_rect.y + cursor_rect.height - 4.0),
                style: text_style,
            }),
        });
    }
}
```

---

## 7. 테스트 가이드

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_office_insert_text() {
        let mut office = OfficeCore::new();

        // 문자 입력
        office.handle_event(InputEvent::KeyDown(KeyEvent {
            key: Key::Char('H'),
            modifiers: Modifiers::none(),
        }));
        office.handle_event(InputEvent::KeyDown(KeyEvent {
            key: Key::Char('i'),
            modifiers: Modifiers::none(),
        }));

        assert_eq!(office.document().plain_text(), "Hi");
    }

    #[test]
    fn test_office_undo_redo() {
        let mut office = OfficeCore::new();

        // 입력
        for c in "Hello".chars() {
            office.handle_event(InputEvent::KeyDown(KeyEvent {
                key: Key::Char(c),
                modifiers: Modifiers::none(),
            }));
        }

        assert_eq!(office.document().plain_text(), "Hello");

        // Undo
        office.undo();
        // 명령 병합으로 인해 전체가 취소될 수 있음

        // Redo
        office.redo();
    }

    #[test]
    fn test_cursor_movement() {
        let mut office = OfficeCore::new();

        // 텍스트 입력
        for c in "Hello".chars() {
            office.handle_event(InputEvent::KeyDown(KeyEvent {
                key: Key::Char(c),
                modifiers: Modifiers::none(),
            }));
        }

        // Home
        office.handle_event(InputEvent::KeyDown(KeyEvent {
            key: Key::Home,
            modifiers: Modifiers::none(),
        }));

        let pos = office.selection().primary().cursor_position();
        assert_eq!(pos.offset, 0);

        // End
        office.handle_event(InputEvent::KeyDown(KeyEvent {
            key: Key::End,
            modifiers: Modifiers::none(),
        }));

        let pos = office.selection().primary().cursor_position();
        assert_eq!(pos.offset, 5);
    }

    #[test]
    fn test_selection() {
        let mut office = OfficeCore::new();

        // 텍스트 입력
        for c in "Hello".chars() {
            office.handle_event(InputEvent::KeyDown(KeyEvent {
                key: Key::Char(c),
                modifiers: Modifiers::none(),
            }));
        }

        // 전체 선택
        office.handle_event(InputEvent::KeyDown(KeyEvent {
            key: Key::Char('a'),
            modifiers: Modifiers::ctrl(),
        }));

        let selection = office.selection().primary();
        assert!(!selection.is_collapsed());
    }
}
```

---

## 변경 이력

| 날짜 | 버전 | 내용 |
|------|------|------|
| 2024-12 | 0.1 | 초안 작성 |
