//! Control data (HWPTAG_CTRL_DATA) parsing.
//!
//! Control data contains arbitrary field data using the Parameter Set format.
//! This is used to store field names, hyperlink URLs, and other control-specific data.
//!
//! ## Parameter Set Format (HWP Spec Table 50-52)
//!
//! Parameter Set:
//! - WORD: Set ID
//! - INT16: Item count
//! - Parameter Items...
//!
//! Parameter Item:
//! - WORD: Item ID
//! - WORD: Type (PIT_*)
//! - Data (type-specific)

use crate::error::Result;
use crate::util::ByteReader;
use std::collections::HashMap;

/// Parameter item types (표 52).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ParameterItemType {
    /// Null value.
    Null = 0x0000,
    /// Binary string (BSTR) - UTF-16LE with length prefix.
    BinaryString = 0x0001,
    /// 1-byte signed integer.
    Int1 = 0x0002,
    /// 2-byte signed integer.
    Int2 = 0x0003,
    /// 4-byte signed integer.
    Int4 = 0x0004,
    /// 1-byte unsigned integer.
    UInt1 = 0x0005,
    /// 2-byte unsigned integer.
    UInt2 = 0x0006,
    /// 4-byte unsigned integer.
    UInt4 = 0x0007,
    /// Boolean (treated as UInt4).
    Bool = 0x0008,
    /// Unknown type.
    Unknown = 0xFFFF,
}

impl ParameterItemType {
    /// Creates from raw u16 value.
    pub fn from_raw(value: u16) -> Self {
        match value {
            0x0000 => Self::Null,
            0x0001 => Self::BinaryString,
            0x0002 => Self::Int1,
            0x0003 => Self::Int2,
            0x0004 => Self::Int4,
            0x0005 => Self::UInt1,
            0x0006 => Self::UInt2,
            0x0007 => Self::UInt4,
            0x0008 => Self::Bool,
            _ => Self::Unknown,
        }
    }
}

/// A parameter item value.
#[derive(Debug, Clone)]
pub enum ParameterValue {
    /// Null value.
    Null,
    /// String value.
    String(String),
    /// Integer value.
    Integer(i64),
    /// Unsigned integer value.
    UnsignedInteger(u64),
    /// Boolean value.
    Boolean(bool),
    /// Binary data (for unknown types).
    Binary(Vec<u8>),
}

impl ParameterValue {
    /// Returns the value as a string, if it is one.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the value as an integer, if it is one.
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(n) => Some(*n),
            Self::UnsignedInteger(n) => Some(*n as i64),
            _ => None,
        }
    }

    /// Returns the value as a boolean, if it is one.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            Self::Integer(n) => Some(*n != 0),
            Self::UnsignedInteger(n) => Some(*n != 0),
            _ => None,
        }
    }
}

/// A parameter item in a parameter set.
#[derive(Debug, Clone)]
pub struct ParameterItem {
    /// Item ID.
    pub id: u16,
    /// Item type.
    pub item_type: ParameterItemType,
    /// Item value.
    pub value: ParameterValue,
}

/// A parameter set containing multiple items.
#[derive(Debug, Clone, Default)]
pub struct ParameterSet {
    /// Set ID.
    pub id: u16,
    /// Items in this set, keyed by item ID.
    items: HashMap<u16, ParameterItem>,
}

impl ParameterSet {
    /// Creates a new empty parameter set.
    pub fn new(id: u16) -> Self {
        Self {
            id,
            items: HashMap::new(),
        }
    }

    /// Parses a parameter set from a reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let set_id = reader.read_u16()?;
        let item_count = reader.read_i16()? as usize;

        let mut set = Self::new(set_id);

        for _ in 0..item_count {
            if reader.remaining() < 4 {
                break;
            }

            let item_id = reader.read_u16()?;
            let type_raw = reader.read_u16()?;
            let item_type = ParameterItemType::from_raw(type_raw);

            let value = match item_type {
                ParameterItemType::Null => ParameterValue::Null,
                ParameterItemType::BinaryString => {
                    let s = reader.read_utf16_string()?;
                    ParameterValue::String(s)
                }
                ParameterItemType::Int1 => {
                    let v = reader.read_i8()? as i64;
                    ParameterValue::Integer(v)
                }
                ParameterItemType::Int2 => {
                    let v = reader.read_i16()? as i64;
                    ParameterValue::Integer(v)
                }
                ParameterItemType::Int4 => {
                    let v = reader.read_i32()? as i64;
                    ParameterValue::Integer(v)
                }
                ParameterItemType::UInt1 => {
                    let v = reader.read_u8()? as u64;
                    ParameterValue::UnsignedInteger(v)
                }
                ParameterItemType::UInt2 => {
                    let v = reader.read_u16()? as u64;
                    ParameterValue::UnsignedInteger(v)
                }
                ParameterItemType::UInt4 => {
                    let v = reader.read_u32()? as u64;
                    ParameterValue::UnsignedInteger(v)
                }
                ParameterItemType::Bool => {
                    let v = reader.read_u32()? != 0;
                    ParameterValue::Boolean(v)
                }
                ParameterItemType::Unknown => {
                    // For nested sets (type >= 0x8000), recursively parse
                    if type_raw >= 0x8000 {
                        // Nested parameter set - parse recursively
                        // The nested set uses the low 15 bits as its set ID
                        let nested = Self::from_reader(reader)?;
                        // Store as binary for now
                        ParameterValue::Binary(format!("NestedSet({})", nested.id).into_bytes())
                    } else {
                        ParameterValue::Null
                    }
                }
            };

            set.items.insert(
                item_id,
                ParameterItem {
                    id: item_id,
                    item_type,
                    value,
                },
            );
        }

