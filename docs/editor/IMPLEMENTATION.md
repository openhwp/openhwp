# Implementation Guide

이 문서는 OpenHWP 에디터의 구현 가이드입니다. 구현 순서와 각 단계별 의존성을 명시합니다.

> **대상 독자**: 이 프로젝트를 구현할 개발자 및 AI 에이전트
>
> **관련 문서**:
> - [DESIGN.md](./DESIGN.md) - 전체 아키텍처
> - [DOCUMENT.md](./DOCUMENT.md) - document 크레이트
> - [LAYOUT.md](./LAYOUT.md) - layout 크레이트
> - [RENDER.md](./RENDER.md) - render 크레이트
> - [EDITOR.md](./EDITOR.md) - editor-core 크레이트
> - [PLATFORM.md](./PLATFORM.md) - 플랫폼 통합

---

## 1. 구현 단계 개요

```
Phase 1: 핵심 데이터 구조
    └── document 크레이트 (Document, Block, TextStorage, Style)

Phase 2: 편집 시스템
    └── edit 모듈 (Command, History, Selection)

Phase 3: 레이아웃 엔진
    └── layout 크레이트 (TextMeasurer, LineBreaker, LayoutEngine)

Phase 4: 렌더링
    └── render-api + render-web (RenderCommand, CanvasRenderer)

Phase 5: 에디터 코어
    └── editor-core (EditorCore, InputEvent handling)

Phase 6: 플랫폼 통합
    └── editor-web + editor-desktop (WebEditorApp, Tauri)
```

---

## 2. Phase 1: 핵심 데이터 구조

### 2.1 목표

편집에 최적화된 문서 데이터 구조를 구현합니다.

### 2.2 구현 순서

```
2.1.1 기본 타입 정의
      ├── Position (block_index, offset)
      ├── Range (start, end)
      └── BlockId (고유 식별자)

2.1.2 스타일 시스템
      ├── CharStyle (폰트, 크기, 굵기 등)
      ├── ParaStyle (정렬, 줄간격, 들여쓰기 등)
      ├── StyleKey (인터닝용 키)
      └── StyleStore (스타일 저장소)

2.1.3 텍스트 저장
      ├── StyleRun (offset, length, style_key)
      └── TextStorage (text, style_runs)

2.1.4 블록 타입
      ├── Paragraph (TextStorage 포함)
      ├── Table (행/열/셀)
      ├── Image (바이너리 참조)
      └── Block enum

2.1.5 문서 구조
      ├── Section (블록 리스트)
      ├── Document (섹션 리스트, 스타일 저장소)
      └── BinaryStore (이미지 등 바이너리)
```

### 2.3 상세 구현 가이드

#### 2.3.1 Position 및 Range

```rust
// crates/document/src/selection/position.rs

/// 문서 내 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    /// 섹션 인덱스
    pub section: usize,
    /// 블록 인덱스
    pub block: usize,
    /// 블록 내 오프셋 (문단의 경우 문자 인덱스)
    pub offset: usize,
}

impl Position {
    pub fn new(section: usize, block: usize, offset: usize) -> Self {
        Self { section, block, offset }
    }

    pub fn start() -> Self {
        Self::new(0, 0, 0)
    }
}

/// 문서 내 범위
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        // start가 항상 end보다 앞에 오도록 정규화
        if start <= end {
            Self { start, end }
        } else {
            Self { start: end, end: start }
        }
    }

    pub fn collapsed(pos: Position) -> Self {
        Self { start: pos, end: pos }
    }

    pub fn is_collapsed(&self) -> bool {
        self.start == self.end
    }
}
```

#### 2.3.2 StyleStore 구현 핵심

