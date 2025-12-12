# document 크레이트 상세 명세

이 문서는 `document` 크레이트의 구현 상세를 정의합니다.

> **상위 문서**: [DESIGN.md](./DESIGN.md)

---

## 1. 개요

### 1.1 목적

`document` 크레이트는 **편집에 최적화된 문서 모델**을 제공합니다.

- 빠른 편집 연산 (삽입, 삭제, 서식 변경)
- Command 패턴 기반 Undo/Redo
- 선택 영역 관리
- IR과의 양방향 변환

### 1.2 IR과의 차이

| 구분 | IR (`ir` 크레이트) | Document (`document` 크레이트) |
|------|--------------------|-------------------------------|
| 목적 | 파일 포맷 변환 | 편집 최적화 |
| 사용 시점 | 파일 열기/저장 | 편집 중 상시 |
| 설계 기준 | HWP/HWPX 합집합 | 편집 성능 |
| 스타일 저장 | 파일 구조 반영 | 인터닝 (중복 제거) |
| 레이아웃 정보 | 없음 | dirty 플래그 포함 |

---

## 2. 파일 구조

```
crates/document/
├── Cargo.toml
└── src/
    ├── lib.rs                    # pub use 및 크레이트 루트
    │
    ├── model/                    # 문서 구조 모델
    │   ├── mod.rs
    │   ├── document.rs           # Document 구조체
    │   ├── block.rs              # Block enum
    │   ├── paragraph.rs          # Paragraph 구조체
    │   ├── table.rs              # Table, Row, Cell
    │   ├── image.rs              # ImageBlock
    │   ├── shape.rs              # ShapeBlock
    │   ├── inline_object.rs      # InlineObject enum
    │   ├── text_storage.rs       # TextStorage enum
    │   ├── style.rs              # CharStyle, ParaStyle
    │   ├── style_runs.rs         # StyleRuns (속성 구간)
    │   ├── style_store.rs        # StyleStore (인터닝)
    │   └── binary_store.rs       # BinaryStore (이미지 등)
    │
    ├── selection/                # 선택 영역
    │   ├── mod.rs
    │   ├── position.rs           # Position
    │   ├── selection.rs          # Selection, SelectionSet
    │   └── range.rs              # DocumentRange
    │
    ├── edit/                     # 편집 연산
    │   ├── mod.rs
    │   ├── command.rs            # Command trait
    │   ├── commands/             # 구체 Command 구현
    │   │   ├── mod.rs
    │   │   ├── insert_text.rs
    │   │   ├── delete.rs
    │   │   ├── delete_range.rs
    │   │   ├── split_paragraph.rs
    │   │   ├── merge_paragraph.rs
    │   │   ├── format_char.rs
    │   │   ├── format_para.rs
    │   │   ├── insert_block.rs
    │   │   ├── delete_block.rs
    │   │   └── batch.rs
    │   ├── history.rs            # CommandHistory
    │   └── clipboard.rs          # ClipboardData
    │
    └── convert/                  # IR 변환
        ├── mod.rs
        ├── from_ir.rs            # IR → Document
        └── to_ir.rs              # Document → IR
```

---

## 3. 핵심 타입

### 3.1 Document

문서의 루트 구조체입니다.

```rust
// src/model/document.rs

use std::collections::HashMap;

/// 편집용 문서 모델
#[derive(Debug, Clone)]
pub struct Document {
    /// 문서 ID (고유 식별자)
    id: DocumentId,

    /// 블록 목록 (문단, 표, 이미지 등)
    blocks: Vec<Block>,

    /// 블록 ID → 인덱스 매핑 (안정적 참조용)
    block_index: HashMap<BlockId, usize>,

    /// 스타일 저장소
    styles: StyleStore,

    /// 바이너리 데이터 저장소 (이미지 등)
    binaries: BinaryStore,

    /// 문서 메타데이터
    metadata: DocumentMetadata,

    /// 다음 블록 ID
    next_block_id: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DocumentId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u64);

#[derive(Debug, Clone, Default)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
}

impl Document {
    /// 빈 문서 생성
    pub fn new() -> Self {
        let mut doc = Self {
            id: DocumentId(0),
            blocks: Vec::new(),
            block_index: HashMap::new(),
            styles: StyleStore::new(),
            binaries: BinaryStore::new(),
            metadata: DocumentMetadata::default(),
            next_block_id: 1,
        };

        // 기본 빈 문단 추가
        let para = Paragraph::new(BlockId(0));
        doc.blocks.push(Block::Paragraph(para));
        doc.block_index.insert(BlockId(0), 0);

        doc
    }

    /// IR에서 변환
    pub fn from_ir(ir: ir::Document) -> Result<Self, ConvertError> {
        convert::from_ir::convert(ir)
    }

    /// IR로 변환
    pub fn to_ir(&self) -> Result<ir::Document, ConvertError> {
        convert::to_ir::convert(self)
    }

    // === 블록 접근 ===

    /// 블록 개수
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// 인덱스로 블록 접근
    pub fn block(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }

    /// 인덱스로 블록 수정 접근
    pub fn block_mut(&mut self, index: usize) -> Option<&mut Block> {
        self.blocks.get_mut(index)
    }

    /// ID로 블록 접근
    pub fn block_by_id(&self, id: BlockId) -> Option<&Block> {
        self.block_index.get(&id).and_then(|&i| self.blocks.get(i))
    }

    /// 블록 삽입
    pub fn insert_block(&mut self, index: usize, block: Block) -> BlockId {
        let id = BlockId(self.next_block_id);
        self.next_block_id += 1;

        self.blocks.insert(index, block);
        self.rebuild_block_index();

        id
    }

    /// 블록 삭제
    pub fn remove_block(&mut self, index: usize) -> Option<Block> {
        if index < self.blocks.len() {
            let block = self.blocks.remove(index);
            self.rebuild_block_index();
            Some(block)
        } else {
            None
        }
    }

    fn rebuild_block_index(&mut self) {
        self.block_index.clear();
        for (i, block) in self.blocks.iter().enumerate() {
            self.block_index.insert(block.id(), i);
        }
    }

    // === 스타일 접근 ===

    pub fn styles(&self) -> &StyleStore {
        &self.styles
    }

    pub fn styles_mut(&mut self) -> &mut StyleStore {
        &mut self.styles
    }

    // === 바이너리 접근 ===

    pub fn binaries(&self) -> &BinaryStore {
        &self.binaries
    }

    pub fn binaries_mut(&mut self) -> &mut BinaryStore {
        &mut self.binaries
    }

    // === 텍스트 접근 (디버깅/테스트용) ===

    /// 전체 문서의 평문 텍스트 추출
    pub fn plain_text(&self) -> String {
        let mut result = String::new();
        for (i, block) in self.blocks.iter().enumerate() {
            if i > 0 {
                result.push('\n');
            }
            match block {
                Block::Paragraph(para) => result.push_str(para.text()),
                Block::Table(table) => result.push_str(&table.plain_text()),
                _ => {}
            }
        }
        result
    }
}
```

### 3.2 Block

블록은 문서를 구성하는 최상위 요소입니다.

```rust
// src/model/block.rs

/// 블록 (문서의 최상위 요소)
#[derive(Debug, Clone)]
pub enum Block {
    /// 문단
    Paragraph(Paragraph),
    /// 표
    Table(Table),
    /// 이미지 (블록 레벨)
    Image(ImageBlock),
    /// 도형 (블록 레벨)
    Shape(ShapeBlock),
}

impl Block {
    /// 블록 ID 반환
    pub fn id(&self) -> BlockId {
        match self {
            Block::Paragraph(p) => p.id,
            Block::Table(t) => t.id,
            Block::Image(i) => i.id,
            Block::Shape(s) => s.id,
        }
    }

    /// 문단으로 변환 (문단이 아니면 None)
    pub fn as_paragraph(&self) -> Option<&Paragraph> {
        match self {
            Block::Paragraph(p) => Some(p),
            _ => None,
        }
    }

    pub fn as_paragraph_mut(&mut self) -> Option<&mut Paragraph> {
        match self {
            Block::Paragraph(p) => Some(p),
            _ => None,
        }
    }

    /// 표로 변환
    pub fn as_table(&self) -> Option<&Table> {
        match self {
            Block::Table(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        match self {
            Block::Table(t) => Some(t),
            _ => None,
        }
    }

    /// 레이아웃 dirty 여부
    pub fn is_layout_dirty(&self) -> bool {
        match self {
            Block::Paragraph(p) => p.is_layout_dirty(),
            Block::Table(t) => t.is_layout_dirty(),
            Block::Image(i) => i.is_layout_dirty(),
            Block::Shape(s) => s.is_layout_dirty(),
        }
    }

    /// dirty 플래그 설정
    pub fn mark_layout_dirty(&mut self) {
        match self {
            Block::Paragraph(p) => p.mark_layout_dirty(),
            Block::Table(t) => t.mark_layout_dirty(),
            Block::Image(i) => i.mark_layout_dirty(),
            Block::Shape(s) => s.mark_layout_dirty(),
        }
    }
}
```

