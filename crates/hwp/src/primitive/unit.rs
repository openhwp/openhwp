//! HWP unit types for measurements.
//!
//! HWP uses a custom unit system where 1 HwpUnit = 1/7200 inch.
//! This module provides type-safe wrappers for these units.

use std::fmt;

/// HWP internal unit (unsigned, 32-bit).
///
/// Represents measurements in 1/7200 inch units.
/// For example, 1 inch = 7200 HwpUnit, 1 cm = 2834.6 HwpUnit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HwpUnit(u32);

impl HwpUnit {
    /// Units per inch (7200).
    pub const UNITS_PER_INCH: u32 = 7200;

    /// Creates a new HwpUnit from raw value.
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw value.
    #[inline]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Converts to inches.
    #[inline]
    pub fn to_inches(self) -> f64 {
        self.0 as f64 / Self::UNITS_PER_INCH as f64
    }

    /// Converts to centimeters.
    #[inline]
    pub fn to_centimeters(self) -> f64 {
        self.to_inches() * 2.54
    }

    /// Converts to millimeters.
    #[inline]
    pub fn to_millimeters(self) -> f64 {
        self.to_inches() * 25.4
    }

    /// Converts to points (1 point = 1/72 inch).
    #[inline]
    pub fn to_points(self) -> f64 {
        self.0 as f64 / 100.0
    }

    /// Creates from inches.
    #[inline]
    pub fn from_inches(inches: f64) -> Self {
        Self((inches * Self::UNITS_PER_INCH as f64) as u32)
    }

    /// Creates from centimeters.
    #[inline]
    pub fn from_centimeters(cm: f64) -> Self {
        Self::from_inches(cm / 2.54)
    }

    /// Creates from millimeters.
    #[inline]
    pub fn from_millimeters(mm: f64) -> Self {
        Self::from_inches(mm / 25.4)
    }

    /// Creates from points.
    #[inline]
    pub fn from_points(points: f64) -> Self {
        Self((points * 100.0) as u32)
    }

    /// Reads from little-endian bytes.
    #[inline]
    pub fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(bytes))
    }

    /// Converts to little-endian bytes.
    #[inline]
    pub fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

impl fmt::Display for HwpUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}pt", self.to_points())
    }
}

impl From<u32> for HwpUnit {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<HwpUnit> for u32 {
    #[inline]
    fn from(unit: HwpUnit) -> Self {
        unit.0
    }
}

/// HWP internal unit (signed, 32-bit).
///
/// Same as HwpUnit but allows negative values.
/// Used for offsets and positions that can be negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SignedHwpUnit(i32);

impl SignedHwpUnit {
    /// Units per inch (7200).
    pub const UNITS_PER_INCH: i32 = 7200;

    /// Creates a new SignedHwpUnit from raw value.
    #[inline]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Returns the raw value.
    #[inline]
    pub const fn value(self) -> i32 {
        self.0
    }

    /// Converts to inches.
    #[inline]
    pub fn to_inches(self) -> f64 {
        self.0 as f64 / Self::UNITS_PER_INCH as f64
    }

    /// Converts to centimeters.
    #[inline]
    pub fn to_centimeters(self) -> f64 {
        self.to_inches() * 2.54
    }

    /// Converts to millimeters.
    #[inline]
    pub fn to_millimeters(self) -> f64 {
        self.to_inches() * 25.4
    }

    /// Converts to points (1 point = 1/72 inch).
    #[inline]
    pub fn to_points(self) -> f64 {
        self.0 as f64 / 100.0
    }

    /// Reads from little-endian bytes.
    #[inline]
    pub fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self(i32::from_le_bytes(bytes))
    }

    /// Converts to little-endian bytes.
    #[inline]
    pub fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

impl fmt::Display for SignedHwpUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}pt", self.to_points())
    }
}

impl From<i32> for SignedHwpUnit {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<SignedHwpUnit> for i32 {
    #[inline]
    fn from(unit: SignedHwpUnit) -> Self {
        unit.0
    }
}

/// HWP internal unit (signed, 16-bit).
///
/// A smaller variant of SignedHwpUnit for space-efficient storage.
/// Has the same unit system (1/7200 inch).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HwpUnit16(i16);

impl HwpUnit16 {
    /// Units per inch (7200).
    pub const UNITS_PER_INCH: i16 = 7200;

    /// Creates a new HwpUnit16 from raw value.
    #[inline]
    pub const fn new(value: i16) -> Self {
        Self(value)
    }

    /// Returns the raw value.
    #[inline]
    pub const fn value(self) -> i16 {
        self.0
    }

    /// Converts to inches.
    #[inline]
    pub fn to_inches(self) -> f64 {
        self.0 as f64 / Self::UNITS_PER_INCH as f64
    }

    /// Converts to points (1 point = 1/72 inch).
    #[inline]
    pub fn to_points(self) -> f64 {
        self.0 as f64 / 100.0
    }

    /// Reads from little-endian bytes.
    #[inline]
    pub fn from_le_bytes(bytes: [u8; 2]) -> Self {
        Self(i16::from_le_bytes(bytes))
    }

    /// Converts to little-endian bytes.
    #[inline]
    pub fn to_le_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}

impl fmt::Display for HwpUnit16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}pt", self.to_points())
    }
}

impl From<i16> for HwpUnit16 {
    #[inline]
    fn from(value: i16) -> Self {
        Self(value)
    }
}

impl From<HwpUnit16> for i16 {
    #[inline]
    fn from(unit: HwpUnit16) -> Self {
        unit.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hwp_unit_conversions() {
        let unit = HwpUnit::new(7200);
        assert!((unit.to_inches() - 1.0).abs() < f64::EPSILON);
        assert!((unit.to_centimeters() - 2.54).abs() < 0.001);
        assert!((unit.to_points() - 72.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_hwp_unit_from_inches() {
        let unit = HwpUnit::from_inches(1.0);
        assert_eq!(unit.value(), 7200);
    }

    #[test]
    fn test_hwp_unit_le_bytes() {
        let unit = HwpUnit::new(0x12345678);
        let bytes = unit.to_le_bytes();
        assert_eq!(bytes, [0x78, 0x56, 0x34, 0x12]);
        assert_eq!(HwpUnit::from_le_bytes(bytes), unit);
    }

    #[test]
    fn test_signed_hwp_unit_negative() {
        let unit = SignedHwpUnit::new(-7200);
        assert!((unit.to_inches() + 1.0).abs() < f64::EPSILON);
    }
}
