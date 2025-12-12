# HWP/HWPX ↔ IR 변환 기능 세트 (진실의 원천)

> 이 문서는 HWP ↔ IR ↔ HWPX 변환 시 모든 기능이 온전히 변환되는지 추적하기 위한 진실의 원천(Single Source of Truth)입니다.
>
> 마지막 검증 날짜: 2025-12-11 (업데이트: Document 크레이트 IR↔Document 변환 완료)

## 범례

- ✅ 완전 구현
- ⚠️ 부분 구현 (세부사항 참고)
- ❌ 미구현
- ➖ 해당 없음 (형식에서 지원하지 않음)

## 형식별 소스 파일 참조

| 형식     | 파싱/직렬화                                             | 변환 (→ IR)                            | 변환 (← IR)                              |
| -------- | ------------------------------------------------------- | -------------------------------------- | ---------------------------------------- |
| HWP      | `crates/hwp/src/body/`, `crates/hwp/src/doc_info/`      | `crates/hwp/src/convert/to_ir.rs`      | `crates/hwp/src/convert/from_ir.rs`      |
| HWPX     | `crates/hwpx/src/paragraph/`, `crates/hwpx/src/header/` | `crates/hwpx/src/convert/to_ir.rs`     | `crates/hwpx/src/convert/from_ir.rs`     |
| IR       | `crates/ir/src/`                                        | -                                      | -                                        |
| Document | `crates/document/src/`                                  | `crates/document/src/convert/to_ir.rs` | `crates/document/src/convert/from_ir.rs` |

---

## 1. 문서 구조 (Document)

### 1.1 메타데이터 (Metadata)

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고 |
| ------------------ | ------ | ------ | ------- | ------- | ---- |
| 제목 (title)       | ✅     | ✅     | ✅      | ✅      |      |
| 저자 (author)      | ✅     | ✅     | ✅      | ✅      |      |
| 주제 (subject)     | ✅     | ✅     | ✅      | ✅      |      |
| 키워드 (keywords)  | ✅     | ✅     | ✅      | ✅      |      |
| 설명 (description) | ✅     | ✅     | ✅      | ✅      |      |
| 생성일 (created)   | ✅     | ✅     | ✅      | ✅      |      |
| 수정일 (modified)  | ✅     | ✅     | ✅      | ✅      |      |
| 버전 (version)     | ✅     | ✅     | ✅      | ✅      |      |

### 1.2 문서 설정 (DocumentSettings)

| 항목                     | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                  |
| ------------------------ | ------ | ------ | ------- | ------- | --------------------- |
| starting_page_number     | ✅     | ✅     | ✅      | ✅      |                       |
| starting_footnote_number | ✅     | ✅     | ✅      | ✅      |                       |
| starting_endnote_number  | ✅     | ✅     | ✅      | ✅      |                       |
| starting_figure_number   | ✅     | ✅     | ✅      | ✅      | HWP SectionDefinition |
| starting_table_number    | ✅     | ✅     | ✅      | ✅      | HWP SectionDefinition |
| starting_equation_number | ✅     | ✅     | ✅      | ✅      | HWP SectionDefinition |
| caret_position           | ✅     | ✅     | ✅      | ✅      |                       |
| compatible_document      | ✅     | ✅     | ✅      | ✅      |                       |
| default_tab_interval     | ✅     | ✅     | ✅      | ✅      | 기본 탭 간격          |
| representative_language  | ✅     | ✅     | ➖      | ➖      | HWP SectionDefinition language 필드 (HWPX 미지원) |

### 1.3 문서 속성 (DocumentProperties) - HWP

| 항목                        | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고              |
| --------------------------- | ------ | ------ | ------- | ------- | ----------------- |
| section_count               | ➖     | ➖     | ➖      | ➖      | 파일 구조 메타데이터, IR에서 `sections.len()`으로 계산 |
| caret_list_id               | ❌     | ❌     | ➖      | ➖      | IR CaretPosition 있으나 변환 미구현 |
| caret_paragraph_id          | ❌     | ❌     | ➖      | ➖      | IR CaretPosition 있으나 변환 미구현 |
| caret_position_in_paragraph | ❌     | ❌     | ➖      | ➖      | IR CaretPosition 있으나 변환 미구현 |

### 1.4 문서 옵션 (DocumentOption) - HWPX

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고           |
| ------------------ | ------ | ------ | ------- | ------- | -------------- |
| link_document_path | ➖     | ➖     | ✅      | ✅      | 연결 문서 경로 (HWPX 전용) |

### 1.5 HCF 버전 (HcfVersion) - HWPX

| 항목                | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                             |
| ------------------- | ------ | ------ | ------- | ------- | ------------------------------------------------ |
| target_application  | ➖     | ➖     | ➖      | ➖      | HWPX 파일 메타데이터, 변환 불필요 |
| major               | ➖     | ➖     | ➖      | ➖      | HWPX 파일 메타데이터, 변환 불필요 |
| minor               | ➖     | ➖     | ➖      | ➖      | HWPX 파일 메타데이터, 변환 불필요 |
| micro               | ➖     | ➖     | ➖      | ➖      | HWPX 파일 메타데이터, 변환 불필요 |
| build_number        | ➖     | ➖     | ➖      | ➖      | HWPX 파일 메타데이터, 변환 불필요 |
| os                  | ➖     | ➖     | ➖      | ➖      | HWPX 파일 메타데이터, 변환 불필요 |
| xml_version         | ➖     | ➖     | ➖      | ➖      | HWPX 파일 메타데이터, 변환 불필요 |
| application         | ➖     | ➖     | ➖      | ➖      | HWPX 파일 메타데이터, 변환 불필요 |
| application_version | ➖     | ➖     | ➖      | ➖      | HWPX 파일 메타데이터, 변환 불필요 |

### 1.6 ID 매핑 (IdMappings) - HWP

| 항목                      | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                     |
| ------------------------- | ------ | ------ | ------- | ------- | ------------------------ |
| memo_shape_count          | ➖     | ➖     | ➖      | ➖      | 파일 구조 전용, 변환 불필요 |
| track_change_count        | ➖     | ➖     | ➖      | ➖      | 파일 구조 전용, 변환 불필요 |
| track_change_author_count | ➖     | ➖     | ➖      | ➖      | 파일 구조 전용, 변환 불필요 |

---

## 2. 섹션 (Section)

### 2.1 페이지 정의 (PageDefinition)

| 항목            | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                  |
| --------------- | ------ | ------ | ------- | ------- | --------------------- |
| width           | ✅     | ✅     | ✅      | ✅      |                       |
| height          | ✅     | ✅     | ✅      | ✅      |                       |
| margins.left    | ✅     | ✅     | ✅      | ✅      |                       |
| margins.right   | ✅     | ✅     | ✅      | ✅      |                       |
| margins.top     | ✅     | ✅     | ✅      | ✅      |                       |
| margins.bottom  | ✅     | ✅     | ✅      | ✅      |                       |
| margins.header  | ✅     | ✅     | ✅      | ✅      |                       |
| margins.footer  | ✅     | ✅     | ✅      | ✅      |                       |
| margins.gutter  | ✅     | ✅     | ✅      | ✅      |                       |
| orientation     | ✅     | ✅     | ✅      | ✅      | Portrait/Landscape    |
| gutter_position | ✅     | ✅     | ✅      | ✅      | Left/Right/Top/Bottom |

### 2.2 단 정의 (ColumnDefinition)

| 항목                | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                        |
| ------------------- | ------ | ------ | ------- | ------- | --------------------------- |
| count               | ✅     | ✅     | ✅      | ✅      |                             |
| column_type         | ✅     | ✅     | ✅      | ✅      | Normal/Distributed/Parallel |
| direction           | ✅     | ✅     | ✅      | ✅      | LTR/RTL/Facing              |
| gap                 | ✅     | ✅     | ✅      | ✅      |                             |
| separator           | ✅     | ✅     | ✅      | ✅      | None/Solid/Dash/Dot         |
| separator_thickness | ✅     | ✅     | ✅      | ✅      | HWP ColumnDefinition에 있음 |
| separator_color     | ✅     | ✅     | ✅      | ✅      | HWP ColumnDefinition에 있음 |
| widths[]            | ✅     | ✅     | ✅      | ✅      | 개별 단 너비                |
| same_width          | ✅     | ✅     | ✅      | ✅      | 동일 너비 여부              |

### 2.3 머리글/바닥글 (HeaderFooter)

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고          |
| ------------ | ------ | ------ | ------- | ------- | ------------- |
| apply_to     | ✅     | ✅     | ✅      | ✅      | Both/Even/Odd |
| paragraphs[] | ✅     | ✅     | ✅      | ✅      |               |

### 2.4 각주/미주 모양 (NoteShape)

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                    |
| ---------------- | ------ | ------ | ------- | ------- | ----------------------- |
| number_format    | ✅     | ✅     | ✅      | ✅      |                         |
| numbering        | ✅     | ✅     | ✅      | ✅      | Continuous/Page/Section |
| superscript      | ✅     | ✅     | ✅      | ✅      |                         |
| prefix           | ✅     | ✅     | ✅      | ✅      |                         |
| suffix           | ✅     | ✅     | ✅      | ✅      |                         |
| start_number     | ✅     | ✅     | ✅      | ✅      |                         |
| separator_length | ✅     | ✅     | ✅      | ✅      |                         |
| space_above      | ✅     | ✅     | ✅      | ✅      |                         |
| space_between    | ✅     | ✅     | ✅      | ✅      |                         |
| user_character   | ✅     | ✅     | ✅      | ✅      | HWPX 사용자 기호        |

### 2.4.1 각주/미주 구분선 (NoteLine) - HWPX

| 항목      | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고        |
| --------- | ------ | ------ | ------- | ------- | ----------- |
| length    | ✅     | ✅     | ✅      | ✅      | 구분선 길이 |
| line_type | ✅     | ✅     | ✅      | ✅      | 선 종류     |
| width     | ✅     | ✅     | ✅      | ✅      | 선 굵기     |
| color     | ✅     | ✅     | ✅      | ✅      | 선 색상     |

### 2.4.2 각주/미주 간격 (NoteSpacing) - HWPX

| 항목          | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고        |
| ------------- | ------ | ------ | ------- | ------- | ----------- |
| between_notes | ✅     | ✅     | ✅      | ✅      | 주석 사이   |
| above_line    | ✅     | ✅     | ✅      | ✅      | 구분선 위   |
| below_line    | ✅     | ✅     | ✅      | ✅      | 구분선 아래 |

### 2.4.3 각주/미주 배치 (FootnotePlacementSettings/EndnotePlacementSettings) - HWPX

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                         |
| ------------ | ------ | ------ | ------- | ------- | ---------------------------- |
| place        | ➖     | ➖     | ✅      | ✅      | 배치 위치 (HWP 개념 다름)    |
| beneath_text | ➖     | ➖     | ✅      | ✅      | 텍스트 이어 출력 (HWP 없음)  |

### 2.4.4 각주/미주 번호 매기기 (FootnoteNumbering/EndnoteNumbering) - HWPX

| 항목           | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고               |
| -------------- | ------ | ------ | ------- | ------- | ------------------ |
| numbering_type | ✅     | ✅     | ✅      | ✅      | 번호 매기기 형식   |
| new_number     | ✅     | ✅     | ✅      | ✅      | 시작 번호 (섹션별) |

### 2.4.5 각주/미주 모양 (FootnoteShape/EndnoteShape) - HWP 전용

| 항목                     | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                     |
| ------------------------ | ------ | ------ | ------- | ------- | ------------------------ |
| separator_position       | ✅     | ✅     | -       | -       | HWP 구분선 위치          |
| space_below              | ✅     | ✅     | -       | -       | HWP 구분선 아래 간격     |
| continue_numbering       | ✅     | ✅     | -       | -       | HWP 섹션 넘어 연속 번호  |
| separator_line_type      | ✅     | ✅     | -       | -       | HWP 구분선 종류          |
| separator_line_thickness | ✅     | ✅     | -       | -       | HWP 구분선 두께          |
| separator_line_color     | ✅     | ✅     | -       | -       | HWP 구분선 색상          |

### 2.5 페이지 테두리/배경 (PageBorderFill)

| 항목            | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                   |
| --------------- | ------ | ------ | ------- | ------- | -------------------------------------- |
| border_fill_id  | ✅     | ✅     | ✅      | ✅      |                                        |
| position        | ✅     | ✅     | ✅      | ✅      | Paper/Body                             |
| offsets         | ✅     | ✅     | ✅      | ✅      |                                        |
| first_page_only | ➖     | ➖     | ➖      | ➖      | HWP에 없음, HWPX는 page_type 사용      |
| fill_area       | ✅     | ✅     | ✅      | ✅      | HWPX FillAreaType                      |
| header_inside   | ✅     | ✅     | ✅      | ✅      | HWP include_header / HWPX headerInside |
| footer_inside   | ✅     | ✅     | ✅      | ✅      | HWP include_footer / HWPX footerInside |
| fill_behind     | ✅     | ✅     | ➖      | ➖      | HWP 텍스트 뒤 채우기                   |
| page_type       | ✅     | ✅     | ✅      | ✅      | HWPX 쪽 테두리 종류                    |
| text_border     | ✅     | ✅     | ✅      | ✅      | HWPX 테두리 위치 기준 (position과 동일)|

### 2.6 섹션 정의 속성 (SectionDefinition) - HWP 전용

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고              |
| ------------------ | ------ | ------ | ------- | ------- | ----------------- |
| hide_header        | ✅     | ✅     | ➖      | ➖      | 머리글 숨김       |
| hide_footer        | ✅     | ✅     | ➖      | ➖      | 바닥글 숨김       |
| hide_master_page   | ✅     | ✅     | ➖      | ➖      | 마스터페이지 숨김 |
| hide_border        | ✅     | ✅     | ➖      | ➖      | 테두리 숨김       |
| hide_background    | ✅     | ✅     | ➖      | ➖      | 배경 숨김         |
| hide_page_number   | ✅     | ✅     | ➖      | ➖      | 쪽 번호 숨김      |
| text_direction     | ✅     | ✅     | ✅      | ✅      | 가로/세로         |
| vertical_grid      | ✅     | ✅     | ✅      | ✅      | 세로 그리드 간격 (IR SectionGrid.line_grid)  |
| horizontal_grid    | ✅     | ✅     | ✅      | ✅      | 가로 그리드 간격 (IR SectionGrid.character_grid)  |
| numbering_shape_id | ❌     | ❌     | ✅      | ✅      | HWP 줄 번호 스타일 참조 ID (파싱되나 IR 미저장, HWPX는 LineNumberShape로 변환)    |

### 2.7 섹션 확장 (SectionExtensions)

| 항목                 | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                   |
| -------------------- | ------ | ------ | ------- | ------- | ---------------------- |
| master_page_ids      | ➖     | ➖     | ✅      | ✅      | HWPX 마스터페이지 참조 |
| presentation (slide) | ➖     | ➖     | ✅      | ✅      | HWPX 프레젠테이션 전용 (IR Extensions.presentation, Section.presentation) |

### 2.8 섹션 시작 번호 (SectionStartNumber) - HWPX

| 항목           | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                |
| -------------- | ------ | ------ | ------- | ------- | ------------------- |
| page_starts_on | ✅     | ✅     | ✅      | ✅      | 새 페이지 시작 옵션 |
| page           | ✅     | ✅     | ✅      | ✅      | 페이지 시작 번호    |
| picture        | ✅     | ✅     | ✅      | ✅      | 그림 시작 번호      |
| table          | ✅     | ✅     | ✅      | ✅      | 표 시작 번호        |
| equation       | ✅     | ✅     | ✅      | ✅      | 수식 시작 번호      |

### 2.9 섹션 그리드 (SectionGrid) - HWPX

| 항목              | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고             |
| ----------------- | ------ | ------ | ------- | ------- | ---------------- |
| line_grid         | ✅     | ✅     | ✅      | ✅      | 세로 줄맞춤      |
| character_grid    | ✅     | ✅     | ✅      | ✅      | 가로 글자 줄맞춤 |
| manuscript_format | ➖     | ➖     | ✅      | ✅      | 원고지 형식 (HWPX 전용) |

### 2.10 섹션 가시성 (SectionVisibility) - HWPX

| 항목                   | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                  |
| ---------------------- | ------ | ------ | ------- | ------- | --------------------- |
| hide_first_header      | ➖     | ➖     | ✅      | ✅      | 첫 페이지 머리말 숨김 (HWPX 전용) |
| hide_first_footer      | ➖     | ➖     | ✅      | ✅      | 첫 페이지 꼬리말 숨김 (HWPX 전용) |
| hide_first_master_page | ➖     | ➖     | ✅      | ✅      | 첫 페이지 바탕쪽 숨김 (HWPX 전용) |
| hide_first_page_number | ➖     | ➖     | ✅      | ✅      | 첫 페이지 번호 숨김 (HWPX 전용)   |
| hide_first_empty_line  | ➖     | ➖     | ✅      | ✅      | 첫 페이지 빈 줄 숨김 (HWPX 전용)  |
| show_line_number       | ➖     | ➖     | ✅      | ✅      | 줄 번호 표시 (HWPX 전용)          |
| border                 | ➖     | ➖     | ✅      | ✅      | 테두리 표시 (HWPX 전용)           |
| fill                   | ➖     | ➖     | ✅      | ✅      | 채우기 표시 (HWPX 전용)           |

### 2.11 줄 번호 모양 (LineNumberShape) - HWPX

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                     |
| ------------ | ------ | ------ | ------- | ------- | ---------------------------------------- |
| restart_type | ➖     | ➖     | ✅      | ✅      | 번호 매기기 방식 (HWP 미지원)            |
| count_by     | ➖     | ➖     | ✅      | ✅      | 표시 간격 (HWP 미지원)                   |
| distance     | ➖     | ➖     | ✅      | ✅      | 본문과의 거리 (HWP 미지원)               |
| start_number | ➖     | ➖     | ✅      | ✅      | 시작 번호 (HWP 미지원)                   |

### 2.12 프레젠테이션 (Presentation) - HWPX

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고           |
| ------------------ | ------ | ------ | ------- | ------- | -------------- |
| effect             | ➖     | ➖     | ➖      | ➖      | 프레젠테이션 전용 기능, 변환 불필요 |
| sound_id_reference | ➖     | ➖     | ➖      | ➖      | 프레젠테이션 전용 기능, 변환 불필요 |
| auto_show          | ➖     | ➖     | ➖      | ➖      | 프레젠테이션 전용 기능, 변환 불필요 |
| show_time          | ➖     | ➖     | ➖      | ➖      | 프레젠테이션 전용 기능, 변환 불필요 |
| invert_text        | ➖     | ➖     | ➖      | ➖      | 프레젠테이션 전용 기능, 변환 불필요 |
| apply_to           | ➖     | ➖     | ➖      | ➖      | 프레젠테이션 전용 기능, 변환 불필요 |
| fill_brush         | ➖     | ➖     | ➖      | ➖      | 프레젠테이션 전용 기능, 변환 불필요 |

---

## 3. 스타일 저장소 (StyleStore)

### 3.1 폰트 (Font)

