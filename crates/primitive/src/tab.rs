//! 탭 관련 타입
//!
//! 탭 정렬 및 채움선 타입을 정의합니다.

/// 탭 정렬 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum TabType {
    /// 왼쪽 정렬
    #[default]
    Left = 0,
    /// 오른쪽 정렬
    Right = 1,
    /// 가운데 정렬
    Center = 2,
    /// 소수점 정렬
    Decimal = 3,
}

impl TabType {
    /// raw 값에서 생성
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::Center,
            3 => Self::Decimal,
            _ => Self::Left,
        }
    }

    /// raw 값으로 반환
    pub const fn as_raw(self) -> u8 {
        self as u8
    }
}

/// 탭 채움선 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum TabLeader {
    /// 없음
    #[default]
    None = 0,
    /// 점선
    Dot = 1,
    /// 긴 점선
    LongDash = 2,
    /// 파선
    Dash = 3,
    /// 밑줄
    Underscore = 4,
}

impl TabLeader {
    /// raw 값에서 생성
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Dot,
            2 => Self::LongDash,
            3 => Self::Dash,
            4 => Self::Underscore,
            _ => Self::None,
        }
    }

    /// raw 값으로 반환
    pub const fn as_raw(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_type() {
        assert_eq!(TabType::from_raw(0), TabType::Left);
        assert_eq!(TabType::from_raw(1), TabType::Right);
        assert_eq!(TabType::from_raw(2), TabType::Center);
        assert_eq!(TabType::from_raw(3), TabType::Decimal);
        assert_eq!(TabType::from_raw(99), TabType::Left);
    }

    #[test]
    fn test_tab_leader() {
        assert_eq!(TabLeader::from_raw(0), TabLeader::None);
        assert_eq!(TabLeader::from_raw(1), TabLeader::Dot);
        assert_eq!(TabLeader::from_raw(4), TabLeader::Underscore);
        assert_eq!(TabLeader::from_raw(99), TabLeader::None);
    }
}
