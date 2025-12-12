# layout 크레이트 상세 명세

이 문서는 `layout` 크레이트의 구현 상세를 정의합니다.

> **상위 문서**: [DESIGN.md](./DESIGN.md)

---

## 1. 개요

### 1.1 목적

`layout` 크레이트는 **문서의 시각적 배치를 계산**합니다.

- 텍스트 측정 및 줄바꿈
- 블록 배치 (문단, 표, 이미지)
- 페이지 분할
- 좌표 ↔ 문서 위치 변환 (hit test)
- dirty 플래그 기반 증분 업데이트

### 1.2 설계 원칙

1. **플랫폼 독립**: 텍스트 측정은 trait로 추상화
2. **증분 업데이트**: 변경된 부분만 재계산
3. **HWP 호환**: HWP/HWPX의 줄바꿈, 페이지 설정 지원

---

## 2. 파일 구조

```
crates/layout/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── engine.rs              # LayoutEngine
    ├── context.rs             # LayoutContext
    ├── cache.rs               # LayoutCache
    │
    ├── text/                  # 텍스트 레이아웃
    │   ├── mod.rs
    │   ├── measurer.rs        # TextMeasurer trait
    │   ├── shaper.rs          # TextShaper trait (정밀 셰이핑)
    │   ├── line_break.rs      # LineBreaker
    │   ├── line_break_config.rs
    │   └── run.rs             # TextRun, ShapedRun
    │
    ├── block/                 # 블록 레이아웃
    │   ├── mod.rs
    │   ├── paragraph.rs       # ParagraphLayout
    │   ├── table.rs           # TableLayout
    │   ├── image.rs           # ImageLayout
    │   └── shape.rs           # ShapeLayout
    │
    ├── page/                  # 페이지 레이아웃
    │   ├── mod.rs
    │   ├── paginator.rs       # Paginator
    │   ├── page.rs            # Page, PageLayout
    │   ├── header_footer.rs   # 머리글/바닥글
    │   └── page_number.rs     # 페이지 번호
    │
    ├── font/                  # 글꼴 처리
    │   ├── mod.rs
    │   ├── resolver.rs        # FontResolver (글꼴 대체)
    │   ├── metrics.rs         # FontMetrics
    │   └── fallback.rs        # 대체 글꼴 체인
    │
    ├── geometry.rs            # Point, Rect, Size
    ├── bidi.rs                # 양방향 텍스트
    └── hit_test.rs            # 좌표 → 위치 변환
```

---

## 3. 핵심 타입

### 3.1 기하학 타입

```rust
// src/geometry.rs

/// 2D 좌표
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ZERO: Point = Point { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// 크기
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const ZERO: Size = Size { width: 0.0, height: 0.0 };

    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

/// 사각형 영역
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn from_point_size(point: Point, size: Size) -> Self {
        Self {
            x: point.x,
            y: point.y,
            width: size.width,
            height: size.height,
        }
    }

    pub fn origin(&self) -> Point {
        Point::new(self.x, self.y)
    }

    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    pub fn left(&self) -> f32 { self.x }
    pub fn right(&self) -> f32 { self.x + self.width }
    pub fn top(&self) -> f32 { self.y }
    pub fn bottom(&self) -> f32 { self.y + self.height }

    pub fn center(&self) -> Point {
        Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

/// 여백 (상하좌우)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Insets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Insets {
    pub fn all(value: f32) -> Self {
        Self { top: value, right: value, bottom: value, left: value }
    }

    pub fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self { top: vertical, right: horizontal, bottom: vertical, left: horizontal }
    }

    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }
}
```

### 3.2 TextMeasurer trait

플랫폼별 텍스트 측정을 추상화합니다.

```rust
// src/text/measurer.rs

use document::{CharStyle, FontId};
use crate::geometry::*;

/// 텍스트 측정 결과
#[derive(Debug, Clone)]
pub struct TextMetrics {
    /// 텍스트 전체 너비 (px)
    pub width: f32,
    /// 각 문자 경계의 X 위치 (커서 배치용)
    /// 길이 = 문자 수 + 1 (마지막은 끝 위치)
    pub char_positions: Vec<f32>,
    /// 기준선 위 높이
    pub ascent: f32,
    /// 기준선 아래 깊이
    pub descent: f32,
    /// 줄 간격 (leading)
    pub leading: f32,
}

impl TextMetrics {
    /// 총 높이 (ascent + descent)
    pub fn height(&self) -> f32 {
        self.ascent + self.descent
    }

    /// 줄 높이 (ascent + descent + leading)
    pub fn line_height(&self) -> f32 {
        self.ascent + self.descent + self.leading
    }
}

/// 글꼴 메트릭 (글꼴 전체에 대한 정보)
#[derive(Debug, Clone)]
pub struct FontMetrics {
    /// 기준선 위 높이 (px)
    pub ascent: f32,
    /// 기준선 아래 깊이 (px)
    pub descent: f32,
    /// 줄 간격 (px)
    pub leading: f32,
    /// 대문자 높이 (px)
    pub cap_height: f32,
    /// 소문자 x 높이 (px)
    pub x_height: f32,
    /// em 단위 크기 (px)
    pub em_size: f32,
}

/// 텍스트 측정 trait (플랫폼별 구현)
pub trait TextMeasurer {
    /// 텍스트 측정
    ///
    /// # Arguments
    /// * `text` - 측정할 텍스트
    /// * `style` - 문자 스타일
    ///
    /// # Returns
    /// 측정 결과 (너비, 문자별 위치, 높이 정보)
    fn measure_text(&self, text: &str, style: &CharStyle) -> TextMetrics;

    /// 글꼴 메트릭 조회
    ///
    /// # Arguments
    /// * `font_id` - 글꼴 ID
    /// * `font_size` - 글꼴 크기 (HwpUnit)
    fn font_metrics(&self, font_id: FontId, font_size: i32) -> FontMetrics;

    /// 특정 문자의 너비 측정
    fn measure_char(&self, c: char, style: &CharStyle) -> f32 {
        self.measure_text(&c.to_string(), style).width
    }
}
```

