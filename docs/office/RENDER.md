# render 크레이트 상세 명세

이 문서는 `render-api` 및 `render-web` 크레이트의 구현 상세를 정의합니다.

> **상위 문서**: [DESIGN.md](./DESIGN.md)

---

## 1. 개요

### 1.1 목적

렌더링 레이어는 **레이아웃 결과를 화면에 표시**합니다.

- `render-api`: 플랫폼 독립적 렌더링 추상화
- `render-web`: Canvas 2D API를 사용한 웹 구현
- `render-native`: tiny-skia를 사용한 네이티브 구현 (추후)

### 1.2 설계 원칙

1. **명령 기반**: 렌더링 명령 목록을 생성하고 배치로 실행
2. **플랫폼 추상화**: trait로 렌더러 인터페이스 정의
3. **성능 최적화**: 클리핑, 배치 렌더링, 캐싱

---

## 2. render-api 크레이트

### 2.1 파일 구조

```
crates/render-api/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── command.rs      # RenderCommand enum
    ├── renderer.rs     # Renderer trait
    ├── text.rs         # TextRenderMethod
    ├── color.rs        # Color
    ├── geometry.rs     # Point, Rect (layout에서 재export 또는 복사)
    └── style.rs        # RenderStyle
```

### 2.2 RenderCommand

```rust
// render-api/src/command.rs

use crate::{Color, Point, Rect, TextRenderMethod, RenderStyle};

/// 렌더링 명령
#[derive(Debug, Clone)]
pub enum RenderCommand {
    // === 도형 ===

    /// 사각형 채우기
    FillRect {
        rect: Rect,
        color: Color,
    },

    /// 사각형 테두리
    StrokeRect {
        rect: Rect,
        color: Color,
        width: f32,
    },

    /// 둥근 사각형 채우기
    FillRoundRect {
        rect: Rect,
        color: Color,
        radius: f32,
    },

    /// 선 그리기
    DrawLine {
        start: Point,
        end: Point,
        color: Color,
        width: f32,
    },

    // === 텍스트 ===

    /// 텍스트 렌더링
    DrawText {
        method: TextRenderMethod,
    },

    // === 이미지 ===

    /// 이미지 렌더링
    DrawImage {
        image_id: ImageId,
        rect: Rect,
    },

    // === 클리핑 ===

    /// 클리핑 영역 설정
    PushClip {
        rect: Rect,
    },

    /// 클리핑 영역 해제
    PopClip,

    // === 변환 ===

    /// 변환 행렬 저장
    Save,

    /// 변환 행렬 복원
    Restore,

    /// 이동
    Translate {
        x: f32,
        y: f32,
    },

    /// 스케일
    Scale {
        x: f32,
        y: f32,
    },

    // === 투명도 ===

    /// 전역 투명도 설정
    SetOpacity {
        opacity: f32,
    },
}

/// 이미지 ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(pub u64);

/// 렌더링 명령 목록
#[derive(Debug, Clone, Default)]
pub struct RenderCommands {
    commands: Vec<RenderCommand>,
}

impl RenderCommands {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn push(&mut self, cmd: RenderCommand) {
        self.commands.push(cmd);
    }

    pub fn extend(&mut self, cmds: impl IntoIterator<Item = RenderCommand>) {
        self.commands.extend(cmds);
    }

    pub fn iter(&self) -> impl Iterator<Item = &RenderCommand> {
        self.commands.iter()
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

impl IntoIterator for RenderCommands {
    type Item = RenderCommand;
    type IntoIter = std::vec::IntoIter<RenderCommand>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}
```

### 2.3 TextRenderMethod

