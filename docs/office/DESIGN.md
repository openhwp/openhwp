# OpenHWP Office Design Document

이 문서는 OpenHWP 리치 텍스트 에디터의 전체 설계를 정의합니다.

> **대상 독자**: 이 프로젝트를 구현할 개발자 및 AI 에이전트
>
> **관련 문서**:
> - [IMPLEMENTATION.md](./IMPLEMENTATION.md) - 구현 가이드 및 순서
> - [DECISIONS.md](./DECISIONS.md) - 설계 결정 요약
> - [DOCUMENT.md](./DOCUMENT.md) - document 크레이트 상세 명세
> - [LAYOUT.md](./LAYOUT.md) - layout 크레이트 상세 명세
> - [RENDER.md](./RENDER.md) - render 크레이트 상세 명세
> - [OFFICE.md](./OFFICE.md) - office-core 상세 명세
> - [PLATFORM.md](./PLATFORM.md) - 플랫폼 통합 상세 명세

---

## 1. 프로젝트 개요

### 1.1 목표

아래아한글, MS Word와 같은 **조판 수준의 오픈소스 리치 텍스트 에디터**를 Rust로 구현합니다.

**핵심 기능**:
- HWP/HWPX 파일 열기 및 저장
- 텍스트 편집 (삽입, 삭제, 서식 적용)
- 표, 이미지, 도형 등 복합 요소 지원
- 페이지 단위 레이아웃 및 인쇄 미리보기
- 한글 입력기(IME) 완벽 지원

### 1.2 플랫폼 우선순위

| 우선순위 | 플랫폼 | 기술 스택 | 비고 |
|---------|--------|----------|------|
| **1순위** | 데스크톱 해상도 웹 | Rust → WASM + Canvas 2D | 먼저 구현 |
| **2순위** | 데스크톱 네이티브 | Tauri (웹뷰 래핑) | 웹 버전 재사용 |
| **3순위** | 데스크톱 네이티브 최적화 | tiny-skia 렌더러 | 성능 필요시 |
| 낮음 | 모바일 웹/앱 | 미정 | 추후 고려 |

### 1.3 지원 파일 포맷

| 포맷 | 확장자 | 설명 | 크레이트 |
|------|--------|------|---------|
| HWP 5.0 | `.hwp` | 한글 2002~2022 바이너리 포맷 (CFB 컨테이너) | `hwp` |
| HWPX | `.hwpx` | KS X 6101:2024 XML 포맷 (ZIP 컨테이너) | `hwpx` |

### 1.4 비기능 요구사항

| 항목 | 목표 |
|------|------|
| 렌더링 성능 | 60fps 유지 |
| 초기 로딩 | WASM 번들 < 2MB (gzip) |
| 메모리 사용 | 일반 문서 (10페이지) < 50MB |
| 접근성 | WCAG 2.1 AA 수준 |

---

## 2. 아키텍처 개요

### 2.1 전체 데이터 흐름

```
┌─────────────────────────────────────────────────────────────────┐
│                         파일 시스템                              │
│                    (HWP/HWPX 파일)                               │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      hwp / hwpx 크레이트                         │
│                   (파일 파싱 및 생성)                             │
│                                                                 │
│  HwpDocument::from_bytes()          hwpx::from_str()            │
│  HwpWriter::write()                 hwpx::to_string()           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                        ir 크레이트                               │
│              (중간 표현 - 파일 포맷 간 변환용)                     │
│                                                                 │
│  - HWP와 HWPX의 합집합 표현                                      │
│  - 파일 열기/저장 시에만 사용                                     │
│  - 편집 중에는 사용하지 않음                                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     document 크레이트                            │
│                  (편집용 문서 모델)                               │
│                                                                 │
│  - 편집 작업에 최적화된 자료구조                                  │
│  - Command 패턴 기반 편집 연산                                   │
│  - Undo/Redo 히스토리                                           │
│  - 선택 영역 관리                                                │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      layout 크레이트                             │
│                    (레이아웃 엔진)                                │
│                                                                 │
│  - 텍스트 측정 및 줄바꿈                                         │
│  - 블록 배치 (문단, 표, 이미지)                                   │
│  - 페이지 분할                                                   │
│  - dirty 플래그 기반 증분 업데이트                                │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    render-api 크레이트                           │
│                   (렌더링 추상화)                                 │
│                                                                 │
│  - 플랫폼 독립적 렌더링 명령                                      │
│  - Renderer trait 정의                                          │
└─────────────────────────────────────────────────────────────────┘
                              │
               ┌──────────────┴──────────────┐
               ▼                             ▼
┌─────────────────────────┐   ┌─────────────────────────┐
│    render-web 크레이트   │   │  render-native 크레이트  │
│    (Canvas 2D 구현)      │   │  (tiny-skia, 추후)       │
└─────────────────────────┘   └─────────────────────────┘
               │                             │
               ▼                             ▼
┌─────────────────────────┐   ┌─────────────────────────┐
│      웹 브라우저         │   │     데스크톱 윈도우      │
│   (Canvas Element)      │   │    (Tauri WebView)      │
└─────────────────────────┘   └─────────────────────────┘
```