### 3.3 TextShaper trait (정밀 셰이핑)

복잡한 스크립트(아랍어, 옛한글 등)를 위한 정밀 셰이핑입니다.

```rust
// src/text/shaper.rs

/// 셰이핑된 글리프
#[derive(Debug, Clone)]
pub struct ShapedGlyph {
    /// 글리프 ID (글꼴 내부 인덱스)
    pub glyph_id: u16,
    /// 원본 텍스트의 클러스터 인덱스
    pub cluster: usize,
    /// X 오프셋 (기준 위치에서)
    pub x_offset: f32,
    /// Y 오프셋
    pub y_offset: f32,
    /// 다음 글리프까지의 전진 너비
    pub x_advance: f32,
    /// 다음 글리프까지의 전진 높이 (세로쓰기용)
    pub y_advance: f32,
}

/// 셰이핑된 텍스트 run
#[derive(Debug, Clone)]
pub struct ShapedRun {
    /// 셰이핑된 글리프들
    pub glyphs: Vec<ShapedGlyph>,
    /// 총 너비
    pub width: f32,
    /// 총 높이
    pub height: f32,
}

/// 텍스트 셰이핑 trait
pub trait TextShaper {
    /// 텍스트 셰이핑
    fn shape(&self, text: &str, style: &CharStyle) -> ShapedRun;

    /// 복잡한 셰이핑이 필요한지 판단
    fn needs_complex_shaping(&self, text: &str) -> bool {
        text.chars().any(|c| {
            matches!(c,
                '\u{0600}'..='\u{06FF}' |  // 아랍어
                '\u{0900}'..='\u{097F}' |  // 데바나가리
                '\u{0E00}'..='\u{0E7F}' |  // 태국어
                '\u{1100}'..='\u{11FF}' |  // 한글 자모 (옛한글)
                '\u{A960}'..='\u{A97F}'    // 한글 자모 확장
            )
        })
    }
}

/// 하이브리드 셰이퍼 (단순 + 정밀 조합)
pub struct HybridShaper<S: TextMeasurer, F: TextShaper> {
    /// 단순 셰이퍼 (일반 텍스트용)
    simple: S,
    /// 정밀 셰이퍼 (복잡한 스크립트용, Option)
    full: Option<F>,
}

impl<S: TextMeasurer, F: TextShaper> HybridShaper<S, F> {
    pub fn new(simple: S, full: Option<F>) -> Self {
        Self { simple, full }
    }

    pub fn shape(&self, text: &str, style: &CharStyle) -> ShapedRun {
        if let Some(ref full) = self.full {
            if full.needs_complex_shaping(text) {
                return full.shape(text, style);
            }
        }

        // 단순 측정을 ShapedRun으로 변환
        let metrics = self.simple.measure_text(text, style);
        self.metrics_to_shaped_run(text, &metrics)
    }

    fn metrics_to_shaped_run(&self, text: &str, metrics: &TextMetrics) -> ShapedRun {
        let mut glyphs = Vec::new();
        let mut prev_x = 0.0;

        for (i, (byte_idx, c)) in text.char_indices().enumerate() {
            let x = metrics.char_positions.get(i).copied().unwrap_or(prev_x);
            let next_x = metrics.char_positions.get(i + 1).copied().unwrap_or(metrics.width);

            glyphs.push(ShapedGlyph {
                glyph_id: 0,  // 단순 모드에서는 사용 안 함
                cluster: byte_idx,
                x_offset: x,
                y_offset: 0.0,
                x_advance: next_x - x,
                y_advance: 0.0,
            });

            prev_x = next_x;
        }

        ShapedRun {
            glyphs,
            width: metrics.width,
            height: metrics.height(),
        }
    }
}
```

---

## 4. 줄바꿈

### 4.1 LineBreakConfig

HWP 호환 줄바꿈 설정입니다.