### 3.3 Paragraph

문단은 텍스트와 인라인 객체를 포함합니다.

```rust
// src/model/paragraph.rs

use std::collections::BTreeMap;

/// 문단
#[derive(Debug, Clone)]
pub struct Paragraph {
    /// 블록 ID
    pub id: BlockId,

    /// 텍스트 저장소
    text: TextStorage,

    /// 문자 스타일 구간
    char_styles: StyleRuns,

    /// 문단 스타일 ID
    para_style: ParaStyleId,

    /// 인라인 객체 (오프셋 → 객체)
    /// 텍스트에서 U+FFFC 위치에 대응
    inline_objects: BTreeMap<usize, InlineObject>,

    /// 레이아웃 dirty 플래그
    layout_dirty: bool,
}

impl Paragraph {
    /// 빈 문단 생성
    pub fn new(id: BlockId) -> Self {
        Self {
            id,
            text: TextStorage::new(),
            char_styles: StyleRuns::new(),
            para_style: ParaStyleId::default(),
            inline_objects: BTreeMap::new(),
            layout_dirty: true,
        }
    }

    /// 텍스트와 함께 생성
    pub fn with_text(id: BlockId, text: &str) -> Self {
        Self {
            id,
            text: TextStorage::from_str(text),
            char_styles: StyleRuns::new(),
            para_style: ParaStyleId::default(),
            inline_objects: BTreeMap::new(),
            layout_dirty: true,
        }
    }

    // === 텍스트 접근 ===

    /// 텍스트 참조
    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    /// 텍스트 길이 (바이트)
    pub fn text_len(&self) -> usize {
        self.text.len()
    }

    /// 텍스트 길이 (문자 수)
    pub fn char_count(&self) -> usize {
        self.text.as_str().chars().count()
    }

    /// 비어있는지 확인
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    // === 텍스트 편집 ===

    /// 텍스트 삽입
    ///
    /// # Arguments
    /// * `offset` - 바이트 오프셋 (char boundary여야 함)
    /// * `s` - 삽입할 문자열
    pub fn insert_text(&mut self, offset: usize, s: &str) {
        // 1. 텍스트 삽입
        self.text.insert(offset, s);

        // 2. 스타일 구간 조정
        self.char_styles.shift_after(offset, s.len() as isize);

        // 3. 인라인 객체 오프셋 조정
        self.shift_inline_objects(offset, s.len() as isize);

        // 4. dirty 표시
        self.layout_dirty = true;
    }

    /// 텍스트 삭제
    ///
    /// # Arguments
    /// * `start` - 시작 바이트 오프셋
    /// * `end` - 끝 바이트 오프셋 (exclusive)
    pub fn delete_text(&mut self, start: usize, end: usize) {
        let len = end - start;

        // 1. 범위 내 인라인 객체 제거
        self.inline_objects.retain(|&offset, _| {
            offset < start || offset >= end
        });

        // 2. 텍스트 삭제
        self.text.delete(start, end);

        // 3. 스타일 구간 조정
        self.char_styles.delete_range(start, end);
        self.char_styles.shift_after(start, -(len as isize));

        // 4. 인라인 객체 오프셋 조정
        self.shift_inline_objects(start, -(len as isize));

        // 5. dirty 표시
        self.layout_dirty = true;
    }

    fn shift_inline_objects(&mut self, after: usize, delta: isize) {
        let mut new_objects = BTreeMap::new();
        for (offset, obj) in self.inline_objects.iter() {
            let new_offset = if *offset >= after {
                (*offset as isize + delta) as usize
            } else {
                *offset
            };
            new_objects.insert(new_offset, obj.clone());
        }
        self.inline_objects = new_objects;
    }

    // === 스타일 ===

    /// 문자 스타일 구간 참조
    pub fn char_styles(&self) -> &StyleRuns {
        &self.char_styles
    }

    pub fn char_styles_mut(&mut self) -> &mut StyleRuns {
        &mut self.char_styles
    }

    /// 특정 오프셋의 문자 스타일 ID
    pub fn char_style_at(&self, offset: usize) -> CharStyleId {
        self.char_styles.style_at(offset)
    }

    /// 범위에 문자 스타일 적용
    pub fn apply_char_style(&mut self, start: usize, end: usize, style_id: CharStyleId) {
        self.char_styles.apply(start, end, style_id);
        self.layout_dirty = true;
    }

    /// 문단 스타일 ID
    pub fn para_style(&self) -> ParaStyleId {
        self.para_style
    }

    /// 문단 스타일 설정
    pub fn set_para_style(&mut self, style_id: ParaStyleId) {
        self.para_style = style_id;
        self.layout_dirty = true;
    }

    // === 인라인 객체 ===

    /// 인라인 객체 참조
    pub fn inline_objects(&self) -> &BTreeMap<usize, InlineObject> {
        &self.inline_objects
    }

    /// 인라인 객체 삽입
    pub fn insert_inline_object(&mut self, offset: usize, obj: InlineObject) {
        // U+FFFC (Object Replacement Character) 삽입
        self.insert_text(offset, "\u{FFFC}");
        self.inline_objects.insert(offset, obj);
    }

    // === 레이아웃 ===

    pub fn is_layout_dirty(&self) -> bool {
        self.layout_dirty
    }

    pub fn mark_layout_dirty(&mut self) {
        self.layout_dirty = true;
    }

    pub fn mark_layout_clean(&mut self) {
        self.layout_dirty = false;
    }
}
```

### 3.4 TextStorage

텍스트 저장을 위한 자료구조입니다.

