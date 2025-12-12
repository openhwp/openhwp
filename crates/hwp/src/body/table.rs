//! Table control parsing.
//!
//! Tables in HWP consist of rows and cells, where each cell
//! contains a list of paragraphs.

use crate::error::Result;
use crate::primitive::{CellPadding, HwpUnit16, TablePadding};
use crate::util::ByteReader;
use primitive::HwpUnit;

use super::paragraph::Paragraph;

/// Table border split behavior at page boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PageBorderSplit {
    /// Don't split the table.
    #[default]
    NoSplit,
    /// Split by cell.
    SplitByCell,
    /// Don't split (alternative value).
    NoSplitAlt,
}

impl PageBorderSplit {
    /// Creates from raw bits.
    pub const fn from_raw(value: u32) -> Self {
        match value & 0x03 {
            0 => Self::NoSplit,
            1 => Self::SplitByCell,
            _ => Self::NoSplitAlt,
        }
    }
}

/// Table zone information (for merged cells).
#[derive(Debug, Clone, Copy, Default)]
pub struct TableZone {
    /// Start column.
    pub start_column: u16,
    /// Start row.
    pub start_row: u16,
    /// End column.
    pub end_column: u16,
    /// End row.
    pub end_row: u16,
    /// Border fill ID reference.
    pub border_fill_id: u16,
}

impl TableZone {
    /// Size of zone info in bytes.
    pub const SIZE: usize = 10;

    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            start_column: reader.read_u16()?,
            start_row: reader.read_u16()?,
            end_column: reader.read_u16()?,
            end_row: reader.read_u16()?,
            border_fill_id: reader.read_u16()?,
        })
    }
}

/// Table cell information.
#[derive(Debug, Clone, Default)]
pub struct TableCell {
    /// Column address (0-based from left).
    pub column: u16,
    /// Row address (0-based from top).
    pub row: u16,
    /// Number of columns this cell spans.
    pub column_span: u16,
    /// Number of rows this cell spans.
    pub row_span: u16,
    /// Cell width.
    pub width: HwpUnit,
    /// Cell height.
    pub height: HwpUnit,
    /// Cell padding.
    pub padding: CellPadding,
    /// Border fill ID reference.
    pub border_fill_id: u16,
    /// Paragraphs contained in this cell.
    pub paragraphs: Vec<Paragraph>,
}

impl TableCell {
    /// Size of cell properties in bytes.
    pub const SIZE: usize = 26;

    /// Parses cell properties from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            column: reader.read_u16()?,
            row: reader.read_u16()?,
            column_span: reader.read_u16()?,
            row_span: reader.read_u16()?,
            width: reader.read_hwp_unit()?,
            height: reader.read_hwp_unit()?,
            padding: CellPadding::from_reader(reader)?,
            border_fill_id: reader.read_u16()?,
            paragraphs: Vec::new(),
        })
    }

    /// Adds a paragraph to this cell.
    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph);
    }

    /// Returns true if this cell spans multiple columns.
    pub const fn is_column_merged(&self) -> bool {
        self.column_span > 1
    }

    /// Returns true if this cell spans multiple rows.
    pub const fn is_row_merged(&self) -> bool {
        self.row_span > 1
    }
}

/// Table properties.
#[derive(Debug, Clone, Default)]
pub struct TableProperties {
    /// Raw properties value.
    properties: u32,
    /// Number of rows.
    pub row_count: u16,
    /// Number of columns.
    pub column_count: u16,
    /// Cell spacing.
    pub cell_spacing: HwpUnit16,
    /// Inner padding.
    pub padding: TablePadding,
    /// Row sizes (heights).
    pub row_sizes: Vec<HwpUnit16>,
    /// Border fill ID reference.
    pub border_fill_id: u16,
    /// Zone information for merged cells.
    pub zones: Vec<TableZone>,
}

impl TableProperties {
    /// Parses table properties from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u32()?;
        let row_count = reader.read_u16()?;
        let column_count = reader.read_u16()?;
        let cell_spacing = reader.read_hwp_unit16()?;
        let padding = TablePadding::from_reader(reader)?;

        // Read row sizes
        let mut row_sizes = Vec::with_capacity(row_count as usize);
        for _ in 0..row_count {
            row_sizes.push(reader.read_hwp_unit16()?);
        }

        let border_fill_id = reader.read_u16()?;

        // Read zones if data available (version 5.0.1.0+)
        let mut zones = Vec::new();
        if !reader.is_empty() {
            let zone_count = reader.read_u16()? as usize;
            zones.reserve(zone_count);
            for _ in 0..zone_count {
                if reader.remaining() >= TableZone::SIZE {
                    zones.push(TableZone::from_reader(reader)?);
                }
            }
        }