        Ok(set)
    }

    /// Gets an item by ID.
    pub fn get(&self, id: u16) -> Option<&ParameterItem> {
        self.items.get(&id)
    }

    /// Gets a string value by item ID.
    pub fn get_string(&self, id: u16) -> Option<&str> {
        self.items.get(&id).and_then(|item| item.value.as_string())
    }

    /// Gets an integer value by item ID.
    pub fn get_integer(&self, id: u16) -> Option<i64> {
        self.items.get(&id).and_then(|item| item.value.as_integer())
    }

    /// Gets a boolean value by item ID.
    pub fn get_bool(&self, id: u16) -> Option<bool> {
        self.items.get(&id).and_then(|item| item.value.as_bool())
    }

    /// Returns true if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the number of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns an iterator over all items.
    pub fn items(&self) -> impl Iterator<Item = &ParameterItem> {
        self.items.values()
    }
}

/// Control data record (HWPTAG_CTRL_DATA).
///
/// Contains parameter sets for field controls, hyperlinks, etc.
#[derive(Debug, Clone, Default)]
pub struct ControlData {
    /// Parameter sets in this control data.
    sets: Vec<ParameterSet>,
}

impl ControlData {
    /// Parses control data from a byte slice.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let mut reader = ByteReader::new(data);
        Self::from_reader(&mut reader)
    }

    /// Parses control data from a reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let mut sets = Vec::new();

        // Parse all parameter sets in the data
        while reader.remaining() >= 4 {
            sets.push(ParameterSet::from_reader(reader)?);
        }

        Ok(Self { sets })
    }

    /// Returns the first parameter set, if any.
    pub fn first_set(&self) -> Option<&ParameterSet> {
        self.sets.first()
    }

    /// Returns all parameter sets.
    pub fn sets(&self) -> &[ParameterSet] {
        &self.sets
    }

    /// Gets a string value from the first set.
    pub fn get_string(&self, item_id: u16) -> Option<&str> {
        self.sets.first().and_then(|set| set.get_string(item_id))
    }

    /// Gets an integer value from the first set.
    pub fn get_integer(&self, item_id: u16) -> Option<i64> {
        self.sets.first().and_then(|set| set.get_integer(item_id))
    }
}

/// Known parameter item IDs for field controls.
pub mod field_item_ids {
    /// Field command/instruction string.
    pub const COMMAND: u16 = 0x4000;
    /// Field name.
    pub const NAME: u16 = 0x4001;
}

/// Known parameter item IDs for hyperlink controls.
pub mod hyperlink_item_ids {
    /// Hyperlink target URL or path.
    pub const TARGET: u16 = 0x4000;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_item_type_from_raw() {
        assert_eq!(ParameterItemType::from_raw(0), ParameterItemType::Null);
        assert_eq!(
            ParameterItemType::from_raw(1),
            ParameterItemType::BinaryString
        );
        assert_eq!(ParameterItemType::from_raw(4), ParameterItemType::Int4);
        assert_eq!(ParameterItemType::from_raw(7), ParameterItemType::UInt4);
        assert_eq!(ParameterItemType::from_raw(8), ParameterItemType::Bool);
        assert_eq!(
            ParameterItemType::from_raw(0x8000),
            ParameterItemType::Unknown
        );
    }

    #[test]
    fn test_parameter_value_conversions() {
        let s = ParameterValue::String("test".to_string());
        assert_eq!(s.as_string(), Some("test"));
        assert_eq!(s.as_integer(), None);

        let n = ParameterValue::Integer(42);
        assert_eq!(n.as_integer(), Some(42));
        assert_eq!(n.as_bool(), Some(true));

        let zero = ParameterValue::Integer(0);
        assert_eq!(zero.as_bool(), Some(false));
    }

    #[test]
    fn test_parameter_set_new() {
        let set = ParameterSet::new(0x1234);
        assert_eq!(set.id, 0x1234);
        assert!(set.is_empty());
    }
}