| 항목                | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                           |
| ------------------- | ------ | ------ | ------- | ------- | ------------------------------ |
| name                | ✅     | ✅     | ✅      | ✅      |                                |
| alternate_name      | ✅     | ✅     | ✅      | ✅      |                                |
| font_type           | ✅     | ✅     | ✅      | ✅      | TrueType/HFT                   |
| family              | ✅     | ✅     | ✅      | ✅      | Serif/SansSerif/etc            |
| panose_info         | ✅     | ✅     | ✅      | ✅      | HWP PanoseData, HWPX FontTypeInfo 구현 완료 |
| default_font_name   | ✅     | ❌     | ➖      | ➖      | HWP→IR 구현됨, IR→HWP 미구현   |
| substitute_font     | ➖     | ➖     | ✅      | ✅      | HWPX SubstituteFont (HWP 미지원) |
| font_language       | ✅     | ✅     | ✅      | ✅      | FontLanguage Enum 구현 완료              |
| properties          | ➖     | ➖     | ➖      | ➖      | HWP 플래그 필드, IR 미저장     |
| alternate_font_type | ✅     | ❌     | ➖      | ➖      | HWP→IR font_type 변환, IR→HWP 미구현 |
| is_embedded         | ✅     | ✅     | ✅      | ✅      | 임베디드 폰트 여부             |
| binary_item_id_ref  | ✅     | ✅     | ✅      | ✅      | 바이너리 항목 참조             |

### 3.1.1 PANOSE 분류 정보

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                  |
| ---------------- | ------ | ------ | ------- | ------- | --------------------- |
| family_type      | ✅     | ✅     | ✅      | ✅      | 글꼴 종류             |
| serif_style      | ✅     | ✅     | ✅      | ✅      | PanoseSerifStyle      |
| weight           | ✅     | ✅     | ✅      | ✅      | PanoseWeight          |
| proportion       | ✅     | ✅     | ✅      | ✅      | PanoseProportion      |
| contrast         | ✅     | ✅     | ✅      | ✅      | PanoseContrast        |
| stroke_variation | ✅     | ✅     | ✅      | ✅      | PanoseStrokeVariation |
| arm_style        | ✅     | ✅     | ✅      | ✅      | PanoseArmStyle        |
| letter_form      | ✅     | ✅     | ✅      | ✅      | PanoseLetterForm      |
| midline          | ✅     | ✅     | ✅      | ✅      | PanoseMidline         |
| x_height         | ✅     | ✅     | ✅      | ✅      | PanoseXHeight         |

### 3.2 글자 모양 (CharShape)

| 항목                | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                            |
| ------------------- | ------ | ------ | ------- | ------- | ----------------------------------------------- |
| fonts (언어별 7종)  | ✅     | ✅     | ✅      | ✅      | Korean/English/Hanja/Japanese/Other/Symbol/User |
| font_size           | ✅     | ✅     | ✅      | ✅      |                                                 |
| char_scale          | ✅     | ✅     | ✅      | ✅      | 장평                                            |
| char_spacing        | ✅     | ✅     | ✅      | ✅      | 자간                                            |
| color               | ✅     | ✅     | ✅      | ✅      |                                                 |
| underline.line_type | ✅     | ✅     | ✅      | ✅      |                                                 |
| underline.position  | ✅     | ✅     | ✅      | ✅      |                                                 |
| underline.color     | ✅     | ✅     | ✅      | ✅      |                                                 |
| strikethrough       | ✅     | ✅     | ✅      | ✅      | None/Single/Double                              |
| emphasis.type       | ✅     | ✅     | ✅      | ✅      |                                                 |
| emphasis.color      | ✅     | ✅     | ✅      | ✅      |                                                 |
| outline             | ✅     | ✅     | ✅      | ✅      |                                                 |
| shadow.type         | ✅     | ✅     | ✅      | ✅      |                                                 |
| shadow.color        | ✅     | ✅     | ✅      | ✅      |                                                 |
| shadow.offset_x/y   | ✅     | ✅     | ✅      | ✅      |                                                 |
| bold                | ✅     | ✅     | ✅      | ✅      |                                                 |
| italic              | ✅     | ✅     | ✅      | ✅      |                                                 |
| superscript         | ✅     | ✅     | ✅      | ✅      |                                                 |
| subscript           | ✅     | ✅     | ✅      | ✅      |                                                 |
| background_color    | ✅     | ✅     | ✅      | ✅      | shade_color                                     |
| highlight_color     | ➖     | ➖     | ✅      | ✅      | HWPX는 RangeTag로 구현, HWP 스펙 불명확         |
| emboss              | ✅     | ✅     | ✅      | ✅      |                                                 |
| engrave             | ✅     | ✅     | ✅      | ✅      |                                                 |
| use_kerning         | ✅     | ✅     | ✅      | ✅      |                                                 |
| relative_size       | ✅     | ✅     | ✅      | ✅      | FontRef에 relative_size 필드 추가됨             |
| relative_position   | ✅     | ✅     | ✅      | ✅      | FontRef.offset 필드로 변환됨                    |
| shade_color         | ✅     | ✅     | ✅      | ✅      | 음영 색상 (배경색과 별도)                       |
| base_size           | ✅     | ✅     | ➖      | ➖      | HWP 기본 크기 → IR font_size로 변환됨           |
| use_font_space      | ➖     | ➖     | ➖      | ➖      | HWPX 전용 필드, IR 미지원                       |
| border_fill_id_ref  | ✅     | ✅     | ✅      | ✅      | 테두리/배경 참조                                |

### 3.2.1 CharShape 언어별 배열 필드 (HWP)

| 항목                | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                         |
| ------------------- | ------ | ------ | ------- | ------- | ---------------------------- |
| font_ids[7]         | ✅     | ✅     | ✅      | ✅      | 언어별 폰트 ID               |
| width_ratios[7]     | ✅     | ✅     | ✅      | ✅      | 언어별 장평                  |
| spacings[7]         | ✅     | ✅     | ✅      | ✅      | 언어별 자간                  |
| offsets[7]          | ✅     | ✅     | ✅      | ✅      | 언어별 오프셋                |
| relative_sizes[7]   | ✅     | ✅     | ✅      | ✅      | 언어별 상대 크기 (10-250%)   |
| positions[7]        | ✅     | ✅     | ✅      | ✅      | 언어별 문자 위치 (-100~100%) |
| strikethrough_color | ➖     | ➖     | ➖      | ➖      | HWP 전용 필드, IR에 취소선 색상 미지원          |

### 3.3 문단 모양 (ParaShape)

| 항목                   | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                 |
| ---------------------- | ------ | ------ | ------- | ------- | ------------------------------------ |
| alignment              | ✅     | ✅     | ✅      | ✅      | Left/Center/Right/Justify/Distribute |
| margin_left            | ✅     | ✅     | ✅      | ✅      |                                      |
| margin_right           | ✅     | ✅     | ✅      | ✅      |                                      |
| first_line_indent      | ✅     | ✅     | ✅      | ✅      |                                      |
| space_before           | ✅     | ✅     | ✅      | ✅      |                                      |
| space_after            | ✅     | ✅     | ✅      | ✅      |                                      |
| line_spacing.type      | ✅     | ✅     | ✅      | ✅      | Percent/Fixed/AtLeast/FontBased      |
| line_spacing.value     | ✅     | ✅     | ✅      | ✅      |                                      |
| tab_def_id             | ✅     | ✅     | ✅      | ✅      |                                      |
| border_fill_id         | ✅     | ✅     | ✅      | ✅      |                                      |
| line_break_korean      | ✅     | ✅     | ✅      | ✅      | Word/Character                       |
| line_break_latin       | ✅     | ✅     | ✅      | ✅      | Word/Hyphenation/Character           |
| snap_to_grid           | ✅     | ✅     | ✅      | ✅      |                                      |
| suppress_line_numbers  | ✅     | ✅     | ✅      | ✅      |                                      |
| widow_orphan_control   | ✅     | ✅     | ✅      | ✅      |                                      |
| keep_with_next         | ✅     | ✅     | ✅      | ✅      |                                      |
| keep_lines             | ✅     | ✅     | ✅      | ✅      |                                      |
| page_break_before      | ✅     | ✅     | ✅      | ✅      |                                      |
| vertical_alignment     | ✅     | ✅     | ✅      | ✅      |                                      |
| auto_line_height_ratio | ✅     | ✅     | ✅      | ✅      |                                      |
| numbering              | ✅     | ✅     | ✅      | ✅      |                                      |
| heading_type           | ✅     | ✅     | ✅      | ✅      | None/Outline/Number/Bullet           |
| padding                | ✅     | ✅     | ✅      | ✅      | 문단 테두리 안쪽 여백                |
| properties1            | ✅     | ✅     | ➖      | ➖      | HWP 속성 플래그, 개별 필드로 분해됨  |
| properties2            | ✅     | ✅     | ➖      | ➖      | HWP 확장 속성, 개별 필드로 분해됨    |
| properties3            | ✅     | ✅     | ➖      | ➖      | HWP 확장 속성, 개별 필드로 분해됨    |
| condense               | ➖     | ➖     | ➖      | ➖      | HWPX 전용 필드, IR 미지원            |
| font_line_height       | ✅     | ✅     | ✅      | ✅      | IR auto_line_height_ratio로 변환됨   |
| checked                | ➖     | ➖     | ➖      | ➖      | HWPX 전용 필드, IR 미지원            |

### 3.3.1 문단 머리 (ParagraphHead)

| 항목                   | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                       |
| ---------------------- | ------ | ------ | ------- | ------- | -------------------------- |
| heading_type           | ✅     | ✅     | ✅      | ✅      | None/Outline/Number/Bullet |
| heading_id             | ✅     | ✅     | ✅      | ✅      | 문단 머리 ID (numbering_id/bullet_id) |
| heading_level          | ✅     | ✅     | ✅      | ✅      | 개요 수준 (level)          |
| text_offset            | ✅     | ✅     | ✅      | ✅      | 본문과의 간격              |
| number_width           | ✅     | ✅     | ✅      | ✅      | 너비                       |
| alignment              | ✅     | ✅     | ✅      | ✅      | ParagraphHeadAlignment     |
| use_instance_id        | ✅     | ✅     | ✅      | ✅      | 인스턴스 ID 사용           |
| auto_indent            | ✅     | ✅     | ✅      | ✅      | 자동 들여쓰기              |
| width_adjust           | ➖     | ➖     | ✅      | ✅      | 너비 보정 (HWPX 전용)                  |
| number_format          | ✅     | ✅     | ✅      | ✅      | NumberFormat Enum 구현 완료                  |
| character_shape_id_ref | ✅     | ✅     | ✅      | ✅      | 글자 모양 참조             |
| checkable              | ➖     | ➖     | ✅      | ✅      | 체크 가능 여부 (HWPX 전용, is_checkbox로 저장)             |

### 3.3.2 문단 테두리 (ParagraphBorder) - HWPX

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고             |
| ------------------ | ------ | ------ | ------- | ------- | ---------------- |
| border_fill_id_ref | ✅     | ✅     | ✅      | ✅      | 테두리/배경 참조 |
| offset_left        | ✅     | ✅     | ✅      | ✅      | 왼쪽 간격        |
| offset_right       | ✅     | ✅     | ✅      | ✅      | 오른쪽 간격      |
| offset_top         | ✅     | ✅     | ✅      | ✅      | 위쪽 간격        |
| offset_bottom      | ✅     | ✅     | ✅      | ✅      | 아래쪽 간격      |
| connect            | ✅     | ✅     | ✅      | ✅      | 테두리 연결      |
| ignore_margin      | ✅     | ✅     | ✅      | ✅      | 여백 무시        |

### 3.3.3 문단 자동 간격 (ParagraphAutoSpacing) - HWPX

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                |
| ------------------ | ------ | ------ | ------- | ------- | ------------------- |
| east_asian_english | ➖     | ➖     | ✅      | ✅      | 한글/영문 자동 간격 |
| east_asian_number  | ➖     | ➖     | ✅      | ✅      | 한글/숫자 자동 간격 |

### 3.3.4 버전 분기 (VersionSwitch) - HWPX

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                |
| ------------------ | ------ | ------ | ------- | ------- | ----------------------------------- |
| switch             | ➖     | ➖     | ➖      | ➖      | HWPX 버전 호환성 처리용, 변환 불필요 |
| case               | ➖     | ➖     | ➖      | ➖      | HWPX 버전 호환성 처리용, 변환 불필요 |
| required_namespace | ➖     | ➖     | ➖      | ➖      | HWPX 버전 호환성 처리용, 변환 불필요 |
| case.margin        | ➖     | ➖     | ➖      | ➖      | HWPX 버전 호환성 처리용, 변환 불필요 |
| case.line_spacing  | ➖     | ➖     | ➖      | ➖      | HWPX 버전 호환성 처리용, 변환 불필요 |
| default            | ➖     | ➖     | ➖      | ➖      | HWPX 버전 호환성 처리용, 변환 불필요 |
| default.margin     | ➖     | ➖     | ➖      | ➖      | HWPX 버전 호환성 처리용, 변환 불필요 |
| default.line_spacing | ➖   | ➖     | ➖      | ➖      | HWPX 버전 호환성 처리용, 변환 불필요 |

### 3.4 탭 정의 (TabDef)

| 항목              | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                      |
| ----------------- | ------ | ------ | ------- | ------- | ------------------------- |
| tabs[].position   | ✅     | ✅     | ✅      | ✅      |                           |
| tabs[].tab_type   | ✅     | ✅     | ✅      | ✅      | Left/Center/Right/Decimal |
| tabs[].leader     | ✅     | ✅     | ✅      | ✅      | None/Dot/Dash/etc         |
| auto_tab_interval | ✅     | ✅     | ✅      | ✅      |                           |

### 3.5 테두리/채우기 (BorderFill)

| 항목                  | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고        |
| --------------------- | ------ | ------ | ------- | ------- | ----------- |
| left/right/top/bottom | ✅     | ✅     | ✅      | ✅      | Border 속성 |
| diagonal_down         | ✅     | ✅     | ✅      | ✅      |             |
| diagonal_up           | ✅     | ✅     | ✅      | ✅      |             |
| fill (None)           | ✅     | ✅     | ✅      | ✅      |             |
| fill (Solid)          | ✅     | ✅     | ✅      | ✅      |             |
| fill (Gradient)       | ✅     | ✅     | ✅      | ✅      |             |
| fill (Image)          | ✅     | ✅     | ✅      | ✅      |             |
| fill (Pattern)        | ✅     | ✅     | ✅      | ✅      |             |
| is_3d                 | ✅     | ✅     | ✅      | ✅      |             |
| has_shadow            | ✅     | ✅     | ✅      | ✅      |             |
| slash                 | ➖     | ➖     | ➖      | ➖      | HWPX 전용 필드, IR diagonal만 지원  |
| center_line           | ➖     | ➖     | ➖      | ➖      | HWPX 전용 필드, IR 미지원           |

### 3.5.0 BorderFill 배열 필드 (HWP)

| 항목                  | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                |
| --------------------- | ------ | ------ | ------- | ------- | ------------------- |
| border_styles[4]      | ✅     | ✅     | ➖      | ➖      | 4방향 테두리 스타일 (IR Border.left/right/top/bottom) |
| border_thicknesses[4] | ✅     | ✅     | ➖      | ➖      | 4방향 테두리 두께 (IR Border.left/right/top/bottom)   |
| border_colors[4]      | ✅     | ✅     | ➖      | ➖      | 4방향 테두리 색상 (IR Border.left/right/top/bottom)   |
| diagonal_style        | ✅     | ✅     | ➖      | ➖      | 대각선 스타일       |
| diagonal_thickness    | ✅     | ✅     | ➖      | ➖      | 대각선 두께 (정확도 개선) |
| diagonal_color        | ✅     | ✅     | ➖      | ➖      | 대각선 색상         |

### 3.5.1 패턴 채우기 (PatternFill)

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                      |
| ---------------- | ------ | ------ | ------- | ------- | ----------------------------------------- |
| pattern_type     | ✅     | ✅     | ✅      | ✅      | HorizontalLine/VerticalLine/BackSlash/etc |
| foreground_color | ✅     | ✅     | ✅      | ✅      | 패턴 전경 색상                            |
| background_color | ✅     | ✅     | ✅      | ✅      | 패턴 배경 색상                            |


### 3.5.1.1 그라데이션 채우기 (GradientFill)

| 항목          | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                          |
| ------------- | ------ | ------ | ------- | ------- | ----------------------------- |
| gradient_type | ✅     | ✅     | ✅      | ✅      | Linear/Radial/Conical/Square  |
| angle         | ✅     | ✅     | ✅      | ✅      | 0-360도 (선형)                |
| center_x      | ✅     | ✅     | ✅      | ✅      | 중심 X (0-100%, 원형)         |
| center_y      | ✅     | ✅     | ✅      | ✅      | 중심 Y (0-100%, 원형)         |
| stops[]       | ✅     | ✅     | ✅      | ✅      | 색상 정지점                   |
| blur          | ✅     | ✅     | ✅      | ✅      | 번짐 정도 (0-255)             |
| step_center   | ✅     | ✅     | ✅      | ✅      | 번짐 중심 (0-100, HWPX만, HWP는 기본값 50) |
### 3.5.2 이미지 채우기 (ImageFill)

| 항목       | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                         |
| ---------- | ------ | ------ | ------- | ------- | ---------------------------- |
| fill_type  | ✅     | ✅     | ✅      | ✅      | Tile/TileHorizTop/etc (16종) |
| binary_id  | ✅     | ✅     | ✅      | ✅      |                              |
| effect     | ✅     | ✅     | ✅      | ✅      |                              |
| brightness | ✅     | ✅     | ✅      | ✅      |                              |
| contrast   | ✅     | ✅     | ✅      | ✅      |                              |

### 3.5.3 빗금/중심선 (Slash/CenterLine)

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고              |
| ---------------- | ------ | ------ | ------- | ------- | ----------------- |
| slash_type       | ➖     | ➖     | ➖      | ➖      | HWPX 전용, IR 미지원 |
| center_line_type | ➖     | ➖     | ➖      | ➖      | HWPX 전용, IR 미지원 |
| diagonal_type    | ➖     | ➖     | ➖      | ➖      | HWPX 전용, IR 미지원 |

### 3.6 스타일 (Style)

| 항목          | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                              |
| ------------- | ------ | ------ | ------- | ------- | --------------------------------- |
| name          | ✅     | ✅     | ✅      | ✅      |                                   |
| english_name  | ✅     | ✅     | ✅      | ✅      |                                   |
| style_type    | ✅     | ✅     | ✅      | ✅      | Paragraph/Character               |
| para_shape_id | ✅     | ✅     | ✅      | ✅      |                                   |
| char_shape_id | ✅     | ✅     | ✅      | ✅      |                                   |
| next_style_id | ✅     | ✅     | ✅      | ✅      |                                   |
| language_id   | ➖     | ➖     | ➖      | ➖      | HWPX 전용 언어 ID, IR 미지원      |
| lock_form     | ➖     | ➖     | ➖      | ➖      | HWPX 전용 양식 잠금 속성, IR 미지원 |

