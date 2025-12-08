# OpenHWP

HWP(한글 워드프로세서) 문서를 읽고 쓰기 위한 Rust 라이브러리입니다.

> 본 제품은 한글과컴퓨터의 한/글 문서 파일(.hwp) 공개 문서를 참고하여 개발하였습니다.

## 지원 포맷

| 포맷 | 확장자 | 설명 | 크레이트 |
|------|--------|------|----------|
| HWP 5.0 | `.hwp` | 바이너리 포맷 (OLE/CFB 컨테이너), 한글 2002 이후 사용 | `hwp` |
| HWPX | `.hwpx` | XML 기반 포맷 (KS X 6101:2024 표준) | `hwpx` |

## 설치

`Cargo.toml`에 필요한 크레이트를 추가하세요:

```toml
[dependencies]
hwp = { git = "https://github.com/openhwp/openhwp", package = "hwp" }
hwpx = { git = "https://github.com/openhwp/openhwp", package = "hwpx" }
```

## 빠른 시작

### HWP 5.0 파일 읽기

```rust
use hwp::HwpDocument;

fn main() -> hwp::Result<()> {
    let bytes = std::fs::read("document.hwp")?;
    let document = HwpDocument::from_bytes(&bytes)?;

    // 문서 속성
    println!("버전: {}", document.version());
    println!("섹션 수: {}", document.section_count());
    println!("문단 수: {}", document.paragraph_count());

    // 전체 텍스트 추출
    println!("{}", document.extract_text());

    Ok(())
}
```

### 암호화된 HWP 파일 읽기

```rust
use hwp::HwpDocument;

fn main() -> hwp::Result<()> {
    let bytes = std::fs::read("encrypted.hwp")?;
    let document = HwpDocument::from_bytes_with_password(&bytes, "비밀번호")?;

    println!("{}", document.extract_text());
    Ok(())
}
```

### HWPX XML 파일 파싱

```rust
use hwpx::version::HcfVersion;
use hwpx::header::Head;
use hwpx::paragraph::Section;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // version.xml 파싱
    let xml = std::fs::read_to_string("version.xml")?;
    let version: HcfVersion = hwpx::from_str(&xml)?;
    println!("버전: {}.{}", version.major, version.minor);

    // header.xml 파싱
    let xml = std::fs::read_to_string("Contents/header.xml")?;
    let head: Head = hwpx::from_str(&xml)?;
    println!("섹션 수: {}", head.section_count);

    // section XML 파싱
    let xml = std::fs::read_to_string("Contents/section0.xml")?;
    let section: Section = hwpx::from_str(&xml)?;
    println!("문단 수: {}", section.paragraph_list.paragraphs.len());

    Ok(())
}
```

### HWPX XML로 직렬화

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

## 기능

### hwp 크레이트
- HWP 5.0 형식 지원 (한글 2002 ~ 2022)
- 암호화된 문서 지원
- 배포용 문서 지원
- 텍스트 및 이미지 추출
- 문서 메타데이터 접근 (제목, 저자 등)
- 미리보기 텍스트/이미지 접근
- 스크립트 접근

### hwpx 크레이트
- KS X 6101:2024 스키마 지원
- 모든 문서 요소에 대한 강타입 모델
- Serde 기반 XML 직렬화/역직렬화
- 실제 문서의 네임스페이스 처리

## HWPX 파일-타입 매핑

| 파일 | 타입 |
|------|------|
| `version.xml` | `hwpx::version::HcfVersion` |
| `Contents/header.xml` | `hwpx::header::Head` |
| `Contents/section*.xml` | `hwpx::paragraph::Section` |
| `masterpage*.xml` | `hwpx::master_page::MasterPage` |
| `history.xml` | `hwpx::history::History` |

## 문서

- [hwp 크레이트 문서](crates/hwp/README.md)
- [hwpx 크레이트 문서](crates/hwpx/README.md)
- [HWP 5.0 규격](docs/hwp/)
- [HWPX/KS X 6101:2024 규격](docs/hwpx/)

## 크레딧

다음 프로젝트들에 감사드립니다:

- https://github.com/hahnlee/hwp.js
- https://github.com/hahnlee/hwp-rs
- https://github.com/sjunepark/hwp
- https://github.com/123jimin/node-hwp
- https://github.com/sboh1214/Hwp-Swift

## 기여자

<a href="https://github.com/openhwp/openhwp/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=openhwp/openhwp" />
</a>

## 라이선스

OpenHWP는 MIT 라이선스로 배포됩니다.

자세한 내용은 [LICENSE](LICENSE)를 참고하세요.
