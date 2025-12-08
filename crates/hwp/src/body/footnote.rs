//! Footnote and endnote parsing.
//!
//! Footnotes appear at the bottom of the page, while endnotes
//! appear at the end of the document or section.

use crate::error::Result;
use crate::primitive::HwpUnit;
use crate::util::ByteReader;

use super::paragraph::Paragraph;

/// Footnote/endnote numbering type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteNumberingType {
    /// Arabic numerals (1, 2, 3, ...).
    #[default]
    Arabic,
    /// Uppercase Roman numerals (I, II, III, ...).
    UpperRoman,
    /// Lowercase Roman numerals (i, ii, iii, ...).
    LowerRoman,
    /// Uppercase letters (A, B, C, ...).
    UpperAlpha,
    /// Lowercase letters (a, b, c, ...).
    LowerAlpha,
    /// Circled numbers.
    CircledNumbers,
    /// Custom symbols.
    Custom,
}

impl NoteNumberingType {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Arabic,
            1 => Self::UpperRoman,
            2 => Self::LowerRoman,
            3 => Self::UpperAlpha,
            4 => Self::LowerAlpha,
            5 => Self::CircledNumbers,
            _ => Self::Custom,
        }
    }
}

/// Footnote/endnote placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotePlacement {
    /// At the end of each page (footnote).
    #[default]
    EndOfPage,
    /// Beneath text on page.
    BeneathText,
    /// At the end of section.
    EndOfSection,
    /// At the end of document (endnote).
    EndOfDocument,
}

impl NotePlacement {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::EndOfPage,
            1 => Self::BeneathText,
            2 => Self::EndOfSection,
            3 => Self::EndOfDocument,
            _ => Self::EndOfPage,
        }
    }
}

/// A footnote in the document.
#[derive(Debug, Clone, Default)]
pub struct Footnote {
    /// Note number.
    number: u16,
    /// Paragraphs in this footnote.
    paragraphs: Vec<Paragraph>,
}

impl Footnote {
    /// Creates a new footnote.
    pub fn new(number: u16) -> Self {
        Self {
            number,
            paragraphs: Vec::new(),
        }
    }

    /// Returns the note number.
    pub const fn number(&self) -> u16 {
        self.number
    }

    /// Returns the paragraphs.
    pub fn paragraphs(&self) -> &[Paragraph] {
        &self.paragraphs
    }