```rust
// render-api/src/text.rs

use crate::{Color, Point};

/// 텍스트 렌더링 방법
#[derive(Debug, Clone)]
pub enum TextRenderMethod {
    /// 문자열 기반 (웹 호환)
    ///
    /// 브라우저가 셰이핑과 렌더링을 수행
    String(StringText),

    /// 글리프 기반 (네이티브 최적화)
    ///
    /// 사전에 셰이핑된 글리프를 직접 렌더링
    Glyphs(GlyphText),
}

/// 문자열 기반 텍스트
#[derive(Debug, Clone)]
pub struct StringText {
    /// 렌더링할 텍스트
    pub text: String,
    /// 렌더링 시작 위치 (기준선 왼쪽)
    pub position: Point,
    /// 텍스트 스타일
    pub style: TextStyle,
}

/// 글리프 기반 텍스트
#[derive(Debug, Clone)]
pub struct GlyphText {
    /// 글리프 목록
    pub glyphs: Vec<PositionedGlyph>,
    /// 글꼴 ID
    pub font_id: u32,
    /// 글꼴 크기 (px)
    pub font_size: f32,
    /// 색상
    pub color: Color,
}

/// 위치가 지정된 글리프
#[derive(Debug, Clone)]
pub struct PositionedGlyph {
    /// 글리프 ID (글꼴 내부 인덱스)
    pub glyph_id: u16,
    /// X 위치
    pub x: f32,
    /// Y 위치 (기준선)
    pub y: f32,
}

/// 텍스트 스타일 (렌더링용)
#[derive(Debug, Clone)]
pub struct TextStyle {
    /// 글꼴 패밀리
    pub font_family: String,
    /// 글꼴 크기 (px)
    pub font_size: f32,
    /// 글꼴 두께 (100-900)
    pub font_weight: u16,
    /// 이탤릭
    pub italic: bool,
    /// 색상
    pub color: Color,
    /// 밑줄
    pub underline: Option<UnderlineStyle>,
    /// 취소선
    pub strikethrough: Option<StrikethroughStyle>,
    /// 자간 (px)
    pub letter_spacing: f32,
}

#[derive(Debug, Clone)]
pub struct UnderlineStyle {
    pub color: Color,
    pub style: LineStyle,
}

#[derive(Debug, Clone)]
pub struct StrikethroughStyle {
    pub color: Color,
    pub style: LineStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineStyle {
    Solid,
    Dashed,
    Dotted,
    Double,
    Wavy,
}

impl TextStyle {
    /// CSS font 문자열 생성
    pub fn to_css_font(&self) -> String {
        let style = if self.italic { "italic" } else { "normal" };
        format!(
            "{} {} {}px {}",
            style,
            self.font_weight,
            self.font_size,
            self.font_family
        )
    }

    /// document::CharStyle에서 변환
    pub fn from_char_style(
        style: &document::CharStyle,
        fonts: &document::StyleStore,
    ) -> Self {
        let font_name = fonts.get_font(style.font_family)
            .map(|f| f.name.clone())
            .unwrap_or_else(|| "sans-serif".to_string());

        Self {
            font_family: font_name,
            font_size: style.font_size as f32 / 100.0,  // HwpUnit → px
            font_weight: match style.font_weight {
                document::FontWeight::Thin => 100,
                document::FontWeight::ExtraLight => 200,
                document::FontWeight::Light => 300,
                document::FontWeight::Normal => 400,
                document::FontWeight::Medium => 500,
                document::FontWeight::SemiBold => 600,
                document::FontWeight::Bold => 700,
                document::FontWeight::ExtraBold => 800,
                document::FontWeight::Black => 900,
            },
            italic: matches!(style.font_style, document::FontStyle::Italic | document::FontStyle::Oblique),
            color: Color::from_document_color(&style.color),
            underline: style.underline.as_ref().map(|u| UnderlineStyle {
                color: Color::from_document_color(&u.color),
                style: convert_line_style(&u.style),
            }),
            strikethrough: style.strikethrough.as_ref().map(|s| StrikethroughStyle {
                color: Color::from_document_color(&s.color),
                style: convert_line_style(&s.style),
            }),
            letter_spacing: style.letter_spacing as f32 / 100.0,
        }
    }
}

fn convert_line_style(style: &document::LineStyle) -> LineStyle {
    match style {
        document::LineStyle::Solid => LineStyle::Solid,
        document::LineStyle::Dashed => LineStyle::Dashed,
        document::LineStyle::Dotted => LineStyle::Dotted,
        document::LineStyle::Double => LineStyle::Double,
        document::LineStyle::Wavy => LineStyle::Wavy,
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_family: "sans-serif".to_string(),
            font_size: 16.0,
            font_weight: 400,
            italic: false,
            color: Color::BLACK,
            underline: None,
            strikethrough: None,
            letter_spacing: 0.0,
        }
    }
}
```

