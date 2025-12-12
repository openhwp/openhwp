//! Memo (annotation) parsing.
//!
//! Memos are annotations attached to text ranges in the document.
//! They consist of a shape definition and a list containing the memo content.

use super::paragraph::Paragraph;
use crate::error::Result;
use crate::util::ByteReader;

/// Memo shape properties.
#[derive(Debug, Clone, Default)]
pub struct MemoShape {
    /// Width of the memo box.
    width: u32,
    /// Line type for memo border.
    line_type: u8,
}

impl MemoShape {
    /// Creates a memo shape with properties.
    pub const fn with_properties(width: u32, line_type: u8) -> Self {
        Self { width, line_type }
    }

    /// Returns the width of the memo box.
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Returns the line type.
    pub const fn line_type(&self) -> u8 {
        self.line_type
    }

    /// Parses memo shape from reader.
    ///
    /// Format (22 bytes per HWP spec):
    /// - UINT32: border color
    /// - UINT32: fill color
    /// - UINT32: croshatch color
    /// - UINT32: width (HWP units)
    /// - UINT32: memo index
    /// - BYTE: line type
    /// - BYTE: line style
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let _border_color = reader.read_u32()?;
        let _fill_color = reader.read_u32()?;
        let _croshatch_color = reader.read_u32()?;
        let width = reader.read_u32()?;
        let _memo_index = reader.read_u32()?;
        let line_type = reader.read_u8()?;
        let _line_style = reader.read_u8()?;

        Ok(Self { width, line_type })
    }
}

/// A memo (annotation) in the document.
#[derive(Debug, Clone, Default)]
pub struct Memo {
    /// Memo shape properties.
    shape: MemoShape,
    /// Paragraphs in this memo.
    paragraphs: Vec<Paragraph>,
}

impl Memo {
    /// Creates a memo with shape.
    pub const fn with_shape(shape: MemoShape) -> Self {
        Self {
            shape,
            paragraphs: Vec::new(),
        }
    }

    /// Returns the memo shape.
    pub const fn shape(&self) -> &MemoShape {
        &self.shape
    }

    /// Returns the paragraphs in this memo.
    pub fn paragraphs(&self) -> &[Paragraph] {
        &self.paragraphs
    }

    /// Adds a paragraph to this memo.
    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph);
    }

    /// Sets the paragraphs.
    pub fn set_paragraphs(&mut self, paragraphs: Vec<Paragraph>) {
        self.paragraphs = paragraphs;
    }

    /// Extracts plain text from this memo.
    pub fn plain_text(&self) -> String {
        self.paragraphs
            .iter()
            .map(|p| p.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memo_shape_default() {
        let shape = MemoShape::default();
        assert_eq!(shape.width(), 0);
        assert_eq!(shape.line_type(), 0);
    }

    #[test]
    fn test_memo_shape_with_properties() {
        let shape = MemoShape::with_properties(5000, 1);
        assert_eq!(shape.width(), 5000);
        assert_eq!(shape.line_type(), 1);
    }

    #[test]
    fn test_memo_default() {
        let memo = Memo::default();
        assert!(memo.paragraphs().is_empty());
    }

    #[test]
    fn test_memo_plain_text() {
        let memo = Memo::default();
        assert_eq!(memo.plain_text(), "");
    }
}