```rust
// src/text/line_break_config.rs

/// 줄바꿈 설정
#[derive(Debug, Clone)]
pub struct LineBreakConfig {
    /// 한글 줄바꿈 단위
    pub korean_unit: KoreanLineBreakUnit,
    /// 영어 줄바꿈 단위
    pub english_unit: EnglishLineBreakUnit,
    /// 행두 금지 문자 (줄 시작에 올 수 없는 문자)
    pub no_line_start: String,
    /// 행말 금지 문자 (줄 끝에 올 수 없는 문자)
    pub no_line_end: String,
    /// 낱자 방지 (외톨이 글자)
    pub prevent_orphan: bool,
    /// 공백 압축
    pub compress_whitespace: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KoreanLineBreakUnit {
    /// 어절 단위 (공백 기준) - CSS word-break: keep-all
    Word,
    /// 글자 단위 - CSS word-break: break-all
    Character,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnglishLineBreakUnit {
    /// 단어 단위
    Word,
    /// 하이픈 허용
    Hyphenate,
    /// 글자 단위
    Character,
}

impl Default for LineBreakConfig {
    fn default() -> Self {
        Self {
            korean_unit: KoreanLineBreakUnit::Word,
            english_unit: EnglishLineBreakUnit::Word,
            // 한글 행두 금지 문자 (KS X 1014 기반)
            no_line_start: ")]},.!?;:~'\"、。, ).!?;:".to_string(),
            // 한글 행말 금지 문자
            no_line_end: "([{~'\"「『(".to_string(),
            prevent_orphan: true,
            compress_whitespace: true,
        }
    }
}
```

### 4.2 LineBreaker

```rust
// src/text/line_break.rs

use unicode_segmentation::UnicodeSegmentation;

/// 줄바꿈 결과
#[derive(Debug, Clone)]
pub struct LineBreak {
    /// 줄 시작 오프셋 (바이트)
    pub start: usize,
    /// 줄 끝 오프셋 (바이트, exclusive)
    pub end: usize,
    /// 줄 너비
    pub width: f32,
    /// 줄바꿈 전 공백 너비 (정렬용)
    pub trailing_whitespace: f32,
}

/// 줄바꿈 기회 (potential break point)
#[derive(Debug, Clone)]
struct BreakOpportunity {
    /// 바이트 오프셋
    offset: usize,
    /// 이 위치까지의 누적 너비
    width: f32,
    /// 강제 줄바꿈 여부
    mandatory: bool,
}

/// 줄바꿈 계산기
pub struct LineBreaker {
    config: LineBreakConfig,
}

impl LineBreaker {
    pub fn new(config: LineBreakConfig) -> Self {
        Self { config }
    }

    /// 텍스트를 줄바꿈하여 줄 목록 반환
    ///
    /// # Arguments
    /// * `text` - 줄바꿈할 텍스트
    /// * `max_width` - 최대 줄 너비
    /// * `measurer` - 텍스트 측정기
    /// * `style` - 문자 스타일
    pub fn break_lines(
        &self,
        text: &str,
        max_width: f32,
        measurer: &dyn TextMeasurer,
        style: &document::CharStyle,
    ) -> Vec<LineBreak> {
        if text.is_empty() {
            return vec![LineBreak {
                start: 0,
                end: 0,
                width: 0.0,
                trailing_whitespace: 0.0,
            }];
        }

        let mut lines = Vec::new();
        let mut line_start = 0;
        let mut current_width = 0.0;
        let mut last_break_opportunity: Option<BreakOpportunity> = None;

        let metrics = measurer.measure_text(text, style);

        for (i, (byte_idx, grapheme)) in text.grapheme_indices(true).enumerate() {
            let grapheme_width = if i + 1 < metrics.char_positions.len() {
                metrics.char_positions[i + 1] - metrics.char_positions[i]
            } else {
                metrics.width - metrics.char_positions[i]
            };

            let grapheme_end = byte_idx + grapheme.len();

            // 줄바꿈 기회 판단
            if self.can_break_before(text, grapheme_end) {
                last_break_opportunity = Some(BreakOpportunity {
                    offset: grapheme_end,
                    width: current_width + grapheme_width,
                    mandatory: false,
                });
            }

            // 강제 줄바꿈 (개행 문자)
            if grapheme == "\n" || grapheme == "\r\n" {
                lines.push(LineBreak {
                    start: line_start,
                    end: byte_idx,
                    width: current_width,
                    trailing_whitespace: 0.0,
                });
                line_start = grapheme_end;
                current_width = 0.0;
                last_break_opportunity = None;
                continue;
            }

            current_width += grapheme_width;

            // 최대 너비 초과
            if current_width > max_width {
                if let Some(brk) = last_break_opportunity.take() {
                    // 마지막 줄바꿈 기회에서 줄바꿈
                    let trailing_ws = self.trailing_whitespace_width(
                        &text[line_start..brk.offset],
                        measurer,
                        style,
                    );

                    lines.push(LineBreak {
                        start: line_start,
                        end: brk.offset,
                        width: brk.width - trailing_ws,
                        trailing_whitespace: trailing_ws,
                    });

                    line_start = self.skip_leading_whitespace(text, brk.offset);
                    current_width = current_width - brk.width;
                } else {
                    // 줄바꿈 기회 없음 - 강제로 현재 위치에서 줄바꿈
                    if byte_idx > line_start {
                        lines.push(LineBreak {
                            start: line_start,
                            end: byte_idx,
                            width: current_width - grapheme_width,
                            trailing_whitespace: 0.0,
                        });
                        line_start = byte_idx;
                        current_width = grapheme_width;
                    }
                }
            }
        }

        // 마지막 줄
        if line_start < text.len() {
            let trailing_ws = self.trailing_whitespace_width(
                &text[line_start..],
                measurer,
                style,
            );

            lines.push(LineBreak {
                start: line_start,
                end: text.len(),
                width: current_width - trailing_ws,
                trailing_whitespace: trailing_ws,
            });
        }

        lines
    }

    /// 이 위치 앞에서 줄바꿈 가능한지 판단
    fn can_break_before(&self, text: &str, offset: usize) -> bool {
        if offset == 0 || offset >= text.len() {
            return false;
        }

        let before = &text[..offset];
        let after = &text[offset..];

        let prev_char = before.chars().last().unwrap();
        let next_char = after.chars().next().unwrap();

        // 행두 금지 문자 체크
        if self.config.no_line_start.contains(next_char) {
            return false;
        }

        // 행말 금지 문자 체크
        if self.config.no_line_end.contains(prev_char) {
            return false;
        }

        // 공백 뒤에서는 항상 줄바꿈 가능
        if prev_char.is_whitespace() {
            return true;
        }

        // 한글 처리
        if is_korean(prev_char) || is_korean(next_char) {
            return match self.config.korean_unit {
                KoreanLineBreakUnit::Word => {
                    // 어절 단위: 공백에서만 줄바꿈
                    prev_char.is_whitespace()
                }
                KoreanLineBreakUnit::Character => {
                    // 글자 단위: 언제나 가능
                    true
                }
            };
        }

        // 영어/기타 처리
        match self.config.english_unit {
            EnglishLineBreakUnit::Word => {
                prev_char.is_whitespace() || prev_char == '-'
            }
            EnglishLineBreakUnit::Character => true,
            EnglishLineBreakUnit::Hyphenate => {
                // TODO: 하이픈 사전 참조
                prev_char.is_whitespace() || prev_char == '-'
            }
        }
    }

    fn trailing_whitespace_width(
        &self,
        text: &str,
        measurer: &dyn TextMeasurer,
        style: &document::CharStyle,
    ) -> f32 {
        let trimmed = text.trim_end();
        if trimmed.len() == text.len() {
            return 0.0;
        }

        let full_width = measurer.measure_text(text, style).width;
        let trimmed_width = measurer.measure_text(trimmed, style).width;
        full_width - trimmed_width
    }

    fn skip_leading_whitespace(&self, text: &str, offset: usize) -> usize {
        let remaining = &text[offset..];
        let trimmed = remaining.trim_start();
        offset + (remaining.len() - trimmed.len())
    }
}

fn is_korean(c: char) -> bool {
    matches!(c,
        '\u{AC00}'..='\u{D7AF}' |  // 완성형 한글
        '\u{1100}'..='\u{11FF}' |  // 한글 자모
        '\u{3130}'..='\u{318F}'    // 호환용 한글
    )
}
```

