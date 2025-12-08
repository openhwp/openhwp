//! List header parsing.
//!
//! ListHeader is used for controls that contain nested paragraphs,
//! such as table cells, captions, headers/footers, etc.

use crate::error::Result;
use crate::primitive::HwpUnit;
use crate::util::ByteReader;

/// List header properties.
///
/// ListHeader appears before nested paragraph content in controls
/// like table cells, captions, text boxes, etc.
#[derive(Debug, Clone, Default)]
pub struct ListHeader {
    /// Number of paragraphs in this list.
    paragraph_count: u16,
    /// Properties flags.
    properties: u32,
    /// Text width.
    text_width: HwpUnit,
    /// Text height.
    text_height: HwpUnit,
}

impl ListHeader {
    /// Minimum size of ListHeader in bytes.
    pub const MIN_SIZE: usize = 12;

    /// Parses ListHeader from a reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let paragraph_count = reader.read_u16()?;
        let properties = reader.read_u32()?;

        // Text dimensions (only present in some versions)
        let text_width = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::new(0)
        };

        let text_height = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::new(0)
        };

        Ok(Self {
            paragraph_count,
            properties,
            text_width,
            text_height,
        })
    }

    /// Returns the number of paragraphs in this list.
    pub const fn paragraph_count(&self) -> u16 {
        self.paragraph_count
    }

    /// Returns the properties flags.
    pub const fn properties(&self) -> u32 {
        self.properties
    }

    /// Returns the text direction.
    pub const fn text_direction(&self) -> TextDirection {
        TextDirection::from_raw((self.properties & 0x07) as u8)
    }

    /// Returns true if using auto height.
    pub const fn is_auto_height(&self) -> bool {
        (self.properties & (1 << 3)) != 0
    }

    /// Returns true if protecting content from editing.
    pub const fn is_protected(&self) -> bool {
        (self.properties & (1 << 4)) != 0
    }

    /// Returns the text width.
    pub const fn text_width(&self) -> HwpUnit {
        self.text_width
    }

    /// Returns the text height.
    pub const fn text_height(&self) -> HwpUnit {
        self.text_height
    }
}

/// Text direction for list content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextDirection {
    /// Horizontal (left to right).
    #[default]
    Horizontal,
    /// Vertical (top to bottom, right to left columns).
    Vertical,
    /// Vertical (top to bottom, left to right columns).
    VerticalLtr,
}

impl TextDirection {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value & 0x07 {
            0 => Self::Horizontal,
            1 => Self::Vertical,
            2 => Self::VerticalLtr,
            _ => Self::Horizontal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_direction() {
        assert_eq!(TextDirection::from_raw(0), TextDirection::Horizontal);
        assert_eq!(TextDirection::from_raw(1), TextDirection::Vertical);
        assert_eq!(TextDirection::from_raw(2), TextDirection::VerticalLtr);
    }

    #[test]
    fn test_list_header_parsing() {
        // Create test data: paragraph_count=3, properties=0, width=1000, height=500
        let mut data = Vec::new();
        data.extend_from_slice(&3u16.to_le_bytes()); // paragraph_count
        data.extend_from_slice(&0u32.to_le_bytes()); // properties
        data.extend_from_slice(&1000i32.to_le_bytes()); // text_width
        data.extend_from_slice(&500i32.to_le_bytes()); // text_height

        let mut reader = ByteReader::new(&data);
        let header = ListHeader::from_reader(&mut reader).unwrap();

        assert_eq!(header.paragraph_count(), 3);
        assert_eq!(header.properties(), 0);
        assert_eq!(header.text_width().value(), 1000);
        assert_eq!(header.text_height().value(), 500);
    }
}
