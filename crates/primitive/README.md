# primitive

HWP/HWPX 문서 형식의 공유 기본 타입입니다.

## 설치

```toml
[dependencies]
primitive = { git = "https://github.com/openhwp/openhwp" }

# Serde 지원이 필요한 경우
primitive = { git = "https://github.com/openhwp/openhwp", features = ["serde"] }
```

## 사용 예시

### 단위

```rust
use primitive::{HwpUnit, Size};

// 단위 변환
let unit = HwpUnit::from_pt(12.0);  // 포인트 → HwpUnit
let mm = unit.to_mm();              // HwpUnit → 밀리미터

// A4 크기
let a4 = Size::new(
    HwpUnit::from_mm(210.0),
    HwpUnit::from_mm(297.0)
);
```

### 색상

```rust
use primitive::Color;

let red = Color::rgb(255, 0, 0);
let transparent = Color::argb(128, 255, 0, 0);
let from_hex = Color::from_hex("#FF8040").unwrap();
```

### ID

```rust
use primitive::{CharShapeId, ParaShapeId, BinaryDataId};

let char_shape = CharShapeId::new(0);
let para_shape = ParaShapeId::new(1);
let binary = BinaryDataId::new("image.png");
```

### 버전

```rust
use primitive::Version;

let version = Version::new(5, 0, 3, 2);
if version >= Version::V5_0_3_0 {
    // 5.0.3.0 이상
}
```

## 주요 타입

### 단위 (`unit`)

| 타입 | 설명 |
|------|------|
| `HwpUnit` | HWP 기본 단위 (1/7200 인치) |
| `Size` | 2차원 크기 |
| `Point` | 2차원 좌표 |
| `Rect` | 사각형 영역 |
| `Insets` | 네 방향 여백 |
| `Percent` | 백분율 |

### ID (`id`)

| 타입 | 설명 |
|------|------|
| `CharShapeId` | 글자 모양 ID |
| `ParaShapeId` | 문단 모양 ID |
| `StyleId` | 스타일 ID |
| `FontId` | 글꼴 ID |
| `BorderFillId` | 테두리/채우기 ID |
| `BinaryDataId` | 바이너리 데이터 ID |

### 기타

| 타입 | 설명 |
|------|------|
| `Color` | ARGB 색상 |
| `Version` | HWP 문서 버전 |
| `Panose` | 글꼴 분류 |
| `Alignment` | 정렬 |
| `LineStyle` | 선 스타일 |
| `FillType` | 채우기 유형 |

## 단위 변환

| 단위 | HwpUnit |
|------|---------|
| 1 pt | 100 |
| 1 in | 7,200 |
| 1 mm | ~283 |
| 1 cm | ~2,835 |

## 라이선스

MIT License
