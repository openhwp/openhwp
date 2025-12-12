# document

리치 텍스트 에디터를 위한 IR 기반 문서 모델입니다.

## 개요

`document` 크레이트는 HWP/HWPX 문서를 편집하기 위한 중간 표현(IR)을 에디터 친화적인 구조로 변환합니다. SlotMap 기반 Arena 패턴을 사용하여 효율적인 편집 연산과 안정적인 ID 관리를 제공합니다.

## 설계 원칙

- **IR 완전 호환**: IR의 모든 기능을 Document로 표현 가능
- **편집 최적화**: 효율적인 삽입, 삭제, 수정 연산
- **안정적인 ID**: SlotMap 기반 ID로 삭제 후에도 유효
- **계층 분리**: Document, Layout, Paint, View 레이어 분리

## 사용법

```rust
use document::Document;

// IR에서 Document 생성
let ir_doc = ir::Document::default();
let doc = Document::from(ir_doc);

// Document를 IR로 변환
let ir_doc: ir::Document = doc.into();
```

## 모듈 구조

### 핵심 타입

| 모듈 | 설명 |
|------|------|
| `document` | 문서 최상위 구조체 (`Document`) |
| `section` | 섹션 (페이지 구분 단위) |
| `paragraph` | 문단 |
| `run` | 런 (동일 서식의 텍스트 단위) |
| `run_content` | 런 내용 (텍스트, 이미지, 컨트롤 등) |
| `table` | 표, 행, 셀 |
| `control` | 컨트롤 (그림, 수식, 도형 등) |

### Arena 시스템

| 모듈 | 설명 |
|------|------|
| `arena` | SlotMap 기반 노드 저장소 (`DocumentArena`) |
| `id` | 타입별 ID 정의 (`SectionId`, `ParagraphId`, `RunId` 등) |

### 편집 시스템

| 모듈 | 설명 |
|------|------|
| `command` | 편집 명령 (Command 패턴) |
| `transaction` | 트랜잭션 (원자적 다중 명령 실행) |
| `history` | Undo/Redo 히스토리 관리 |
| `cursor` | 커서 및 선택 영역 |

### 변환 및 확장

| 모듈 | 설명 |
|------|------|
| `convert` | IR ↔ Document 변환 |
| `extensions` | 형식별 확장 (HWP, HWPX, 변경 추적) |

## 문서 구조

```
Document
├── metadata: Metadata          # 문서 메타데이터
├── settings: DocumentSettings  # 문서 설정
├── styles: StyleStore          # 스타일 정의
├── sections: Vec<SectionId>    # 섹션 목록
├── binary_data: BinaryDataStore # 바이너리 데이터
├── extensions: Extensions      # 형식별 확장
└── arena: DocumentArena        # 노드 저장소
    ├── sections
    ├── paragraphs
    ├── runs
    ├── controls
    ├── rows
    ├── cells
    └── header_footers
```

## 편집 명령

Command 패턴을 사용하여 모든 편집 작업을 실행 취소 가능하게 구현합니다.

```rust
use document::{Document, Command, InsertText, DeleteText};
use document::cursor::Position;

let mut doc = Document::new();
// ... 문서 구성 ...

// 텍스트 삽입
let mut insert = InsertText::new(Position::new(0, 0, 0, 5), "Hello");
insert.execute(&mut doc)?;

// 실행 취소
insert.undo(&mut doc)?;
```

### 제공 명령

- `InsertText`: 텍스트 삽입
- `DeleteText`: 텍스트 삭제
- `InsertParagraph`: 문단 삽입
- `DeleteParagraph`: 문단 삭제

## 트랜잭션

여러 명령을 원자적으로 실행합니다. 중간에 실패하면 전체 롤백됩니다.

```rust
use document::{Transaction, TransactionBuilder};
use document::command::{InsertText, DeleteText};
use document::cursor::Position;

let mut doc = Document::new();
// ... 문서 구성 ...

// 빌더 패턴
let transaction = TransactionBuilder::new("Replace text")
    .with_command(DeleteText::new(Position::new(0, 0, 0, 6), 5))
    .with_command(InsertText::new(Position::new(0, 0, 0, 6), "Rust"))
    .execute(&mut doc)?;

// 트랜잭션 전체 취소
transaction.undo(&mut doc)?;
```

## Undo/Redo

```rust
use document::{Document, CommandHistory};
use document::command::InsertText;
use document::cursor::Position;

let mut doc = Document::new();
let mut history = CommandHistory::new();

// 명령 실행
let cmd = InsertText::new(Position::new(0, 0, 0, 0), "Hello");
history.execute(Box::new(cmd), &mut doc)?;

// Undo
if history.can_undo() {
    history.undo(&mut doc)?;
}

// Redo
if history.can_redo() {
    history.redo(&mut doc)?;
}
```

## 커서 및 선택

```rust
use document::{Document, Cursor};
use document::cursor::Position;

let doc = Document::new();
let mut cursor = Cursor::new();

// 위치 이동
cursor.move_char(5, &doc);           // 5글자 앞으로
cursor.move_to_paragraph_end(&doc);   // 문단 끝으로
cursor.move_to_document_start();      // 문서 시작으로

// 선택
cursor.start_selection();
cursor.extend_selection(Position::new(0, 0, 1, 3));

if let Some(selection) = cursor.selection() {
    println!("선택 범위: {:?} ~ {:?}", selection.start, selection.end);
}
```

## 의존성

- `ir`: 중간 표현 크레이트
- `primitive`: 기본 타입 정의
- `slotmap`: Arena 패턴 구현