```rust
// src/model/text_storage.rs

/// 텍스트 저장소
///
/// 대부분의 문단은 짧으므로 (90%가 500자 미만) String 사용.
/// 64KB 이상의 긴 문단만 청크 분할.
#[derive(Debug, Clone)]
pub enum TextStorage {
    /// 일반적인 경우 (< 64KB)
    Inline(String),
    /// 매우 긴 문단 (>= 64KB)
    Chunked(ChunkedText),
}

const CHUNK_THRESHOLD: usize = 64 * 1024; // 64KB
const CHUNK_SIZE: usize = 32 * 1024;      // 32KB

#[derive(Debug, Clone)]
pub struct ChunkedText {
    chunks: Vec<String>,
    total_len: usize,
}

impl TextStorage {
    pub fn new() -> Self {
        TextStorage::Inline(String::new())
    }

    pub fn from_str(s: &str) -> Self {
        if s.len() < CHUNK_THRESHOLD {
            TextStorage::Inline(s.to_string())
        } else {
            TextStorage::Chunked(ChunkedText::from_str(s))
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            TextStorage::Inline(s) => s,
            TextStorage::Chunked(c) => {
                // 주의: 이 구현은 임시. 실제로는 Cow나 별도 버퍼 필요
                // 여기서는 단순화를 위해 panic
                panic!("Chunked text requires explicit concatenation")
            }
        }
    }

    /// 전체 텍스트를 String으로 반환 (청크된 경우 병합)
    pub fn to_string(&self) -> String {
        match self {
            TextStorage::Inline(s) => s.clone(),
            TextStorage::Chunked(c) => c.to_string(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            TextStorage::Inline(s) => s.len(),
            TextStorage::Chunked(c) => c.total_len,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(&mut self, offset: usize, s: &str) {
        match self {
            TextStorage::Inline(text) => {
                text.insert_str(offset, s);

                // 임계치 초과 시 청크로 전환
                if text.len() >= CHUNK_THRESHOLD {
                    *self = TextStorage::Chunked(ChunkedText::from_str(text));
                }
            }
            TextStorage::Chunked(chunks) => {
                chunks.insert(offset, s);
            }
        }
    }

    pub fn delete(&mut self, start: usize, end: usize) {
        match self {
            TextStorage::Inline(text) => {
                text.drain(start..end);
            }
            TextStorage::Chunked(chunks) => {
                chunks.delete(start, end);

                // 임계치 미만이면 인라인으로 전환
                if chunks.total_len < CHUNK_THRESHOLD / 2 {
                    *self = TextStorage::Inline(chunks.to_string());
                }
            }
        }
    }
}

impl ChunkedText {
    fn from_str(s: &str) -> Self {
        let mut chunks = Vec::new();
        let mut remaining = s;

        while !remaining.is_empty() {
            let chunk_end = remaining
                .char_indices()
                .take_while(|(i, _)| *i < CHUNK_SIZE)
                .last()
                .map(|(i, c)| i + c.len_utf8())
                .unwrap_or(remaining.len());

            chunks.push(remaining[..chunk_end].to_string());
            remaining = &remaining[chunk_end..];
        }

        Self {
            total_len: s.len(),
            chunks,
        }
    }

    fn to_string(&self) -> String {
        self.chunks.concat()
    }

    fn insert(&mut self, offset: usize, s: &str) {
        // 오프셋이 속한 청크 찾기
        let mut current_offset = 0;
        for chunk in &mut self.chunks {
            if offset <= current_offset + chunk.len() {
                let local_offset = offset - current_offset;
                chunk.insert_str(local_offset, s);
                self.total_len += s.len();

                // 청크가 너무 커지면 분할
                if chunk.len() > CHUNK_SIZE * 2 {
                    // TODO: 청크 분할 로직
                }
                return;
            }
            current_offset += chunk.len();
        }
    }

    fn delete(&mut self, start: usize, end: usize) {
        // 단순화된 구현: 전체를 문자열로 변환 후 삭제 후 재구성
        let mut text = self.to_string();
        text.drain(start..end);
        *self = ChunkedText::from_str(&text);
    }
}

impl Default for TextStorage {
    fn default() -> Self {
        Self::new()
    }
}
```

### 3.5 InlineObject

텍스트 내에 삽입되는 인라인 객체입니다.

```rust
// src/model/inline_object.rs

/// 인라인 객체 (텍스트 흐름 내 삽입)
#[derive(Debug, Clone)]
pub enum InlineObject {
    /// 인라인 이미지
    Image(InlineImage),
    /// 필드 (페이지 번호, 날짜 등)
    Field(Field),
    /// 각주 참조
    FootnoteRef(FootnoteRef),
    /// 미주 참조
    EndnoteRef(EndnoteRef),
    /// 책갈피
    Bookmark(Bookmark),
    /// 하이퍼링크
    Hyperlink(Hyperlink),
}

#[derive(Debug, Clone)]
pub struct InlineImage {
    pub binary_id: BinaryId,
    pub width: f32,
    pub height: f32,
    pub alt_text: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Field {
    PageNumber,
    TotalPages,
    Date { format: String },
    Time { format: String },
    FileName,
    Author,
    Custom { name: String, value: String },
}

#[derive(Debug, Clone)]
pub struct FootnoteRef {
    pub footnote_id: u32,
}

#[derive(Debug, Clone)]
pub struct EndnoteRef {
    pub endnote_id: u32,
}

#[derive(Debug, Clone)]
pub struct Bookmark {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Hyperlink {
    pub url: String,
    pub tooltip: Option<String>,
}
```

---

## 4. 스타일 시스템

### 4.1 CharStyle

문자 스타일은 **모든 속성을 명시적으로 지정**합니다 (상속 없음).

```rust
// src/model/style.rs

/// 문자 스타일 ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CharStyleId(pub u32);

/// 문단 스타일 ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ParaStyleId(pub u32);

/// 문자 스타일 (모든 속성 필수 - 상속 없음)
#[derive(Debug, Clone, PartialEq)]
pub struct CharStyle {
    // === 글꼴 ===
    /// 글꼴 ID
    pub font_family: FontId,
    /// 글꼴 크기 (HwpUnit, 1pt = 100 HwpUnit)
    pub font_size: i32,
    /// 글꼴 두께
    pub font_weight: FontWeight,
    /// 글꼴 스타일 (기울임)
    pub font_style: FontStyle,

    // === 색상 ===
    /// 글자색
    pub color: Color,
    /// 배경색 (None = 투명)
    pub background: Option<Color>,
    /// 밑줄
    pub underline: Option<UnderlineStyle>,
    /// 취소선
    pub strikethrough: Option<StrikethroughStyle>,

    // === 위치 ===
    /// 위 첨자
    pub superscript: bool,
    /// 아래 첨자
    pub subscript: bool,

    // === HWP 특수 ===
    /// 자간 (글자 사이 간격, HwpUnit)
    pub letter_spacing: i32,
    /// 장평 (가로 비율, 100 = 100%)
    pub width_ratio: u32,
    /// 글자 위치 조정 (기준선 기준, HwpUnit)
    pub baseline_offset: i32,

    // === 효과 ===
    /// 그림자
    pub shadow: Option<TextShadow>,
    /// 외곽선
    pub outline: Option<TextOutline>,
    /// 양각/음각
    pub emboss: Option<EmbossStyle>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FontId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,       // 100
    ExtraLight, // 200
    Light,      // 300
    Normal,     // 400
    Medium,     // 500
    SemiBold,   // 600
    Bold,       // 700
    ExtraBold,  // 800
    Black,      // 900
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnderlineStyle {
    pub style: LineStyle,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StrikethroughStyle {
    pub style: LineStyle,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineStyle {
    Solid,
    Dashed,
    Dotted,
    Double,
    Wavy,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextShadow {
    pub offset_x: i32,
    pub offset_y: i32,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextOutline {
    pub width: i32,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbossStyle {
    Emboss,   // 양각
    Engrave,  // 음각
}

impl CharStyle {
    /// 기본 스타일 생성
    pub fn default_style() -> Self {
        Self {
            font_family: FontId(0),
            font_size: 1000,  // 10pt
            font_weight: FontWeight::Normal,
            font_style: FontStyle::Normal,
            color: Color::BLACK,
            background: None,
            underline: None,
            strikethrough: None,
            superscript: false,
            subscript: false,
            letter_spacing: 0,
            width_ratio: 100,
            baseline_offset: 0,
            shadow: None,
            outline: None,
            emboss: None,
        }
    }

    /// 굵게 적용
    pub fn with_bold(mut self) -> Self {
        self.font_weight = FontWeight::Bold;
        self
    }

    /// 기울임 적용
    pub fn with_italic(mut self) -> Self {
        self.font_style = FontStyle::Italic;
        self
    }

    /// 글꼴 크기 설정
    pub fn with_font_size(mut self, size: i32) -> Self {
        self.font_size = size;
        self
    }

    /// 색상 설정
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Default for CharStyle {
    fn default() -> Self {
        Self::default_style()
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Normal
    }
}
```

### 4.2 ParaStyle

문단 스타일입니다.

