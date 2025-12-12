//! 글자 스타일 관련 타입
//!
//! 글자 모양에 사용되는 스타일 타입들을 정의합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Color, EmphasisType, HwpUnit, ShadowType, UnderlinePosition, UnderlineType};

/// 밑줄 스타일
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnderlineStyle {
    /// 밑줄 종류
    pub line_type: UnderlineType,
    /// 밑줄 위치
    pub position: UnderlinePosition,
    /// 밑줄 색상 (None이면 글자 색상 사용)
    pub color: Option<Color>,
}

/// 강조점 스타일
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EmphasisStyle {
    /// 강조점 종류
    pub emphasis_type: EmphasisType,
    /// 강조점 색상 (None이면 글자 색상 사용)
    pub color: Option<Color>,
}

/// 그림자 스타일 (텍스트용)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CharShadowStyle {
    /// 그림자 종류
    pub shadow_type: ShadowType,
    /// 그림자 색상
    pub color: Option<Color>,
    /// X 오프셋
    pub offset_x: HwpUnit,
    /// Y 오프셋
    pub offset_y: HwpUnit,
}

/// 대체 글꼴
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubstituteFont {
    /// 글꼴 이름
    pub face: String,
    /// 글꼴 타입
    pub font_type: FontType,
    /// 임베드 여부
    pub is_embedded: bool,
    /// 바이너리 아이템 참조 ID
    pub binary_item_id_ref: Option<String>,
}

/// 글꼴 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FontType {
    /// 대표 글꼴
    #[default]
    Representative,
    /// TrueType 글꼴
    TrueType,
    /// 한글 전용 글꼴
    HangeulOnly,
}

/// 글꼴 패밀리
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FontFamily {
    /// 알 수 없음
    #[default]
    Unknown,
    /// 세리프
    Serif,
    /// 산세리프
    SansSerif,
    /// 장식체
    Decorative,
    /// 스크립트
    Script,
    /// 고정폭
    Monospace,
}