### 2.4 Color

```rust
// render-api/src/color.rs

/// 색상 (RGBA)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// CSS 색상 문자열로 변환
    pub fn to_css(&self) -> String {
        if self.a == 255 {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a as f32 / 255.0)
        }
    }

    /// document::Color에서 변환
    pub fn from_document_color(c: &document::Color) -> Self {
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }

    /// 투명도 적용
    pub fn with_alpha(self, alpha: u8) -> Self {
        Self { a: alpha, ..self }
    }

    /// 밝기 조절
    pub fn lighten(self, amount: f32) -> Self {
        Self {
            r: (self.r as f32 + (255.0 - self.r as f32) * amount) as u8,
            g: (self.g as f32 + (255.0 - self.g as f32) * amount) as u8,
            b: (self.b as f32 + (255.0 - self.b as f32) * amount) as u8,
            a: self.a,
        }
    }

    /// 어둡게
    pub fn darken(self, amount: f32) -> Self {
        Self {
            r: (self.r as f32 * (1.0 - amount)) as u8,
            g: (self.g as f32 * (1.0 - amount)) as u8,
            b: (self.b as f32 * (1.0 - amount)) as u8,
            a: self.a,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}
```

### 2.5 Renderer trait

```rust
// render-api/src/renderer.rs

use crate::{RenderCommand, RenderCommands, Color, Rect, Point, TextRenderMethod, ImageId};

/// 렌더러 trait
pub trait Renderer {
    /// 렌더링 명령 실행
    fn execute(&mut self, commands: &RenderCommands);

    /// 단일 명령 실행
    fn execute_command(&mut self, command: &RenderCommand);

    /// 프레임 시작 (버퍼 클리어 등)
    fn begin_frame(&mut self, clear_color: Color);

    /// 프레임 종료 (버퍼 스왑 등)
    fn end_frame(&mut self);

    /// 글리프 렌더링 지원 여부
    fn supports_glyph_rendering(&self) -> bool {
        false
    }

    /// 캔버스 크기
    fn size(&self) -> (f32, f32);

    /// 캔버스 크기 변경
    fn resize(&mut self, width: f32, height: f32);
}

/// 렌더러 팩토리 (플랫폼별 생성)
pub trait RendererFactory {
    type Renderer: Renderer;

    fn create(&self) -> Self::Renderer;
}
```

---

## 3. render-web 크레이트

### 3.1 파일 구조

```
crates/render-web/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── canvas_renderer.rs   # CanvasRenderer
    ├── text_measurer.rs     # CanvasTextMeasurer
    └── image_cache.rs       # 이미지 캐시
```

### 3.2 Cargo.toml

```toml
[package]
name = "render-web"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
render-api = { path = "../render-api" }
layout = { path = "../layout" }
document = { path = "../document" }

wasm-bindgen = "0.2"
js-sys = "0.3"

[dependencies.web-sys]
version = "0.3"
features = [
    "Window",
    "Document",
    "Element",
    "HtmlCanvasElement",
    "CanvasRenderingContext2d",
    "TextMetrics",
    "ImageData",
    "HtmlImageElement",
    "DomRect",
]
```

### 3.3 CanvasRenderer