```rust
// src/model/style.rs (계속)

/// 문단 스타일 (모든 속성 필수 - 상속 없음)
#[derive(Debug, Clone, PartialEq)]
pub struct ParaStyle {
    // === 정렬 ===
    /// 가로 정렬
    pub alignment: Alignment,
    /// 세로 정렬 (표 셀 내)
    pub vertical_alignment: VerticalAlignment,

    // === 들여쓰기 ===
    /// 왼쪽 들여쓰기 (HwpUnit)
    pub indent_left: i32,
    /// 오른쪽 들여쓰기 (HwpUnit)
    pub indent_right: i32,
    /// 첫 줄 들여쓰기 (HwpUnit, 음수면 내어쓰기)
    pub indent_first: i32,

    // === 간격 ===
    /// 문단 앞 간격 (HwpUnit)
    pub space_before: i32,
    /// 문단 뒤 간격 (HwpUnit)
    pub space_after: i32,
    /// 줄 간격
    pub line_spacing: LineSpacing,

    // === 줄바꿈 ===
    /// 줄바꿈 설정
    pub line_break: LineBreakConfig,

    // === 탭 ===
    /// 탭 정지점
    pub tab_stops: Vec<TabStop>,

    // === 테두리/배경 ===
    /// 문단 테두리
    pub border: Option<Border>,
    /// 문단 배경
    pub background: Option<Background>,

    // === 기타 ===
    /// 페이지 나누기 방지 (widows/orphans)
    pub keep_lines_together: bool,
    /// 다음 문단과 함께
    pub keep_with_next: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
    Distribute,  // 균등 배분
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineSpacing {
    /// 배수 (100 = 1배, 150 = 1.5배)
    Multiple(u32),
    /// 고정 (HwpUnit)
    Fixed(i32),
    /// 최소 (HwpUnit)
    AtLeast(i32),
}

/// 줄바꿈 설정 (HWP 호환)
#[derive(Debug, Clone, PartialEq)]
pub struct LineBreakConfig {
    /// 한글 줄바꿈 단위
    pub korean_unit: KoreanLineBreakUnit,
    /// 영어 줄바꿈 단위
    pub english_unit: EnglishLineBreakUnit,
    /// 행두 금지 문자
    pub no_line_start_chars: String,
    /// 행말 금지 문자
    pub no_line_end_chars: String,
    /// 낱자 방지 (외톨이 글자)
    pub prevent_orphan_char: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KoreanLineBreakUnit {
    /// 어절 단위 (공백 기준)
    Word,
    /// 글자 단위
    Character,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnglishLineBreakUnit {
    /// 단어 단위
    Word,
    /// 하이픈 허용
    Hyphenate,
    /// 글자 단위
    Character,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TabStop {
    /// 위치 (HwpUnit)
    pub position: i32,
    /// 정렬
    pub alignment: TabAlignment,
    /// 채움 문자
    pub leader: Option<char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabAlignment {
    Left,
    Center,
    Right,
    Decimal,  // 소수점 정렬
}

#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    pub top: Option<BorderLine>,
    pub right: Option<BorderLine>,
    pub bottom: Option<BorderLine>,
    pub left: Option<BorderLine>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BorderLine {
    pub style: LineStyle,
    pub width: i32,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Background {
    Color(Color),
    // 추후: 그라데이션, 패턴 등
}

impl ParaStyle {
    pub fn default_style() -> Self {
        Self {
            alignment: Alignment::Left,
            vertical_alignment: VerticalAlignment::Top,
            indent_left: 0,
            indent_right: 0,
            indent_first: 0,
            space_before: 0,
            space_after: 0,
            line_spacing: LineSpacing::Multiple(160),  // 160%
            line_break: LineBreakConfig::default(),
            tab_stops: Vec::new(),
            border: None,
            background: None,
            keep_lines_together: false,
            keep_with_next: false,
        }
    }
}

impl Default for ParaStyle {
    fn default() -> Self {
        Self::default_style()
    }
}

impl Default for LineBreakConfig {
    fn default() -> Self {
        Self {
            korean_unit: KoreanLineBreakUnit::Word,
            english_unit: EnglishLineBreakUnit::Word,
            no_line_start_chars: ")]},.!?;:".to_string(),
            no_line_end_chars: "([{".to_string(),
            prevent_orphan_char: true,
        }
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Left
    }
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        VerticalAlignment::Top
    }
}
```

### 4.3 StyleRuns

문자 스타일 구간을 관리합니다.

```rust
// src/model/style_runs.rs

/// 문자 스타일 구간 (Run-Length Encoding)
///
/// 텍스트의 각 구간에 적용된 스타일 ID를 저장합니다.
/// 예: "안녕하세요" (0-5: 기본), "반갑습니다" (5-10: 굵게)
#[derive(Debug, Clone)]
pub struct StyleRuns {
    /// (시작 오프셋, 스타일 ID) 쌍의 정렬된 리스트
    /// 각 항목은 해당 오프셋부터 다음 항목 전까지의 스타일을 나타냄
    runs: Vec<(usize, CharStyleId)>,
}

impl StyleRuns {
    pub fn new() -> Self {
        // 기본 스타일로 시작
        Self {
            runs: vec![(0, CharStyleId::default())],
        }
    }

    /// 특정 오프셋의 스타일 ID 반환
    pub fn style_at(&self, offset: usize) -> CharStyleId {
        // 이진 검색으로 해당 오프셋이 속한 구간 찾기
        match self.runs.binary_search_by_key(&offset, |&(o, _)| o) {
            Ok(i) => self.runs[i].1,
            Err(i) => {
                if i > 0 {
                    self.runs[i - 1].1
                } else {
                    CharStyleId::default()
                }
            }
        }
    }

    /// 범위에 스타일 적용
    pub fn apply(&mut self, start: usize, end: usize, style_id: CharStyleId) {
        if start >= end {
            return;
        }

        // 1. 시작점 이전의 스타일 보존
        let before_style = self.style_at(start);

        // 2. 끝점 이후의 스타일 보존
        let after_style = self.style_at(end);

        // 3. 영향받는 구간 제거
        self.runs.retain(|&(offset, _)| offset < start || offset >= end);

        // 4. 새 구간 추가
        // 시작점
        if start > 0 && before_style != style_id {
            self.insert_run(start, style_id);
        } else if start == 0 {
            self.insert_run(0, style_id);
        }

        // 끝점 (다른 스타일로 복원)
        if after_style != style_id {
            self.insert_run(end, after_style);
        }

        // 5. 정렬 및 중복 제거
        self.normalize();
    }

    /// 텍스트 삽입 시 오프셋 조정
    pub fn shift_after(&mut self, after: usize, delta: isize) {
        for (offset, _) in &mut self.runs {
            if *offset > after {
                *offset = (*offset as isize + delta) as usize;
            }
        }
    }

    /// 범위 삭제 시 구간 조정
    pub fn delete_range(&mut self, start: usize, end: usize) {
        // 범위 내의 구간 시작점 제거
        self.runs.retain(|&(offset, _)| offset < start || offset >= end);
    }

    /// 모든 구간 순회
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, CharStyleId)> + '_ {
        self.runs.windows(2).map(|w| (w[0].0, w[1].0, w[0].1))
            .chain(
                self.runs.last().map(|&(start, id)| (start, usize::MAX, id))
            )
    }

    fn insert_run(&mut self, offset: usize, style_id: CharStyleId) {
        match self.runs.binary_search_by_key(&offset, |&(o, _)| o) {
            Ok(i) => self.runs[i].1 = style_id,
            Err(i) => self.runs.insert(i, (offset, style_id)),
        }
    }

    fn normalize(&mut self) {
        // 정렬
        self.runs.sort_by_key(|&(o, _)| o);

        // 연속된 동일 스타일 병합
        self.runs.dedup_by(|a, b| a.1 == b.1);
    }
}

impl Default for StyleRuns {
    fn default() -> Self {
        Self::new()
    }
}
```

### 4.4 StyleStore

스타일 인터닝을 통해 중복 스타일 객체를 공유합니다.

