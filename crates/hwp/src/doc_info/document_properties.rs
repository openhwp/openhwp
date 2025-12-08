//! Document properties record.
//!
//! Contains document-level settings like section count and numbering.

use crate::error::Result;
use crate::util::ByteReader;

/// Document properties.
///
/// Contains section count, starting numbers, and caret position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentProperties {
    /// Number of sections in the document.
    section_count: u16,
    /// Starting page number.
    page_start_number: u16,
    /// Starting footnote number.
    footnote_start_number: u16,
    /// Starting endnote number.
    endnote_start_number: u16,
    /// Starting figure number.
    figure_start_number: u16,
    /// Starting table number.
    table_start_number: u16,
    /// Starting equation number.
    equation_start_number: u16,
    /// Caret position - list ID.
    caret_list_id: u32,
    /// Caret position - paragraph ID.
    caret_paragraph_id: u32,
    /// Caret position - character position within paragraph.
    caret_position_in_paragraph: u32,
}

impl DocumentProperties {
    /// Parses DocumentProperties from a reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            section_count: reader.read_u16()?,
            page_start_number: reader.read_u16()?,
            footnote_start_number: reader.read_u16()?,
            endnote_start_number: reader.read_u16()?,
            figure_start_number: reader.read_u16()?,
            table_start_number: reader.read_u16()?,
            equation_start_number: reader.read_u16()?,
            caret_list_id: reader.read_u32()?,
            caret_paragraph_id: reader.read_u32()?,
            caret_position_in_paragraph: reader.read_u32()?,
        })
    }

    /// Returns the number of sections in the document.
    #[inline]
    pub const fn section_count(&self) -> u16 {
        self.section_count
    }

    /// Returns the starting page number.
    #[inline]
    pub const fn page_start_number(&self) -> u16 {
        self.page_start_number
    }

    /// Returns the starting footnote number.
    #[inline]
    pub const fn footnote_start_number(&self) -> u16 {
        self.footnote_start_number
    }

    /// Returns the starting endnote number.
    #[inline]
    pub const fn endnote_start_number(&self) -> u16 {
        self.endnote_start_number
    }

    /// Returns the starting figure number.
    #[inline]
    pub const fn figure_start_number(&self) -> u16 {
        self.figure_start_number
    }

    /// Returns the starting table number.
    #[inline]
    pub const fn table_start_number(&self) -> u16 {
        self.table_start_number
    }

    /// Returns the starting equation number.
    #[inline]
    pub const fn equation_start_number(&self) -> u16 {
        self.equation_start_number
    }

    /// Returns the caret list ID.
    #[inline]
    pub const fn caret_list_id(&self) -> u32 {
        self.caret_list_id
    }

    /// Returns the caret paragraph ID.
    #[inline]
    pub const fn caret_paragraph_id(&self) -> u32 {
        self.caret_paragraph_id
    }

    /// Returns the caret position within the paragraph.
    #[inline]
    pub const fn caret_position_in_paragraph(&self) -> u32 {
        self.caret_position_in_paragraph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_document_properties() {
        let data = [
            0x01, 0x00, // section_count = 1
            0x01, 0x00, // page_start_number = 1
            0x01, 0x00, // footnote_start_number = 1
            0x01, 0x00, // endnote_start_number = 1
            0x01, 0x00, // figure_start_number = 1
            0x01, 0x00, // table_start_number = 1
            0x01, 0x00, // equation_start_number = 1
            0x00, 0x00, 0x00, 0x00, // caret_list_id = 0
            0x00, 0x00, 0x00, 0x00, // caret_paragraph_id = 0
            0x00, 0x00, 0x00, 0x00, // caret_position_in_paragraph = 0
        ];

        let mut reader = ByteReader::new(&data);
        let props = DocumentProperties::from_reader(&mut reader).unwrap();

        assert_eq!(props.section_count(), 1);
        assert_eq!(props.page_start_number(), 1);
        assert_eq!(props.footnote_start_number(), 1);
    }
}
