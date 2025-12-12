//! 호환성 및 확장 관련 타입
//!
//! 문서 호환성 및 확장 설정을 정의합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// HWPX 대상 프로그램
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HwpxTargetProgram {
    /// 한글 2010 이후
    #[default]
    Hwp201X,
    /// 한글 2007 이전
    Hwp200X,
    /// Microsoft Word
    MsWord,
}

/// 배포용 문서 데이터
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DistributeDocData {
    /// 플래그
    pub flags: u32,
    /// 데이터
    pub data: Vec<u8>,
}

/// 레이아웃 호환성
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LayoutCompatibility {
    /// 대상 버전
    pub target_version: Option<String>,
    /// 플래그
    pub flags: u32,
}

/// 변경 이력 항목
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChangeHistoryEntry {
    /// 타임스탬프
    pub timestamp: String,
    /// 작성자
    pub author: Option<String>,
    /// 설명
    pub description: Option<String>,
}

/// 프레젠테이션 설정
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PresentationSettings {
    /// 전환 효과
    pub transition: Option<String>,
    /// 자동 전환 시간 (초)
    pub auto_advance_seconds: Option<u32>,
}

/// HWPX 레이아웃 호환성
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HwpxLayoutCompatibility {
    /// 대상 프로그램
    pub target_program: HwpxTargetProgram,
    /// 플래그
    pub flags: u64,
}

/// HWPX 문서 옵션
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HwpxDocumentOption {
    /// 연결 문서 경로
    pub link_document_path: Option<String>,
    /// 플래그
    pub flags: u32,
}

/// 변경 추적 설정
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TrackChangeConfig {
    /// 활성화 여부
    pub enabled: bool,
    /// 삽입 색상
    pub insert_color: Option<String>,
    /// 삭제 색상
    pub delete_color: Option<String>,
}