```rust
// crates/document/src/model/style_store.rs

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// 스타일 키 (인터닝된 스타일 참조)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StyleKey(u64);

/// 스타일 저장소
pub struct StyleStore {
    /// 문자 스타일: 키 → 스타일
    char_styles: HashMap<StyleKey, CharStyle>,
    /// 해시 → 키 (중복 방지용)
    char_style_lookup: HashMap<u64, StyleKey>,

    /// 문단 스타일: 키 → 스타일
    para_styles: HashMap<StyleKey, ParaStyle>,
    para_style_lookup: HashMap<u64, StyleKey>,

    /// 다음 키 ID
    next_id: u64,
}

impl StyleStore {
    pub fn new() -> Self {
        let mut store = Self {
            char_styles: HashMap::new(),
            char_style_lookup: HashMap::new(),
            para_styles: HashMap::new(),
            para_style_lookup: HashMap::new(),
            next_id: 1,
        };

        // 기본 스타일 등록
        store.register_char_style(CharStyle::default());
        store.register_para_style(ParaStyle::default());

        store
    }

    /// 문자 스타일 등록 또는 기존 키 반환
    pub fn register_char_style(&mut self, style: CharStyle) -> StyleKey {
        let hash = Self::hash_style(&style);

        if let Some(&key) = self.char_style_lookup.get(&hash) {
            return key;
        }

        let key = StyleKey(self.next_id);
        self.next_id += 1;

        self.char_styles.insert(key, style);
        self.char_style_lookup.insert(hash, key);

        key
    }

    /// 문자 스타일 조회
    pub fn get_char_style(&self, key: StyleKey) -> Option<&CharStyle> {
        self.char_styles.get(&key)
    }

    /// 스타일 해시 계산
    fn hash_style<T: Hash>(style: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        style.hash(&mut hasher);
        hasher.finish()
    }

    /// 기본 문자 스타일 키
    pub fn default_char_style_key(&self) -> StyleKey {
        StyleKey(1)
    }

    /// 기본 문단 스타일 키
    pub fn default_para_style_key(&self) -> StyleKey {
        StyleKey(2)
    }
}
```

#### 2.3.3 TextStorage 구현 핵심

```rust
// crates/document/src/model/text_storage.rs

/// 스타일 런 (동일 스타일 구간)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleRun {
    /// 시작 오프셋 (문자 인덱스)
    pub offset: usize,
    /// 길이 (문자 수)
    pub length: usize,
    /// 스타일 키
    pub style_key: StyleKey,
}

/// 텍스트 저장소
#[derive(Debug, Clone)]
pub struct TextStorage {
    /// 텍스트 내용
    text: String,
    /// 스타일 런 리스트 (정렬됨)
    style_runs: Vec<StyleRun>,
}

impl TextStorage {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            style_runs: Vec::new(),
        }
    }

    pub fn with_text(text: impl Into<String>, style_key: StyleKey) -> Self {
        let text = text.into();
        let length = text.chars().count();

        let style_runs = if length > 0 {
            vec![StyleRun { offset: 0, length, style_key }]
        } else {
            Vec::new()
        };

        Self { text, style_runs }
    }

    /// 텍스트 길이 (문자 수)
    pub fn len(&self) -> usize {
        self.text.chars().count()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// 텍스트 삽입
    ///
    /// # Arguments
    /// * `char_offset` - 삽입할 문자 위치
    /// * `text` - 삽입할 텍스트
    /// * `style_key` - 삽입 텍스트의 스타일
    pub fn insert(&mut self, char_offset: usize, text: &str, style_key: StyleKey) {
        if text.is_empty() {
            return;
        }

        // 문자 오프셋을 바이트 오프셋으로 변환
        let byte_offset = self.char_to_byte_offset(char_offset);
        let insert_len = text.chars().count();

        // 텍스트 삽입
        self.text.insert_str(byte_offset, text);

        // 스타일 런 업데이트
        self.insert_style_runs(char_offset, insert_len, style_key);
    }

    /// 텍스트 삭제
    ///
    /// # Arguments
    /// * `char_offset` - 삭제 시작 문자 위치
    /// * `char_count` - 삭제할 문자 수
    pub fn delete(&mut self, char_offset: usize, char_count: usize) {
        if char_count == 0 {
            return;
        }

        let start_byte = self.char_to_byte_offset(char_offset);
        let end_byte = self.char_to_byte_offset(char_offset + char_count);

        // 텍스트 삭제
        self.text.replace_range(start_byte..end_byte, "");

        // 스타일 런 업데이트
        self.delete_style_runs(char_offset, char_count);
    }

    /// 문자 오프셋을 바이트 오프셋으로 변환
    fn char_to_byte_offset(&self, char_offset: usize) -> usize {
        self.text
            .char_indices()
            .nth(char_offset)
            .map(|(i, _)| i)
            .unwrap_or(self.text.len())
    }

    /// 스타일 런 삽입 처리
    fn insert_style_runs(&mut self, offset: usize, length: usize, style_key: StyleKey) {
        // 1. 삽입 위치에 걸쳐 있는 런을 찾아서 분할
        // 2. 새 런 삽입
        // 3. 이후 런들의 오프셋 조정
        // 4. 인접한 동일 스타일 런 병합

        // 구현 상세는 DOCUMENT.md 참조
    }

    /// 스타일 런 삭제 처리
    fn delete_style_runs(&mut self, offset: usize, length: usize) {
        // 1. 삭제 범위와 겹치는 런 조정
        // 2. 완전히 삭제된 런 제거
        // 3. 이후 런들의 오프셋 조정
        // 4. 인접한 동일 스타일 런 병합

        // 구현 상세는 DOCUMENT.md 참조
    }

    /// 범위의 스타일 변경
    pub fn apply_style(&mut self, start: usize, end: usize, style_key: StyleKey) {
        // 1. 범위 시작점에서 기존 런 분할
        // 2. 범위 끝점에서 기존 런 분할
        // 3. 중간 런들의 스타일 변경
        // 4. 인접한 동일 스타일 런 병합
    }

    /// 위치의 스타일 키 조회
    pub fn style_at(&self, char_offset: usize) -> Option<StyleKey> {
        for run in &self.style_runs {
            if char_offset >= run.offset && char_offset < run.offset + run.length {
                return Some(run.style_key);
            }
        }
        None
    }

    /// 스타일 런 이터레이터
    pub fn style_runs(&self) -> impl Iterator<Item = &StyleRun> {
        self.style_runs.iter()
    }
}
```

