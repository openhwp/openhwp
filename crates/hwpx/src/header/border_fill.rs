//! [AI 생성 문서화] 테두리 채우기
//!
//! KS X 6101:2024 - header.xsd 기반 설명입니다. 실제 스키마는 `docs/hwpx/schemas/header.xsd`를 참고하세요.

use serde::{Deserialize, Serialize};

use crate::core::{
    enums::{LineStyleType2, LineWidth},
    types::{FillBrush, RgbColor},
};

/// [AI 생성] 슬래시 유형
///
/// 원본: `SlashType.type` 속성의 익명 타입. 셀 대각선 표현 방식을 지정합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SlashDiagonalType {
    /// 없음
    #[default]
    #[serde(rename = "NONE")]
    None,
    /// 중심선 하나
    #[serde(rename = "CENTER")]
    Center,
    /// 중심선 + 중심선 아래의 사선
    #[serde(rename = "CENTER_BELOW")]
    CenterBelow,
    /// 중심선 + 중심선 위의 사선
    #[serde(rename = "CENTER_ABOVE")]
    CenterAbove,
    /// 중심선 + 중심선 아래의 사선 + 중심선 위의 사선
    #[serde(rename = "ALL")]
    All,
}

/// [AI 생성] 중심선 유형
///
/// 원본: `centerLine` 속성의 익명 타입. 셀 가운데 그려지는 기준선을 제어합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CenterLineType {
    /// 없음
    #[default]
    #[serde(rename = "NONE")]
    None,
    /// 세로
    #[serde(rename = "VERTICAL")]
    Vertical,
    /// 가로
    #[serde(rename = "HORIZONTAL")]
    Horizontal,
    /// 교차
    #[serde(rename = "CROSS")]
    Cross,
}

/// [AI 생성] 슬래시 대각선
///
/// 원본: `SlashType`. 대각선 종류와 변형 옵션을 포함합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slash {
    /// [AI 생성] 대각선 유형 (`type` 속성)
    #[serde(rename = "@type")]
    pub diagonal_type: SlashDiagonalType,

    /// [AI 생성] 꺾은선 여부 (`Crooked` 속성)
    #[serde(rename = "@Crooked")]
    pub crooked: bool,

    /// [AI 생성] 역방향 여부 (`isCounter` 속성)
    #[serde(rename = "@isCounter")]
    pub is_counter: bool,
}

/// [AI 생성] 테두리선
///
/// 원본: `BorderType`. 선 종류/굵기/색상을 묶습니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Border {
    /// [AI 생성] 테두리선 종류 (`type` 속성)
    #[serde(rename = "@type")]
    pub line_type: LineStyleType2,

    /// [AI 생성] 테두리선 굵기 (`width` 속성)
    #[serde(rename = "@width")]
    pub width: LineWidth,

    /// [AI 생성] 테두리선 색상 (`color` 속성)
    #[serde(rename = "@color")]
    pub color: RgbColor,
}

/// [AI 생성] 테두리 채우기
///
/// 원본: `BorderFillType`. 셀/문단 배경에 적용되는 선, 슬래시, 채우기 브러시 정보를 담습니다.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "borderFill")]
pub struct BorderFill {
    /// [AI 생성] 슬래시 대각선 (`slash` 요소)
    #[serde(rename = "slash", skip_serializing_if = "Option::is_none")]
    pub slash: Option<Slash>,

    /// [AI 생성] 역슬래시 대각선 (`backSlash` 요소)
    #[serde(rename = "backSlash", skip_serializing_if = "Option::is_none")]
    pub back_slash: Option<Slash>,

    /// 왼쪽 테두리
    ///
    /// 원본: `leftBorder` 요소
    #[serde(rename = "leftBorder", skip_serializing_if = "Option::is_none")]
    pub left_border: Option<Border>,

    /// 오른쪽 테두리
    ///
    /// 원본: `rightBorder` 요소
    #[serde(rename = "rightBorder", skip_serializing_if = "Option::is_none")]
    pub right_border: Option<Border>,

    /// 위쪽 테두리
    ///
    /// 원본: `topBorder` 요소
    #[serde(rename = "topBorder", skip_serializing_if = "Option::is_none")]
    pub top_border: Option<Border>,

    /// [AI 생성] 아래쪽 테두리 (`bottomBorder` 요소)
    #[serde(rename = "bottomBorder", skip_serializing_if = "Option::is_none")]
    pub bottom_border: Option<Border>,

    /// [AI 생성] 대각선 테두리 (`diagonal` 요소)
    #[serde(rename = "diagonal", skip_serializing_if = "Option::is_none")]
    pub diagonal: Option<Border>,

    /// [AI 생성] 채우기 브러시 (`fillBrush` 요소)
    #[serde(rename = "fillBrush", skip_serializing_if = "Option::is_none")]
    pub fill_brush: Option<FillBrush>,

    /// [AI 생성] 테두리 채우기 아이디 (`id` 속성)
    #[serde(rename = "@id")]
    pub id: u32,

    /// [AI 생성] 3차원 효과 여부 (`threeD` 속성)
    #[serde(rename = "@threeD", default)]
    pub three_dimensional: bool,

    /// [AI 생성] 그림자 효과 여부 (`shadow` 속성)
    #[serde(rename = "@shadow", default)]
    pub shadow: bool,

    /// [AI 생성] 중심선 종류 (`centerLine` 속성)
    #[serde(rename = "@centerLine", skip_serializing_if = "Option::is_none")]
    pub center_line: Option<CenterLineType>,

    /// [AI 생성] 자동으로 나눈 표의 경계선 설정 여부 (`breakCellSeparateLine` 속성)
    #[serde(rename = "@breakCellSeparateLine", default)]
    pub break_cell_separate_line: bool,
}

/// [AI 생성] 테두리 채우기 목록
///
/// 원본: `borderFills` 요소의 익명 타입. 여러 테두리 채우기 정의와 개수를 포함합니다.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "borderFills")]
pub struct BorderFillList {
    /// [AI 생성] 테두리 채우기 목록 (`borderFill` 요소)
    #[serde(rename = "borderFill")]
    pub border_fills: Vec<BorderFill>,

    /// [AI 생성] 항목 개수 (`itemCnt` 속성)
    #[serde(rename = "@itemCnt")]
    pub item_count: u32,
}
