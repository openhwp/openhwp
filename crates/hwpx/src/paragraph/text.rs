//! [AI 생성] 텍스트 인라인 마크업
//!
//! 문단/런 안에서 글자 수준의 마크업(형광펜, 제목 표시, 탭, 추적 기록)을 담는 요소입니다. 변경 추적과 혼용되므로 `$value` 시퀀스 순서를 유지해야 합니다. KS X 6101:2024 `paralist.xsd`.

use serde::{Deserialize, Serialize};

use super::control::TrackChangeTag;
use super::enums::InlineTabType;
use crate::core::{enums::LineStyleType2, types::{RgbColor, StyleIdRef}};

/// [AI 생성] 형광펜 시작 (`markpenBegin`)
///
/// 원본: `markpenBegin` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "markpenBegin")]
pub struct MarkPenBegin {
    /// [AI 생성] 색상 (`color`)
    ///
    /// 원본: `color` 속성
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<RgbColor>,
}

/// [AI 생성] 형광펜 종료 (`markpenEnd`)
///
/// 원본: `markpenEnd` 요소 (빈 요소)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "markpenEnd")]
pub struct MarkPenEnd;

/// [AI 생성] 제목 표시 (`titleMark`)
///
/// 원본: `titleMark` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "titleMark")]
pub struct TitleMark {
    /// [AI 생성] 무시 여부 (`ignore`, true 시 제목 처리 생략)
    ///
    /// 원본: `ignore` 속성
    #[serde(rename = "@ignore", default)]
    pub ignore: bool,
}

/// [AI 생성] 인라인 탭 (`tab`)
///
/// 원본: `tab` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tab")]
pub struct InlineTab {
    /// [AI 생성] 너비 (HWP 유닛, `width`)
    ///
    /// 원본: `width` 속성
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// [AI 생성] 채움 선 종류 (`leader`)
    ///
    /// 원본: `leader` 속성
    #[serde(rename = "@leader", skip_serializing_if = "Option::is_none")]
    pub leader: Option<LineStyleType2>,

    /// [AI 생성] 탭 종류 (`type`)
    ///
    /// 원본: `type` 속성
    #[serde(rename = "@type")]
    pub tab_type: InlineTabType,
}

/// [AI 생성] 줄바꿈 (`lineBreak`)
///
/// 원본: `lineBreak` 요소 (빈 요소)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "lineBreak")]
pub struct LineBreak;

/// [AI 생성] 하이픈 (`hyphen`)
///
/// 원본: `hyphen` 요소 (빈 요소)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "hyphen")]
pub struct Hyphen;

/// [AI 생성] 비분리 공백 (`nbSpace`)
///
/// 원본: `nbSpace` 요소 (빈 요소)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "nbSpace")]
pub struct NonBreakingSpace;

/// [AI 생성] 고정폭 공백 (`fwSpace`)
///
/// 원본: `fwSpace` 요소 (빈 요소)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "fwSpace")]
pub struct FixedWidthSpace;

/// [AI 생성] 텍스트 내 마크업 항목 (텍스트/변경 추적/제어 코드 등)
///
/// 원본: `t` 요소 내 choice 요소들
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextMarkup {
    /// [AI 생성] 형광펜 시작
    #[serde(rename = "markpenBegin")]
    MarkPenBegin(MarkPenBegin),
    /// [AI 생성] 형광펜 종료
    #[serde(rename = "markpenEnd")]
    MarkPenEnd(MarkPenEnd),
    /// [AI 생성] 제목 표시
    #[serde(rename = "titleMark")]
    TitleMark(TitleMark),
    /// [AI 생성] 탭
    #[serde(rename = "tab")]
    Tab(InlineTab),
    /// [AI 생성] 줄바꿈
    #[serde(rename = "lineBreak")]
    LineBreak(LineBreak),
    /// [AI 생성] 하이픈
    #[serde(rename = "hyphen")]
    Hyphen(Hyphen),
    /// [AI 생성] 비분리 공백
    #[serde(rename = "nbSpace")]
    NonBreakingSpace(NonBreakingSpace),
    /// [AI 생성] 고정폭 공백
    #[serde(rename = "fwSpace")]
    FixedWidthSpace(FixedWidthSpace),
    /// [AI 생성] 삽입 시작
    #[serde(rename = "insertBegin")]
    InsertBegin(TrackChangeTag),
    /// [AI 생성] 삽입 종료
    #[serde(rename = "insertEnd")]
    InsertEnd(TrackChangeTag),
    /// [AI 생성] 삭제 시작
    #[serde(rename = "deleteBegin")]
    DeleteBegin(TrackChangeTag),
    /// [AI 생성] 삭제 종료
    #[serde(rename = "deleteEnd")]
    DeleteEnd(TrackChangeTag),
    /// [AI 생성] 텍스트 내용
    #[serde(rename = "$text")]
    Text(String),
}

/// [AI 생성] 텍스트 블록 (`t` 요소)
///
/// 원본: `t` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "t")]
pub struct TextElement {
    /// [AI 생성] 텍스트 내용 및 마크업 항목들 (`$value` 시퀀스)
    #[serde(rename = "$value", default, skip_serializing_if = "Vec::is_empty")]
    pub contents: Vec<TextMarkup>,

    /// [AI 생성] 글자 스타일 아이디 참조 (`charStyleIDRef`)
    ///
    /// 원본: `charStyleIDRef` 속성
    #[serde(rename = "@charStyleIDRef", skip_serializing_if = "Option::is_none")]
    pub character_style_id_reference: Option<StyleIdRef>,
}

impl TextElement {
    /// 텍스트 내용만 추출
    pub fn text(&self) -> String {
        self.contents
            .iter()
            .filter_map(|m| {
                if let TextMarkup::Text(s) = m {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .collect()
    }
}