```rust
// src/model/style_store.rs

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// 스타일 저장소 (인터닝)
///
/// 동일한 스타일은 같은 ID를 공유하여 메모리 절약
#[derive(Debug, Clone)]
pub struct StyleStore {
    /// 문자 스타일 목록
    char_styles: Vec<CharStyle>,
    /// 문자 스타일 해시 → ID 매핑
    char_style_index: HashMap<u64, CharStyleId>,

    /// 문단 스타일 목록
    para_styles: Vec<ParaStyle>,
    /// 문단 스타일 해시 → ID 매핑
    para_style_index: HashMap<u64, ParaStyleId>,

    /// 글꼴 목록
    fonts: Vec<FontInfo>,
    /// 글꼴 이름 → ID 매핑
    font_index: HashMap<String, FontId>,
}

#[derive(Debug, Clone)]
pub struct FontInfo {
    /// 글꼴 이름
    pub name: String,
    /// 글꼴 유형
    pub font_type: FontType,
    /// 대체 글꼴 ID
    pub fallback: Option<FontId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontType {
    /// 명조 계열
    Serif,
    /// 고딕 계열
    SansSerif,
    /// 고정폭
    Monospace,
    /// 필기체
    Cursive,
    /// 장식체
    Fantasy,
}

impl StyleStore {
    pub fn new() -> Self {
        let mut store = Self {
            char_styles: Vec::new(),
            char_style_index: HashMap::new(),
            para_styles: Vec::new(),
            para_style_index: HashMap::new(),
            fonts: Vec::new(),
            font_index: HashMap::new(),
        };

        // 기본 스타일 등록
        store.intern_char_style(CharStyle::default_style());
        store.intern_para_style(ParaStyle::default_style());

        // 기본 글꼴 등록
        store.intern_font(FontInfo {
            name: "함초롬돋움".to_string(),
            font_type: FontType::SansSerif,
            fallback: None,
        });

        store
    }

    // === 문자 스타일 ===

    /// 문자 스타일 등록 (동일 스타일이면 기존 ID 반환)
    pub fn intern_char_style(&mut self, style: CharStyle) -> CharStyleId {
        let hash = self.hash_char_style(&style);

        if let Some(&id) = self.char_style_index.get(&hash) {
            // 해시 충돌 확인
            if self.char_styles[id.0 as usize] == style {
                return id;
            }
        }

        let id = CharStyleId(self.char_styles.len() as u32);
        self.char_styles.push(style);
        self.char_style_index.insert(hash, id);
        id
    }

    /// ID로 문자 스타일 조회
    pub fn get_char_style(&self, id: CharStyleId) -> Option<&CharStyle> {
        self.char_styles.get(id.0 as usize)
    }

    fn hash_char_style(&self, style: &CharStyle) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();

        style.font_family.0.hash(&mut hasher);
        style.font_size.hash(&mut hasher);
        (style.font_weight as u8).hash(&mut hasher);
        (style.font_style as u8).hash(&mut hasher);
        style.color.r.hash(&mut hasher);
        style.color.g.hash(&mut hasher);
        style.color.b.hash(&mut hasher);
        style.color.a.hash(&mut hasher);
        // ... 필요한 필드 모두 해시

        hasher.finish()
    }

    // === 문단 스타일 ===

    /// 문단 스타일 등록
    pub fn intern_para_style(&mut self, style: ParaStyle) -> ParaStyleId {
        let hash = self.hash_para_style(&style);

        if let Some(&id) = self.para_style_index.get(&hash) {
            if self.para_styles[id.0 as usize] == style {
                return id;
            }
        }

        let id = ParaStyleId(self.para_styles.len() as u32);
        self.para_styles.push(style);
        self.para_style_index.insert(hash, id);
        id
    }

    /// ID로 문단 스타일 조회
    pub fn get_para_style(&self, id: ParaStyleId) -> Option<&ParaStyle> {
        self.para_styles.get(id.0 as usize)
    }

    fn hash_para_style(&self, style: &ParaStyle) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();

        (style.alignment as u8).hash(&mut hasher);
        style.indent_left.hash(&mut hasher);
        style.indent_right.hash(&mut hasher);
        style.indent_first.hash(&mut hasher);
        // ... 필요한 필드 모두 해시

        hasher.finish()
    }

    // === 글꼴 ===

    /// 글꼴 등록
    pub fn intern_font(&mut self, font: FontInfo) -> FontId {
        if let Some(&id) = self.font_index.get(&font.name) {
            return id;
        }

        let id = FontId(self.fonts.len() as u32);
        self.font_index.insert(font.name.clone(), id);
        self.fonts.push(font);
        id
    }

    /// ID로 글꼴 조회
    pub fn get_font(&self, id: FontId) -> Option<&FontInfo> {
        self.fonts.get(id.0 as usize)
    }

    /// 이름으로 글꼴 조회
    pub fn get_font_by_name(&self, name: &str) -> Option<FontId> {
        self.font_index.get(name).copied()
    }

    // === 복사/붙여넣기용 ===

    /// 다른 StyleStore에서 스타일 병합
    ///
    /// 반환: (old_char_id → new_char_id, old_para_id → new_para_id)
    pub fn merge(&mut self, other: &StyleStore) -> (
        HashMap<CharStyleId, CharStyleId>,
        HashMap<ParaStyleId, ParaStyleId>,
    ) {
        let mut char_map = HashMap::new();
        let mut para_map = HashMap::new();

        // 글꼴 먼저 병합
        let mut font_map = HashMap::new();
        for (i, font) in other.fonts.iter().enumerate() {
            let old_id = FontId(i as u32);
            let new_id = self.intern_font(font.clone());
            font_map.insert(old_id, new_id);
        }

        // 문자 스타일 병합 (글꼴 ID 재매핑)
        for (i, style) in other.char_styles.iter().enumerate() {
            let old_id = CharStyleId(i as u32);
            let mut new_style = style.clone();
            if let Some(&new_font) = font_map.get(&style.font_family) {
                new_style.font_family = new_font;
            }
            let new_id = self.intern_char_style(new_style);
            char_map.insert(old_id, new_id);
        }

        // 문단 스타일 병합
        for (i, style) in other.para_styles.iter().enumerate() {
            let old_id = ParaStyleId(i as u32);
            let new_id = self.intern_para_style(style.clone());
            para_map.insert(old_id, new_id);
        }

        (char_map, para_map)
    }
}

impl Default for StyleStore {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## 5. 선택 영역

### 5.1 Position

문서 내 위치를 나타냅니다.

```rust
// src/selection/position.rs

/// 문서 내 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// 블록 인덱스
    pub block_index: usize,
    /// 블록 내 오프셋 (바이트)
    pub offset: usize,
}

impl Position {
    /// 문서 시작 위치
    pub fn start() -> Self {
        Self {
            block_index: 0,
            offset: 0,
        }
    }

    /// 새 위치 생성
    pub fn new(block_index: usize, offset: usize) -> Self {
        Self { block_index, offset }
    }

    /// 문단 내 위치인지 확인
    pub fn is_in_paragraph(&self, doc: &Document) -> bool {
        doc.block(self.block_index)
            .map(|b| matches!(b, Block::Paragraph(_)))
            .unwrap_or(false)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.block_index.cmp(&other.block_index) {
            std::cmp::Ordering::Equal => self.offset.cmp(&other.offset),
            ord => ord,
        }
    }
}
```

### 5.2 Selection

선택 영역을 나타냅니다.

```rust
// src/selection/selection.rs

/// 선택 영역
///
/// anchor는 선택 시작점 (고정), focus는 현재 커서 위치 (움직임)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// 선택 시작점 (고정)
    pub anchor: Position,
    /// 현재 커서 위치 (움직임)
    pub focus: Position,
}

impl Selection {
    /// 커서 위치로 선택 생성 (선택 없음)
    pub fn cursor(pos: Position) -> Self {
        Self {
            anchor: pos,
            focus: pos,
        }
    }

    /// 범위 선택 생성
    pub fn range(anchor: Position, focus: Position) -> Self {
        Self { anchor, focus }
    }

    /// 선택 없음 (커서만) 여부
    pub fn is_collapsed(&self) -> bool {
        self.anchor == self.focus
    }

    /// 정방향 선택 여부 (anchor <= focus)
    pub fn is_forward(&self) -> bool {
        self.anchor <= self.focus
    }

    /// 정규화된 범위 반환 (start <= end)
    pub fn ordered(&self) -> (Position, Position) {
        if self.is_forward() {
            (self.anchor, self.focus)
        } else {
            (self.focus, self.anchor)
        }
    }

    /// 시작 위치
    pub fn start(&self) -> Position {
        self.ordered().0
    }

    /// 끝 위치
    pub fn end(&self) -> Position {
        self.ordered().1
    }

    /// 커서 위치 (focus)
    pub fn cursor_position(&self) -> Position {
        self.focus
    }

    /// 커서 이동 (선택 해제)
    pub fn move_to(&mut self, pos: Position) {
        self.anchor = pos;
        self.focus = pos;
    }

    /// 선택 확장 (anchor 유지, focus 이동)
    pub fn extend_to(&mut self, pos: Position) {
        self.focus = pos;
    }

    /// 선택 영역이 특정 블록을 포함하는지
    pub fn contains_block(&self, block_index: usize) -> bool {
        let (start, end) = self.ordered();
        block_index >= start.block_index && block_index <= end.block_index
    }

