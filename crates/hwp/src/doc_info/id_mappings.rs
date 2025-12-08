//! ID mappings header record.
//!
//! Contains counts for various ID-mapped elements in the document.

use crate::error::Result;
use crate::util::ByteReader;

/// ID mappings header.
///
/// Contains the count of each type of ID-mapped element.
/// The order of elements after IdMappings follows this count order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IdMappings {
    /// Number of binary data items.
    binary_data_count: i32,
    /// Number of Korean fonts.
    korean_font_count: i32,
    /// Number of English fonts.
    english_font_count: i32,
    /// Number of Chinese fonts.
    chinese_font_count: i32,
    /// Number of Japanese fonts.
    japanese_font_count: i32,
    /// Number of other fonts.
    other_font_count: i32,
    /// Number of symbol fonts.
    symbol_font_count: i32,
    /// Number of user-defined fonts.
    user_font_count: i32,
    /// Number of border/fill definitions.
    border_fill_count: i32,
    /// Number of character shapes.
    character_shape_count: i32,
    /// Number of tab definitions.
    tab_definition_count: i32,
    /// Number of paragraph numberings.
    numbering_count: i32,
    /// Number of bullets.
    bullet_count: i32,
    /// Number of paragraph shapes.
    paragraph_shape_count: i32,
    /// Number of styles.
    style_count: i32,
    /// Number of memo shapes (version 5.0.2.1+).
    memo_shape_count: i32,
    /// Number of track changes (version 5.0.3.2+).
    track_change_count: i32,
    /// Number of track change authors (version 5.0.3.2+).
    track_change_author_count: i32,
}

impl IdMappings {
    /// Parses IdMappings from a reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        // Basic counts (always present)
        let binary_data_count = reader.read_i32()?;
        let korean_font_count = reader.read_i32()?;
        let english_font_count = reader.read_i32()?;
        let chinese_font_count = reader.read_i32()?;
        let japanese_font_count = reader.read_i32()?;
        let other_font_count = reader.read_i32()?;
        let symbol_font_count = reader.read_i32()?;
        let user_font_count = reader.read_i32()?;
        let border_fill_count = reader.read_i32()?;
        let character_shape_count = reader.read_i32()?;
        let tab_definition_count = reader.read_i32()?;
        let numbering_count = reader.read_i32()?;
        let bullet_count = reader.read_i32()?;
        let paragraph_shape_count = reader.read_i32()?;
        let style_count = reader.read_i32()?;

        // Optional counts (version-dependent)
        let memo_shape_count = if reader.remaining() >= 4 {
            reader.read_i32()?
        } else {
            0
        };

        let track_change_count = if reader.remaining() >= 4 {
            reader.read_i32()?
        } else {
            0
        };

        let track_change_author_count = if reader.remaining() >= 4 {
            reader.read_i32()?
        } else {
            0
        };

        Ok(Self {
            binary_data_count,
            korean_font_count,
            english_font_count,
            chinese_font_count,
            japanese_font_count,
            other_font_count,
            symbol_font_count,
            user_font_count,
            border_fill_count,
            character_shape_count,
            tab_definition_count,
            numbering_count,
            bullet_count,
            paragraph_shape_count,
            style_count,
            memo_shape_count,
            track_change_count,
            track_change_author_count,
        })
    }

    /// Returns the number of binary data items.
    #[inline]
    pub const fn binary_data_count(&self) -> i32 {
        self.binary_data_count
    }

    /// Returns the number of Korean fonts.
    #[inline]
    pub const fn korean_font_count(&self) -> i32 {
        self.korean_font_count
    }

    /// Returns the number of English fonts.
    #[inline]
    pub const fn english_font_count(&self) -> i32 {
        self.english_font_count
    }

    /// Returns the number of Chinese fonts.
    #[inline]
    pub const fn chinese_font_count(&self) -> i32 {
        self.chinese_font_count
    }

    /// Returns the number of Japanese fonts.
    #[inline]
    pub const fn japanese_font_count(&self) -> i32 {
        self.japanese_font_count
    }

    /// Returns the number of other fonts.
    #[inline]
    pub const fn other_font_count(&self) -> i32 {
        self.other_font_count
    }

    /// Returns the number of symbol fonts.
    #[inline]
    pub const fn symbol_font_count(&self) -> i32 {
        self.symbol_font_count
    }

    /// Returns the number of user-defined fonts.
    #[inline]
    pub const fn user_font_count(&self) -> i32 {
        self.user_font_count
    }

    /// Returns the total number of all fonts.
    #[inline]
    pub fn total_font_count(&self) -> i32 {
        self.korean_font_count
            + self.english_font_count
            + self.chinese_font_count
            + self.japanese_font_count
            + self.other_font_count
            + self.symbol_font_count
            + self.user_font_count
    }

    /// Returns the number of border/fill definitions.
    #[inline]
    pub const fn border_fill_count(&self) -> i32 {
        self.border_fill_count
    }

    /// Returns the number of character shapes.
    #[inline]
    pub const fn character_shape_count(&self) -> i32 {
        self.character_shape_count
    }

    /// Returns the number of tab definitions.
    #[inline]
    pub const fn tab_definition_count(&self) -> i32 {
        self.tab_definition_count
    }

    /// Returns the number of paragraph numberings.
    #[inline]
    pub const fn numbering_count(&self) -> i32 {
        self.numbering_count
    }

    /// Returns the number of bullets.
    #[inline]
    pub const fn bullet_count(&self) -> i32 {
        self.bullet_count
    }

    /// Returns the number of paragraph shapes.
    #[inline]
    pub const fn paragraph_shape_count(&self) -> i32 {
        self.paragraph_shape_count
    }

    /// Returns the number of styles.
    #[inline]
    pub const fn style_count(&self) -> i32 {
        self.style_count
    }

    /// Returns the number of memo shapes.
    #[inline]
    pub const fn memo_shape_count(&self) -> i32 {
        self.memo_shape_count
    }

    /// Returns the number of track changes.
    #[inline]
    pub const fn track_change_count(&self) -> i32 {
        self.track_change_count
    }

    /// Returns the number of track change authors.
    #[inline]
    pub const fn track_change_author_count(&self) -> i32 {
        self.track_change_author_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_id_mappings_basic() {
        // 15 i32 values (60 bytes)
        let mut data = vec![0u8; 60];
        // Set some counts
        data[0..4].copy_from_slice(&5i32.to_le_bytes()); // binary_data_count = 5
        data[4..8].copy_from_slice(&3i32.to_le_bytes()); // korean_font_count = 3

        let mut reader = ByteReader::new(&data);
        let mappings = IdMappings::from_reader(&mut reader).unwrap();

        assert_eq!(mappings.binary_data_count(), 5);
        assert_eq!(mappings.korean_font_count(), 3);
    }

    #[test]
    fn test_total_font_count() {
        let mappings = IdMappings {
            korean_font_count: 3,
            english_font_count: 2,
            chinese_font_count: 1,
            japanese_font_count: 1,
            other_font_count: 1,
            symbol_font_count: 1,
            user_font_count: 0,
            ..Default::default()
        };

        assert_eq!(mappings.total_font_count(), 9);
    }
}
