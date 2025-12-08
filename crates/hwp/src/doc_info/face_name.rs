//! Font face name record.
//!
//! Defines fonts used in the document.

use crate::error::Result;
use crate::util::ByteReader;

/// Alternate font type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlternateFontType {
    /// Unknown font type.
    Unknown,
    /// TrueType font.
    TrueType,
    /// Hangul-specific font (HFT).
    HangulFont,
}

impl AlternateFontType {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            1 => Self::TrueType,
            2 => Self::HangulFont,
            _ => Self::Unknown,
        }
    }
}

/// Font type information (PANOSE-like).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FontTypeInfo {
    /// Font family kind.
    pub family_kind: u8,
    /// Serif style.
    pub serif_style: u8,
    /// Weight.
    pub weight: u8,
    /// Proportion.
    pub proportion: u8,
    /// Contrast.
    pub contrast: u8,
    /// Stroke variation.
    pub stroke_variation: u8,
    /// Arm style.
    pub arm_style: u8,
    /// Letterform.
    pub letterform: u8,
    /// Midline.
    pub midline: u8,
    /// X-height.
    pub x_height: u8,
}

impl FontTypeInfo {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            family_kind: reader.read_u8()?,
            serif_style: reader.read_u8()?,
            weight: reader.read_u8()?,
            proportion: reader.read_u8()?,
            contrast: reader.read_u8()?,
            stroke_variation: reader.read_u8()?,
            arm_style: reader.read_u8()?,
            letterform: reader.read_u8()?,
            midline: reader.read_u8()?,
            x_height: reader.read_u8()?,
        })
    }
}

/// Font face name.
///
/// Defines a font used in the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaceName {
    /// Properties flags.
    properties: u8,
    /// Font name.
    name: String,
    /// Alternate font type.
    alternate_font_type: Option<AlternateFontType>,
    /// Alternate font name.
    alternate_font_name: Option<String>,
    /// Font type info (PANOSE-like).
    font_type_info: Option<FontTypeInfo>,
    /// Default font name.
    default_font_name: Option<String>,
}

impl FaceName {
    /// Flag for alternate font presence.
    const HAS_ALTERNATE: u8 = 0x80;
    /// Flag for font type info presence.
    const HAS_TYPE_INFO: u8 = 0x40;
    /// Flag for default font presence.
    const HAS_DEFAULT: u8 = 0x20;

    /// Parses FaceName from a reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u8()?;
        let name = reader.read_utf16_string()?;

        let mut alternate_font_type = None;
        let mut alternate_font_name = None;
        if properties & Self::HAS_ALTERNATE != 0 {
            alternate_font_type = Some(AlternateFontType::from_raw(reader.read_u8()?));
            alternate_font_name = Some(reader.read_utf16_string()?);
        }

        let font_type_info = if properties & Self::HAS_TYPE_INFO != 0 {
            Some(FontTypeInfo::from_reader(reader)?)
        } else {
            None
        };

        let default_font_name = if properties & Self::HAS_DEFAULT != 0 {
            Some(reader.read_utf16_string()?)
        } else {
            None
        };

        Ok(Self {
            properties,
            name,
            alternate_font_type,
            alternate_font_name,
            font_type_info,
            default_font_name,
        })
    }

    /// Returns the font name.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether an alternate font is specified.
    #[inline]
    pub const fn has_alternate(&self) -> bool {
        self.properties & Self::HAS_ALTERNATE != 0
    }

    /// Returns the alternate font type.
    #[inline]
    pub const fn alternate_font_type(&self) -> Option<AlternateFontType> {
        self.alternate_font_type
    }

    /// Returns the alternate font name.
    #[inline]
    pub fn alternate_font_name(&self) -> Option<&str> {
        self.alternate_font_name.as_deref()
    }

    /// Returns whether font type info is present.
    #[inline]
    pub const fn has_type_info(&self) -> bool {
        self.properties & Self::HAS_TYPE_INFO != 0
    }

    /// Returns the font type info.
    #[inline]
    pub const fn font_type_info(&self) -> Option<&FontTypeInfo> {
        self.font_type_info.as_ref()
    }

    /// Returns whether a default font is specified.
    #[inline]
    pub const fn has_default(&self) -> bool {
        self.properties & Self::HAS_DEFAULT != 0
    }

    /// Returns the default font name.
    #[inline]
    pub fn default_font_name(&self) -> Option<&str> {
        self.default_font_name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_face_name() {
        let data = [
            0x00, // properties (no alternates, no type info, no default)
            0x05, 0x00, // name length = 5
            b'A', 0x00, b'r', 0x00, b'i', 0x00, b'a', 0x00, b'l', 0x00, // "Arial"
        ];

        let mut reader = ByteReader::new(&data);
        let face = FaceName::from_reader(&mut reader).unwrap();

        assert_eq!(face.name(), "Arial");
        assert!(!face.has_alternate());
        assert!(!face.has_type_info());
        assert!(!face.has_default());
    }

    #[test]
    fn test_alternate_font_type() {
        assert_eq!(AlternateFontType::from_raw(0), AlternateFontType::Unknown);
        assert_eq!(AlternateFontType::from_raw(1), AlternateFontType::TrueType);
        assert_eq!(AlternateFontType::from_raw(2), AlternateFontType::HangulFont);
    }
}
