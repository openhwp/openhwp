//! Text art (WordArt-like) parsing.
//!
//! Text art allows decorative text with various shapes and effects,
//! similar to WordArt in Microsoft Word.

use crate::error::Result;
use crate::util::ByteReader;

/// Text art shape type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextArtShape {
    /// Plain text (no deformation).
    #[default]
    Plain,
    /// Wave shape.
    Wave,
    /// Arc up shape.
    ArcUp,
    /// Arc down shape.
    ArcDown,
    /// Circle shape.
    Circle,
    /// Button shape.
    Button,
    /// Inflate shape.
    Inflate,
    /// Deflate shape.
    Deflate,
    /// Fade right shape.
    FadeRight,
    /// Fade left shape.
    FadeLeft,
    /// Slant up shape.
    SlantUp,
    /// Slant down shape.
    SlantDown,
    /// Unknown shape.
    Unknown,
}

impl TextArtShape {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Plain,
            1 => Self::Wave,
            2 => Self::ArcUp,
            3 => Self::ArcDown,
            4 => Self::Circle,
            5 => Self::Button,
            6 => Self::Inflate,
            7 => Self::Deflate,
            8 => Self::FadeRight,
            9 => Self::FadeLeft,
            10 => Self::SlantUp,
            11 => Self::SlantDown,
            _ => Self::Unknown,
        }
    }
}

/// Text art alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextArtAlignment {
    /// Left alignment.
    #[default]
    Left,
    /// Center alignment.
    Center,
    /// Right alignment.
    Right,
    /// Justify alignment.
    Justify,
}

impl TextArtAlignment {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value & 0x03 {
            0 => Self::Left,
            1 => Self::Center,
            2 => Self::Right,
            3 => Self::Justify,
            _ => Self::Left,
        }
    }
}

/// A text art object.
#[derive(Debug, Clone, Default)]
pub struct TextArt {
    /// The text content.
    text: String,
    /// Font name.
    font_name: String,
    /// Font style (bold, italic flags).
    font_style: u8,
    /// Shape type.
    shape: TextArtShape,
    /// Text alignment.
    alignment: TextArtAlignment,
    /// Text color (RGB).
    text_color: u32,
    /// Outline color (RGB).
    outline_color: u32,
    /// Shadow color (RGB).
    shadow_color: u32,
}

