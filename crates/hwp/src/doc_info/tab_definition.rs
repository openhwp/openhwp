//! Tab definition record.

use crate::error::Result;
use crate::primitive::HwpUnit;
use crate::util::ByteReader;

/// Tab alignment type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabType {
    #[default]
    Left,
    Right,
    Center,
    Decimal,
}

impl TabType {
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::Center,
            3 => Self::Decimal,
            _ => Self::Left,
        }
    }
}

/// Individual tab stop information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabInfo {
    pub position: HwpUnit,
    pub tab_type: TabType,
    pub fill_type: u8,
}

impl TabInfo {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let position = reader.read_hwp_unit()?;
        let tab_type = TabType::from_raw(reader.read_u8()?);
        let fill_type = reader.read_u8()?;
        reader.skip(2)?; // padding
        Ok(Self {
            position,
            tab_type,
            fill_type,
        })
    }
}

/// Tab definition.
#[derive(Debug, Clone)]
pub struct TabDefinition {
    properties: u32,
    tabs: Vec<TabInfo>,
}

impl TabDefinition {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u32()?;
        let count = reader.read_i16()? as usize;

        let mut tabs = Vec::with_capacity(count);
        for _ in 0..count {
            tabs.push(TabInfo::from_reader(reader)?);
        }

        Ok(Self { properties, tabs })
    }

    /// Whether auto tab at left paragraph edge is enabled.
    pub const fn has_left_auto_tab(&self) -> bool {
        (self.properties & (1 << 0)) != 0
    }

    /// Whether auto tab at right paragraph edge is enabled.
    pub const fn has_right_auto_tab(&self) -> bool {
        (self.properties & (1 << 1)) != 0
    }

    /// Returns the tab stops.
    pub fn tabs(&self) -> &[TabInfo] {
        &self.tabs
    }
}
