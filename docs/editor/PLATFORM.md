# Platform Integration Specification

이 문서는 OpenHWP 에디터의 플랫폼 통합 계층을 정의합니다.

> **관련 문서**:
> - [DESIGN.md](./DESIGN.md) - 전체 아키텍처
> - [EDITOR.md](./EDITOR.md) - editor-core 명세
> - [RENDER.md](./RENDER.md) - 렌더링 명세

---

## 1. 개요

### 1.1 플랫폼 계층 구조

```
┌─────────────────────────────────────────────────────────────┐
│                    애플리케이션 계층                          │
│              (editor-web / editor-desktop)                  │
├─────────────────────────────────────────────────────────────┤
│                    플랫폼 추상화 계층                         │
│                    (platform-api)                           │
├─────────────────────────────────────────────────────────────┤
│                    에디터 코어 계층                           │
│                    (editor-core)                            │
└─────────────────────────────────────────────────────────────┘
```

### 1.2 크레이트 구조

```
crates/
├── platform-api/        # 플랫폼 추상화 트레잇
├── editor-web/          # 웹 플랫폼 구현 (WASM)
└── editor-desktop/      # 데스크톱 플랫폼 구현 (Tauri)
```

---

## 2. platform-api 크레이트

### 2.1 개요

플랫폼 독립적인 추상화 계층을 정의합니다. 모든 플랫폼별 구현은 이 트레잇을 구현합니다.

### 2.2 PlatformContext 트레잇

```rust
/// 플랫폼별 컨텍스트
///
/// 각 플랫폼(웹, 데스크톱)은 이 트레잇을 구현하여
/// 에디터 코어에 필요한 기능을 제공합니다.
pub trait PlatformContext {
    /// 클립보드 접근
    type Clipboard: ClipboardAccess;

    /// 파일 시스템 접근
    type FileSystem: FileSystemAccess;

    /// 타이머/스케줄러
    type Scheduler: Scheduler;

    /// 다이얼로그 (파일 열기/저장, 알림 등)
    type Dialog: DialogProvider;

    fn clipboard(&self) -> &Self::Clipboard;
    fn file_system(&self) -> &Self::FileSystem;
    fn scheduler(&self) -> &Self::Scheduler;
    fn dialog(&self) -> &Self::Dialog;

    /// 현재 시간 (밀리초)
    fn now_millis(&self) -> u64;

    /// 디바이스 픽셀 비율
    fn device_pixel_ratio(&self) -> f64;
}
```

### 2.3 ClipboardAccess 트레잇

```rust
use std::future::Future;

/// 클립보드 데이터 형식
#[derive(Debug, Clone)]
pub enum ClipboardFormat {
    /// 일반 텍스트
    PlainText,
    /// 서식 있는 텍스트 (HTML)
    RichText,
    /// OpenHWP 내부 포맷
    OpenHwpInternal,
    /// 이미지 (PNG)
    ImagePng,
    /// 이미지 (JPEG)
    ImageJpeg,
}

/// 클립보드 데이터
#[derive(Debug, Clone)]
pub struct ClipboardData {
    /// 일반 텍스트
    pub plain_text: Option<String>,
    /// HTML 형식 (서식 유지용)
    pub html: Option<String>,
    /// 내부 포맷 (완벽한 서식 보존)
    pub internal: Option<Vec<u8>>,
    /// 이미지 데이터
    pub image: Option<ImageData>,
}

#[derive(Debug, Clone)]
pub struct ImageData {
    pub format: ImageFormat,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    Png,
    Jpeg,
}

/// 클립보드 접근 인터페이스
///
/// 비동기 API로 정의 - 웹에서 Clipboard API가 Promise 기반이기 때문
pub trait ClipboardAccess {
    type ReadFuture: Future<Output = Result<ClipboardData, ClipboardError>>;
    type WriteFuture: Future<Output = Result<(), ClipboardError>>;

    /// 클립보드에서 읽기
    fn read(&self) -> Self::ReadFuture;

    /// 클립보드에 쓰기
    fn write(&self, data: ClipboardData) -> Self::WriteFuture;

    /// 지원 포맷 확인
    fn supports_format(&self, format: ClipboardFormat) -> bool;
}

#[derive(Debug, Clone)]
pub enum ClipboardError {
    /// 권한 없음
    PermissionDenied,
    /// 지원하지 않는 포맷
    UnsupportedFormat,
    /// 클립보드 비어 있음
    Empty,
    /// 기타 오류
    Other(String),
}
```

### 2.4 FileSystemAccess 트레잇

```rust
use std::future::Future;
use std::path::PathBuf;

/// 파일 필터
#[derive(Debug, Clone)]
pub struct FileFilter {
    /// 표시 이름 (예: "한글 문서")
    pub name: String,
    /// 확장자 목록 (예: ["hwp", "hwpx"])
    pub extensions: Vec<String>,
}

/// 파일 열기 옵션
#[derive(Debug, Clone, Default)]
pub struct OpenFileOptions {
    /// 파일 필터
    pub filters: Vec<FileFilter>,
    /// 다중 선택 허용
    pub multiple: bool,
    /// 시작 디렉토리 (선택사항)
    pub default_path: Option<PathBuf>,
}

/// 파일 저장 옵션
#[derive(Debug, Clone, Default)]
pub struct SaveFileOptions {
    /// 파일 필터
    pub filters: Vec<FileFilter>,
    /// 기본 파일명
    pub default_name: Option<String>,
    /// 시작 디렉토리
    pub default_path: Option<PathBuf>,
}

/// 선택된 파일 정보
#[derive(Debug, Clone)]
pub struct SelectedFile {
    /// 파일 이름 (경로 없음)
    pub name: String,
    /// 전체 경로 (데스크톱에서만 유효)
    pub path: Option<PathBuf>,
    /// 파일 내용
    pub content: Vec<u8>,
}

/// 파일 시스템 접근 인터페이스
pub trait FileSystemAccess {
    type OpenFuture: Future<Output = Result<Vec<SelectedFile>, FileSystemError>>;
    type SaveFuture: Future<Output = Result<(), FileSystemError>>;
    type ReadFuture: Future<Output = Result<Vec<u8>, FileSystemError>>;
    type WriteFuture: Future<Output = Result<(), FileSystemError>>;

    /// 파일 열기 다이얼로그
    fn open_file(&self, options: OpenFileOptions) -> Self::OpenFuture;

    /// 파일 저장 다이얼로그
    fn save_file(&self, options: SaveFileOptions, content: Vec<u8>) -> Self::SaveFuture;

    /// 경로로 파일 읽기 (데스크톱 전용)
    fn read_file(&self, path: &PathBuf) -> Self::ReadFuture;

    /// 경로로 파일 쓰기 (데스크톱 전용)
    fn write_file(&self, path: &PathBuf, content: Vec<u8>) -> Self::WriteFuture;

    /// 최근 파일 목록 (데스크톱 전용)
    fn recent_files(&self) -> Vec<PathBuf>;

    /// 최근 파일 추가
    fn add_recent_file(&self, path: PathBuf);
}

#[derive(Debug, Clone)]
pub enum FileSystemError {
    /// 사용자가 취소함
    Cancelled,
    /// 파일을 찾을 수 없음
    NotFound,
    /// 권한 없음
    PermissionDenied,
    /// 읽기 오류
    ReadError(String),
    /// 쓰기 오류
    WriteError(String),
}
```

### 2.5 Scheduler 트레잇