### 2.2 핵심 설계 원칙

#### 원칙 1: 관심사 분리

| 레이어 | 책임 | 의존성 |
|--------|------|--------|
| `ir` | 파일 포맷 호환 | 없음 |
| `document` | 편집 로직 | `ir` (변환용) |
| `layout` | 레이아웃 계산 | `document` |
| `render-api` | 렌더링 추상화 | `layout` |
| `render-*` | 플랫폼별 렌더링 | `render-api` |
| `office-core` | 에디터 통합 | 모든 크레이트 |

#### 원칙 2: 플랫폼 독립적 코어

```
플랫폼 독립 (순수 Rust)          플랫폼 의존
┌────────────────────────┐     ┌────────────────────┐
│  document              │     │  render-web        │
│  layout                │     │  render-native     │
│  render-api            │     │  office-web        │
│  office-core           │     │  office-native    │
│  input (추상화)         │     │  input-web         │
└────────────────────────┘     └────────────────────┘
```

#### 원칙 3: 결정론적 이벤트 루프

```rust
// 모든 상태 변경은 순수 함수로 처리
fn handle_event(state: &mut State, event: Event) -> UpdateResult {
    // 부작용 없음, 동일 입력 → 동일 출력
}

// 렌더링은 플랫폼에 위임
fn request_render() {
    // 웹: requestAnimationFrame
    // 네이티브: 윈도우 시스템에 invalidate
}
```

#### 원칙 4: 점진적 구현

1. **MVP**: 기본 텍스트 편집 + 웹 렌더링
2. **알파**: 표, 이미지, 스타일 편집
3. **베타**: 페이지 레이아웃, 인쇄
4. **정식**: 모든 HWP/HWPX 기능

---

## 3. 크레이트 구조

### 3.1 전체 구조

