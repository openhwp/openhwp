//! [AI 생성 문서화] 시작 번호 정보
//!
//! 페이지·각주·미주·그림·표·수식의 기본 시작 번호를 한 곳에 정의합니다. 스타일/번호 정의와 조합해 문서 전체의 번호 체계를 결정합니다. KS X 6101:2024 `header.xsd` 기준.

use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

/// [AI 생성] 시작 번호
///
/// 원본: `beginNum` 요소의 익명 타입. 각 번호 체계의 초기 값을 정의합니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeginNumber {
    /// [AI 생성] 페이지 시작 번호 (`page` 속성). 섹션 첫 페이지 기본값.
    #[serde(rename = "@page")]
    pub page: NonZeroU32,

    /// [AI 생성] 각주 시작 번호 (`footnote` 속성)
    #[serde(rename = "@footnote")]
    pub footnote: NonZeroU32,

    /// [AI 생성] 미주 시작 번호 (`endnote` 속성)
    #[serde(rename = "@endnote")]
    pub endnote: NonZeroU32,

    /// [AI 생성] 그림 시작 번호 (`pic` 속성)
    #[serde(rename = "@pic")]
    pub picture: NonZeroU32,

    /// [AI 생성] 표 시작 번호 (`tbl` 속성)
    #[serde(rename = "@tbl")]
    pub table: NonZeroU32,

    /// [AI 생성] 수식 시작 번호 (`equation` 속성)
    #[serde(rename = "@equation")]
    pub equation: NonZeroU32,
}

impl Default for BeginNumber {
    fn default() -> Self {
        Self {
            page: NonZeroU32::new(1).unwrap(),
            footnote: NonZeroU32::new(1).unwrap(),
            endnote: NonZeroU32::new(1).unwrap(),
            picture: NonZeroU32::new(1).unwrap(),
            table: NonZeroU32::new(1).unwrap(),
            equation: NonZeroU32::new(1).unwrap(),
        }
    }
}