### 2.4 테스트 케이스

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_text() {
        let style_key = StyleKey(1);
        let mut storage = TextStorage::new();

        storage.insert(0, "Hello", style_key);
        assert_eq!(storage.as_str(), "Hello");
        assert_eq!(storage.len(), 5);

        storage.insert(5, " World", style_key);
        assert_eq!(storage.as_str(), "Hello World");
    }

    #[test]
    fn test_insert_korean() {
        let style_key = StyleKey(1);
        let mut storage = TextStorage::new();

        storage.insert(0, "안녕", style_key);
        assert_eq!(storage.len(), 2); // 문자 수

        storage.insert(1, "하세요", style_key);
        assert_eq!(storage.as_str(), "안하세요녕");
    }

    #[test]
    fn test_delete_text() {
        let style_key = StyleKey(1);
        let mut storage = TextStorage::with_text("Hello World", style_key);

        storage.delete(5, 6); // " World" 삭제
        assert_eq!(storage.as_str(), "Hello");
    }

    #[test]
    fn test_style_runs_merge() {
        // 동일 스타일이 인접하면 병합되어야 함
        let style_key = StyleKey(1);
        let mut storage = TextStorage::new();

        storage.insert(0, "Hello", style_key);
        storage.insert(5, " World", style_key);

        // 하나의 런으로 병합됨
        assert_eq!(storage.style_runs().count(), 1);
    }
}
```

### 2.5 완료 기준

- [ ] `Position`, `Range` 타입 구현 및 테스트
- [ ] `CharStyle`, `ParaStyle` 타입 구현
- [ ] `StyleStore` 인터닝 구현 및 테스트
- [ ] `TextStorage` 삽입/삭제 구현 및 테스트
- [ ] `TextStorage` 스타일 런 관리 테스트
- [ ] `Block` enum (Paragraph, Table, Image)
- [ ] `Section`, `Document` 구조체
- [ ] 한글 (멀티바이트 문자) 처리 테스트

---

## 3. Phase 2: 편집 시스템

### 3.1 목표

Command 패턴 기반의 편집 연산과 Undo/Redo를 구현합니다.

### 3.2 의존성

- Phase 1 완료 필요

### 3.3 구현 순서

```
3.1 Selection 모델
    ├── Selection (anchor, focus)
    ├── SelectionSet (다중 선택 지원)
    └── Selection 연산 (확장, 축소, 이동)

3.2 Command 트레잇
    ├── Command trait (execute, undo)
    ├── CommandResult
    └── DocumentCommand wrapper

3.3 기본 커맨드
    ├── InsertTextCommand
    ├── DeleteCommand
    ├── SplitParagraphCommand
    ├── MergeParagraphCommand
    └── ApplyStyleCommand

3.4 History 시스템
    ├── CommandHistory (undo/redo 스택)
    ├── 커맨드 그룹핑
    └── 히스토리 제한

3.5 Clipboard
    ├── ClipboardData
    ├── 복사 (선택 → ClipboardData)
    └── 붙여넣기 (ClipboardData → 커맨드)
