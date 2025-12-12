//! 변경 추적 시스템
//!
//! 문서의 변경 내역을 추적하고 관리합니다.

use crate::cursor::Position;

/// 변경 추적 관리자
#[derive(Debug, Clone, Default)]
pub struct TrackChangeManager {
    /// 변경 추적 활성화 여부
    enabled: bool,
    /// 변경 내역 목록
    changes: Vec<TrackChange>,
    /// 현재 사용자 ID
    current_user_id: Option<String>,
    /// 현재 사용자 이름
    current_user_name: Option<String>,
    /// 다음 변경 ID
    next_id: u32,
}

impl TrackChangeManager {
    /// 새 변경 추적 관리자 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 변경 추적 활성화
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// 변경 추적 비활성화
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// 변경 추적 활성화 여부
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// 현재 사용자 설정
    pub fn set_current_user(&mut self, user_id: impl Into<String>, name: impl Into<String>) {
        self.current_user_id = Some(user_id.into());
        self.current_user_name = Some(name.into());
    }

    /// 변경 기록
    pub fn record_change(
        &mut self,
        change_type: TrackChangeType,
        position: Position,
    ) -> Option<u32> {
        if !self.enabled {
            return None;
        }

        let id = self.next_id;
        self.next_id += 1;

        let change = TrackChange {
            id,
            change_type,
            position,
            user_id: self.current_user_id.clone(),
            user_name: self.current_user_name.clone(),
            timestamp: current_timestamp(),
            accepted: false,
            rejected: false,
        };

        self.changes.push(change);
        Some(id)
    }

    /// 삽입 기록
    pub fn record_insert(&mut self, position: Position, text: impl Into<String>) -> Option<u32> {
        self.record_change(TrackChangeType::Insert { text: text.into() }, position)
    }

    /// 삭제 기록
    pub fn record_delete(&mut self, position: Position, text: impl Into<String>) -> Option<u32> {
        self.record_change(TrackChangeType::Delete { text: text.into() }, position)
    }

    /// 서식 변경 기록
    pub fn record_format_change(
        &mut self,
        position: Position,
        description: impl Into<String>,
    ) -> Option<u32> {
        self.record_change(
            TrackChangeType::Format {
                description: description.into(),
            },
            position,
        )
    }

    /// 변경 승인
    pub fn accept_change(&mut self, change_id: u32) -> bool {
        if let Some(change) = self
            .changes
            .iter_mut()
            .find(|c| c.id == change_id && !c.rejected)
        {
            change.accepted = true;
            return true;
        }
        false
    }

    /// 변경 거부
    pub fn reject_change(&mut self, change_id: u32) -> bool {
        if let Some(change) = self
            .changes
            .iter_mut()
            .find(|c| c.id == change_id && !c.accepted)
        {
            change.rejected = true;
            return true;
        }
        false
    }

    /// 모든 변경 승인
    pub fn accept_all(&mut self) {
        for change in &mut self.changes {
            if !change.rejected {
                change.accepted = true;
            }
        }
    }

    /// 모든 변경 거부
    pub fn reject_all(&mut self) {
        for change in &mut self.changes {
            if !change.accepted {
                change.rejected = true;
            }
        }
    }

    /// 변경 목록 반환
    pub fn changes(&self) -> &[TrackChange] {
        &self.changes
    }

    /// 미결 변경 목록 반환
    pub fn pending_changes(&self) -> Vec<&TrackChange> {
        self.changes
            .iter()
            .filter(|c| !c.accepted && !c.rejected)
            .collect()
    }

    /// 변경 수
    pub const fn change_count(&self) -> usize {
        self.changes.len()
    }

    /// 미결 변경 수
    pub fn pending_count(&self) -> usize {
        self.changes
            .iter()
            .filter(|c| !c.accepted && !c.rejected)
            .count()
    }

    /// 특정 ID의 변경 조회
    pub fn get_change(&self, change_id: u32) -> Option<&TrackChange> {
        self.changes.iter().find(|c| c.id == change_id)
    }

    /// 변경 내역 초기화
    pub fn clear(&mut self) {
        self.changes.clear();
        self.next_id = 0;
    }

    /// 승인/거부된 변경 제거
    pub fn cleanup(&mut self) {
        self.changes.retain(|c| !c.accepted && !c.rejected);
    }
}

