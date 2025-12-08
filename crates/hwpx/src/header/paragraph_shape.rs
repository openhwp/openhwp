//! [AI 생성 문서화] 문단 모양
//!
//! KS X 6101:2024 - header.xsd 기반 설명입니다. 실제 스키마는 `docs/hwpx/schemas/header.xsd`를 교차 확인하세요.

use serde::{Deserialize, Serialize};

use crate::core::types::{BorderFillIdRef, HwpValue, ParaShapeIdRef, TabDefIdRef};

/// [AI 생성] 가로 정렬
///
/// 원본: `align.horizontal` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HorizontalAlignment {
    /// [AI 생성] 양쪽 정렬
    #[default]
    #[serde(rename = "JUSTIFY")]
    Justify,
    /// [AI 생성] 왼쪽 정렬
    #[serde(rename = "LEFT")]
    Left,
    /// [AI 생성] 오른쪽 정렬
    #[serde(rename = "RIGHT")]
    Right,
    /// [AI 생성] 가운데 정렬
    #[serde(rename = "CENTER")]
    Center,
    /// [AI 생성] 배분 정렬
    #[serde(rename = "DISTRIBUTE")]
    Distribute,
    /// [AI 생성] 나눔 정렬 (공백에만 배분)
    #[serde(rename = "DISTRIBUTE_SPACE")]
    DistributeSpace,
}

/// [AI 생성] 세로 정렬
///
/// 원본: `align.vertical` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum VerticalAlignment {
    /// [AI 생성] 글꼴 기준
    #[default]
    #[serde(rename = "BASELINE")]
    Baseline,
    /// [AI 생성] 위쪽 정렬
    #[serde(rename = "TOP")]
    Top,
    /// [AI 생성] 가운데 정렬
    #[serde(rename = "CENTER")]
    Center,
    /// [AI 생성] 아래 정렬
    #[serde(rename = "BOTTOM")]
    Bottom,
}

/// [AI 생성] 문단 머리 유형
///
/// 원본: `heading.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HeadingType {
    /// [AI 생성] 없음
    #[default]
    #[serde(rename = "NONE")]
    None,
    /// [AI 생성] 개요
    #[serde(rename = "OUTLINE")]
    Outline,
    /// [AI 생성] 번호
    #[serde(rename = "NUMBER")]
    Number,
    /// [AI 생성] 글머리표
    #[serde(rename = "BULLET")]
    Bullet,
}

/// [AI 생성] 라틴 문자 줄나눔 단위
///
/// 원본: `breakSetting.breakLatinWord` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LatinWordBreak {
    /// [AI 생성] 단어 단위 유지
    #[default]
    #[serde(rename = "KEEP_WORD")]
    KeepWord,
    /// [AI 생성] 하이픈 사용
    #[serde(rename = "HYPHENATION")]
    Hyphenation,
    /// [AI 생성] 글자 단위 분리
    #[serde(rename = "BREAK_WORD")]
    BreakWord,
}

/// [AI 생성] 비라틴 문자 줄나눔 단위
///
/// 원본: `breakSetting.breakNonLatinWord` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum NonLatinWordBreak {
    /// [AI 생성] 어절 단위 유지
    #[default]
    #[serde(rename = "KEEP_WORD")]
    KeepWord,
    /// [AI 생성] 글자 단위 분리
    #[serde(rename = "BREAK_WORD")]
    BreakWord,
}

/// [AI 생성] 줄바꿈 형식
///
/// 원본: `breakSetting.lineWrap` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LineWrapType {
    /// [AI 생성] 일반 줄바꿈
    #[default]
    #[serde(rename = "BREAK")]
    Break,
    /// [AI 생성] 자간 압축 유지
    #[serde(rename = "SQUEEZE")]
    Squeeze,
    /// [AI 생성] 한 줄 유지 (KEEP)
    #[serde(rename = "KEEP")]
    Keep,
}