---

## 5. 블록 레이아웃

### 5.1 ParagraphLayout

```rust
// src/block/paragraph.rs

use crate::text::{LineBreaker, LineBreak, TextMeasurer};
use crate::geometry::*;
use document::{Paragraph, ParaStyle, CharStyle, StyleRuns};

/// 문단 레이아웃 결과
#[derive(Debug, Clone)]
pub struct ParagraphLayout {
    /// 문단 전체 크기
    pub bounds: Rect,
    /// 줄 레이아웃 목록
    pub lines: Vec<LineLayout>,
}

/// 줄 레이아웃
#[derive(Debug, Clone)]
pub struct LineLayout {
    /// 줄 영역 (문단 기준 상대 좌표)
    pub bounds: Rect,
    /// 기준선 Y 위치 (줄 상단 기준)
    pub baseline: f32,
    /// Run 레이아웃 목록
    pub runs: Vec<RunLayout>,
    /// 원본 텍스트 범위 (바이트)
    pub text_range: std::ops::Range<usize>,
}

/// Run 레이아웃 (동일 스타일 텍스트 조각)
#[derive(Debug, Clone)]
pub struct RunLayout {
    /// Run 영역 (줄 기준 상대 좌표)
    pub bounds: Rect,
    /// 텍스트
    pub text: String,
    /// 스타일 ID
    pub style_id: document::CharStyleId,
    /// 글자별 X 위치 (커서 배치용)
    pub char_positions: Vec<f32>,
    /// 원본 텍스트 범위 (바이트)
    pub text_range: std::ops::Range<usize>,
}

/// 문단 레이아웃 계산기
pub struct ParagraphLayouter<'a> {
    measurer: &'a dyn TextMeasurer,
    line_breaker: LineBreaker,
}

impl<'a> ParagraphLayouter<'a> {
    pub fn new(measurer: &'a dyn TextMeasurer, line_break_config: LineBreakConfig) -> Self {
        Self {
            measurer,
            line_breaker: LineBreaker::new(line_break_config),
        }
    }

    /// 문단 레이아웃 계산
    ///
    /// # Arguments
    /// * `para` - 문단
    /// * `para_style` - 문단 스타일
    /// * `styles` - 스타일 저장소
    /// * `available_width` - 사용 가능한 너비
    pub fn layout(
        &self,
        para: &Paragraph,
        para_style: &ParaStyle,
        styles: &document::StyleStore,
        available_width: f32,
    ) -> ParagraphLayout {
        let text = para.text();

        // 들여쓰기 적용
        let content_width = available_width
            - para_style.indent_left as f32 / 100.0
            - para_style.indent_right as f32 / 100.0;

        // 스타일별로 Run 분리
        let runs = self.split_into_runs(para, styles);

        // 각 Run의 텍스트를 측정하고 줄바꿈 계산
        let mut lines = Vec::new();
        let mut y = para_style.space_before as f32 / 100.0;

        // 첫 줄 들여쓰기
        let first_line_indent = para_style.indent_first as f32 / 100.0;
        let first_line_width = content_width - first_line_indent.max(0.0);

        // 단순화된 구현: 전체 텍스트에 대해 기본 스타일로 줄바꿈 계산
        // TODO: 스타일 변경점에서 재측정
        let default_style = styles.get_char_style(para.char_style_at(0))
            .unwrap_or(&CharStyle::default_style());

        let line_breaks = self.line_breaker.break_lines(
            text,
            if lines.is_empty() { first_line_width } else { content_width },
            self.measurer,
            default_style,
        );

        for (i, line_break) in line_breaks.iter().enumerate() {
            let is_first_line = i == 0;
            let line_indent = if is_first_line { first_line_indent.max(0.0) } else { 0.0 };

            // 줄 내의 Run 레이아웃
            let line_runs = self.layout_line_runs(
                &text[line_break.start..line_break.end],
                line_break.start,
                para.char_styles(),
                styles,
            );

            // 줄 높이 계산 (가장 큰 ascent + descent)
            let line_height = self.calculate_line_height(&line_runs, para_style);

            // 정렬 적용
            let x_offset = self.calculate_alignment_offset(
                line_break.width,
                content_width - line_indent,
                para_style.alignment,
            );

            lines.push(LineLayout {
                bounds: Rect::new(
                    para_style.indent_left as f32 / 100.0 + line_indent + x_offset,
                    y,
                    line_break.width,
                    line_height,
                ),
                baseline: y + line_height * 0.8,  // 대략적인 기준선
                runs: line_runs,
                text_range: line_break.start..line_break.end,
            });

            y += line_height;
        }

        y += para_style.space_after as f32 / 100.0;

        ParagraphLayout {
            bounds: Rect::new(0.0, 0.0, available_width, y),
            lines,
        }
    }

    fn split_into_runs(
        &self,
        para: &Paragraph,
        styles: &document::StyleStore,
    ) -> Vec<(std::ops::Range<usize>, document::CharStyleId)> {
        let mut runs = Vec::new();
        let mut current_start = 0;
        let mut current_style = para.char_style_at(0);

        for (offset, _, style_id) in para.char_styles().iter() {
            if style_id != current_style && offset > current_start {
                runs.push((current_start..offset, current_style));
                current_start = offset;
                current_style = style_id;
            }
        }

        if current_start < para.text_len() {
            runs.push((current_start..para.text_len(), current_style));
        }

        runs
    }

    fn layout_line_runs(
        &self,
        line_text: &str,
        line_start_offset: usize,
        char_styles: &StyleRuns,
        styles: &document::StyleStore,
    ) -> Vec<RunLayout> {
        let mut runs = Vec::new();
        let mut x = 0.0;

        // 단순화: 전체 줄을 하나의 Run으로
        // TODO: 스타일 변경점에서 분리
        let style_id = char_styles.style_at(line_start_offset);
        let style = styles.get_char_style(style_id)
            .unwrap_or(&CharStyle::default_style());

        let metrics = self.measurer.measure_text(line_text, style);

        runs.push(RunLayout {
            bounds: Rect::new(x, 0.0, metrics.width, metrics.height()),
            text: line_text.to_string(),
            style_id,
            char_positions: metrics.char_positions,
            text_range: line_start_offset..line_start_offset + line_text.len(),
        });

        runs
    }

    fn calculate_line_height(
        &self,
        runs: &[RunLayout],
        para_style: &ParaStyle,
    ) -> f32 {
        let base_height = runs.iter()
            .map(|r| r.bounds.height)
            .fold(0.0f32, |a, b| a.max(b));

        match para_style.line_spacing {
            document::LineSpacing::Multiple(percent) => {
                base_height * (percent as f32 / 100.0)
            }
            document::LineSpacing::Fixed(value) => {
                value as f32 / 100.0
            }
            document::LineSpacing::AtLeast(value) => {
                base_height.max(value as f32 / 100.0)
            }
        }
    }

    fn calculate_alignment_offset(
        &self,
        line_width: f32,
        available_width: f32,
        alignment: document::Alignment,
    ) -> f32 {
        match alignment {
            document::Alignment::Left => 0.0,
            document::Alignment::Center => (available_width - line_width) / 2.0,
            document::Alignment::Right => available_width - line_width,
            document::Alignment::Justify => 0.0,  // TODO: 균등 배치
            document::Alignment::Distribute => 0.0,  // TODO: 균등 배분
        }
    }
}
```

