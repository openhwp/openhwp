# hwp

HWP 5.0 (`.hwp`) 파일을 읽기 위한 Rust 라이브러리입니다.

## 설치

```toml
[dependencies]
hwp = { git = "https://github.com/openhwp/openhwp" }
```

## 사용 예시

### 기본 사용법

```rust
use hwp::HwpDocument;

let bytes = std::fs::read("document.hwp")?;
let doc = HwpDocument::from_bytes(&bytes)?;

// 텍스트 추출
println!("{}", doc.extract_text());

// 문서 정보
println!("버전: {}", doc.version());
println!("섹션 수: {}", doc.section_count());
```

### 암호화된 문서

```rust
let doc = HwpDocument::from_bytes_with_password(&bytes, "비밀번호")?;
```

### 메타데이터

```rust
if let Some(title) = doc.title() {
    println!("제목: {}", title);
}
if let Some(author) = doc.author() {
    println!("저자: {}", author);
}
```

### 이미지 추출

```rust
for id in doc.binary_data_ids() {
    if let Some(data) = doc.get_binary_data(id) {
        std::fs::write(format!("image_{}.bin", id), data)?;
    }
}
```

### 섹션/문단 순회

```rust
for section in doc.sections() {
    for paragraph in section.paragraphs() {
        println!("{}", paragraph.plain_text());
    }
}
```

## 주요 타입

| 타입 | 설명 |
|------|------|
| `HwpDocument` | 문서 진입점 |
| `Section` | 섹션 |
| `Paragraph` | 문단 |
| `Table` | 표 |
| `Picture` | 그림 |
| `CharacterShape` | 글자 모양 |
| `ParagraphShape` | 문단 모양 |

## 지원 기능

- HWP 5.0 형식 (한글 2002~2022)
- 암호화된 문서
- 배포용 문서
- 텍스트/이미지 추출
- 미리보기 텍스트/이미지

## 라이선스

MIT License
