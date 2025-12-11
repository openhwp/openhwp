//! Page definition and layout settings.
//!
//! Page definitions specify the page size, margins, and layout
//! for each section of the document.

use crate::error::Result;
use crate::util::ByteReader;
use primitive::HwpUnit;

/// Page orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PageOrientation {
    /// Portrait (vertical).
    #[default]
    Portrait,
    /// Landscape (horizontal).
    Landscape,
}

impl PageOrientation {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        if value == 0 {
            Self::Portrait
        } else {
            Self::Landscape
        }
    }
}

/// Page gutter position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GutterPosition {
    /// Gutter on the left.
    #[default]
    Left,
    /// Gutter on the right.
    Right,
    /// Gutter on top.
    Top,
    /// Gutter on bottom.
    Bottom,
}

impl GutterPosition {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value & 0x03 {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::Top,
            3 => Self::Bottom,
            _ => Self::Left,
        }
    }
}

/// Page margins.
#[derive(Debug, Clone, Copy, Default)]
pub struct PageMargins {
    /// Left margin.
    pub left: HwpUnit,
    /// Right margin.
    pub right: HwpUnit,
    /// Top margin.
    pub top: HwpUnit,
    /// Bottom margin.
    pub bottom: HwpUnit,
    /// Header margin (from top).
    pub header: HwpUnit,
    /// Footer margin (from bottom).
    pub footer: HwpUnit,
    /// Gutter (binding) margin.
    pub gutter: HwpUnit,
}

impl PageMargins {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            left: reader.read_hwp_unit()?,
            right: reader.read_hwp_unit()?,
            top: reader.read_hwp_unit()?,
            bottom: reader.read_hwp_unit()?,
            header: reader.read_hwp_unit()?,
            footer: reader.read_hwp_unit()?,
            gutter: reader.read_hwp_unit()?,
        })
    }
}

/// Page definition (section page settings).
#[derive(Debug, Clone, Default)]
pub struct PageDefinition {
    /// Paper width.
    pub width: HwpUnit,
    /// Paper height.
    pub height: HwpUnit,
    /// Page margins.
    pub margins: PageMargins,
    /// Page orientation.
    pub orientation: PageOrientation,
    /// Gutter position.
    pub gutter_position: GutterPosition,
}

impl PageDefinition {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let width = reader.read_hwp_unit()?;
        let height = reader.read_hwp_unit()?;
        let margins = PageMargins::from_reader(reader)?;
        let orientation = PageOrientation::from_raw(reader.read_u8()?);
        let gutter_position = GutterPosition::from_raw(reader.read_u8()?);

        Ok(Self {
            width,
            height,
            margins,
            orientation,
            gutter_position,
        })
    }

    /// Returns the effective width (considering orientation).
    pub fn effective_width(&self) -> HwpUnit {
        match self.orientation {
            PageOrientation::Portrait => self.width,
            PageOrientation::Landscape => self.height,
        }
    }

    /// Returns the effective height (considering orientation).
    pub fn effective_height(&self) -> HwpUnit {
        match self.orientation {
            PageOrientation::Portrait => self.height,
            PageOrientation::Landscape => self.width,
        }
    }

    /// Returns the printable width (page width minus left and right margins).
    pub fn printable_width(&self) -> i32 {
        let total_margin = self.margins.left.value() + self.margins.right.value();
        let effective = self.effective_width().value();
        effective.saturating_sub(total_margin)
    }

    /// Returns the printable height (page height minus top and bottom margins).
    pub fn printable_height(&self) -> i32 {
        let total_margin = self.margins.top.value() + self.margins.bottom.value();
        let effective = self.effective_height().value();
        effective.saturating_sub(total_margin)
    }
}

/// Page border fill position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PageBorderFillPosition {
    /// Border relative to paper edge.
    #[default]
    Paper,
    /// Border relative to body text area.
    Body,
}

impl PageBorderFillPosition {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        if value == 0 {
            Self::Paper
        } else {
            Self::Body
        }
    }
}

