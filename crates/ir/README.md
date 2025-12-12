# ir

HWP와 HWPX 문서 형식 간 변환을 위한 중간 표현(Intermediate Representation)입니다.

## 개요

IR은 확장자에 독립적인 문서 모델로, 다양한 한글 문서 형식(HWP, HWPX)을 통합된 방식으로 다룰 수 있게 합니다. HWP ↔ HWPX 변환의 중간 단계로 사용됩니다.

```
HWP 문서 (.hwp)  ──→  IR Document  ──→  HWPX 문서 (.hwpx)
HWPX 문서 (.hwpx) ──→  IR Document  ──→  HWP 문서 (.hwp)
```

## 설계 원칙

- **형식 독립성**: HWP, HWPX 등 특정 형식에 의존하지 않음
- **완전한 표현**: 양쪽 형식의 모든 개념을 표현 가능
- **확장 가능**: 형식별 고유 기능을 위한 확장 필드 제공
- **단위 통합**: HwpUnit 기반 단일 단위 시스템

## 사용법

```rust
use ir::Document;

// HWP에서 IR로 변환
let hwp_doc = hwp::HwpDocument::from_bytes(&bytes)?;
let ir_doc = ir::Document::try_from(hwp_doc)?;

// IR에서 HWPX로 변환
let hwpx_doc = hwpx::Document::try_from(ir_doc)?;

// 텍스트 추출
let text = ir_doc.to_plain_text();
```

## 문서 구조

```
Document
├── metadata: Metadata           # 문서 메타데이터 (제목, 저자 등)
├── settings: DocumentSettings   # 문서 설정
├── styles: StyleStore           # 스타일 저장소
│   ├── fonts                    # 폰트 정의
│   ├── char_shapes              # 글자 모양
│   ├── para_shapes              # 문단 모양
│   ├── tab_defs                 # 탭 정의
│   ├── border_fills             # 테두리/채우기
│   ├── styles                   # 스타일
│   ├── numberings               # 번호 매기기
│   └── bullets                  # 글머리 기호
├── sections: Vec<Section>       # 본문 섹션들
│   └── Section
│       ├── page                 # 페이지 설정
│       ├── paragraphs           # 문단 목록
│       │   └── Paragraph
│       │       ├── runs         # 런 목록
│       │       │   └── Run
│       │       │       ├── char_shape_id
│       │       │       └── contents: Vec<RunContent>
│       │       └── para_shape_id
│       ├── columns              # 단 설정
│       ├── headers/footers      # 머리글/바닥글
│       └── footnote/endnote     # 각주/미주 설정
├── binary_data: BinaryDataStore # 바이너리 데이터 (이미지 등)
└── extensions: Extensions       # 형식별 확장 데이터
    ├── hwp: HwpExtensions       # HWP 고유 기능
    └── hwpx: HwpxExtensions     # HWPX 고유 기능
```

## 모듈 구성

### 핵심 모듈

| 모듈 | 설명 |
|------|------|
| `document` | 문서 최상위 구조 (`Document`) |
| `section` | 섹션, 페이지 설정, 단 설정 |
| `paragraph` | 문단, 런, 런 내용 |
| `style` | 스타일 저장소, 번호 매기기 |

### 스타일 모듈

| 모듈 | 설명 |
|------|------|
| `char_shape` | 글자 모양 (폰트, 크기, 색상 등) |
| `para_shape` | 문단 모양 (정렬, 들여쓰기, 줄 간격 등) |
| `border_fill` | 테두리와 채우기 |

### 컨트롤 모듈

| 모듈 | 설명 |
|------|------|
| `control` | 컨트롤 열거형 및 공통 속성 |
| `table` | 표, 행, 셀 |
| `picture` | 그림 |
| `shape` | 도형 (선, 사각형, 타원 등) |

### 기타 모듈

| 모듈 | 설명 |
|------|------|
| `binary` | 바이너리 데이터 (이미지, OLE 등) |
| `metadata` | 문서 메타데이터 |
| `extensions` | 형식별 확장 데이터 |
| `error` | 변환 에러 및 경고 |

## 런 내용 (RunContent)

문단의 런은 다양한 내용을 포함할 수 있습니다:

```rust
pub enum RunContent {
    Text(Text),              // 텍스트
    Tab(TabChar),            // 탭
    LineBreak,               // 줄 바꿈 (Shift+Enter)
    Hyphen,                  // 하이픈
    NonBreakingSpace,        // 줄 바꿈 없는 공백
    FixedWidthSpace,         // 고정 너비 공백
    Control(Box<Control>),   // 컨트롤 (표, 그림 등)
    FieldStart(FieldStart),  // 필드 시작
    FieldEnd(FieldEnd),      // 필드 끝
    BookmarkStart(..),       // 책갈피 시작
    BookmarkEnd(..),         // 책갈피 끝
    Compose(Compose),        // 글자 겹침
    Dutmal(Dutmal),          // 덧말 (Ruby)
}
```

## 컨트롤 (Control)

문단에 삽입되는 객체들입니다:

```rust
pub enum Control {
    Table(..),        // 표
    Picture(..),      // 그림
    Shape(..),        // 도형
    Equation(..),     // 수식
    Ole(..),          // OLE 객체
    TextBox(..),      // 텍스트 박스
    Header(..),       // 머리글
    Footer(..),       // 바닥글
    Footnote(..),     // 각주
    Endnote(..),      // 미주
    Hyperlink(..),    // 하이퍼링크
    Bookmark(..),     // 책갈피
    AutoNumber(..),   // 자동 번호
    Chart(..),        // 차트
    Video(..),        // 비디오
    FormObject(..),   // 양식 객체
    TextArt(..),      // 글맵시
    Memo(..),         // 메모
    // ...
}
```

## 스타일 시스템

### StyleStore

문서의 모든 스타일을 관리합니다:

```rust
use ir::{StyleStore, CharShape, ParaShape, Font};

let mut styles = StyleStore::new();

// 폰트 추가
let font_id = styles.add_font(Font {
    name: "맑은 고딕".to_string(),
    ..Default::default()
});

// 글자 모양 추가
let char_shape_id = styles.add_char_shape(CharShape {
    font_refs: FontSet::all(font_id),
    height: HwpUnit::from_pt(10.0),
    ..Default::default()
});

// 문단 모양 추가
let para_shape_id = styles.add_para_shape(ParaShape {
    alignment: Alignment::Justify,
    ..Default::default()
});
```

### ID 참조 시스템

스타일은 ID로 참조됩니다:

```rust
// 문단에서 스타일 참조
let paragraph = Paragraph {
    para_shape_id: Some(ParaShapeId::new(0)),
    style_id: Some(StyleId::new(0)),
    runs: vec![
        Run {
            char_shape_id: Some(CharShapeId::new(0)),
            contents: vec![RunContent::Text(Text::new("안녕하세요"))],
        }
    ],
    ..Default::default()
};
```

## 확장 데이터 (Extensions)

형식 고유 기능은 Extensions에 저장됩니다:

### HWP 확장 (HwpExtensions)

HWPX로 변환 시 손실될 수 있는 HWP 고유 기능:

- `distribute_doc_data`: 배포용 문서 데이터
- `scripts`: JavaScript 스크립트
- `layout_compatibility`: 레이아웃 호환성 설정
- `document_data`: 임의 데이터

### HWPX 확장 (HwpxExtensions)

HWP로 변환 시 손실될 수 있는 HWPX 고유 기능:

- `change_history`: 변경 이력
- `master_pages`: 마스터 페이지
- `presentation`: 프레젠테이션 설정
- `forbidden_words`: 금칙 문자

## 에러 처리

변환 시 발생하는 에러와 경고를 처리합니다:

```rust
use ir::{ConversionError, ConversionWarning, WarningCollector};

// 에러 생성
let err = ConversionError::unsupported("지원하지 않는 기능")
    .with_location("Section[0]/Paragraph[5]");

// 경고 수집
let mut warnings = WarningCollector::new();
warnings.data_loss("HWPX에서 지원하지 않는 스크립트 제거");
warnings.default_substituted("알 수 없는 폰트를 기본 폰트로 대체");

// 결과와 함께 반환
let result = warnings.into_result(converted_doc);
if result.has_warnings() {
    for w in &result.warnings {
        eprintln!("경고: {}", w);
    }
}
```

## 의존성

- `primitive`: 기본 타입 (HwpUnit, Color, ID 등)

## Re-exports

편의를 위해 `primitive` 크레이트의 주요 타입을 re-export합니다:

```rust
// primitive에서 re-export
pub use primitive::{
    Color, HwpUnit, Insets, Percent, Point, Rect, Size,
    BinaryDataId, BorderFillId, CharShapeId, FontId,
    NumberingId, ParaShapeId, StyleId, TabDefId,
    Panose, HeadingType, StyleType, TabLeader, TabType,
};
```
