//! ID 타입 정의
//!
//! 문서 내 각 요소를 고유하게 식별하는 ID 타입들입니다.
//! SlotMap의 키 타입을 사용하여 안정적인 ID와 O(1) 조회를 제공합니다.

use slotmap::new_key_type;

// SlotMap 키 타입 정의
new_key_type! {
    /// 섹션 ID
    pub struct SectionId;

    /// 문단 ID
    pub struct ParagraphId;

    /// 런 ID
    pub struct RunId;

    /// 컨트롤 ID
    pub struct ControlId;

    /// 표 행 ID
    pub struct RowId;

    /// 표 셀 ID
    pub struct CellId;

    /// 머리글/바닥글 ID
    pub struct HeaderFooterId;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_traits() {
        // Copy, Clone 확인
        let id = SectionId::default();
        let id2 = id;
        let id3 = id.clone();
        assert_eq!(id, id2);
        assert_eq!(id, id3);
    }

    #[test]
    fn test_id_equality() {
        let id1 = ParagraphId::default();
        let id2 = ParagraphId::default();
        // default()는 null 키를 반환하므로 같음
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let id = RunId::default();
        set.insert(id);
        assert!(set.contains(&id));
    }
}
