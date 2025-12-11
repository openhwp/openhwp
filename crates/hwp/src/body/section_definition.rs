//! Section and column definition parsing.
//!
//! Section definition (secd) and column definition (cold) control types.

use crate::error::Result;
use crate::primitive::ColorReference;
use crate::util::ByteReader;

/// Section definition (구역 정의, 표 129).
#[derive(Debug, Clone, Default)]
pub struct SectionDefinition {
    /// Properties (표 130).
    pub properties: u32,
    /// Gap between columns in same page.
    pub column_gap: u16,
    /// Vertical grid interval (0 = off).
    pub vertical_grid: u16,
    /// Horizontal grid interval (0 = off).
    pub horizontal_grid: u16,
    /// Default tab interval.
    pub default_tab_interval: u32,
    /// Numbering paragraph shape ID.
    pub numbering_shape_id: u16,
    /// Starting page number (0 = continue from previous section).
    pub starting_page_number: u16,
    /// Figure, table, equation starting numbers.
    pub starting_figure_number: u16,
    pub starting_table_number: u16,
    pub starting_equation_number: u16,
    /// Representative language.
    pub language: u16,
}

impl SectionDefinition {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u32()?;
        let column_gap = reader.read_u16()?;
        let vertical_grid = reader.read_u16()?;
        let horizontal_grid = reader.read_u16()?;
        let default_tab_interval = reader.read_u32()?;
        let numbering_shape_id = reader.read_u16()?;
        let starting_page_number = reader.read_u16()?;
        let starting_figure_number = reader.read_u16()?;
        let starting_table_number = reader.read_u16()?;
        let starting_equation_number = reader.read_u16()?;
        let language = if reader.remaining() >= 2 {
            reader.read_u16()?
        } else {
            0
        };

        Ok(Self {
            properties,
            column_gap,
            vertical_grid,
            horizontal_grid,
            default_tab_interval,
            numbering_shape_id,
            starting_page_number,
            starting_figure_number,
            starting_table_number,
            starting_equation_number,
            language,
        })
    }

    /// Returns whether header is hidden.
    pub const fn hide_header(&self) -> bool {
        (self.properties & (1 << 0)) != 0
    }

    /// Returns whether footer is hidden.
    pub const fn hide_footer(&self) -> bool {
        (self.properties & (1 << 1)) != 0
    }

    /// Returns whether master page is hidden.
    pub const fn hide_master_page(&self) -> bool {
        (self.properties & (1 << 2)) != 0
    }

    /// Returns whether border is hidden.
    pub const fn hide_border(&self) -> bool {
        (self.properties & (1 << 3)) != 0
    }

    /// Returns whether background is hidden.
    pub const fn hide_background(&self) -> bool {
        (self.properties & (1 << 4)) != 0
    }

    /// Returns whether page number position is hidden.
    pub const fn hide_page_number(&self) -> bool {
        (self.properties & (1 << 5)) != 0
    }

    /// Returns text direction (0: horizontal, 1: vertical).
    pub const fn text_direction(&self) -> u8 {
        ((self.properties >> 16) & 0x07) as u8
    }

    /// Returns page starts on option (bits 20-21).
    /// 0: Both, 1: Even, 2: Odd
    pub const fn page_starts_on(&self) -> u8 {
        ((self.properties >> 20) & 0x03) as u8
    }
}

/// Column type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnType {
    /// Normal multi-column.
    #[default]
    Normal,
    /// Distributed multi-column.
    Distributed,
    /// Parallel multi-column.
    Parallel,
}

impl ColumnType {
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Distributed,
            2 => Self::Parallel,
            _ => Self::Normal,
        }
    }
}

/// Column direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnDirection {
    /// Left to right.
    #[default]
    LeftToRight,
    /// Right to left.
    RightToLeft,
    /// Facing pages.
    FacingPages,
}

impl ColumnDirection {
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::LeftToRight,
            1 => Self::RightToLeft,
            2 => Self::FacingPages,
            _ => Self::LeftToRight,
        }
    }
}

/// Column definition (단 정의, 표 138).
#[derive(Debug, Clone, Default)]
pub struct ColumnDefinition {
    /// Properties bits 0-15 (표 139).
    pub properties1: u16,
    /// Gap between columns.
    pub column_gap: u16,
    /// Column widths (if not same width).
    pub column_widths: Vec<u16>,
    /// Properties bits 16-32 (표 139).
    pub properties2: u16,
    /// Column separator line style.
    pub separator_style: u8,
    /// Column separator line thickness.
    pub separator_thickness: u8,
    /// Column separator line color.
    pub separator_color: ColorReference,
}

impl ColumnDefinition {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties1 = reader.read_u16()?;
        let column_gap = reader.read_u16()?;

        // Extract column count from properties
        let column_count = ((properties1 >> 2) & 0xFF) as usize;
        let same_width = (properties1 & (1 << 12)) != 0;

        // Read column widths if not same width
        let column_widths = if !same_width && column_count > 0 {
            let mut widths = Vec::with_capacity(column_count);
            for _ in 0..column_count {
                if reader.remaining() >= 2 {
                    widths.push(reader.read_u16()?);
                }
            }
            widths
        } else {
            Vec::new()
        };

        let properties2 = if reader.remaining() >= 2 {
            reader.read_u16()?
        } else {
            0
        };

        let separator_style = if reader.remaining() >= 1 {
            reader.read_u8()?
        } else {
            0
        };

        let separator_thickness = if reader.remaining() >= 1 {
            reader.read_u8()?
        } else {
            0
        };

        let separator_color = if reader.remaining() >= 4 {
            reader.read_color()?
        } else {
            ColorReference::default()
        };

        Ok(Self {
            properties1,
            column_gap,
            column_widths,
            properties2,
            separator_style,
            separator_thickness,
            separator_color,
        })
    }

    /// Returns column type.
    pub const fn column_type(&self) -> ColumnType {
        ColumnType::from_raw((self.properties1 & 0x03) as u8)
    }

    /// Returns number of columns.
    pub const fn column_count(&self) -> u8 {
        ((self.properties1 >> 2) & 0xFF) as u8
    }

    /// Returns column direction.
    pub const fn direction(&self) -> ColumnDirection {
        ColumnDirection::from_raw(((self.properties1 >> 10) & 0x03) as u8)
    }

    /// Returns whether columns have same width.
    pub const fn same_width(&self) -> bool {
        (self.properties1 & (1 << 12)) != 0
    }
}