```rust
/// 스케줄러 인터페이스
///
/// 타이머, 애니메이션 프레임, 지연 실행 등을 처리합니다.
pub trait Scheduler {
    /// 타이머 ID
    type TimerId: Copy + Eq;

    /// 지연 실행
    ///
    /// `delay_ms` 후에 콜백을 실행합니다.
    fn set_timeout<F>(&self, callback: F, delay_ms: u32) -> Self::TimerId
    where
        F: FnOnce() + 'static;

    /// 타이머 취소
    fn clear_timeout(&self, id: Self::TimerId);

    /// 반복 실행
    fn set_interval<F>(&self, callback: F, interval_ms: u32) -> Self::TimerId
    where
        F: FnMut() + 'static;

    /// 인터벌 취소
    fn clear_interval(&self, id: Self::TimerId);

    /// 다음 프레임에 실행 (애니메이션용)
    ///
    /// 웹: requestAnimationFrame
    /// 데스크톱: 16ms 타이머 또는 vsync
    fn request_animation_frame<F>(&self, callback: F)
    where
        F: FnOnce(f64) + 'static; // f64는 타임스탬프

    /// 유휴 시간에 실행 (낮은 우선순위 작업)
    ///
    /// 웹: requestIdleCallback
    /// 데스크톱: 낮은 우선순위 큐
    fn request_idle_callback<F>(&self, callback: F)
    where
        F: FnOnce() + 'static;
}
```

### 2.6 DialogProvider 트레잇

```rust
use std::future::Future;

/// 메시지 박스 버튼
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoxButtons {
    Ok,
    OkCancel,
    YesNo,
    YesNoCancel,
}

/// 메시지 박스 결과
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoxResult {
    Ok,
    Cancel,
    Yes,
    No,
}

/// 메시지 박스 아이콘
#[derive(Debug, Clone, Copy)]
pub enum MessageBoxIcon {
    None,
    Info,
    Warning,
    Error,
    Question,
}

/// 메시지 박스 옵션
#[derive(Debug, Clone)]
pub struct MessageBoxOptions {
    pub title: String,
    pub message: String,
    pub buttons: MessageBoxButtons,
    pub icon: MessageBoxIcon,
}

/// 다이얼로그 제공자 인터페이스
pub trait DialogProvider {
    type MessageFuture: Future<Output = MessageBoxResult>;
    type ConfirmFuture: Future<Output = bool>;

    /// 메시지 박스 표시
    fn show_message(&self, options: MessageBoxOptions) -> Self::MessageFuture;

    /// 확인 다이얼로그 (간단한 Yes/No)
    fn confirm(&self, title: &str, message: &str) -> Self::ConfirmFuture;

    /// 알림 표시 (비차단)
    fn notify(&self, title: &str, message: &str);
}
```

---

## 3. editor-web 크레이트

### 3.1 개요

웹 플랫폼(WASM)을 위한 에디터 구현입니다. Canvas 2D를 사용하여 렌더링하고,
DOM 이벤트를 에디터 이벤트로 변환합니다.

### 3.2 의존성

```toml
[package]
name = "editor-web"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
editor-core = { path = "../editor-core" }
render-web = { path = "../render-web" }
platform-api = { path = "../platform-api" }
document = { path = "../document" }
layout = { path = "../layout" }

wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
js-sys = "0.3"
web-sys = { version = "0.3", features = [
    "Window",
    "Document",
    "Element",
    "HtmlElement",
    "HtmlCanvasElement",
    "HtmlInputElement",
    "CanvasRenderingContext2d",
    "MouseEvent",
    "WheelEvent",
    "KeyboardEvent",
    "CompositionEvent",
    "ClipboardEvent",
    "FocusEvent",
    "InputEvent",
    "DragEvent",
    "DataTransfer",
    "Clipboard",
    "Navigator",
    "Performance",
    "ResizeObserver",
    "ResizeObserverEntry",
    "DomRectReadOnly",
    "CssStyleDeclaration",
    "EventTarget",
    "AddEventListenerOptions",
] }

serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
console_error_panic_hook = "0.1"
```

### 3.3 WebEditorApp 구조체

```rust
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, HtmlInputElement};
use std::rc::Rc;
use std::cell::RefCell;

use editor_core::{EditorCore, InputEvent, UpdateResult};
use render_web::CanvasRenderer;
use document::Document;

/// 웹 에디터 애플리케이션
///
/// JavaScript에서 생성하고 사용합니다.
#[wasm_bindgen]
pub struct WebEditorApp {
    /// 에디터 코어 (내부 가변성)
    core: Rc<RefCell<EditorCore>>,

    /// Canvas 렌더러
    renderer: Rc<RefCell<CanvasRenderer>>,

    /// 메인 Canvas 요소
    canvas: HtmlCanvasElement,

    /// 숨겨진 입력 요소 (IME용)
    hidden_input: HtmlInputElement,

    /// 플랫폼 컨텍스트
    platform: WebPlatformContext,

    /// 이벤트 리스너 클로저들 (Drop 시 정리용)
    listeners: Vec<EventListenerHandle>,

    /// 현재 IME 조합 중 여부
    is_composing: bool,
}

#[wasm_bindgen]
impl WebEditorApp {
    /// 새 에디터 인스턴스 생성
    ///
    /// # Arguments
    /// * `container_id` - 에디터를 마운트할 컨테이너 요소의 ID
    ///
    /// # Returns
    /// 에디터 인스턴스 또는 오류 메시지
    #[wasm_bindgen(constructor)]
    pub fn new(container_id: &str) -> Result<WebEditorApp, JsValue> {
        // 패닉 훅 설치 (디버깅용)
        console_error_panic_hook::set_once();

        // 컨테이너 찾기
        let window = web_sys::window()
            .ok_or_else(|| JsValue::from_str("window not found"))?;
        let document = window.document()
            .ok_or_else(|| JsValue::from_str("document not found"))?;
        let container = document.get_element_by_id(container_id)
            .ok_or_else(|| JsValue::from_str("container not found"))?;

        // Canvas 생성
        let canvas = create_canvas(&document, &container)?;

        // 숨겨진 입력 요소 생성 (IME용)
        let hidden_input = create_hidden_input(&document, &container)?;

        // 렌더러 초기화
        let renderer = CanvasRenderer::new(&canvas)?;

        // 에디터 코어 초기화
        let doc = Document::new();
        let core = EditorCore::new(doc);

        let mut app = WebEditorApp {
            core: Rc::new(RefCell::new(core)),
            renderer: Rc::new(RefCell::new(renderer)),
            canvas,
            hidden_input,
            platform: WebPlatformContext::new(),
            listeners: Vec::new(),
            is_composing: false,
        };

        // 이벤트 리스너 설정
        app.setup_event_listeners()?;

        // 초기 렌더링
        app.render()?;

        Ok(app)
    }

    /// 문서 로드 (HWP/HWPX 바이트 배열)
    #[wasm_bindgen]
    pub fn load_document(&mut self, bytes: &[u8], filename: &str) -> Result<(), JsValue> {
        let doc = load_document_from_bytes(bytes, filename)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.core.borrow_mut().set_document(doc);
        self.render()?;

        Ok(())
    }

    /// 문서 저장 (HWPX 바이트 배열 반환)
    #[wasm_bindgen]
    pub fn save_document(&self) -> Result<Vec<u8>, JsValue> {
        let doc = self.core.borrow().document();
        save_document_to_bytes(&doc)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// 수동 렌더링 트리거
    #[wasm_bindgen]
    pub fn render(&self) -> Result<(), JsValue> {
        let core = self.core.borrow();
        let commands = core.generate_render_commands();

        let mut renderer = self.renderer.borrow_mut();
        renderer.render(&commands)?;

        Ok(())
    }

    /// Canvas 크기 업데이트
    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        let dpr = self.platform.device_pixel_ratio();

        // Canvas 크기 설정
        self.canvas.set_width((width as f64 * dpr) as u32);
        self.canvas.set_height((height as f64 * dpr) as u32);

        // CSS 크기 설정
        let style = self.canvas.style();
        style.set_property("width", &format!("{}px", width))?;
        style.set_property("height", &format!("{}px", height))?;

        // 에디터 코어에 뷰포트 크기 알림
        self.core.borrow_mut().set_viewport_size(width as f64, height as f64);

        // 렌더러 컨텍스트 스케일 재설정
        self.renderer.borrow_mut().set_scale(dpr)?;

        self.render()?;

        Ok(())
    }

    /// 포커스 설정
    #[wasm_bindgen]
    pub fn focus(&self) {
        let _ = self.hidden_input.focus();
    }

    /// 포커스 해제
    #[wasm_bindgen]
    pub fn blur(&self) {
        let _ = self.hidden_input.blur();
    }

    /// 리소스 정리
    #[wasm_bindgen]
    pub fn destroy(&mut self) {
        // 이벤트 리스너 제거
        for listener in self.listeners.drain(..) {
            listener.remove();
        }

        // DOM 요소 제거
        if let Some(parent) = self.canvas.parent_element() {
            let _ = parent.remove_child(&self.canvas);
            let _ = parent.remove_child(&self.hidden_input);
        }
    }
}
```