impl TextArt {
    /// Creates a new text art.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a text art with text content.
    pub fn with_text(text: String) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }

    /// Returns the text content.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Sets the text content.
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    /// Returns the font name.
    pub fn font_name(&self) -> &str {
        &self.font_name
    }

    /// Sets the font name.
    pub fn set_font_name(&mut self, name: String) {
        self.font_name = name;
    }

    /// Returns the font style flags.
    pub const fn font_style(&self) -> u8 {
        self.font_style
    }

    /// Returns true if bold.
    pub const fn is_bold(&self) -> bool {
        self.font_style & 0x01 != 0
    }

    /// Returns true if italic.
    pub const fn is_italic(&self) -> bool {
        self.font_style & 0x02 != 0
    }

    /// Sets the font style.
    pub fn set_font_style(&mut self, style: u8) {
        self.font_style = style;
    }

    /// Returns the shape type.
    pub const fn shape(&self) -> TextArtShape {
        self.shape
    }

    /// Sets the shape type.
    pub fn set_shape(&mut self, shape: TextArtShape) {
        self.shape = shape;
    }

    /// Returns the text alignment.
    pub const fn alignment(&self) -> TextArtAlignment {
        self.alignment
    }

    /// Sets the text alignment.
    pub fn set_alignment(&mut self, alignment: TextArtAlignment) {
        self.alignment = alignment;
    }

    /// Returns the text color.
    pub const fn text_color(&self) -> u32 {
        self.text_color
    }

    /// Sets the text color.
    pub fn set_text_color(&mut self, color: u32) {
        self.text_color = color;
    }

    /// Returns the outline color.
    pub const fn outline_color(&self) -> u32 {
        self.outline_color
    }

    /// Sets the outline color.
    pub fn set_outline_color(&mut self, color: u32) {
        self.outline_color = color;
    }

    /// Returns the shadow color.
    pub const fn shadow_color(&self) -> u32 {
        self.shadow_color
    }

    /// Sets the shadow color.
    pub fn set_shadow_color(&mut self, color: u32) {
        self.shadow_color = color;
    }

    /// Parses text art from reader.
    ///
    /// Format (variable length per HWP spec):
    /// - WCHAR[]: text content (length-prefixed)
    /// - WCHAR[]: font name (length-prefixed)
    /// - UINT32: font style flags
    /// - UINT32: shape type
    /// - UINT32: alignment
    /// - UINT32: text color
    /// - UINT32: outline color
    /// - UINT32: shadow color
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        // Read text content
        let text = if !reader.is_empty() {
            let text_len = reader.read_u16()? as usize;
            if text_len > 0 && reader.remaining() >= text_len * 2 {
                let mut chars = Vec::with_capacity(text_len);
                for _ in 0..text_len {
                    chars.push(reader.read_u16()?);
                }
                String::from_utf16_lossy(&chars)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Read font name
        let font_name = if !reader.is_empty() {
            let font_len = reader.read_u16()? as usize;
            if font_len > 0 && reader.remaining() >= font_len * 2 {
                let mut chars = Vec::with_capacity(font_len);
                for _ in 0..font_len {
                    chars.push(reader.read_u16()?);
                }
                String::from_utf16_lossy(&chars)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Read style and shape properties
        let font_style = if reader.remaining() >= 4 {
            reader.read_u32()? as u8
        } else {
            0
        };

        let shape = if reader.remaining() >= 4 {
            TextArtShape::from_raw(reader.read_u32()? as u8)
        } else {
            TextArtShape::Plain
        };

        let alignment = if reader.remaining() >= 4 {
            TextArtAlignment::from_raw(reader.read_u32()? as u8)
        } else {
            TextArtAlignment::Left
        };

        // Read colors
        let text_color = if reader.remaining() >= 4 {
            reader.read_u32()?
        } else {
            0
        };

        let outline_color = if reader.remaining() >= 4 {
            reader.read_u32()?
        } else {
            0
        };

        let shadow_color = if reader.remaining() >= 4 {
            reader.read_u32()?
        } else {
            0
        };

        Ok(Self {
            text,
            font_name,
            font_style,
            shape,
            alignment,
            text_color,
            outline_color,
            shadow_color,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_art_shape_from_raw() {
        assert_eq!(TextArtShape::from_raw(0), TextArtShape::Plain);
        assert_eq!(TextArtShape::from_raw(1), TextArtShape::Wave);
        assert_eq!(TextArtShape::from_raw(4), TextArtShape::Circle);
        assert_eq!(TextArtShape::from_raw(255), TextArtShape::Unknown);
    }

    #[test]
    fn test_text_art_alignment_from_raw() {
        assert_eq!(TextArtAlignment::from_raw(0), TextArtAlignment::Left);
        assert_eq!(TextArtAlignment::from_raw(1), TextArtAlignment::Center);
        assert_eq!(TextArtAlignment::from_raw(2), TextArtAlignment::Right);
        assert_eq!(TextArtAlignment::from_raw(3), TextArtAlignment::Justify);
    }

    #[test]
    fn test_text_art_new() {
        let art = TextArt::new();
        assert_eq!(art.text(), "");
        assert_eq!(art.shape(), TextArtShape::Plain);
        assert_eq!(art.alignment(), TextArtAlignment::Left);
    }

    #[test]
    fn test_text_art_with_text() {
        let art = TextArt::with_text("Hello".to_string());
        assert_eq!(art.text(), "Hello");
    }

    #[test]
    fn test_text_art_font_style() {
        let mut art = TextArt::new();
        assert!(!art.is_bold());
        assert!(!art.is_italic());

        art.set_font_style(0x01);
        assert!(art.is_bold());
        assert!(!art.is_italic());

        art.set_font_style(0x02);
        assert!(!art.is_bold());
        assert!(art.is_italic());

        art.set_font_style(0x03);
        assert!(art.is_bold());
        assert!(art.is_italic());
    }

    #[test]
    fn test_text_art_colors() {
        let mut art = TextArt::new();
        art.set_text_color(0xFF0000);
        art.set_outline_color(0x00FF00);
        art.set_shadow_color(0x0000FF);

        assert_eq!(art.text_color(), 0xFF0000);
        assert_eq!(art.outline_color(), 0x00FF00);
        assert_eq!(art.shadow_color(), 0x0000FF);
    }
}