```

### 3.4 상세 구현 가이드

#### 3.4.1 Command 트레잇

```rust
// crates/document/src/edit/command.rs

/// 편집 커맨드 결과
#[derive(Debug)]
pub struct CommandResult {
    /// 성공 여부
    pub success: bool,
    /// 새 커서 위치 (없으면 변경 없음)
    pub new_selection: Option<Selection>,
    /// 영향받은 블록 범위
    pub affected_range: Option<(usize, usize)>,
}

/// 편집 커맨드 트레잇
pub trait Command: std::fmt::Debug {
    /// 커맨드 실행
    fn execute(&mut self, doc: &mut Document) -> CommandResult;

    /// 실행 취소
    fn undo(&mut self, doc: &mut Document) -> CommandResult;

    /// 커맨드 설명 (디버깅/로깅용)
    fn description(&self) -> &str;

    /// 이전 커맨드와 병합 가능 여부
    ///
    /// 예: 연속된 문자 입력은 하나의 Undo 단위로 병합
    fn can_merge(&self, other: &dyn Command) -> bool {
        false
    }

    /// 다른 커맨드와 병합
    fn merge(&mut self, other: Box<dyn Command>) -> bool {
        false
    }
}
```

#### 3.4.2 InsertTextCommand 구현

```rust
// crates/document/src/edit/commands/insert_text.rs

#[derive(Debug)]
pub struct InsertTextCommand {
    /// 삽입 위치
    position: Position,
    /// 삽입할 텍스트
    text: String,
    /// 스타일 키
    style_key: StyleKey,
    /// 이전 상태 (Undo용)
    previous_state: Option<InsertUndoState>,
}

#[derive(Debug)]
struct InsertUndoState {
    /// 삭제할 범위
    delete_range: (usize, usize),
}

impl InsertTextCommand {
    pub fn new(position: Position, text: String, style_key: StyleKey) -> Self {
        Self {
            position,
            text,
            style_key,
            previous_state: None,
        }
    }
}

impl Command for InsertTextCommand {
    fn execute(&mut self, doc: &mut Document) -> CommandResult {
        let section = match doc.sections.get_mut(self.position.section) {
            Some(s) => s,
            None => return CommandResult { success: false, new_selection: None, affected_range: None },
        };

        let block = match section.blocks.get_mut(self.position.block) {
            Some(b) => b,
            None => return CommandResult { success: false, new_selection: None, affected_range: None },
        };

        if let Block::Paragraph(para) = block {
            let start_offset = self.position.offset;
            let insert_len = self.text.chars().count();

            // 텍스트 삽입
            para.text.insert(start_offset, &self.text, self.style_key);

            // Undo 정보 저장
            self.previous_state = Some(InsertUndoState {
                delete_range: (start_offset, start_offset + insert_len),
            });

            // 새 커서 위치
            let new_pos = Position::new(
                self.position.section,
                self.position.block,
                start_offset + insert_len,
            );

            CommandResult {
                success: true,
                new_selection: Some(Selection::collapsed(new_pos)),
                affected_range: Some((self.position.block, self.position.block)),
            }
        } else {
            CommandResult { success: false, new_selection: None, affected_range: None }
        }
    }

    fn undo(&mut self, doc: &mut Document) -> CommandResult {
        let state = match &self.previous_state {
            Some(s) => s,
            None => return CommandResult { success: false, new_selection: None, affected_range: None },
        };

        let section = &mut doc.sections[self.position.section];
        let block = &mut section.blocks[self.position.block];

        if let Block::Paragraph(para) = block {
            let (start, end) = state.delete_range;
            para.text.delete(start, end - start);

            CommandResult {
                success: true,
                new_selection: Some(Selection::collapsed(self.position)),
                affected_range: Some((self.position.block, self.position.block)),
            }
        } else {
            CommandResult { success: false, new_selection: None, affected_range: None }
        }
    }

    fn description(&self) -> &str {
        "텍스트 삽입"
    }

    fn can_merge(&self, other: &dyn Command) -> bool {
        // 연속된 InsertTextCommand인지 확인
        if let Some(other_insert) = other.as_any().downcast_ref::<InsertTextCommand>() {
            // 같은 위치에서 연속 입력이면 병합 가능
            // (구현 상세 생략)
            false
        } else {
            false
        }
    }
}
```

#### 3.4.3 CommandHistory 구현

```rust
// crates/document/src/edit/history.rs

