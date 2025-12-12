//! Paragraph shape record.
//!
//! Defines paragraph formatting including margins, alignment, and spacing.

pub use primitive::HeadingType;

use crate::error::Result;
use crate::util::ByteReader;

/// Text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    /// Justify both sides.
    #[default]
    Justify,
    /// Left align.
    Left,
    /// Right align.
    Right,
    /// Center align.
    Center,
    /// Distribute evenly.
    Distribute,
    /// Divide evenly.
    Divide,
}

impl Alignment {
    /// Creates from raw value.
    pub const fn from_raw(value: u32) -> Self {
        match (value >> 2) & 0x07 {
            0 => Self::Justify,
            1 => Self::Left,
            2 => Self::Right,
            3 => Self::Center,
            4 => Self::Distribute,
            5 => Self::Divide,
            _ => Self::Justify,
        }
    }
}

/// Line break rule for English.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BreakLatinWord {
    /// Break at word boundary.
    #[default]
    Word,
    /// Break at hyphen.
    Hyphen,
    /// Break at character.
    Character,
}

impl BreakLatinWord {
    /// Creates from raw value.
    pub const fn from_raw(value: u32) -> Self {
        match (value >> 5) & 0x03 {
            0 => Self::Word,
            1 => Self::Hyphen,
            2 => Self::Character,
            _ => Self::Word,
        }
    }
}

/// Line break rule for non-Latin (Korean, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BreakNonLatinWord {
    /// Break at word boundary.
    #[default]
    Word,
    /// Break at character.
    Character,
}

impl BreakNonLatinWord {
    /// Creates from raw value.
    pub const fn from_raw(value: u32) -> Self {
        match (value >> 7) & 0x01 {
            0 => Self::Word,
            1 => Self::Character,
            _ => Self::Word,
        }
    }
}

/// Vertical alignment within line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalAlignment {
    /// Align to font baseline.
    #[default]
    Baseline,
    /// Top align.
    Top,
    /// Center align.
    Center,
    /// Bottom align.
    Bottom,
}

impl VerticalAlignment {
    /// Creates from raw value.
    pub const fn from_raw(value: u32) -> Self {
        match (value >> 20) & 0x03 {
            0 => Self::Baseline,
            1 => Self::Top,
            2 => Self::Center,
            3 => Self::Bottom,
            _ => Self::Baseline,
        }
    }
}

/// HeadingType raw 값 파싱 헬퍼
pub const fn heading_type_from_raw(value: u32) -> HeadingType {
    match (value >> 23) & 0x03 {
        0 => HeadingType::None,
        1 => HeadingType::Outline,
        2 => HeadingType::Number,
        3 => HeadingType::Bullet,
        _ => HeadingType::None,
    }
}

/// Line spacing type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineSpacingType {
    /// Percentage of character size.
    #[default]
    Percent,
    /// Fixed value.
    Fixed,
    /// Space between lines only.
    BetweenLines,
    /// Minimum.
    Minimum,
}

impl LineSpacingType {
    /// Creates from raw value (properties 1 for old versions).
    pub const fn from_properties1(value: u32) -> Self {
        match value & 0x03 {
            0 => Self::Percent,
            1 => Self::Fixed,
            2 => Self::BetweenLines,
            _ => Self::Percent,
        }
    }

    /// Creates from raw value (properties 3 for version 5.0.2.5+).
    pub const fn from_properties3(value: u32) -> Self {
        match value & 0x1F {
            0 => Self::Percent,
            1 => Self::Fixed,
            2 => Self::BetweenLines,
            3 => Self::Minimum,
            _ => Self::Percent,
        }
    }
}

/// Paragraph shape.
///
/// Defines paragraph formatting.
#[derive(Debug, Clone)]
pub struct ParagraphShape {
    /// Properties 1.
    properties1: u32,
    /// Left margin.
    left_margin: i32,
    /// Right margin.
    right_margin: i32,
    /// Indent (positive) or outdent (negative).
    indent: i32,
    /// Spacing before paragraph.
    space_before: i32,
    /// Spacing after paragraph.
    space_after: i32,
    /// Line spacing (for older versions).
    line_spacing_old: i32,
    /// Tab definition ID.
    tab_definition_id: u16,
    /// Numbering/bullet ID.
    numbering_bullet_id: u16,
    /// Border fill ID.
    border_fill_id: u16,
    /// Border left margin.
    border_left: i16,
    /// Border right margin.
    border_right: i16,
    /// Border top margin.
    border_top: i16,
    /// Border bottom margin.
    border_bottom: i16,
    /// Properties 2 (version 5.0.1.7+).
    properties2: Option<u32>,
    /// Properties 3 (version 5.0.2.5+).
    properties3: Option<u32>,
    /// Line spacing (version 5.0.2.5+).
    line_spacing: Option<u32>,
}