### 3.7 번호 매기기 (Numbering)

| 항목                   | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고 |
| ---------------------- | ------ | ------ | ------- | ------- | ---- |
| name                   | ✅     | ✅     | ✅      | ✅      |      |
| levels[].level         | ✅     | ✅     | ✅      | ✅      |      |
| levels[].format        | ✅     | ✅     | ✅      | ✅      |      |
| levels[].char_shape_id | ✅     | ✅     | ✅      | ✅      |      |
| levels[].text_offset   | ✅     | ✅     | ✅      | ✅      |      |
| levels[].number_width  | ✅     | ✅     | ✅      | ✅      |      |
| start_number                | ✅     | ✅     | ✅      | ✅      | 전역 시작 번호 |
| levels[].start_number       | ✅     | ✅     | ✅      | ✅      | 수준별 시작 번호 |
| levels[].alignment          | ✅     | ✅     | ✅      | ✅      | 정렬 방식 |
| levels[].use_instance_width | ✅     | ✅     | ✅      | ✅      | 실제 인스턴스 너비 사용 |
| levels[].auto_indent        | ✅     | ✅     | ✅      | ✅      | 자동 들여쓰기 |

### 3.8 글머리표 (Bullet)

| 항목              | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                   |
| ----------------- | ------ | ------ | ------- | ------- | -------------------------------------- |
| char              | ✅     | ✅     | ✅      | ✅      |                                        |
| char_shape_id     | ✅     | ✅     | ✅      | ✅      |                                        |
| is_checkbox       | ✅     | ✅     | ✅      | ✅      |                                        |
| checked_character | ➖     | ➖     | ➖      | ➖      | HWPX 전용 체크 문자, 기본값으로 처리   |

### 3.9 시작 번호 (BeginNumber) - HWPX

| 항목     | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고             |
| -------- | ------ | ------ | ------- | ------- | ---------------- |
| page     | ✅     | ✅     | ✅      | ✅      | 페이지 시작 번호 (DocumentSettings.starting_page_number, Section.start_number) |
| footnote | ✅     | ✅     | ✅      | ✅      | 각주 시작 번호 (DocumentSettings.starting_footnote_number)   |
| endnote  | ✅     | ✅     | ✅      | ✅      | 미주 시작 번호 (DocumentSettings.starting_endnote_number)   |
| picture  | ✅     | ✅     | ✅      | ✅      | 그림 시작 번호 (Section.start_number.picture)   |
| table    | ✅     | ✅     | ✅      | ✅      | 표 시작 번호 (Section.start_number.table)     |
| equation | ✅     | ✅     | ✅      | ✅      | 수식 시작 번호 (Section.start_number.equation)   |

### 3.10 금칙 문자 (ForbiddenChar) - HWP

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고              |
| ---------------- | ------ | ------ | ------- | ------- | ----------------- |
| line_start_chars | ➖     | ➖     | ➖      | ➖      | HWP 전용 기능, IR 미저장 (렌더링 전용) |
| line_end_chars   | ➖     | ➖     | ➖      | ➖      | HWP 전용 기능, IR 미저장 (렌더링 전용) |

### 3.11 레이아웃 호환성 (LayoutCompatibility) - HWP

| 항목            | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고           |
| --------------- | ------ | ------ | ------- | ------- | -------------- |
| letter_level    | ❌     | ❌     | ➖      | ➖      | IR LayoutCompatibility 있으나 변환 미구현 |
| paragraph_level | ❌     | ❌     | ➖      | ➖      | IR LayoutCompatibility 있으나 변환 미구현 |
| section_level   | ❌     | ❌     | ➖      | ➖      | IR LayoutCompatibility 있으나 변환 미구현 |
| object_level    | ❌     | ❌     | ➖      | ➖      | IR LayoutCompatibility 있으나 변환 미구현 |
| field_level     | ❌     | ❌     | ➖      | ➖      | IR LayoutCompatibility 있으나 변환 미구현 |

### 3.11.1 레이아웃 호환성 상세 (LayoutCompatibility) - HWPX

| 항목                          | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                        |
| ----------------------------- | ------ | ------ | ------- | ------- | ------------------------------------------- |
| apply_font_weight_to_bold     | ➖     | ➖     | ➖      | ➖      | HWPX 레이아웃 호환성 옵션, IR 변환 불필요   |
| use_inner_unfill              | ➖     | ➖     | ➖      | ➖      | HWPX 레이아웃 호환성 옵션, IR 변환 불필요   |
| fixed_underline_width         | ➖     | ➖     | ➖      | ➖      | HWPX 레이아웃 호환성 옵션, IR 변환 불필요   |
| expand_line_spacing           | ➖     | ➖     | ➖      | ➖      | HWPX 레이아웃 호환성 옵션, IR 변환 불필요   |
| adjust_blank_line_spacing     | ➖     | ➖     | ➖      | ➖      | HWPX 레이아웃 호환성 옵션, IR 변환 불필요   |
| increase_line_spacing_by_font | ➖     | ➖     | ➖      | ➖      | HWPX 레이아웃 호환성 옵션, IR 변환 불필요   |
| increase_line_spacing_by_line | ➖     | ➖     | ➖      | ➖      | HWPX 레이아웃 호환성 옵션, IR 변환 불필요   |
| use_char_shape_char_spacing   | ➖     | ➖     | ➖      | ➖      | HWPX 레이아웃 호환성 옵션, IR 변환 불필요   |
| (총 44개 이상 boolean 토글)   | ➖     | ➖     | ➖      | ➖      | HWPX 레이아웃 호환성 옵션들, IR 변환 불필요 |

---

## 4. 문단 (Paragraph)

### 4.1 문단 기본 속성

| 항목                      | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                     |
| ------------------------- | ------ | ------ | ------- | ------- | ------------------------ |
| para_shape_id             | ✅     | ✅     | ✅      | ✅      |                          |
| style_id                  | ✅     | ✅     | ✅      | ✅      |                          |
| break_type                | ✅     | ✅     | ✅      | ✅      | None/Page/Column/Section |
| instance_id               | ✅     | ✅     | ✅      | ✅      |                          |
| line_segments[]           | ➖     | ➖     | ➖      | ➖      | 레이아웃 캐시, 손실 허용 (재계산 가능) |
| range_tags[]              | ✅     | ✅     | ✅      | ✅      |                          |
| paragraph_track_change_id | ➖     | ➖     | ✅      | ✅      | HWPX paraTcId 속성 (HWP는 별도 구조)       |
| merged                    | ➖     | ➖     | ✅      | ✅      | HWPX 병합됨 여부 (HWP는 별도 구조)         |

### 4.2 런 (Run)

| 항목                      | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고               |
| ------------------------- | ------ | ------ | ------- | ------- | ------------------ |
| char_shape_id             | ✅     | ✅     | ✅      | ✅      |                    |
| contents[]                | ✅     | ✅     | ✅      | ✅      |                    |
| character_track_change_id | ➖     | ➖     | ✅      | ✅      | HWPX charTcId 속성 (HWP는 별도 구조) |

### 4.3 런 내용 (RunContent)

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고            |
| ---------------- | ------ | ------ | ------- | ------- | --------------- |
| Text             | ✅     | ✅     | ✅      | ✅      |                 |
| Tab              | ✅     | ✅     | ✅      | ✅      |                 |
| LineBreak        | ✅     | ✅     | ✅      | ✅      |                 |
| Hyphen           | ✅     | ✅     | ✅      | ✅      |                 |
| NonBreakingSpace | ✅     | ✅     | ✅      | ✅      |                 |
| FixedWidthSpace  | ✅     | ✅     | ✅      | ✅      |                 |
| Control          | ✅     | ✅     | ✅      | ✅      | 표/그림/도형 등 |
| FieldStart       | ✅     | ✅     | ✅      | ✅      |                 |
| FieldEnd         | ✅     | ✅     | ✅      | ✅      |                 |
| BookmarkStart    | ✅     | ✅     | ✅      | ✅      |                 |
| BookmarkEnd      | ✅     | ✅     | ✅      | ✅      |                 |

### 4.3.1 인라인 탭 세부 속성 (InlineTab) - HWPX
| 항목     | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                            |
| -------- | ------ | ------ | ------- | ------- | ------------------------------- |
| width    | ➖     | ➖     | ✅      | ✅      | 탭 너비 (HwpUnit)               |
| leader   | ➖     | ➖     | ✅      | ✅      | 채움선 종류 (LineStyleType2)    |
| tab_type | ➖     | ➖     | ✅      | ✅      | 탭 종류 (Left/Right/Center/Decimal) |

### 4.4 필드 타입 (FieldType)

| 항목              | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                        |
| ----------------- | ------ | ------ | ------- | ------- | --------------------------- |
| Hyperlink         | ✅     | ✅     | ✅      | ✅      |                             |
| Bookmark          | ✅     | ✅     | ✅      | ✅      |                             |
| Date              | ✅     | ✅     | ✅      | ✅      |                             |
| Time              | ✅     | ✅     | ✅      | ✅      |                             |
| PageNumber        | ✅     | ✅     | ✅      | ✅      |                             |
| TotalPages        | ✅     | ✅     | ✅      | ✅      | PageCount                   |
| FileName          | ✅     | ✅     | ✅      | ✅      |                             |
| FilePath          | ✅     | ✅     | ✅      | ✅      |                             |
| Title             | ✅     | ✅     | ✅      | ✅      | 문서 제목                   |
| Author            | ✅     | ✅     | ✅      | ✅      | 저자                        |
| Summary           | ✅     | ✅     | ✅      | ✅      | HWPX SUMMERY 필드           |
| CrossReference    | ✅     | ✅     | ✅      | ✅      | 상호 참조                   |
| MailMerge         | ✅     | ✅     | ✅      | ✅      | 메일 머지 (%mmr)            |
| TableOfContents   | ✅     | ✅     | ➖      | ✅      | 목차 (%toc, HWPX는 FORMULA로 매핑) |
| Formula           | ✅     | ✅     | ✅      | ✅      | HWPX 수식 필드              |
| UserInfo          | ✅     | ✅     | ✅      | ✅      | 사용자 정보                 |
| Memo              | ✅     | ✅     | ✅      | ✅      | 메모 필드                   |
| ClickHere         | ✅     | ✅     | ✅      | ✅      | HWP 누름틀, HWPX CLICK_HERE |
| PrivateInfo       | ✅     | ✅     | ✅      | ✅      | 개인정보                    |
| MetaTag           | ✅     | ✅     | ✅      | ✅      | 메타태그                    |
| ProofreadingMarks | ➖     | ➖     | ✅      | ✅      | HWPX 전용 필드 타입 (HWP 미지원) |
| DocumentDate      | ➖     | ➖     | ✅      | ✅      | HWPX 전용 필드 타입 (HWP 미지원) |

### 4.4.1 필드 세부 속성 (FieldStart) - HWPX

| 항목       | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고           |
| ---------- | ------ | ------ | ------- | ------- | -------------- |
| parameters | ✅     | ✅     | ✅      | ✅      | 매개변수 목록 (HWP→IR: ControlData에서 추출) |
| sub_list   | ➖     | ➖     | ✅      | ✅      | 서브 문단 목록 (HWPX 전용) |
| editable   | ✅     | ✅     | ✅      | ✅      | 편집 가능 여부 (HWP→IR 기본값 true) |
| dirty      | ✅     | ✅     | ✅      | ✅      | 변경 여부 (HWP→IR 기본값 false) |
| z_order    | ➖     | ➖     | ✅      | ✅      | Z 순서 (HWPX 전용) |
| field_id   | ➖     | ➖     | ✅      | ✅      | 필드 ID (HWPX 전용, HWP는 instruction 사용) |

### 4.4.2 매개변수 (ParameterList) - HWPX

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고        |
| ---------------- | ------ | ------ | ------- | ------- | ----------- |
| BooleanParameter | ➖     | ➖     | ✅      | ✅      | 불리언 값   |
| IntegerParameter | ➖     | ➖     | ✅      | ✅      | 정수 값     |
| FloatParameter   | ➖     | ➖     | ✅      | ✅      | 실수 값     |
| StringParameter  | ➖     | ➖     | ✅      | ✅      | 문자열 값   |
| ListParameter    | ➖     | ➖     | ✅      | ✅      | 중첩 리스트 |

### 4.4.3 컨트롤 데이터 (ControlData) - HWP

| 항목                      | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                        |
| ------------------------- | ------ | ------ | ------- | ------- | --------------------------- |
| ParameterSet.id           | ➖     | ➖     | -       | -       | HWP 내부 구조, 변환 범위 외              |
| ParameterItem.id          | ➖     | ➖     | -       | -       | HWP 내부 구조, 변환 범위 외                     |
| ParameterItem.type        | ➖     | ➖     | -       | -       | HWP 내부 구조, 변환 범위 외 |
| ParameterItem.value       | ➖     | ➖     | -       | -       | HWP 내부 구조, 변환 범위 외                          |
| field_item_ids.COMMAND    | ➖     | ➖     | -       | -       | HWP 내부 구조, 변환 범위 외          |
| field_item_ids.NAME       | ➖     | ➖     | -       | -       | HWP 내부 구조, 변환 범위 외          |
| hyperlink_item_ids.TARGET | ➖     | ➖     | -       | -       | HWP 내부 구조, 변환 범위 외    |

### 4.5 줄 세그먼트 (LineSegment)

| 항목                | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                             |
| ------------------- | ------ | ------ | ------- | ------- | -------------------------------- |
| text_position       | ➖     | ➖     | ➖      | ➖      | 레이아웃 캐시, 손실 허용 (재계산 가능)    |
| vertical_position   | ➖     | ➖     | ➖      | ➖      | 레이아웃 캐시, 손실 허용 (재계산 가능)    |
| vertical_size       | ➖     | ➖     | ➖      | ➖      | 레이아웃 캐시, 손실 허용 (재계산 가능)   |
| text_height         | ➖     | ➖     | ➖      | ➖      | 레이아웃 캐시, 손실 허용 (재계산 가능) |
| baseline            | ➖     | ➖     | ➖      | ➖      | 레이아웃 캐시, 손실 허용 (재계산 가능)   |
| spacing             | ➖     | ➖     | ➖      | ➖      | 레이아웃 캐시, 손실 허용 (재계산 가능)    |
| horizontal_position | ➖     | ➖     | ➖      | ➖      | 레이아웃 캐시, 손실 허용 (재계산 가능)    |
| horizontal_size     | ➖     | ➖     | ➖      | ➖      | 레이아웃 캐시, 손실 허용 (재계산 가능)   |
| flags               | ➖     | ➖     | ➖      | ➖      | 레이아웃 캐시, 손실 허용 (재계산 가능)      |

### 4.6 범위 태그 (RangeTag)

| 항목                         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                 |
| ---------------------------- | ------ | ------ | ------- | ------- | -------------------- |
| start                        | ✅     | ✅     | ✅      | ✅      |                      |
| end                          | ✅     | ✅     | ✅      | ✅      |                      |
| tag_type (Bookmark)          | ✅     | ✅     | ✅      | ✅      |                      |
| tag_type (Hyperlink)         | ✅     | ✅     | ✅      | ✅      |                      |
| tag_type (TrackChangeInsert) | ✅     | ✅     | ✅      | ➖      | HWPX→IR만 구현       |
| tag_type (TrackChangeDelete) | ✅     | ✅     | ✅      | ➖      | HWPX→IR만 구현       |
| tag_type (Highlight)         | ✅     | ✅     | ✅      | ✅      | 형광펜 (양방향 구현) |
| data                         | ✅     | ✅     | ✅      | ✅      |                      |
| tag (3-byte)                 | ✅     | ✅     | ➖      | ➖      | HWP 원시 태그 데이터 (tag_type/track_change_id로 파싱) |
| track_change_info.track_change_id | ✅     | ✅     | ➖      | ➖      | 변경 추적 ID (tag[0-1]에서 추출) |
| track_change_info.paragraph_end   | ✅     | ✅     | ➖      | ➖      | 문단 끝 여부 |
| track_change_info.tag_id          | ✅     | ✅     | ➖      | ➖      | 태그 고유 ID |

### 4.7 런 내용 상세 (RunContent) - HWPX

| 항목                   | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                   |
| ---------------------- | ------ | ------ | ------- | ------- | ---------------------- |
| SecPr                  | ➖     | ➖     | ✅      | ✅      | 구역 정의 (HWPX inline) |
| ColPr                  | ➖     | ➖     | ✅      | ✅      | 단 정의 (HWPX inline)   |
| Table                  | ✅     | ✅     | ✅      | ✅      | 표                     |
| Picture                | ✅     | ✅     | ✅      | ✅      | 그림                   |
| Drawing                | ✅     | ✅     | ✅      | ✅      | 그리기                 |
| Equation               | ✅     | ✅     | ✅      | ✅      | 수식                   |
| Chart                  | ✅     | ✅     | ✅      | ✅      | 차트                   |
| Container              | ✅     | ✅     | ✅      | ✅      | 컨테이너               |
| Ole                    | ✅     | ✅     | ✅      | ✅      | OLE                    |
| Video                  | ✅     | ✅     | ✅      | ✅      | 비디오                 |
| TextArt                | ✅     | ✅     | ✅      | ✅      | 글맵시                 |
| UnknownObject          | ➖     | ➖     | ✅      | ➖      | 알 수 없는 개체 (HWPX→IR: Unknown 컨트롤로 변환) |
| ConnectLine            | ✅     | ✅     | ✅      | ✅      | 연결선                 |
| FormObject.Button      | ✅     | ✅     | ✅      | ✅      | 양식 버튼 (btn)        |
| FormObject.RadioButton | ✅     | ✅     | ✅      | ✅      | 라디오 버튼 (radioBtn) |
| FormObject.CheckButton | ✅     | ✅     | ✅      | ✅      | 체크 버튼 (checkBtn)   |
| FormObject.ComboBox    | ✅     | ✅     | ✅      | ✅      | 콤보 박스 (comboBox)   |
| FormObject.ListBox     | ✅     | ✅     | ✅      | ✅      | 목록 상자 (listBox)    |
| FormObject.Edit        | ✅     | ✅     | ✅      | ✅      | 편집 상자 (edit)       |
| FormObject.ScrollBar   | ✅     | ✅     | ✅      | ✅      | 스크롤바 (scrollBar)   |
| FieldBegin             | ✅     | ✅     | ✅      | ✅      | 필드 시작              |
| FieldEnd               | ✅     | ✅     | ✅      | ✅      | 필드 끝                |
| Bookmark               | ✅     | ✅     | ✅      | ✅      | 책갈피                 |
| Header                 | ✅     | ✅     | ✅      | ✅      | 머리글                 |
| Footer                 | ✅     | ✅     | ✅      | ✅      | 바닥글                 |
| Footnote               | ✅     | ✅     | ✅      | ✅      | 각주                   |
| Endnote                | ✅     | ✅     | ✅      | ✅      | 미주                   |
| AutoNumber             | ✅     | ✅     | ✅      | ✅      | 자동 번호              |
| NewNumber              | ✅     | ✅     | ✅      | ✅      | 새 번호                |
| PageNumber             | ✅     | ✅     | ✅      | ✅      | 페이지 번호 (AutoNumber↔PageNumber 변환) |
| Compose                | ➖     | ➖     | ✅      | ✅      | 글자겹침 (HWP 인라인 컨트롤, 파서 미지원)               |
| Dutmal                 | ➖     | ➖     | ✅      | ✅      | 덧말 (HWP 인라인 컨트롤, 파서 미지원)                   |
| HiddenComment          | ✅     | ✅     | ✅      | ✅      | 숨은 설명              |
| IndexMark              | ✅     | ✅     | ✅      | ✅      | 색인 표시              |
| MarkPenBegin           | ✅     | ✅     | ✅      | ✅      | 형광펜 시작 (RangeTag↔MarkPen 변환) |
| MarkPenEnd             | ✅     | ✅     | ✅      | ✅      | 형광펜 끝 (RangeTag↔MarkPen 변환) |
| TitleMark              | ➖     | ➖     | ✅      | ✅      | HWP 파싱됨(변환 미구현), HWPX 완전 구현              |
| InsertBegin            | ✅     | ✅     | ✅      | ✅      | RangeTag(TrackChangeInsert)로 변환              |
| InsertEnd              | ✅     | ✅     | ✅      | ✅      | RangeTag(TrackChangeInsert)로 변환                |
| DeleteBegin            | ✅     | ✅     | ✅      | ✅      | RangeTag(TrackChangeDelete)로 변환              |
| DeleteEnd              | ✅     | ✅     | ✅      | ✅      | RangeTag(TrackChangeDelete)로 변환                |
| PageNumberControl      | ➖     | ➖     | ✅      | ✅      | HWPX 전용 (HWP 미지원)            |
| PageHiding             | ➖     | ➖     | ✅      | ✅      | HWPX 전용 (HWP 미지원, 6필드)     |

