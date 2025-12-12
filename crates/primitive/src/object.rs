//! 개체 공통 타입
//!
//! 문서 내 개체(컨트롤)에서 공통으로 사용되는 타입들입니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::HwpUnit;

/// 개체 여백
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ObjectMargin {
    /// 왼쪽 여백
    pub left: HwpUnit,
    /// 오른쪽 여백
    pub right: HwpUnit,
    /// 위쪽 여백
    pub top: HwpUnit,
    /// 아래쪽 여백
    pub bottom: HwpUnit,
}

/// Edit 컨트롤 텍스트 정렬
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EditTextAlignment {
    /// 좌측 정렬
    #[default]
    Left,
    /// 중앙 정렬
    Center,
    /// 우측 정렬
    Right,
}

/// 글맵시 글꼴 스타일
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextArtFontStyle {
    /// 일반
    #[default]
    Regular,
    /// 굵게
    Bold,
    /// 기울임
    Italic,
    /// 굵은 기울임
    BoldItalic,
}

/// 글맵시 모양
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextArtShapeType {
    /// 사각형
    #[default]
    Rectangle,
    /// 원형
    Circle,
    /// 아치형 위
    ArchUp,
    /// 아치형 아래
    ArchDown,
    /// 물결
    Wave,
    /// 실린더
    Cylinder,
    /// 볼록
    Inflate,
    /// 오목
    Deflate,
    /// 기타
    Other(u32),
}
