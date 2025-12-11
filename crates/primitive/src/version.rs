//! HWP 문서 버전
//!
//! HWP 버전은 0xMMnnPPrr 형식으로 저장됩니다:
//! - MM: 주 버전 (문서 형식 구조 변경)
//! - nn: 부 버전 (중요 변경)
//! - PP: 빌드 버전 (레코드 추가, 하위 호환)
//! - rr: 리비전 (사소한 추가, 하위 호환)

use std::fmt;

/// HWP 문서 버전
///
/// 버전 번호는 0xMMnnPPrr 형식으로 저장됩니다.
/// 예: 버전 5.0.3.0은 0x05000300으로 저장됩니다.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version(u32);

impl Version {
    /// HWP 5.0.0.0 - 최소 지원 버전
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

    /// 컴포넌트로 새 버전 생성
    #[inline]
    pub const fn new(major: u8, minor: u8, build: u8, revision: u8) -> Self {
        Self(
            ((major as u32) << 24)
                | ((minor as u32) << 16)
                | ((build as u32) << 8)
                | (revision as u32),
        )
    }

    /// raw 값으로 버전 생성
    #[inline]
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }

    /// raw 버전 값 반환
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// 주 버전 번호 반환
    ///
    /// 주 버전 변경은 호환되지 않는 형식 변경을 나타냅니다.
    #[inline]
    pub const fn major(self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    /// 부 버전 번호 반환
    ///
    /// 부 버전 변경은 구조적으로 유사하지만 중요한 변경을 나타냅니다.
    #[inline]
    pub const fn minor(self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// 빌드 버전 번호 반환
    ///
    /// 빌드 버전 변경은 레코드 추가 (하위 호환)를 나타냅니다.
    #[inline]
    pub const fn build(self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// 리비전 번호 반환
    ///
    /// 리비전 변경은 사소한 추가 (하위 호환)를 나타냅니다.
    #[inline]
    pub const fn revision(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// 이 버전이 주어진 버전 이상인지 확인
    #[inline]
    pub const fn is_at_least(self, other: Self) -> bool {
        self.0 >= other.0
    }

    /// HWP 5.x 버전인지 확인
    #[inline]
    pub const fn is_hwp5(self) -> bool {
        self.major() == 5
    }

    /// 리틀 엔디안 바이트에서 읽기
    #[inline]
    pub fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(bytes))
    }

    /// 리틀 엔디안 바이트로 변환
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