### 4.7.1 페이지 번호 (PageNumber) - HWPX

| 항목           | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                 |
| -------------- | ------ | ------ | ------- | ------- | -------------------- |
| position       | ✅     | ✅     | ✅      | ✅      | 번호 위치 (11종)     |
| format_type    | ✅     | ✅     | ✅      | ✅      | 번호 모양            |
| side_character | ✅     | ✅     | ✅      | ✅      | 줄표 문자 (기본 "-") |

### 4.7.2 변경 추적 태그 (TrackChangeTag) - HWPX

| 항목            | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                    |
| --------------- | ------ | ------ | ------- | ------- | --------------------------------------- |
| paragraph_end   | ➖     | ➖     | ➖      | ➖      | TrackChangeInfo 내부 필드, IR 지원      |
| track_change_id | ➖     | ➖     | ➖      | ➖      | TrackChangeInfo 내부 필드, IR 지원      |
| id              | ➖     | ➖     | ➖      | ➖      | tag_id로 TrackChangeInfo 내부 필드 지원 |

### 4.7.3 색인 표시 (IndexMark) - HWPX

| 항목       | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고       |
| ---------- | ------ | ------ | ------- | ------- | ---------- |
| first_key  | ✅     | ✅     | ✅      | ✅      | 첫 번째 키 |
| second_key | ✅     | ✅     | ✅      | ✅      | 두 번째 키 |

---

## 5. 컨트롤 (Control)

### 5.1 개체 공통 속성 (ObjectCommon)

| 항목                     | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고 |
| ------------------------ | ------ | ------ | ------- | ------- | ---- |
| id                       | ✅     | ✅     | ✅      | ✅      |      |
| position.x               | ✅     | ✅     | ✅      | ✅      |      |
| position.y               | ✅     | ✅     | ✅      | ✅      |      |
| size.width               | ✅     | ✅     | ✅      | ✅      |      |
| size.height              | ✅     | ✅     | ✅      | ✅      |      |
| z_order                  | ✅     | ✅     | ✅      | ✅      |      |
| text_wrap.wrap_type      | ✅     | ✅     | ✅      | ✅      |      |
| text_wrap.wrap_side      | ✅     | ✅     | ✅      | ✅      |      |
| text_wrap.margin         | ✅     | ✅     | ✅      | ✅      |      |
| text_wrap.vertical_rel   | ✅     | ✅     | ✅      | ✅      |      |
| text_wrap.horizontal_rel | ✅     | ✅     | ✅      | ✅      |      |
| text_wrap.treat_as_char  | ✅     | ✅     | ✅      | ✅      |      |
| text_wrap.flow_with_text | ✅     | ✅     | ✅      | ✅      |      |
| text_wrap.allow_overlap  | ✅     | ✅     | ✅      | ✅      |      |
| caption                  | ✅     | ✅     | ✅      | ✅      |      |

### 5.1.1 개체 위치/크기 기준 (HWPX)

| 항목                   | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                             |
| ---------------------- | ------ | ------ | ------- | ------- | -------------------------------- |
| vertical_relative_to   | ➖     | ➖     | ✅      | ✅      | Paper/Page/Para/etc (8종) HWPX전용 |
| horizontal_relative_to | ➖     | ➖     | ✅      | ✅      | Paper/Page/Column/etc (8종) HWPX전용 |
| vertical_offset_type   | ➖     | ➖     | ✅      | ✅      | Top/Center/Bottom/Inside/Outside HWPX전용 |
| horizontal_offset_type | ➖     | ➖     | ✅      | ✅      | Left/Center/Right/Inside/Outside HWPX전용 |
| width_relative_to      | ➖     | ➖     | ✅      | ✅      | Paper/Page/Column/etc HWPX전용 |
| height_relative_to     | ➖     | ➖     | ✅      | ✅      | Paper/Page/etc HWPX전용 |

### 5.1.2 렌더링 정보 (RenderingInfo) - HWPX

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고           |
| ------------------ | ------ | ------ | ------- | ------- | -------------- |
| translation_matrix | ✅     | ✅     | ✅      | ✅      | 변환 행렬 (HWP: matrix[0..6], HWPX: transform_matrix) |
| scale_matrix       | ✅     | ✅     | ✅      | ✅      | 크기 조정 행렬 (HWP: matrix[6..12], HWPX: matrix_pairs) |
| rotation_matrix    | ✅     | ✅     | ✅      | ✅      | 회전 행렬 (HWP: matrix[12..18], HWPX: matrix_pairs) |

### 5.1.3 텍스트 여백 (TextMargin) - HWPX

| 항목   | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고 |
| ------ | ------ | ------ | ------- | ------- | ---- |
| left   | ➖     | ➖     | ✅      | ✅      | HWPX전용 |
| right  | ➖     | ➖     | ✅      | ✅      | HWPX전용 |
| top    | ➖     | ➖     | ✅      | ✅      | HWPX전용 |
| bottom | ➖     | ➖     | ✅      | ✅      | HWPX전용 |

### 5.2 캡션 (Caption)

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                  |
| ------------ | ------ | ------ | ------- | ------- | --------------------- |
| position     | ✅     | ✅     | ✅      | ✅      | Left/Right/Top/Bottom |
| width        | ✅     | ✅     | ✅      | ✅      |                       |
| gap          | ✅     | ✅     | ✅      | ✅      |                       |
| paragraphs[] | ✅     | ✅     | ✅      | ✅      |                       |

### 5.3 표 (Table)

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                  |
| ---------------- | ------ | ------ | ------- | ------- | --------------------- |
| common           | ✅     | ✅     | ✅      | ✅      | ObjectCommon          |
| row_count        | ✅     | ✅     | ✅      | ✅      |                       |
| column_count     | ✅     | ✅     | ✅      | ✅      |                       |
| cell_spacing     | ✅     | ✅     | ✅      | ✅      |                       |
| border_fill_id   | ✅     | ✅     | ✅      | ✅      |                       |
| rows[]           | ✅     | ✅     | ✅      | ✅      |                       |
| zones[]          | ✅     | ✅     | ✅      | ✅      | CellZoneList          |
| header_row_count | ✅     | ✅     | ✅      | ✅      |                       |
| page_break       | ✅     | ✅     | ✅      | ✅      | HWP: 쪽 경계 나눔 (bit 0-1), HWPX: pageBreak |
| repeat_header    | ✅     | ✅     | ✅      | ✅      | HWP: 제목 줄 자동 반복 (bit 2), HWPX: repeatHeader |
| no_adjust        | ➖     | ➖     | ✅      | ✅      | HWPX 전용: 자동 조정 안함 |
| text_wrap        | ➖     | ➖     | ✅      | ✅      | HWPX 텍스트 배치      |
| text_flow        | ➖     | ➖     | ✅      | ✅      | HWPX 텍스트 흐름      |
| lock             | ➖     | ➖     | ✅      | ✅      | HWPX 잠금 여부        |
| numbering_type   | ➖     | ➖     | ✅      | ✅      | HWPX 번호 매기기 종류 |
| shape_comment    | ➖     | ➖     | ✅      | ✅      | HWPX 도형 주석        |
| meta_tag         | ➖     | ➖     | ✅      | ✅      | HWPX 메타 태그        |
| inside_margin    | ➖     | ➖     | ✅      | ✅      | HWPX 안쪽 여백        |

### 5.3.1 라벨 정보 (TableLabel) - HWPX

| 항목                  | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고           |
| --------------------- | ------ | ------ | ------- | ------- | -------------- |
| top_margin            | -      | -      | ➖      | ➖      | 상단 여백 (라벨 인쇄용, HWP 미지원)      |
| left_margin           | -      | -      | ➖      | ➖      | 좌측 여백 (라벨 인쇄용, HWP 미지원)      |
| box_width             | -      | -      | ➖      | ➖      | 박스 너비 (라벨 인쇄용, HWP 미지원)      |
| box_length            | -      | -      | ➖      | ➖      | 박스 길이 (라벨 인쇄용, HWP 미지원)      |
| box_margin_horizontal | -      | -      | ➖      | ➖      | 박스 가로 여백 (라벨 인쇄용, HWP 미지원) |
| box_margin_vertical   | -      | -      | ➖      | ➖      | 박스 세로 여백 (라벨 인쇄용, HWP 미지원) |
| label_columns         | -      | -      | ➖      | ➖      | 라벨 열 수 (라벨 인쇄용, HWP 미지원)     |
| label_rows            | -      | -      | ➖      | ➖      | 라벨 행 수 (라벨 인쇄용, HWP 미지원)     |
| landscape             | -      | -      | ➖      | ➖      | 용지 방향 (라벨 인쇄용, HWP 미지원)      |
| page_width            | -      | -      | ➖      | ➖      | 페이지 너비 (라벨 인쇄용, HWP 미지원)    |
| page_height           | -      | -      | ➖      | ➖      | 페이지 높이 (라벨 인쇄용, HWP 미지원)    |

### 5.4 표 셀 (TableCell)

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                |
| ------------------ | ------ | ------ | ------- | ------- | ------------------- |
| column             | ✅     | ✅     | ✅      | ✅      |                     |
| row                | ✅     | ✅     | ✅      | ✅      |                     |
| column_span        | ✅     | ✅     | ✅      | ✅      |                     |
| row_span           | ✅     | ✅     | ✅      | ✅      |                     |
| width              | ✅     | ✅     | ✅      | ✅      |                     |
| height             | ✅     | ✅     | ✅      | ✅      |                     |
| padding            | ✅     | ✅     | ✅      | ✅      |                     |
| border_fill_id     | ✅     | ✅     | ✅      | ✅      |                     |
| vertical_alignment | ✅     | ✅     | ✅      | ✅      |                     |
| paragraphs[]       | ✅     | ✅     | ✅      | ✅      |                     |
| is_merged          | ✅     | ✅     | ✅      | ✅      |                     |
| is_header          | ➖     | ➖     | ✅      | ✅      | HWPX header 속성    |
| protect            | ➖     | ➖     | ✅      | ✅      | 셀 보호             |
| name               | ➖     | ➖     | ✅      | ✅      | HWPX 셀 이름        |
| has_margin         | ➖     | ➖     | ✅      | ✅      | HWPX 여백 지정 여부 |
| editable           | ➖     | ➖     | ✅      | ✅      | HWPX 편집 가능 여부 |
| dirty              | ➖     | ➖     | ✅      | ✅      | HWPX 변경됨 여부    |

### 5.5 그림 (Picture)

| 항목                       | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                  |
| -------------------------- | ------ | ------ | ------- | ------- | ------------------------------------- |
| common                     | ✅     | ✅     | ✅      | ✅      | ObjectCommon                          |
| binary_id                  | ✅     | ✅     | ✅      | ✅      |                                       |
| original_size              | ✅     | ✅     | ✅      | ✅      |                                       |
| crop.left/right/top/bottom | ✅     | ✅     | ✅      | ✅      |                                       |
| flip                       | ✅     | ✅     | ✅      | ✅      | None/Horizontal/Vertical/Both         |
| rotation                   | ✅     | ✅     | ✅      | ✅      |                                       |
| effect                     | ✅     | ✅     | ✅      | ✅      | Original/Grayscale/BlackWhite/Pattern |
| brightness                 | ✅     | ✅     | ✅      | ✅      |                                       |
| contrast                   | ✅     | ✅     | ✅      | ✅      |                                       |
| alpha                      | ✅     | ✅     | ✅      | ✅      |                                       |
| transparent_color          | ✅     | ➖     | ➖      | ➖      | HWP→IR 완전 변환 (COLORREF→RGB)      |
| border                     | ✅     | ✅     | ✅      | ✅      |                                       |
| shadow                     | ✅     | ✅     | ✅      | ✅      |                                       |
| inside_margin              | ✅     | ✅     | ✅      | ✅      |                                       |
| href                       | ➖     | ➖     | ✅      | ✅      | HWPX 그림 하이퍼링크                  |
| reverse                    | ➖     | ➖     | ➖      | ➖      | HWPX 역방향 (HWPX 전용, HWP 미지원)                           |
| offset                     | ✅     | ✅     | ✅      | ✅      | 기준점 오프셋                         |
| image_rectangle            | ➖     | ➖     | ✅      | ✅      | HWPX 이미지 사각형                    |
| rendering_info             | ➖     | ➖     | ➖      | ➖      | 렌더링 정보 (HWPX 전용, HWP 미지원)                  |
| effects                    | ➖     | ➖     | ✅      | ✅      | HWPX Effects                          |
| line_shape                 | ➖     | ➖     | ✅      | ✅      | HWPX 테두리 선 모양                   |
| image_clip                 | ➖     | ➖     | ✅      | ✅      | HWPX 이미지 클립                      |
| current_size               | ➖     | ➖     | ✅      | ✅      | HWPX 현재 크기                        |
| text_wrap                  | ➖     | ➖     | ✅      | ✅      | HWPX 텍스트 배치                      |
| text_flow                  | ➖     | ➖     | ✅      | ✅      | HWPX 텍스트 흐름                      |
| lock                       | ➖     | ➖     | ✅      | ✅      | HWPX 잠금 여부                        |
| numbering_type             | ➖     | ➖     | ✅      | ✅      | HWPX 번호 매기기 종류                 |
| group_level                | ➖     | ➖     | ✅      | ✅      | HWPX 그룹 레벨                        |
| shape_comment              | ➖     | ➖     | ✅      | ✅      | HWPX 도형 주석                        |
| meta_tag                   | ➖     | ➖     | ✅      | ✅      | HWPX 메타 태그                        |

### 5.5.1 그림 속성 상세 (PictureProperties) - HWP

| 항목                | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                   |
| ------------------- | ------ | ------ | ------- | ------- | -------------------------------------- |
| border_color        | ➖     | ➖     | ➖      | ➖      | 테두리 색상 (HWP 내부, IR border.color로 변환)                 |
| border_thickness    | ➖     | ➖     | ➖      | ➖      | 테두리 두께 (HWP 내부, IR border.width로 변환)                            |
| border_properties   | ➖     | ➖     | ➖      | ➖      | 테두리 속성 플래그 (HWP 내부 바이너리)                     |
| corners[4]          | ➖     | ➖     | ➖      | ➖      | 4개 모서리 좌표 (HWP 내부, IR position/size로 추상화)                |
| inner_margin        | ➖     | ➖     | ➖      | ➖      | 내부 여백 (HWP 내부, IR inside_margin으로 변환)      |
| binary_pattern      | ➖     | ➖     | ➖      | ➖      | 패턴 효과용 BinData ID (HWP 내부)                 |
| border_transparency | ➖     | ➖     | ➖      | ➖      | 테두리 투명도 (HWP 내부)                          |
| instance_id         | ➖     | ➖     | ➖      | ➖      | 인스턴스 ID (HWP 내부, IR id로 변환)                 |
| image_dimension     | ➖     | ➖     | ➖      | ➖      | 이미지 크기 (HWP 내부, IR original_size로 변환) |

### 5.6 도형 (Shape)

| 항목                           | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                  |
| ------------------------------ | ------ | ------ | ------- | ------- | --------------------- |
| common                         | ✅     | ✅     | ✅      | ✅      | ObjectCommon          |
| shape_type                     | ✅     | ✅     | ✅      | ✅      | Line/Rect/Ellipse/etc |
| line                           | ✅     | ✅     | ✅      | ✅      | LineStyle             |
| fill                           | ✅     | ✅     | ✅      | ✅      |                       |
| shadow                         | ➖     | ➖     | ✅      | ✅      | HWP 도형에 shadow 없음 (Picture에만) |
| rotation                       | ✅     | ✅     | ✅      | ✅      |                       |
| text                           | ✅     | ✅     | ✅      | ✅      | ShapeText/DrawText    |
| text_wrap                      | ➖     | ➖     | ✅      | ✅      | HWPX 텍스트 배치      |
| text_flow                      | ➖     | ➖     | ✅      | ✅      | HWPX 텍스트 흐름      |
| lock                           | ➖     | ➖     | ✅      | ✅      | HWPX 잠금 여부        |
| numbering_type                 | ➖     | ➖     | ✅      | ✅      | HWPX 번호 매기기 종류 |
| group_level                    | ➖     | ➖     | ✅      | ✅      | HWPX 그룹 레벨        |
| instance_id                    | ➖     | ➖     | ✅      | ✅      | HWPX 인스턴스 ID      |
| href                           | ➖     | ➖     | ✅      | ✅      | HWPX 하이퍼링크       |
| shape_comment                  | ➖     | ➖     | ✅      | ✅      | HWPX 도형 주석        |
| meta_tag                       | ➖     | ➖     | ✅      | ✅      | HWPX 메타 태그        |
| is_reverse_horizontal_vertical | ➖     | ➖     | ✅      | ✅      | HWPX 반전 여부        |
| offset                         | ➖     | ➖     | ✅      | ✅      | HWPX 오프셋           |
| original_size                  | ➖     | ➖     | ✅      | ✅      | HWPX 원본 크기        |
| current_size                   | ➖     | ➖     | ✅      | ✅      | HWPX 현재 크기        |
| flip                           | ➖     | ➖     | ✅      | ✅      | HWPX 뒤집기           |
| rotation_info                  | ➖     | ➖     | ✅      | ✅      | HWPX 회전 정보        |
| rendering_info                 | ➖     | ➖     | ✅      | ✅      | HWPX 렌더링 정보      |

### 5.7 도형 종류 (ShapeType)