### 3.4 이벤트 핸들링

```rust
impl WebEditorApp {
    /// 이벤트 리스너 설정
    fn setup_event_listeners(&mut self) -> Result<(), JsValue> {
        self.setup_mouse_listeners()?;
        self.setup_keyboard_listeners()?;
        self.setup_composition_listeners()?;
        self.setup_clipboard_listeners()?;
        self.setup_focus_listeners()?;
        self.setup_resize_observer()?;

        Ok(())
    }

    /// 마우스 이벤트 리스너
    fn setup_mouse_listeners(&mut self) -> Result<(), JsValue> {
        let core = Rc::clone(&self.core);
        let renderer = Rc::clone(&self.renderer);
        let canvas = self.canvas.clone();

        // mousedown
        let onmousedown = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {
            e.prevent_default();

            let (x, y) = get_canvas_coords(&canvas, &e);
            let button = match e.button() {
                0 => MouseButton::Left,
                1 => MouseButton::Middle,
                2 => MouseButton::Right,
                _ => return,
            };

            let event = InputEvent::MouseDown {
                x,
                y,
                button,
                modifiers: get_modifiers_from_mouse(&e),
            };

            let result = core.borrow_mut().handle_event(event);
            handle_update_result(&result, &renderer, &canvas);
        });

        self.canvas.add_event_listener_with_callback(
            "mousedown",
            onmousedown.as_ref().unchecked_ref(),
        )?;

        self.listeners.push(EventListenerHandle::new(
            self.canvas.clone().into(),
            "mousedown",
            onmousedown,
        ));

        // mousemove, mouseup, wheel 등 유사하게 구현...

        Ok(())
    }

    /// 키보드 이벤트 리스너
    fn setup_keyboard_listeners(&mut self) -> Result<(), JsValue> {
        let core = Rc::clone(&self.core);
        let renderer = Rc::clone(&self.renderer);
        let canvas = self.canvas.clone();
        let hidden_input = self.hidden_input.clone();

        // keydown
        let onkeydown = Closure::<dyn FnMut(_)>::new(move |e: web_sys::KeyboardEvent| {
            // IME 조합 중에는 일부 키 이벤트 무시
            if e.is_composing() {
                return;
            }

            let key = parse_key_code(&e);
            let modifiers = get_modifiers_from_keyboard(&e);

            // 특수 키 처리 (방향키, 백스페이스 등)
            if should_handle_key(&key) {
                e.prevent_default();

                let event = InputEvent::KeyDown {
                    key,
                    modifiers,
                };

                let result = core.borrow_mut().handle_event(event);
                handle_update_result(&result, &renderer, &canvas);
            }
        });

        self.hidden_input.add_event_listener_with_callback(
            "keydown",
            onkeydown.as_ref().unchecked_ref(),
        )?;

        self.listeners.push(EventListenerHandle::new(
            self.hidden_input.clone().into(),
            "keydown",
            onkeydown,
        ));

        Ok(())
    }

    /// IME 조합 이벤트 리스너
    fn setup_composition_listeners(&mut self) -> Result<(), JsValue> {
        let core = Rc::clone(&self.core);
        let renderer = Rc::clone(&self.renderer);
        let canvas = self.canvas.clone();
        let hidden_input = self.hidden_input.clone();

        // compositionstart
        let oncompositionstart = Closure::<dyn FnMut(_)>::new(move |e: web_sys::CompositionEvent| {
            let event = InputEvent::CompositionStart;
            let result = core.borrow_mut().handle_event(event);
            handle_update_result(&result, &renderer, &canvas);
        });

        // compositionupdate
        let core = Rc::clone(&self.core);
        let renderer = Rc::clone(&self.renderer);
        let canvas = self.canvas.clone();

        let oncompositionupdate = Closure::<dyn FnMut(_)>::new(move |e: web_sys::CompositionEvent| {
            if let Some(data) = e.data() {
                let event = InputEvent::CompositionUpdate { text: data };
                let result = core.borrow_mut().handle_event(event);
                handle_update_result(&result, &renderer, &canvas);
            }
        });

        // compositionend
        let core = Rc::clone(&self.core);
        let renderer = Rc::clone(&self.renderer);
        let canvas = self.canvas.clone();

        let oncompositionend = Closure::<dyn FnMut(_)>::new(move |e: web_sys::CompositionEvent| {
            if let Some(data) = e.data() {
                let event = InputEvent::CompositionEnd { text: data };
                let result = core.borrow_mut().handle_event(event);
                handle_update_result(&result, &renderer, &canvas);

                // 숨겨진 입력 필드 초기화
                hidden_input.set_value("");
            }
        });

        // 리스너 등록...

        Ok(())
    }

    /// 클립보드 이벤트 리스너
    fn setup_clipboard_listeners(&mut self) -> Result<(), JsValue> {
        let core = Rc::clone(&self.core);
        let renderer = Rc::clone(&self.renderer);
        let canvas = self.canvas.clone();

        // copy
        let oncopy = Closure::<dyn FnMut(_)>::new(move |e: web_sys::ClipboardEvent| {
            e.prevent_default();

            let core_ref = core.borrow();
            if let Some(clipboard_data) = core_ref.get_selection_as_clipboard_data() {
                if let Some(dt) = e.clipboard_data() {
                    // 텍스트 설정
                    if let Some(text) = &clipboard_data.plain_text {
                        let _ = dt.set_data("text/plain", text);
                    }
                    // HTML 설정
                    if let Some(html) = &clipboard_data.html {
                        let _ = dt.set_data("text/html", html);
                    }
                    // 내부 포맷 (base64 인코딩)
                    if let Some(internal) = &clipboard_data.internal {
                        let encoded = base64_encode(internal);
                        let _ = dt.set_data("application/x-openhwp", &encoded);
                    }
                }
            }
        });

        // paste
        let core = Rc::clone(&self.core);
        let renderer = Rc::clone(&self.renderer);
        let canvas = self.canvas.clone();

        let onpaste = Closure::<dyn FnMut(_)>::new(move |e: web_sys::ClipboardEvent| {
            e.prevent_default();

            if let Some(dt) = e.clipboard_data() {
                let clipboard_data = extract_clipboard_data(&dt);
                let event = InputEvent::Paste { data: clipboard_data };
                let result = core.borrow_mut().handle_event(event);
                handle_update_result(&result, &renderer, &canvas);
            }
        });

        // cut 유사하게 구현...

        Ok(())
    }
}
```

