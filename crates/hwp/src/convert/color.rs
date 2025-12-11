//! Color conversion utilities for HWP ↔ IR conversions.
//!
//! This module provides unified color conversion functions to avoid
//! code duplication across the conversion modules.

use crate::primitive::ColorReference;
use primitive::Color;

/// HWP ↔ IR color conversion utilities.
///
/// Provides methods for converting between HWP's ColorReference (BGR format)
/// and IR's Color (RGBA format).
pub struct ColorConvert;

impl ColorConvert {
    /// Converts HWP ColorReference to IR Color.
    ///
    /// HWP uses BGR format (0x00BBGGRR), where:
    /// - Bits 0-7: Red
    /// - Bits 8-15: Green
    /// - Bits 16-23: Blue
    #[inline]
    pub fn to_ir(color_ref: ColorReference) -> Color {
        let value = color_ref.value();
        Color {
            red: (value & 0xFF) as u8,
            green: ((value >> 8) & 0xFF) as u8,
            blue: ((value >> 16) & 0xFF) as u8,
            alpha: 255,
        }
    }

    /// Converts IR Color to HWP ColorReference.
    #[inline]
    pub fn from_ir(color: &Color) -> ColorReference {
        ColorReference::from_rgb(color.red, color.green, color.blue)
    }

    /// Converts IR Color to u32 in BGR format.
    ///
    /// This is useful when writing raw color values to HWP binary data.
    #[inline]
    pub fn to_bgr_u32(color: &Color) -> u32 {
        ((color.blue as u32) << 16) | ((color.green as u32) << 8) | (color.red as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ir() {
        // Red color in BGR format: 0x0000FF
        let color_ref = ColorReference::from_rgb(255, 0, 0);
        let ir_color = ColorConvert::to_ir(color_ref);
        assert_eq!(ir_color.red, 255);
        assert_eq!(ir_color.green, 0);
        assert_eq!(ir_color.blue, 0);
        assert_eq!(ir_color.alpha, 255);
    }

    #[test]
    fn test_from_ir() {
        let ir_color = Color {
            red: 100,
            green: 150,
            blue: 200,
            alpha: 255,
        };
        let color_ref = ColorConvert::from_ir(&ir_color);
        // Convert back to verify
        let back = ColorConvert::to_ir(color_ref);
        assert_eq!(back.red, 100);
        assert_eq!(back.green, 150);
        assert_eq!(back.blue, 200);
    }

    #[test]
    fn test_to_bgr_u32() {
        let ir_color = Color {
            red: 0x12,
            green: 0x34,
            blue: 0x56,
            alpha: 255,
        };
        let bgr = ColorConvert::to_bgr_u32(&ir_color);
        // BGR format: 0x00BBGGRR
        assert_eq!(bgr, 0x00563412);
    }

    #[test]
    fn test_roundtrip() {
        let original = ColorReference::from_rgb(128, 64, 32);
        let ir_color = ColorConvert::to_ir(original);
        let back = ColorConvert::from_ir(&ir_color);
        assert_eq!(back.value(), original.value());
    }
}