```rust
// render-web/src/canvas_renderer.rs

use render_api::*;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Canvas 2D 렌더러
pub struct CanvasRenderer {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    width: f32,
    height: f32,
    /// 클리핑 스택 깊이
    clip_depth: usize,
    /// 변환 스택 깊이
    save_depth: usize,
}

impl CanvasRenderer {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let ctx = canvas
            .get_context("2d")?
            .ok_or("Failed to get 2d context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let width = canvas.width() as f32;
        let height = canvas.height() as f32;

        // 기본 설정
        ctx.set_text_baseline("alphabetic");
        ctx.set_text_align("left");

        Ok(Self {
            canvas,
            ctx,
            width,
            height,
            clip_depth: 0,
            save_depth: 0,
        })
    }

    /// Canvas 요소 참조
    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    /// Context 참조
    pub fn context(&self) -> &CanvasRenderingContext2d {
        &self.ctx
    }
}

impl Renderer for CanvasRenderer {
    fn execute(&mut self, commands: &RenderCommands) {
        for cmd in commands.iter() {
            self.execute_command(cmd);
        }
    }

    fn execute_command(&mut self, command: &RenderCommand) {
        match command {
            // === 도형 ===
            RenderCommand::FillRect { rect, color } => {
                self.ctx.set_fill_style(&JsValue::from_str(&color.to_css()));
                self.ctx.fill_rect(
                    rect.x as f64,
                    rect.y as f64,
                    rect.width as f64,
                    rect.height as f64,
                );
            }

            RenderCommand::StrokeRect { rect, color, width } => {
                self.ctx.set_stroke_style(&JsValue::from_str(&color.to_css()));
                self.ctx.set_line_width(*width as f64);
                self.ctx.stroke_rect(
                    rect.x as f64,
                    rect.y as f64,
                    rect.width as f64,
                    rect.height as f64,
                );
            }

            RenderCommand::FillRoundRect { rect, color, radius } => {
                self.ctx.set_fill_style(&JsValue::from_str(&color.to_css()));
                self.draw_round_rect(rect, *radius);
                self.ctx.fill();
            }

            RenderCommand::DrawLine { start, end, color, width } => {
                self.ctx.set_stroke_style(&JsValue::from_str(&color.to_css()));
                self.ctx.set_line_width(*width as f64);
                self.ctx.begin_path();
                self.ctx.move_to(start.x as f64, start.y as f64);
                self.ctx.line_to(end.x as f64, end.y as f64);
                self.ctx.stroke();
            }

            // === 텍스트 ===
            RenderCommand::DrawText { method } => {
                self.draw_text(method);
            }

            // === 이미지 ===
            RenderCommand::DrawImage { image_id, rect } => {
                // TODO: 이미지 캐시에서 로드하여 렌더링
            }

            // === 클리핑 ===
            RenderCommand::PushClip { rect } => {
                self.ctx.save();
                self.ctx.begin_path();
                self.ctx.rect(
                    rect.x as f64,
                    rect.y as f64,
                    rect.width as f64,
                    rect.height as f64,
                );
                self.ctx.clip();
                self.clip_depth += 1;
            }

            RenderCommand::PopClip => {
                if self.clip_depth > 0 {
                    self.ctx.restore();
                    self.clip_depth -= 1;
                }
            }

            // === 변환 ===
            RenderCommand::Save => {
                self.ctx.save();
                self.save_depth += 1;
            }

            RenderCommand::Restore => {
                if self.save_depth > 0 {
                    self.ctx.restore();
                    self.save_depth -= 1;
                }
            }

            RenderCommand::Translate { x, y } => {
                let _ = self.ctx.translate(*x as f64, *y as f64);
            }

            RenderCommand::Scale { x, y } => {
                let _ = self.ctx.scale(*x as f64, *y as f64);
            }

            // === 투명도 ===
            RenderCommand::SetOpacity { opacity } => {
                self.ctx.set_global_alpha(*opacity as f64);
            }
        }
    }

    fn begin_frame(&mut self, clear_color: Color) {
        // 캔버스 클리어
        self.ctx.set_fill_style(&JsValue::from_str(&clear_color.to_css()));
        self.ctx.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);

        // 상태 초기화
        while self.clip_depth > 0 {
            self.ctx.restore();
            self.clip_depth -= 1;
        }
        while self.save_depth > 0 {
            self.ctx.restore();
            self.save_depth -= 1;
        }

        self.ctx.set_global_alpha(1.0);
    }

    fn end_frame(&mut self) {
        // Canvas 2D는 즉시 모드이므로 특별한 처리 불필요
    }

    fn supports_glyph_rendering(&self) -> bool {
        false
    }

    fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    fn resize(&mut self, width: f32, height: f32) {
        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
        self.width = width;
        self.height = height;

        // 리사이즈 후 기본 설정 재적용
        self.ctx.set_text_baseline("alphabetic");
        self.ctx.set_text_align("left");
    }
}

impl CanvasRenderer {
    fn draw_text(&self, method: &TextRenderMethod) {
        match method {
            TextRenderMethod::String(text) => {
                // 글꼴 설정
                self.ctx.set_font(&text.style.to_css_font());
                self.ctx.set_fill_style(&JsValue::from_str(&text.style.color.to_css()));

                // 자간 적용 (글자별 렌더링)
                if text.style.letter_spacing != 0.0 {
                    self.draw_text_with_spacing(text);
                } else {
                    let _ = self.ctx.fill_text(
                        &text.text,
                        text.position.x as f64,
                        text.position.y as f64,
                    );
                }

                // 밑줄
                if let Some(underline) = &text.style.underline {
                    self.draw_underline(text, underline);
                }

                // 취소선
                if let Some(strike) = &text.style.strikethrough {
                    self.draw_strikethrough(text, strike);
                }
            }

            TextRenderMethod::Glyphs(_) => {
                // Canvas는 글리프 렌더링 미지원
                // fallback: 에러 로그
                web_sys::console::warn_1(&"Glyph rendering not supported in Canvas".into());
            }
        }
    }

    fn draw_text_with_spacing(&self, text: &StringText) {
        let mut x = text.position.x;

        for c in text.text.chars() {
            let s = c.to_string();
            let _ = self.ctx.fill_text(&s, x as f64, text.position.y as f64);

            // 문자 너비 측정
            let metrics = self.ctx.measure_text(&s).unwrap();
            x += metrics.width() as f32 + text.style.letter_spacing;
        }
    }

    fn draw_underline(&self, text: &StringText, style: &UnderlineStyle) {
        let metrics = self.ctx.measure_text(&text.text).unwrap();
        let width = metrics.width() as f32;

        // 기준선 아래에 밑줄
        let y = text.position.y + text.style.font_size * 0.15;

        self.ctx.set_stroke_style(&JsValue::from_str(&style.color.to_css()));
        self.ctx.set_line_width(1.0);

        self.ctx.begin_path();
        self.ctx.move_to(text.position.x as f64, y as f64);
        self.ctx.line_to((text.position.x + width) as f64, y as f64);
        self.ctx.stroke();
    }

    fn draw_strikethrough(&self, text: &StringText, style: &StrikethroughStyle) {
        let metrics = self.ctx.measure_text(&text.text).unwrap();
        let width = metrics.width() as f32;

        // 텍스트 중간에 취소선
        let y = text.position.y - text.style.font_size * 0.3;

        self.ctx.set_stroke_style(&JsValue::from_str(&style.color.to_css()));
        self.ctx.set_line_width(1.0);

        self.ctx.begin_path();
        self.ctx.move_to(text.position.x as f64, y as f64);
        self.ctx.line_to((text.position.x + width) as f64, y as f64);
        self.ctx.stroke();
    }

    fn draw_round_rect(&self, rect: &Rect, radius: f32) {
        let r = radius as f64;
        let x = rect.x as f64;
        let y = rect.y as f64;
        let w = rect.width as f64;
        let h = rect.height as f64;

        self.ctx.begin_path();
        self.ctx.move_to(x + r, y);
        self.ctx.line_to(x + w - r, y);
        self.ctx.arc_to(x + w, y, x + w, y + r, r).unwrap();
        self.ctx.line_to(x + w, y + h - r);
        self.ctx.arc_to(x + w, y + h, x + w - r, y + h, r).unwrap();
        self.ctx.line_to(x + r, y + h);
        self.ctx.arc_to(x, y + h, x, y + h - r, r).unwrap();
        self.ctx.line_to(x, y + r);
        self.ctx.arc_to(x, y, x + r, y, r).unwrap();
        self.ctx.close_path();
    }
}
```

