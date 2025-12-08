//! Style definition record.

use crate::error::Result;
use crate::util::ByteReader;

/// Style type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StyleType {
    #[default]
    Paragraph,
    Character,
}

impl StyleType {
    pub const fn from_raw(value: u8) -> Self {
        match value & 0x07 {
            0 => Self::Paragraph,
            1 => Self::Character,
            _ => Self::Paragraph,
        }
    }
}

/// Style definition.
#[derive(Debug, Clone)]
pub struct Style {
    local_name: String,
    english_name: String,
    style_type: StyleType,
    next_style_id: u8,
    language_id: i16,
    paragraph_shape_id: u16,
    character_shape_id: u16,
}

impl Style {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let local_name = reader.read_utf16_string()?;
        let english_name = reader.read_utf16_string()?;
        let properties = reader.read_u8()?;
        let style_type = StyleType::from_raw(properties);
        let next_style_id = reader.read_u8()?;
        let language_id = reader.read_i16()?;
        let paragraph_shape_id = reader.read_u16()?;
        let character_shape_id = reader.read_u16()?;

        Ok(Self {
            local_name,
            english_name,
            style_type,
            next_style_id,
            language_id,
            paragraph_shape_id,
            character_shape_id,
        })
    }

    /// Returns the local (Korean) style name.
    pub fn local_name(&self) -> &str {
        &self.local_name
    }

    /// Returns the English style name.
    pub fn english_name(&self) -> &str {
        &self.english_name
    }

    /// Returns the style type.
    pub const fn style_type(&self) -> StyleType {
        self.style_type
    }

    /// Returns the next style ID.
    pub const fn next_style_id(&self) -> u8 {
        self.next_style_id
    }

    /// Returns the paragraph shape ID.
    pub const fn paragraph_shape_id(&self) -> u16 {
        self.paragraph_shape_id
    }

    /// Returns the character shape ID.
    pub const fn character_shape_id(&self) -> u16 {
        self.character_shape_id
    }

    /// Returns the language ID.
    pub const fn language_id(&self) -> i16 {
        self.language_id
    }
}
