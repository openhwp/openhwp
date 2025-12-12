//! 각주/미주 관련 타입
//!
//! 문서의 각주와 미주 설정에 사용되는 타입들입니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Color, HwpUnit, LineType, NumberFormat};

/// 각주/미주 번호 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NoteNumberPositionType {
    /// 위 첨자
    #[default]
    Superscript,
    /// 아래 첨자
    Subscript,
}

/// 각주/미주 기본 모양
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NoteShapeBase {
    /// 번호 형식
    pub number_format: NumberFormat,
    /// 번호 매김 방식 (연속/섹션별/페이지별)
    pub numbering: crate::NoteNumbering,
    /// 위 첨자 여부
    pub superscript: bool,
    /// 접두사
    pub prefix: Option<String>,
    /// 접미사
    pub suffix: Option<String>,
    /// 시작 번호
    pub start_number: u32,
    /// 사용자 기호 (번호 형식이 사용자 정의일 때)
    pub user_character: Option<String>,
    /// 구분선 길이 (HwpUnit 절대값, 또는 페이지/단 너비 기준 상대값)
    pub separator_length: HwpUnit,
    /// 구분선 위치 (HWP 전용, 여백으로부터의 거리)
    pub separator_position: Option<HwpUnit>,
    /// 구분선 종류
    pub separator_line_type: LineType,
    /// 구분선 굵기 (0.1mm 단위)
    pub separator_line_width: u8,
    /// 구분선 색상
    pub separator_line_color: Color,
    /// 본문과의 간격 (구분선 위)
    pub space_above: HwpUnit,
    /// 구분선 아래 간격
    pub space_below: HwpUnit,
    /// 각주 간 간격
    pub space_between: HwpUnit,
    /// 텍스트에 이어 바로 출력 여부
    pub beneath_text: bool,
}

/// 각주 구분선
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NoteSeparatorLine {
    /// 선 종류
    pub line_type: LineType,
    /// 선 두께
    pub width: HwpUnit,
    /// 선 색상
    pub color: Color,
    /// 선 길이 (퍼센트)
    pub length: crate::Percent,
}