/// 커맨드 히스토리
pub struct CommandHistory {
    /// 실행된 커맨드 (Undo 대상)
    undo_stack: Vec<Box<dyn Command>>,
    /// 취소된 커맨드 (Redo 대상)
    redo_stack: Vec<Box<dyn Command>>,
    /// 최대 히스토리 크기
    max_size: usize,
    /// 현재 그룹 ID (커맨드 그룹핑용)
    current_group: Option<u64>,
    /// 다음 그룹 ID
    next_group_id: u64,
}

impl CommandHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
            current_group: None,
            next_group_id: 1,
        }
    }

    /// 커맨드 실행 및 히스토리에 추가
    pub fn execute(&mut self, mut command: Box<dyn Command>, doc: &mut Document) -> CommandResult {
        let result = command.execute(doc);

        if result.success {
            // Redo 스택 클리어 (새 커맨드 실행 시)
            self.redo_stack.clear();

            // 이전 커맨드와 병합 시도
            let merged = if let Some(last) = self.undo_stack.last_mut() {
                if last.can_merge(command.as_ref()) {
                    last.merge(command)
                } else {
                    false
                }
            } else {
                false
            };

            if !merged {
                self.undo_stack.push(command);
            }

            // 크기 제한 적용
            while self.undo_stack.len() > self.max_size {
                self.undo_stack.remove(0);
            }
        }

        result
    }

    /// 실행 취소
    pub fn undo(&mut self, doc: &mut Document) -> Option<CommandResult> {
        let mut command = self.undo_stack.pop()?;
        let result = command.undo(doc);

        if result.success {
            self.redo_stack.push(command);
        } else {
            // 실패하면 다시 넣기
            self.undo_stack.push(command);
        }

        Some(result)
    }

    /// 다시 실행
    pub fn redo(&mut self, doc: &mut Document) -> Option<CommandResult> {
        let mut command = self.redo_stack.pop()?;
        let result = command.execute(doc);

        if result.success {
            self.undo_stack.push(command);
        } else {
            self.redo_stack.push(command);
        }

        Some(result)
    }

    /// 실행 취소 가능 여부
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// 다시 실행 가능 여부
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// 그룹 시작 (여러 커맨드를 하나의 Undo 단위로)
    pub fn begin_group(&mut self) {
        self.current_group = Some(self.next_group_id);
        self.next_group_id += 1;
    }

    /// 그룹 종료
    pub fn end_group(&mut self) {
        self.current_group = None;
    }
}
```

### 3.5 완료 기준

- [ ] `Selection` 타입 및 연산 구현
- [ ] `Command` 트레잇 정의
- [ ] `InsertTextCommand` 구현 및 테스트
- [ ] `DeleteCommand` 구현 및 테스트
- [ ] `SplitParagraphCommand` 구현 (Enter 키)
- [ ] `MergeParagraphCommand` 구현 (Backspace로 문단 병합)
- [ ] `ApplyStyleCommand` 구현
- [ ] `CommandHistory` 구현 및 테스트
- [ ] 커맨드 병합 (연속 입력) 테스트
- [ ] `ClipboardData` 및 복사/붙여넣기 구현

---

## 4. Phase 3: 레이아웃 엔진

### 4.1 목표

텍스트 측정, 줄바꿈, 페이지 분할 등 레이아웃 기능을 구현합니다.

### 4.2 의존성

- Phase 1 완료 필요 (Document 구조)

### 4.3 구현 순서

```
4.1 텍스트 측정
    ├── TextMeasurer trait
    └── FontMetrics

4.2 줄바꿈
    ├── LineBreaker
    ├── BreakOpportunity
    └── HWP 호환 규칙

4.3 문단 레이아웃
    ├── ParagraphLayout
    ├── LineLayout
    └── RunLayout

4.4 레이아웃 엔진
    ├── LayoutEngine
    ├── LayoutCache
    └── dirty 플래그 관리

4.5 히트 테스트
    ├── 좌표 → Position 변환
    └── Position → 좌표 변환