/// 변경 추적 항목
#[derive(Debug, Clone)]
pub struct TrackChange {
    /// 변경 ID
    pub id: u32,
    /// 변경 타입
    pub change_type: TrackChangeType,
    /// 변경 위치
    pub position: Position,
    /// 사용자 ID
    pub user_id: Option<String>,
    /// 사용자 이름
    pub user_name: Option<String>,
    /// 타임스탬프
    pub timestamp: String,
    /// 승인 여부
    pub accepted: bool,
    /// 거부 여부
    pub rejected: bool,
}

impl TrackChange {
    /// 미결 상태인지 확인
    pub const fn is_pending(&self) -> bool {
        !self.accepted && !self.rejected
    }
}

/// 변경 타입
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackChangeType {
    /// 삽입
    Insert {
        /// 삽입된 텍스트
        text: String,
    },
    /// 삭제
    Delete {
        /// 삭제된 텍스트
        text: String,
    },
    /// 서식 변경
    Format {
        /// 변경 설명
        description: String,
    },
    /// 이동
    Move {
        /// 원래 위치
        from: Position,
        /// 새 위치
        to: Position,
    },
    /// 표 변경
    TableChange {
        /// 변경 설명
        description: String,
    },
}

/// 현재 타임스탬프 생성
fn current_timestamp() -> String {
    // 간단한 구현 - 실제로는 chrono 등 사용
    "2025-01-01T00:00:00Z".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_change_manager_creation() {
        let manager = TrackChangeManager::new();
        assert!(!manager.is_enabled());
        assert_eq!(manager.change_count(), 0);
    }

    #[test]
    fn test_enable_disable() {
        let mut manager = TrackChangeManager::new();
        manager.enable();
        assert!(manager.is_enabled());
        manager.disable();
        assert!(!manager.is_enabled());
    }

    #[test]
    fn test_record_changes() {
        let mut manager = TrackChangeManager::new();
        manager.enable();
        manager.set_current_user("user1", "Test User");

        let id1 = manager.record_insert(Position::start(), "Hello").unwrap();
        let id2 = manager.record_delete(Position::start(), "World").unwrap();

        assert_eq!(manager.change_count(), 2);
        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
    }

    #[test]
    fn test_disabled_no_record() {
        let mut manager = TrackChangeManager::new();
        // 활성화하지 않음

        let result = manager.record_insert(Position::start(), "Hello");
        assert!(result.is_none());
        assert_eq!(manager.change_count(), 0);
    }

    #[test]
    fn test_accept_reject() {
        let mut manager = TrackChangeManager::new();
        manager.enable();

        let id1 = manager.record_insert(Position::start(), "Hello").unwrap();
        let id2 = manager.record_insert(Position::start(), "World").unwrap();

        assert!(manager.accept_change(id1));
        assert!(manager.reject_change(id2));

        assert!(manager.get_change(id1).unwrap().accepted);
        assert!(manager.get_change(id2).unwrap().rejected);

        assert_eq!(manager.pending_count(), 0);
    }

    #[test]
    fn test_accept_all() {
        let mut manager = TrackChangeManager::new();
        manager.enable();

        manager.record_insert(Position::start(), "A");
        manager.record_insert(Position::start(), "B");
        manager.record_insert(Position::start(), "C");

        assert_eq!(manager.pending_count(), 3);

        manager.accept_all();

        assert_eq!(manager.pending_count(), 0);
        for change in manager.changes() {
            assert!(change.accepted);
        }
    }

    #[test]
    fn test_cleanup() {
        let mut manager = TrackChangeManager::new();
        manager.enable();

        let id1 = manager.record_insert(Position::start(), "A").unwrap();
        manager.record_insert(Position::start(), "B");
        let id3 = manager.record_insert(Position::start(), "C").unwrap();

        manager.accept_change(id1);
        manager.reject_change(id3);

        assert_eq!(manager.change_count(), 3);

        manager.cleanup();

        assert_eq!(manager.change_count(), 1); // B만 남음
    }

    #[test]
    fn test_pending_changes() {
        let mut manager = TrackChangeManager::new();
        manager.enable();

        let id1 = manager.record_insert(Position::start(), "A").unwrap();
        manager.record_insert(Position::start(), "B");
        manager.record_insert(Position::start(), "C");

        manager.accept_change(id1);

        let pending = manager.pending_changes();
        assert_eq!(pending.len(), 2);
    }
}