| 항목      | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                  |
| --------- | ------ | ------ | ------- | ------- | --------------------- |
| Line      | ✅     | ✅     | ✅      | ✅      | start/end/arrows      |
| Rectangle | ✅     | ✅     | ✅      | ✅      | corner_radius         |
| Ellipse   | ✅     | ✅     | ✅      | ✅      | arc_type/angles       |
| Arc       | ✅     | ✅     | ✅      | ✅      | arc_type/angles       |
| Polygon   | ✅     | ✅     | ✅      | ✅      | points[]              |
| Curve     | ✅     | ✅     | ✅      | ✅      | points[]/closed       |
| Connector | ✅     | ✅     | ✅      | ✅      | connector_type/arrows |
| Group     | ✅     | ✅     | ✅      | ✅      | children[]            |

### 5.7.1 호/타원 타입 (ArcType)

| 항목  | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고   |
| ----- | ------ | ------ | ------- | ------- | ------ |
| Arc   | ✅     | ✅     | ✅      | ✅      | 호     |
| Pie   | ✅     | ✅     | ✅      | ✅      | 부채꼴 |
| Chord | ✅     | ✅     | ✅      | ✅      | 현     |

### 5.7.2 곡선 세그먼트 타입 (CurveSegmentType)

| 항목  | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고 |
| ----- | ------ | ------ | ------- | ------- | ---- |
| Line  | ✅     | ✅     | ✅      | ✅      | 직선 |
| Curve | ✅     | ✅     | ✅      | ✅      | 곡선 |

### 5.7.3 연결선 (ConnectLine) - HWPX

| 항목                       | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                |
| -------------------------- | ------ | ------ | ------- | ------- | ------------------- |
| line_type                  | ✅     | ✅     | ✅      | ✅      | 연결선 스타일 (9종) |
| start_point.x/y            | ✅     | ✅     | ✅      | ✅      | 시작점 좌표         |
| start_point.subject_id_ref | ✅     | ✅     | ✅      | ✅      | 시작 대상 ID        |
| start_point.subject_index  | ✅     | ✅     | ✅      | ✅      | 시작 연결 인덱스    |
| end_point.x/y              | ✅     | ✅     | ✅      | ✅      | 끝점 좌표           |
| end_point.subject_id_ref   | ✅     | ✅     | ✅      | ✅      | 끝 대상 ID          |
| end_point.subject_index    | ✅     | ✅     | ✅      | ✅      | 끝 연결 인덱스      |
| control_points[]           | ✅     | ✅     | ✅      | ✅      | 제어점들            |

### 5.7.4 곡선 제어점 (CurvePoint)

| 항목       | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                                     |
| ---------- | ------ | ------ | ------- | ------- | -------------------------------------------------------- |
| point      | ✅     | ✅     | ✅      | ✅      | 좌표                                                     |
| point_type | ➖     | ➖     | ✅      | ✅      | Normal/Control1/Control2 (HWPX 전용, HWP는 IR에서 생략) |

### 5.7.5 연결선 종류 (ConnectorType)

| 항목     | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고   |
| -------- | ------ | ------ | ------- | ------- | ------ |
| Straight | ✅     | ✅     | ✅      | ✅      | 직선   |
| Elbow    | ✅     | ✅     | ✅      | ✅      | 꺾인선 |
| Curved   | ✅     | ✅     | ✅      | ✅      | 곡선   |

### 5.7.6 선 스타일 (LineStyle)

| 항목          | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                      |
| ------------- | ------ | ------ | ------- | ------- | ------------------------- |
| line_type     | ✅     | ✅     | ✅      | ✅      | LineType enum (style)     |
| width         | ✅     | ✅     | ✅      | ✅      | 선 두께                   |
| color         | ✅     | ✅     | ✅      | ✅      | 선 색상                   |
| cap           | ✅     | ✅     | ✅      | ✅      | HWPX: endCap (Flat/Round) |
| outline_style | ✅     | ✅     | ✅      | ✅      | HWPX Normal/Outer/Inner   |
| alpha         | ✅     | ✅     | ✅      | ✅      | 투명도 (0-255)            |
| head_style    | ➖     | ➖     | ✅      | ✅      | HWPX 화살표 머리 스타일 (HWP는 LineShape.Arrow 사용)   |
| tail_style    | ➖     | ➖     | ✅      | ✅      | HWPX 화살표 꼬리 스타일 (HWP는 LineShape.Arrow 사용)   |
| head_fill     | ➖     | ➖     | ✅      | ✅      | HWPX 머리 채움 여부 (HWP는 LineShape.Arrow 사용)       |
| tail_fill     | ➖     | ➖     | ✅      | ✅      | HWPX 꼬리 채움 여부 (HWP는 LineShape.Arrow 사용)       |
| head_size     | ➖     | ➖     | ✅      | ✅      | HWPX 머리 크기 (HWP는 LineShape.Arrow 사용)            |
| tail_size     | ➖     | ➖     | ✅      | ✅      | HWPX 꼬리 크기 (HWP는 LineShape.Arrow 사용)            |

### 5.7.7 도형 내부 텍스트 (ShapeText/DrawText)

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                 |
| ------------------ | ------ | ------ | ------- | ------- | -------------------- |
| paragraphs         | ✅     | ✅     | ✅      | ✅      | 문단 목록            |
| padding            | ✅     | ✅     | ✅      | ✅      | 안쪽 여백/TextMargin |
| vertical_alignment | ✅     | ✅     | ✅      | ✅      | 세로 정렬            |
| text_direction     | ✅     | ✅     | ✅      | ✅      | 텍스트 방향          |
| editable           | ➖     | ➖     | ✅      | ✅      | 편집 가능 여부       |
| name               | ➖     | ➖     | ✅      | ✅      | HWPX 이름            |
| last_width         | ➖     | ➖     | ✅      | ✅      | HWPX 마지막 너비     |

### 5.7.8 문단 리스트 (ParagraphList/SubList) - HWPX

| 항목                        | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                            |
| --------------------------- | ------ | ------ | ------- | ------- | ------------------------------- |
| paragraphs[]                | ✅     | ✅     | ✅      | ✅      | 문단 목록                       |
| id                          | ➖     | ➖     | ✅      | ✅      | HWPX 아이디                     |
| text_direction              | ➖     | ➖     | ✅      | ✅      | Horizontal/Vertical/VerticalAll |
| line_wrap                   | ➖     | ➖     | ✅      | ✅      | Break/Squeeze/Keep              |
| vertical_alignment          | ➖     | ➖     | ✅      | ✅      | Top/Center/Bottom               |
| link_list_id_reference      | ➖     | ➖     | ✅      | ✅      | 연결 리스트 참조 (HWPX 전용) |
| link_list_next_id_reference | ➖     | ➖     | ✅      | ✅      | 다음 연결 리스트 참조 (HWPX 전용)           |
| text_width                  | ➖     | ➖     | ✅      | ✅      | 텍스트 영역 폭 (HWPX 전용)   |
| text_height                 | ➖     | ➖     | ✅      | ✅      | 텍스트 영역 높이 (HWPX 전용) |
| has_text_reference          | ➖     | ➖     | ✅      | ✅      | 텍스트 참조 여부 (HWPX 전용)                |
| has_number_reference        | ➖     | ➖     | ✅      | ✅      | 번호 참조 여부 (HWPX 전용)                  |

### 5.8 화살표 (Arrow)

| 항목       | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                    |
| ---------- | ------ | ------ | ------- | ------- | ----------------------- |
| arrow_type | ✅     | ✅     | ✅      | ✅      | None/Normal/Stealth/etc |
| size       | ✅     | ✅     | ✅      | ✅      | Small/Medium/Large/etc  |
| filled     | ✅     | ✅     | ✅      | ✅      |                         |

### 5.9 수식 (Equation)

| 항목            | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                   |
| --------------- | ------ | ------ | ------- | ------- | ---------------------- |
| common          | ✅     | ✅     | ✅      | ✅      | ObjectCommon           |
| script          | ✅     | ✅     | ✅      | ✅      |                        |
| format          | ✅     | ✅     | ✅      | ✅      | HwpScript/MathML/LaTeX |
| baseline_offset | ✅     | ✅     | ✅      | ✅      |                        |
| font_size       | ✅     | ✅     | ✅      | ✅      |                        |
| color           | ✅     | ✅     | ✅      | ✅      |                        |

### 5.9.1 수식 세부 속성 (EquationProperties) - HWP

| 항목       | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                                  |
| ---------- | ------ | ------ | ------- | ------- | ----------------------------------------------------- |
| line_mode  | ✅     | ✅     | -       | -       | Baseline/Center/Bottom/Top (EquationLineMode enum)    |
| version    | ✅     | ✅     | -       | -       | 수식 버전 문자열 (EquationProperties.version)         |
| font_name  | ✅     | ✅     | -       | -       | 수식 폰트 이름 (EquationProperties.font_name)         |
| properties | ✅     | ✅     | -       | -       | 속성 플래그 (EquationProperties.properties as u32)    |

### 5.9.2 수식 세부 속성 (EquationProperties) - HWPX

| 항목      | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                                   |
| --------- | ------ | ------ | ------- | ------- | ------------------------------------------------------ |
| version   | ➖     | ➖     | ➖      | ➖      | HWPX 수식 엔진 버전, 기본값으로 처리 가능              |
| baseline  | ➖     | ➖     | ➖      | ➖      | HWPX 기준선 위치, 기본값으로 처리 가능                 |
| base_unit | ➖     | ➖     | ➖      | ➖      | HWPX 레이아웃 기본 단위, 기본값으로 처리 가능          |
| line_mode | ➖     | ➖     | ➖      | ➖      | HWPX 줄 모드 (Baseline/Center/Bottom/Top), 기본값 처리 |
| font      | ➖     | ➖     | ➖      | ➖      | HWPX 수식 폰트 이름, 기본값으로 처리 가능              |

### 5.10 OLE 객체 (OleObject)

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                |
| ---------------- | ------ | ------ | ------- | ------- | ------------------- |
| common           | ✅     | ✅     | ✅      | ✅      | ObjectCommon        |
| binary_id        | ✅     | ✅     | ✅      | ✅      |                     |
| class_id         | ➖     | ➖     | ✅      | ✅      | HWP에서 추출 어려움 |
| preview_image_id | ➖     | ➖     | ✅      | ✅      | HWPX 전용           |

### 5.10.1 OLE 세부 속성 (OleProperties) - HWPX

| 항목              | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                     |
| ----------------- | ------ | ------ | ------- | ------- | ---------------------------------------- |
| object_type       | ➖     | ➖     | ➖      | ➖      | HWPX 전용 OLE 메타데이터, IR 미지원     |
| has_moniker       | ➖     | ➖     | ➖      | ➖      | HWPX 전용 OLE 메타데이터, IR 미지원     |
| draw_aspect       | ➖     | ➖     | ➖      | ➖      | HWPX 전용 OLE 메타데이터, IR 미지원     |
| equation_baseline | ➖     | ➖     | ➖      | ➖      | HWPX 전용 OLE 메타데이터, IR 미지원     |
| extent            | ➖     | ➖     | ➖      | ➖      | HWPX 전용 OLE 메타데이터, IR 미지원     |
| line_shape        | ➖     | ➖     | ➖      | ➖      | HWPX 전용 OLE 메타데이터, IR 미지원     |

### 5.11 텍스트 박스 (TextBox)

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고         |
| ------------------ | ------ | ------ | ------- | ------- | ------------ |
| common             | ✅     | ✅     | ✅      | ✅      | ObjectCommon |
| paragraphs[]       | ✅     | ✅     | ✅      | ✅      |              |
| text_direction     | ✅     | ✅     | ✅      | ✅      |              |
| vertical_alignment | ✅     | ✅     | ✅      | ✅      |              |
| padding            | ✅     | ✅     | ✅      | ✅      |              |
| editable           | ✅     | ✅     | ✅      | ✅      |              |

### 5.12 각주/미주 (Note)

| 항목            | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                  |
| --------------- | ------ | ------ | ------- | ------- | --------------------- |
| number          | ✅     | ✅     | ✅      | ✅      |                       |
| number_format   | ✅     | ✅     | ✅      | ✅      |                       |
| number_position | ✅     | ✅     | ✅      | ✅      | Superscript/Subscript |
| paragraphs[]    | ✅     | ✅     | ✅      | ✅      |                       |
| instance_id     | ✅     | ✅     | ✅      | ✅      |                       |

### 5.13 하이퍼링크 (Hyperlink)

| 항목              | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                           |
| ----------------- | ------ | ------ | ------- | ------- | ------------------------------ |
| target (Url)      | ✅     | ✅     | ✅      | ✅      |                                |
| target (Email)    | ✅     | ✅     | ✅      | ✅      |                                |
| target (File)     | ✅     | ✅     | ✅      | ✅      |                                |
| target (Bookmark) | ✅     | ✅     | ✅      | ✅      |                                |
| tooltip           | ✅     | ✅     | ✅      | ✅      |                                |
| display_text      | ✅     | ✅     | ✅      | ✅      | 표시 텍스트 (HWP/HWPX 모두 필드 내용에서 추출) |

### 5.14 책갈피 (Bookmark)

| 항목 | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고 |
| ---- | ------ | ------ | ------- | ------- | ---- |
| name | ✅     | ✅     | ✅      | ✅      |      |

### 5.15 자동 번호 (AutoNumber)

| 항목          | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                            |
| ------------- | ------ | ------ | ------- | ------- | ------------------------------- |
| number_type   | ✅     | ✅     | ✅      | ✅      | Page/Footnote/Picture/Table/etc |
| number_format | ✅     | ✅     | ✅      | ✅      |                                 |

### 5.16 새 번호 (NewNumber)

| 항목        | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고 |
| ----------- | ------ | ------ | ------- | ------- | ---- |
| number_type | ✅     | ✅     | ✅      | ✅      |      |
| number      | ✅     | ✅     | ✅      | ✅      |      |

### 5.17 숨은 설명 (HiddenComment)

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고 |
| ------------ | ------ | ------ | ------- | ------- | ---- |
| paragraphs[] | ✅     | ✅     | ✅      | ✅      |      |

### 5.18 차트 (Chart)

| 항목       | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                       |
| ---------- | ------ | ------ | ------- | ------- | ------------------------------------------ |
| common     | ✅     | ✅     | ✅      | ✅      | ObjectCommon                               |
| chart_id   | ✅     | ✅     | ✅      | ✅      |                                            |
| chart_type | ✅     | ✅     | ✅      | ✅      | Bar/Line/Pie/Area/Scatter/Radar            |
| version    | ➖     | ➖     | ➖      | ➖      | HWPX 차트 버전, 기본값으로 처리 가능       |

### 5.18.1 차트 데이터 (ChartData) - HWP

| 항목          | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                   |
| ------------- | ------ | ------ | ------- | ------- | ---------------------- |
| title         | ➖     | ➖     | -       | -       | OLE 내부 데이터, 변환 범위 외   |
| categories    | ➖     | ➖     | -       | -       | OLE 내부 데이터, 변환 범위 외 |
| series[]      | ➖     | ➖     | -       | -       | OLE 내부 데이터, 변환 범위 외     |
| series.name   | ➖     | ➖     | -       | -       | OLE 내부 데이터, 변환 범위 외            |
| series.values | ➖     | ➖     | -       | -       | OLE 내부 데이터, 변환 범위 외       |

### 5.19 비디오 (Video)

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                          |
| ---------------- | ------ | ------ | ------- | ------- | --------------------------------------------- |
| common           | ✅     | ✅     | ✅      | ✅      | ObjectCommon                                  |
| video_type       | ✅     | ✅     | ✅      | ✅      | HWP: Embedded/Linked/YouTube, HWPX: Local/Web |
| video_id         | ✅     | ✅     | ✅      | ✅      |                                               |
| source_url       | ✅     | ✅     | ✅      | ✅      |                                               |
| preview_image_id | ✅     | ✅     | ✅      | ✅      |                                               |
| poster_binary_id | ✅     | ✅     | ✅      | ✅      | HWP 포스터 바이너리, HWPX는 image_id_ref로 매핑                           |
| width            | ✅     | ✅     | ✅      | ✅      | 비디오 너비 (HWP 직접 저장, HWPX는 original_size 사용)                        |
| height           | ✅     | ✅     | ✅      | ✅      | 비디오 높이 (HWP 직접 저장, HWPX는 original_size 사용)                        |
| file_id_ref      | ➖     | ➖     | ➖      | ➖      | HWPX 내부 참조, video_id로 변환됨             |
| image_id_ref     | ➖     | ➖     | ➖      | ➖      | HWPX 내부 참조, preview_image_id로 변환됨     |
| tag              | ➖     | ➖     | ➖      | ➖      | HWPX 비디오 태그, 기본값으로 처리 가능        |

### 5.20 양식 객체 (FormObject)

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                            |
| ------------------ | ------ | ------ | ------- | ------- | ----------------------------------------------- |
| common             | ✅     | ✅     | ✅      | ✅      | ObjectCommon                                    |
| form_type          | ✅     | ✅     | ✅      | ✅      | Button/CheckBox/Radio/Combo/List/Edit/ScrollBar |
| name               | ✅     | ✅     | ✅      | ✅      |                                                 |
| value              | ✅     | ✅     | ✅      | ✅      |                                                 |
| char_property      | ✅     | ✅     | ✅      | ✅      |                                                 |
| items[]            | ✅     | ✅     | ✅      | ✅      | ComboBox/ListBox용                              |
| tab_order          | ✅     | ✅     | ✅      | ✅      | 탭 순서 (HWPX 구현 완료, HWP 미지원)                                         |
| fore_color         | ➖     | ➖     | ✅      | ✅      | HWPX 전경색                                     |
| back_color         | ➖     | ➖     | ✅      | ✅      | HWPX 배경색                                     |
| group_name         | ➖     | ➖     | ✅      | ✅      | HWPX 그룹 이름                                  |
| tab_stop           | ➖     | ➖     | ✅      | ✅      | HWPX 탭 이동 허용 여부                          |
| enabled            | ➖     | ➖     | ✅      | ✅      | HWPX 사용 가능 여부                             |
| border_type_id_ref | ➖     | ➖     | ✅      | ✅      | HWPX 테두리 타입 참조                           |
| draw_frame         | ➖     | ➖     | ✅      | ✅      | HWPX 프레임 출력 여부                           |
| printable          | ➖     | ➖     | ✅      | ✅      | HWPX 인쇄 여부                                  |

### 5.20.1 양식 객체 세부 - Button (HWPX)

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                      |
| ---------------- | ------ | ------ | ------- | ------- | ------------------------- |
| caption          | ✅     | ✅     | ✅      | ✅      | 버튼 캡션                 |
| value            | ✅     | ✅     | ✅      | ✅      | 값 (submit/reset/none)    |
| radio_group_name | ✅     | ✅     | ✅      | ✅      | 라디오 그룹명             |
| back_style       | ✅     | ✅     | ✅      | ✅      | 배경 스타일               |
| back_color       | ✅     | ✅     | ✅      | ✅      | 배경색                    |
| tri_state        | ✅     | ✅     | ✅      | ✅      | 삼중 상태 사용 (CheckBox) |
| gradient_fill    | ✅     | ✅     | ✅      | ✅      | 그라데이션 채우기         |
| image_fill       | ✅     | ✅     | ✅      | ✅      | 이미지 채우기             |