/// Page border fill settings.
///
/// This record (HWPTAG_PAGE_BORDER_FILL, 0x04B) defines the border
/// and fill properties for pages in a section.
#[derive(Debug, Clone, Default)]
pub struct PageBorderFill {
    /// BorderFill ID reference (index into DocInfo border_fills).
    pub border_fill_id: u16,
    /// Position relative to paper or body.
    pub position: PageBorderFillPosition,
    /// Whether to include header in border area.
    pub include_header: bool,
    /// Whether to include footer in border area.
    pub include_footer: bool,
    /// Whether to fill behind text.
    pub fill_behind: bool,
    /// Offset from paper edge (left).
    pub offset_left: HwpUnit,
    /// Offset from paper edge (right).
    pub offset_right: HwpUnit,
    /// Offset from paper edge (top).
    pub offset_top: HwpUnit,
    /// Offset from paper edge (bottom).
    pub offset_bottom: HwpUnit,
}

impl PageBorderFill {
    /// Creates a new page border fill with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses page border fill from reader.
    ///
    /// Format (per HWP spec - HWPTAG_PAGE_BORDER_FILL):
    /// - UINT32: Properties
    /// - UINT16: BorderFill ID
    /// - HWPUNIT: Left offset
    /// - HWPUNIT: Right offset
    /// - HWPUNIT: Top offset
    /// - HWPUNIT: Bottom offset
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u32()?;
        let position = PageBorderFillPosition::from_raw((properties & 0x01) as u8);
        let include_header = (properties & 0x02) != 0;
        let include_footer = (properties & 0x04) != 0;
        let fill_behind = (properties & 0x08) != 0;

        let border_fill_id = reader.read_u16()?;

        let offset_left = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        let offset_right = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        let offset_top = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        let offset_bottom = if reader.remaining() >= 4 {
            reader.read_hwp_unit()?
        } else {
            HwpUnit::default()
        };

        Ok(Self {
            border_fill_id,
            position,
            include_header,
            include_footer,
            fill_behind,
            offset_left,
            offset_right,
            offset_top,
            offset_bottom,
        })
    }
}

// ============================================================================
// Conversions from/to primitive types
// ============================================================================

impl From<GutterPosition> for primitive::GutterPosition {
    fn from(pos: GutterPosition) -> Self {
        match pos {
            GutterPosition::Left => Self::Left,
            GutterPosition::Right => Self::Right,
            GutterPosition::Top => Self::Top,
            GutterPosition::Bottom => Self::Bottom,
        }
    }
}

impl From<primitive::GutterPosition> for GutterPosition {
    fn from(pos: primitive::GutterPosition) -> Self {
        match pos {
            primitive::GutterPosition::Left => Self::Left,
            primitive::GutterPosition::Right => Self::Right,
            primitive::GutterPosition::Top => Self::Top,
            primitive::GutterPosition::Bottom => Self::Bottom,
        }
    }
}

impl From<PageMargins> for primitive::PageMargins {
    fn from(margins: PageMargins) -> Self {
        Self::new(
            margins.left,
            margins.right,
            margins.top,
            margins.bottom,
            margins.header,
            margins.footer,
            margins.gutter,
        )
    }
}

