# hwpx

HWPX XML 문서를 파싱하고 생성하기 위한 Rust 라이브러리입니다. KS X 6101:2024 표준을 따릅니다.

## 특징

- KS X 6101:2024 스키마 지원
- 강타입 Rust 모델 (`core`, `header`, `paragraph`, `master_page`, `history`, `version`)
- `serde` + `quick-xml` 기반 XML 직렬화/역직렬화
- 네임스페이스 별칭 및 선택적 속성/요소 처리
- 실제 한/글 문서와의 호환성 검증

## 설치

```toml
[dependencies]
hwpx = { git = "https://github.com/openhwp/openhwp", package = "hwpx" }
```

## 빠른 시작

### version.xml 파싱

```rust
use hwpx::version::HcfVersion;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = std::fs::read_to_string("version.xml")?;
    let version: HcfVersion = hwpx::from_str(&xml)?;

    println!("버전: {}.{}", version.major, version.minor);
    println!("빌드: {}", version.build_number);

    Ok(())
}
```

### header.xml 파싱

```rust
use hwpx::header::Head;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = std::fs::read_to_string("Contents/header.xml")?;
    let head: Head = hwpx::from_str(&xml)?;

    println!("섹션 수: {}", head.section_count);

    if let Some(doc_option) = &head.document_option {
        println!("문서 옵션: {:?}", doc_option);
    }

    Ok(())
}
```

### section.xml 파싱

```rust
use hwpx::paragraph::Section;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = std::fs::read_to_string("Contents/section0.xml")?;
    let section: Section = hwpx::from_str(&xml)?;

    println!("문단 수: {}", section.paragraph_list.paragraphs.len());

    Ok(())
}
```

### XML로 직렬화

```rust
use hwpx::header::compatible_document::{CompatibleDocument, LayoutCompatibility, TargetProgram};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = CompatibleDocument {
        target_program: TargetProgram::Hwp201X,
        layout_compatibility: LayoutCompatibility::default(),
    };

    let xml = hwpx::to_string(&doc)?;
    println!("{}", xml);

    Ok(())
}
```

## API 개요

### 헬퍼 함수

| 함수 | 설명 |
|------|------|
| `hwpx::from_str(&str)` | XML 문자열을 Rust 타입으로 역직렬화 |
| `hwpx::to_string(&T)` | Rust 타입을 XML 문자열로 직렬화 |

### 모듈 구조

| 모듈 | 설명 |
|------|------|
| `hwpx::core` | 공통 속성, 열거형, 타입 |
| `hwpx::header` | 문서 헤더 (글꼴, 스타일, 문단 모양 등) |
| `hwpx::paragraph` | 본문 (문단, 표, 그림, 컨트롤 등) |
| `hwpx::master_page` | 바탕쪽 |
| `hwpx::history` | 변경 이력 |
| `hwpx::version` | 버전 정보 |

## 파일-타입 매핑

| 파일 | 타입 |
|------|------|
| `version.xml` | `hwpx::version::HcfVersion` |
| `Contents/header.xml` | `hwpx::header::Head` |
| `Contents/section*.xml` | `hwpx::paragraph::Section` |
| `masterpage*.xml` | `hwpx::master_page::MasterPage` |
| `history.xml` | `hwpx::history::History` |

### header 모듈 주요 타입

| 타입 | 설명 |
|------|------|
| `Head` | 문서 헤더 루트 |
| `Font` | 글꼴 정의 |
| `CharacterShape` | 글자 모양 |
| `ParagraphShape` | 문단 모양 |
| `Style` | 스타일 |
| `BorderFill` | 테두리/배경 |
| `Numbering` | 번호 매김 |
| `Bullet` | 글머리표 |

### paragraph 모듈 주요 타입

| 타입 | 설명 |
|------|------|
| `Section` | 섹션 루트 |
| `Paragraph` | 문단 |
| `Run` | 텍스트 런 |
| `Table` | 표 |
| `Picture` | 그림 |
| `Rectangle` | 사각형 |
| `Control` | 컨트롤 (단, 페이지 번호 등) |

## 테스트

```bash
cargo test -p hwpx
```

테스트는 `fixtures/sample*` 디렉토리의 XML 파일을 사용하여 파싱 회귀 테스트를 수행합니다.

## 제약 사항

- `header.xml`의 `metaTag`는 일부 문서에서 네임스페이스 문제로 파싱 대상에서 제외
- 스키마 대부분을 커버하지만 WIP 상태이므로 누락된 요소가 있을 수 있음
- 일부 실사용 문서의 오타(`trackchageConfig`)는 alias로 처리

## 관련 문서

- [HWPX/KS X 6101:2024 규격](../../docs/hwpx/)

## 라이선스

MIT License
