//! Document arbitrary data (HWPTAG_DOC_DATA) parsing.
//!
//! Stores label document information and print dialog settings.
//! Uses a Parameter Set structure for flexible key-value storage.

use crate::error::Result;
use crate::util::ByteReader;

/// Parameter item type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParameterType {
    /// Unknown type.
    #[default]
    Unknown,
    /// Null/empty value.
    Null,
    /// Boolean value.
    Bool,
    /// 8-bit signed integer.
    I1,
    /// 16-bit signed integer.
    I2,
    /// 32-bit signed integer.
    I4,
    /// 8-bit unsigned integer.
    UI1,
    /// 16-bit unsigned integer.
    UI2,
    /// 32-bit unsigned integer.
    UI4,
    /// Unicode string.
    String,
    /// Binary data reference.
    BinaryData,
    /// Nested parameter set.
    Set,
    /// Array of parameter sets.
    Array,
}

impl ParameterType {
    /// Creates from raw value.
    pub const fn from_raw(value: u16) -> Self {
        match value {
            0 => Self::Null,
            1 => Self::Bool,
            2 => Self::I1,
            3 => Self::I2,
            4 => Self::I4,
            5 => Self::UI1,
            6 => Self::UI2,
            7 => Self::String,
            8 => Self::UI4,
            0x8000 => Self::Set,
            0x8001 => Self::Array,
            0x8002 => Self::BinaryData,
            _ => Self::Unknown,
        }
    }
}

/// Parameter item value.
#[derive(Debug, Clone)]
pub enum ParameterValue {
    /// Null value.
    Null,
    /// Boolean value.
    Bool(bool),
    /// Integer value.
    Integer(i64),
    /// Unsigned integer value.
    Unsigned(u64),
    /// String value.
    String(String),
    /// Binary data ID.
    BinaryData(u16),
    /// Nested parameter set.
    Set(Box<ParameterSet>),
    /// Array of parameter sets.
    Array(Vec<ParameterSet>),
}

impl Default for ParameterValue {
    fn default() -> Self {
        Self::Null
    }
}

/// Parameter item (key-value pair).
#[derive(Debug, Clone, Default)]
pub struct ParameterItem {
    /// Item ID.
    id: u16,
    /// Item type.
    item_type: ParameterType,
    /// Item value.
    value: ParameterValue,
}

impl ParameterItem {
    /// Creates a new parameter item.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the item ID.
    pub const fn id(&self) -> u16 {
        self.id
    }

    /// Returns the item type.
    pub const fn item_type(&self) -> ParameterType {
        self.item_type
    }

    /// Returns the item value.
    pub fn value(&self) -> &ParameterValue {
        &self.value
    }