/// [AI 생성] 줄 간격 유형
///
/// 원본: `lineSpacing.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LineSpacingType {
    /// [AI 생성] 글자 기준 퍼센트
    #[default]
    #[serde(rename = "PERCENT")]
    Percent,
    /// [AI 생성] 고정 값
    #[serde(rename = "FIXED")]
    Fixed,
    /// [AI 생성] 여백만 지정
    #[serde(rename = "BETWEEN_LINES")]
    BetweenLines,
    /// [AI 생성] 최소 값
    #[serde(rename = "AT_LEAST")]
    AtLeast,
}

/// [AI 생성] 줄 간격 단위
///
/// 원본: `lineSpacing.unit` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LineSpacingUnit {
    /// [AI 생성] 글자 단위
    #[serde(rename = "CHAR")]
    Character,
    /// [AI 생성] HWP 유닛
    #[default]
    #[serde(rename = "HWPUNIT")]
    HwpUnit,
}

/// [AI 생성] 문단 정렬
///
/// 원본: `align` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "align")]
pub struct ParagraphAlignment {
    /// [AI 생성] 가로 정렬 (`horizontal` 속성)
    #[serde(rename = "@horizontal")]
    pub horizontal: HorizontalAlignment,

    /// [AI 생성] 세로 정렬 (`vertical` 속성)
    #[serde(rename = "@vertical")]
    pub vertical: VerticalAlignment,
}

/// [AI 생성] 문단 머리
///
/// 원본: `heading` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "heading")]
pub struct ParagraphHeading {
    /// [AI 생성] 문단 머리 유형 (`type` 속성)
    #[serde(rename = "@type")]
    pub heading_type: HeadingType,

    /// [AI 생성] 번호/글머리표 문단 모양 아이디 참조 (`idRef` 속성)
    #[serde(rename = "@idRef")]
    pub id_reference: ParaShapeIdRef,

    /// [AI 생성] 단계 (`level` 속성)
    #[serde(rename = "@level")]
    pub level: u32,
}

/// [AI 생성] 문단 줄나눔 설정
///
/// 원본: `breakSetting` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "breakSetting")]
pub struct ParagraphBreakSetting {
    /// [AI 생성] 라틴 문자 줄나눔 단위 (`breakLatinWord` 속성)
    #[serde(rename = "@breakLatinWord")]
    pub break_latin_word: LatinWordBreak,

    /// [AI 생성] 비라틴 문자 줄나눔 단위 (`breakNonLatinWord` 속성)
    #[serde(rename = "@breakNonLatinWord")]
    pub break_non_latin_word: NonLatinWordBreak,

    /// [AI 생성] 외톨이줄 보호 여부 (`widowOrphan` 속성)
    #[serde(rename = "@widowOrphan")]
    pub widow_orphan: bool,

    /// [AI 생성] 다음 문단과 함께 (`keepWithNext` 속성)
    #[serde(rename = "@keepWithNext")]
    pub keep_with_next: bool,

    /// [AI 생성] 문단 보호 여부 (`keepLines` 속성)
    #[serde(rename = "@keepLines")]
    pub keep_lines: bool,

    /// [AI 생성] 앞에서 쪽나눔 여부 (`pageBreakBefore` 속성)
    #[serde(rename = "@pageBreakBefore")]
    pub page_break_before: bool,

    /// [AI 생성] 한 줄 입력 시 형식 (`lineWrap` 속성)
    #[serde(rename = "@lineWrap")]
    pub line_wrap: LineWrapType,
}

/// [AI 생성] 문단 여백
///
/// 원본: `margin` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "margin")]
pub struct ParagraphMargin {
    /// [AI 생성] 들여쓰기/내어쓰기 (`intent` 요소)
    /// n > 0 들여쓰기, n = 0 기본, n < 0 내어쓰기
    #[serde(rename = "intent")]
    pub indent: HwpValue,

    /// [AI 생성] 왼쪽 여백 (`left` 요소)
    #[serde(rename = "left")]
    pub left: HwpValue,

    /// [AI 생성] 오른쪽 여백 (`right` 요소)
    #[serde(rename = "right")]
    pub right: HwpValue,

    /// [AI 생성] 문단 간격 위 (`prev` 요소)
    #[serde(rename = "prev")]
    pub previous: HwpValue,

    /// [AI 생성] 문단 간격 아래 (`next` 요소)
    #[serde(rename = "next")]
    pub next: HwpValue,
}

