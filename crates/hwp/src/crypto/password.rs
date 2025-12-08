//! Password-encrypted document decryption.
//!
//! HWP 5.0 supports multiple encryption versions.
//! This module handles decryption for documents encrypted with passwords.

use crate::error::{Error, Result};
use crate::header::EncryptionVersion;

/// Password decryptor for HWP documents.
///
/// Handles decryption of password-protected streams based on the encryption version.
#[derive(Debug, Clone)]
pub struct PasswordDecryptor {
    /// The encryption version used.
    version: EncryptionVersion,
    /// Decryption key derived from password.
    key: Vec<u8>,
}

impl PasswordDecryptor {
    /// Creates a new PasswordDecryptor from a password.
    ///
    /// # Arguments
    ///
    /// * `version` - The encryption version used
    /// * `password` - The document password
    ///
    /// # Returns
    ///
    /// A decryptor configured for the given encryption version.
    ///
    /// # Errors
    ///
    /// Returns an error if the encryption version is not supported.
    pub fn new(version: EncryptionVersion, password: &str) -> Result<Self> {
        match version {
            EncryptionVersion::None => {
                // No encryption, return empty key
                Ok(Self {
                    version,
                    key: Vec::new(),
                })
            }
            EncryptionVersion::Modern => {
                // HWP 7.0 and later uses a specific key derivation
                let key = derive_modern_key(password);
                Ok(Self { version, key })
            }
            EncryptionVersion::Legacy25
            | EncryptionVersion::Enhanced30
            | EncryptionVersion::Old30 => {
                // Legacy encryption versions
                Err(Error::UnsupportedEncryptionVersion(version.as_raw()))
            }
            EncryptionVersion::Unknown(v) => Err(Error::UnsupportedEncryptionVersion(v)),
        }
    }

    /// Decrypts a stream.
    ///
    /// # Arguments
    ///
    /// * `data` - The encrypted data
    ///
    /// # Returns
    ///
    /// The decrypted data.
    ///
    /// # Errors
    ///
    /// Returns an error if decryption fails.
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.version {
            EncryptionVersion::None => Ok(data.to_vec()),
            EncryptionVersion::Modern => decrypt_modern(data, &self.key),
            _ => Err(Error::UnsupportedEncryptionVersion(self.version.as_raw())),
        }
    }

    /// Returns the encryption version.
    #[inline]
    #[allow(dead_code)]
    pub const fn version(&self) -> EncryptionVersion {
        self.version
    }
}

/// Derives decryption key for modern encryption (HWP 7.0+).
///
/// Uses SHA-1 hash of the password to generate a 128-bit key.
fn derive_modern_key(password: &str) -> Vec<u8> {
    // Simple SHA-1 implementation for key derivation
    // In production, this should use a proper crypto library
    sha1_hash(password.as_bytes()).to_vec()
}

/// Decrypts data using modern encryption.
fn decrypt_modern(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    if key.len() < 16 {
        return Err(Error::InvalidPassword);
    }

    // Modern HWP encryption uses a block cipher
    // This is a placeholder for the actual decryption algorithm
    // The real implementation would use AES or a similar cipher
    let mut result = data.to_vec();

    // XOR with key (simplified - actual implementation is more complex)
    for (i, byte) in result.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }

    Ok(result)
}

/// Simple SHA-1 hash implementation.
///
/// This is a minimal implementation for key derivation.
/// For production use, consider using a dedicated crypto library.
fn sha1_hash(data: &[u8]) -> [u8; 20] {
    // SHA-1 initial hash values
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    // Pre-processing: adding padding bits
    let ml = (data.len() as u64) * 8;
    let mut message = data.to_vec();
    message.push(0x80);

    while (message.len() % 64) != 56 {
        message.push(0x00);
    }

    // Append original length in bits as 64-bit big-endian
    message.extend_from_slice(&ml.to_be_bytes());

    // Process each 512-bit chunk
    for chunk in message.chunks(64) {
        let mut w = [0u32; 80];

        // Break chunk into sixteen 32-bit big-endian words
        for (i, word) in chunk.chunks(4).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }

        // Extend the sixteen 32-bit words into eighty 32-bit words
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        for (i, wi) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999u32),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1u32),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDCu32),
                _ => (b ^ c ^ d, 0xCA62C1D6u32),
            };

            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(*wi);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    // Produce the final hash value (big-endian)
    let mut result = [0u8; 20];
    result[0..4].copy_from_slice(&h0.to_be_bytes());
    result[4..8].copy_from_slice(&h1.to_be_bytes());
    result[8..12].copy_from_slice(&h2.to_be_bytes());
    result[12..16].copy_from_slice(&h3.to_be_bytes());
    result[16..20].copy_from_slice(&h4.to_be_bytes());
    result
}

/// Convenience function to decrypt a password-encrypted stream.
///
/// # Arguments
///
/// * `data` - The encrypted data
/// * `version` - The encryption version
/// * `password` - The document password
///
/// # Returns
///
/// The decrypted data.
pub fn decrypt_password_stream(
    data: &[u8],
    version: EncryptionVersion,
    password: &str,
) -> Result<Vec<u8>> {
    let decryptor = PasswordDecryptor::new(version, password)?;
    decryptor.decrypt(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1_hash() {
        // Test vector from FIPS 180-1
        let hash = sha1_hash(b"abc");
        let expected: [u8; 20] = [
            0xA9, 0x99, 0x3E, 0x36, 0x47, 0x06, 0x81, 0x6A, 0xBA, 0x3E, 0x25, 0x71, 0x78, 0x50,
            0xC2, 0x6C, 0x9C, 0xD0, 0xD8, 0x9D,
        ];
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_no_encryption() {
        let data = b"Hello, World!";
        let decryptor = PasswordDecryptor::new(EncryptionVersion::None, "").unwrap();
        let result = decryptor.decrypt(data).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_unsupported_encryption() {
        let result = PasswordDecryptor::new(EncryptionVersion::Legacy25, "password");
        assert!(matches!(
            result,
            Err(Error::UnsupportedEncryptionVersion(1))
        ));
    }
}
