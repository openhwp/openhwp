//! Character shape properties bit field utilities.
//!
//! Provides a builder pattern for constructing the 32-bit properties field
//! used in HWP character shape records.

use bitflags::bitflags;

bitflags! {
    /// Character shape boolean flags (single-bit properties).
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct CharShapeFlags: u32 {
        /// Bit 0: Italic
        const ITALIC = 1 << 0;
        /// Bit 1: Bold
        const BOLD = 1 << 1;
        /// Bit 13: Emboss
        const EMBOSS = 1 << 13;
        /// Bit 14: Engrave
        const ENGRAVE = 1 << 14;
        /// Bit 15: Superscript
        const SUPERSCRIPT = 1 << 15;
        /// Bit 16: Subscript
        const SUBSCRIPT = 1 << 16;
        /// Bit 30: Kerning
        const KERNING = 1 << 30;
    }
}

/// Builder for character shape properties bit field.
///
/// Provides a convenient API for constructing the complex 32-bit properties
/// field that contains both boolean flags and multi-bit values.
#[derive(Clone, Copy, Debug, Default)]
pub struct CharShapePropsBuilder {
    value: u32,
}

impl CharShapePropsBuilder {
    /// Creates a new builder with all bits set to 0.
    pub const fn new() -> Self {
        Self { value: 0 }
    }

    /// Sets a boolean flag.
    #[inline]
    pub const fn with_flag(mut self, flag: CharShapeFlags) -> Self {
        self.value |= flag.bits();
        self
    }

    /// Sets a boolean flag conditionally.
    #[inline]
    pub const fn with_flag_if(self, condition: bool, flag: CharShapeFlags) -> Self {
        if condition {
            self.with_flag(flag)
        } else {
            self
        }
    }

    /// Sets a multi-bit field value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set
    /// * `mask` - The bit mask for this field
    /// * `shift` - The bit position to shift the value to
    #[inline]
    pub const fn with_field(mut self, value: u32, mask: u32, shift: u32) -> Self {
        self.value |= (value & mask) << shift;
        self
    }

    /// Sets the underline position (bits 2-3).
    /// 0 = None, 1 = Bottom, 3 = Top
    #[inline]
    pub const fn with_underline_pos(self, pos: u32) -> Self {
        self.with_field(pos, 0x03, 2)
    }

    /// Sets the underline shape (bits 4-7).
    #[inline]
    pub const fn with_underline_shape(self, shape: u32) -> Self {
        self.with_field(shape, 0x0F, 4)
    }

    /// Sets the outline type (bits 8-10).
    #[inline]
    pub const fn with_outline(self, outline: u32) -> Self {
        self.with_field(outline, 0x07, 8)
    }

    /// Sets the shadow type (bits 11-12).
    #[inline]
    pub const fn with_shadow(self, shadow: u32) -> Self {
        self.with_field(shadow, 0x03, 11)
    }

    /// Sets the strikethrough type (bits 18-20).
    #[inline]
    pub const fn with_strikethrough(self, strike: u32) -> Self {
        self.with_field(strike, 0x07, 18)
    }

    /// Sets the emphasis type (bits 21-24).
    #[inline]
    pub const fn with_emphasis(self, emphasis: u32) -> Self {
        self.with_field(emphasis, 0x0F, 21)
    }

    /// Builds the final 32-bit properties value.
    #[inline]
    pub const fn build(self) -> u32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_shape_flags() {
        let flags = CharShapeFlags::ITALIC | CharShapeFlags::BOLD;
        assert_eq!(flags.bits(), 0b11);
    }

    #[test]
    fn test_builder_basic() {
        let props = CharShapePropsBuilder::new()
            .with_flag(CharShapeFlags::ITALIC)
            .with_flag(CharShapeFlags::BOLD)
            .build();
        assert_eq!(props, 0b11);
    }

    #[test]
    fn test_builder_conditional() {
        let props = CharShapePropsBuilder::new()
            .with_flag_if(true, CharShapeFlags::ITALIC)
            .with_flag_if(false, CharShapeFlags::BOLD)
            .build();
        assert_eq!(props, 0b01);
    }

    #[test]
    fn test_builder_with_fields() {
        let props = CharShapePropsBuilder::new()
            .with_flag(CharShapeFlags::ITALIC)
            .with_underline_pos(1) // Bottom
            .with_underline_shape(2) // Double
            .build();
        // bit 0: italic = 1
        // bits 2-3: underline_pos = 1 (0b01 << 2 = 0b0100)
        // bits 4-7: underline_shape = 2 (0b0010 << 4 = 0b00100000)
        assert_eq!(props, 0b00100101);
    }

    #[test]
    fn test_builder_kerning() {
        let props = CharShapePropsBuilder::new()
            .with_flag(CharShapeFlags::KERNING)
            .build();
        assert_eq!(props, 1 << 30);
    }
}