impl From<primitive::PageMargins> for PageMargins {
    fn from(margins: primitive::PageMargins) -> Self {
        Self {
            left: margins.left,
            right: margins.right,
            top: margins.top,
            bottom: margins.bottom,
            header: margins.header,
            footer: margins.footer,
            gutter: margins.gutter,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_orientation() {
        assert_eq!(PageOrientation::from_raw(0), PageOrientation::Portrait);
        assert_eq!(PageOrientation::from_raw(1), PageOrientation::Landscape);
    }

    #[test]
    fn test_gutter_position() {
        assert_eq!(GutterPosition::from_raw(0), GutterPosition::Left);
        assert_eq!(GutterPosition::from_raw(1), GutterPosition::Right);
        assert_eq!(GutterPosition::from_raw(2), GutterPosition::Top);
        assert_eq!(GutterPosition::from_raw(3), GutterPosition::Bottom);
    }

    #[test]
    fn test_page_definition_parsing() {
        let mut data = Vec::new();
        // Width: 59528 (A4 width in HWP units, approximately 210mm)
        data.extend_from_slice(&59528i32.to_le_bytes());
        // Height: 84188 (A4 height in HWP units, approximately 297mm)
        data.extend_from_slice(&84188i32.to_le_bytes());
        // Margins: left, right, top, bottom, header, footer, gutter
        data.extend_from_slice(&8504i32.to_le_bytes()); // left: 30mm
        data.extend_from_slice(&8504i32.to_le_bytes()); // right: 30mm
        data.extend_from_slice(&5669i32.to_le_bytes()); // top: 20mm
        data.extend_from_slice(&5669i32.to_le_bytes()); // bottom: 20mm
        data.extend_from_slice(&4252i32.to_le_bytes()); // header: 15mm
        data.extend_from_slice(&4252i32.to_le_bytes()); // footer: 15mm
        data.extend_from_slice(&0i32.to_le_bytes()); // gutter: 0
        // Orientation: Portrait
        data.push(0);
        // Gutter position: Left
        data.push(0);

        let mut reader = ByteReader::new(&data);
        let page_def = PageDefinition::from_reader(&mut reader).unwrap();

        assert_eq!(page_def.width.value(), 59528);
        assert_eq!(page_def.height.value(), 84188);
        assert_eq!(page_def.orientation, PageOrientation::Portrait);
        assert_eq!(page_def.gutter_position, GutterPosition::Left);
    }

    #[test]
    fn test_effective_dimensions() {
        let page_def = PageDefinition {
            width: HwpUnit::new(100),
            height: HwpUnit::new(200),
            orientation: PageOrientation::Portrait,
            ..Default::default()
        };

        assert_eq!(page_def.effective_width().value(), 100);
        assert_eq!(page_def.effective_height().value(), 200);

        let landscape_page = PageDefinition {
            width: HwpUnit::new(100),
            height: HwpUnit::new(200),
            orientation: PageOrientation::Landscape,
            ..Default::default()
        };

        assert_eq!(landscape_page.effective_width().value(), 200);
        assert_eq!(landscape_page.effective_height().value(), 100);
    }

    #[test]
    fn test_page_border_fill_position() {
        assert_eq!(PageBorderFillPosition::from_raw(0), PageBorderFillPosition::Paper);
        assert_eq!(PageBorderFillPosition::from_raw(1), PageBorderFillPosition::Body);
    }

    #[test]
    fn test_page_border_fill_new() {
        let border_fill = PageBorderFill::new();
        assert_eq!(border_fill.border_fill_id, 0);
        assert_eq!(border_fill.position, PageBorderFillPosition::Paper);
        assert!(!border_fill.include_header);
        assert!(!border_fill.include_footer);
    }

    #[test]
    fn test_page_border_fill_from_reader() {
        let mut data = Vec::new();
        // Properties: position=Body (1), include_header (2), include_footer (4)
        data.extend_from_slice(&0x07u32.to_le_bytes());
        // BorderFill ID: 1
        data.extend_from_slice(&1u16.to_le_bytes());
        // Offsets: left, right, top, bottom
        data.extend_from_slice(&100i32.to_le_bytes());
        data.extend_from_slice(&100i32.to_le_bytes());
        data.extend_from_slice(&50i32.to_le_bytes());
        data.extend_from_slice(&50i32.to_le_bytes());

        let mut reader = ByteReader::new(&data);
        let border_fill = PageBorderFill::from_reader(&mut reader).unwrap();

        assert_eq!(border_fill.border_fill_id, 1);
        assert_eq!(border_fill.position, PageBorderFillPosition::Body);
        assert!(border_fill.include_header);
        assert!(border_fill.include_footer);
        assert_eq!(border_fill.offset_left.value(), 100);
    }
}
