//! Binary data record.
//!
//! Defines binary data items like images and OLE objects.

use crate::error::Result;
use crate::util::ByteReader;

/// Type of binary data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryDataType {
    /// Link to external file.
    Link,
    /// Embedded binary data.
    Embedding,
    /// OLE storage.
    Storage,
}

impl BinaryDataType {
    /// Creates from raw value.
    pub const fn from_raw(value: u16) -> Self {
        match value & 0x0F {
            0 => Self::Link,
            1 => Self::Embedding,
            2 => Self::Storage,
            _ => Self::Embedding,
        }
    }
}

/// Compression mode for binary data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMode {
    /// Follow storage default.
    Default,
    /// Always compress.
    Compress,
    /// Never compress.
    NoCompress,
}

impl CompressionMode {
    /// Creates from raw value.
    pub const fn from_raw(value: u16) -> Self {
        match (value >> 4) & 0x03 {
            0 => Self::Default,
            1 => Self::Compress,
            2 => Self::NoCompress,
            _ => Self::Default,
        }
    }
}

/// Access state of binary data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryDataState {
    /// Not yet accessed.
    NotAccessed,
    /// Successfully accessed.
    AccessSuccess,
    /// Access failed.
    AccessFailed,
    /// Link access failed but ignored.
    AccessIgnored,
}

impl BinaryDataState {
    /// Creates from raw value.
    pub const fn from_raw(value: u16) -> Self {
        match (value >> 8) & 0x03 {
            0 => Self::NotAccessed,
            1 => Self::AccessSuccess,
            2 => Self::AccessFailed,
            3 => Self::AccessIgnored,
            _ => Self::NotAccessed,
        }
    }
}

/// Binary data item.
///
/// Represents an image, OLE object, or other binary resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryData {
    /// Data type.
    data_type: BinaryDataType,
    /// Compression mode.
    compression: CompressionMode,
    /// Access state.
    state: BinaryDataState,
    /// Absolute path (for Link type).
    absolute_path: Option<String>,
    /// Relative path (for Link type).
    relative_path: Option<String>,
    /// Binary data ID in storage (for Embedding/Storage types).
    storage_id: Option<u16>,
    /// File extension without dot (for Embedding type).
    extension: Option<String>,
}

impl BinaryData {
    /// Parses BinaryData from a reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u16()?;
        let data_type = BinaryDataType::from_raw(properties);
        let compression = CompressionMode::from_raw(properties);
        let state = BinaryDataState::from_raw(properties);

        let mut absolute_path = None;
        let mut relative_path = None;
        let mut storage_id = None;
        let mut extension = None;

        match data_type {
            BinaryDataType::Link => {
                absolute_path = Some(reader.read_utf16_string()?);
                relative_path = Some(reader.read_utf16_string()?);
            }
            BinaryDataType::Embedding => {
                storage_id = Some(reader.read_u16()?);
                extension = Some(reader.read_utf16_string()?);
            }
            BinaryDataType::Storage => {
                storage_id = Some(reader.read_u16()?);
            }
        }

        Ok(Self {
            data_type,
            compression,
            state,
            absolute_path,
            relative_path,
            storage_id,
            extension,
        })
    }

    /// Returns the data type.
    #[inline]
    pub const fn data_type(&self) -> BinaryDataType {
        self.data_type
    }

    /// Returns the compression mode.
    #[inline]
    pub const fn compression(&self) -> CompressionMode {
        self.compression
    }

    /// Returns the access state.
    #[inline]
    pub const fn state(&self) -> BinaryDataState {
        self.state
    }

    /// Returns the absolute path (for Link type).
    #[inline]
    pub fn absolute_path(&self) -> Option<&str> {
        self.absolute_path.as_deref()
    }

    /// Returns the relative path (for Link type).
    #[inline]
    pub fn relative_path(&self) -> Option<&str> {
        self.relative_path.as_deref()
    }

    /// Returns the storage ID (for Embedding/Storage types).
    #[inline]
    pub const fn storage_id(&self) -> Option<u16> {
        self.storage_id
    }

    /// Returns the file extension (for Embedding type).
    #[inline]
    pub fn extension(&self) -> Option<&str> {
        self.extension.as_deref()
    }

    /// Returns the stream name in BinData storage.
    ///
    /// The format is "BIN{id:04X}.{extension}".
    pub fn stream_name(&self) -> Option<String> {
        self.storage_id.map(|id| {
            let ext = self.extension.as_deref().unwrap_or("bin");
            format!("BIN{:04X}.{}", id, ext)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_data_type() {
        assert_eq!(BinaryDataType::from_raw(0x0000), BinaryDataType::Link);
        assert_eq!(BinaryDataType::from_raw(0x0001), BinaryDataType::Embedding);
        assert_eq!(BinaryDataType::from_raw(0x0002), BinaryDataType::Storage);
    }

    #[test]
    fn test_compression_mode() {
        assert_eq!(CompressionMode::from_raw(0x0000), CompressionMode::Default);
        assert_eq!(CompressionMode::from_raw(0x0010), CompressionMode::Compress);
        assert_eq!(
            CompressionMode::from_raw(0x0020),
            CompressionMode::NoCompress
        );
    }

    #[test]
    fn test_binary_data_state() {
        assert_eq!(BinaryDataState::from_raw(0x0000), BinaryDataState::NotAccessed);
        assert_eq!(
            BinaryDataState::from_raw(0x0100),
            BinaryDataState::AccessSuccess
        );
    }
}