### 3.5 헬퍼 함수

```rust
/// Canvas 생성
fn create_canvas(
    document: &web_sys::Document,
    container: &web_sys::Element,
) -> Result<HtmlCanvasElement, JsValue> {
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    canvas.set_attribute("tabindex", "0")?;
    canvas.style().set_property("outline", "none")?;
    canvas.style().set_property("touch-action", "none")?;

    container.append_child(&canvas)?;

    Ok(canvas)
}

/// 숨겨진 입력 요소 생성 (IME용)
fn create_hidden_input(
    document: &web_sys::Document,
    container: &web_sys::Element,
) -> Result<HtmlInputElement, JsValue> {
    let input = document
        .create_element("input")?
        .dyn_into::<HtmlInputElement>()?;

    // 화면 밖에 위치시키되 접근성 유지
    let style = input.style();
    style.set_property("position", "absolute")?;
    style.set_property("left", "-9999px")?;
    style.set_property("top", "0")?;
    style.set_property("width", "1px")?;
    style.set_property("height", "1px")?;
    style.set_property("opacity", "0")?;
    style.set_property("pointer-events", "none")?;

    // IME 힌트
    input.set_attribute("autocomplete", "off")?;
    input.set_attribute("autocorrect", "off")?;
    input.set_attribute("autocapitalize", "off")?;
    input.set_attribute("spellcheck", "false")?;

    container.append_child(&input)?;

    Ok(input)
}

/// Canvas 좌표 계산
fn get_canvas_coords(canvas: &HtmlCanvasElement, e: &web_sys::MouseEvent) -> (f64, f64) {
    let rect = canvas.get_bounding_client_rect();
    let x = e.client_x() as f64 - rect.left();
    let y = e.client_y() as f64 - rect.top();
    (x, y)
}

/// 키보드 이벤트에서 수정자 키 추출
fn get_modifiers_from_keyboard(e: &web_sys::KeyboardEvent) -> Modifiers {
    Modifiers {
        ctrl: e.ctrl_key() || e.meta_key(), // macOS Command 지원
        shift: e.shift_key(),
        alt: e.alt_key(),
    }
}

/// 마우스 이벤트에서 수정자 키 추출
fn get_modifiers_from_mouse(e: &web_sys::MouseEvent) -> Modifiers {
    Modifiers {
        ctrl: e.ctrl_key() || e.meta_key(),
        shift: e.shift_key(),
        alt: e.alt_key(),
    }
}

/// 키 코드 파싱
fn parse_key_code(e: &web_sys::KeyboardEvent) -> Key {
    match e.key().as_str() {
        "Backspace" => Key::Backspace,
        "Delete" => Key::Delete,
        "Enter" => Key::Enter,
        "Tab" => Key::Tab,
        "Escape" => Key::Escape,
        "ArrowLeft" => Key::ArrowLeft,
        "ArrowRight" => Key::ArrowRight,
        "ArrowUp" => Key::ArrowUp,
        "ArrowDown" => Key::ArrowDown,
        "Home" => Key::Home,
        "End" => Key::End,
        "PageUp" => Key::PageUp,
        "PageDown" => Key::PageDown,
        "a" | "A" => Key::Char('a'),
        "c" | "C" => Key::Char('c'),
        "v" | "V" => Key::Char('v'),
        "x" | "X" => Key::Char('x'),
        "z" | "Z" => Key::Char('z'),
        "y" | "Y" => Key::Char('y'),
        s if s.len() == 1 => Key::Char(s.chars().next().unwrap()),
        _ => Key::Unknown,
    }
}

/// 업데이트 결과 처리
fn handle_update_result(
    result: &UpdateResult,
    renderer: &Rc<RefCell<CanvasRenderer>>,
    canvas: &HtmlCanvasElement,
) {
    if result.needs_render {
        // 다음 프레임에 렌더링 예약
        request_animation_frame(|| {
            // 렌더링 로직...
        });
    }

    if let Some(cursor) = &result.cursor_style {
        canvas.style().set_property("cursor", cursor).ok();
    }
}
```

### 3.6 WebPlatformContext 구현

```rust
/// 웹 플랫폼 컨텍스트
pub struct WebPlatformContext {
    window: web_sys::Window,
}

impl WebPlatformContext {
    pub fn new() -> Self {
        let window = web_sys::window().expect("window should exist");
        Self { window }
    }
}

impl PlatformContext for WebPlatformContext {
    type Clipboard = WebClipboard;
    type FileSystem = WebFileSystem;
    type Scheduler = WebScheduler;
    type Dialog = WebDialog;

    fn clipboard(&self) -> &Self::Clipboard {
        &WebClipboard
    }

    fn file_system(&self) -> &Self::FileSystem {
        &WebFileSystem
    }

    fn scheduler(&self) -> &Self::Scheduler {
        &WebScheduler
    }

    fn dialog(&self) -> &Self::Dialog {
        &WebDialog
    }

    fn now_millis(&self) -> u64 {
        self.window.performance()
            .map(|p| p.now() as u64)
            .unwrap_or(0)
    }

    fn device_pixel_ratio(&self) -> f64 {
        self.window.device_pixel_ratio()
    }
}

/// 웹 클립보드 구현
pub struct WebClipboard;

impl ClipboardAccess for WebClipboard {
    type ReadFuture = impl Future<Output = Result<ClipboardData, ClipboardError>>;
    type WriteFuture = impl Future<Output = Result<(), ClipboardError>>;

    fn read(&self) -> Self::ReadFuture {
        async {
            let window = web_sys::window().ok_or(ClipboardError::Other("no window".into()))?;
            let navigator = window.navigator();
            let clipboard = navigator.clipboard()
                .ok_or(ClipboardError::PermissionDenied)?;

            let promise = clipboard.read_text();
            let result = wasm_bindgen_futures::JsFuture::from(promise).await
                .map_err(|_| ClipboardError::PermissionDenied)?;

            let text = result.as_string();

            Ok(ClipboardData {
                plain_text: text,
                html: None,
                internal: None,
                image: None,
            })
        }
    }

    fn write(&self, data: ClipboardData) -> Self::WriteFuture {
        async move {
            let window = web_sys::window().ok_or(ClipboardError::Other("no window".into()))?;
            let navigator = window.navigator();
            let clipboard = navigator.clipboard()
                .ok_or(ClipboardError::PermissionDenied)?;

            if let Some(text) = data.plain_text {
                let promise = clipboard.write_text(&text);
                wasm_bindgen_futures::JsFuture::from(promise).await
                    .map_err(|_| ClipboardError::PermissionDenied)?;
            }

            Ok(())
        }
    }

    fn supports_format(&self, format: ClipboardFormat) -> bool {
        matches!(format, ClipboardFormat::PlainText | ClipboardFormat::RichText)
    }
}

/// 웹 파일 시스템 구현
pub struct WebFileSystem;

impl FileSystemAccess for WebFileSystem {
    // File System Access API 또는 폴백으로 input[type=file] 사용
    // ...
}

/// 웹 스케줄러 구현
pub struct WebScheduler;

impl Scheduler for WebScheduler {
    type TimerId = i32;

    fn set_timeout<F>(&self, callback: F, delay_ms: u32) -> Self::TimerId
    where
        F: FnOnce() + 'static,
    {
        let window = web_sys::window().unwrap();
        let closure = Closure::once(callback);
        let id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                delay_ms as i32,
            )
            .unwrap();
        closure.forget();
        id
    }

    fn clear_timeout(&self, id: Self::TimerId) {
        let window = web_sys::window().unwrap();
        window.clear_timeout_with_handle(id);
    }

    fn set_interval<F>(&self, callback: F, interval_ms: u32) -> Self::TimerId
    where
        F: FnMut() + 'static,
    {
        let window = web_sys::window().unwrap();
        let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);
        let id = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                interval_ms as i32,
            )
            .unwrap();
        closure.forget();
        id
    }

    fn clear_interval(&self, id: Self::TimerId) {
        let window = web_sys::window().unwrap();
        window.clear_interval_with_handle(id);
    }

    fn request_animation_frame<F>(&self, callback: F)
    where
        F: FnOnce(f64) + 'static,
    {
        let window = web_sys::window().unwrap();
        let closure = Closure::once(callback);
        window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    fn request_idle_callback<F>(&self, callback: F)
    where
        F: FnOnce() + 'static,
    {
        // requestIdleCallback 지원 확인 후 사용
        // 미지원 시 setTimeout(callback, 0) 폴백
        let window = web_sys::window().unwrap();
        let closure = Closure::once(move |_: JsValue| callback());
        // window.request_idle_callback(closure.as_ref().unchecked_ref()).unwrap();
        // 폴백:
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                0,
            )
            .unwrap();
        closure.forget();
    }
}
```

