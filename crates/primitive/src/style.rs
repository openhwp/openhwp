//! 스타일 관련 타입
//!
//! 문서 스타일(문단 스타일, 글자 스타일)의 종류를 정의합니다.

/// 스타일 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum StyleType {
    /// 문단 스타일
    #[default]
    Paragraph = 0,
    /// 글자 스타일
    Character = 1,
}

impl StyleType {
    /// raw 값에서 생성
    pub const fn from_raw(value: u8) -> Self {
        match value & 0x07 {
            0 => Self::Paragraph,
            1 => Self::Character,
            _ => Self::Paragraph,
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
    fn test_style_type() {
        assert_eq!(StyleType::from_raw(0), StyleType::Paragraph);
        assert_eq!(StyleType::from_raw(1), StyleType::Character);
        assert_eq!(StyleType::from_raw(8), StyleType::Paragraph); // masked
        assert_eq!(StyleType::from_raw(9), StyleType::Character); // masked
    }

    #[test]
    fn test_as_raw() {
        assert_eq!(StyleType::Paragraph.as_raw(), 0);
        assert_eq!(StyleType::Character.as_raw(), 1);
    }
}