    /// 선택 영역이 특정 위치를 포함하는지
    pub fn contains(&self, pos: Position) -> bool {
        let (start, end) = self.ordered();
        pos >= start && pos <= end
    }
}

/// 다중 선택 (Ctrl+클릭)
#[derive(Debug, Clone)]
pub struct SelectionSet {
    /// 선택 목록
    pub selections: Vec<Selection>,
    /// 주 선택 인덱스
    pub primary: usize,
}

impl SelectionSet {
    /// 단일 선택으로 생성
    pub fn single(selection: Selection) -> Self {
        Self {
            selections: vec![selection],
            primary: 0,
        }
    }

    /// 주 선택 반환
    pub fn primary(&self) -> &Selection {
        &self.selections[self.primary]
    }

    pub fn primary_mut(&mut self) -> &mut Selection {
        &mut self.selections[self.primary]
    }

    /// 선택 추가
    pub fn add(&mut self, selection: Selection) {
        self.selections.push(selection);
        self.primary = self.selections.len() - 1;
    }

    /// 모든 선택 순회
    pub fn iter(&self) -> impl Iterator<Item = &Selection> {
        self.selections.iter()
    }

    /// 선택 개수
    pub fn len(&self) -> usize {
        self.selections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.selections.is_empty()
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::cursor(Position::start())
    }
}

impl Default for SelectionSet {
    fn default() -> Self {
        Self::single(Selection::default())
    }
}
```

---

## 6. 편집 명령 (Command 패턴)

### 6.1 Command trait

```rust
// src/edit/command.rs

use std::any::Any;
use std::ops::Range;

/// 편집 명령 trait
pub trait Command: std::fmt::Debug + Send + Sync {
    /// 명령 실행
    ///
    /// 반환: 영향받은 블록 범위
    fn execute(&self, doc: &mut Document) -> Range<usize>;

    /// 명령 취소
    fn undo(&self, doc: &mut Document);

    /// 다른 명령과 병합 시도
    ///
    /// 연속 입력 등을 하나의 명령으로 병합
    fn merge(&self, other: &dyn Command) -> Option<Box<dyn Command>> {
        None
    }

    /// 명령 설명 (디버깅용)
    fn description(&self) -> &str;

    /// 다운캐스트용
    fn as_any(&self) -> &dyn Any;
}

/// 명령 실행 결과
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// 영향받은 블록 범위
    pub affected_range: Range<usize>,
    /// 새 커서 위치
    pub new_cursor: Option<Position>,
}
```

### 6.2 InsertTextCommand

```rust
// src/edit/commands/insert_text.rs

/// 텍스트 삽입 명령
#[derive(Debug, Clone)]
pub struct InsertTextCommand {
    /// 삽입 위치
    position: Position,
    /// 삽입할 텍스트
    text: String,
    /// 적용할 스타일 (None이면 위치의 기존 스타일 사용)
    style: Option<CharStyleId>,
}

impl InsertTextCommand {
    pub fn new(position: Position, text: impl Into<String>) -> Self {
        Self {
            position,
            text: text.into(),
            style: None,
        }
    }

    pub fn with_style(mut self, style: CharStyleId) -> Self {
        self.style = Some(style);
        self
    }
}

impl Command for InsertTextCommand {
    fn execute(&self, doc: &mut Document) -> Range<usize> {
        let block = doc.block_mut(self.position.block_index)
            .expect("Invalid block index");

        if let Some(para) = block.as_paragraph_mut() {
            // 1. 텍스트 삽입
            para.insert_text(self.position.offset, &self.text);

            // 2. 스타일 적용
            if let Some(style) = self.style {
                let end = self.position.offset + self.text.len();
                para.apply_char_style(self.position.offset, end, style);
            }
        }

        self.position.block_index..self.position.block_index + 1
    }

    fn undo(&self, doc: &mut Document) {
        let block = doc.block_mut(self.position.block_index)
            .expect("Invalid block index");

        if let Some(para) = block.as_paragraph_mut() {
            let end = self.position.offset + self.text.len();
            para.delete_text(self.position.offset, end);
        }
    }

    fn merge(&self, other: &dyn Command) -> Option<Box<dyn Command>> {
        // 연속 입력 병합
        let other = other.as_any().downcast_ref::<InsertTextCommand>()?;

        // 같은 블록이고 연속된 위치인 경우만 병합
        if self.position.block_index != other.position.block_index {
            return None;
        }

        let expected_pos = self.position.offset + self.text.len();
        if other.position.offset != expected_pos {
            return None;
        }

        // 스타일이 같아야 병합
        if self.style != other.style {
            return None;
        }

        Some(Box::new(InsertTextCommand {
            position: self.position,
            text: format!("{}{}", self.text, other.text),
            style: self.style,
        }))
    }

    fn description(&self) -> &str {
        "Insert text"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
```

### 6.3 DeleteCommand

```rust
// src/edit/commands/delete.rs

/// 삭제 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteDirection {
    /// 뒤로 삭제 (Backspace)
    Backward,
    /// 앞으로 삭제 (Delete)
    Forward,
}

/// 단일 문자/단위 삭제 명령
#[derive(Debug, Clone)]
pub struct DeleteCommand {
    /// 삭제 위치
    position: Position,
    /// 삭제 방향
    direction: DeleteDirection,
    /// 삭제 단위
    unit: DeleteUnit,
    /// 삭제된 내용 (undo용)
    deleted: Option<DeletedContent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteUnit {
    /// 문자 단위
    Character,
    /// 단어 단위
    Word,
}

#[derive(Debug, Clone)]
struct DeletedContent {
    text: String,
    style: CharStyleId,
}

impl DeleteCommand {
    pub fn backward(position: Position) -> Self {
        Self {
            position,
            direction: DeleteDirection::Backward,
            unit: DeleteUnit::Character,
            deleted: None,
        }
    }

    pub fn forward(position: Position) -> Self {
        Self {
            position,
            direction: DeleteDirection::Forward,
            unit: DeleteUnit::Character,
            deleted: None,
        }
    }

    pub fn backward_word(position: Position) -> Self {
        Self {
            position,
            direction: DeleteDirection::Backward,
            unit: DeleteUnit::Word,
            deleted: None,
        }
    }

    pub fn forward_word(position: Position) -> Self {
        Self {
            position,
            direction: DeleteDirection::Forward,
            unit: DeleteUnit::Word,
            deleted: None,
        }
    }
}

impl Command for DeleteCommand {
    fn execute(&self, doc: &mut Document) -> Range<usize> {
        let block = doc.block_mut(self.position.block_index)
            .expect("Invalid block index");

        if let Some(para) = block.as_paragraph_mut() {
            let text = para.text();

            let (start, end) = match (self.direction, self.unit) {
                (DeleteDirection::Backward, DeleteUnit::Character) => {
                    if self.position.offset == 0 {
                        // TODO: 이전 문단과 병합
                        return self.position.block_index..self.position.block_index;
                    }
                    let start = prev_grapheme_boundary(text, self.position.offset);
                    (start, self.position.offset)
                }
                (DeleteDirection::Forward, DeleteUnit::Character) => {
                    if self.position.offset >= text.len() {
                        // TODO: 다음 문단과 병합
                        return self.position.block_index..self.position.block_index;
                    }
                    let end = next_grapheme_boundary(text, self.position.offset);
                    (self.position.offset, end)
                }
                (DeleteDirection::Backward, DeleteUnit::Word) => {
                    let start = prev_word_boundary(text, self.position.offset);
                    (start, self.position.offset)
                }
                (DeleteDirection::Forward, DeleteUnit::Word) => {
                    let end = next_word_boundary(text, self.position.offset);
                    (self.position.offset, end)
                }
            };

            if start < end {
                para.delete_text(start, end);
            }
        }

        self.position.block_index..self.position.block_index + 1
    }

    fn undo(&self, doc: &mut Document) {
        // TODO: deleted 내용 복원
    }

