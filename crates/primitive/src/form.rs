//! 양식 객체 관련 타입
//!
//! 양식(Form) 컨트롤의 속성을 정의합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 스크롤바 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScrollBarType {
    /// 수평 스크롤바
    #[default]
    Horizontal,
    /// 수직 스크롤바
    Vertical,
}

/// 버튼 상태 값
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ButtonValue {
    /// 선택 해제 상태
    #[default]
    Unchecked,
    /// 선택됨 상태
    Checked,
    /// 불확정(혼합) 상태
    Indeterminate,
}

/// 버튼 배경 스타일
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ButtonBackStyle {
    /// 배경 투명
    #[default]
    Transparent,
    /// 배경 불투명
    Opaque,
}

/// Edit 스크롤바 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EditScrollBars {
    /// 스크롤바 없음
    #[default]
    None,
    /// 세로 스크롤바만 표시
    Vertical,
    /// 가로 스크롤바만 표시
    Horizontal,
    /// 가로/세로 모두 표시
    Both,
}

/// Edit 탭키 동작
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EditTabKeyBehavior {
    /// 탭키로 다음 개체로 이동
    #[default]
    NextObject,
    /// 탭 문자를 입력
    InsertTab,
}

/// 양식 글자 속성
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FormCharProperty {
    /// 글자 모양 참조 ID
    pub char_shape_id: Option<u32>,
    /// 문맥 따라가기 여부
    pub follow_context: bool,
    /// 자동 크기 여부
    pub auto_size: bool,
    /// 자동 줄바꿈 여부
    pub word_wrap: bool,
}

/// 양식 목록 항목 (ComboBox, ListBox용)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FormListItem {
    /// 표시 텍스트
    pub display_text: Option<String>,
    /// 값
    pub value: Option<String>,
}
