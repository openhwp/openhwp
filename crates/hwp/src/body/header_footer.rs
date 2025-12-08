//! Header and footer parsing.
//!
//! Headers and footers are controls that contain paragraphs displayed
//! at the top/bottom of each page.
//!
//! ## Instantiation Pattern
//!
//! These types use **dynamic instantiation** rather than `from_reader()` because:
//! - Header/Footer are control types identified by ControlHeader ID (`head`/`foot`)
//! - The target (both/even/odd pages) is parsed from ControlHeader data bytes
//! - Paragraphs are collected from subsequent ListHeader contexts
//!
//! Use [`Header::new()`] / [`Footer::new()`] with the parsed target for instantiation.

use super::paragraph::Paragraph;

/// Header/Footer application target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HeaderFooterTarget {
    /// Both pages.
    #[default]
    BothPages,
    /// Even pages only.
    EvenPages,
    /// Odd pages only.
    OddPages,
}

impl HeaderFooterTarget {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value & 0x03 {
            0 => Self::BothPages,
            1 => Self::EvenPages,
            2 => Self::OddPages,
            _ => Self::BothPages,
        }
    }
}

/// A header element in the document.
#[derive(Debug, Clone, Default)]
pub struct Header {
    /// Target pages (both, even, odd).
    target: HeaderFooterTarget,
    /// Paragraphs in this header.
    paragraphs: Vec<Paragraph>,
}

impl Header {
    /// Creates a new header.
    pub fn new(target: HeaderFooterTarget) -> Self {
        Self {
            target,
            paragraphs: Vec::new(),
        }
    }

    /// Returns the target pages.
    pub const fn target(&self) -> HeaderFooterTarget {
        self.target
    }

    /// Returns the paragraphs.
    pub fn paragraphs(&self) -> &[Paragraph] {
        &self.paragraphs
    }

    /// Adds a paragraph to this header.
    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph);
    }

    /// Sets the paragraphs.
    pub fn set_paragraphs(&mut self, paragraphs: Vec<Paragraph>) {
        self.paragraphs = paragraphs;
    }

    /// Extracts plain text from this header.
    pub fn plain_text(&self) -> String {
        self.paragraphs
            .iter()
            .map(|p| p.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// A footer element in the document.
#[derive(Debug, Clone, Default)]
pub struct Footer {
    /// Target pages (both, even, odd).
    target: HeaderFooterTarget,
    /// Paragraphs in this footer.
    paragraphs: Vec<Paragraph>,
}

impl Footer {
    /// Creates a new footer.
    pub fn new(target: HeaderFooterTarget) -> Self {
        Self {
            target,
            paragraphs: Vec::new(),
        }
    }

    /// Returns the target pages.
    pub const fn target(&self) -> HeaderFooterTarget {
        self.target
    }

    /// Returns the paragraphs.
    pub fn paragraphs(&self) -> &[Paragraph] {
        &self.paragraphs
    }

    /// Adds a paragraph to this footer.
    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph);
    }

    /// Sets the paragraphs.
    pub fn set_paragraphs(&mut self, paragraphs: Vec<Paragraph>) {
        self.paragraphs = paragraphs;
    }

    /// Extracts plain text from this footer.
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
    fn test_header_footer_target() {
        assert_eq!(HeaderFooterTarget::from_raw(0), HeaderFooterTarget::BothPages);
        assert_eq!(HeaderFooterTarget::from_raw(1), HeaderFooterTarget::EvenPages);
        assert_eq!(HeaderFooterTarget::from_raw(2), HeaderFooterTarget::OddPages);
        assert_eq!(HeaderFooterTarget::from_raw(3), HeaderFooterTarget::BothPages);
    }

    #[test]
    fn test_header_new() {
        let header = Header::new(HeaderFooterTarget::EvenPages);
        assert_eq!(header.target(), HeaderFooterTarget::EvenPages);
        assert!(header.paragraphs().is_empty());
    }

    #[test]
    fn test_footer_new() {
        let footer = Footer::new(HeaderFooterTarget::OddPages);
        assert_eq!(footer.target(), HeaderFooterTarget::OddPages);
        assert!(footer.paragraphs().is_empty());
    }

    #[test]
    fn test_header_plain_text() {
        let header = Header::default();
        assert_eq!(header.plain_text(), "");
    }

    #[test]
    fn test_footer_plain_text() {
        let footer = Footer::default();
        assert_eq!(footer.plain_text(), "");
    }
}
