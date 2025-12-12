# hwpx

HWPX (`.hwpx`) 파일을 읽고 쓰기 위한 Rust 라이브러리입니다. KS X 6101:2024 표준을 따릅니다.

## 설치

```toml
[dependencies]
hwpx = { git = "https://github.com/openhwp/openhwp" }
```

## 사용 예시

### XML 파싱

```rust
use hwpx::header::Head;
use hwpx::paragraph::Section;

// header.xml 파싱
let xml = std::fs::read_to_string("Contents/header.xml")?;
let head: Head = hwpx::from_str(&xml)?;

// section0.xml 파싱
let xml = std::fs::read_to_string("Contents/section0.xml")?;
let section: Section = hwpx::from_str(&xml)?;
```

### XML 생성

```rust
use hwpx::header::Head;

let head = Head::default();
let xml = hwpx::to_string(&head)?;
```

## 파일-타입 매핑

| 파일 | 타입 |
|------|------|
| `version.xml` | `hwpx::version::HcfVersion` |
| `Contents/header.xml` | `hwpx::header::Head` |
| `Contents/section*.xml` | `hwpx::paragraph::Section` |
| `Contents/masterpage*.xml` | `hwpx::master_page::MasterPage` |
| `Contents/history.xml` | `hwpx::history::History` |

## 주요 타입

### header 모듈

| 타입 | 설명 |
|------|------|
| `Head` | 문서 헤더 |
| `Font` | 글꼴 |
| `CharacterShape` | 글자 모양 |
| `ParagraphShape` | 문단 모양 |
| `Style` | 스타일 |
| `BorderFill` | 테두리/배경 |

### paragraph 모듈

| 타입 | 설명 |
|------|------|
| `Section` | 섹션 |
| `Paragraph` | 문단 |
| `Run` | 텍스트 런 |
| `Table` | 표 |
| `Picture` | 그림 |

## 지원 기능

- KS X 6101:2024 스키마
- XML 직렬화/역직렬화
- 강타입 Rust 모델

## 라이선스

MIT License