```

### 4.4 상세 구현 가이드

레이아웃 상세는 [LAYOUT.md](./LAYOUT.md)를 참조하세요.

### 4.5 완료 기준

- [ ] `TextMeasurer` 트레잇 정의
- [ ] `FontMetrics` 구조체
- [ ] `LineBreaker` 구현 (한글 어절/글자 단위 옵션)
- [ ] `ParagraphLayout` 구현
- [ ] `LayoutEngine` 구현
- [ ] dirty 플래그 기반 캐싱 테스트
- [ ] 히트 테스트 (좌표 ↔ Position) 구현 및 테스트

---

## 5. Phase 4: 렌더링

### 5.1 목표

레이아웃 결과를 화면에 그리는 렌더링 시스템을 구현합니다.

### 5.2 의존성

- Phase 3 완료 필요 (Layout 결과 사용)

### 5.3 구현 순서

```
5.1 렌더링 API
    ├── RenderCommand enum
    ├── Renderer trait
    └── Color, Point, Rect 타입

5.2 웹 렌더러
    ├── CanvasRenderer
    ├── CanvasTextMeasurer
    └── ImageCache
```

### 5.4 상세 구현 가이드

렌더링 상세는 [RENDER.md](./RENDER.md)를 참조하세요.

### 5.5 완료 기준

- [ ] `RenderCommand` enum 정의
- [ ] `Renderer` 트레잇 정의
- [ ] `CanvasRenderer` 구현
- [ ] `CanvasTextMeasurer` 구현
- [ ] 텍스트 렌더링 테스트
- [ ] 선택 영역 하이라이트 렌더링
- [ ] 커서 렌더링 (깜빡임 포함)

---

## 6. Phase 5: 에디터 코어

### 6.1 목표

입력 처리와 에디터 로직을 통합하는 코어를 구현합니다.

### 6.2 의존성

- Phase 2 완료 필요 (편집 시스템)
- Phase 3 완료 필요 (레이아웃)
- Phase 4 완료 필요 (렌더링)

### 6.3 구현 순서

```
6.1 입력 이벤트
    ├── InputEvent enum
    ├── Key, MouseButton 타입
    └── Modifiers (Ctrl, Shift, Alt)

6.2 에디터 코어
    ├── EditorCore 구조체
    ├── handle_event() 메서드
    └── UpdateResult

6.3 이벤트 핸들러
    ├── KeyboardHandler
    ├── MouseHandler
    ├── ImeHandler
    └── ClipboardHandler

6.4 뷰포트
    ├── Viewport (스크롤 위치, 크기)
    └── 자동 스크롤 (커서 따라가기)
```

### 6.4 상세 구현 가이드

에디터 코어 상세는 [EDITOR.md](./EDITOR.md)를 참조하세요.

### 6.5 완료 기준

- [ ] `InputEvent` enum 정의
- [ ] `EditorCore` 기본 구조
- [ ] 키보드 핸들러 (방향키, Backspace, Delete 등)
- [ ] 마우스 핸들러 (클릭, 드래그 선택)
- [ ] IME 핸들러 (한글 조합)
- [ ] 클립보드 핸들러
- [ ] 뷰포트 관리 및 자동 스크롤
- [ ] 커서 깜빡임 타이머

---

## 7. Phase 6: 플랫폼 통합

### 7.1 목표

웹 및 데스크톱 플랫폼에 에디터를 통합합니다.

### 7.2 의존성

- Phase 5 완료 필요 (에디터 코어)

### 7.3 구현 순서

```
7.1 플랫폼 API
    ├── PlatformContext trait
    ├── ClipboardAccess trait
    ├── FileSystemAccess trait
    └── Scheduler trait

7.2 웹 플랫폼
    ├── WebEditorApp (wasm-bindgen)
    ├── DOM 이벤트 바인딩
    ├── 숨겨진 textarea (IME)
    └── JavaScript API

7.3 데스크톱 플랫폼
    ├── Tauri 설정
    ├── 메뉴 바
    ├── 파일 연결
    └── 네이티브 다이얼로그
```

### 7.4 상세 구현 가이드

플랫폼 통합 상세는 [PLATFORM.md](./PLATFORM.md)를 참조하세요.

### 7.5 완료 기준

- [ ] `PlatformContext` 트레잇 정의
- [ ] `WebEditorApp` WASM 바인딩
- [ ] DOM 이벤트 → InputEvent 변환
- [ ] 숨겨진 textarea IME 통합
- [ ] JavaScript API 노출
- [ ] TypeScript 타입 정의
- [ ] Tauri 프로젝트 설정
- [ ] 메뉴 바 구현
- [ ] 파일 열기/저장 다이얼로그

---

## 8. 크레이트 의존성 그래프

```
hwp ────────────┐
                │