### 3.4 CanvasTextMeasurer

```rust
// render-web/src/text_measurer.rs

use layout::text::{TextMeasurer, TextMetrics, FontMetrics};
use document::{CharStyle, FontId};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

/// Canvas 기반 텍스트 측정기
pub struct CanvasTextMeasurer {
    ctx: CanvasRenderingContext2d,
}

impl CanvasTextMeasurer {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self { ctx }
    }

    /// CharStyle을 CSS font 문자열로 변환
    fn style_to_css_font(&self, style: &CharStyle, fonts: Option<&document::StyleStore>) -> String {
        let font_name = fonts
            .and_then(|f| f.get_font(style.font_family))
            .map(|f| f.name.clone())
            .unwrap_or_else(|| "sans-serif".to_string());

        let weight = match style.font_weight {
            document::FontWeight::Thin => 100,
            document::FontWeight::ExtraLight => 200,
            document::FontWeight::Light => 300,
            document::FontWeight::Normal => 400,
            document::FontWeight::Medium => 500,
            document::FontWeight::SemiBold => 600,
            document::FontWeight::Bold => 700,
            document::FontWeight::ExtraBold => 800,
            document::FontWeight::Black => 900,
        };

        let style_str = match style.font_style {
            document::FontStyle::Normal => "normal",
            document::FontStyle::Italic => "italic",
            document::FontStyle::Oblique => "oblique",
        };

        let size = style.font_size as f32 / 100.0;  // HwpUnit → px

        format!("{} {} {}px {}", style_str, weight, size, font_name)
    }
}

impl TextMeasurer for CanvasTextMeasurer {
    fn measure_text(&self, text: &str, style: &CharStyle) -> TextMetrics {
        let css_font = self.style_to_css_font(style, None);
        self.ctx.set_font(&css_font);

        // 전체 너비
        let full_metrics = self.ctx.measure_text(text).unwrap();
        let width = full_metrics.width() as f32;

        // 각 문자 경계 위치 계산
        let mut char_positions = vec![0.0];
        let mut current_text = String::new();

        for c in text.chars() {
            current_text.push(c);
            let metrics = self.ctx.measure_text(&current_text).unwrap();
            char_positions.push(metrics.width() as f32);
        }

        // 수직 메트릭 (Canvas에서는 제한적)
        let font_size = style.font_size as f32 / 100.0;
        let ascent = font_size * 0.8;   // 대략적인 값
        let descent = font_size * 0.2;
        let leading = font_size * 0.1;

        // 자간 적용
        if style.letter_spacing != 0 {
            let spacing = style.letter_spacing as f32 / 100.0;
            let char_count = text.chars().count();
            let total_spacing = spacing * (char_count.saturating_sub(1)) as f32;

            // 위치 재계산
            for (i, pos) in char_positions.iter_mut().enumerate().skip(1) {
                *pos += spacing * (i - 1) as f32;
            }

            return TextMetrics {
                width: width + total_spacing,
                char_positions,
                ascent,
                descent,
                leading,
            };
        }

        TextMetrics {
            width,
            char_positions,
            ascent,
            descent,
            leading,
        }
    }

    fn font_metrics(&self, font_id: FontId, font_size: i32) -> FontMetrics {
        // Canvas에서는 정확한 글꼴 메트릭 접근이 제한적
        let size = font_size as f32 / 100.0;

        FontMetrics {
            ascent: size * 0.8,
            descent: size * 0.2,
            leading: size * 0.1,
            cap_height: size * 0.7,
            x_height: size * 0.5,
            em_size: size,
        }
    }
}
```

