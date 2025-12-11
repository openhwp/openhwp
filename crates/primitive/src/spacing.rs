//! 여백/패딩 정의
//!
//! 네 방향 여백/패딩 값을 나타내는 제네릭 구조체입니다.

/// 네 방향 여백 또는 패딩
///
/// 왼쪽, 오른쪽, 위, 아래 방향의 여백 값을 나타냅니다.
/// 다양한 단위 타입과 함께 사용할 수 있는 제네릭 구조체입니다.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Spacing<T> {
    /// 왼쪽 여백
    pub left: T,
    /// 오른쪽 여백
    pub right: T,
    /// 위쪽 여백
    pub top: T,
    /// 아래쪽 여백
    pub bottom: T,
}

impl<T> Spacing<T> {
    /// 주어진 값으로 새 Spacing 생성
    pub const fn new(left: T, right: T, top: T, bottom: T) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

impl<T: Copy> Spacing<T> {
    /// 모든 방향에 동일한 값으로 Spacing 생성
    pub const fn all(value: T) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    /// 가로/세로 값으로 Spacing 생성
    pub const fn symmetric(horizontal: T, vertical: T) -> Self {
        Self {
            left: horizontal,
            right: horizontal,
            top: vertical,
            bottom: vertical,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_new() {
        let spacing = Spacing::new(1u32, 2u32, 3u32, 4u32);
        assert_eq!(spacing.left, 1);
        assert_eq!(spacing.right, 2);
        assert_eq!(spacing.top, 3);
        assert_eq!(spacing.bottom, 4);
    }

    #[test]
    fn test_spacing_default() {
        let spacing: Spacing<u32> = Spacing::default();
        assert_eq!(spacing.left, 0);
        assert_eq!(spacing.right, 0);
        assert_eq!(spacing.top, 0);
        assert_eq!(spacing.bottom, 0);
    }

    #[test]
    fn test_spacing_all() {
        let spacing = Spacing::all(10u32);
        assert_eq!(spacing.left, 10);
        assert_eq!(spacing.right, 10);
        assert_eq!(spacing.top, 10);
        assert_eq!(spacing.bottom, 10);
    }

    #[test]
    fn test_spacing_symmetric() {
        let spacing = Spacing::symmetric(5u32, 10u32);
        assert_eq!(spacing.left, 5);
        assert_eq!(spacing.right, 5);
        assert_eq!(spacing.top, 10);
        assert_eq!(spacing.bottom, 10);
    }
}