        Ok(Self {
            properties,
            row_count,
            column_count,
            cell_spacing,
            padding,
            row_sizes,
            border_fill_id,
            zones,
        })
    }

    /// Returns the page border split behavior.
    pub const fn page_border_split(&self) -> PageBorderSplit {
        PageBorderSplit::from_raw(self.properties)
    }

    /// Returns true if title row should auto-repeat.
    pub const fn auto_repeat_title_row(&self) -> bool {
        (self.properties & (1 << 2)) != 0
    }

    /// Returns total number of cells.
    pub const fn cell_count(&self) -> usize {
        self.row_count as usize * self.column_count as usize
    }
}

/// A table in the document.
#[derive(Debug, Clone, Default)]
pub struct Table {
    /// Table properties.
    pub properties: TableProperties,
    /// Table cells.
    pub cells: Vec<TableCell>,
}

impl Table {
    /// Creates a new table with properties.
    pub const fn new(properties: TableProperties) -> Self {
        Self {
            properties,
            cells: Vec::new(),
        }
    }

    /// Parses table from reader (after object common properties).
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = TableProperties::from_reader(reader)?;
        Ok(Self::new(properties))
    }

    /// Adds a cell to the table.
    pub fn add_cell(&mut self, cell: TableCell) {
        self.cells.push(cell);
    }

    /// Returns the number of rows.
    pub const fn row_count(&self) -> u16 {
        self.properties.row_count
    }

    /// Returns the number of columns.
    pub const fn column_count(&self) -> u16 {
        self.properties.column_count
    }

    /// Returns cells in a specific row.
    pub fn cells_in_row(&self, row: u16) -> Vec<&TableCell> {
        self.cells.iter().filter(|c| c.row == row).collect()
    }

    /// Returns a cell at specific position.
    pub fn cell_at(&self, row: u16, column: u16) -> Option<&TableCell> {
        self.cells
            .iter()
            .find(|c| c.row == row && c.column == column)
    }

    /// Extracts plain text from all cells in the table.
    pub fn plain_text(&self) -> String {
        let mut result = Vec::new();
        for row in 0..self.row_count() {
            let row_cells = self.cells_in_row(row);
            let row_text: Vec<String> = row_cells
                .iter()
                .map(|cell| {
                    cell.paragraphs
                        .iter()
                        .map(|p| p.plain_text())
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .collect();
            result.push(row_text.join("\t"));
        }
        result.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates test table properties data.
    fn create_test_table_data() -> Vec<u8> {
        let mut data = Vec::new();
        // properties: u32 (0)
        data.extend_from_slice(&0u32.to_le_bytes());
        // row_count: u16 (2)
        data.extend_from_slice(&2u16.to_le_bytes());
        // column_count: u16 (3)
        data.extend_from_slice(&3u16.to_le_bytes());
        // cell_spacing: HwpUnit16 (100)
        data.extend_from_slice(&100i16.to_le_bytes());
        // padding: left, right, top, bottom (각 50)
        data.extend_from_slice(&50i16.to_le_bytes());
        data.extend_from_slice(&50i16.to_le_bytes());
        data.extend_from_slice(&50i16.to_le_bytes());
        data.extend_from_slice(&50i16.to_le_bytes());
        // row_sizes (2 rows)
        data.extend_from_slice(&500i16.to_le_bytes());
        data.extend_from_slice(&600i16.to_le_bytes());
        // border_fill_id
        data.extend_from_slice(&1u16.to_le_bytes());

        data
    }

    #[test]
    fn test_table_properties_parsing() {
        let data = create_test_table_data();
        let mut reader = ByteReader::new(&data);

        let table = Table::from_reader(&mut reader).unwrap();

        assert_eq!(table.row_count(), 2);
        assert_eq!(table.column_count(), 3);
        assert_eq!(table.properties.cell_spacing.value(), 100);
        assert_eq!(table.properties.padding.left.value(), 50);
        assert_eq!(table.properties.row_sizes.len(), 2);
    }

    #[test]
    fn test_table_cell_operations() {
        let mut table = Table::default();
        table.properties.row_count = 2;
        table.properties.column_count = 2;

        // Add cells
        table.add_cell(TableCell {
            row: 0,
            column: 0,
            column_span: 1,
            row_span: 1,
            ..Default::default()
        });
        table.add_cell(TableCell {
            row: 0,
            column: 1,
            column_span: 1,
            row_span: 1,
            ..Default::default()
        });
        table.add_cell(TableCell {
            row: 1,
            column: 0,
            column_span: 2, // Merged cell
            row_span: 1,
            ..Default::default()
        });

        assert_eq!(table.cells.len(), 3);
        assert_eq!(table.cells_in_row(0).len(), 2);
        assert_eq!(table.cells_in_row(1).len(), 1);

        let cell = table.cell_at(1, 0).unwrap();
        assert!(cell.is_column_merged());
        assert!(!cell.is_row_merged());
    }

    #[test]
    fn test_table_properties_snapshot() {
        let data = create_test_table_data();
        let mut reader = ByteReader::new(&data);

        let table = Table::from_reader(&mut reader).unwrap();

        insta::assert_debug_snapshot!(table);
    }
}