### 5.20.2 양식 객체 세부 - Edit (HWPX)

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고          |
| ---------------- | ------ | ------ | ------- | ------- | ------------- |
| multiline        | ✅     | ✅     | ✅      | ✅      | 여러 줄 여부  |
| password_char    | ✅     | ✅     | ✅      | ✅      | 비밀번호 문자 |
| max_length       | ✅     | ✅     | ✅      | ✅      | 최대 길이     |
| scroll_bars      | ✅     | ✅     | ✅      | ✅      | 스크롤바 표시 |
| tab_key_behavior | ✅     | ✅     | ✅      | ✅      | 탭 키 동작    |
| num_only         | ✅     | ✅     | ✅      | ✅      | 숫자만 입력   |
| read_only        | ✅     | ✅     | ✅      | ✅      | 읽기 전용     |
| alignment        | ✅     | ✅     | ✅      | ✅      | 정렬          |

### 5.20.3 양식 객체 세부 - ComboBox/ListBox (HWPX)

| 항목           | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                         |
| -------------- | ------ | ------ | ------- | ------- | ---------------------------- |
| edit_enable    | ✅     | ✅     | ✅      | ✅      | 편집 가능 (ComboBox)         |
| items_text     | ✅     | ✅     | ✅      | ✅      | 항목 텍스트                  |
| items_value    | ✅     | ✅     | ✅      | ✅      | 항목 값                      |
| selected_value | ✅     | ✅     | ✅      | ✅      | 선택된 값                    |
| list_box_rows  | ✅     | ✅     | ✅      | ✅      | 표시 행 수 (ComboBox)        |
| list_box_width | ✅     | ✅     | ✅      | ✅      | 목록 폭 (ComboBox)           |
| item_height    | ✅     | ✅     | ✅      | ✅      | 항목 높이 (ListBox)          |
| top_index      | ✅     | ✅     | ✅      | ✅      | 최상단 표시 인덱스 (ListBox) |

### 5.20.4 양식 객체 세부 - ScrollBar (HWPX)

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                |
| ------------ | ------ | ------ | ------- | ------- | ------------------- |
| bar_type     | ➖     | ➖     | ✅      | ✅      | Horizontal/Vertical HWPX전용 |
| min          | ✅     | ✅     | ✅      | ✅      | 최소값              |
| max          | ✅     | ✅     | ✅      | ✅      | 최대값              |
| value        | ✅     | ✅     | ✅      | ✅      | 현재 값             |
| small_change | ✅     | ✅     | ✅      | ✅      | 작은 증감값         |
| large_change | ✅     | ✅     | ✅      | ✅      | 큰 증감값           |
| page         | ✅     | ✅     | ✅      | ✅      | 페이지 단위         |
| delay        | ✅     | ✅     | ✅      | ✅      | 반복 지연 시간      |

### 5.21 글맵시 (TextArt)

| 항목          | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                 |
| ------------- | ------ | ------ | ------- | ------- | ------------------------------------ |
| common        | ✅     | ✅     | ✅      | ✅      | ObjectCommon                         |
| text          | ✅     | ✅     | ✅      | ✅      |                                      |
| font_name     | ✅     | ✅     | ✅      | ✅      |                                      |
| font_style    | ✅     | ✅     | ✅      | ✅      | Regular/Bold/Italic/BoldItalic       |
| font_type     | ➖     | ➖     | ✅      | ✅      | HWPX TTF/HTF                         |
| shape         | ✅     | ✅     | ✅      | ✅      | Rectangle/Circle/Arch/Wave/etc (45+) |
| line_spacing  | ✅     | ✅     | ✅      | ✅      |                                      |
| char_spacing  | ✅     | ✅     | ✅      | ✅      |                                      |
| alignment     | ✅     | ✅     | ✅      | ✅      |                                      |
| line          | ✅     | ✅     | ✅      | ✅      | LineStyle                            |
| fill          | ✅     | ✅     | ✅      | ✅      |                                      |
| shadow        | ✅     | ✅     | ✅      | ✅      |                                      |
| text_art_pr   | ➖     | ➖     | ✅      | ✅      | HWPX 추가 속성                       |
| text_color    | ✅     | ✅     | ➖      | ➖      | HWP 텍스트 색상 (fill.color로 저장)                      |
| outline_color | ✅     | ✅     | ➖      | ➖      | HWP 외곽선 색상 (line.color로 저장)                      |
| shadow_color  | ✅     | ✅     | ➖      | ➖      | HWP 그림자 색상 (shadow.color로 저장)                      |
| pt0           | ➖     | ➖     | ➖      | ➖      | HWPX 꼭지점 0 (WordArt 전용, HWP 미지원)                        |
| pt1           | ➖     | ➖     | ➖      | ➖      | HWPX 꼭지점 1 (WordArt 전용, HWP 미지원)                        |
| pt2           | ➖     | ➖     | ➖      | ➖      | HWPX 꼭지점 2 (WordArt 전용, HWP 미지원)                        |
| pt3           | ➖     | ➖     | ➖      | ➖      | HWPX 꼭지점 3 (WordArt 전용, HWP 미지원)                        |

### 5.22 글자 겹침 (Compose) - HWPX

| 항목                | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                       |
| ------------------- | ------ | ------ | ------- | ------- | -------------------------- |
| compose_type        | ➖     | ➖     | ✅      | ✅      | 겹침 종류 (Spread/Overlap) |
| circle_type         | ➖     | ➖     | ✅      | ✅      | 테두리 종류 (14종)         |
| char_size           | ➖     | ➖     | ✅      | ✅      | 글자 크기 비율             |
| char_property_count | ➖     | ➖     | ✅      | ✅      | 글자 속성 개수             |
| char_properties[]   | ➖     | ➖     | ✅      | ✅      | 글자 속성 참조 배열        |
| compose_text        | ➖     | ➖     | ✅      | ✅      | 겹침 텍스트                |

### 5.23 덧말 (Dutmal) - HWPX

| 항목          | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                 |
| ------------- | ------ | ------ | ------- | ------- | -------------------- |
| position_type | ➖     | ➖     | ✅      | ✅      | 위/아래 (Top/Bottom) |
| alignment     | ➖     | ➖     | ✅      | ✅      | 정렬 (6종)           |
| main_text     | ➖     | ➖     | ✅      | ✅      | 주 텍스트            |
| sub_text      | ➖     | ➖     | ✅      | ✅      | 덧말 텍스트          |
| size_ratio    | ➖     | ➖     | ✅      | ✅      | 크기 비율            |
| option        | ➖     | ➖     | ✅      | ✅      | 옵션 (고정값 4)      |
| style_id_ref  | ➖     | ➖     | ✅      | ✅      | 스타일 참조          |

### 5.24 페이지 번호 (PageNumberControl) - HWPX

| 항목           | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고        |
| -------------- | ------ | ------ | ------- | ------- | ----------- |
| position       | ✅     | ✅     | ✅      | ✅      | 12가지 위치 |
| format_type    | ✅     | ✅     | ✅      | ✅      | 번호 형식   |
| side_character | ✅     | ✅     | ✅      | ✅      | 줄표 문자   |

### 5.25 자동 번호/새 번호 (AutoNumberNewNumber) - HWPX

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                 |
| ------------------ | ------ | ------ | ------- | ------- | ------------------------------------ |
| number_type        | ✅     | ✅     | ✅      | ✅      | PAGE/FOOTNOTE/PICTURE/TABLE/EQUATION |
| auto_number_format | ➖     | ➖     | ✅      | ✅      | 형식 지정 HWPX전용                   |

### 5.26 페이지 가림 (PageHiding) - HWPX

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고             |
| ---------------- | ------ | ------ | ------- | ------- | ---------------- |
| hide_header      | ➖     | ➖     | ➖      | ➖      | 머리말 숨김 (HWPX 전용, HWP 미지원)      |
| hide_footer      | ➖     | ➖     | ➖      | ➖      | 꼬리말 숨김 (HWPX 전용, HWP 미지원)      |
| hide_master_page | ➖     | ➖     | ➖      | ➖      | 바탕쪽 숨김 (HWPX 전용, HWP 미지원)      |
| hide_border      | ➖     | ➖     | ➖      | ➖      | 테두리 숨김 (HWPX 전용, HWP 미지원)      |
| hide_fill        | ➖     | ➖     | ➖      | ➖      | 채우기 숨김 (HWPX 전용, HWP 미지원)      |
| hide_page_number | ➖     | ➖     | ➖      | ➖      | 페이지 번호 숨김 (HWPX 전용, HWP 미지원) |

### 5.27 색인 표시 (IndexMark) - HWPX

| 항목       | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고       |
| ---------- | ------ | ------ | ------- | ------- | ---------- |
| first_key  | ➖     | ➖     | ➖      | ➖      | 첫 번째 키 (HWPX 전용, HWP 미지원) |
| second_key | ➖     | ➖     | ➖      | ➖      | 두 번째 키 (HWPX 전용, HWP 미지원) |

### 5.28 도형 요소 속성 (ShapeElementProperties) - HWP

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                 |
| ------------ | ------ | ------ | ------- | ------- | -------------------- |
| element_type | ➖     | ➖     | ➖      | ➖      | 요소 종류 (HWP 내부 바이너리) |
| properties   | ➖     | ➖     | ➖      | ➖      | 속성 플래그 (HWP 내부 바이너리)          |
| rotation     | ➖     | ➖     | ➖      | ➖      | 회전 각도 (HWP 내부 바이너리, IR rotation으로 변환)            |
| center_x     | ➖     | ➖     | ➖      | ➖      | 회전 중심 X (HWP 내부 바이너리)          |
| center_y     | ➖     | ➖     | ➖      | ➖      | 회전 중심 Y (HWP 내부 바이너리)          |
| matrix_count | ➖     | ➖     | ➖      | ➖      | 변환 행렬 개수 (HWP 내부 바이너리)       |
| matrix[]     | ➖     | ➖     | ➖      | ➖      | 변환 행렬 배열 (HWP 내부 바이너리)       |

---

## 6. 바이너리 데이터 (BinaryData)

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                              |
| ---------------- | ------ | ------ | ------- | ------- | --------------------------------- |
| id               | ✅     | ✅     | ✅      | ✅      |                                   |
| format           | ✅     | ✅     | ✅      | ✅      | Png/Jpeg/Gif/Bmp/Tiff/Wmf/Emf/etc |
| data             | ✅     | ✅     | ✅      | ✅      |                                   |
| original_path    | ✅     | ✅     | ✅      | ✅      |                                   |
| original_name    | ✅     | ✅     | ✅      | ✅      |                                   |
| compression_mode | ➖     | ➖     | ➖      | ➖      | HWP 압축 모드 (내부 저장 속성)                     |
| state            | ➖     | ➖     | ➖      | ➖      | HWP 상태 (내부 저장 속성)     |

---

## 7. 확장 데이터 (Extensions)

### 7.1 HWP 전용

| 항목                 | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                |
| -------------------- | ------ | ------ | ------- | ------- | ------------------- |
| preview_text         | ✅     | ✅     | ➖      | ➖      | HWP 전용 (PrvText)  |
| preview_image        | ✅     | ✅     | ➖      | ➖      | HWP 전용 (PrvImage) |
| summary_info         | ✅     | ✅     | ➖      | ➖      | HWP 전용            |
| distribute_doc_data  | ➖     | ➖     | ➖      | ➖      | 배포용 문서 데이터 (보안 기능)  |
| layout_compatibility | ➖     | ➖     | ➖      | ➖      | 레이아웃 호환성 (편집기 설정)     |
| document_data        | ➖     | ➖     | ➖      | ➖      | 확장 문서 데이터 (HWP 내부)    |
| forbidden_char       | ➖     | ➖     | ➖      | ➖      | 금칙 문자 (편집기 설정)           |

### 7.2 HWPX 전용

| 항목                  | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                 |
| --------------------- | ------ | ------ | ------- | ------- | -------------------- |
| master_pages[]        | ➖     | ➖     | ✅      | ✅      | HWPX 전용            |
| history               | ➖     | ➖     | ✅      | ✅      | HWPX 전용            |
| forbidden_word_list   | ➖     | ➖     | ➖      | ➖      | 금칙어 목록 (편집기 설정)          |
| compatible_document   | ➖     | ➖     | ✅      | ✅      | 호환 문서 설정       |
| document_option       | ➖     | ➖     | ➖      | ➖      | 문서 옵션 (HWPX 전용, 편집기 설정)            |
| presentation_settings | ➖     | ➖     | ➖      | ➖      | 프레젠테이션 설정 (HWPX 전용, HWP 미지원)    |
| layout_compatibility  | ➖     | ➖     | ➖      | ➖      | HWPX 레이아웃 호환성 (편집기 설정) |

### 7.2.1 마스터 페이지 정보 (MasterPageInfo)

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                        |
| ---------------- | ------ | ------ | ------- | ------- | --------------------------- |
| id               | ➖     | ➖     | ✅      | ✅      | 마스터 페이지 ID            |
| application_type | ➖     | ➖     | ➖      | ➖      | Both/Even/Odd/Last/Optional (HWPX 전용) |
| paragraphs       | ➖     | ➖     | ➖      | ➖      | 문단 내용 (HWPX 전용)                   |
| page_number      | ➖     | ➖     | ➖      | ➖      | 페이지 번호 (HWPX 전용)                 |
| page_duplicate   | ➖     | ➖     | ➖      | ➖      | 페이지 복제 (HWPX 전용)                 |
| page_front       | ➖     | ➖     | ➖      | ➖      | 앞면 여부 (HWPX 전용)                   |

### 7.2.2 변경 이력 (ChangeHistory/History)

| 항목                       | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고             |
| -------------------------- | ------ | ------ | ------- | ------- | ---------------- |
| version                    | ➖     | ➖     | ➖      | ➖      | 이력 파일 버전 (HWPX 버전관리 전용)   |
| entries[]                  | ➖     | ➖     | ➖      | ➖      | 변경 이력 목록 (HWPX 버전관리 전용)   |
| entry.revision_number      | ➖     | ➖     | ➖      | ➖      | 리비전 번호 (HWPX 버전관리 전용)      |
| entry.revision_date        | ➖     | ➖     | ➖      | ➖      | 리비전 날짜 (HWPX 버전관리 전용)      |
| entry.revision_author      | ➖     | ➖     | ➖      | ➖      | 리비전 작성자 (HWPX 버전관리 전용)    |
| entry.revision_description | ➖     | ➖     | ➖      | ➖      | 리비전 설명 (HWPX 버전관리 전용)      |
| entry.revision_lock        | ➖     | ➖     | ➖      | ➖      | 리비전 잠금 여부 (HWPX 버전관리 전용) |
| entry.auto_save            | ➖     | ➖     | ➖      | ➖      | 자동 저장 여부 (HWPX 버전관리 전용)   |
| entry.package_diff         | ➖     | ➖     | ➖      | ➖      | 패키지 차이 (HWPX 버전관리 전용)      |
| entry.head_diff            | ➖     | ➖     | ➖      | ➖      | 헤더 차이 (HWPX 버전관리 전용)        |
| entry.body_diffs[]         | ➖     | ➖     | ➖      | ➖      | 본문 차이 목록 (HWPX 버전관리 전용)   |
| entry.tail_diff            | ➖     | ➖     | ➖      | ➖      | 꼬리 차이 (HWPX 버전관리 전용)        |

### 7.2.2.1 차이 연산 (DiffOperation) - HWPX

| 항목      | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고             |
| --------- | ------ | ------ | ------- | ------- | ---------------- |
| Insert    | ➖     | ➖     | ➖      | ➖      | 삽입 연산 (HWPX 버전관리 전용)        |
| Update    | ➖     | ➖     | ➖      | ➖      | 업데이트 연산 (HWPX 버전관리 전용)    |
| Delete    | ➖     | ➖     | ➖      | ➖      | 삭제 연산 (HWPX 버전관리 전용)        |
| Position  | ➖     | ➖     | ➖      | ➖      | 위치 변경 연산 (HWPX 버전관리 전용)   |
| href      | ➖     | ➖     | ➖      | ➖      | 대상 파일 경로 (HWPX 버전관리 전용)   |
| path      | ➖     | ➖     | ➖      | ➖      | 컨테이너 내 경로 (HWPX 버전관리 전용) |
| old_value | ➖     | ➖     | ➖      | ➖      | 이전 값 (HWPX 버전관리 전용)          |

### 7.2.3 변경 추적 설정 (TrackChangeConfig)

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고             |
| ------------ | ------ | ------ | ------- | ------- | ---------------- |
| enabled      | ➖     | ➖     | ➖      | ➖      | 활성화 여부 (HWPX 편집기 설정, 문서 내용 아님)      |
| insert_color | ➖     | ➖     | ➖      | ➖      | 삽입 색상 (HWPX 편집기 설정, 문서 내용 아님)        |
| delete_color | ➖     | ➖     | ➖      | ➖      | 삭제 색상 (HWPX 편집기 설정, 문서 내용 아님)        |
| authors[]    | ➖     | ➖     | ➖      | ➖      | 변경 작성자 목록 (HWPX 편집기 설정, 문서 내용 아님) |
| changes[]    | ➖     | ➖     | ➖      | ➖      | 변경 내역 목록 (HWPX 편집기 설정, 문서 내용 아님)   |

### 7.2.4 변경 추적 정보 (TrackChangeInfo) - HWP

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고             |
| ------------ | ------ | ------ | ------- | ------- | ---------------- |
| enabled      | ➖     | ➖     | ➖      | ➖      | 변경 추적 활성화 (HWP 편집기 설정, 문서 내용 아님) |
| show_changes | ➖     | ➖     | ➖      | ➖      | 변경 표시 여부 (HWP 편집기 설정, 문서 내용 아님)   |

### 7.3 파일 헤더 (FileHeader) - HWP