hwpx ───────────┼───► ir ───► document ───► layout ───► render-api
                │                 │              │              │
                │                 │              │              ▼
                │                 │              │        render-web
                │                 │              │
                │                 ▼              ▼
                │         editor-core ◄─────────┘
                │              │
                │              ▼
                └────► platform-api
                              │
                    ┌─────────┴─────────┐
                    ▼                   ▼
               editor-web        editor-desktop
```

---

## 9. 테스트 전략

### 9.1 단위 테스트

각 크레이트에서 개별 기능을 테스트합니다.

```bash
# 전체 테스트
cargo test

# 특정 크레이트 테스트
cargo test -p document
cargo test -p layout
```

### 9.2 통합 테스트

`tests/` 디렉토리에서 크레이트 간 통합을 테스트합니다.

### 9.3 E2E 테스트

웹 에디터의 경우 Playwright나 Cypress를 사용합니다.

### 9.4 성능 테스트

```rust
#[cfg(test)]
mod benchmarks {
    use test::Bencher;

    #[bench]
    fn bench_insert_text(b: &mut Bencher) {
        let mut doc = Document::new();
        // ...
        b.iter(|| {
            // 텍스트 삽입 벤치마크
        });
    }
}
```

---

## 10. 코드 스타일 가이드

### 10.1 일반 규칙

- `rustfmt` 사용 (기본 설정)
- `clippy` 경고 해결
- 모든 public API에 문서 주석

### 10.2 네이밍 규칙

| 항목 | 규칙 | 예시 |
|------|------|------|
| 타입 | PascalCase | `TextStorage`, `StyleRun` |
| 함수 | snake_case | `insert_text()`, `get_style()` |
| 상수 | SCREAMING_SNAKE_CASE | `MAX_UNDO_STACK` |
| 모듈 | snake_case | `text_storage`, `style_store` |

### 10.3 에러 처리

```rust
/// 커스텀 에러 타입
#[derive(Debug, thiserror::Error)]
pub enum DocumentError {
    #[error("Invalid position: {0:?}")]
    InvalidPosition(Position),

    #[error("Block not found: {0}")]
    BlockNotFound(usize),

    #[error("Style not found: {0:?}")]
    StyleNotFound(StyleKey),
}

pub type Result<T> = std::result::Result<T, DocumentError>;
```

---

## 11. 문서화 요구사항

### 11.1 코드 문서

모든 public 항목에 문서 주석을 작성합니다.

```rust
/// 텍스트 저장소
///
/// 문단의 텍스트와 스타일 정보를 관리합니다.
///
/// # Example
///
/// ```
/// let mut storage = TextStorage::new();
/// storage.insert(0, "Hello", default_style);
/// ```
pub struct TextStorage { ... }
```

### 11.2 변경 로그

주요 변경사항은 CHANGELOG.md에 기록합니다.

---

## 12. 체크리스트 요약

### Phase 1: 핵심 데이터 구조
- [ ] Position, Range
- [ ] CharStyle, ParaStyle
- [ ] StyleStore
- [ ] TextStorage
- [ ] Block (Paragraph, Table, Image)
- [ ] Section, Document

### Phase 2: 편집 시스템
- [ ] Selection
- [ ] Command trait
- [ ] InsertTextCommand
- [ ] DeleteCommand
- [ ] SplitParagraphCommand
- [ ] MergeParagraphCommand
- [ ] ApplyStyleCommand
- [ ] CommandHistory
- [ ] ClipboardData

### Phase 3: 레이아웃 엔진
- [ ] TextMeasurer trait
- [ ] FontMetrics
- [ ] LineBreaker
- [ ] ParagraphLayout
- [ ] LayoutEngine
- [ ] 히트 테스트

### Phase 4: 렌더링
- [ ] RenderCommand
- [ ] Renderer trait
- [ ] CanvasRenderer
- [ ] CanvasTextMeasurer

### Phase 5: 에디터 코어
- [ ] InputEvent
- [ ] EditorCore
- [ ] KeyboardHandler
- [ ] MouseHandler
- [ ] ImeHandler
- [ ] ClipboardHandler
- [ ] Viewport

### Phase 6: 플랫폼 통합
- [ ] PlatformContext
- [ ] WebEditorApp
- [ ] DOM 이벤트 바인딩
- [ ] Tauri 통합
