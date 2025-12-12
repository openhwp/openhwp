//! 메모 관련 타입
//!
//! 메모(Memo/Annotation)의 속성을 정의합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 메모 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MemoType {
    /// 일반
    #[default]
    Normal,
    /// 사용자 삽입
    UserInsert,
    /// 사용자 삭제
    UserDelete,
    /// 사용자 수정
    UserUpdate,
}
