//! [AI 생성] 단(Columns) 정의
//!
//! 신문형/균형형 단을 지면에 적용할 때 사용하는 설정입니다. 단 개수 1은 단 없음과 동일하며, `sameSz`에 따라 개별 폭 지정 또는 균등 분배를 결정합니다. KS X 6101:2024 `paralist.xsd`.

use serde::{Deserialize, Serialize};

use super::enums::{ColumnLayout, ColumnType};
use crate::core::{
    enums::{LineStyleType2, LineWidth},
    types::RgbColor,
};

/// [AI 생성] 단 구분선
///
/// 원본: `colLine` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "colLine")]
pub struct ColumnLine {
    /// [AI 생성] 선 종류 (`type` 속성)
    #[serde(rename = "@type", default)]
    pub line_type: LineStyleType2,

    /// [AI 생성] 선 굵기 (`width` 속성)
    #[serde(rename = "@width", default = "default_column_line_width")]
    pub width: LineWidth,

    /// [AI 생성] 선 색상 (`color` 속성)
    #[serde(rename = "@color", default = "RgbColor::black")]
    pub color: RgbColor,
}

fn default_column_line_width() -> LineWidth {
    LineWidth::Mm0_12
}

/// [AI 생성] 단 크기
///
/// sameSize가 false일 때, 각 단의 크기 및 사이 간격
///
/// 원본: `colSz` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "colSz")]
pub struct ColumnSize {
    /// [AI 생성] 너비 (`width` 속성)
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// [AI 생성] 간격 (`gap` 속성)
    #[serde(rename = "@gap", skip_serializing_if = "Option::is_none")]
    pub gap: Option<u32>,
}

/// [AI 생성] 단 정의
///
/// 원본: `ColumnDefType`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "colPr")]
pub struct ColumnDefinition {
    /// [AI 생성] 단 구분선 (`colLine` 요소)
    #[serde(rename = "colLine", skip_serializing_if = "Option::is_none")]
    pub column_line: Option<ColumnLine>,

    /// [AI 생성] 단 크기 (sameSize가 false일 때) (`colSz` 요소)
    #[serde(rename = "colSz", default, skip_serializing_if = "Vec::is_empty")]
    pub column_sizes: Vec<ColumnSize>,

    /// [AI 생성] 단 정의 아이디 (`id` 속성)
    #[serde(rename = "@id")]
    pub id: String,

    /// [AI 생성] 단 종류 (`type` 속성)
    #[serde(rename = "@type", default)]
    pub column_type: ColumnType,

    /// [AI 생성] 단 방향 지정 (`layout` 속성)
    #[serde(rename = "@layout", default)]
    pub layout: ColumnLayout,

    /// [AI 생성] 단 개수 (`colCount` 속성, 1이면 단 없음)
    #[serde(rename = "@colCount", default = "default_column_count")]
    pub column_count: u8,

    /// [AI 생성] 단 너비 동일 여부 (`sameSz` 속성)
    ///
    /// false면 `colSz`로 개별 폭 지정, true면 `sameGap`과 `colCount`로 균등 분배
    #[serde(rename = "@sameSz", default)]
    pub same_size: bool,

    /// [AI 생성] 단 사이 간격 (`sameGap` 속성, sameSize=true일 때 사용)
    #[serde(rename = "@sameGap", default)]
    pub same_gap: u32,
}

fn default_column_count() -> u8 {
    1
}