    /// Adds a paragraph to this footnote.
    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph);
    }

    /// Sets the paragraphs.
    pub fn set_paragraphs(&mut self, paragraphs: Vec<Paragraph>) {
        self.paragraphs = paragraphs;
    }

    /// Extracts plain text from this footnote.
    pub fn plain_text(&self) -> String {
        self.paragraphs
            .iter()
            .map(|p| p.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// An endnote in the document.
#[derive(Debug, Clone, Default)]
pub struct Endnote {
    /// Note number.
    number: u16,
    /// Paragraphs in this endnote.
    paragraphs: Vec<Paragraph>,
}

impl Endnote {
    /// Creates a new endnote.
    pub fn new(number: u16) -> Self {
        Self {
            number,
            paragraphs: Vec::new(),
        }
    }

    /// Returns the note number.
    pub const fn number(&self) -> u16 {
        self.number
    }

    /// Returns the paragraphs.
    pub fn paragraphs(&self) -> &[Paragraph] {
        &self.paragraphs
    }

    /// Adds a paragraph to this endnote.
    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph);
    }

    /// Sets the paragraphs.
    pub fn set_paragraphs(&mut self, paragraphs: Vec<Paragraph>) {
        self.paragraphs = paragraphs;
    }

    /// Extracts plain text from this endnote.
    pub fn plain_text(&self) -> String {
        self.paragraphs
            .iter()
            .map(|p| p.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Footnote/endnote shape settings.
///
/// This record (HWPTAG_FOOTNOTE_SHAPE, 0x04A) defines the visual
/// properties and layout of footnotes and endnotes in a section.
#[derive(Debug, Clone, Default)]
pub struct FootnoteShape {
    /// Numbering type for footnotes.
    pub numbering_type: NoteNumberingType,
    /// Placement of footnotes.
    pub placement: NotePlacement,
    /// Starting number.
    pub start_number: u16,
    /// Separator line length (HwpUnit).
    pub separator_length: HwpUnit,
    /// Separator line position (from margin).
    pub separator_position: HwpUnit,
    /// Space above separator.
    pub space_above: HwpUnit,
    /// Space below separator.
    pub space_below: HwpUnit,
    /// Space between notes.
    pub space_between: HwpUnit,
    /// Whether to continue numbering across sections.
    pub continue_numbering: bool,
    /// Custom symbol (if using custom numbering).
    pub custom_symbol: Option<String>,
}

impl FootnoteShape {
    /// Creates a new footnote shape with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses footnote shape from reader.
    ///
    /// Format (per HWP spec - HWPTAG_FOOTNOTE_SHAPE):
    /// - UINT32: Properties (numbering type in bits 0-3, placement in bits 4-5)
    /// - UINT16: Starting number
    /// - HWPUNIT: Separator line length
    /// - HWPUNIT: Separator position
    /// - HWPUNIT: Space above separator
    /// - HWPUNIT: Space below separator
    /// - HWPUNIT: Space between notes
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u32()?;
        let numbering_type = NoteNumberingType::from_raw((properties & 0x0F) as u8);
        let placement = NotePlacement::from_raw(((properties >> 4) & 0x03) as u8);
        let continue_numbering = (properties & 0x100) != 0;

        let start_number = reader.read_u16()?;

        // Read layout measurements (may not be present in all versions)
        let separator_length = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        let separator_position = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        let space_above = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        let space_below = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        let space_between = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        Ok(Self {
            numbering_type,
            placement,
            start_number,
            separator_length,
            separator_position,
            space_above,
            space_below,
            space_between,
            continue_numbering,
            custom_symbol: None,
        })
    }
}

/// Endnote shape settings.
///
/// Similar to FootnoteShape but for endnotes.
#[derive(Debug, Clone, Default)]
pub struct EndnoteShape {
    /// Numbering type for endnotes.
    pub numbering_type: NoteNumberingType,
    /// Placement of endnotes.
    pub placement: NotePlacement,
    /// Starting number.
    pub start_number: u16,
    /// Space between notes.
    pub space_between: HwpUnit,
    /// Whether to continue numbering across sections.
    pub continue_numbering: bool,
}

impl EndnoteShape {
    /// Creates a new endnote shape with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses endnote shape from reader (same format as FootnoteShape).
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u32()?;
        let numbering_type = NoteNumberingType::from_raw((properties & 0x0F) as u8);
        let placement = NotePlacement::from_raw(((properties >> 4) & 0x03) as u8);
        let continue_numbering = (properties & 0x100) != 0;

        let start_number = reader.read_u16()?;

        // Skip separator fields (not used for endnotes)
        let _separator_length = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        let _separator_position = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        let _space_above = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        let _space_below = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        let space_between = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        Ok(Self {
            numbering_type,
            placement,
            start_number,
            space_between,
            continue_numbering,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_numbering_type() {
        assert_eq!(NoteNumberingType::from_raw(0), NoteNumberingType::Arabic);
        assert_eq!(NoteNumberingType::from_raw(1), NoteNumberingType::UpperRoman);
        assert_eq!(NoteNumberingType::from_raw(2), NoteNumberingType::LowerRoman);
        assert_eq!(NoteNumberingType::from_raw(3), NoteNumberingType::UpperAlpha);
        assert_eq!(NoteNumberingType::from_raw(4), NoteNumberingType::LowerAlpha);
    }

    #[test]
    fn test_note_placement() {
        assert_eq!(NotePlacement::from_raw(0), NotePlacement::EndOfPage);
        assert_eq!(NotePlacement::from_raw(1), NotePlacement::BeneathText);
        assert_eq!(NotePlacement::from_raw(2), NotePlacement::EndOfSection);
        assert_eq!(NotePlacement::from_raw(3), NotePlacement::EndOfDocument);
    }

    #[test]
    fn test_footnote_new() {
        let footnote = Footnote::new(1);
        assert_eq!(footnote.number(), 1);
        assert!(footnote.paragraphs().is_empty());
    }

    #[test]
    fn test_endnote_new() {
        let endnote = Endnote::new(2);
        assert_eq!(endnote.number(), 2);
        assert!(endnote.paragraphs().is_empty());
    }

    #[test]
    fn test_footnote_plain_text() {
        let footnote = Footnote::default();
        assert_eq!(footnote.plain_text(), "");
    }

    #[test]
    fn test_endnote_plain_text() {
        let endnote = Endnote::default();
        assert_eq!(endnote.plain_text(), "");
    }

    #[test]
    fn test_footnote_shape_new() {
        let shape = FootnoteShape::new();
        assert_eq!(shape.numbering_type, NoteNumberingType::Arabic);
        assert_eq!(shape.placement, NotePlacement::EndOfPage);
        assert_eq!(shape.start_number, 0);
    }

    #[test]
    fn test_endnote_shape_new() {
        let shape = EndnoteShape::new();
        assert_eq!(shape.numbering_type, NoteNumberingType::Arabic);
        assert_eq!(shape.placement, NotePlacement::EndOfPage);
        assert_eq!(shape.start_number, 0);
    }

    #[test]
    fn test_footnote_shape_from_reader() {
        let mut data = Vec::new();
        // Properties: Arabic (0) numbering, EndOfPage (0) placement
        data.extend_from_slice(&0x00u32.to_le_bytes());
        // Start number: 1
        data.extend_from_slice(&1u16.to_le_bytes());
        // Separator length
        data.extend_from_slice(&1000i32.to_le_bytes());
        // Separator position
        data.extend_from_slice(&0i32.to_le_bytes());
        // Space above
        data.extend_from_slice(&500i32.to_le_bytes());
        // Space below
        data.extend_from_slice(&500i32.to_le_bytes());
        // Space between
        data.extend_from_slice(&300i32.to_le_bytes());

        let mut reader = ByteReader::new(&data);
        let shape = FootnoteShape::from_reader(&mut reader).unwrap();

        assert_eq!(shape.numbering_type, NoteNumberingType::Arabic);
        assert_eq!(shape.placement, NotePlacement::EndOfPage);
        assert_eq!(shape.start_number, 1);
        assert_eq!(shape.separator_length.value(), 1000);
    }
}
