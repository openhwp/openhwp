//! Footnote and endnote parsing.
//!
//! Footnotes appear at the bottom of the page, while endnotes
//! appear at the end of the document or section.

use crate::error::Result;
use crate::primitive::ColorReference;
use crate::util::ByteReader;
use primitive::HwpUnit;

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
    pub const fn new(number: u16) -> Self {
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
    pub const fn new(number: u16) -> Self {
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
    /// Prefix character (앞 장식 문자).
    pub prefix: Option<String>,
    /// Suffix character (뒤 장식 문자).
    pub suffix: Option<String>,
    /// Superscript display (위 첨자로 표시).
    pub superscript: bool,
    /// Separator line type.
    pub separator_line_type: u8,
    /// Separator line thickness (0.1mm 단위).
    pub separator_line_thickness: u8,
    /// Separator line color.
    pub separator_line_color: ColorReference,
}

impl FootnoteShape {
    /// Parses footnote shape from reader.
    ///
    /// Format (per HWP spec 5.0 - HWPTAG_FOOTNOTE_SHAPE, 표 133):
    /// - UINT32: Properties (numbering type in bits 0-7, placement in bits 8-9, superscript in bit 10)
    /// - WCHAR: Custom symbol (사용자 기호)
    /// - WCHAR: Prefix character (앞 장식 문자)
    /// - WCHAR: Suffix character (뒤 장식 문자)
    /// - UINT16: Starting number
    /// - HWPUNIT16: Separator line length
    /// - HWPUNIT16: Separator position
    /// - HWPUNIT16: Space above separator
    /// - HWPUNIT16: Space below separator
    /// - HWPUNIT16: Space between notes
    /// - UINT8: Separator line type
    /// - UINT8: Separator line thickness
    /// - COLORREF: Separator line color
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u32()?;
        let numbering_type = NoteNumberingType::from_raw((properties & 0xFF) as u8);
        let placement = NotePlacement::from_raw(((properties >> 8) & 0x03) as u8);
        let superscript = (properties & 0x400) != 0; // bit 10
        let continue_numbering = (properties & 0x1000) != 0; // bit 12

        // Read custom symbol (WCHAR, 2 bytes)
        let custom_symbol = if reader.remaining() >= 2 {
            let ch = reader.read_u16()?;
            if ch != 0 {
                Some(
                    char::from_u32(ch as u32)
                        .map(|c| c.to_string())
                        .unwrap_or_default(),
                )
            } else {
                None
            }
        } else {
            None
        };

        // Read prefix character (WCHAR, 2 bytes)
        let prefix = if reader.remaining() >= 2 {
            let ch = reader.read_u16()?;
            if ch != 0 {
                Some(
                    char::from_u32(ch as u32)
                        .map(|c| c.to_string())
                        .unwrap_or_default(),
                )
            } else {
                None
            }
        } else {
            None
        };

        // Read suffix character (WCHAR, 2 bytes)
        let suffix = if reader.remaining() >= 2 {
            let ch = reader.read_u16()?;
            if ch != 0 {
                Some(
                    char::from_u32(ch as u32)
                        .map(|c| c.to_string())
                        .unwrap_or_default(),
                )
            } else {
                None
            }
        } else {
            None
        };

        // Read starting number
        let start_number = if reader.remaining() >= 2 {
            reader.read_u16()?
        } else {
            1
        };

        // Read layout measurements (HWPUNIT16 = 2 bytes each)
        let separator_length = if reader.remaining() >= 2 {
            HwpUnit::new(reader.read_u16()? as i32)
        } else {
            HwpUnit::default()
        };

        let separator_position = if reader.remaining() >= 2 {
            HwpUnit::new(reader.read_u16()? as i32)
        } else {
            HwpUnit::default()
        };

        let space_above = if reader.remaining() >= 2 {
            HwpUnit::new(reader.read_u16()? as i32)
        } else {
            HwpUnit::default()
        };

        let space_below = if reader.remaining() >= 2 {
            HwpUnit::new(reader.read_u16()? as i32)
        } else {
            HwpUnit::default()
        };

        let space_between = if reader.remaining() >= 2 {
            HwpUnit::new(reader.read_u16()? as i32)
        } else {
            HwpUnit::default()
        };

        // Read separator line type (1 byte)
        let separator_line_type = if reader.remaining() >= 1 {
            reader.read_u8()?
        } else {
            0
        };

        // Read separator line thickness (1 byte)
        let separator_line_thickness = if reader.remaining() >= 1 {
            reader.read_u8()?
        } else {
            1
        };

        // Read separator line color (COLORREF, 4 bytes)
        let separator_line_color = if reader.remaining() >= 4 {
            reader.read_color()?
        } else {
            ColorReference::default()
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
            custom_symbol,
            prefix,
            suffix,
            superscript,
            separator_line_type,
            separator_line_thickness,
            separator_line_color,
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
    /// Custom symbol (if using custom numbering).
    pub custom_symbol: Option<String>,
    /// Prefix character (앞 장식 문자).
    pub prefix: Option<String>,
    /// Suffix character (뒤 장식 문자).
    pub suffix: Option<String>,
    /// Superscript display (위 첨자로 표시).
    pub superscript: bool,
    /// Separator line type (stored but typically not used for endnotes).
    pub separator_line_type: u8,
    /// Separator line thickness (stored but typically not used for endnotes).
    pub separator_line_thickness: u8,
    /// Separator line color (stored but typically not used for endnotes).
    pub separator_line_color: ColorReference,
}

impl EndnoteShape {
    /// Parses endnote shape from reader (same format as FootnoteShape).
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u32()?;
        let numbering_type = NoteNumberingType::from_raw((properties & 0xFF) as u8);
        let placement = NotePlacement::from_raw(((properties >> 8) & 0x03) as u8);
        let superscript = (properties & 0x400) != 0; // bit 10
        let continue_numbering = (properties & 0x1000) != 0; // bit 12

        // Read custom symbol (WCHAR, 2 bytes)
        let custom_symbol = if reader.remaining() >= 2 {
            let ch = reader.read_u16()?;
            if ch != 0 {
                Some(
                    char::from_u32(ch as u32)
                        .map(|c| c.to_string())
                        .unwrap_or_default(),
                )
            } else {
                None
            }
        } else {
            None
        };

        // Read prefix character (WCHAR, 2 bytes)
        let prefix = if reader.remaining() >= 2 {
            let ch = reader.read_u16()?;
            if ch != 0 {
                Some(
                    char::from_u32(ch as u32)
                        .map(|c| c.to_string())
                        .unwrap_or_default(),
                )
            } else {
                None
            }
        } else {
            None
        };

        // Read suffix character (WCHAR, 2 bytes)
        let suffix = if reader.remaining() >= 2 {
            let ch = reader.read_u16()?;
            if ch != 0 {
                Some(
                    char::from_u32(ch as u32)
                        .map(|c| c.to_string())
                        .unwrap_or_default(),
                )
            } else {
                None
            }
        } else {
            None
        };

        // Read starting number
        let start_number = if reader.remaining() >= 2 {
            reader.read_u16()?
        } else {
            1
        };

        // Skip separator fields (not used for endnotes) - HWPUNIT16 each
        if reader.remaining() >= 2 {
            let _ = reader.read_u16()?; // separator_length
        }
        if reader.remaining() >= 2 {
            let _ = reader.read_u16()?; // separator_position
        }
        if reader.remaining() >= 2 {
            let _ = reader.read_u16()?; // space_above
        }
        if reader.remaining() >= 2 {
            let _ = reader.read_u16()?; // space_below
        }

        let space_between = if reader.remaining() >= 2 {
            HwpUnit::new(reader.read_u16()? as i32)
        } else {
            HwpUnit::default()
        };

        // Read separator line type (1 byte) - stored but typically not used for endnotes
        let separator_line_type = if reader.remaining() >= 1 {
            reader.read_u8()?
        } else {
            0
        };

        // Read separator line thickness (1 byte)
        let separator_line_thickness = if reader.remaining() >= 1 {
            reader.read_u8()?
        } else {
            1
        };

        // Read separator line color (COLORREF, 4 bytes)
        let separator_line_color = if reader.remaining() >= 4 {
            reader.read_color()?
        } else {
            ColorReference::default()
        };

        Ok(Self {
            numbering_type,
            placement,
            start_number,
            space_between,
            continue_numbering,
            custom_symbol,
            prefix,
            suffix,
            superscript,
            separator_line_type,
            separator_line_thickness,
            separator_line_color,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_numbering_type() {
        assert_eq!(NoteNumberingType::from_raw(0), NoteNumberingType::Arabic);
        assert_eq!(
            NoteNumberingType::from_raw(1),
            NoteNumberingType::UpperRoman
        );
        assert_eq!(
            NoteNumberingType::from_raw(2),
            NoteNumberingType::LowerRoman
        );
        assert_eq!(
            NoteNumberingType::from_raw(3),
            NoteNumberingType::UpperAlpha
        );
        assert_eq!(
            NoteNumberingType::from_raw(4),
            NoteNumberingType::LowerAlpha
        );
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
        let shape = FootnoteShape::default();
        assert_eq!(shape.numbering_type, NoteNumberingType::Arabic);
        assert_eq!(shape.placement, NotePlacement::EndOfPage);
        assert_eq!(shape.start_number, 0);
    }

    #[test]
    fn test_endnote_shape_new() {
        let shape = EndnoteShape::default();
        assert_eq!(shape.numbering_type, NoteNumberingType::Arabic);
        assert_eq!(shape.placement, NotePlacement::EndOfPage);
        assert_eq!(shape.start_number, 0);
    }

    #[test]
    fn test_footnote_shape_from_reader() {
        let mut data = Vec::new();
        // Properties: Arabic (0) numbering, EndOfPage (0) placement, superscript (bit 10)
        data.extend_from_slice(&0x400u32.to_le_bytes()); // superscript = true
        // Custom symbol (WCHAR): none
        data.extend_from_slice(&0u16.to_le_bytes());
        // Prefix character (WCHAR): '(' = 0x28
        data.extend_from_slice(&0x28u16.to_le_bytes());
        // Suffix character (WCHAR): ')' = 0x29
        data.extend_from_slice(&0x29u16.to_le_bytes());
        // Start number: 1
        data.extend_from_slice(&1u16.to_le_bytes());
        // Separator length (HWPUNIT16)
        data.extend_from_slice(&1000u16.to_le_bytes());
        // Separator position (HWPUNIT16)
        data.extend_from_slice(&0u16.to_le_bytes());
        // Space above (HWPUNIT16)
        data.extend_from_slice(&500u16.to_le_bytes());
        // Space below (HWPUNIT16)
        data.extend_from_slice(&500u16.to_le_bytes());
        // Space between (HWPUNIT16)
        data.extend_from_slice(&300u16.to_le_bytes());
        // Separator line type, thickness, color (1 + 1 + 4 bytes)
        data.extend_from_slice(&[0u8; 6]);

        let mut reader = ByteReader::new(&data);
        let shape = FootnoteShape::from_reader(&mut reader).unwrap();

        assert_eq!(shape.numbering_type, NoteNumberingType::Arabic);
        assert_eq!(shape.placement, NotePlacement::EndOfPage);
        assert!(shape.superscript);
        assert_eq!(shape.prefix, Some("(".to_string()));
        assert_eq!(shape.suffix, Some(")".to_string()));
        assert_eq!(shape.start_number, 1);
        assert_eq!(shape.separator_length.value(), 1000);
    }
}