### 3.7 JavaScript 바인딩 API

```rust
/// JavaScript에서 사용하는 API
#[wasm_bindgen]
impl WebEditorApp {
    /// 텍스트 삽입
    #[wasm_bindgen(js_name = "insertText")]
    pub fn insert_text(&mut self, text: &str) {
        let event = InputEvent::TextInput { text: text.to_string() };
        let result = self.core.borrow_mut().handle_event(event);
        if result.needs_render {
            let _ = self.render();
        }
    }

    /// 선택 영역 텍스트 가져오기
    #[wasm_bindgen(js_name = "getSelectedText")]
    pub fn get_selected_text(&self) -> Option<String> {
        self.core.borrow().get_selected_text()
    }

    /// 실행 취소
    #[wasm_bindgen]
    pub fn undo(&mut self) -> bool {
        let result = self.core.borrow_mut().undo();
        if result {
            let _ = self.render();
        }
        result
    }

    /// 다시 실행
    #[wasm_bindgen]
    pub fn redo(&mut self) -> bool {
        let result = self.core.borrow_mut().redo();
        if result {
            let _ = self.render();
        }
        result
    }

    /// 전체 선택
    #[wasm_bindgen(js_name = "selectAll")]
    pub fn select_all(&mut self) {
        self.core.borrow_mut().select_all();
        let _ = self.render();
    }

    /// 현재 스타일 가져오기 (JSON)
    #[wasm_bindgen(js_name = "getCurrentStyle")]
    pub fn get_current_style(&self) -> JsValue {
        let style = self.core.borrow().get_current_style();
        serde_wasm_bindgen::to_value(&style).unwrap_or(JsValue::NULL)
    }

    /// 스타일 적용 (JSON)
    #[wasm_bindgen(js_name = "applyStyle")]
    pub fn apply_style(&mut self, style_json: &str) -> Result<(), JsValue> {
        let style: CharStyle = serde_json::from_str(style_json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.core.borrow_mut().apply_style(style);
        let _ = self.render();

        Ok(())
    }

    /// 문서 상태 가져오기 (수정됨 여부)
    #[wasm_bindgen(js_name = "isModified")]
    pub fn is_modified(&self) -> bool {
        self.core.borrow().is_modified()
    }

    /// 페이지 수 가져오기
    #[wasm_bindgen(js_name = "getPageCount")]
    pub fn get_page_count(&self) -> u32 {
        self.core.borrow().page_count()
    }

    /// 특정 페이지로 이동
    #[wasm_bindgen(js_name = "goToPage")]
    pub fn go_to_page(&mut self, page: u32) {
        self.core.borrow_mut().go_to_page(page);
        let _ = self.render();
    }

    /// 확대/축소 설정
    #[wasm_bindgen(js_name = "setZoom")]
    pub fn set_zoom(&mut self, zoom: f64) {
        self.core.borrow_mut().set_zoom(zoom);
        let _ = self.render();
    }

    /// 확대/축소 가져오기
    #[wasm_bindgen(js_name = "getZoom")]
    pub fn get_zoom(&self) -> f64 {
        self.core.borrow().zoom()
    }
}
```

### 3.8 TypeScript 타입 정의

```typescript
// editor-web/pkg/editor_web.d.ts (자동 생성 + 수동 보강)

export class WebEditorApp {
    constructor(containerId: string);

    loadDocument(bytes: Uint8Array, filename: string): void;
    saveDocument(): Uint8Array;

    render(): void;
    resize(width: number, height: number): void;

    focus(): void;
    blur(): void;
    destroy(): void;

    insertText(text: string): void;
    getSelectedText(): string | null;

    undo(): boolean;
    redo(): boolean;
    selectAll(): void;

    getCurrentStyle(): CharStyle | null;
    applyStyle(styleJson: string): void;

    isModified(): boolean;
    getPageCount(): number;
    goToPage(page: number): void;

    setZoom(zoom: number): void;
    getZoom(): number;
}

export interface CharStyle {
    fontFamily?: string;
    fontSize?: number;
    bold?: boolean;
    italic?: boolean;
    underline?: boolean;
    strikethrough?: boolean;
    color?: string;
    backgroundColor?: string;
}

export interface ParaStyle {
    alignment?: 'left' | 'center' | 'right' | 'justify';
    lineHeight?: number;
    indentFirst?: number;
    indentLeft?: number;
    indentRight?: number;
    spaceBefore?: number;
    spaceAfter?: number;
}
```

---

## 4. editor-desktop 크레이트

### 4.1 개요

Tauri를 사용한 데스크톱 애플리케이션입니다.
웹 에디터를 WebView로 래핑하고, 네이티브 기능을 추가합니다.

### 4.2 프로젝트 구조

```
editor-desktop/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs      # Tauri 커맨드
│   │   ├── menu.rs          # 메뉴 바
│   │   └── file_handler.rs  # 파일 처리
│   └── icons/
└── src/                     # 프론트엔드 (웹 에디터 사용)
    ├── index.html
    ├── main.ts
    └── style.css
```

### 4.3 Tauri 설정

