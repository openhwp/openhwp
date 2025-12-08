//! FileHeader stream parsing.
//!
//! The FileHeader is always 256 bytes and contains document identification
//! and properties.

use crate::error::{Error, Result};
use crate::primitive::Version;
use crate::util::ByteReader;

use super::properties::{EncryptionVersion, FileProperties, KoglCountry, LicenseInfo};

/// HWP document file signature.
///
/// All HWP 5.0 documents start with "HWP Document File" (null-padded to 32 bytes).
pub const HWP_SIGNATURE: &[u8; 32] = b"HWP Document File\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

/// FileHeader size in bytes.
pub const FILE_HEADER_SIZE: usize = 256;

/// FileHeader stream content.
///
/// Contains document identification, version, and properties.
///
/// # Structure
///
/// | Offset | Size | Description |
/// |--------|------|-------------|
/// | 0 | 32 | Signature ("HWP Document File") |
/// | 32 | 4 | Version (0xMMnnPPrr) |
/// | 36 | 4 | File properties |
/// | 40 | 4 | License info |
/// | 44 | 4 | Encryption version |
/// | 48 | 1 | KOGL country |
/// | 49 | 207 | Reserved |
#[derive(Debug, Clone)]
pub struct FileHeader {
    /// Document version.
    version: Version,
    /// File properties.
    properties: FileProperties,
    /// License information.
    license_info: LicenseInfo,
    /// Encryption version.
    encryption_version: EncryptionVersion,
    /// KOGL country code.
    kogl_country: KoglCountry,
}

impl FileHeader {
    /// Size of the FileHeader in bytes.
    pub const SIZE: usize = FILE_HEADER_SIZE;

    /// Parses a FileHeader from bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - Exactly 256 bytes of FileHeader data
    ///
    /// # Returns
    ///
    /// The parsed FileHeader.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Data is not exactly 256 bytes
    /// - Signature is invalid
    /// - Version is not HWP 5.x
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() != FILE_HEADER_SIZE {
            return Err(Error::UnexpectedEndOfData {
                expected: FILE_HEADER_SIZE,
                actual: data.len(),
            });
        }

        let mut reader = ByteReader::new(data);

        // Read and verify signature
        let signature = reader.read_array::<32>()?;
        if &signature != HWP_SIGNATURE {
            return Err(Error::InvalidSignature);
        }

        // Read version
        let version = Version::from_raw(reader.read_u32()?);
        if !version.is_hwp5() {
            return Err(Error::UnsupportedVersion {
                major: version.major(),
                minor: version.minor(),
                build: version.build(),
                revision: version.revision(),
            });
        }

        // Read properties
        let properties = FileProperties::new(reader.read_u32()?);
        let license_info = LicenseInfo::new(reader.read_u32()?);
        let encryption_version = EncryptionVersion::from_raw(reader.read_u32()?);
        let kogl_country = KoglCountry::from_raw(reader.read_u8()?);

        Ok(Self {
            version,
            properties,
            license_info,
            encryption_version,
            kogl_country,
        })
    }

    /// Returns the document version.
    #[inline]
    pub const fn version(&self) -> Version {
        self.version
    }

    /// Returns the file properties.
    #[inline]
    pub const fn properties(&self) -> FileProperties {
        self.properties
    }

    /// Returns the license information.
    #[inline]
    pub const fn license_info(&self) -> LicenseInfo {
        self.license_info
    }

    /// Returns the encryption version.
    #[inline]
    pub const fn encryption_version(&self) -> EncryptionVersion {
        self.encryption_version
    }

    /// Returns the KOGL country code.
    #[inline]
    pub const fn kogl_country(&self) -> KoglCountry {
        self.kogl_country
    }

    /// Returns true if the document is compressed.
    #[inline]
    pub const fn is_compressed(&self) -> bool {
        self.properties.is_compressed()
    }

    /// Returns true if the document is password-encrypted.
    #[inline]
    pub const fn is_encrypted(&self) -> bool {
        self.properties.is_encrypted()
    }

    /// Returns true if this is a distribution document.
    #[inline]
    pub const fn is_distribution(&self) -> bool {
        self.properties.is_distribution()
    }

    /// Returns true if the document has scripts.
    #[inline]
    pub const fn has_script(&self) -> bool {
        self.properties.has_script()
    }

    /// Returns true if the document has document history.
    #[inline]
    pub const fn has_document_history(&self) -> bool {
        self.properties.has_document_history()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_header(version: Version, properties: u32) -> [u8; FILE_HEADER_SIZE] {
        let mut data = [0u8; FILE_HEADER_SIZE];

        // Signature
        data[..32].copy_from_slice(HWP_SIGNATURE);

        // Version
        let version_bytes = version.to_le_bytes();
        data[32..36].copy_from_slice(&version_bytes);

        // Properties
        let props_bytes = properties.to_le_bytes();
        data[36..40].copy_from_slice(&props_bytes);

        data
    }

    #[test]
    fn test_parse_valid_header() {
        let data = create_test_header(Version::V5_0_3_0, 0b0000_0001);
        let header = FileHeader::from_bytes(&data).unwrap();

        assert_eq!(header.version(), Version::V5_0_3_0);
        assert!(header.is_compressed());
        assert!(!header.is_encrypted());
    }

    #[test]
    fn test_invalid_signature() {
        let mut data = [0u8; FILE_HEADER_SIZE];
        data[..16].copy_from_slice(b"Invalid Signatur");

        let result = FileHeader::from_bytes(&data);
        assert!(matches!(result, Err(Error::InvalidSignature)));
    }

    #[test]
    fn test_unsupported_version() {
        let mut data = [0u8; FILE_HEADER_SIZE];
        data[..32].copy_from_slice(HWP_SIGNATURE);
        // Version 4.0.0.0
        data[32..36].copy_from_slice(&[0, 0, 0, 4]);

        let result = FileHeader::from_bytes(&data);
        assert!(matches!(result, Err(Error::UnsupportedVersion { major: 4, .. })));
    }

    #[test]
    fn test_encrypted_header() {
        let data = create_test_header(Version::V5_0_3_0, 0b0000_0011);
        let header = FileHeader::from_bytes(&data).unwrap();

        assert!(header.is_compressed());
        assert!(header.is_encrypted());
    }

    #[test]
    fn test_distribution_header() {
        let data = create_test_header(Version::V5_0_3_0, 0b0000_0100);
        let header = FileHeader::from_bytes(&data).unwrap();

        assert!(header.is_distribution());
    }

    #[test]
    fn test_insufficient_data() {
        let data = [0u8; 100];
        let result = FileHeader::from_bytes(&data);
        assert!(matches!(
            result,
            Err(Error::UnexpectedEndOfData {
                expected: 256,
                actual: 100
            })
        ));
    }
}
