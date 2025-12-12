//! File properties and flags for HWP documents.

/// File properties extracted from FileHeader.
///
/// These flags indicate various document attributes such as
/// compression, encryption, and feature presence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FileProperties {
    raw: u32,
}

impl FileProperties {
    /// Creates new FileProperties from raw value.
    #[inline]
    pub const fn new(raw: u32) -> Self {
        Self { raw }
    }

    /// Returns the raw properties value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.raw
    }

    /// Whether the document is compressed.
    ///
    /// When true, DocInfo, BodyText, and DocHistory streams are zlib compressed.
    #[inline]
    pub const fn is_compressed(self) -> bool {
        (self.raw & (1 << 0)) != 0
    }

    /// Whether the document is password-encrypted.
    #[inline]
    pub const fn is_encrypted(self) -> bool {
        (self.raw & (1 << 1)) != 0
    }

    /// Whether this is a distribution document.
    ///
    /// Distribution documents have restricted editing and use ViewText instead of BodyText.
    #[inline]
    pub const fn is_distribution(self) -> bool {
        (self.raw & (1 << 2)) != 0
    }

    /// Whether scripts are stored in the document.
    #[inline]
    pub const fn has_script(self) -> bool {
        (self.raw & (1 << 3)) != 0
    }

    /// Whether this is a DRM-protected document.
    #[inline]
    pub const fn is_drm_protected(self) -> bool {
        (self.raw & (1 << 4)) != 0
    }

    /// Whether the XMLTemplate storage exists.
    #[inline]
    pub const fn has_xml_template(self) -> bool {
        (self.raw & (1 << 5)) != 0
    }

    /// Whether document history exists.
    #[inline]
    pub const fn has_document_history(self) -> bool {
        (self.raw & (1 << 6)) != 0
    }

    /// Whether digital signature information exists.
    #[inline]
    pub const fn has_digital_signature(self) -> bool {
        (self.raw & (1 << 7)) != 0
    }

    /// Whether the document is encrypted with a public certificate.
    #[inline]
    pub const fn is_certificate_encrypted(self) -> bool {
        (self.raw & (1 << 8)) != 0
    }

    /// Whether digital signature is stored for later.
    #[inline]
    pub const fn has_signature_reserve(self) -> bool {
        (self.raw & (1 << 9)) != 0
    }

    /// Whether the document is a certificate DRM document.
    #[inline]
    pub const fn is_certificate_drm(self) -> bool {
        (self.raw & (1 << 10)) != 0
    }

    /// Whether this is a CCL (Creative Commons License) document.
    #[inline]
    pub const fn is_ccl_document(self) -> bool {
        (self.raw & (1 << 11)) != 0
    }

    /// Whether the document is optimized for mobile.
    #[inline]
    pub const fn is_mobile_optimized(self) -> bool {
        (self.raw & (1 << 12)) != 0
    }

    /// Whether this is a personal information protected document.
    #[inline]
    pub const fn is_privacy_protected(self) -> bool {
        (self.raw & (1 << 13)) != 0
    }

    /// Whether the document has track changes enabled.
    #[inline]
    pub const fn has_track_changes(self) -> bool {
        (self.raw & (1 << 14)) != 0
    }

    /// Whether this is a KOGL (Korea Open Government License) document.
    #[inline]
    pub const fn is_kogl_document(self) -> bool {
        (self.raw & (1 << 15)) != 0
    }

    /// Whether the document contains video controls.
    #[inline]
    pub const fn has_video_control(self) -> bool {
        (self.raw & (1 << 16)) != 0
    }

    /// Whether the document contains table of contents field controls.
    #[inline]
    pub const fn has_toc_field(self) -> bool {
        (self.raw & (1 << 17)) != 0
    }
}

/// License information properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LicenseInfo {
    raw: u32,
}

impl LicenseInfo {
    /// Creates new LicenseInfo from raw value.
    #[inline]
    pub const fn new(raw: u32) -> Self {
        Self { raw }
    }

    /// Returns the raw license info value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.raw
    }

    /// Whether CCL/KOGL license info exists.
    #[inline]
    pub const fn has_license_info(self) -> bool {
        (self.raw & (1 << 0)) != 0
    }

    /// Whether copying is restricted.
    #[inline]
    pub const fn is_copy_restricted(self) -> bool {
        (self.raw & (1 << 1)) != 0
    }

    /// Whether copying is allowed under same conditions.
    ///
    /// Only meaningful when copying is not restricted.
    #[inline]
    pub const fn allows_same_condition_copy(self) -> bool {
        (self.raw & (1 << 2)) != 0
    }
}

/// Encryption version used for password-protected documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionVersion {
    /// No encryption.
    None,
    /// Hangul 2.5 or earlier.
    Legacy25,
    /// Hangul 3.0 Enhanced.
    Enhanced30,
    /// Hangul 3.0 Old.
    Old30,
    /// Hangul 7.0 and later.
    Modern,
    /// Unknown encryption version.
    Unknown(u32),
}

impl EncryptionVersion {
    /// Creates an EncryptionVersion from raw value.
    pub const fn from_raw(value: u32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Legacy25,
            2 => Self::Enhanced30,
            3 => Self::Old30,
            4 => Self::Modern,
            _ => Self::Unknown(value),
        }
    }

    /// Returns the raw encryption version value.
    pub const fn as_raw(self) -> u32 {
        match self {
            Self::None => 0,
            Self::Legacy25 => 1,
            Self::Enhanced30 => 2,
            Self::Old30 => 3,
            Self::Modern => 4,
            Self::Unknown(v) => v,
        }
    }

    /// Returns true if the document is encrypted.
    #[inline]
    pub const fn is_encrypted(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// KOGL (Korea Open Government License) country code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KoglCountry {
    /// Korea (6).
    Korea,
    /// United States (15).
    UnitedStates,
    /// Unknown country code.
    Unknown(u8),
}

impl KoglCountry {
    /// Creates a KoglCountry from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            6 => Self::Korea,
            15 => Self::UnitedStates,
            _ => Self::Unknown(value),
        }
    }

    /// Returns the raw country code.
    pub const fn as_raw(self) -> u8 {
        match self {
            Self::Korea => 6,
            Self::UnitedStates => 15,
            Self::Unknown(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_properties_compressed() {
        let props = FileProperties::new(0b0000_0001);
        assert!(props.is_compressed());
        assert!(!props.is_encrypted());
    }

    #[test]
    fn test_file_properties_encrypted() {
        let props = FileProperties::new(0b0000_0011);
        assert!(props.is_compressed());
        assert!(props.is_encrypted());
    }

    #[test]
    fn test_file_properties_distribution() {
        let props = FileProperties::new(0b0000_0100);
        assert!(props.is_distribution());
    }

    #[test]
    fn test_encryption_version() {
        assert_eq!(EncryptionVersion::from_raw(0), EncryptionVersion::None);
        assert_eq!(EncryptionVersion::from_raw(4), EncryptionVersion::Modern);
        assert!(!EncryptionVersion::None.is_encrypted());
        assert!(EncryptionVersion::Modern.is_encrypted());
    }

    #[test]
    fn test_kogl_country() {
        assert_eq!(KoglCountry::from_raw(6), KoglCountry::Korea);
        assert_eq!(KoglCountry::from_raw(15), KoglCountry::UnitedStates);
    }
}