### 3.5 이미지 캐시

```rust
// render-web/src/image_cache.rs

use std::collections::HashMap;
use render_api::ImageId;
use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

/// 이미지 캐시
pub struct ImageCache {
    images: HashMap<ImageId, CachedImage>,
    next_id: u64,
}

struct CachedImage {
    element: HtmlImageElement,
    width: u32,
    height: u32,
    loaded: bool,
}

impl ImageCache {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            next_id: 0,
        }
    }

    /// 바이너리 데이터에서 이미지 로드
    pub fn load_from_bytes(&mut self, data: &[u8], mime_type: &str) -> ImageId {
        let id = ImageId(self.next_id);
        self.next_id += 1;

        // Base64 인코딩
        let base64 = base64_encode(data);
        let data_url = format!("data:{};base64,{}", mime_type, base64);

        // 이미지 요소 생성
        let element = HtmlImageElement::new().unwrap();
        element.set_src(&data_url);

        self.images.insert(id, CachedImage {
            element,
            width: 0,
            height: 0,
            loaded: false,
        });

        id
    }

    /// URL에서 이미지 로드
    pub fn load_from_url(&mut self, url: &str) -> ImageId {
        let id = ImageId(self.next_id);
        self.next_id += 1;

        let element = HtmlImageElement::new().unwrap();
        element.set_src(url);

        self.images.insert(id, CachedImage {
            element,
            width: 0,
            height: 0,
            loaded: false,
        });

        id
    }

    /// 이미지 요소 반환
    pub fn get(&self, id: ImageId) -> Option<&HtmlImageElement> {
        self.images.get(&id).map(|c| &c.element)
    }

    /// 이미지 로드 완료 여부
    pub fn is_loaded(&self, id: ImageId) -> bool {
        self.images.get(&id).map(|c| c.loaded).unwrap_or(false)
    }

    /// 이미지 제거
    pub fn remove(&mut self, id: ImageId) {
        self.images.remove(&id);
    }

    /// 캐시 클리어
    pub fn clear(&mut self) {
        self.images.clear();
    }
}

fn base64_encode(data: &[u8]) -> String {
    // 간단한 Base64 인코딩 (실제로는 base64 크레이트 사용)
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let mut i = 0;

    while i < data.len() {
        let b0 = data[i] as usize;
        let b1 = data.get(i + 1).copied().unwrap_or(0) as usize;
        let b2 = data.get(i + 2).copied().unwrap_or(0) as usize;

        result.push(ALPHABET[b0 >> 2] as char);
        result.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);

        if i + 1 < data.len() {
            result.push(ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }

        if i + 2 < data.len() {
            result.push(ALPHABET[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }

        i += 3;
    }

    result
}

impl Default for ImageCache {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## 4. 렌더링 명령 생성

레이아웃 결과에서 렌더링 명령을 생성하는 예시입니다.

```rust
// office-core에서 사용 예시