```
crates/
├── hwp/                  # [기존] HWP 5.0 파서
├── hwpx/                 # [기존] HWPX 파서
├── ir/                   # [기존] 중간 표현
│
├── document/             # [신규] 편집용 문서 모델
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── model/        # 문서 구조
│       │   ├── mod.rs
│       │   ├── document.rs
│       │   ├── block.rs
│       │   ├── paragraph.rs
│       │   ├── table.rs
│       │   ├── image.rs
│       │   ├── shape.rs
│       │   ├── text_storage.rs
│       │   ├── style.rs
│       │   ├── style_store.rs
│       │   └── binary_store.rs
│       ├── selection/    # 선택 영역
│       │   ├── mod.rs
│       │   ├── position.rs
│       │   ├── selection.rs
│       │   └── range.rs
│       ├── edit/         # 편집 연산
│       │   ├── mod.rs
│       │   ├── command.rs
│       │   ├── commands/
│       │   │   ├── mod.rs
│       │   │   ├── insert_text.rs
│       │   │   ├── delete.rs
│       │   │   ├── format.rs
│       │   │   ├── split_paragraph.rs
│       │   │   ├── merge_paragraph.rs
│       │   │   └── ...
│       │   ├── history.rs
│       │   └── clipboard.rs
│       └── convert/      # IR 변환
│           ├── mod.rs
│           ├── from_ir.rs
│           └── to_ir.rs
│
├── layout/               # [신규] 레이아웃 엔진
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── engine.rs     # LayoutEngine 메인
│       ├── context.rs    # LayoutContext
│       ├── cache.rs      # 레이아웃 캐시
│       ├── text/         # 텍스트 레이아웃
│       │   ├── mod.rs
│       │   ├── measurer.rs
│       │   ├── shaper.rs
│       │   ├── line_break.rs
│       │   ├── line_break_config.rs
│       │   └── run.rs
│       ├── block/        # 블록 레이아웃
│       │   ├── mod.rs
│       │   ├── paragraph.rs
│       │   ├── table.rs
│       │   ├── image.rs
│       │   └── shape.rs
│       ├── page/         # 페이지 레이아웃
│       │   ├── mod.rs
│       │   ├── paginator.rs
│       │   ├── page.rs
│       │   ├── header_footer.rs
│       │   └── page_number.rs
│       ├── font/         # 글꼴 처리
│       │   ├── mod.rs
│       │   ├── resolver.rs
│       │   ├── metrics.rs
│       │   └── fallback.rs
│       ├── bidi.rs       # 양방향 텍스트
│       └── hit_test.rs   # 좌표 → 위치 변환
│
├── render-api/           # [신규] 렌더링 추상화
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── command.rs    # RenderCommand enum
│       ├── renderer.rs   # Renderer trait
│       ├── text.rs       # TextRenderMethod
│       ├── color.rs      # Color
│       ├── geometry.rs   # Point, Rect, Size
│       └── style.rs      # 렌더링용 스타일
│
├── render-web/           # [신규] Canvas 렌더러
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── canvas_renderer.rs
│       ├── text_measurer.rs
│       └── image_cache.rs
│
├── render-native/        # [신규, 추후] 네이티브 렌더러
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── skia_renderer.rs
│       └── ...
│
├── input/                # [신규] 입력 추상화
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── event.rs      # InputEvent enum
│       ├── keyboard.rs   # 키보드 이벤트
│       ├── mouse.rs      # 마우스 이벤트
│       ├── ime.rs        # IME 이벤트
│       └── clipboard.rs  # 클립보드 추상화
│
├── office-core/          # [신규] 에디터 코어
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── office.rs     # OfficeCore
│       ├── state.rs      # OfficeState
│       ├── event.rs      # OfficeEvent
│       ├── handler/      # 이벤트 핸들러
│       │   ├── mod.rs
│       │   ├── keyboard.rs
│       │   ├── mouse.rs
│       │   ├── ime.rs
│       │   └── clipboard.rs
│       ├── viewport.rs   # 뷰포트 관리
│       └── cursor.rs     # 커서 렌더링
│
├── office-web/           # [신규] 웹 에디터
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── platform.rs   # WebPlatform
│       ├── event_bridge.rs
│       ├── ime_handler.rs
│       └── bindings.rs   # wasm-bindgen
│
└── office-native/       # [신규, 추후] 데스크톱 에디터
    ├── Cargo.toml
    ├── src/
    │   └── main.rs
    └── tauri.conf.json
```

### 3.2 크레이트 의존성

```toml
# document/Cargo.toml
[dependencies]
ir = { path = "../ir" }

# layout/Cargo.toml
[dependencies]
document = { path = "../document" }
unicode-bidi = "0.3"
unicode-segmentation = "1.10"

# render-api/Cargo.toml
[dependencies]
# 의존성 없음 (순수 타입 정의)

# render-web/Cargo.toml
[dependencies]
render-api = { path = "../render-api" }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [...] }

# office-core/Cargo.toml
[dependencies]
document = { path = "../document" }
layout = { path = "../layout" }
render-api = { path = "../render-api" }
input = { path = "../input" }

# office-web/Cargo.toml
[dependencies]
office-core = { path = "../office-core" }
render-web = { path = "../render-web" }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [...] }
```