---

## 6. 레이아웃 엔진

### 6.1 LayoutEngine

```rust
// src/engine.rs

use document::Document;
use crate::block::*;
use crate::geometry::*;

/// 레이아웃 엔진
pub struct LayoutEngine {
    /// 블록별 레이아웃 캐시
    block_layouts: Vec<Option<BlockLayout>>,
    /// 블록별 Y 위치
    block_positions: Vec<f32>,
    /// 전체 문서 높이
    total_height: f32,
    /// 문서 너비
    document_width: f32,
}

/// 블록 레이아웃 (enum)
#[derive(Debug, Clone)]
pub enum BlockLayout {
    Paragraph(ParagraphLayout),
    Table(TableLayout),
    Image(ImageLayout),
    Shape(ShapeLayout),
}

impl BlockLayout {
    pub fn height(&self) -> f32 {
        match self {
            BlockLayout::Paragraph(p) => p.bounds.height,
            BlockLayout::Table(t) => t.bounds.height,
            BlockLayout::Image(i) => i.bounds.height,
            BlockLayout::Shape(s) => s.bounds.height,
        }
    }
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            block_layouts: Vec::new(),
            block_positions: Vec::new(),
            total_height: 0.0,
            document_width: 0.0,
        }
    }

    /// 레이아웃 업데이트
    ///
    /// dirty 블록만 재계산
    pub fn update(
        &mut self,
        doc: &Document,
        viewport: &Rect,
        measurer: &dyn TextMeasurer,
    ) {
        self.document_width = viewport.width;

        // 블록 수 맞추기
        while self.block_layouts.len() < doc.block_count() {
            self.block_layouts.push(None);
            self.block_positions.push(0.0);
        }
        while self.block_layouts.len() > doc.block_count() {
            self.block_layouts.pop();
            self.block_positions.pop();
        }

        // dirty 블록 재계산
        let mut y = 0.0;
        let layouter = ParagraphLayouter::new(measurer, LineBreakConfig::default());

        for i in 0..doc.block_count() {
            self.block_positions[i] = y;

            let block = doc.block(i).unwrap();

            // dirty 또는 캐시 없음이면 재계산
            if block.is_layout_dirty() || self.block_layouts[i].is_none() {
                let layout = self.layout_block(block, doc, &layouter, viewport.width);
                self.block_layouts[i] = Some(layout);
            }

            y += self.block_layouts[i].as_ref().unwrap().height();
        }

        self.total_height = y;
    }

    fn layout_block(
        &self,
        block: &document::Block,
        doc: &Document,
        layouter: &ParagraphLayouter,
        available_width: f32,
    ) -> BlockLayout {
        match block {
            document::Block::Paragraph(para) => {
                let para_style = doc.styles().get_para_style(para.para_style())
                    .unwrap_or(&document::ParaStyle::default_style());

                let layout = layouter.layout(para, para_style, doc.styles(), available_width);
                BlockLayout::Paragraph(layout)
            }
            document::Block::Table(table) => {
                // TODO: TableLayout
                BlockLayout::Table(TableLayout {
                    bounds: Rect::new(0.0, 0.0, available_width, 100.0),
                    rows: Vec::new(),
                })
            }
            document::Block::Image(image) => {
                // TODO: ImageLayout
                BlockLayout::Image(ImageLayout {
                    bounds: Rect::new(0.0, 0.0, 100.0, 100.0),
                    binary_id: image.binary_id,
                })
            }
            document::Block::Shape(shape) => {
                // TODO: ShapeLayout
                BlockLayout::Shape(ShapeLayout {
                    bounds: Rect::new(0.0, 0.0, 100.0, 100.0),
                })
            }
        }
    }

    /// 블록 레이아웃 조회
    pub fn block_layout(&self, index: usize) -> Option<&BlockLayout> {
        self.block_layouts.get(index).and_then(|l| l.as_ref())
    }

    /// 블록 Y 위치 조회
    pub fn block_y(&self, index: usize) -> f32 {
        self.block_positions.get(index).copied().unwrap_or(0.0)
    }

    /// 전체 높이
    pub fn total_height(&self) -> f32 {
        self.total_height
    }

    /// 뷰포트 내의 블록 인덱스 범위
    pub fn visible_blocks(&self, viewport: &Rect) -> std::ops::Range<usize> {
        let start = self.block_at_y(viewport.y);
        let end = self.block_at_y(viewport.y + viewport.height) + 1;
        start..end.min(self.block_layouts.len())
    }

    /// Y 좌표에 해당하는 블록 인덱스
    pub fn block_at_y(&self, y: f32) -> usize {
        self.block_positions
            .iter()
            .position(|&pos| pos > y)
            .map(|i| i.saturating_sub(1))
            .unwrap_or(self.block_positions.len().saturating_sub(1))
    }
}

// 임시 구조체 (추후 구현)
#[derive(Debug, Clone)]
pub struct TableLayout {
    pub bounds: Rect,
    pub rows: Vec<TableRowLayout>,
}

#[derive(Debug, Clone)]
pub struct TableRowLayout {
    pub bounds: Rect,
    pub cells: Vec<TableCellLayout>,
}

#[derive(Debug, Clone)]
pub struct TableCellLayout {
    pub bounds: Rect,
    pub content: Box<BlockLayout>,
}

#[derive(Debug, Clone)]
pub struct ImageLayout {
    pub bounds: Rect,
    pub binary_id: document::BinaryId,
}

#[derive(Debug, Clone)]
pub struct ShapeLayout {
    pub bounds: Rect,
}
```