use render_api::*;
use layout::*;

pub fn render_document(
    engine: &LayoutEngine,
    doc: &Document,
    viewport: &Rect,
    selection: &Selection,
) -> RenderCommands {
    let mut commands = RenderCommands::new();

    // 뷰포트 클리핑
    commands.push(RenderCommand::PushClip {
        rect: viewport.clone(),
    });

    // 뷰포트 이동
    commands.push(RenderCommand::Save);
    commands.push(RenderCommand::Translate {
        x: -viewport.x,
        y: -viewport.y,
    });

    // 보이는 블록만 렌더링
    let visible = engine.visible_blocks(viewport);

    for block_idx in visible {
        let block_y = engine.block_y(block_idx);

        if let Some(layout) = engine.block_layout(block_idx) {
            commands.push(RenderCommand::Save);
            commands.push(RenderCommand::Translate { x: 0.0, y: block_y });

            match layout {
                BlockLayout::Paragraph(para) => {
                    render_paragraph(&mut commands, para, doc, block_idx, selection);
                }
                BlockLayout::Table(table) => {
                    render_table(&mut commands, table, doc);
                }
                BlockLayout::Image(image) => {
                    render_image(&mut commands, image);
                }
                BlockLayout::Shape(shape) => {
                    render_shape(&mut commands, shape);
                }
            }

            commands.push(RenderCommand::Restore);
        }
    }

    commands.push(RenderCommand::Restore);
    commands.push(RenderCommand::PopClip);

    commands
}