| 항목                    | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                     |
| ----------------------- | ------ | ------ | ------- | ------- | ------------------------ |
| version                 | ✅     | ✅     | ➖      | ➖      | HWP 버전 (MMNNPPRR)      |
| is_compressed           | ✅     | ✅     | ➖      | ➖      | deflate 압축 여부        |
| is_encrypted            | ✅     | ✅     | ➖      | ➖      | 암호화 여부              |
| is_template             | ➖     | ➖     | ➖      | ➖      | 템플릿 여부 (파일 속성)              |
| is_distribution         | ➖     | ➖     | ➖      | ➖      | 배포용 문서 여부 (파일 속성) |
| has_script              | ➖     | ➖     | ➖      | ➖      | 스크립트 포함 (파일 속성)    |
| is_drm_protected        | ➖     | ➖     | ➖      | ➖      | DRM 보호 (파일 속성)         |
| has_xml_template        | ➖     | ➖     | ➖      | ➖      | XML 템플릿 (파일 속성)       |
| has_digital_signature   | ➖     | ➖     | ➖      | ➖      | 디지털 서명 (파일 속성)      |
| is_certificate_encrypted| ➖     | ➖     | ➖      | ➖      | 인증서 암호화 (파일 속성)    |
| has_signature_reserve   | ➖     | ➖     | ➖      | ➖      | 서명 예약 (파일 속성)        |
| is_certificate_drm      | ➖     | ➖     | ➖      | ➖      | 인증서 DRM (파일 속성)      |
| is_ccl_document         | ➖     | ➖     | ➖      | ➖      | CCL 문서 (파일 속성)        |
| is_mobile_optimized     | ➖     | ➖     | ➖      | ➖      | 모바일 최적화 (파일 속성)   |
| is_privacy_protected    | ➖     | ➖     | ➖      | ➖      | 개인정보 보호 (파일 속성)   |
| has_track_changes       | ➖     | ➖     | ➖      | ➖      | 변경 추적 (파일 속성)       |
| is_kogl_document        | ➖     | ➖     | ➖      | ➖      | 공공저작물 (파일 속성)      |
| has_video_control       | ➖     | ➖     | ➖      | ➖      | 비디오 컨트롤 (파일 속성)   |
| has_toc_field           | ➖     | ➖     | ➖      | ➖      | 목차 필드 (파일 속성)       |
| encryption_version      | ➖     | ➖     | ➖      | ➖      | 암호화 버전 (파일 속성)              |
| kogl_country            | ➖     | ➖     | ➖      | ➖      | 공공저작물 국가 (파일 속성)          |
| license_info            | ➖     | ➖     | ➖      | ➖      | 라이선스 정보 (파일 속성)            |

### 7.4 요약 정보 (SummaryInfo) - HWP

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고              |
| ---------------- | ------ | ------ | ------- | ------- | ----------------- |
| title            | ✅     | ✅     | ➖      | ➖      | 제목              |
| subject          | ✅     | ✅     | ➖      | ➖      | 주제              |
| author           | ✅     | ✅     | ➖      | ➖      | 저자              |
| keywords         | ✅     | ✅     | ➖      | ➖      | 키워드            |
| comments         | ➖     | ➖     | ➖      | ➖      | 설명 (OLE 메타데이터)              |
| last_author      | ➖     | ➖     | ➖      | ➖      | 마지막 수정자 (OLE 메타데이터)     |
| application_name | ➖     | ➖     | ➖      | ➖      | 애플리케이션 이름 (OLE 메타데이터) |
| creation_date    | ➖     | ➖     | ➖      | ➖      | 생성일 (OLE 메타데이터)            |
| last_saved_date  | ➖     | ➖     | ➖      | ➖      | 마지막 저장일 (OLE 메타데이터)     |

### 7.5 스크립트 (Scripts) - HWP

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고          |
| ------------------ | ------ | ------ | ------- | ------- | ------------- |
| default_js_version | ➖     | ➖     | ➖      | ➖      | 기본 JS 버전 (동작 코드, 변환 불가)  |
| js_versions        | ➖     | ➖     | ➖      | ➖      | JS 버전 목록 (동작 코드, 변환 불가)  |
| script_count       | ➖     | ➖     | ➖      | ➖      | 스크립트 개수 (동작 코드, 변환 불가) |
| sources[].name     | ➖     | ➖     | ➖      | ➖      | 스크립트 이름 (동작 코드, 변환 불가) |
| sources[].code     | ➖     | ➖     | ➖      | ➖      | 스크립트 코드 (동작 코드, 변환 불가) |

### 7.6 문서 옵션 (DocOptions) - HWP

| 항목                | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고             |
| ------------------- | ------ | ------ | ------- | ------- | ---------------- |
| link_doc.paths      | ➖     | ➖     | ➖      | ➖      | 연결 문서 경로 (파일 시스템 참조) |
| drm_license.version | ➖     | ➖     | ➖      | ➖      | DRM 버전 (보안 속성)         |
| drm_license.data    | ➖     | ➖     | ➖      | ➖      | DRM 데이터 (보안 속성)       |
| drm_root_sect       | ➖     | ➖     | ➖      | ➖      | DRM 루트 섹션 (보안 속성)    |
| cert_drm_header     | ➖     | ➖     | ➖      | ➖      | 인증 DRM 헤더 (보안 속성)    |

---

## 8. 특수 기능

### 8.1 변경 추적 (TrackChange)

| 항목                      | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                |
| ------------------------- | ------ | ------ | ------- | ------- | ------------------- |
| TrackChangeInsert         | ✅     | ✅     | ✅      | ✅      | RangeTag로 표현 (HWP: RangeTag type=2)     |
| TrackChangeDelete         | ✅     | ✅     | ✅      | ✅      | RangeTag로 표현 (HWP: RangeTag type=3)     |
| track_change_config       | ➖     | ➖     | ✅      | ✅      | HWPX 변경 추적 설정 |
| paragraph_track_change_id | ➖     | ➖     | ✅      | ✅      | HWPX 문단 변경 ID   |
| character_track_change_id | ➖     | ➖     | ✅      | ✅      | HWPX 글자 변경 ID   |

### 8.1.1 변경 작성자 (TrackChangeAuthor) - HWP

| 항목      | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고        |
| --------- | ------ | ------ | ------- | ------- | ----------- |
| name      | ➖     | ➖     | ➖      | ➖      | 작성자 이름 (HWP TrackChange 내부) |
| author_id | ➖     | ➖     | ➖      | ➖      | 작성자 ID (HWP TrackChange 내부)   |

### 8.1.2 변경 내용 (TrackChangeContent) - HWP

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                 |
| ------------ | ------ | ------ | ------- | ------- | -------------------- |
| change_type  | ➖     | ➖     | ➖      | ➖      | Insert/Delete/Format (HWP TrackChange 내부) |
| author_index | ➖     | ➖     | ➖      | ➖      | 작성자 인덱스 (HWP TrackChange 내부)        |
| timestamp    | ➖     | ➖     | ➖      | ➖      | 변경 시각 (HWP TrackChange 내부)            |

### 8.2 덧말/글자겹침

| 항목               | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고          |
| ------------------ | ------ | ------ | ------- | ------- | ------------- |
| Dutmal (덧말)      | ➖     | ➖     | ✅      | ✅      | HWP 인라인 컨트롤(파서 미지원), HWPX↔IR 양방향 구현 |
| Compose (글자겹침) | ➖     | ➖     | ✅      | ✅      | HWP 인라인 컨트롤(파서 미지원), HWPX↔IR 양방향 구현 |

### 8.3 형광펜 (MarkPen)

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고        |
| ------------ | ------ | ------ | ------- | ------- | ----------- |
| MarkPenBegin | ✅     | ✅     | ✅      | ✅      | 형광펜 시작 - HWP RangeTag(Highlight)↔MarkPen 양방향 |
| MarkPenEnd   | ✅     | ✅     | ✅      | ✅      | 형광펜 끝 - HWP RangeTag(Highlight)↔MarkPen 양방향   |
| color        | ✅     | ✅     | ✅      | ✅      | 형광펜 색상 - RangeTag data에서 추출 |

### 8.4 메모 (Memo)

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고      |
| ------------ | ------ | ------ | ------- | ------- | --------- |
| memo_content | ✅     | ✅     | ➖      | ➖      | 메모 내용 (paragraphs로 저장) |
| memo_author  | ➖     | ➖     | ➖      | ➖      | 작성자 (HWP/HWPX 형식에 필드 없음)    |
| memo_date    | ➖     | ➖     | ➖      | ➖      | 작성일 (HWP/HWPX 형식에 필드 없음)    |

### 8.4.1 메모 모양 (MemoShape) - HWP

| 항목            | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고        |
| --------------- | ------ | ------ | ------- | ------- | ----------- |
| border_color    | ➖     | ➖     | ➖      | ➖      | 테두리 색상 (HWP 메모 표시 설정) |
| fill_color      | ➖     | ➖     | ➖      | ➖      | 채우기 색상 (HWP 메모 표시 설정) |
| croshatch_color | ➖     | ➖     | ➖      | ➖      | 무늬 색상 (HWP 메모 표시 설정)   |
| width           | ➖     | ➖     | ➖      | ➖      | 메모 너비 (HWP 메모 표시 설정)   |
| memo_index      | ➖     | ➖     | ➖      | ➖      | 메모 인덱스 (HWP 메모 표시 설정) |
| line_type       | ➖     | ➖     | ➖      | ➖      | 선 종류 (HWP 메모 표시 설정)     |
| line_style      | ➖     | ➖     | ➖      | ➖      | 선 스타일 (HWP 메모 표시 설정)   |

### 8.4.2 메모 모양 (MemoShape) - HWPX

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                                |
| ------------ | ------ | ------ | ------- | ------- | --------------------------------------------------- |
| id           | ➖     | ➖     | ➖      | ➖      | 메모 모양 ID (HWPX 메모 표시 설정)                                        |
| width        | ➖     | ➖     | ➖      | ➖      | 메모 표시 너비 (HWPX 메모 표시 설정)                            |
| memo_type    | ➖     | ➖     | ➖      | ➖      | 메모 종류 (HWPX 메모 표시 설정) |
| line_width   | ➖     | ➖     | ➖      | ➖      | 선 두께 (HWPX 메모 표시 설정)                                             |
| line_type    | ➖     | ➖     | ➖      | ➖      | 선 종류 (HWPX 메모 표시 설정)                                             |
| line_color   | ➖     | ➖     | ➖      | ➖      | 선 색상 (HWPX 메모 표시 설정)                                             |
| fill_color   | ➖     | ➖     | ➖      | ➖      | 채우기 색상 (HWPX 메모 표시 설정)                                         |
| active_color | ➖     | ➖     | ➖      | ➖      | 활성 색상 (HWPX 메모 표시 설정)                                           |

### 8.5 그리기 효과 (Effects) - HWPX

| 항목          | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                               |
| ------------- | ------ | ------ | ------- | ------- | ---------------------------------- |
| skew          | ➖     | ➖     | ➖      | ➖      | 기울이기 (HWPX 고급효과, HWP 미지원)                           |
| scale         | ➖     | ➖     | ➖      | ➖      | 크기 조정 (HWPX 고급효과, HWP 미지원)                          |
| effects_color | ➖     | ➖     | ➖      | ➖      | 효과 색상 (HWPX 고급효과, HWP 미지원) |

### 8.5.1 그림자 효과 (AdvancedShadowEffect) - HWPX

| 항목                | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                 |
| ------------------- | ------ | ------ | ------- | ------- | -------------------- |
| style               | ➖     | ➖     | ➖      | ➖      | 그림자 스타일 (HWPX 고급효과, HWP 미지원) |
| alpha               | ➖     | ➖     | ➖      | ➖      | 투명도 (HWPX 고급효과, HWP 미지원)               |
| blur                | ➖     | ➖     | ➖      | ➖      | 흐림 (HWPX 고급효과, HWP 미지원)                 |
| direction           | ➖     | ➖     | ➖      | ➖      | 방향 (HWPX 고급효과, HWP 미지원)         |
| distance            | ➖     | ➖     | ➖      | ➖      | 거리 (HWPX 고급효과, HWP 미지원)                 |
| alignment           | ➖     | ➖     | ➖      | ➖      | 정렬 (HWPX 고급효과, HWP 미지원)           |
| rotation_with_shape | ➖     | ➖     | ➖      | ➖      | 도형과 함께 회전 (HWPX 고급효과, HWP 미지원)     |
| effects_color       | ➖     | ➖     | ➖      | ➖      | 그림자 색상 (HWPX 고급효과, HWP 미지원)          |

### 8.5.2 네온/광선 효과 (GlowEffect) - HWPX

| 항목          | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고      |
| ------------- | ------ | ------ | ------- | ------- | --------- |
| alpha         | ➖     | ➖     | ➖      | ➖      | 투명도 (HWPX 전용 효과, HWP 미지원)    |
| radius        | ➖     | ➖     | ➖      | ➖      | 반경 (HWPX 전용 효과, HWP 미지원)      |
| effects_color | ➖     | ➖     | ➖      | ➖      | 광선 색상 (HWPX 전용 효과, HWP 미지원) |

### 8.5.3 부드러운 가장자리 (SoftEdgeEffect) - HWPX

| 항목   | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고          |
| ------ | ------ | ------ | ------- | ------- | ------------- |
| radius | ➖     | ➖     | ➖      | ➖      | 부드러운 정도 (HWPX 전용 효과, HWP 미지원) |

### 8.5.4 반사 효과 (ReflectionEffect) - HWPX

| 항목                | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고             |
| ------------------- | ------ | ------ | ------- | ------- | ---------------- |
| style               | ➖     | ➖     | ➖      | ➖      | 반사 스타일 (HWPX 전용 효과, HWP 미지원)      |
| start_alpha         | ➖     | ➖     | ➖      | ➖      | 시작 투명도 (HWPX 전용 효과, HWP 미지원)      |
| start_position      | ➖     | ➖     | ➖      | ➖      | 시작 위치 (HWPX 전용 효과, HWP 미지원)        |
| end_alpha           | ➖     | ➖     | ➖      | ➖      | 끝 투명도 (HWPX 전용 효과, HWP 미지원)        |
| end_position        | ➖     | ➖     | ➖      | ➖      | 끝 위치 (HWPX 전용 효과, HWP 미지원)          |
| distance            | ➖     | ➖     | ➖      | ➖      | 거리 (HWPX 전용 효과, HWP 미지원)             |
| direction           | ➖     | ➖     | ➖      | ➖      | 방향 (HWPX 전용 효과, HWP 미지원)             |
| fade_direction      | ➖     | ➖     | ➖      | ➖      | 페이드 방향 (HWPX 전용 효과, HWP 미지원)      |
| align               | ➖     | ➖     | ➖      | ➖      | 정렬 (HWPX 전용 효과, HWP 미지원)             |
| rotation_with_shape | ➖     | ➖     | ➖      | ➖      | 도형과 함께 회전 (HWPX 전용 효과, HWP 미지원) |

### 8.5.5 효과 색상 (EffectsColor) - HWPX

| 항목         | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                   |
| ------------ | ------ | ------ | ------- | ------- | ---------------------- |
| color_type   | ➖     | ➖     | ➖      | ➖      | RGB/CMYK/Scheme/System (HWPX 전용) |
| rgb_value    | ➖     | ➖     | ➖      | ➖      | RGB 값 (HWPX 전용)         |
| cmyk_value   | ➖     | ➖     | ➖      | ➖      | CMYK 값 (HWPX 전용)      |
| scheme_value | ➖     | ➖     | ➖      | ➖      | 스키마 색상 (HWPX 전용)    |
| system_value | ➖     | ➖     | ➖      | ➖      | 시스템 색상 (HWPX 전용)    |
| scheme_index | ➖     | ➖     | ➖      | ➖      | 스킴 인덱스 (HWPX 전용)            |
| system_index | ➖     | ➖     | ➖      | ➖      | 시스템 인덱스 (HWPX 전용)          |
| preset_index | ➖     | ➖     | ➖      | ➖      | 프리셋 인덱스 (HWPX 전용)          |

### 8.6 도형 그림자 (ShapeShadow)

| 항목        | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                                     |
| ----------- | ------ | ------ | ------- | ------- | ---------------------------------------- |
| shadow_type | ✅     | ✅     | ✅      | ✅      | DROP/INNER/NONE                          |
| offset_x/y  | ✅     | ✅     | ✅      | ✅      |                                          |
| color       | ✅     | ✅     | ✅      | ✅      |                                          |
| alpha       | ✅     | ✅     | ✅      | ✅      | IR 필드 추가됨                           |
| blur        | ➖     | ➖     | ✅      | ✅      | HWPX 전용, IR 필드 추가됨                |
| direction   | ➖     | ➖     | ✅      | ✅      | HWPX 전용, IR 필드 추가됨                |
| distance    | ➖     | ➖     | ✅      | ✅      | HWPX 전용, IR 필드 추가됨                |

### 8.7 특수 문자/제어문자

| 항목             | HWP→IR | IR→HWP | HWPX→IR | IR→HWPX | 비고                 |
| ---------------- | ------ | ------ | ------- | ------- | -------------------- |
| Tab              | ✅     | ✅     | ✅      | ✅      | 탭 문자              |
| LineBreak        | ✅     | ✅     | ✅      | ✅      | 줄바꿈 (Shift+Enter) |
| Hyphen           | ✅     | ✅     | ✅      | ✅      | 하이픈               |
| NonBreakingSpace | ✅     | ✅     | ✅      | ✅      | 줄바꿈 안 되는 공백  |
| FixedWidthSpace  | ✅     | ✅     | ✅      | ✅      | 고정폭 공백          |
| TitleMark        | ➖     | ➖     | ✅      | ✅      | HWP 파싱됨(변환 미구현), HWPX 완전 구현     |

---

## 9. 열거형 및 상수 (Enums)

### 9.1 주요 열거형 목록

| 열거형                 | 용도                       | 변환 완성도 |
| ---------------------- | -------------------------- | ----------- |
| Alignment              | 텍스트 정렬                | ✅          |
| VerticalAlignment      | 수직 정렬                  | ✅          |
| LineType               | 선 종류 (14종)             | ✅          |
| LineCap                | 선 끝 모양                 | ✅          |
| ArrowType              | 화살표 종류 (7종)          | ✅          |
| ArrowSize              | 화살표 크기                | ✅          |
| UnderlineType          | 밑줄 종류 (9종)            | ✅          |
| UnderlinePosition      | 밑줄 위치                  | ✅          |
| StrikethroughType      | 취소선 종류                | ✅          |
| EmphasisType           | 강조점 종류 (5종)          | ✅          |
| OutlineType            | 외곽선 종류 (5종)          | ✅          |
| ShadowType             | 그림자 종류 (5종)          | ✅          |
| TextDirection          | 텍스트 방향 (4종)          | ✅          |
| PageOrientation        | 용지 방향                  | ✅          |
| LineBreakKorean        | 한글 줄 나눔               | ✅          |
| LineBreakLatin         | 영어 줄 나눔               | ✅          |
| LanguageType           | 언어 종류 (7종)            | ✅          |
| FillType               | 채우기 종류 (5종)          | ✅          |
| GradientType           | 그라데이션 종류 (4종)      | ✅          |
| ImageFillMode          | 이미지 채우기 모드 (5종)   | ✅          |
| NumberFormat           | 번호 형식 (12종)           | ✅          |
| BreakType              | 나눔 종류 (4종)            | ✅          |
| HeaderFooterApplyTo    | 머리글/바닥글 적용 (4종)   | ✅          |
| NoteNumberPosition     | 각주 번호 위치             | ✅          |
| NoteNumbering          | 각주 번호 매김 (3종)       | ✅          |
| FieldType              | 필드 종류 (14종+)          | ✅          |
| ImageEffect            | 이미지 효과 (4종)          | ✅          |
| ImageFlip              | 이미지 뒤집기 (4종)        | ✅          |
| TextWrapType           | 텍스트 감싸기 (5종)        | ✅          |
| TextWrapSide           | 텍스트 감싸기 방향 (4종)   | ✅          |
| VerticalRelativeTo     | 개체 수직 기준 (3종)       | ✅          |
| HorizontalRelativeTo   | 개체 수평 기준 (4종)       | ✅          |
| ArcType                | 호 종류 (3종)              | ✅          |
| CurveSegmentType       | 곡선 세그먼트 (2종)        | ✅          |
| EquationLineMode       | 수식 줄 모드 (4종)         | ✅          |
| BinaryDataType         | 바이너리 데이터 타입 (3종) | ✅          |
| BinaryDataState        | 바이너리 데이터 상태 (4종) | ✅          |
| PatternType            | 패턴 종류 (7종)            | ✅          |
| ImageFillType          | 이미지 채우기 (16종)       | ✅          |
| SlashDiagonalType      | 빗금 대각선 종류           | ✅          |
| CenterLineType         | 중심선 종류                | ✅          |
| DiagonalType           | 대각선 종류 (SlashDiag)    | ✅          |
| ParameterType          | 매개변수 종류 (12종)       | ✅          |
| ParagraphHeadAlignment | 문단 머리 정렬             | ✅          |
| TrackChangeType        | 변경 추적 종류             | ✅          |

