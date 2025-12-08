//! [AI 생성 문서화] 문단 머리 정보
//!
//! 번호/글머리표 문단에서 레벨별 머리 문자열, 정렬, 들여쓰기 기준을 정의합니다. 제어코드(^)로 상위 레벨 번호를 포함하는 문자열 포맷을 지정합니다. KS X 6101:2024 `header.xsd` 기반.

use serde::{Deserialize, Serialize};

use crate::core::enums::NumberFormatType1;
use crate::core::types::CharShapeIdRef;

/// [AI 생성] 문단 머리 정렬
///
/// 원본: `paraHead.align` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ParagraphHeadAlignment {
    /// [AI 생성] 왼쪽 정렬
    #[default]
    #[serde(rename = "LEFT")]
    Left,
    /// [AI 생성] 가운데 정렬
    #[serde(rename = "CENTER")]
    Center,
    /// [AI 생성] 오른쪽 정렬
    #[serde(rename = "RIGHT")]
    Right,
}

/// [AI 생성] 본문 거리 단위 유형
///
/// 원본: `paraHead.textOffsetType` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TextOffsetType {
    /// [AI 생성] 퍼센트
    #[default]
    #[serde(rename = "PERCENT")]
    Percent,
    /// [AI 생성] HWP 유닛
    #[serde(rename = "HWPUNIT")]
    HwpUnit,
}

/// [AI 생성] 문단 머리 정보
///
/// 각 번호 문단 머리의 정보. 문자열 내 특정 문자에 제어코드(^)를 붙임으로써 다음에서 표시되는 번호 문단 머리의 포맷을 제어한다.
/// - ^n : 레벨 경로를 표시한다. (예: 1.1.1.1.1.1.1)
/// - ^N : 레벨 경로를 표시하며 마지막에 마침표를 하나 더 찍는다. (예: 1.1.1.1.1.1.1.)
/// - ^레벨번호(1-7) : 해당 레벨에 해당하는 숫자 또는 문자 또는 기호를 표시한다.
///
/// 원본: `ParaHeadType`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "paraHead")]
pub struct ParagraphHead {
    /// [AI 생성] 내용 텍스트 (mixed content)
    #[serde(rename = "$text", default)]
    pub text: String,

    /// [AI 생성] 시작 번호 (`start` 속성)
    #[serde(rename = "@start", default = "default_start")]
    pub start: u32,

    /// [AI 생성] 수준 (`level` 속성)
    #[serde(rename = "@level")]
    pub level: u32,

    /// [AI 생성] 정렬 (`align` 속성). 머리표 텍스트 정렬.
    #[serde(rename = "@align", default)]
    pub alignment: ParagraphHeadAlignment,

    /// [AI 생성] 번호 너비를 실제 인스턴스 문자열 너비에 맞출지 여부 (`useInstWidth` 속성). true면 내용 폭에 맞춤.
    #[serde(rename = "@useInstWidth", default = "default_true")]
    pub use_instance_width: bool,

    /// [AI 생성] 자동 내어쓰기 여부 (`autoIndent` 속성). 번호 폭만큼 본문 들여쓰기.
    #[serde(rename = "@autoIndent", default = "default_true")]
    pub auto_indent: bool,

    /// [AI 생성] 번호 너비 보정값 hwpunit (`widthAdjust` 속성). 추가 보정 오프셋.
    #[serde(rename = "@widthAdjust", default)]
    pub width_adjust: i32,

    /// [AI 생성] 수준별 본문 거리 단위 (`textOffsetType` 속성). 퍼센트/HWP 유닛 선택.
    #[serde(rename = "@textOffsetType", default)]
    pub text_offset_type: TextOffsetType,

    /// [AI 생성] 본문과의 거리 (`textOffset` 속성). 머리표와 본문 사이 간격.
    #[serde(rename = "@textOffset", default = "default_text_offset")]
    pub text_offset: i32,

    /// [AI 생성] 번호 포맷 (`numFormat` 속성, 글머리표에서는 미사용). 숫자/로마자/알파 등.
    #[serde(rename = "@numFormat", default)]
    pub number_format: NumberFormatType1,

    /// [AI 생성] 글자 모양 아이디 참조 (`charPrIDRef` 속성). 머리표 전용 글자 모양.
    #[serde(rename = "@charPrIDRef", skip_serializing_if = "Option::is_none")]
    pub character_shape_id_reference: Option<CharShapeIdRef>,

    /// [AI 생성] 확인용 글머리표 (`checkable` 속성). 체크박스 스타일 여부.
    #[serde(rename = "@checkable", skip_serializing_if = "Option::is_none")]
    pub checkable: Option<bool>,
}

fn default_start() -> u32 {
    1
}

fn default_true() -> bool {
    true
}

fn default_text_offset() -> i32 {
    50
}
