# document

리치 텍스트 에디터를 위한 문서 모델입니다.

IR을 에디터 친화적인 구조로 변환하여 효율적인 편집 연산을 지원합니다.

## 설치

```toml
[dependencies]
document = { git = "https://github.com/openhwp/openhwp" }
```

## 사용 예시

### IR에서 Document 생성

```rust
use document::Document;
use ir::Document as IrDocument;

let ir_doc = IrDocument::default();
let doc = Document::from(ir_doc);
```

### 텍스트 편집

```rust
use document::{Document, Command, InsertText, Position};

let mut doc = Document::new();
// ... 섹션과 문단 추가 ...

// 텍스트 삽입
let mut cmd = InsertText::new(Position::new(0, 0, 0, 0), "Hello");
cmd.execute(&mut doc)?;

// 실행 취소
cmd.undo(&mut doc)?;
```

### Undo/Redo

```rust
use document::{Document, CommandHistory, InsertText, Position};

let mut doc = Document::new();
// ... 섹션과 문단 추가 ...
let mut history = CommandHistory::new();

// 명령 실행 및 기록
let cmd = InsertText::new(Position::new(0, 0, 0, 0), "Hello");
history.execute(Box::new(cmd), &mut doc)?;

// Undo/Redo
history.undo(&mut doc)?;
history.redo(&mut doc)?;
```

### 트랜잭션

```rust
use document::{Document, TransactionBuilder, InsertText, DeleteText, Position, Command};

let mut doc = Document::new();
// ... 섹션과 문단 추가 ...

// 여러 명령을 원자적으로 실행
let mut transaction = TransactionBuilder::new("Replace text")
    .with_command(DeleteText::new(Position::new(0, 0, 0, 0), 5))
    .with_command(InsertText::new(Position::new(0, 0, 0, 0), "World"))
    .execute(&mut doc)?;

// 전체 롤백
transaction.undo(&mut doc)?;
```

## 주요 타입

| 타입 | 설명 |
|------|------|
| `Document` | 문서 루트 |
| `Section` | 섹션 |
| `Paragraph` | 문단 |
| `Run` | 텍스트 런 |
| `Table` | 표 |
| `Cursor` | 커서/선택 영역 |
| `CommandHistory` | Undo/Redo 관리 |

## 편집 명령

| 명령 | 설명 |
|------|------|
| `InsertText` | 텍스트 삽입 |
| `DeleteText` | 텍스트 삭제 |

## 설계 특징

- **SlotMap 기반 Arena**: 삭제 후에도 안정적인 ID 참조
- **Command 패턴**: 모든 편집 작업 실행 취소 가능
- **트랜잭션**: 여러 명령의 원자적 실행

## 라이선스

MIT License
