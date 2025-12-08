# hwpx

HWPX XML 스키마(KS X 6101:2024)를 러스트 타입으로 옮기고 `serde`/`quick-xml` 기반 직렬화·역직렬화를 제공하는 크레이트입니다. 문서 헤더, 본문(문단/표/그림), 바탕쪽, 버전, 변경 이력 등의 요소를 안전하게 다룰 수 있도록 각 XSD를 대응하는 모듈로 분할했습니다.

## 특징

- KS X 6101:2024 스키마를 따라간 강타입 모델(`core`, `header`, `paragraph`, `master_page`, `history`, `version`).
- `hwpx::from_str`/`hwpx::to_string` 헬퍼로 XML 문자열을 간단히 파싱/생성.
- 한/글 샘플 문서에서 추출한 `fixtures/sample*` XML로 회귀 테스트를 수행하여 실제 문서와의 호환성 확인.
- `serde`를 활용한 네임스페이스 별칭, 선택적 속성/요소 처리 및 기본값 지정.

## 설치

워크스페이스에 이미 포함되어 있다면 다음처럼 의존성을 선언합니다.

```toml
[dependencies]
hwpx = { path = "crates/hwpx" }
```

별도 저장소에서 가져올 경우에는 git 의존성으로 추가할 수 있습니다.

```toml
[dependencies]
hwpx = { git = "https://github.com/openhwp/openhwp", package = "hwpx" }
```

## 빠른 시작

### 1) version.xml 파싱

```rust
use hwpx::version::HcfVersion;

let xml = std::fs::read_to_string("version.xml")?;
let version: HcfVersion = hwpx::from_str(&xml)?;
println!("target={:?} version={}.{} build={}", version.target_application, version.major, version.minor, version.build_number);
```

### 2) header.xml 파싱

```rust
use hwpx::header::Head;

let xml = std::fs::read_to_string("Contents/header.xml")?;
let head: Head = hwpx::from_str(&xml)?;

println!("섹션 개수: {}", head.section_count);
if let Some(doc_option) = &head.document_option {
    // 문서 옵션 활용
    println!("문서 옵션 있음: {:?}", doc_option);
}
```

### 3) section.xml 안의 문단/표 파싱

```rust
use hwpx::paragraph::Section;

let xml = std::fs::read_to_string("Contents/section0.xml")?;
let section: Section = hwpx::from_str(&xml)?;
println!("문단 수: {}", section.paragraph_list.paragraphs.len());
```

### 4) 구조체를 다시 XML로 직렬화

```rust
use hwpx::header::compatible_document::CompatibleDocument;
use hwpx::header::compatible_document::{LayoutCompatibility, TargetProgram};

let doc = CompatibleDocument {
    target_program: TargetProgram::Hwp201X,
    layout_compatibility: LayoutCompatibility { apply_font_weight_to_bold: Some(()), ..Default::default() },
};

let xml = hwpx::to_string(&doc)?;
assert!(xml.contains("compatibleDocument"));
```

## 파일 ↔ 타입 매핑

- `version.xml` → `hwpx::version::HcfVersion`
- `Contents/header.xml` → `hwpx::header::Head`
- `Contents/section*.xml` → `hwpx::paragraph::Section` (문단/표/컨트롤 등 포함)
- `masterpage*.xml` → `hwpx::master_page::MasterPage`
- `history.xml` → `hwpx::history::History`
- 공통 속성·열거형 → `hwpx::core::{attributes,enums,types}`

## 테스트

샘플 문서에서 추출한 XML들을 대상으로 파싱 회귀 테스트가 준비되어 있습니다.

```bash
cargo test -p hwpx
```

테스트는 `fixtures/sample{n}` 디렉토리의 version/header/section 파일을 순회하며 모든 구조체가 올바르게 역직렬화되는지 확인합니다.

## 제약 및 참고

- `header.xml`의 `metaTag`는 네임스페이스가 섞여 있는 일부 문서에서 순서/중복 문제가 있어 현재 파싱 대상에서 제외되어 있습니다.
- 스키마 대부분을 커버하지만 0.1.0 시점의 WIP 상태입니다. 필요한 요소가 누락되었다면 PR이나 이슈를 환영합니다.
- 기본 XML 네임스페이스는 KS X 6101:2024를 따르되, 일부 실사용 문서의 오타(`trackchageConfig`)를 alias로 처리합니다.

## 함께 보면 좋은 것들

- `crates/openhwp_parser`: HWP 바이너리 포맷을 HWPX XML로 펼치는 파서
- `docs/hwpx/*`: 표준 규격 번역/정리 문서
- `crates/hwpx/tests`: 실사용 예제를 그대로 옮긴 테스트 코드 모음
