//! 문서 단위 체계
//!
//! HWP와 HWPX 모두 HwpUnit(1/7200 인치)을 기본 단위로 사용합니다.
//! 이 모듈은 단위 변환과 관련 복합 타입을 제공합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// HWP 기본 단위 (1/7200 인치)
///
/// - 1 포인트 = 100 HwpUnit
/// - 1 인치 = 7200 HwpUnit
/// - 1 mm ≈ 283.465 HwpUnit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HwpUnit(pub i32);

impl HwpUnit {
    /// 인치당 단위 수 (7200)
    pub const UNITS_PER_INCH: i32 = 7200;

    /// 0 단위
    pub const ZERO: Self = Self(0);

    /// HwpUnit 값 생성
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// 포인트에서 변환 (1pt = 100 HwpUnit)
    pub const fn from_pt(pt: f64) -> Self {
        Self((pt * 100.0) as i32)
    }

    /// 포인트로 변환 (to_points의 별칭)
    pub const fn to_pt(self) -> f64 {
        self.0 as f64 / 100.0
    }

    /// 포인트로 변환
    #[inline]
    pub const fn to_points(self) -> f64 {
        self.to_pt()
    }

    /// 밀리미터에서 변환
    pub const fn from_mm(mm: f64) -> Self {
        Self((mm * 7200.0 / 25.4) as i32)
    }

    /// 밀리미터로 변환
    pub const fn to_mm(self) -> f64 {
        self.0 as f64 * 25.4 / 7200.0
    }

    /// 인치에서 변환 (from_inches의 별칭)
    pub const fn from_inch(inch: f64) -> Self {
        Self((inch * 7200.0) as i32)
    }

    /// 인치에서 변환
    #[inline]
    pub const fn from_inches(inches: f64) -> Self {
        Self::from_inch(inches)
    }

    /// 인치로 변환 (to_inches의 별칭)
    pub const fn to_inch(self) -> f64 {
        self.0 as f64 / 7200.0
    }

    /// 인치로 변환
    #[inline]
    pub const fn to_inches(self) -> f64 {
        self.to_inch()
    }

    /// 센티미터로 변환
    #[inline]
    pub const fn to_centimeters(self) -> f64 {
        self.to_inches() * 2.54
    }

    /// 내부 값 반환
    pub const fn value(self) -> i32 {
        self.0
    }

    /// Little-endian 바이트에서 읽기
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self(i32::from_le_bytes(bytes))
    }

    /// Little-endian 바이트로 변환
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

impl From<i32> for HwpUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<u32> for HwpUnit {
    fn from(value: u32) -> Self {
        Self(value as i32)
    }
}

impl From<u16> for HwpUnit {
    fn from(value: u16) -> Self {
        Self(value as i32)
    }
}

impl From<HwpUnit> for i32 {
    fn from(unit: HwpUnit) -> Self {
        unit.0
    }
}

/// 2차원 크기
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Size {
    /// 너비
    pub width: HwpUnit,
    /// 높이
    pub height: HwpUnit,
}

impl Size {
    /// 크기 생성
    pub const fn new(width: HwpUnit, height: HwpUnit) -> Self {
        Self { width, height }
    }

    /// 0 크기
    pub const ZERO: Self = Self {
        width: HwpUnit::ZERO,
        height: HwpUnit::ZERO,
    };
}

/// 2차원 좌표
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point {
    /// X 좌표
    pub x: HwpUnit,
    /// Y 좌표
    pub y: HwpUnit,
}

impl Point {
    /// 좌표 생성
    pub const fn new(x: HwpUnit, y: HwpUnit) -> Self {
        Self { x, y }
    }

    /// 원점 (0, 0)
    pub const ZERO: Self = Self {
        x: HwpUnit::ZERO,
        y: HwpUnit::ZERO,
    };
}

/// 사각형 영역
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect {
    /// 왼쪽 상단 좌표
    pub origin: Point,
    /// 크기
    pub size: Size,
}

impl Rect {
    /// 사각형 생성
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    /// 좌표와 크기로 생성
    pub const fn from_xywh(x: HwpUnit, y: HwpUnit, width: HwpUnit, height: HwpUnit) -> Self {
        Self {
            origin: Point::new(x, y),
            size: Size::new(width, height),
        }
    }
}

/// 네 방향 여백/간격
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Insets {
    /// 왼쪽
    pub left: HwpUnit,
    /// 오른쪽
    pub right: HwpUnit,
    /// 위
    pub top: HwpUnit,
    /// 아래
    pub bottom: HwpUnit,
}

impl Insets {
    /// 여백 생성
    pub const fn new(left: HwpUnit, right: HwpUnit, top: HwpUnit, bottom: HwpUnit) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// 모든 방향 동일한 여백
    pub const fn all(value: HwpUnit) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    /// 0 여백
    pub const ZERO: Self = Self {
        left: HwpUnit::ZERO,
        right: HwpUnit::ZERO,
        top: HwpUnit::ZERO,
        bottom: HwpUnit::ZERO,
    };
}

/// 백분율 값 (0.0 ~ 100.0 또는 그 이상)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Percent(pub f64);

impl Percent {
    /// 백분율 생성
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    /// 100%
    pub const HUNDRED: Self = Self(100.0);

    /// 0%
    pub const ZERO: Self = Self(0.0);

    /// 비율로 변환 (100% = 1.0)
    pub const fn to_ratio(self) -> f64 {
        self.0 / 100.0
    }

    /// 비율에서 변환
    pub const fn from_ratio(ratio: f64) -> Self {
        Self(ratio * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hwp_unit_conversions() {
        // 1 포인트 = 100 HwpUnit
        let unit = HwpUnit::from_pt(10.0);
        assert_eq!(unit.0, 1000);
        assert!((unit.to_pt() - 10.0).abs() < 0.001);

        // 1 인치 = 7200 HwpUnit
        let unit = HwpUnit::from_inch(1.0);
        assert_eq!(unit.0, 7200);
        assert!((unit.to_inch() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_size() {
        let size = Size::new(HwpUnit(1000), HwpUnit(2000));
        assert_eq!(size.width.0, 1000);
        assert_eq!(size.height.0, 2000);
    }

    #[test]
    fn test_insets() {
        let insets = Insets::all(HwpUnit(100));
        assert_eq!(insets.left, insets.right);
        assert_eq!(insets.top, insets.bottom);
    }
}