---

## 7. Hit Test

화면 좌표를 문서 위치로 변환합니다.

```rust
// src/hit_test.rs

use document::Position;
use crate::engine::*;
use crate::geometry::*;

/// Hit test 결과
#[derive(Debug, Clone)]
pub struct HitTestResult {
    /// 문서 위치
    pub position: Position,
    /// 글자의 뒤쪽을 클릭했는지
    pub is_trailing: bool,
    /// 클릭한 영역 유형
    pub hit_type: HitType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitType {
    /// 텍스트 내부
    Text,
    /// 줄 끝 (텍스트 오른쪽 여백)
    LineEnd,
    /// 문단 끝 (마지막 줄 아래)
    ParagraphEnd,
    /// 블록 외부
    Outside,
}

impl LayoutEngine {
    /// 화면 좌표 → 문서 위치
    pub fn hit_test(&self, point: Point) -> HitTestResult {
        // 1. 블록 찾기
        let block_index = self.block_at_y(point.y);

        if block_index >= self.block_layouts.len() {
            return HitTestResult {
                position: Position::new(self.block_layouts.len().saturating_sub(1), 0),
                is_trailing: true,
                hit_type: HitType::Outside,
            };
        }

        let block_y = self.block_y(block_index);
        let local_point = Point::new(point.x, point.y - block_y);

        match self.block_layout(block_index) {
            Some(BlockLayout::Paragraph(layout)) => {
                self.hit_test_paragraph(block_index, layout, local_point)
            }
            Some(BlockLayout::Table(_)) => {
                // TODO: 표 hit test
                HitTestResult {
                    position: Position::new(block_index, 0),
                    is_trailing: false,
                    hit_type: HitType::Outside,
                }
            }
            _ => {
                HitTestResult {
                    position: Position::new(block_index, 0),
                    is_trailing: false,
                    hit_type: HitType::Outside,
                }
            }
        }
    }

    fn hit_test_paragraph(
        &self,
        block_index: usize,
        layout: &ParagraphLayout,
        point: Point,
    ) -> HitTestResult {
        // 2. 줄 찾기
        let line = layout.lines.iter().find(|line| {
            point.y >= line.bounds.y && point.y < line.bounds.y + line.bounds.height
        });

        let line = match line {
            Some(l) => l,
            None => {
                // 문단 영역 밖
                let last_line = layout.lines.last();
                if point.y < 0.0 {
                    return HitTestResult {
                        position: Position::new(block_index, 0),
                        is_trailing: false,
                        hit_type: HitType::Outside,
                    };
                } else if let Some(last) = last_line {
                    return HitTestResult {
                        position: Position::new(block_index, last.text_range.end),
                        is_trailing: true,
                        hit_type: HitType::ParagraphEnd,
                    };
                } else {
                    return HitTestResult {
                        position: Position::new(block_index, 0),
                        is_trailing: false,
                        hit_type: HitType::Outside,
                    };
                }
            }
        };

        // 3. 줄 내에서 문자 찾기
        let local_x = point.x - line.bounds.x;

        if local_x < 0.0 {
            return HitTestResult {
                position: Position::new(block_index, line.text_range.start),
                is_trailing: false,
                hit_type: HitType::Text,
            };
        }

        if local_x > line.bounds.width {
            return HitTestResult {
                position: Position::new(block_index, line.text_range.end),
                is_trailing: true,
                hit_type: HitType::LineEnd,
            };
        }

        // Run 내에서 문자 찾기
        for run in &line.runs {
            if local_x >= run.bounds.x && local_x < run.bounds.x + run.bounds.width {
                return self.hit_test_run(block_index, run, local_x - run.bounds.x);
            }
        }

        HitTestResult {
            position: Position::new(block_index, line.text_range.end),
            is_trailing: true,
            hit_type: HitType::LineEnd,
        }
    }

    fn hit_test_run(
        &self,
        block_index: usize,
        run: &RunLayout,
        x: f32,
    ) -> HitTestResult {
        // 문자 위치 검색
        for (i, &char_x) in run.char_positions.iter().enumerate() {
            let next_x = run.char_positions.get(i + 1).copied()
                .unwrap_or(run.bounds.width);

            if x >= char_x && x < next_x {
                let mid = (char_x + next_x) / 2.0;
                let is_trailing = x >= mid;

                // 바이트 오프셋 계산
                let char_offset = run.text.char_indices()
                    .nth(i)
                    .map(|(idx, _)| idx)
                    .unwrap_or(0);

                let byte_offset = if is_trailing {
                    run.text.char_indices()
                        .nth(i + 1)
                        .map(|(idx, _)| idx)
                        .unwrap_or(run.text.len())
                } else {
                    char_offset
                };

                return HitTestResult {
                    position: Position::new(block_index, run.text_range.start + byte_offset),
                    is_trailing,
                    hit_type: HitType::Text,
                };
            }
        }

        HitTestResult {
            position: Position::new(block_index, run.text_range.end),
            is_trailing: true,
            hit_type: HitType::Text,
        }
    }

    /// 문서 위치 → 화면 좌표 (커서 렌더링용)
    pub fn position_to_rect(&self, pos: Position) -> Option<Rect> {
        let layout = self.block_layout(pos.block_index)?;
        let block_y = self.block_y(pos.block_index);

        match layout {
            BlockLayout::Paragraph(para) => {
                // 해당 오프셋이 포함된 줄 찾기
                for line in &para.lines {
                    if pos.offset >= line.text_range.start && pos.offset <= line.text_range.end {
                        // Run 내에서 X 위치 찾기
                        for run in &line.runs {
                            if pos.offset >= run.text_range.start && pos.offset <= run.text_range.end {
                                let local_offset = pos.offset - run.text_range.start;

                                // 문자 인덱스로 변환
                                let char_index = run.text[..local_offset].chars().count();
                                let x = run.char_positions.get(char_index).copied()
                                    .unwrap_or(0.0);

                                return Some(Rect::new(
                                    line.bounds.x + run.bounds.x + x,
                                    block_y + line.bounds.y,
                                    2.0,  // 커서 너비
                                    line.bounds.height,
                                ));
                            }
                        }
                    }
                }

                // 기본값: 문단 시작
                Some(Rect::new(0.0, block_y, 2.0, para.bounds.height.max(20.0)))
            }
            _ => {
                Some(Rect::new(0.0, block_y, 2.0, 20.0))
            }
        }
    }
}
```