```json
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:3000",
    "distDir": "../dist"
  },
  "package": {
    "productName": "OpenHWP Editor",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": true
      },
      "dialog": {
        "all": true,
        "open": true,
        "save": true,
        "message": true,
        "ask": true,
        "confirm": true
      },
      "fs": {
        "all": true,
        "readFile": true,
        "writeFile": true,
        "readDir": true,
        "createDir": true,
        "removeDir": true,
        "removeFile": true,
        "renameFile": true,
        "exists": true
      },
      "path": {
        "all": true
      },
      "clipboard": {
        "all": true,
        "writeText": true,
        "readText": true
      },
      "window": {
        "all": true,
        "setTitle": true,
        "close": true,
        "minimize": true,
        "maximize": true
      }
    },
    "bundle": {
      "active": true,
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "identifier": "com.openhwp.editor",
      "targets": ["app", "dmg", "msi", "deb"],
      "macOS": {
        "minimumSystemVersion": "10.13"
      },
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": ""
      },
      "fileAssociations": [
        {
          "ext": ["hwp"],
          "mimeType": "application/x-hwp",
          "description": "HWP Document",
          "role": "Editor"
        },
        {
          "ext": ["hwpx"],
          "mimeType": "application/hwpx+zip",
          "description": "HWPX Document",
          "role": "Editor"
        }
      ]
    },
    "security": {
      "csp": "default-src 'self'; style-src 'self' 'unsafe-inline'"
    },
    "windows": [
      {
        "fullscreen": false,
        "height": 800,
        "width": 1200,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true,
        "title": "OpenHWP Editor",
        "center": true
      }
    ]
  }
}
```

### 4.4 Tauri 커맨드

```rust
// src-tauri/src/commands.rs

use tauri::{command, State, Window};
use std::path::PathBuf;
use std::sync::Mutex;

use document::Document;
use hwp::HwpDocument;
use hwpx;

/// 애플리케이션 상태
pub struct AppState {
    /// 현재 열린 파일 경로
    pub current_file: Mutex<Option<PathBuf>>,
    /// 최근 파일 목록
    pub recent_files: Mutex<Vec<PathBuf>>,
    /// 문서 수정 여부
    pub is_modified: Mutex<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_file: Mutex::new(None),
            recent_files: Mutex::new(Vec::new()),
            is_modified: Mutex::new(false),
        }
    }
}

/// 파일 열기
#[command]
pub async fn open_file(
    path: String,
    state: State<'_, AppState>,
    window: Window,
) -> Result<Vec<u8>, String> {
    let path = PathBuf::from(&path);

    // 파일 읽기
    let bytes = std::fs::read(&path)
        .map_err(|e| format!("파일을 읽을 수 없습니다: {}", e))?;

    // 상태 업데이트
    *state.current_file.lock().unwrap() = Some(path.clone());
    *state.is_modified.lock().unwrap() = false;

    // 최근 파일에 추가
    add_recent_file(&state, path.clone());

    // 창 제목 업데이트
    let title = format!(
        "{} - OpenHWP Editor",
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
    );
    window.set_title(&title).ok();

    Ok(bytes)
}

/// 파일 저장
#[command]
pub async fn save_file(
    path: Option<String>,
    content: Vec<u8>,
    state: State<'_, AppState>,
    window: Window,
) -> Result<String, String> {
    let path = match path {
        Some(p) => PathBuf::from(p),
        None => {
            // 현재 파일 경로 사용
            state.current_file.lock().unwrap()
                .clone()
                .ok_or_else(|| "저장할 경로를 지정해주세요".to_string())?
        }
    };

    // 파일 쓰기
    std::fs::write(&path, &content)
        .map_err(|e| format!("파일을 저장할 수 없습니다: {}", e))?;

    // 상태 업데이트
    *state.current_file.lock().unwrap() = Some(path.clone());
    *state.is_modified.lock().unwrap() = false;

    // 창 제목 업데이트
    let title = format!(
        "{} - OpenHWP Editor",
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
    );
    window.set_title(&title).ok();

    Ok(path.to_string_lossy().to_string())
}

/// 파일 저장 다이얼로그
#[command]
pub async fn save_file_dialog(
    default_name: Option<String>,
) -> Result<Option<String>, String> {
    use tauri::api::dialog::FileDialogBuilder;

    let (tx, rx) = std::sync::mpsc::channel();

    FileDialogBuilder::new()
        .set_file_name(&default_name.unwrap_or_else(|| "새 문서.hwpx".to_string()))
        .add_filter("HWPX 문서", &["hwpx"])
        .add_filter("HWP 문서", &["hwp"])
        .add_filter("모든 파일", &["*"])
        .save_file(move |path| {
            tx.send(path).ok();
        });

    let path = rx.recv()
        .map_err(|_| "다이얼로그가 취소되었습니다")?;

    Ok(path.map(|p| p.to_string_lossy().to_string()))
}

/// 파일 열기 다이얼로그
#[command]
pub async fn open_file_dialog() -> Result<Option<String>, String> {
    use tauri::api::dialog::FileDialogBuilder;

    let (tx, rx) = std::sync::mpsc::channel();

    FileDialogBuilder::new()
        .add_filter("한글 문서", &["hwp", "hwpx"])
        .add_filter("HWP 문서", &["hwp"])
        .add_filter("HWPX 문서", &["hwpx"])
        .add_filter("모든 파일", &["*"])
        .pick_file(move |path| {
            tx.send(path).ok();
        });

    let path = rx.recv()
        .map_err(|_| "다이얼로그가 취소되었습니다")?;

    Ok(path.map(|p| p.to_string_lossy().to_string()))
}

/// 최근 파일 목록 가져오기
#[command]
pub fn get_recent_files(state: State<'_, AppState>) -> Vec<String> {
    state.recent_files.lock().unwrap()
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect()
}

/// 문서 수정됨 표시
#[command]
pub fn set_modified(
    modified: bool,
    state: State<'_, AppState>,
    window: Window,
) {
    *state.is_modified.lock().unwrap() = modified;

    // 창 제목에 수정됨 표시
    let current_file = state.current_file.lock().unwrap();
    let filename = current_file
        .as_ref()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("새 문서");

    let title = if modified {
        format!("*{} - OpenHWP Editor", filename)
    } else {
        format!("{} - OpenHWP Editor", filename)
    };

    window.set_title(&title).ok();
}

/// 최근 파일에 추가
fn add_recent_file(state: &State<'_, AppState>, path: PathBuf) {
    let mut recent = state.recent_files.lock().unwrap();

    // 이미 있으면 제거
    recent.retain(|p| p != &path);

    // 맨 앞에 추가
    recent.insert(0, path);

    // 최대 10개 유지
    recent.truncate(10);
}

/// 인쇄 미리보기
#[command]
pub async fn print_preview(window: Window) -> Result<(), String> {
    // 인쇄 미리보기 창 열기
    // TODO: 구현
    Ok(())
}

/// PDF로 내보내기
#[command]
pub async fn export_pdf(path: String, content: Vec<u8>) -> Result<(), String> {
    // TODO: PDF 변환 구현
    Err("PDF 내보내기는 아직 구현되지 않았습니다".to_string())
}
```

### 4.5 메뉴 설정