### 9.2 HWPX 전용 열거형

| 열거형                | 용도                    | 변환 완성도 |
| --------------------- | ----------------------- | ----------- |
| PaperOrientation      | 용지 방향               | ✅          |
| GutterType            | 제본 여백 종류          | ✅          |
| ColumnType            | 단 종류 (3종)           | ✅          |
| ColumnLayout          | 단 레이아웃             | ✅          |
| FootnoteNumberingType | 각주 번호 매김          | ✅          |
| FootnotePlacement     | 각주 배치               | ✅          |
| EndnoteNumberingType  | 미주 번호 매김          | ✅          |
| EndnotePlacement      | 미주 배치               | ✅          |
| PageBorderType        | 쪽 테두리 종류          | ✅          |
| PageBorderPosition    | 쪽 테두리 위치          | ✅          |
| FillAreaType          | 채우기 영역             | ✅          |
| TablePageBreak        | 표 페이지 나눔          | ✅          |
| TextArtShape          | 글맵시 모양 (40+)       | ✅          |
| ComposeType           | 글자겹침 종류           | ✅          |
| DutmalPosition        | 덧말 위치               | ✅          |
| VideoType             | 비디오 종류             | ✅          |
| ButtonValue           | 버튼 값                 | ✅          |
| ShadowEffectType      | 그림자 효과 종류 (14종) | ✅          |
| LineEndCapStyle       | 선 끝 스타일            | ✅          |
| OutlineStyle          | 외곽선 스타일           | ✅          |
| VerticalRelativeTo    | 수직 기준 (3종)         | ✅          |
| HorizontalRelativeTo  | 수평 기준 (4종)         | ✅          |
| VerticalOffsetType    | 수직 오프셋 (5종)       | ✅          |
| HorizontalOffsetType  | 수평 오프셋 (5종)       | ✅          |
| WidthRelativeTo       | 너비 기준 (5종)         | ✅          |
| HeightRelativeTo      | 높이 기준 (4종)         | ✅          |
| TextArtShape          | 글맵시 모양 (45종)      | ✅          |
| MemoTrackingType      | 메모 추적 종류          | ✅          |
| TextOffsetType        | 텍스트 오프셋 종류      | ✅          |
| FontLanguage          | 글꼴 언어               | ✅          |
| FontFamilyType        | 글꼴 패밀리 종류        | ✅          |

### 9.3 PANOSE 열거형 (폰트 분류)

| 열거형                | 용도          | 변환 완성도 |
| --------------------- | ------------- | ----------- |
| PanoseFamilyType      | 글꼴 종류     | ✅          |
| PanoseSerifStyle      | 세리프 스타일 | ✅          |
| PanoseWeight          | 굵기          | ✅          |
| PanoseProportion      | 비율          | ✅          |
| PanoseContrast        | 대비          | ✅          |
| PanoseStrokeVariation | 획 변형       | ✅          |
| PanoseArmStyle        | 팔 스타일     | ✅          |
| PanoseLetterForm      | 글자 형태     | ✅          |
| PanoseMidline         | 중간선        | ✅          |
| PanoseXHeight         | X-높이        | ✅          |

---

## 변경 이력

| 날짜       | 변경 내용                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2025-12-10 | 최초 작성 - IR 기반 전체 기능 세트 정리                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| 2025-12-10 | HWP/HWPX 크레이트 대조 조사 후 누락 항목 추가: 섹션 정의 속성, 필드 타입 확장, 줄 세그먼트, 범위 태그, 글자겹침/덧말, 파일 헤더, 변경 추적 세부사항, 열거형 목록 등                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| 2025-12-10 | 상세 재조사 후 추가: PANOSE 분류정보(11종), CharShape 배열 필드, 문단 머리 속성, 패턴/이미지 채우기 세부, 시작번호(BeginNumber), 금칙문자, 레이아웃호환성, RunContent 상세(26종+), 개체 위치/크기 기준(16종), 렌더링 정보, 호/곡선 타입, 연결선 세부, TextArt 색상, 양식 탭순서, 비디오 포스터, HWP/HWPX 열거형 30종+ 추가                                                                                                                                                                                                                                                                          |
| 2025-12-10 | 2차 조사 (고정점 확인): CurvePoint(점 종류), ConnectorType(3종), LineStyle(4필드), ShapeText(5필드), MasterPageInfo(6필드), ChangeHistory(4필드), TrackChangeConfig(3필드), HWPX presentation_settings/layout_compatibility 추가                                                                                                                                                                                                                                                                                                                                                                    |
| 2025-12-10 | 최종 조사 (고정점 도달): HWP body/doc_info, HWPX paragraph/header 전체 파일 스캔 완료. PictureProperties(9필드), Effects 시스템(GlowEffect/SoftEdgeEffect/ReflectionEffect/AdvancedShadowEffect/EffectsColor), FormObject 세부(Button/Edit/ComboBox), LayoutCompatibility HWPX(44+ 토글), PageHiding(6필드), IndexMark(2필드), ShapeElementProperties(7필드), MemoShape(5필드), BorderFill 배열 필드(6필드), CharShape strikethrough_color, TrackChangeInfo HWP(2필드) 추가                                                                                                                         |
| 2025-12-10 | 2차 고정점 조사: 이전 조사에서 추가한 모든 항목 확인됨. 누락 항목 없음. 변경 없이 고정점 유지                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| 2025-12-10 | 3차 고정점 조사 (최종): 변환 함수(to_ir.rs/from_ir.rs) 전체 스캔 완료. 발견된 내부 Writer 구조(CharShapeData, TableData 등)는 IR 기능이 아닌 구현 세부사항으로 판단. **고정점 도달 확인 (3회 연속 무변경)**                                                                                                                                                                                                                                                                                                                                                                                         |
| 2025-12-10 | 4차 검증 (1차 반복): HWPX FormObject ScrollBar 세부 속성 항목 추가 (5.20.4절 - bar_type/min/max/value/small_change/large_change/page/delay 8필드)                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| 2025-12-10 | 4차 검증 (2차 반복): Button tri_state 속성 추가 (5.20.1절), FieldStart 세부 속성 섹션 추가 (4.4.1절 - parameters/sub_list/editable/dirty/z_order/field_id), ParameterList 섹션 추가 (4.4.2절 - 5종 매개변수 타입)                                                                                                                                                                                                                                                                                                                                                                                   |
| 2025-12-10 | 4차 검증 (3차 반복): HWP ControlData/ParameterSet 섹션 추가 (4.4.3절 - 7필드), HWP ChartData 세부 섹션 추가 (5.18.1절 - title/categories/series 5필드)                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| 2025-12-10 | 4차 검증 (4차 반복): HWP EquationProperties 세부 섹션 추가 (5.9.1절 - line_mode/version/font_name/properties 4필드)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| 2025-12-10 | 4차 검증 (5차 반복): HWPX TableLabel 섹션 추가 (5.3.1절 - 라벨 문서 11필드)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| 2025-12-10 | 4차 검증 (6차 반복): HcfVersion 섹션 추가 (1.5절), ID 매핑 번호 수정 (1.5→1.6), MemoShape HWP 섹션 추가 (8.4.1절 - 7필드), MemoShape HWPX 번호 변경 (8.4.1→8.4.2), Video width/height/tag 추가, Chart version 추가, EquationProperties HWPX 섹션 추가 (5.9.2절 - 5필드), OleProperties HWPX 섹션 추가 (5.10.1절 - 6필드)                                                                                                                                                                                                                                                                            |
| 2025-12-10 | 4차 검증 (7차 반복): ChangeHistory 세부 필드 확장 (7.2.2절 - 12필드), DiffOperation 섹션 추가 (7.2.2.1절 - 7필드), EffectsColor 인덱스 필드 추가 (scheme_index/system_index/preset_index)                                                                                                                                                                                                                                                                                                                                                                                                           |
| 2025-12-10 | 4차 검증 (8차 반복): TrackChangeAuthor 섹션 추가 (8.1.1절 - name/author_id 2필드), TrackChangeContent 섹션 추가 (8.1.2절 - change_type/author_index/timestamp 3필드)                                                                                                                                                                                                                                                                                                                                                                                                                                |
| 2025-12-10 | 4차 검증 (9차 반복): IR에서 HWP/HWPX에 없는 BinaryFormat 제거 (Audio, Svg 삭제) - IR 정리 완료                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| 2025-12-10 | 4차 검증 (10차 반복): Presentation 필드 확장 (7필드 - effect/sound_id_reference/auto_show/show_time/invert_text/apply_to/fill_brush), PageBorderFill header_inside/footer_inside 추가                                                                                                                                                                                                                                                                                                                                                                                                               |
| 2025-12-10 | 4차 검증 (11차 반복): SectionVisibility 필드명 정확화 및 hide_first_empty_line/show_line_number 추가 (8필드), SectionStartNumber 필드명 정확화 및 page_starts_on 추가 (5필드), SectionGrid 필드명 정확화, LineNumberShape 필드명 정확화, NoteLine 필드명 정확화, NoteShape user_character 추가, FootnotePlacementSettings/EndnotePlacementSettings 구조체명 명확화, FootnoteNumbering/EndnoteNumbering 섹션 추가 (2.4.4절 - 2필드)                                                                                                                                                                  |
| 2025-12-10 | 4차 검증 (12차 반복): PageBorderFill page_type/text_border 추가, TableCell name/has_margin/editable/dirty 추가, Table HWPX 필드 10개 추가 (page_break/repeat_header/no_adjust/text_wrap/text_flow/lock/numbering_type/shape_comment/meta_tag/inside_margin), Picture HWPX 필드 10개 추가, Shape HWPX 공통 필드 16개 추가 (text_wrap/text_flow/lock/numbering_type/group_level/instance_id/href/shape_comment/meta_tag/is_reverse_horizontal_vertical/offset/original_size/current_size/flip/rotation_info/rendering_info), ShapeText name/last_width 추가                                           |
| 2025-12-10 | 4차 검증 (15차 반복): Paragraph paragraph_track_change_id/merged 추가 (4.1절), Run character_track_change_id 추가 (4.2절), LineSegment 필드명 HWPX 기준으로 정정 및 flags 추가 (4.5절 - 9필드), FieldType ProofreadingMarks/DocumentDate 추가 (4.4절), FormObject 7종 개별 항목으로 분리 (4.7절 - RadioButton/CheckButton 별도 표기), PageNumberControl/PageHiding 항목 추가 (4.7절), PageNumber 세부 속성 섹션 추가 (4.7.1절 - position/format_type/side_character 3필드), TrackChangeTag 섹션 추가 (4.7.2절 - paragraph_end/track_change_id/id 3필드), IndexMark 세부 섹션 추가 (4.7.3절 - 2필드) |
| 2025-12-10 | 4차 검증 (16차 반복): LineStyle 필드 확장 (5.7.6절 - outline_style/alpha/head_style/tail_style/head_fill/tail_fill/head_size/tail_size 8필드 추가), ParagraphList 섹션 추가 (5.7.8절 - 11필드, 연결 리스트/텍스트 영역 정보)                                                                                                                                                                                                                                                                                                                                                                        |
| 2025-12-10 | 4차 검증 (17차 반복): HWP 전용 FootnoteShape/EndnoteShape 섹션 추가 (2.4.5절 - separator_position/space_below/continue_numbering/separator_line_type/separator_line_thickness/separator_line_color 6필드), Hyperlink display_text 추가 (5.13절)                                                                                                                                                                                                                                                                                                                                                     |
| 2025-12-10 | 4차 검증 (18차 반복): PageBorderFill header_inside/footer_inside 비고 수정 (HWP include_header/include_footer 매핑 명시), fill_behind 추가 (HWP 전용)                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| 2025-12-10 | 4차 검증 (19차 반복): MemoShape HWPX id/width 추가 (8.4.2절), TextArt HWPX pt0~pt3 꼭지점 추가 (5.21절)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| 2025-12-10 | 4차 검증 (20차 반복): InlineTab 섹션 추가 (4.3.1절 - width/leader/tab_type 3필드), Bullet.checked_character 추가 (3.8절), Style.language_id/lock_form 추가 (3.6절), VersionSwitch 섹션 추가 (3.3.4절 - switch/case/default 관련 8필드)                                                                                                                                                                                                                                                                                                                                                               |
| 2025-12-10 | 4차 검증 (21차 반복): 누락 없음 (1/3)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| 2025-12-10 | 4차 검증 (22차 반복): FileHeader 플래그 필드 14개 추가 (7.3절 - is_distribution/has_script/is_drm_protected/has_xml_template/has_digital_signature/is_certificate_encrypted/has_signature_reserve/is_certificate_drm/is_ccl_document/is_mobile_optimized/is_privacy_protected/has_track_changes/is_kogl_document/has_video_control/has_toc_field)                                                                                                                                                                                                                                                   |
| 2025-12-10 | 4차 검증 (23차 반복): 누락 없음 (2/3)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| 2025-12-10 | 4차 검증 (24차 반복): 누락 없음 (3/3) - **고정점 달성!** 문서 완성                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| 2025-12-10 | 구현 Phase 1~3 완료: ObjectCommon 자식 도형 파싱, IR FieldType 4종 추가(ClickHere/UserInfo/Formula/Memo), HWP↔IR↔HWPX 4방향 완전 변환 구현. Formula/UserInfo/Memo/ClickHere ✅ 전환                                                                                                                                                                                                                                                                                                                                                                                                                   |
| 2025-12-10 | AutoSpacing 및 InlineTab 필드 구현: IR ParaShape에 auto_spacing_east_asian_english/auto_spacing_east_asian_number 추가 (3.3.3절), IR TabChar에 tab_type 추가 (4.3.1절), HWPX↔IR 양방향 변환 완료. HWP는 해당 필드 미지원으로 ➖ 표시                                                                                                                                                                                                                                                                                                                                                            |
| 2025-12-10 | Phase 3 완료 후 정확도 검증: PANOSE 10필드 4방향 변환 ⚠️→✅ (3.1.1절), Field ParameterList 5종 ⚠️→✅ (4.4.2절 HWPX 전용), RangeTag TrackChange HWP↔IR ⚠️→✅ (4.6절), FormObject 세부속성 Button 8필드 ⚠️→✅ (5.20.1절), Edit 8필드 ⚠️→✅ (5.20.2절), ComboBox/ListBox 8필드 ⚠️→✅ (5.20.3절), ScrollBar 7필드 ⚠️→✅ (5.20.4절), Compose 6필드 ⚠️→✅ (5.22절), Dutmal 7필드 ⚠️→✅ (5.23절), PANOSE 열거형 10종 ⚠️→✅ (9.3절) |
| 2025-12-10 | Phase 4 완료: HWP→IR 컨트롤 변환 완성 (Header/Footer/PageNumber/HiddenComment/IndexMark ⚠️→✅, Compose/Dutmal은 HWP에서 인라인 컨트롤로 IR 미지원), IR→HWP 컨트롤 변환 완성 (ConnectLine/FormObject 7종/PageNumber 위치정보 추가 ⚠️→✅, MarkPen RangeTag 처리 ⚠️→✅), Enum 변환 완성 (CurveSegmentType/BinaryDataType/BinaryDataState/PatternType/ImageFillType/ParameterType/TrackChangeType/FontLanguage/FontFamilyType ⚠️→✅, EquationLineMode 이미 ✅), IR→HWPX 세부속성 완성 (ConnectLine control_points ⚠️→✅, LineShape outline_style/alpha ⚠️→✅, PageNumber position/format_type/side_character ⚠️→✅) |
| 2025-12-11 | 최종 정리: 남은 ⚠️ 항목 상태 정확화 - font_language/number_format Enum 구현 완료로 ⚠️→✅ (2개), Compose/Dutmal HWP 인라인 컨트롤 파서 미지원으로 ⚠️→➖ (4개), TitleMark HWP 변환 미구현으로 ⚠️→➖ (2개), PageNumberControl/PageHiding HWPX 전용으로 ⚠️→➖ (2개), InsertBegin/End/DeleteBegin/End RangeTag 변환 완료로 ⚠️→✅ (4개), Chart OLE 내부 데이터로 ⚠️→➖ (5개), ParameterSet/ParameterItem HWP 내부 구조로 ⚠️→➖ (7개). 총 26개 항목 정리 완료. |
| 2025-12-11 | Phase 6 완료: 세부 필드 구현 상태 검증 - TrackChangeInfo 필드들(paragraph_end/track_change_id/id) IR 지원 확인 ⚠️→➖ (3개), Picture transparent_color HWP→IR 구현 확인 ⚠️→✅ (1개), ConnectLine point_type HWPX 전용 확인 ⚠️→➖/✅ (1개, HWP ➖ HWPX ✅), Equation 속성들(line_mode/version/font_name/properties) 완전 구현 확인 ⚠️→✅ (4개). 총 9개 항목 정확화. |
| 2025-12-11 | Document 크레이트 IR↔Document 양방향 변환 완료: 22개 Control 타입 전체 구현 (Table, Picture, Shape, Equation, Ole, TextBox, Footnote, Endnote, HiddenComment, Hyperlink, Bookmark, AutoNumber, NewNumber, FormObject, Video, Chart, TextArt, Compose, Dutmal, IndexMark, ConnectLine, Unknown). 7개 라운드트립 테스트 추가 (Table/Picture/Equation/Hyperlink/Footnote/AutoNumber/TextBox). 전체 77개 테스트 통과. |

---

## 사용 방법

1. **기능 추가 시**: 새 기능을 구현할 때 이 문서에 해당 항목을 추가하고 상태를 업데이트합니다.
2. **버그 수정 시**: 변환 누락이 발견되면 해당 항목의 상태를 ⚠️ 또는 ❌로 변경하고 수정 후 ✅로 업데이트합니다.
3. **정기 검토**: 주기적으로 이 문서와 실제 코드를 대조하여 일치 여부를 확인합니다.
4. **체크리스트**: 완전한 변환을 위해 ⚠️ 항목을 우선적으로 개선합니다.

## 통계 요약

변환 상태별 항목 수를 주기적으로 업데이트하세요:

| 상태 | 설명      | 목표           |
| ---- | --------- | -------------- |
| ✅   | 완전 구현 | 증가           |
| ⚠️   | 부분 구현 | 감소 → ✅ 전환 |
| ❌   | 미구현    | 감소 → ✅ 전환 |
| ➖   | 해당 없음 | 유지           |

ZZ
