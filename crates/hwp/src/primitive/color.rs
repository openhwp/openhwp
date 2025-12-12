//! Color representation for HWP documents.
//!
//! HWP uses COLORREF format (0x00BBGGRR) for colors.

use std::fmt;

/// Color value in COLORREF format (0x00BBGGRR).
///
/// This is the same format used by Windows GDI.
/// The byte order is: Blue, Green, Red (from high to low byte).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ColorReference(u32);

impl ColorReference {
    /// Black color (0x00000000).
    pub const BLACK: Self = Self(0x00000000);

    /// White color (0x00FFFFFF).
    pub const WHITE: Self = Self(0x00FFFFFF);

    /// Red color (0x000000FF).
    pub const RED: Self = Self(0x000000FF);

    /// Green color (0x0000FF00).
    pub const GREEN: Self = Self(0x0000FF00);

    /// Blue color (0x00FF0000).
    pub const BLUE: Self = Self(0x00FF0000);

    /// Creates a new ColorReference from raw COLORREF value.
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Creates a color from RGB components.
    #[inline]
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self((blue as u32) << 16 | (green as u32) << 8 | (red as u32))
    }

    /// Returns the raw COLORREF value.
    #[inline]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Returns the red component (0-255).
    #[inline]
    pub const fn red(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Returns the green component (0-255).
    #[inline]
    pub const fn green(self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Returns the blue component (0-255).
    #[inline]
    pub const fn blue(self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Returns the color as (red, green, blue) tuple.
    #[inline]
    pub const fn to_rgb(self) -> (u8, u8, u8) {
        (self.red(), self.green(), self.blue())
    }

    /// Converts to CSS hex color string (e.g., "#FF0000").
    pub fn to_css_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red(), self.green(), self.blue())
    }

    /// Reads from little-endian bytes.
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(bytes))
    }

    /// Converts to little-endian bytes.
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

impl fmt::Debug for ColorReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ColorReference")
            .field("value", &format_args!("0x{:06X}", self.0))
            .field(
                "rgb",
                &format_args!("({}, {}, {})", self.red(), self.green(), self.blue()),
            )
            .finish()
    }
}

impl fmt::Display for ColorReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css_hex())
    }
}

impl From<u32> for ColorReference {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<ColorReference> for u32 {
    #[inline]
    fn from(color: ColorReference) -> Self {
        color.0
    }
}

impl From<(u8, u8, u8)> for ColorReference {
    #[inline]
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::from_rgb(r, g, b)
    }
}

// ============================================================================
// primitive::Color 변환
// ============================================================================

impl From<primitive::Color> for ColorReference {
    /// primitive::Color를 ColorReference로 변환
    ///
    /// primitive::Color는 ARGB, ColorReference는 BGR (알파 무시)
    #[inline]
    fn from(color: primitive::Color) -> Self {
        Self::from_rgb(color.red, color.green, color.blue)
    }
}

impl From<ColorReference> for primitive::Color {
    /// ColorReference를 primitive::Color로 변환
    ///
    /// ColorReference는 알파가 없으므로 255(불투명)로 설정
    #[inline]
    fn from(color: ColorReference) -> Self {
        Self::rgb(color.red(), color.green(), color.blue())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_components() {
        let color = ColorReference::from_rgb(0x12, 0x34, 0x56);
        assert_eq!(color.red(), 0x12);
        assert_eq!(color.green(), 0x34);
        assert_eq!(color.blue(), 0x56);
    }

    #[test]
    fn test_color_value() {
        let color = ColorReference::from_rgb(0xFF, 0x00, 0x00);
        assert_eq!(color.value(), 0x000000FF);

        let color = ColorReference::from_rgb(0x00, 0xFF, 0x00);
        assert_eq!(color.value(), 0x0000FF00);

        let color = ColorReference::from_rgb(0x00, 0x00, 0xFF);
        assert_eq!(color.value(), 0x00FF0000);
    }

    #[test]
    fn test_color_css_hex() {
        let color = ColorReference::from_rgb(255, 128, 0);
        assert_eq!(color.to_css_hex(), "#FF8000");
    }

    #[test]
    fn test_color_le_bytes() {
        let color = ColorReference::new(0x00563412);
        let bytes = color.to_le_bytes();
        assert_eq!(bytes, [0x12, 0x34, 0x56, 0x00]);
        assert_eq!(ColorReference::from_le_bytes(bytes), color);
    }

    #[test]
    fn test_primitive_color_conversion() {
        // ColorReference -> primitive::Color
        let color_ref = ColorReference::from_rgb(255, 128, 64);
        let primitive_color: primitive::Color = color_ref.into();
        assert_eq!(primitive_color.red, 255);
        assert_eq!(primitive_color.green, 128);
        assert_eq!(primitive_color.blue, 64);
        assert_eq!(primitive_color.alpha, 255); // 불투명

        // primitive::Color -> ColorReference
        let primitive_color = primitive::Color::rgb(100, 150, 200);
        let color_ref: ColorReference = primitive_color.into();
        assert_eq!(color_ref.red(), 100);
        assert_eq!(color_ref.green(), 150);
        assert_eq!(color_ref.blue(), 200);
    }
}
