# OpenHWP

HWP(한글 워드프로세서) 문서를 읽고 쓰기 위한 Rust 라이브러리입니다.

> 본 제품은 한글과컴퓨터의 한/글 문서 파일(.hwp) 공개 문서를 참고하여 개발하였습니다.

## 크레이트

| 크레이트 | 설명 | 용도 |
|----------|------|------|
| [`hwp`](crates/hwp/) | HWP 5.0 바이너리 파서 | `.hwp` 파일 읽기 |
| [`hwpx`](crates/hwpx/) | HWPX XML 파서 | `.hwpx` 파일 읽기/쓰기 |
| [`ir`](crates/ir/) | 중간 표현 | HWP ↔ HWPX 변환 |
| [`document`](crates/document/) | 에디터용 문서 모델 | 문서 편집기 개발 |
| [`primitive`](crates/primitive/) | 공유 기본 타입 | 단위, 색상, ID 등 |

## 설치

```toml
[dependencies]
hwp = { git = "https://github.com/openhwp/openhwp" }   # HWP 파일 읽기
hwpx = { git = "https://github.com/openhwp/openhwp" }  # HWPX 파일 읽기/쓰기
```

## 사용 예시

### HWP 파일 읽기

```rust
use hwp::HwpDocument;

let bytes = std::fs::read("document.hwp")?;
let doc = HwpDocument::from_bytes(&bytes)?;

println!("{}", doc.extract_text());
```

### 암호화된 HWP 파일

```rust
let doc = HwpDocument::from_bytes_with_password(&bytes, "비밀번호")?;
```

### HWPX 파일 읽기

```rust
use hwpx::header::Head;

let xml = std::fs::read_to_string("Contents/header.xml")?;
let head: Head = hwpx::from_str(&xml)?;
```

### HWP → HWPX 변환

```rust
use hwp::HwpDocument;
use ir::Document as IrDocument;

let hwp = HwpDocument::from_bytes(&bytes)?;
let ir: IrDocument = hwp.into();
// ir을 hwpx로 변환...
```

## 지원 형식

| 형식 | 확장자 | 버전 | 읽기 | 쓰기 |
|------|--------|------|:----:|:----:|
| HWP 5.0 | `.hwp` | 한글 2002~2022 | O | - |
| HWPX | `.hwpx` | KS X 6101:2024 | O | O |

## 문서

- [HWP 5.0 규격](docs/hwp/)
- [HWPX 규격 (KS X 6101:2024)](docs/hwpx/)

## 크레딧

- https://github.com/hahnlee/hwp.js
- https://github.com/hahnlee/hwp-rs
- https://github.com/sjunepark/hwp
- https://github.com/123jimin/node-hwp
- https://github.com/sboh1214/Hwp-Swift

## 라이선스

MIT License