---

## 8. 테스트 가이드

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Mock TextMeasurer
    struct MockMeasurer;

    impl TextMeasurer for MockMeasurer {
        fn measure_text(&self, text: &str, _style: &CharStyle) -> TextMetrics {
            // 단순화: 각 문자 10px 너비
            let char_count = text.chars().count();
            let width = char_count as f32 * 10.0;

            TextMetrics {
                width,
                char_positions: (0..=char_count).map(|i| i as f32 * 10.0).collect(),
                ascent: 12.0,
                descent: 4.0,
                leading: 2.0,
            }
        }

        fn font_metrics(&self, _font_id: FontId, _font_size: i32) -> FontMetrics {
            FontMetrics {
                ascent: 12.0,
                descent: 4.0,
                leading: 2.0,
                cap_height: 10.0,
                x_height: 8.0,
                em_size: 16.0,
            }
        }
    }

    #[test]
    fn test_line_break_simple() {
        let breaker = LineBreaker::new(LineBreakConfig::default());
        let measurer = MockMeasurer;
        let style = CharStyle::default_style();

        let lines = breaker.break_lines(
            "Hello World",
            50.0,  // 5문자 너비
            &measurer,
            &style,
        );

        assert_eq!(lines.len(), 2);
        assert_eq!(&"Hello World"[lines[0].start..lines[0].end], "Hello ");
    }

    #[test]
    fn test_hit_test() {
        let mut engine = LayoutEngine::new();
        let mut doc = Document::new();

        // 텍스트 추가
        if let Some(para) = doc.block_mut(0).and_then(|b| b.as_paragraph_mut()) {
            para.insert_text(0, "Hello");
        }

        let viewport = Rect::new(0.0, 0.0, 200.0, 100.0);
        engine.update(&doc, &viewport, &MockMeasurer);

        // 첫 글자 클릭
        let result = engine.hit_test(Point::new(5.0, 8.0));
        assert_eq!(result.position.block_index, 0);
        assert_eq!(result.position.offset, 0);
    }
}
```

---

## 변경 이력

| 날짜 | 버전 | 내용 |
|------|------|------|
| 2024-12 | 0.1 | 초안 작성 |
