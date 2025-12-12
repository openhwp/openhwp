# primitive

한글 문서 형식(HWP, HWPX)의 기본 타입 라이브러리입니다.

## 개요

`primitive` 크레이트는 HWP, HWPX, IR 크레이트 간 공유되는 기본 타입들을 정의합니다. 문서 단위, 색상, ID 참조, 스타일 속성 등 문서 형식에 필요한 모든 primitive 타입을 제공합니다.

## 사용법

```rust
use primitive::{HwpUnit, Color, CharShapeId, Version};

// 단위 변환
let unit = HwpUnit::from_pt(12.0);  // 12포인트
println!("{}mm", unit.to_mm());

// 색상 생성
let color = Color::rgb(255, 0, 0);
let hex = color.to_hex_rgb();  // "#FF0000"

// 타입 안전한 ID
let char_shape_id = CharShapeId::new(42);

// 버전 비교
let version = Version::new(5, 0, 3, 2);
if version.is_at_least(Version::V5_0_3_0) {
    // 5.0.3.0 이상 기능 사용
}
```

## 모듈 구성

### 단위 체계 (`unit`)

| 타입 | 설명 |
|------|------|
| `HwpUnit` | HWP 기본 단위 (1/7200 인치) |
| `Size` | 2차원 크기 (너비, 높이) |
| `Point` | 2차원 좌표 (x, y) |
| `Rect` | 사각형 영역 |
| `Insets` | 네 방향 여백 (상하좌우) |
| `Percent` | 백분율 값 |

```rust
use primitive::{HwpUnit, Size, Insets};

// 단위 변환
let unit = HwpUnit::from_pt(10.0);   // 포인트 → HwpUnit
let pt = unit.to_pt();               // HwpUnit → 포인트
let mm = unit.to_mm();               // HwpUnit → 밀리미터
let inch = unit.to_inches();         // HwpUnit → 인치

// 크기
let size = Size::new(HwpUnit::from_mm(210.0), HwpUnit::from_mm(297.0));  // A4

// 여백
let margin = Insets::all(HwpUnit::from_mm(20.0));  // 모든 방향 20mm
```

### 색상 (`color`)

ARGB 색상을 표현합니다.

```rust
use primitive::Color;

// RGB 생성 (알파 = 255)
let red = Color::rgb(255, 0, 0);

// ARGB 생성
let semi_transparent = Color::argb(128, 255, 0, 0);

// Hex 문자열
let color = Color::from_hex("#FF8040").unwrap();
let hex = color.to_hex_rgb();  // "#FF8040"

// u32 변환
let value = color.to_argb_u32();  // 0xFFFF8040

// 기본 색상
let black = Color::BLACK;
let white = Color::WHITE;
let transparent = Color::TRANSPARENT;
```

### ID 시스템 (`id`)

타입 안전한 ID 참조 시스템입니다.

```rust
use primitive::{CharShapeId, ParaShapeId, StyleId, BinaryDataId};

// 숫자 ID
let char_shape = CharShapeId::new(0);
let para_shape = ParaShapeId::new(1);
let style = StyleId::new(2);

// 문자열 ID
let binary = BinaryDataId::new("image.png");
let binary_hwp = BinaryDataId::from_numeric(0x1234);  // "BIN1234"
```

**숫자 ID 타입:**
- `CharShapeId` - 글자 모양
- `ParaShapeId` - 문단 모양
- `StyleId` - 스타일
- `BorderFillId` - 테두리/채우기
- `FontId` - 폰트
- `TabDefId` - 탭 정의
- `NumberingId` - 번호 정의

**문자열 ID 타입:**
- `BinaryDataId` - 바이너리 데이터
- `FileId` - 파일
- `ImageId` - 이미지
- `MasterPageId` - 마스터 페이지

### 버전 (`version`)

HWP 문서 버전을 표현합니다. 형식: `MM.nn.PP.rr`

```rust
use primitive::Version;

let version = Version::new(5, 0, 3, 2);

// 컴포넌트 접근
println!("주 버전: {}", version.major());
println!("부 버전: {}", version.minor());
println!("빌드: {}", version.build());
println!("리비전: {}", version.revision());

// 버전 비교
if version >= Version::V5_0_3_0 {
    // 새 기능 사용
}

// 버전 상수
Version::V5_0_0_0  // 최소 지원 버전
Version::V5_0_3_0  // 일반적인 버전
Version::V5_1_0_0  // 최신 버전
```

### PANOSE (`panose`)

PANOSE 1.0 글꼴 분류 시스템입니다.

```rust
use primitive::{Panose, FamilyType, Weight};

// 바이트 배열에서 생성
let panose = Panose::from_bytes([2, 11, 8, 9, 2, 2, 2, 2, 2, 3]);

// 속성 접근
assert_eq!(panose.family_type, FamilyType::Gothic);
assert_eq!(panose.weight, Weight::Bold);

// 바이트 배열로 변환
let bytes = panose.to_bytes();
```

### 스타일 속성

#### 정렬 (`alignment`)

```rust
use primitive::{Alignment, VerticalAlignment};

let h_align = Alignment::Center;
let v_align = VerticalAlignment::Middle;
```

#### 텍스트 장식 (`text_decoration`)

```rust
use primitive::{UnderlineType, StrikethroughType, EmphasisType};

let underline = UnderlineType::Single;
let strikethrough = StrikethroughType::Single;
let emphasis = EmphasisType::Dot;
```

#### 선 스타일 (`line_style`, `line`)

```rust
use primitive::{LineStyle, BorderLineStyle, LineType, LineCap};

let line = LineStyle::Solid;
let border = BorderLineStyle::Solid;
let line_type = LineType::Solid;
let cap = LineCap::Round;
```

#### 채우기 (`fill`)

```rust
use primitive::{FillType, GradientType, PatternType};

let fill = FillType::Solid;
let gradient = GradientType::Linear;
let pattern = PatternType::Horizontal;
```

### 기타 모듈

| 모듈 | 설명 |
|------|------|
| `arrow` | 화살표 타입/크기 |
| `direction` | 텍스트 방향, 페이지 방향 |
| `field` | 필드 종류, 바이너리 데이터 타입 |
| `font` | 글꼴 계열, 언어 |
| `heading` | 문단 머리 종류 |
| `image` | 이미지 효과, 뒤집기 |
| `line_break` | 줄 나눔 규칙 |
| `numbering` | 번호 매기기, 각주/미주 |
| `page` | 페이지 설정, 여백 |
| `positioning` | 개체 위치 기준 |
| `spacing` | 여백/패딩 |
| `style` | 스타일 종류 |
| `tab` | 탭 정렬, 채움선 |
| `table` | 표 관련 |
| `wrap` | 텍스트 감싸기 |

## 피처 플래그

### `serde`

Serde 직렬화/역직렬화를 활성화합니다.

```toml
[dependencies]
primitive = { version = "0.1", features = ["serde"] }
```

```rust
use primitive::Color;
use serde_json;

let color = Color::rgb(255, 128, 64);
let json = serde_json::to_string(&color).unwrap();
```

## 단위 변환 참조

| 단위 | HwpUnit 값 |
|------|-----------|
| 1 포인트 (pt) | 100 |
| 1 인치 (in) | 7,200 |
| 1 밀리미터 (mm) | ~283.465 |
| 1 센티미터 (cm) | ~2,834.65 |