```rust
// src-tauri/src/menu.rs

use tauri::{Menu, MenuItem, Submenu, CustomMenuItem};

pub fn create_menu() -> Menu {
    let file_menu = Submenu::new("파일", Menu::new()
        .add_item(CustomMenuItem::new("new", "새 문서").accelerator("CmdOrCtrl+N"))
        .add_item(CustomMenuItem::new("open", "열기...").accelerator("CmdOrCtrl+O"))
        .add_native_item(MenuItem::Separator)
        .add_item(CustomMenuItem::new("save", "저장").accelerator("CmdOrCtrl+S"))
        .add_item(CustomMenuItem::new("save_as", "다른 이름으로 저장...").accelerator("CmdOrCtrl+Shift+S"))
        .add_native_item(MenuItem::Separator)
        .add_submenu(Submenu::new("최근 파일", Menu::new()
            .add_item(CustomMenuItem::new("recent_clear", "최근 파일 지우기"))
        ))
        .add_native_item(MenuItem::Separator)
        .add_item(CustomMenuItem::new("print", "인쇄...").accelerator("CmdOrCtrl+P"))
        .add_item(CustomMenuItem::new("print_preview", "인쇄 미리보기"))
        .add_native_item(MenuItem::Separator)
        .add_item(CustomMenuItem::new("export_pdf", "PDF로 내보내기..."))
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit)
    );

    let edit_menu = Submenu::new("편집", Menu::new()
        .add_item(CustomMenuItem::new("undo", "실행 취소").accelerator("CmdOrCtrl+Z"))
        .add_item(CustomMenuItem::new("redo", "다시 실행").accelerator("CmdOrCtrl+Shift+Z"))
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::SelectAll)
        .add_native_item(MenuItem::Separator)
        .add_item(CustomMenuItem::new("find", "찾기...").accelerator("CmdOrCtrl+F"))
        .add_item(CustomMenuItem::new("replace", "찾아 바꾸기...").accelerator("CmdOrCtrl+H"))
    );

    let format_menu = Submenu::new("서식", Menu::new()
        .add_item(CustomMenuItem::new("bold", "굵게").accelerator("CmdOrCtrl+B"))
        .add_item(CustomMenuItem::new("italic", "기울임꼴").accelerator("CmdOrCtrl+I"))
        .add_item(CustomMenuItem::new("underline", "밑줄").accelerator("CmdOrCtrl+U"))
        .add_native_item(MenuItem::Separator)
        .add_submenu(Submenu::new("정렬", Menu::new()
            .add_item(CustomMenuItem::new("align_left", "왼쪽 맞춤"))
            .add_item(CustomMenuItem::new("align_center", "가운데 맞춤"))
            .add_item(CustomMenuItem::new("align_right", "오른쪽 맞춤"))
            .add_item(CustomMenuItem::new("align_justify", "양쪽 맞춤"))
        ))
        .add_native_item(MenuItem::Separator)
        .add_item(CustomMenuItem::new("font", "글꼴..."))
        .add_item(CustomMenuItem::new("paragraph", "문단..."))
    );

    let view_menu = Submenu::new("보기", Menu::new()
        .add_item(CustomMenuItem::new("zoom_in", "확대").accelerator("CmdOrCtrl+Plus"))
        .add_item(CustomMenuItem::new("zoom_out", "축소").accelerator("CmdOrCtrl+Minus"))
        .add_item(CustomMenuItem::new("zoom_reset", "100%").accelerator("CmdOrCtrl+0"))
        .add_native_item(MenuItem::Separator)
        .add_item(CustomMenuItem::new("ruler", "눈금자").accelerator("CmdOrCtrl+R"))
        .add_item(CustomMenuItem::new("grid", "격자"))
    );

    let help_menu = Submenu::new("도움말", Menu::new()
        .add_item(CustomMenuItem::new("about", "OpenHWP Editor 정보"))
        .add_item(CustomMenuItem::new("website", "웹사이트"))
    );

    Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
        .add_submenu(format_menu)
        .add_submenu(view_menu)
        .add_submenu(help_menu)
}
```

### 4.6 메인 진입점

```rust
// src-tauri/src/main.rs

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod menu;

use commands::AppState;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .menu(menu::create_menu())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::open_file,
            commands::save_file,
            commands::save_file_dialog,
            commands::open_file_dialog,
            commands::get_recent_files,
            commands::set_modified,
            commands::print_preview,
            commands::export_pdf,
        ])
        .on_menu_event(|event| {
            let window = event.window();

            match event.menu_item_id() {
                "new" => {
                    window.emit("menu-new", ()).ok();
                }
                "open" => {
                    window.emit("menu-open", ()).ok();
                }
                "save" => {
                    window.emit("menu-save", ()).ok();
                }
                "save_as" => {
                    window.emit("menu-save-as", ()).ok();
                }
                "undo" => {
                    window.emit("menu-undo", ()).ok();
                }
                "redo" => {
                    window.emit("menu-redo", ()).ok();
                }
                "bold" => {
                    window.emit("menu-bold", ()).ok();
                }
                "italic" => {
                    window.emit("menu-italic", ()).ok();
                }
                "underline" => {
                    window.emit("menu-underline", ()).ok();
                }
                "zoom_in" => {
                    window.emit("menu-zoom-in", ()).ok();
                }
                "zoom_out" => {
                    window.emit("menu-zoom-out", ()).ok();
                }
                "zoom_reset" => {
                    window.emit("menu-zoom-reset", ()).ok();
                }
                "about" => {
                    // 정보 다이얼로그 표시
                    tauri::api::dialog::message(
                        Some(&window),
                        "OpenHWP Editor",
                        "버전 0.1.0\n\n오픈소스 한글 문서 편집기"
                    );
                }
                _ => {}
            }
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                // 저장되지 않은 변경사항 확인
                let window = event.window();
                let state = window.state::<AppState>();

                if *state.is_modified.lock().unwrap() {
                    api.prevent_close();
                    window.emit("close-requested", ()).ok();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 4.7 프론트엔드 통합

```typescript
// src/main.ts

import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { ask, message } from '@tauri-apps/api/dialog';
import init, { WebEditorApp } from 'editor-web';

let editor: WebEditorApp | null = null;

async function initEditor() {
    // WASM 초기화
    await init();

    // 에디터 생성
    editor = new WebEditorApp('editor-container');

    // 창 크기에 맞춤
    const container = document.getElementById('editor-container')!;
    editor.resize(container.clientWidth, container.clientHeight);

    // 리사이즈 처리
    window.addEventListener('resize', () => {
        if (editor) {
            editor.resize(container.clientWidth, container.clientHeight);
        }
    });

    // 메뉴 이벤트 리스너
    setupMenuListeners();

    // 닫기 요청 리스너
    listen('close-requested', async () => {
        const shouldClose = await ask('저장하지 않은 변경사항이 있습니다. 저장하시겠습니까?', {
            title: '문서 저장',
            type: 'warning',
        });

        if (shouldClose) {
            // 저장 후 닫기
            await saveDocument();
        }
        // 창 닫기 허용
        // window.close() 또는 appWindow.close()
    });
}

function setupMenuListeners() {
    listen('menu-new', () => {
        // 새 문서
        if (editor) {
            // TODO: 새 문서 생성
        }
    });

    listen('menu-open', async () => {
        const path = await invoke<string | null>('open_file_dialog');
        if (path && editor) {
            const bytes = await invoke<number[]>('open_file', { path });
            editor.loadDocument(new Uint8Array(bytes), path);
        }
    });

    listen('menu-save', async () => {
        await saveDocument();
    });

    listen('menu-save-as', async () => {
        await saveDocumentAs();
    });

    listen('menu-undo', () => {
        editor?.undo();
    });

    listen('menu-redo', () => {
        editor?.redo();
    });

    listen('menu-bold', () => {
        editor?.applyStyle(JSON.stringify({ bold: true }));
    });

    listen('menu-italic', () => {
        editor?.applyStyle(JSON.stringify({ italic: true }));
    });

    listen('menu-underline', () => {
        editor?.applyStyle(JSON.stringify({ underline: true }));
    });

    listen('menu-zoom-in', () => {
        if (editor) {
            editor.setZoom(editor.getZoom() * 1.1);
        }
    });

    listen('menu-zoom-out', () => {
        if (editor) {
            editor.setZoom(editor.getZoom() / 1.1);
        }
    });

    listen('menu-zoom-reset', () => {
        editor?.setZoom(1.0);
    });
}