    fn description(&self) -> &str {
        match (self.direction, self.unit) {
            (DeleteDirection::Backward, DeleteUnit::Character) => "Backspace",
            (DeleteDirection::Forward, DeleteUnit::Character) => "Delete",
            (DeleteDirection::Backward, DeleteUnit::Word) => "Delete word backward",
            (DeleteDirection::Forward, DeleteUnit::Word) => "Delete word forward",
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// === 유틸리티 함수 ===

/// 이전 grapheme cluster 경계 찾기
fn prev_grapheme_boundary(text: &str, offset: usize) -> usize {
    use unicode_segmentation::UnicodeSegmentation;

    let mut last_boundary = 0;
    for (i, _) in text.grapheme_indices(true) {
        if i >= offset {
            break;
        }
        last_boundary = i;
    }
    last_boundary
}

/// 다음 grapheme cluster 경계 찾기
fn next_grapheme_boundary(text: &str, offset: usize) -> usize {
    use unicode_segmentation::UnicodeSegmentation;

    for (i, g) in text.grapheme_indices(true) {
        if i > offset {
            return i;
        }
        if i == offset {
            return i + g.len();
        }
    }
    text.len()
}

/// 이전 단어 경계 찾기
fn prev_word_boundary(text: &str, offset: usize) -> usize {
    use unicode_segmentation::UnicodeSegmentation;

    let mut last_boundary = 0;
    for (i, _) in text.split_word_bound_indices() {
        if i >= offset {
            break;
        }
        last_boundary = i;
    }
    last_boundary
}

/// 다음 단어 경계 찾기
fn next_word_boundary(text: &str, offset: usize) -> usize {
    use unicode_segmentation::UnicodeSegmentation;

    for (i, _) in text.split_word_bound_indices() {
        if i > offset {
            return i;
        }
    }
    text.len()
}
```

### 6.4 CommandHistory

```rust
// src/edit/history.rs

/// 명령 히스토리 (Undo/Redo)
#[derive(Debug)]
pub struct CommandHistory {
    /// Undo 스택
    undo_stack: Vec<Box<dyn Command>>,
    /// Redo 스택
    redo_stack: Vec<Box<dyn Command>>,
    /// 최대 히스토리 크기
    max_size: usize,
    /// 저장 이후 변경 여부 추적용 인덱스
    saved_index: Option<usize>,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: 1000,
            saved_index: Some(0),
        }
    }

    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            max_size,
            ..Self::new()
        }
    }

