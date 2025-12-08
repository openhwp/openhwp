//! [AI 생성 문서화] 속성 그룹 관련 타입
//!
//! KS X 6101:2024 - core.xsd를 근거로 하며, 이 파일의 모든 타입/필드 설명은 AI가 작성했습니다. 실제 스키마(`docs/hwpx/schemas/core.xsd`)와 차이가 있으면 TODO로 남겨 주세요.

use serde::{Deserialize, Serialize};

use super::enums::{BorderLineStyle, BorderLineWidth};

/// [AI 생성] 여백 정보
///
/// 원본: `MarginAttributeGroup`. 단위는 HWPUNIT.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Margin {
    /// [AI 생성] 왼쪽 여백 (`left` 속성, HWPUNIT)
    #[serde(rename = "@left")]
    pub left: u32,

    /// [AI 생성] 오른쪽 여백 (`right` 속성, HWPUNIT)
    #[serde(rename = "@right")]
    pub right: u32,

    /// [AI 생성] 위 여백 (`top` 속성, HWPUNIT)
    #[serde(rename = "@top")]
    pub top: u32,

    /// [AI 생성] 아래 여백 (`bottom` 속성, HWPUNIT)
    #[serde(rename = "@bottom")]
    pub bottom: u32,
}

/// [AI 생성] 테두리 속성 (공통)
///
/// 원본: `BorderAttributeGroup`. 테두리선 종류/굵기/색상을 묶어 관리합니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BorderAttributes {
    /// [AI 생성] 테두리선 종류 (`type` 속성)
    #[serde(rename = "@type", default)]
    pub border_type: BorderLineStyle,

    /// [AI 생성] 테두리선 굵기 (`width` 속성)
    #[serde(rename = "@width", default)]
    pub width: BorderLineWidth,

    /// [AI 생성] 테두리선 색상 (`color` 속성, RGB 0x00bbggrr 십진수)
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
}

impl Default for BorderAttributes {
    fn default() -> Self {
        Self {
            border_type: BorderLineStyle::Solid,
            width: BorderLineWidth::Mm0_12,
            color: None,
        }
    }
}
