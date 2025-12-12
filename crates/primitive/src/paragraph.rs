//! 문단 관련 타입
//!
//! 문단의 서식과 속성을 정의합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{BorderFillId, HeadingType, HwpUnit, TabLeader, TabType};

/// 줄 간격 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LineSpacingType {
    /// 퍼센트
    #[default]
    Percent,
    /// 고정값
    Fixed,
    /// 최소
    AtLeast,
    /// 글꼴 기준
    FontBased,
}

/// 표 페이지 나누기
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TablePageBreak {
    /// 없음
    #[default]
    None,
    /// 셀 단위
    Cell,
    /// 표 단위
    Table,
}

/// 문단 번호 매기기
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ParagraphNumbering {
    /// 머리 종류
    pub heading_type: HeadingType,
    /// 번호 매기기 ID
    pub numbering_id: Option<u32>,
    /// 글머리 기호 ID
    pub bullet_id: Option<u32>,
    /// 수준
    pub level: u8,
}

/// 탭 정의
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TabDef {
    /// 탭 목록
    pub tabs: Vec<Tab>,
    /// 자동 탭 간격 (없으면 비활성)
    pub auto_tab_interval: Option<HwpUnit>,
}

/// 탭
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tab {
    /// 위치
    pub position: HwpUnit,
    /// 탭 종류
    pub tab_type: TabType,
    /// 채움선
    pub leader: TabLeader,
}

/// 문단 테두리
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ParagraphBorder {
    /// 테두리/채우기 참조 ID
    pub border_fill_id_ref: BorderFillId,
    /// 왼쪽 오프셋
    pub offset_left: HwpUnit,
    /// 오른쪽 오프셋
    pub offset_right: HwpUnit,
    /// 위쪽 오프셋
    pub offset_top: HwpUnit,
    /// 아래쪽 오프셋
    pub offset_bottom: HwpUnit,
    /// 연결 여부
    pub connect: bool,
    /// 여백 무시 여부
    pub ignore_margin: bool,
}

/// 자동 번호 형식
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AutoNumberFormat {
    /// 사용자 기호
    pub user_character: Option<String>,
    /// 앞 장식 문자
    pub prefix_character: Option<String>,
    /// 뒤 장식 문자
    pub suffix_character: String,
    /// 위첨자 형식 여부
    pub superscript: bool,
    /// 번호 위치 (HWPX PageNumber 전용 - 11종)
    pub position: Option<crate::PageNumberPosition>,
    /// 번호 모양 종류 (HWPX PageNumber 전용)
    pub format_type: Option<crate::NumberFormat>,
    /// 줄표 넣기 문자 (HWPX PageNumber 전용, 기본값 "-")
    pub side_character: Option<String>,
}
