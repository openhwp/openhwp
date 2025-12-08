# hwp

HWP 5.0 바이너리 문서 파일을 파싱하기 위한 Rust 라이브러리입니다.

> 본 제품은 한글과컴퓨터의 한/글 문서 파일(.hwp) 공개 문서를 참고하여 개발하였습니다.

## 특징

- HWP 5.0 형식 지원 (한글 2002 ~ 2022)
- OLE/CFB 컨테이너 파싱
- 암호화된 문서 지원
- 배포용 문서 지원
- 텍스트 및 이미지 추출
- 문서 메타데이터 접근 (제목, 저자 등)
- 미리보기 텍스트/이미지 접근
- 스크립트 접근

## 설치

```toml
[dependencies]
hwp = { git = "https://github.com/openhwp/openhwp", package = "hwp" }
```

## 빠른 시작

### 기본 사용법

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

### 암호화된 문서 읽기

```rust
use hwp::HwpDocument;

fn main() -> hwp::Result<()> {
    let bytes = std::fs::read("encrypted.hwp")?;
    let document = HwpDocument::from_bytes_with_password(&bytes, "비밀번호")?;

    println!("{}", document.extract_text());
    Ok(())
}
```

### 문서 메타데이터 접근

```rust
use hwp::HwpDocument;

fn main() -> hwp::Result<()> {
    let bytes = std::fs::read("document.hwp")?;
    let document = HwpDocument::from_bytes(&bytes)?;

    // 요약 정보
    if let Some(title) = document.title() {
        println!("제목: {}", title);
    }
    if let Some(author) = document.author() {
        println!("저자: {}", author);
    }

    // 문서 속성
    println!("암호화 여부: {}", document.is_encrypted());
    println!("배포용 문서: {}", document.is_distribution_document());

    Ok(())
}
```

### 이미지 추출

```rust
use hwp::HwpDocument;

fn main() -> hwp::Result<()> {
    let bytes = std::fs::read("document.hwp")?;
    let document = HwpDocument::from_bytes(&bytes)?;

    for id in document.binary_data_ids() {
        if let Some(data) = document.get_binary_data(id) {
            std::fs::write(format!("image_{}.bin", id), data)?;
        }
    }

    Ok(())
}
```

### 미리보기 접근

```rust
use hwp::HwpDocument;

fn main() -> hwp::Result<()> {
    let bytes = std::fs::read("document.hwp")?;
    let document = HwpDocument::from_bytes(&bytes)?;

    // 미리보기 텍스트
    if let Some(preview) = document.preview_text() {
        println!("미리보기: {}", preview.text());
    }

    // 미리보기 이미지 (썸네일)
    if let Some(image) = document.preview_image() {
        std::fs::write("preview.png", image.data())?;
    }

    Ok(())
}
```

### 섹션 및 문단 순회

```rust
use hwp::HwpDocument;

fn main() -> hwp::Result<()> {
    let bytes = std::fs::read("document.hwp")?;
    let document = HwpDocument::from_bytes(&bytes)?;

    for (i, section) in document.sections().iter().enumerate() {
        println!("=== 섹션 {} ===", i + 1);
        for paragraph in section.paragraphs() {
            println!("{}", paragraph.text());
        }
    }

    Ok(())
}
```

## API 개요

### 주요 타입

| 타입 | 설명 |
|------|------|
| `HwpDocument` | HWP 파일을 읽기 위한 메인 진입점 |
| `FileHeader` | 파일 헤더 정보 (버전, 암호화 여부 등) |
| `DocInfo` | 문서 정보 (폰트, 스타일, 문단 모양) |
| `Section` | 문단을 포함하는 문서 섹션 |
| `Paragraph` | 텍스트와 컨트롤을 포함하는 문단 |

### 본문 타입

| 타입 | 설명 |
|------|------|
| `Control` | 특수 개체 (표, 그림 등) |
| `Table` | 표 |
| `Picture` | 그림 |
| `Equation` | 수식 |
| `Shape` | 도형 |
| `TextBox` | 글상자 |
| `Hyperlink` | 하이퍼링크 |
| `Footnote` / `Endnote` | 각주 / 미주 |
| `Header` / `Footer` | 머리글 / 꼬리글 |

### 문서 정보 타입

| 타입 | 설명 |
|------|------|
| `CharacterShape` | 글자 모양 |
| `ParagraphShape` | 문단 모양 |
| `Style` | 스타일 |
| `FaceName` | 글꼴 이름 |
| `BorderFill` | 테두리/배경 |
| `Numbering` | 번호 매김 정의 |
| `Bullet` | 글머리표 정의 |

## HWP 5.0 파일 구조

HWP 5.0 파일은 OLE(CFB) 컨테이너 형식을 사용하며, 다음과 같은 스트림을 포함합니다:

| 스트림 | 설명 |
|--------|------|
| `FileHeader` | 문서 식별 및 속성 정보 |
| `DocInfo` | 문서 수준 정보 (폰트, 스타일, 번호 매김) |
| `BodyText/SectionN` | 본문 내용 (섹션별 스트림) |
| `BinData` | 바이너리 데이터 (이미지 등) |
| `PrvText`, `PrvImage` | 미리보기 텍스트 및 이미지 |
| `Scripts` | 폼용 JavaScript |
| `DocOptions` | 문서 옵션 (연결 문서, DRM) |

## 에러 처리

```rust
use hwp::{HwpDocument, Error};

fn main() {
    let bytes = std::fs::read("document.hwp").unwrap();

    match HwpDocument::from_bytes(&bytes) {
        Ok(doc) => println!("로드 완료: {} 문단", doc.paragraph_count()),
        Err(Error::EncryptedDocument) => {
            println!("암호화된 문서입니다. from_bytes_with_password()를 사용하세요.");
        }
        Err(Error::InvalidSignature) => {
            println!("유효한 HWP 파일이 아닙니다.");
        }
        Err(e) => println!("에러: {}", e),
    }
}
```

## 지원 버전

| 버전 | 지원 |
|------|------|
| 한글 2002 | O |
| 한글 2004 | O |
| 한글 2005 | O |
| 한글 2007 | O |
| 한글 2010 | O |
| 한글 2014 | O |
| 한글 2018 | O |
| 한글 2020 | O |
| 한글 2022 | O |

## 테스트

```bash
cargo test -p hwp
```

테스트 파일은 `fixtures/` 디렉토리에 위치합니다. 라이선스 문제로 테스트 파일은 저장소에 포함되지 않습니다.

## 라이선스

MIT License