---

## 4. 핵심 데이터 흐름

### 4.1 파일 열기

```
1. 사용자가 파일 선택
2. 파일 바이트 읽기
3. hwp/hwpx 크레이트로 파싱
4. IR로 변환
5. Document로 변환
6. 레이아웃 계산
7. 화면 렌더링
```

```rust
// 의사 코드
async fn open_file(bytes: &[u8], file_type: FileType) -> Result<(), Error> {
    // 1. 파일 파싱
    let ir_document = match file_type {
        FileType::Hwp => {
            let hwp = HwpDocument::from_bytes(bytes)?;
            hwp.to_ir()?
        }
        FileType::Hwpx => {
            let hwpx = hwpx::Document::from_bytes(bytes)?;
            hwpx.to_ir()?
        }
    };

    // 2. Document로 변환
    let document = Document::from_ir(ir_document)?;

    // 3. 에디터에 로드
    office.load_document(document);

    // 4. 레이아웃 및 렌더링은 자동으로 트리거됨
    Ok(())
}
```

### 4.2 파일 저장

```
1. Document → IR 변환
2. IR → hwp/hwpx로 직렬화
3. 파일 쓰기
```

```rust
async fn save_file(office: &Office, file_type: FileType) -> Result<Vec<u8>, Error> {
    // 1. IR로 변환
    let ir_document = office.document().to_ir()?;

    // 2. 파일 포맷으로 직렬화
    let bytes = match file_type {
        FileType::Hwp => {
            let hwp = HwpDocument::from_ir(ir_document)?;
            HwpWriter::write(&hwp)?
        }
        FileType::Hwpx => {
            let hwpx = hwpx::Document::from_ir(ir_document)?;
            hwpx.to_bytes()?
        }
    };

    Ok(bytes)
}
```

### 4.3 편집 이벤트 처리

```
1. 플랫폼 이벤트 수신 (DOM Event, Win32 Message 등)
2. InputEvent로 변환
3. OfficeCore.handle_event() 호출
4. Command 생성 및 실행
5. Document 변경
6. 영향받은 블록 dirty 표시
7. 레이아웃 재계산 요청
8. 렌더링 요청
```

```rust
// office-core/src/office.rs
impl OfficeCore {
    pub fn handle_event(&mut self, event: InputEvent) -> UpdateResult {
        match event {
            InputEvent::KeyPress(key) => self.handle_key_press(key),
            InputEvent::MouseDown(pos) => self.handle_mouse_down(pos),
            InputEvent::ImeComposition(text) => self.handle_ime_composition(text),
            InputEvent::ImeCommit(text) => self.handle_ime_commit(text),
            // ...
        }
    }

    fn handle_key_press(&mut self, key: Key) -> UpdateResult {
        match key {
            Key::Char(c) => {
                let cmd = InsertTextCommand::new(self.selection.cursor(), c.to_string());
                self.execute_command(cmd)
            }
            Key::Backspace => {
                let cmd = DeleteCommand::backward(self.selection.cursor());
                self.execute_command(cmd)
            }
            // ...
        }
    }

    fn execute_command(&mut self, cmd: Box<dyn Command>) -> UpdateResult {
        // 1. 명령 실행
        let affected_range = cmd.execute(&mut self.document);

        // 2. 히스토리에 추가
        self.history.push(cmd);

        // 3. 결과 반환
        UpdateResult {
            document_changed: true,
            dirty_range: Some(affected_range),
            scroll_to_cursor: true,
        }
    }
}
```

### 4.4 레이아웃 및 렌더링

```
1. UpdateResult 확인
2. dirty 블록 레이아웃 재계산
3. RenderCommand 생성
4. 플랫폼 렌더러로 실행
```

```rust
// office-web/src/platform.rs
impl WebPlatform {
    fn update_and_render(&mut self) {
        // 1. 레이아웃 업데이트
        self.core.update_layout(&self.viewport, &self.text_measurer);

        // 2. 렌더링 명령 생성
        let commands = self.core.render(&self.viewport);

        // 3. Canvas에 렌더링
        self.renderer.execute(&commands);
    }
}
```