/// [AI 생성] 줄 간격
///
/// 원본: `lineSpacing` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "lineSpacing")]
pub struct LineSpacing {
    /// [AI 생성] 줄 간격 종류 (`type` 속성)
    #[serde(rename = "@type")]
    pub spacing_type: LineSpacingType,

    /// [AI 생성] 줄 간격 값 (`value` 속성). Percent일 때 0-500%
    #[serde(rename = "@value")]
    pub value: i32,

    /// [AI 생성] 줄 간격 단위 (`unit` 속성)
    #[serde(rename = "@unit", default)]
    pub unit: LineSpacingUnit,
}

/// [AI 생성] 문단 테두리
///
/// 원본: `border` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "border")]
pub struct ParagraphBorder {
    /// [AI 생성] 테두리/배경 모양 아이디 참조 (`borderFillIDRef` 속성)
    #[serde(rename = "@borderFillIDRef")]
    pub border_fill_id_reference: BorderFillIdRef,

    /// [AI 생성] 문단 테두리 왼쪽 간격 hwpunit (`offsetLeft` 속성)
    #[serde(rename = "@offsetLeft", default)]
    pub offset_left: i32,

    /// [AI 생성] 문단 테두리 오른쪽 간격 hwpunit (`offsetRight` 속성)
    #[serde(rename = "@offsetRight", default)]
    pub offset_right: i32,

    /// [AI 생성] 문단 테두리 위쪽 간격 hwpunit (`offsetTop` 속성)
    #[serde(rename = "@offsetTop", default)]
    pub offset_top: i32,

    /// [AI 생성] 문단 테두리 아래쪽 간격 hwpunit (`offsetBottom` 속성)
    #[serde(rename = "@offsetBottom", default)]
    pub offset_bottom: i32,

    /// [AI 생성] 문단 테두리 연결 여부 (`connect` 속성)
    #[serde(rename = "@connect", default)]
    pub connect: bool,

    /// [AI 생성] 문단 테두리 여백 무시 여부 (`ignoreMargin` 속성)
    #[serde(rename = "@ignoreMargin", default)]
    pub ignore_margin: bool,
}

/// [AI 생성] 문단 자동 간격 조절
///
/// 원본: `autoSpacing` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "autoSpacing")]
pub struct ParagraphAutoSpacing {
    /// [AI 생성] 한글-영어 간격 자동 조절 (`eAsianEng` 속성)
    #[serde(rename = "@eAsianEng")]
    pub east_asian_english: bool,

    /// [AI 생성] 한글-숫자 간격 자동 조절 (`eAsianNum` 속성)
    #[serde(rename = "@eAsianNum")]
    pub east_asian_number: bool,
}

/// [AI 생성] 문단 모양
///
/// 원본: `ParaShapeType` (요소명 `paraPr`)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "paraPr")]
pub struct ParagraphShape {
    /// [AI 생성] 문단 정렬 (`align` 요소)
    #[serde(rename = "align")]
    pub alignment: ParagraphAlignment,

    /// [AI 생성] 문단 머리 (`heading` 요소)
    #[serde(rename = "heading")]
    pub heading: ParagraphHeading,

    /// [AI 생성] 문단 줄나눔 설정 (`breakSetting` 요소)
    #[serde(rename = "breakSetting")]
    pub break_setting: ParagraphBreakSetting,

    /// [AI 생성] 문단 여백 (`margin` 요소, hp:switch 내부 정의 시 None)
    #[serde(rename = "margin", default, skip_serializing_if = "Option::is_none")]
    pub margin: Option<ParagraphMargin>,

    /// [AI 생성] 줄 간격 (`lineSpacing` 요소, hp:switch 내부 정의 시 None)
    #[serde(rename = "lineSpacing", default, skip_serializing_if = "Option::is_none")]
    pub line_spacing: Option<LineSpacing>,

    /// [AI 생성] 문단 테두리 (`border` 요소)
    #[serde(rename = "border")]
    pub border: ParagraphBorder,

    /// [AI 생성] 문단 자동 간격 조절 (`autoSpacing` 요소)
    #[serde(rename = "autoSpacing")]
    pub auto_spacing: ParagraphAutoSpacing,