    /// Parses parameter item from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        if reader.remaining() < 4 {
            return Ok(Self::default());
        }

        let id = reader.read_u16()?;
        let type_raw = reader.read_u16()?;
        let item_type = ParameterType::from_raw(type_raw);

        let value = match item_type {
            ParameterType::Null => ParameterValue::Null,
            ParameterType::Bool => {
                let v = if reader.remaining() >= 4 {
                    reader.read_u32()? != 0
                } else {
                    false
                };
                ParameterValue::Bool(v)
            }
            ParameterType::I1 => {
                let v = if reader.remaining() >= 1 {
                    reader.read_u8()? as i8 as i64
                } else {
                    0
                };
                ParameterValue::Integer(v)
            }
            ParameterType::I2 => {
                let v = if reader.remaining() >= 2 {
                    reader.read_i16()? as i64
                } else {
                    0
                };
                ParameterValue::Integer(v)
            }
            ParameterType::I4 => {
                let v = if reader.remaining() >= 4 {
                    reader.read_i32()? as i64
                } else {
                    0
                };
                ParameterValue::Integer(v)
            }
            ParameterType::UI1 => {
                let v = if reader.remaining() >= 1 {
                    reader.read_u8()? as u64
                } else {
                    0
                };
                ParameterValue::Unsigned(v)
            }
            ParameterType::UI2 => {
                let v = if reader.remaining() >= 2 {
                    reader.read_u16()? as u64
                } else {
                    0
                };
                ParameterValue::Unsigned(v)
            }
            ParameterType::UI4 => {
                let v = if reader.remaining() >= 4 {
                    reader.read_u32()? as u64
                } else {
                    0
                };
                ParameterValue::Unsigned(v)
            }
            ParameterType::String => {
                let s = if reader.remaining() >= 2 {
                    let len = reader.read_u16()? as usize;
                    if len > 0 && reader.remaining() >= len * 2 {
                        let mut chars = Vec::with_capacity(len);
                        for _ in 0..len {
                            chars.push(reader.read_u16()?);
                        }
                        String::from_utf16_lossy(&chars)
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                ParameterValue::String(s)
            }
            ParameterType::BinaryData => {
                let id = if reader.remaining() >= 2 {
                    reader.read_u16()?
                } else {
                    0
                };
                ParameterValue::BinaryData(id)
            }
            ParameterType::Set => {
                let set = ParameterSet::from_reader(reader)?;
                ParameterValue::Set(Box::new(set))
            }
            ParameterType::Array => {
                let count = if reader.remaining() >= 2 {
                    reader.read_u16()? as usize
                } else {
                    0
                };
                let mut array = Vec::with_capacity(count);
                for _ in 0..count {
                    array.push(ParameterSet::from_reader(reader)?);
                }
                ParameterValue::Array(array)
            }
            ParameterType::Unknown => ParameterValue::Null,
        };

        Ok(Self {
            id,
            item_type,
            value,
        })
    }
}

/// Parameter set (collection of parameter items).
#[derive(Debug, Clone, Default)]
pub struct ParameterSet {
    /// Set ID.
    set_id: u16,
    /// Items in this set.
    items: Vec<ParameterItem>,
}

impl ParameterSet {
    /// Creates a new parameter set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the set ID.
    pub const fn set_id(&self) -> u16 {
        self.set_id
    }

    /// Returns the items.
    pub fn items(&self) -> &[ParameterItem] {
        &self.items
    }

    /// Parses parameter set from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        if reader.remaining() < 4 {
            return Ok(Self::default());
        }

        let set_id = reader.read_u16()?;
        let item_count = reader.read_u16()? as usize;

        let mut items = Vec::with_capacity(item_count);
        for _ in 0..item_count {
            if reader.is_empty() {
                break;
            }
            items.push(ParameterItem::from_reader(reader)?);
        }

        Ok(Self { set_id, items })
    }
}

/// Document arbitrary data (HWPTAG_DOC_DATA).
///
/// Stores label document information, print settings, etc.
#[derive(Debug, Clone, Default)]
pub struct DocumentData {
    /// Root parameter set.
    parameter_set: ParameterSet,
}

impl DocumentData {
    /// Creates a new document data.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the parameter set.
    pub fn parameter_set(&self) -> &ParameterSet {
        &self.parameter_set
    }

    /// Parses document data from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let parameter_set = ParameterSet::from_reader(reader)?;
        Ok(Self { parameter_set })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_type_from_raw() {
        assert_eq!(ParameterType::from_raw(0), ParameterType::Null);
        assert_eq!(ParameterType::from_raw(1), ParameterType::Bool);
        assert_eq!(ParameterType::from_raw(7), ParameterType::String);
        assert_eq!(ParameterType::from_raw(0x8000), ParameterType::Set);
        assert_eq!(ParameterType::from_raw(0xFFFF), ParameterType::Unknown);
    }

    #[test]
    fn test_parameter_item_new() {
        let item = ParameterItem::new();
        assert_eq!(item.id(), 0);
        assert_eq!(item.item_type(), ParameterType::Unknown);
    }

    #[test]
    fn test_parameter_set_new() {
        let set = ParameterSet::new();
        assert_eq!(set.set_id(), 0);
        assert!(set.items().is_empty());
    }

    #[test]
    fn test_document_data_new() {
        let data = DocumentData::new();
        assert_eq!(data.parameter_set().set_id(), 0);
    }
}
