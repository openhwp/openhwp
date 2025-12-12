//! HWP unit types for measurements.
//!
//! HWP uses a custom unit system where 1 HwpUnit = 1/7200 inch.
//! This module provides type-safe wrappers for these units.

use std::fmt;

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
    pub const fn to_inches(self) -> f64 {
        self.0 as f64 / Self::UNITS_PER_INCH as f64
    }

    /// Converts to points (1 point = 1/72 inch).
    #[inline]
    pub const fn to_points(self) -> f64 {
        self.0 as f64 / 100.0
    }

    /// Reads from little-endian bytes.
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; 2]) -> Self {
        Self(i16::from_le_bytes(bytes))
    }

    /// Converts to little-endian bytes.
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }

    /// Converts to primitive::HwpUnit (32-bit).
    #[inline]
    pub const fn to_hwp_unit(self) -> primitive::HwpUnit {
        primitive::HwpUnit::new(self.0 as i32)
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
