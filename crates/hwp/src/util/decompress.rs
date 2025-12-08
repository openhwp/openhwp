//! Decompression utilities for HWP streams.
//!
//! HWP uses raw deflate compression (without zlib header) for DocInfo,
//! BodyText, and DocHistory streams.

use crate::error::{Error, Result};
use miniz_oxide::inflate::{decompress_to_vec, decompress_to_vec_zlib};

/// Decompresses a compressed stream.
///
/// HWP streams use raw deflate compression without a zlib header.
/// This function first tries raw deflate, and falls back to zlib
/// if that fails (for compatibility with older documents).
///
/// # Arguments
///
/// * `data` - The compressed data
///
/// # Returns
///
/// The decompressed data as a Vec<u8>.
///
/// # Errors
///
/// Returns an error if decompression fails.
pub fn decompress_stream(data: &[u8]) -> Result<Vec<u8>> {
    // HWP uses raw deflate (no zlib header)
    decompress_to_vec(data)
        .or_else(|_| {
            // Fall back to zlib for compatibility
            decompress_to_vec_zlib(data)
        })
        .map_err(|e| Error::DecompressionFailed {
            description: format!("{:?}", e),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use miniz_oxide::deflate::compress_to_vec_zlib;

    #[test]
    fn test_decompress_roundtrip() {
        let original = b"Hello, HWP World! This is a test of zlib compression.";
        let compressed = compress_to_vec_zlib(original, 6);
        let decompressed = decompress_stream(&compressed).unwrap();
        assert_eq!(&decompressed, original);
    }

    #[test]
    fn test_decompress_empty() {
        // Empty zlib stream (just header and checksum)
        let compressed = compress_to_vec_zlib(&[], 6);
        let decompressed = decompress_stream(&compressed).unwrap();
        assert!(decompressed.is_empty());
    }

    #[test]
    fn test_decompress_invalid_data() {
        let invalid_data = [0x00, 0x01, 0x02, 0x03];
        assert!(decompress_stream(&invalid_data).is_err());
    }
}
