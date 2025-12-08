//! Layout compatibility settings record.

use crate::error::Result;
use crate::util::ByteReader;

/// Layout compatibility settings.
///
/// Contains flags for compatibility behavior at different
/// document structure levels (character, paragraph, section, object, field).
#[derive(Debug, Clone, Copy)]
pub struct LayoutCompatibility {
    /// Character-level format compatibility flags.
    character_format: u32,
    /// Paragraph-level format compatibility flags.
    paragraph_format: u32,
    /// Section-level format compatibility flags.
    section_format: u32,
    /// Object-level format compatibility flags.
    object_format: u32,
    /// Field-level format compatibility flags.
    field_format: u32,
}

impl LayoutCompatibility {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            character_format: reader.read_u32()?,
            paragraph_format: reader.read_u32()?,
            section_format: reader.read_u32()?,
            object_format: reader.read_u32()?,
            field_format: reader.read_u32()?,
        })
    }

    /// Returns character-level format compatibility flags.
    pub const fn character_format(&self) -> u32 {
        self.character_format
    }

    /// Returns paragraph-level format compatibility flags.
    pub const fn paragraph_format(&self) -> u32 {
        self.paragraph_format
    }

    /// Returns section-level format compatibility flags.
    pub const fn section_format(&self) -> u32 {
        self.section_format
    }

    /// Returns object-level format compatibility flags.
    pub const fn object_format(&self) -> u32 {
        self.object_format
    }

    /// Returns field-level format compatibility flags.
    pub const fn field_format(&self) -> u32 {
        self.field_format
    }
}
