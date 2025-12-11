//! Distribution document decryption.
//!
//! Distribution documents are special HWP documents designed for read-only distribution.
//! They use a different encryption scheme with optional copy/print restrictions.

use crate::error::{Error, Result};

/// Distribution document decryptor.
///
/// Handles decryption of distribution documents which use ViewText instead of BodyText.
#[derive(Debug, Clone)]
pub struct DistributionDecryptor {
    /// The decryption key (first 16 bytes of SHA-1 hash).
    key: [u8; 16],
}

impl DistributionDecryptor {
    /// Size of the distribution document data record.
    pub const DISTRIBUTION_DATA_SIZE: usize = 256;

    /// Creates a new DistributionDecryptor from distribution data.
    ///
    /// The distribution data is 256 bytes that contains the encrypted
    /// hash code and options.
    ///
    /// # Arguments
    ///
    /// * `distribution_data` - The 256-byte distribution document data
    ///
    /// # Returns
    ///
    /// A configured decryptor.
    pub fn from_distribution_data(distribution_data: &[u8]) -> Result<Self> {
        if distribution_data.len() < Self::DISTRIBUTION_DATA_SIZE {
            return Err(Error::UnexpectedEndOfData {
                expected: Self::DISTRIBUTION_DATA_SIZE,
                actual: distribution_data.len(),
            });
        }

        // Extract seed (first 4 bytes)
        let seed = u32::from_le_bytes([
            distribution_data[0],
            distribution_data[1],
            distribution_data[2],
            distribution_data[3],
        ]);

        // Generate random array using MSVC-compatible random
        let random_array = generate_random_array(seed);

        // XOR the distribution data with the random array to get hash code
        let mut decoded = [0u8; 82];
        for i in 0..82 {
            decoded[i] = distribution_data[i] ^ random_array[i];
        }

        // First 80 bytes are the SHA-1 hash (as WCHAR[40]), take first 16 bytes as key
        let mut key = [0u8; 16];
        key.copy_from_slice(&decoded[0..16]);

        Ok(Self { key })
    }

    /// Decrypts a distribution document stream.
    ///
    /// # Arguments
    ///
    /// * `data` - The encrypted stream data
    ///
    /// # Returns
    ///
    /// The decrypted data.
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        // Distribution documents use a simple XOR cipher with the key
        let mut result = Vec::with_capacity(data.len());
        for (i, &byte) in data.iter().enumerate() {
            result.push(byte ^ self.key[i % 16]);
        }

        Ok(result)
    }
}

/// Generates a random array using MSVC-compatible random number generator.
///
/// This replicates the behavior of MSVC's srand()/rand() functions
/// which are used in the HWP distribution document encryption.
fn generate_random_array(seed: u32) -> [u8; 256] {
    let mut result = [0u8; 256];
    let mut state = seed;

    for byte in result.iter_mut() {
        // MSVC rand() implementation: state = state * 214013 + 2531011
        state = state.wrapping_mul(214013).wrapping_add(2531011);
        // Extract bits 16-22 (7 bits) for the random value
        *byte = ((state >> 16) & 0xFF) as u8;
    }

    result
}

/// Convenience function to decrypt a distribution document stream.
///
/// # Arguments
///
/// * `data` - The encrypted stream data
/// * `distribution_data` - The 256-byte distribution document data
///
/// # Returns
///
/// The decrypted data.
pub fn decrypt_distribution_stream(data: &[u8], distribution_data: &[u8]) -> Result<Vec<u8>> {
    let decryptor = DistributionDecryptor::from_distribution_data(distribution_data)?;
    decryptor.decrypt(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_array_deterministic() {
        let arr1 = generate_random_array(12345);
        let arr2 = generate_random_array(12345);
        assert_eq!(arr1, arr2);

        let arr3 = generate_random_array(54321);
        assert_ne!(arr1, arr3);
    }

    #[test]
    fn test_decrypt_empty() {
        let distribution_data = [0u8; 256];
        let decryptor = DistributionDecryptor::from_distribution_data(&distribution_data).unwrap();
        let result = decryptor.decrypt(&[]).unwrap();
        assert!(result.is_empty());
    }
}