impl ParagraphShape {
    /// Parses ParagraphShape from a reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties1 = reader.read_u32()?;
        let left_margin = reader.read_i32()?;
        let right_margin = reader.read_i32()?;
        let indent = reader.read_i32()?;
        let space_before = reader.read_i32()?;
        let space_after = reader.read_i32()?;
        let line_spacing_old = reader.read_i32()?;
        let tab_definition_id = reader.read_u16()?;
        let numbering_bullet_id = reader.read_u16()?;
        let border_fill_id = reader.read_u16()?;
        let border_left = reader.read_i16()?;
        let border_right = reader.read_i16()?;
        let border_top = reader.read_i16()?;
        let border_bottom = reader.read_i16()?;

        let properties2 = if reader.remaining() >= 4 {
            Some(reader.read_u32()?)
        } else {
            None
        };

        let properties3 = if reader.remaining() >= 4 {
            Some(reader.read_u32()?)
        } else {
            None
        };

        let line_spacing = if reader.remaining() >= 4 {
            Some(reader.read_u32()?)
        } else {
            None
        };

        Ok(Self {
            properties1,
            left_margin,
            right_margin,
            indent,
            space_before,
            space_after,
            line_spacing_old,
            tab_definition_id,
            numbering_bullet_id,
            border_fill_id,
            border_left,
            border_right,
            border_top,
            border_bottom,
            properties2,
            properties3,
            line_spacing,
        })
    }

    /// Returns the alignment.
    #[inline]
    pub const fn alignment(&self) -> Alignment {
        Alignment::from_raw(self.properties1)
    }

    /// Returns the Latin word break rule.
    #[inline]
    pub const fn break_latin_word(&self) -> BreakLatinWord {
        BreakLatinWord::from_raw(self.properties1)
    }

    /// Returns the non-Latin word break rule.
    #[inline]
    pub const fn break_non_latin_word(&self) -> BreakNonLatinWord {
        BreakNonLatinWord::from_raw(self.properties1)
    }

    /// Returns whether widow/orphan protection is enabled.
    #[inline]
    pub const fn is_widow_orphan_protected(&self) -> bool {
        (self.properties1 & (1 << 16)) != 0
    }

    /// Returns whether to keep with next paragraph.
    #[inline]
    pub const fn keep_with_next(&self) -> bool {
        (self.properties1 & (1 << 17)) != 0
    }

    /// Returns whether paragraph is protected from splitting.
    #[inline]
    pub const fn is_protected(&self) -> bool {
        (self.properties1 & (1 << 18)) != 0
    }

    /// Returns whether to force page break before.
    #[inline]
    pub const fn page_break_before(&self) -> bool {
        (self.properties1 & (1 << 19)) != 0
    }

    /// Returns the vertical alignment.
    #[inline]
    pub const fn vertical_alignment(&self) -> VerticalAlignment {
        VerticalAlignment::from_raw(self.properties1)
    }

    /// Returns the heading type.
    #[inline]
    pub const fn heading_type(&self) -> HeadingType {
        heading_type_from_raw(self.properties1)
    }

    /// Returns the heading level (1-7).
    #[inline]
    pub const fn heading_level(&self) -> u8 {
        (((self.properties1 >> 25) & 0x07) + 1) as u8
    }

    /// Returns whether border is connected to adjacent paragraphs.
    #[inline]
    pub const fn is_border_connected(&self) -> bool {
        (self.properties1 & (1 << 28)) != 0
    }

    /// Returns whether paragraph margin is ignored.
    #[inline]
    pub const fn is_margin_ignored(&self) -> bool {
        (self.properties1 & (1 << 29)) != 0
    }

    /// Returns the left margin in HWP units.
    #[inline]
    pub const fn left_margin(&self) -> i32 {
        self.left_margin
    }

    /// Returns the right margin in HWP units.
    #[inline]
    pub const fn right_margin(&self) -> i32 {
        self.right_margin
    }

    /// Returns the indent in HWP units (positive = indent, negative = outdent).
    #[inline]
    pub const fn indent(&self) -> i32 {
        self.indent
    }

    /// Returns the spacing before in HWP units.
    #[inline]
    pub const fn space_before(&self) -> i32 {
        self.space_before
    }

    /// Returns the spacing after in HWP units.
    #[inline]
    pub const fn space_after(&self) -> i32 {
        self.space_after
    }

    /// Returns the line spacing type.
    pub fn line_spacing_type(&self) -> LineSpacingType {
        if let Some(props3) = self.properties3 {
            LineSpacingType::from_properties3(props3)
        } else {
            LineSpacingType::from_properties1(self.properties1)
        }
    }

    /// Returns the line spacing value.
    #[inline]
    pub fn line_spacing_value(&self) -> i32 {
        self.line_spacing
            .map(|v| v as i32)
            .unwrap_or(self.line_spacing_old)
    }

    /// Returns the tab definition ID.
    #[inline]
    pub const fn tab_definition_id(&self) -> u16 {
        self.tab_definition_id
    }

    /// Returns the numbering/bullet ID.
    #[inline]
    pub const fn numbering_bullet_id(&self) -> u16 {
        self.numbering_bullet_id
    }

    /// Returns the border fill ID.
    #[inline]
    pub const fn border_fill_id(&self) -> u16 {
        self.border_fill_id
    }

    /// Returns border margins as (left, right, top, bottom).
    #[inline]
    pub const fn border_margins(&self) -> (i16, i16, i16, i16) {
        (
            self.border_left,
            self.border_right,
            self.border_top,
            self.border_bottom,
        )
    }

    /// Returns properties 2 (version 5.0.1.7+).
    #[inline]
    pub const fn properties2(&self) -> Option<u32> {
        self.properties2
    }

    /// Returns whether snap to grid is enabled (bit 8 of properties1).
    #[inline]
    pub const fn snap_to_grid(&self) -> bool {
        (self.properties1 & (1 << 8)) != 0
    }

    /// Returns whether line numbers are suppressed (bit 0-1 of properties2).
    /// Only valid for version 5.0.1.7+.
    #[inline]
    pub fn suppress_line_numbers(&self) -> bool {
        self.properties2.is_some_and(|p| (p & 0x03) != 0)
    }

    /// Returns whether auto line height based on font is enabled (bit 22 of properties1).
    #[inline]
    pub const fn auto_line_height(&self) -> bool {
        (self.properties1 & (1 << 22)) != 0
    }

    /// Returns whether auto spacing between East Asian and English is enabled (bit 4 of properties2).
    /// Only valid for version 5.0.1.7+.
    #[inline]
    pub fn auto_spacing_east_asian_english(&self) -> bool {
        self.properties2.is_some_and(|p| (p & (1 << 4)) != 0)
    }

    /// Returns whether auto spacing between East Asian and numbers is enabled (bit 5 of properties2).
    /// Only valid for version 5.0.1.7+.
    #[inline]
    pub fn auto_spacing_east_asian_number(&self) -> bool {
        self.properties2.is_some_and(|p| (p & (1 << 5)) != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment() {
        assert_eq!(Alignment::from_raw(0b0000_0000), Alignment::Justify);
        assert_eq!(Alignment::from_raw(0b0000_0100), Alignment::Left);
        assert_eq!(Alignment::from_raw(0b0000_1000), Alignment::Right);
        assert_eq!(Alignment::from_raw(0b0000_1100), Alignment::Center);
    }

    #[test]
    fn test_heading_type() {
        assert_eq!(heading_type_from_raw(0), HeadingType::None);
        // Bits 24-23 = 01 for Outline (bit 23 set)
        assert_eq!(heading_type_from_raw(1 << 23), HeadingType::Outline);
        // Bits 24-23 = 10 for Number (bit 24 set)
        assert_eq!(heading_type_from_raw(2 << 23), HeadingType::Number);
        // Bits 24-23 = 11 for Bullet (both bits set)
        assert_eq!(heading_type_from_raw(3 << 23), HeadingType::Bullet);
    }
}
