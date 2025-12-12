# ir

HWP와 HWPX 문서 형식 간 변환을 위한 중간 표현(Intermediate Representation)입니다.

```
HWP (.hwp)  →  IR  →  HWPX (.hwpx)
HWPX (.hwpx) →  IR  →  HWP (.hwp)
```

## 설치

```toml
[dependencies]
ir = { git = "https://github.com/openhwp/openhwp" }
```

## 사용 예시

### HWP → IR 변환

```rust
use hwp::HwpDocument;
use ir::Document;

let bytes = std::fs::read("document.hwp")?;
let hwp = HwpDocument::from_bytes(&bytes)?;
let ir: Document = hwp.into();
```

### 문서 생성

```rust
use ir::{Document, Section, Paragraph, Run};

let mut doc = Document::default();

let paragraph = Paragraph::with_text("안녕하세요");

let mut section = Section::default();
section.paragraphs.push(paragraph);
doc.sections.push(section);
```

## 문서 구조

```
Document
├── metadata          # 제목, 저자 등
├── settings          # 문서 설정
├── styles            # 글꼴, 글자/문단 모양, 스타일
├── sections[]        # 본문 섹션
│   ├── paragraphs[]  # 문단
│   │   └── runs[]    # 런 (동일 서식 텍스트)
│   └── page          # 페이지 설정
├── binary_data       # 이미지 등
└── extensions        # HWP/HWPX 고유 기능
```

## 주요 타입

| 타입 | 설명 |
|------|------|
| `Document` | 문서 루트 |
| `Section` | 섹션 |
| `Paragraph` | 문단 |
| `Run` | 텍스트 런 |
| `RunContent` | 런 내용 (텍스트, 탭, 컨트롤 등) |
| `StyleStore` | 스타일 저장소 |
| `CharShape` | 글자 모양 |
| `Font` | 글꼴 |

## 컨트롤 타입

| 타입 | 설명 |
|------|------|
| `Table` | 표 |
| `Picture` | 그림 |
| `Shape` | 도형 |
| `Equation` | 수식 |
| `TextBox` | 글상자 |

## 확장 데이터

형식별 고유 기능은 `Extensions`에 저장됩니다:

- **HWP 전용**: 배포용 문서, 스크립트
- **HWPX 전용**: 변경 이력, 마스터 페이지

## 라이선스

MIT License