fn render_paragraph(
    commands: &mut RenderCommands,
    layout: &ParagraphLayout,
    doc: &Document,
    block_idx: usize,
    selection: &Selection,
) {
    for line in &layout.lines {
        // 선택 영역 하이라이트
        if selection_overlaps_line(selection, block_idx, &line.text_range) {
            let highlight_rect = calculate_selection_rect(selection, block_idx, line);
            commands.push(RenderCommand::FillRect {
                rect: highlight_rect,
                color: Color::rgba(0, 120, 215, 80),  // 선택 색상
            });
        }

        // 각 Run 렌더링
        for run in &line.runs {
            let style = doc.styles().get_char_style(run.style_id)
                .unwrap_or(&CharStyle::default_style());

            commands.push(RenderCommand::DrawText {
                method: TextRenderMethod::String(StringText {
                    text: run.text.clone(),
                    position: Point::new(
                        line.bounds.x + run.bounds.x,
                        line.baseline,
                    ),
                    style: TextStyle::from_char_style(style, doc.styles()),
                }),
            });
        }
    }
}

fn render_table(commands: &mut RenderCommands, layout: &TableLayout, doc: &Document) {
    // 표 테두리
    commands.push(RenderCommand::StrokeRect {
        rect: layout.bounds.clone(),
        color: Color::BLACK,
        width: 1.0,
    });

    // TODO: 셀 렌더링
}

fn render_image(commands: &mut RenderCommands, layout: &ImageLayout) {
    commands.push(RenderCommand::DrawImage {
        image_id: ImageId(layout.binary_id.0),
        rect: layout.bounds.clone(),
    });
}

fn render_shape(commands: &mut RenderCommands, layout: &ShapeLayout) {
    // TODO: 도형 렌더링
    commands.push(RenderCommand::StrokeRect {
        rect: layout.bounds.clone(),
        color: Color::rgb(128, 128, 128),
        width: 1.0,
    });
}

fn selection_overlaps_line(selection: &Selection, block_idx: usize, line_range: &Range<usize>) -> bool {
    if selection.is_collapsed() {
        return false;
    }

    let (start, end) = selection.ordered();

    if block_idx < start.block_index || block_idx > end.block_index {
        return false;
    }

    // 간단한 오버랩 체크
    true
}

fn calculate_selection_rect(selection: &Selection, block_idx: usize, line: &LineLayout) -> Rect {
    // 간단한 구현: 전체 줄 하이라이트
    Rect::new(
        line.bounds.x,
        line.bounds.y,
        line.bounds.width,
        line.bounds.height,
    )
}
```

---

## 5. 성능 최적화

### 5.1 배치 렌더링

동일 스타일의 텍스트를 묶어서 렌더링:

```rust
pub fn batch_text_commands(commands: &mut RenderCommands) {
    // TODO: 연속된 동일 스타일 텍스트 병합
}
```

### 5.2 더티 영역만 렌더링

```rust
pub fn render_dirty_region(
    commands: &mut RenderCommands,
    engine: &LayoutEngine,
    dirty_range: Range<usize>,
    viewport: &Rect,
) {
    // dirty 블록이 포함된 영역만 렌더링
}
```

---

## 변경 이력

| 날짜 | 버전 | 내용 |
|------|------|------|
| 2024-12 | 0.1 | 초안 작성 |