async function saveDocument() {
    if (!editor) return;

    const content = editor.saveDocument();
    await invoke('save_file', {
        path: null, // 현재 경로 사용
        content: Array.from(content),
    });

    await invoke('set_modified', { modified: false });
}

async function saveDocumentAs() {
    if (!editor) return;

    const path = await invoke<string | null>('save_file_dialog', {
        defaultName: '새 문서.hwpx',
    });

    if (path) {
        const content = editor.saveDocument();
        await invoke('save_file', {
            path,
            content: Array.from(content),
        });

        await invoke('set_modified', { modified: false });
    }
}

// 문서 변경 시 호출
function onDocumentModified() {
    invoke('set_modified', { modified: true });
}

// 앱 시작
initEditor().catch(console.error);
```

---

## 5. 플랫폼별 고려사항

### 5.1 웹 플랫폼

| 항목 | 고려사항 | 해결 방안 |
|------|---------|----------|
| **파일 접근** | 보안 제한으로 직접 파일 접근 불가 | File System Access API 또는 input[type=file] 사용 |
| **클립보드** | 권한 필요, 일부 형식 제한 | Clipboard API + 폴백 |
| **IME** | CompositionEvent 사용 | 숨겨진 textarea로 IME 입력 캡처 |
| **글꼴** | 시스템 글꼴 접근 제한 | 웹 글꼴 사용 또는 Local Font Access API |
| **인쇄** | 브라우저 인쇄 대화상자 사용 | window.print() 또는 PDF 생성 |
| **성능** | WASM 번들 크기, 메모리 제한 | 코드 분할, 청킹 로드 |

### 5.2 데스크톱 플랫폼 (Tauri)

| 항목 | 고려사항 | 해결 방안 |
|------|---------|----------|
| **파일 연결** | .hwp, .hwpx 파일 연결 | tauri.conf.json의 fileAssociations |
| **메뉴** | 네이티브 메뉴 바 | Tauri Menu API |
| **시스템 트레이** | 백그라운드 실행 | 선택적 구현 |
| **자동 업데이트** | 앱 업데이트 | Tauri Updater |
| **창 상태** | 크기/위치 저장 | 설정 파일에 저장 |

### 5.3 macOS 특이사항

```rust
// macOS 특정 설정
#[cfg(target_os = "macos")]
fn macos_setup(app: &tauri::App) {
    use tauri::WindowEvent;

    // Command 키를 Ctrl 키처럼 동작하도록 설정
    // (이미 JS에서 처리됨: e.metaKey를 ctrl로 취급)

    // 앱 메뉴 첫 번째 항목은 앱 이름
    // (Tauri가 자동 처리)

    // 풀스크린 지원
    // (tauri.conf.json에서 설정)
}
```

### 5.4 Windows 특이사항

```rust
// Windows 특정 설정
#[cfg(target_os = "windows")]
fn windows_setup(app: &tauri::App) {
    // 고DPI 설정
    // (Tauri가 자동 처리)

    // 파일 연결 레지스트리 등록
    // (빌드 시 자동 처리)
}
```

---

## 6. 빌드 및 배포

### 6.1 웹 빌드

```bash
# WASM 빌드
cd crates/editor-web
wasm-pack build --target web --release

# 번들 크기 최적화
wasm-opt -O3 -o pkg/editor_web_bg_opt.wasm pkg/editor_web_bg.wasm

# npm 패키지 생성
cd pkg
npm pack
```

### 6.2 데스크톱 빌드

```bash
# 개발 모드
cd editor-desktop
npm run tauri dev

# 릴리스 빌드
npm run tauri build

# 특정 플랫폼 빌드
npm run tauri build -- --target x86_64-pc-windows-msvc
npm run tauri build -- --target x86_64-apple-darwin
npm run tauri build -- --target aarch64-apple-darwin
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

### 6.3 CI/CD 파이프라인

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-web:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
      - name: Install wasm-pack
        run: cargo install wasm-pack
      - name: Build WASM
        run: |
          cd crates/editor-web
          wasm-pack build --target web --release
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: wasm-package
          path: crates/editor-web/pkg/

  build-desktop:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: windows-latest
            target: x86_64-pc-windows-msvc
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
      - name: Install dependencies (Linux)
        if: matrix.os == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev
      - name: Build Tauri
        run: |
          cd editor-desktop
          npm install
          npm run tauri build -- --target ${{ matrix.target }}
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: desktop-${{ matrix.target }}
          path: editor-desktop/src-tauri/target/${{ matrix.target }}/release/bundle/
```

---

## 7. 테스트 전략

### 7.1 웹 플랫폼 테스트

```rust
// crates/editor-web/tests/web_tests.rs

#[cfg(target_arch = "wasm32")]
mod tests {
    use wasm_bindgen_test::*;
    use editor_web::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_editor_creation() {
        // 테스트용 컨테이너 생성
        let document = web_sys::window()
            .unwrap()
            .document()
            .unwrap();

        let container = document.create_element("div").unwrap();
        container.set_id("test-container");
        document.body().unwrap().append_child(&container).unwrap();

        // 에디터 생성
        let editor = WebEditorApp::new("test-container");
        assert!(editor.is_ok());

        // 정리
        container.remove();
    }

    #[wasm_bindgen_test]
    async fn test_text_insertion() {
        // ...
    }
}
```

### 7.2 데스크톱 플랫폼 테스트

```rust
// editor-desktop/src-tauri/tests/integration_tests.rs

#[cfg(test)]
mod tests {
    use super::commands::*;

    #[test]
    fn test_app_state() {
        let state = AppState::new();
        assert!(state.current_file.lock().unwrap().is_none());
        assert!(!*state.is_modified.lock().unwrap());
    }
}
```

---

## 8. 구현 체크리스트

### 8.1 platform-api

- [ ] `PlatformContext` 트레잇 정의
- [ ] `ClipboardAccess` 트레잇 정의
- [ ] `FileSystemAccess` 트레잇 정의
- [ ] `Scheduler` 트레잇 정의
- [ ] `DialogProvider` 트레잇 정의

### 8.2 editor-web

- [ ] `WebEditorApp` 기본 구조
- [ ] Canvas 생성 및 설정
- [ ] 숨겨진 입력 요소 (IME)
- [ ] 마우스 이벤트 핸들링
- [ ] 키보드 이벤트 핸들링
- [ ] IME 조합 이벤트 핸들링
- [ ] 클립보드 이벤트 핸들링
- [ ] 리사이즈 처리
- [ ] 웹 클립보드 구현
- [ ] 웹 파일 시스템 구현
- [ ] 웹 스케줄러 구현
- [ ] JavaScript API 바인딩
- [ ] TypeScript 타입 정의

### 8.3 editor-desktop

- [ ] Tauri 프로젝트 설정
- [ ] 메뉴 바 구현
- [ ] 파일 열기/저장 커맨드
- [ ] 파일 연결 설정
- [ ] 창 상태 관리
- [ ] 프론트엔드 통합
- [ ] 빌드 스크립트

---

## 9. 참고 자료

### 9.1 웹 API

- [Clipboard API](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard_API)
- [File System Access API](https://developer.mozilla.org/en-US/docs/Web/API/File_System_Access_API)
- [Canvas API](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API)
- [CompositionEvent](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent)

### 9.2 Rust/WASM

- [wasm-bindgen 문서](https://rustwasm.github.io/wasm-bindgen/)
- [web-sys 문서](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)

### 9.3 Tauri

- [Tauri 공식 문서](https://tauri.app/v1/guides/)
- [Tauri API 레퍼런스](https://tauri.app/v1/api/js/)