    /// [AI 생성] 버전 분기 처리 (`switch` 요소, margin/lineSpacing 분기)
    #[serde(rename = "switch", default, skip_serializing_if = "Option::is_none")]
    pub switch: Option<VersionSwitch>,

    /// [AI 생성] 문단 모양 아이디 (`id` 속성)
    #[serde(rename = "@id")]
    pub id: u32,

    /// [AI 생성] 탭 정의 아이디 참조 (`tabPrIDRef` 속성)
    #[serde(rename = "@tabPrIDRef", skip_serializing_if = "Option::is_none")]
    pub tab_definition_id_reference: Option<TabDefIdRef>,

    /// [AI 생성] 공백 최소값 0-75% (`condense` 속성)
    #[serde(rename = "@condense", skip_serializing_if = "Option::is_none")]
    pub condense: Option<u8>,

    /// [AI 생성] 글꼴에 어울리는 줄 높이 사용 여부 (`fontLineHeight` 속성)
    #[serde(rename = "@fontLineHeight", default)]
    pub font_line_height: bool,

    /// [AI 생성] 편집 용지 줄 격자 사용 여부 (`snapToGrid` 속성)
    #[serde(rename = "@snapToGrid", default = "default_true")]
    pub snap_to_grid: bool,

    /// [AI 생성] 줄 번호 건너뜀 (`suppressLineNumbers` 속성)
    #[serde(rename = "@suppressLineNumbers", default)]
    pub suppress_line_numbers: bool,

    /// [AI 생성] 선택 글머리표 여부 (`checked` 속성)
    #[serde(rename = "@checked", default)]
    pub checked: bool,
}

fn default_true() -> bool {
    true
}

/// [AI 생성] 버전 분기 케이스
///
/// hp:switch 요소 내의 hp:case. 특정 네임스페이스 조건에서 적용되는 값.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "case")]
pub struct VersionSwitchCase {
    /// [AI 생성] 필요한 네임스페이스 (`required-namespace` 속성)
    #[serde(rename = "@required-namespace", default)]
    pub required_namespace: Option<String>,

    /// [AI 생성] 문단 여백 (케이스 버전)
    #[serde(rename = "margin", default, skip_serializing_if = "Option::is_none")]
    pub margin: Option<ParagraphMargin>,

    /// [AI 생성] 줄 간격 (케이스 버전)
    #[serde(rename = "lineSpacing", default, skip_serializing_if = "Option::is_none")]
    pub line_spacing: Option<LineSpacing>,
}

/// [AI 생성] 버전 분기 기본값
///
/// hp:switch 요소 내 hp:default. 조건이 없을 때 적용되는 기본 값.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "default")]
pub struct VersionSwitchDefault {
    /// [AI 생성] 문단 여백 (기본 버전)
    #[serde(rename = "margin", default, skip_serializing_if = "Option::is_none")]
    pub margin: Option<ParagraphMargin>,

    /// [AI 생성] 줄 간격 (기본 버전)
    #[serde(rename = "lineSpacing", default, skip_serializing_if = "Option::is_none")]
    pub line_spacing: Option<LineSpacing>,
}

/// [AI 생성] 버전 분기 처리
///
/// hp:switch 요소. HWPX 버전 호환을 위해 다른 값을 분기 저장합니다.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "switch")]
pub struct VersionSwitch {
    /// [AI 생성] 조건부 케이스 (`case` 요소)
    #[serde(rename = "case", default, skip_serializing_if = "Option::is_none")]
    pub case: Option<VersionSwitchCase>,

    /// [AI 생성] 기본값 (`default` 요소)
    #[serde(rename = "default", default, skip_serializing_if = "Option::is_none")]
    pub default: Option<VersionSwitchDefault>,
}

/// [AI 생성] 문단 모양 목록
///
/// 원본: `paraShapes` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "paraShapes")]
pub struct ParagraphShapeList {
    /// [AI 생성] 문단 모양 목록 (`paraShape` 요소)
    #[serde(rename = "paraShape")]
    pub paragraph_shapes: Vec<ParagraphShape>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성)
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}