---

## 5. 구현 로드맵

### Phase 1: 기반 구축 (MVP)

**목표**: 기본 텍스트 편집이 가능한 웹 에디터

| 작업 | 크레이트 | 설명 |
|------|---------|------|
| 1.1 | `document` | Document, Paragraph, TextStorage 구현 |
| 1.2 | `document` | Position, Selection 구현 |
| 1.3 | `document` | InsertText, Delete Command 구현 |
| 1.4 | `document` | CommandHistory (Undo/Redo) 구현 |
| 1.5 | `layout` | TextMeasurer trait 정의 |
| 1.6 | `layout` | 기본 줄바꿈 구현 |
| 1.7 | `layout` | ParagraphLayout 구현 |
| 1.8 | `render-api` | RenderCommand, Renderer trait 정의 |
| 1.9 | `render-web` | CanvasRenderer 구현 |
| 1.10 | `render-web` | CanvasTextMeasurer 구현 |
| 1.11 | `office-core` | OfficeCore 기본 구조 |
| 1.12 | `office-core` | 키보드 이벤트 처리 |
| 1.13 | `office-web` | WebPlatform, DOM 이벤트 연결 |
| 1.14 | `office-web` | IME 통합 (숨겨진 textarea) |

**완료 기준**: 웹 브라우저에서 텍스트 입력/삭제/Undo가 동작

### Phase 2: 서식 및 스타일

**목표**: 리치 텍스트 서식 편집

| 작업 | 크레이트 | 설명 |
|------|---------|------|
| 2.1 | `document` | CharStyle, ParaStyle 구현 |
| 2.2 | `document` | StyleStore (인터닝) 구현 |
| 2.3 | `document` | StyleRuns (속성 구간) 구현 |
| 2.4 | `document` | FormatCommand 구현 |
| 2.5 | `layout` | 스타일별 텍스트 측정 |
| 2.6 | `layout` | Run 단위 레이아웃 |
| 2.7 | `render-*` | 스타일 적용 렌더링 |
| 2.8 | `office-core` | 서식 단축키 처리 |

**완료 기준**: 굵게, 기울임, 글꼴 크기 변경이 동작

### Phase 3: 복합 요소

**목표**: 표, 이미지 지원

| 작업 | 크레이트 | 설명 |
|------|---------|------|
| 3.1 | `document` | Table, Cell 구현 |
| 3.2 | `document` | ImageBlock 구현 |
| 3.3 | `document` | BinaryStore 구현 |
| 3.4 | `layout` | TableLayout 구현 |
| 3.5 | `layout` | ImageLayout 구현 |
| 3.6 | `render-*` | 표, 이미지 렌더링 |
| 3.7 | `office-core` | 표 편집 (셀 이동, 선택) |
| 3.8 | `office-core` | 이미지 삽입/크기 조절 |

**완료 기준**: 표와 이미지가 포함된 문서 편집 가능

### Phase 4: 파일 연동

**목표**: HWP/HWPX 파일 열기/저장

| 작업 | 크레이트 | 설명 |
|------|---------|------|
| 4.1 | `document` | IR → Document 변환 |
| 4.2 | `document` | Document → IR 변환 |
| 4.3 | `office-web` | 파일 열기 UI |
| 4.4 | `office-web` | 파일 저장 UI |

**완료 기준**: 실제 HWP/HWPX 파일 열고 편집 후 저장 가능

### Phase 5: 페이지 레이아웃

**목표**: 페이지 단위 편집 및 인쇄

| 작업 | 크레이트 | 설명 |
|------|---------|------|
| 5.1 | `layout` | Paginator 구현 |
| 5.2 | `layout` | 머리글/바닥글 레이아웃 |
| 5.3 | `layout` | 페이지 번호 |
| 5.4 | `render-*` | 페이지 구분선 렌더링 |
| 5.5 | `office-core` | 페이지 뷰 모드 |

