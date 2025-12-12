//! Text box and caption parsing.
//!
//! Text boxes are floating containers that hold paragraphs.
//! Captions are text attached to tables, images, or other objects.
//!
//! ## TextBox/Caption Data Format
//!
//! TextBox and Caption are parsed from ListHeader records with specific attributes.
//! The vertical alignment and caption properties come from the ListHeader.

use crate::error::Result;
use crate::util::ByteReader;

use super::paragraph::Paragraph;

/// Text box vertical alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalAlignment {
    /// Top alignment.
    #[default]
    Top,
    /// Center alignment.
    Center,
    /// Bottom alignment.
    Bottom,
}

impl VerticalAlignment {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value & 0x03 {
            0 => Self::Top,
            1 => Self::Center,
            2 => Self::Bottom,
            _ => Self::Top,
        }
    }
}

/// A text box in the document.
#[derive(Debug, Clone, Default)]
pub struct TextBox {
    /// Vertical alignment.
    vertical_align: VerticalAlignment,
    /// Paragraphs in this text box.
    paragraphs: Vec<Paragraph>,
}

impl TextBox {
    /// Creates a new text box.
    pub const fn new() -> Self {
        Self {
            vertical_align: VerticalAlignment::Top,
            paragraphs: Vec::new(),
        }
    }

    /// Creates a text box with vertical alignment.
    pub const fn with_alignment(vertical_align: VerticalAlignment) -> Self {
        Self {
            vertical_align,
            paragraphs: Vec::new(),
        }
    }

    /// Returns the vertical alignment.
    pub const fn vertical_align(&self) -> VerticalAlignment {
        self.vertical_align
    }

    /// Returns the paragraphs.
    pub fn paragraphs(&self) -> &[Paragraph] {
        &self.paragraphs
    }

    /// Adds a paragraph to this text box.
    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph);
    }

    /// Sets the paragraphs.
    pub fn set_paragraphs(&mut self, paragraphs: Vec<Paragraph>) {
        self.paragraphs = paragraphs;
    }

    /// Extracts plain text from this text box.
    pub fn plain_text(&self) -> String {
        self.paragraphs
            .iter()
            .map(|p| p.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Parses text box properties from ListHeader data.
    ///
    /// The ListHeader for a text box contains:
    /// - WORD: paragraph count
    /// - DWORD: properties (text direction, vertical align in bits)
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        // Skip paragraph count (handled separately)
        let _paragraph_count = reader.read_u16()?;

        // Read properties
        let props = reader.read_u32()?;

        // Vertical alignment is in bits 2-3
        let vertical_align = VerticalAlignment::from_raw(((props >> 2) & 0x03) as u8);

        Ok(Self {
            vertical_align,
            paragraphs: Vec::new(),
        })
    }

    /// Sets the vertical alignment.
    pub fn set_vertical_align(&mut self, align: VerticalAlignment) {
        self.vertical_align = align;
    }
}

/// Caption direction relative to the object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CaptionDirection {
    /// Caption below the object.
    #[default]
    Below,
    /// Caption above the object.
    Above,
    /// Caption to the left of the object.
    Left,
    /// Caption to the right of the object.
    Right,
}

impl CaptionDirection {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value & 0x03 {
            0 => Self::Below,
            1 => Self::Above,
            2 => Self::Left,
            3 => Self::Right,
            _ => Self::Below,
        }
    }
}

/// A caption attached to an object.
#[derive(Debug, Clone, Default)]
pub struct Caption {
    /// Direction relative to the object.
    direction: CaptionDirection,
    /// Gap between caption and object.
    gap: i32,
    /// Paragraphs in this caption.
    paragraphs: Vec<Paragraph>,
}

impl Caption {
    /// Creates a new caption.
    pub const fn new() -> Self {
        Self {
            direction: CaptionDirection::Below,
            gap: 0,
            paragraphs: Vec::new(),
        }
    }

    /// Creates a caption with direction and gap.
    pub const fn with_direction(direction: CaptionDirection, gap: i32) -> Self {
        Self {
            direction,
            gap,
            paragraphs: Vec::new(),
        }
    }

    /// Returns the caption direction.
    pub const fn direction(&self) -> CaptionDirection {
        self.direction
    }

    /// Returns the gap between caption and object.
    pub const fn gap(&self) -> i32 {
        self.gap
    }

    /// Returns the paragraphs.
    pub fn paragraphs(&self) -> &[Paragraph] {
        &self.paragraphs
    }

    /// Adds a paragraph to this caption.
    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph);
    }

    /// Sets the paragraphs.
    pub fn set_paragraphs(&mut self, paragraphs: Vec<Paragraph>) {
        self.paragraphs = paragraphs;
    }

    /// Extracts plain text from this caption.
    pub fn plain_text(&self) -> String {
        self.paragraphs
            .iter()
            .map(|p| p.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Parses caption properties from ListHeader data.
    ///
    /// The ListHeader for a caption contains:
    /// - WORD: paragraph count
    /// - DWORD: properties (direction, etc.)
    /// - HWPUNIT: gap (distance from object)
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        // Skip paragraph count (handled separately)
        let _paragraph_count = reader.read_u16()?;

        // Read properties
        let props = reader.read_u32()?;

        // Direction is in bits 0-1
        let direction = CaptionDirection::from_raw((props & 0x03) as u8);

        // Read gap if available
        let gap = if reader.remaining() >= 4 {
            reader.read_i32()?
        } else {
            0
        };

        Ok(Self {
            direction,
            gap,
            paragraphs: Vec::new(),
        })
    }

    /// Sets the direction.
    pub fn set_direction(&mut self, direction: CaptionDirection) {
        self.direction = direction;
    }

    /// Sets the gap.
    pub fn set_gap(&mut self, gap: i32) {
        self.gap = gap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertical_alignment() {
        assert_eq!(VerticalAlignment::from_raw(0), VerticalAlignment::Top);
        assert_eq!(VerticalAlignment::from_raw(1), VerticalAlignment::Center);
        assert_eq!(VerticalAlignment::from_raw(2), VerticalAlignment::Bottom);
    }

    #[test]
    fn test_caption_direction() {
        assert_eq!(CaptionDirection::from_raw(0), CaptionDirection::Below);
        assert_eq!(CaptionDirection::from_raw(1), CaptionDirection::Above);
        assert_eq!(CaptionDirection::from_raw(2), CaptionDirection::Left);
        assert_eq!(CaptionDirection::from_raw(3), CaptionDirection::Right);
    }

    #[test]
    fn test_text_box_new() {
        let text_box = TextBox::new();
        assert_eq!(text_box.vertical_align(), VerticalAlignment::Top);
        assert!(text_box.paragraphs().is_empty());
    }

    #[test]
    fn test_caption_new() {
        let caption = Caption::new();
        assert_eq!(caption.direction(), CaptionDirection::Below);
        assert_eq!(caption.gap(), 0);
        assert!(caption.paragraphs().is_empty());
    }

    #[test]
    fn test_text_box_plain_text() {
        let text_box = TextBox::new();
        assert_eq!(text_box.plain_text(), "");
    }

    #[test]
    fn test_caption_plain_text() {
        let caption = Caption::new();
        assert_eq!(caption.plain_text(), "");
    }
}
