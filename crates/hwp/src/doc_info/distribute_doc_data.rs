//! Distribution document data (HWPTAG_DISTRIBUTE_DOC_DATA) parsing.
//!
//! Distribution documents have special encryption applied to all streams.
//! This 256-byte record appears in all streams of a distribution document.

use crate::error::Result;
use crate::util::ByteReader;

/// Distribution document data size in bytes.
pub const DISTRIBUTE_DOC_DATA_SIZE: usize = 256;

/// Distribution document data.
///
/// This 256-byte record is present in distribution documents and contains
/// encryption-related data that is used to protect document content.
#[derive(Debug, Clone)]
pub struct DistributeDocData {
    /// Raw distribution data (256 bytes).
    data: [u8; DISTRIBUTE_DOC_DATA_SIZE],
}

impl Default for DistributeDocData {
    fn default() -> Self {
        Self::new()
    }
}

impl DistributeDocData {
    /// Creates a new empty distribution document data.
    pub fn new() -> Self {
        Self {
            data: [0u8; DISTRIBUTE_DOC_DATA_SIZE],
        }
    }

    /// Creates from raw data.
    pub fn from_data(data: [u8; DISTRIBUTE_DOC_DATA_SIZE]) -> Self {
        Self { data }
    }

    /// Returns the raw data.
    pub fn data(&self) -> &[u8; DISTRIBUTE_DOC_DATA_SIZE] {
        &self.data
    }

    /// Returns a slice of the data.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Parses distribution document data from reader.
    ///
    /// Format (per HWP spec - HWPTAG_DISTRIBUTE_DOC_DATA):
    /// - BYTE array[256]: Distribution document data
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let mut data = [0u8; DISTRIBUTE_DOC_DATA_SIZE];

        if reader.remaining() >= DISTRIBUTE_DOC_DATA_SIZE {
            let bytes = reader.read_bytes(DISTRIBUTE_DOC_DATA_SIZE)?;
            data.copy_from_slice(bytes);
        } else {
            // Read what's available
            let available = reader.remaining();
            if available > 0 {
                let bytes = reader.read_bytes(available)?;
                data[..available].copy_from_slice(bytes);
            }
        }

        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute_doc_data_new() {
        let data = DistributeDocData::new();
        assert_eq!(data.data().len(), 256);
        assert!(data.as_slice().iter().all(|&b| b == 0));
    }

    #[test]
    fn test_distribute_doc_data_from_data() {
        let mut raw = [0u8; 256];
        raw[0] = 0x12;
        raw[1] = 0x34;
        raw[255] = 0xFF;

        let data = DistributeDocData::from_data(raw);
        assert_eq!(data.data()[0], 0x12);
        assert_eq!(data.data()[1], 0x34);
        assert_eq!(data.data()[255], 0xFF);
    }

    #[test]
    fn test_distribute_doc_data_from_reader() {
        let mut raw = vec![0u8; 256];
        raw[0] = 0xAB;
        raw[255] = 0xCD;

        let mut reader = ByteReader::new(&raw);
        let data = DistributeDocData::from_reader(&mut reader).unwrap();

        assert_eq!(data.data()[0], 0xAB);
        assert_eq!(data.data()[255], 0xCD);
    }

    #[test]
    fn test_distribute_doc_data_partial() {
        // Test with less than 256 bytes
        let raw = vec![0x11, 0x22, 0x33];
        let mut reader = ByteReader::new(&raw);
        let data = DistributeDocData::from_reader(&mut reader).unwrap();

        assert_eq!(data.data()[0], 0x11);
        assert_eq!(data.data()[1], 0x22);
        assert_eq!(data.data()[2], 0x33);
        assert_eq!(data.data()[3], 0x00); // Rest is zeros
    }
}
