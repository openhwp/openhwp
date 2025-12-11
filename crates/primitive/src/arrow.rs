//! 화살표 관련 열거형

/// 화살표 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrowType {
    /// 없음
    #[default]
    None,
    /// 일반 화살표
    Arrow,
    /// 속이 빈 화살표
    ArrowOpen,
    /// 스텔스 화살표
    Stealth,
    /// 다이아몬드
    Diamond,
    /// 원
    Circle,
    /// 사각형
    Square,
}

/// 화살표 크기
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrowSize {
    /// 작은
    Small,
    /// 중간
    #[default]
    Medium,
    /// 큰
    Large,
}