**완료 기준**: 페이지 단위로 문서가 표시되고 인쇄 미리보기 가능

### Phase 6: 고급 기능

**목표**: 완성도 높은 에디터

| 작업 | 설명 |
|------|------|
| 6.1 | 복사/붙여넣기 완성 |
| 6.2 | 찾기/바꾸기 |
| 6.3 | 도형 편집 |
| 6.4 | 접근성 (ARIA) |
| 6.5 | 데스크톱 앱 (Tauri) |

---

## 6. 테스트 전략

### 6.1 단위 테스트

| 크레이트 | 테스트 대상 |
|---------|------------|
| `document` | 편집 명령, Undo/Redo, 스타일 병합 |
| `layout` | 줄바꿈, 블록 배치, 페이지 분할 |
| `render-api` | 명령 생성 |

```rust
#[test]
fn test_insert_text() {
    let mut doc = Document::new();
    let cmd = InsertTextCommand::new(Position::start(), "Hello");
    cmd.execute(&mut doc);

    assert_eq!(doc.get_text(), "Hello");
}

#[test]
fn test_undo_redo() {
    let mut office = OfficeCore::new();
    office.insert_text("Hello");
    office.undo();

    assert_eq!(office.document().get_text(), "");

    office.redo();
    assert_eq!(office.document().get_text(), "Hello");
}
```

### 6.2 스냅샷 테스트

레이아웃 결과를 스냅샷으로 검증:

```rust
#[test]
fn test_paragraph_layout() {
    let para = Paragraph::new("안녕하세요 반갑습니다");
    let layout = layout_paragraph(&para, 100.0, &mock_measurer());

    insta::assert_debug_snapshot!(layout);
}
```

### 6.3 통합 테스트

```rust
#[test]
fn test_file_roundtrip() {
    let original = std::fs::read("fixtures/sample.hwp").unwrap();
    let hwp = HwpDocument::from_bytes(&original).unwrap();
    let ir = hwp.to_ir().unwrap();
    let doc = Document::from_ir(ir).unwrap();

    let ir2 = doc.to_ir().unwrap();
    let hwp2 = HwpDocument::from_ir(ir2).unwrap();
    let saved = HwpWriter::write(&hwp2).unwrap();

    // 내용이 동일한지 검증 (바이트 동일은 아닐 수 있음)
    assert_eq!(hwp.text_content(), hwp2.text_content());
}
```

### 6.4 E2E 테스트 (추후)

Playwright를 사용한 브라우저 테스트:

```typescript
test('basic editing', async ({ page }) => {
    await page.goto('/office');
    await page.keyboard.type('안녕하세요');

    const text = await page.evaluate(() => office.getText());
    expect(text).toBe('안녕하세요');
});
```

---

## 7. 성능 고려사항

### 7.1 레이아웃 최적화

- **dirty 플래그**: 변경된 블록만 재계산
- **뷰포트 가상화**: 보이는 영역만 레이아웃
- **레이아웃 캐시**: 계산 결과 캐싱

### 7.2 렌더링 최적화

- **더블 버퍼링**: 깜빡임 방지
- **클리핑**: 뷰포트 외부 렌더링 스킵
- **배치 렌더링**: 동일 스타일 텍스트 묶어서 렌더링

### 7.3 메모리 최적화

- **스타일 인터닝**: 중복 스타일 객체 공유
- **문자열 최적화**: 짧은 문단은 String, 긴 문단만 청크 분할
- **이미지 캐시**: 디코딩된 이미지 LRU 캐시

---

## 8. 보안 고려사항

### 8.1 파일 파싱

- 악의적 파일에 대한 방어 (DoS, 메모리 소진)
- 파싱 타임아웃
- 메모리 제한

### 8.2 스크립트 실행

- HWP 매크로 실행하지 않음 (보안상 비활성화)

### 8.3 외부 리소스

- 외부 URL 이미지 로딩 시 사용자 확인
- CORS 정책 준수

---

## 변경 이력

| 날짜 | 버전 | 내용 |
|------|------|------|
| 2024-12 | 0.1 | 초안 작성 |