    /// 명령 추가
    pub fn push(&mut self, command: Box<dyn Command>) {
        // 병합 시도
        if let Some(last) = self.undo_stack.last() {
            if let Some(merged) = last.merge(&*command) {
                self.undo_stack.pop();
                self.undo_stack.push(merged);
                self.redo_stack.clear();
                return;
            }
        }

        self.undo_stack.push(command);
        self.redo_stack.clear();

        // 크기 제한
        while self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
            if let Some(ref mut idx) = self.saved_index {
                if *idx > 0 {
                    *idx -= 1;
                } else {
                    self.saved_index = None;
                }
            }
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

    /// Undo 실행
    pub fn undo(&mut self, doc: &mut Document) -> bool {
        if let Some(command) = self.undo_stack.pop() {
            command.undo(doc);
            self.redo_stack.push(command);
            true
        } else {
            false
        }
    }

    /// Redo 실행
    pub fn redo(&mut self, doc: &mut Document) -> bool {
        if let Some(command) = self.redo_stack.pop() {
            command.execute(doc);
            self.undo_stack.push(command);
            true
        } else {
            false
        }
    }

    /// 저장됨으로 표시
    pub fn mark_saved(&mut self) {
        self.saved_index = Some(self.undo_stack.len());
    }

    /// 저장 이후 변경되었는지
    pub fn is_modified(&self) -> bool {
        self.saved_index != Some(self.undo_stack.len())
    }

    /// 히스토리 초기화
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.saved_index = Some(0);
    }
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## 7. 클립보드

```rust
// src/edit/clipboard.rs

use std::collections::HashMap;

/// 내부 클립보드 데이터
#[derive(Debug, Clone)]
pub struct ClipboardData {
    /// 복사된 블록들
    pub blocks: Vec<Block>,
    /// 사용된 스타일 정의
    pub styles: StyleStore,
    /// 사용된 바이너리 데이터
    pub binaries: HashMap<BinaryId, Vec<u8>>,
}

impl ClipboardData {
    /// 선택 영역에서 클립보드 데이터 생성
    pub fn from_selection(doc: &Document, selection: &Selection) -> Self {
        let (start, end) = selection.ordered();
        let mut blocks = Vec::new();
        let mut styles = StyleStore::new();
        let mut binaries = HashMap::new();

        for block_idx in start.block_index..=end.block_index {
            if let Some(block) = doc.block(block_idx) {
                let block_copy = if block_idx == start.block_index && block_idx == end.block_index {
                    // 단일 블록 내 부분 선택
                    Self::copy_block_range(doc, block, start.offset, end.offset)
                } else if block_idx == start.block_index {
                    // 첫 블록 (시작부터 끝까지)
                    Self::copy_block_range(doc, block, start.offset, usize::MAX)
                } else if block_idx == end.block_index {
                    // 마지막 블록 (처음부터 끝 위치까지)
                    Self::copy_block_range(doc, block, 0, end.offset)
                } else {
                    // 중간 블록 (전체)
                    block.clone()
                };

                // 스타일 수집
                Self::collect_styles(&block_copy, doc, &mut styles);

                // 바이너리 수집
                Self::collect_binaries(&block_copy, doc, &mut binaries);

                blocks.push(block_copy);
            }
        }

        Self { blocks, styles, binaries }
    }

    fn copy_block_range(doc: &Document, block: &Block, start: usize, end: usize) -> Block {
        match block {
            Block::Paragraph(para) => {
                let actual_end = end.min(para.text_len());
                let text = &para.text()[start..actual_end];

                let mut new_para = Paragraph::with_text(para.id, text);
                // TODO: 스타일 복사
                Block::Paragraph(new_para)
            }
            _ => block.clone(),
        }
    }

    fn collect_styles(block: &Block, doc: &Document, styles: &mut StyleStore) {
        // TODO: 블록에서 사용된 스타일 수집
    }

    fn collect_binaries(block: &Block, doc: &Document, binaries: &mut HashMap<BinaryId, Vec<u8>>) {
        // TODO: 블록에서 참조된 바이너리 수집
    }

    /// HTML로 변환 (외부 앱 붙여넣기용)
    pub fn to_html(&self) -> String {
        let mut html = String::from("<html><body>");

        for block in &self.blocks {
            match block {
                Block::Paragraph(para) => {
                    html.push_str("<p>");
                    html.push_str(&html_escape(para.text()));
                    html.push_str("</p>");
                }
                // TODO: 다른 블록 타입
                _ => {}
            }
        }

        html.push_str("</body></html>");
        html
    }

    /// 평문 텍스트로 변환
    pub fn to_plain_text(&self) -> String {
        let mut text = String::new();

        for (i, block) in self.blocks.iter().enumerate() {
            if i > 0 {
                text.push('\n');
            }
            match block {
                Block::Paragraph(para) => text.push_str(para.text()),
                Block::Table(table) => text.push_str(&table.plain_text()),
                _ => {}
            }
        }

        text
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
```

---

## 8. IR 변환

### 8.1 IR → Document

```rust
// src/convert/from_ir.rs

use crate::*;

/// IR → Document 변환 에러
#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),
}

/// IR → Document 변환
pub fn convert(ir: ir::Document) -> Result<Document, ConvertError> {
    let mut doc = Document::new();
    doc.blocks.clear();
    doc.block_index.clear();

    // 스타일 변환
    convert_styles(&ir, &mut doc)?;

    // 바이너리 변환
    convert_binaries(&ir, &mut doc)?;

    // 섹션/블록 변환
    for section in &ir.sections {
        for block in &section.blocks {
            let converted = convert_block(block, &ir, &doc)?;
            let id = doc.insert_block(doc.block_count(), converted);
        }
    }

    // 빈 문서면 기본 문단 추가
    if doc.block_count() == 0 {
        let para = Paragraph::new(BlockId(0));
        doc.blocks.push(Block::Paragraph(para));
        doc.block_index.insert(BlockId(0), 0);
    }

    Ok(doc)
}

fn convert_styles(ir: &ir::Document, doc: &mut Document) -> Result<(), ConvertError> {
    // 글꼴 변환
    for font in &ir.fonts {
        doc.styles_mut().intern_font(FontInfo {
            name: font.name.clone(),
            font_type: convert_font_type(&font.font_type),
            fallback: None,
        });
    }

    // 문자 스타일 변환
    for char_shape in &ir.char_shapes {
        let style = convert_char_style(char_shape)?;
        doc.styles_mut().intern_char_style(style);
    }

    // 문단 스타일 변환
    for para_shape in &ir.para_shapes {
        let style = convert_para_style(para_shape)?;
        doc.styles_mut().intern_para_style(style);
    }

    Ok(())
}

fn convert_char_style(ir: &ir::CharShape) -> Result<CharStyle, ConvertError> {
    Ok(CharStyle {
        font_family: FontId(ir.font_id as u32),
        font_size: ir.font_size,
        font_weight: if ir.bold { FontWeight::Bold } else { FontWeight::Normal },
        font_style: if ir.italic { FontStyle::Italic } else { FontStyle::Normal },
        color: convert_color(&ir.color),
        background: ir.background.as_ref().map(convert_color),
        underline: ir.underline.as_ref().map(|u| UnderlineStyle {
            style: convert_line_style(&u.style),
            color: convert_color(&u.color),
        }),
        strikethrough: ir.strikethrough.as_ref().map(|s| StrikethroughStyle {
            style: convert_line_style(&s.style),
            color: convert_color(&s.color),
        }),
        superscript: ir.superscript,
        subscript: ir.subscript,
        letter_spacing: ir.letter_spacing,
        width_ratio: ir.width_ratio,
        baseline_offset: ir.baseline_offset,
        shadow: None,  // TODO
        outline: None, // TODO
        emboss: None,  // TODO
    })
}

fn convert_para_style(ir: &ir::ParaShape) -> Result<ParaStyle, ConvertError> {
    Ok(ParaStyle {
        alignment: convert_alignment(&ir.alignment),
        vertical_alignment: VerticalAlignment::Top,
        indent_left: ir.indent_left,
        indent_right: ir.indent_right,
        indent_first: ir.indent_first,
        space_before: ir.space_before,
        space_after: ir.space_after,
        line_spacing: convert_line_spacing(&ir.line_spacing),
        line_break: LineBreakConfig::default(),  // TODO
        tab_stops: ir.tab_stops.iter().map(convert_tab_stop).collect(),
        border: None,     // TODO
        background: None, // TODO
        keep_lines_together: ir.keep_lines_together,
        keep_with_next: ir.keep_with_next,
    })
}

fn convert_block(ir_block: &ir::Block, ir: &ir::Document, doc: &Document) -> Result<Block, ConvertError> {
    match ir_block {
        ir::Block::Paragraph(para) => {
            let mut new_para = Paragraph::new(BlockId(0));  // ID는 나중에 할당

            // 텍스트 변환
            for run in &para.runs {
                let start = new_para.text_len();
                new_para.insert_text(start, &run.text);

                // 스타일 적용
                let style_id = CharStyleId(run.char_style_id as u32);
                new_para.apply_char_style(start, new_para.text_len(), style_id);
            }

            // 문단 스타일
            new_para.set_para_style(ParaStyleId(para.para_style_id as u32));

            Ok(Block::Paragraph(new_para))
        }
        ir::Block::Table(table) => {
            // TODO: 표 변환
            Err(ConvertError::UnsupportedFeature("Table".to_string()))
        }
        ir::Block::Image(image) => {
            // TODO: 이미지 변환
            Err(ConvertError::UnsupportedFeature("Image".to_string()))
        }
        _ => Err(ConvertError::UnsupportedFeature("Unknown block type".to_string())),
    }
}

fn convert_binaries(ir: &ir::Document, doc: &mut Document) -> Result<(), ConvertError> {
    for (id, data) in &ir.binaries {
        doc.binaries_mut().insert(*id, data.clone());
    }
    Ok(())
}

// === 유틸리티 변환 함수 ===

fn convert_color(ir: &ir::Color) -> Color {
    Color::rgba(ir.r, ir.g, ir.b, ir.a.unwrap_or(255))
}

fn convert_font_type(ir: &ir::FontType) -> FontType {
    match ir {
        ir::FontType::Serif => FontType::Serif,
        ir::FontType::SansSerif => FontType::SansSerif,
        ir::FontType::Monospace => FontType::Monospace,
        ir::FontType::Cursive => FontType::Cursive,
        ir::FontType::Fantasy => FontType::Fantasy,
    }
}

fn convert_alignment(ir: &ir::Alignment) -> Alignment {
    match ir {
        ir::Alignment::Left => Alignment::Left,
        ir::Alignment::Center => Alignment::Center,
        ir::Alignment::Right => Alignment::Right,
        ir::Alignment::Justify => Alignment::Justify,
        ir::Alignment::Distribute => Alignment::Distribute,
    }
}

fn convert_line_style(ir: &ir::LineStyle) -> LineStyle {
    match ir {
        ir::LineStyle::Solid => LineStyle::Solid,
        ir::LineStyle::Dashed => LineStyle::Dashed,
        ir::LineStyle::Dotted => LineStyle::Dotted,
        ir::LineStyle::Double => LineStyle::Double,
        ir::LineStyle::Wavy => LineStyle::Wavy,
    }
}

fn convert_line_spacing(ir: &ir::LineSpacing) -> LineSpacing {
    match ir {
        ir::LineSpacing::Multiple(v) => LineSpacing::Multiple(*v),
        ir::LineSpacing::Fixed(v) => LineSpacing::Fixed(*v),
        ir::LineSpacing::AtLeast(v) => LineSpacing::AtLeast(*v),
    }
}

fn convert_tab_stop(ir: &ir::TabStop) -> TabStop {
    TabStop {
        position: ir.position,
        alignment: match ir.alignment {
            ir::TabAlignment::Left => TabAlignment::Left,
            ir::TabAlignment::Center => TabAlignment::Center,
            ir::TabAlignment::Right => TabAlignment::Right,
            ir::TabAlignment::Decimal => TabAlignment::Decimal,
        },
        leader: ir.leader,
    }
}
```

---

## 9. 테스트 가이드

### 9.1 단위 테스트 예시

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paragraph_insert_text() {
        let mut para = Paragraph::new(BlockId(0));
        para.insert_text(0, "Hello");

        assert_eq!(para.text(), "Hello");
        assert_eq!(para.text_len(), 5);
    }

    #[test]
    fn test_paragraph_delete_text() {
        let mut para = Paragraph::with_text(BlockId(0), "Hello World");
        para.delete_text(5, 11);  // " World" 삭제

        assert_eq!(para.text(), "Hello");
    }

    #[test]
    fn test_style_runs() {
        let mut runs = StyleRuns::new();
        let bold = CharStyleId(1);

        // "Hello World" 중 "World"에 굵게 적용
        runs.apply(6, 11, bold);

        assert_eq!(runs.style_at(0), CharStyleId::default());
        assert_eq!(runs.style_at(5), CharStyleId::default());
        assert_eq!(runs.style_at(6), bold);
        assert_eq!(runs.style_at(10), bold);
    }

    #[test]
    fn test_selection_ordered() {
        let sel = Selection::range(
            Position::new(1, 10),
            Position::new(0, 5),
        );

        let (start, end) = sel.ordered();
        assert_eq!(start, Position::new(0, 5));
        assert_eq!(end, Position::new(1, 10));
    }

    #[test]
    fn test_command_history_undo_redo() {
        let mut doc = Document::new();
        let mut history = CommandHistory::new();

        // 텍스트 삽입
        let cmd = InsertTextCommand::new(Position::start(), "Hello");
        cmd.execute(&mut doc);
        history.push(Box::new(cmd));

        assert_eq!(doc.plain_text(), "Hello");

        // Undo
        history.undo(&mut doc);
        assert_eq!(doc.plain_text(), "");

        // Redo
        history.redo(&mut doc);
        assert_eq!(doc.plain_text(), "Hello");
    }
}
```

---

## 변경 이력

| 날짜 | 버전 | 내용 |
|------|------|------|
| 2024-12 | 0.1 | 초안 작성 |
