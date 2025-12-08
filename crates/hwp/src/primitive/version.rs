//! HWP document version representation.
//!
//! HWP version is stored as 0xMMnnPPrr where:
//! - MM: Major version (document format structure changes)
//! - nn: Minor version (significant changes)
//! - PP: Build version (record additions, backward compatible)
//! - rr: Revision (minor additions, backward compatible)

use std::fmt;

/// HWP document version.
///
/// The version number is stored as 0xMMnnPPrr format.
/// For example, version 5.0.3.0 would be stored as 0x05000300.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version(u32);

impl Version {
    /// HWP 5.0.0.0 - Minimum supported version.
    pub const V5_0_0_0: Self = Self::new(5, 0, 0, 0);

    /// HWP 5.0.1.0
    pub const V5_0_1_0: Self = Self::new(5, 0, 1, 0);

    /// HWP 5.0.1.7
    pub const V5_0_1_7: Self = Self::new(5, 0, 1, 7);

    /// HWP 5.0.2.1
    pub const V5_0_2_1: Self = Self::new(5, 0, 2, 1);

    /// HWP 5.0.2.5
    pub const V5_0_2_5: Self = Self::new(5, 0, 2, 5);

    /// HWP 5.0.3.0
    pub const V5_0_3_0: Self = Self::new(5, 0, 3, 0);

    /// HWP 5.0.3.2
    pub const V5_0_3_2: Self = Self::new(5, 0, 3, 2);

    /// HWP 5.1.0.0
    pub const V5_1_0_0: Self = Self::new(5, 1, 0, 0);

    /// Creates a new version from components.
    #[inline]
    pub const fn new(major: u8, minor: u8, build: u8, revision: u8) -> Self {
        Self(
            ((major as u32) << 24)
                | ((minor as u32) << 16)
                | ((build as u32) << 8)
                | (revision as u32),
        )
    }

    /// Creates a version from raw value.
    #[inline]
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw version value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Returns the major version number.
    ///
    /// Changes in major version indicate incompatible format changes.
    #[inline]
    pub const fn major(self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    /// Returns the minor version number.
    ///
    /// Changes in minor version indicate significant but structurally similar changes.
    #[inline]
    pub const fn minor(self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Returns the build version number.
    ///
    /// Changes in build version indicate record additions (backward compatible).
    #[inline]
    pub const fn build(self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Returns the revision number.
    ///
    /// Changes in revision indicate minor additions (backward compatible).
    #[inline]
    pub const fn revision(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Checks if this version is at least the given version.
    #[inline]
    pub const fn is_at_least(self, other: Self) -> bool {
        self.0 >= other.0
    }

    /// Checks if this version is an HWP 5.x version.
    #[inline]
    pub const fn is_hwp5(self) -> bool {
        self.major() == 5
    }

    /// Reads from little-endian bytes.
    #[inline]
    pub fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(bytes))
    }

    /// Converts to little-endian bytes.
    #[inline]
    pub fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Version")
            .field("major", &self.major())
            .field("minor", &self.minor())
            .field("build", &self.build())
            .field("revision", &self.revision())
            .finish()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major(),
            self.minor(),
            self.build(),
            self.revision()
        )
    }
}

impl PartialOrd for Version {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::V5_0_0_0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_components() {
        let version = Version::new(5, 0, 3, 2);
        assert_eq!(version.major(), 5);
        assert_eq!(version.minor(), 0);
        assert_eq!(version.build(), 3);
        assert_eq!(version.revision(), 2);
    }

    #[test]
    fn test_version_raw() {
        let version = Version::new(5, 0, 3, 0);
        assert_eq!(version.raw(), 0x05000300);
    }

    #[test]
    fn test_version_display() {
        let version = Version::new(5, 0, 3, 2);
        assert_eq!(version.to_string(), "5.0.3.2");
    }

    #[test]
    fn test_version_comparison() {
        assert!(Version::V5_0_3_2 > Version::V5_0_3_0);
        assert!(Version::V5_0_3_0 > Version::V5_0_2_5);
        assert!(Version::V5_1_0_0 > Version::V5_0_3_2);
    }

    #[test]
    fn test_version_is_at_least() {
        let v = Version::new(5, 0, 2, 5);
        assert!(v.is_at_least(Version::V5_0_2_1));
        assert!(v.is_at_least(Version::V5_0_2_5));
        assert!(!v.is_at_least(Version::V5_0_3_0));
    }

    #[test]
    fn test_version_le_bytes() {
        let version = Version::new(5, 0, 3, 0);
        let bytes = version.to_le_bytes();
        assert_eq!(bytes, [0x00, 0x03, 0x00, 0x05]);
        assert_eq!(Version::from_le_bytes(bytes), version);
    }
}
