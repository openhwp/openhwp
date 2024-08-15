use crate::{HwpDocumentError, HwpRead};

#[derive(Debug)]
pub struct FileHeader {
    /// 파일 버전 정보
    pub version: Version,
    /// 파일 속성 정보
    pub properties: Properties,
    /// 라이선스 정보
    pub license: License,
    /// 암호화 버전 정보
    pub encrypted_version: EncryptedVersion,
    /// 공공누리(KOGL) 라이선스 지원 국가 정보
    pub kogl_license_support_country: KoglLicenseSupportCountry,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub build: u8,
    pub revision: u8,
}

#[derive(Debug, Clone)]
pub struct Properties {
    /// 압축 여부
    pub compressed: Compressed,
    /// 암호 설정 여부
    pub encrypted: bool,
    /// 배포용 문서 여부
    pub distribution: bool,
    /// 스크립트 저장 여부
    pub script: bool,
    /// DRM 보안 문서 여부
    pub drm: bool,
    /// XMLTemplate 스토리지 존재 여부
    pub has_xml_template_storage: bool,
    /// 문서 이력 관리 존재 여부
    pub vcs: bool,
    /// 전자 서명 정보 존재 여부
    pub has_electronic_signature_information: bool,
    /// 공인 인증서 암호화 여부
    pub certificate_encryption: bool,
    /// 전자 서명 예비 저장 여부
    pub prepare_signature: bool,
    /// 공인 인증서 DRM 보안 문서 여부
    pub certificate_drm: bool,
    /// CCL 문서 여부
    pub ccl: bool,
    /// 모바일 최적화 여부
    pub mobile: bool,
    /// 개인 정보 보안 문서 여부
    pub is_privacy_security_document: bool,
    /// 변경 추적 문서 여부
    pub track_changes: bool,
    /// 공공누리(KOGL) 저작권 문서
    pub kogl: bool,
    /// 비디오 컨트롤 포함 여부
    pub has_video_control: bool,
    /// 차례 필드 컨트롤 포함 여부
    pub has_order_field_control: bool,
}

#[derive(Debug, Clone)]
pub struct License {
    /// CCL, 공공누리 라이선스 정보
    pub ccl: bool,
    /// 복제 제한 여부
    pub copy_limit: bool,
    /// 동일 조건 하에 복제 허가 여부 (복제 제한인 경우 무시)
    pub copy_same: bool,
}

#[derive(Debug, Clone)]
pub enum EncryptedVersion {
    /// None
    None = 0,
    /// (한/글 2.5 버전 이하)
    LessThan2_5 = 1,
    /// (한/글 3.0 버전 Enhanced)
    Enhanced3_0 = 2,
    /// (한/글 3.0 버전 Old)
    Old3_0 = 3,
    /// (한/글 7.0 버전 이후)
    GreaterThan7_0 = 4,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum KoglLicenseSupportCountry {
    NONE = 0,
    KOR = 6,
    US = 15,
}

#[derive(Debug, Clone, Copy)]
pub enum Compressed {
    No,
    Yes,
}

#[derive(Debug, Error)]
pub enum FileHeaderError {
    #[error("FileHeader must be 256 bytes, Received: {0}")]
    InvalidSize(usize),
    #[error("Invalid signature. Received version: {0:?}")]
    InvalidSignature(Vec<u8>),
    #[error("Only support 5.1.0.0 format. Received version: {0:?}")]
    UnsupportedVersion(Version),
    #[error("Unknown encrypted version: {0}")]
    UnknownEncryptedVersion(u8),
    #[error("Unknown Kogl license support country: {0}")]
    UnknownKoglLicenseSupportCountry(u8),
}

impl FileHeader {
    pub fn from_reader<R: HwpRead>(reader: &mut R) -> Result<Self, HwpDocumentError> {
        Ok(Self::from_vec(reader.header()?)?)
    }

    pub fn from_vec(buf: Vec<u8>) -> Result<Self, FileHeaderError> {
        let buf = match <[u8; 256]>::try_from(buf.as_slice()) {
            Ok(buf) if !buf.starts_with(b"HWP Document File") => {
                return Err(FileHeaderError::InvalidSignature(buf[0..32].to_vec()))
            }
            Ok(buf) => buf,
            Err(_) => return Err(FileHeaderError::InvalidSize(buf.len())),
        };

        let version = Version {
            major: buf[35],
            minor: buf[34],
            build: buf[33],
            revision: buf[32],
        };
        if version < Version::V5_0_1_7 {
            return Err(FileHeaderError::UnsupportedVersion(version));
        }

        let properties = Properties {
            compressed: match buf[36] & 0b0000_0001 != 0 {
                true => Compressed::Yes,
                false => Compressed::No,
            },
            encrypted: buf[36] & 0b0000_0010 != 0,
            distribution: buf[36] & 0b0000_0100 != 0,
            script: buf[36] & 0b0000_1000 != 0,
            drm: buf[36] & 0b0001_0000 != 0,
            has_xml_template_storage: buf[36] & 0b0010_0000 != 0,
            vcs: buf[36] & 0b0100_0000 != 0,
            has_electronic_signature_information: buf[36] & 0b1000_0000 != 0,
            certificate_encryption: buf[37] & 0b0000_0001 != 0,
            prepare_signature: buf[37] & 0b0000_0010 != 0,
            certificate_drm: buf[37] & 0b0000_0100 != 0,
            ccl: buf[37] & 0b0000_1000 != 0,
            mobile: buf[37] & 0b0001_0000 != 0,
            is_privacy_security_document: buf[37] & 0b0010_0000 != 0,
            track_changes: buf[37] & 0b0100_0000 != 0,
            kogl: buf[37] & 0b1000_0000 != 0,
            has_video_control: buf[38] & 0b0000_0001 != 0,
            has_order_field_control: buf[38] & 0b0000_0010 != 0,
        };
        let license = License {
            ccl: buf[40] & 0b0000_0001 != 0,
            copy_limit: buf[40] & 0b0000_0010 != 0,
            copy_same: buf[40] & 0b0000_01000 != 0,
        };
        let encrypted_version = match buf[44] {
            0 => EncryptedVersion::None,
            1 => EncryptedVersion::LessThan2_5,
            2 => EncryptedVersion::Enhanced3_0,
            3 => EncryptedVersion::Old3_0,
            4 => EncryptedVersion::GreaterThan7_0,
            byte => return Err(FileHeaderError::UnknownEncryptedVersion(byte)),
        };
        let kogl_license_support_country = match buf[48] {
            0 => KoglLicenseSupportCountry::NONE,
            6 => KoglLicenseSupportCountry::KOR,
            15 => KoglLicenseSupportCountry::US,
            byte => return Err(FileHeaderError::UnknownKoglLicenseSupportCountry(byte)),
        };

        Ok(Self {
            version,
            properties,
            license,
            encrypted_version,
            kogl_license_support_country,
        })
    }
}

impl Version {
    pub const V5_1_0_0: Self = Self {
        major: 5,
        minor: 1,
        build: 0,
        revision: 0,
    };
    pub const V5_0_1_7: Self = Self {
        major: 5,
        minor: 0,
        build: 1,
        revision: 7,
    };

    pub const V5_0_2_1: Self = Self {
        major: 5,
        minor: 0,
        build: 2,
        revision: 1,
    };

    pub const V5_0_2_5: Self = Self {
        major: 5,
        minor: 0,
        build: 2,
        revision: 5,
    };

    pub const V5_0_3_2: Self = Self {
        major: 5,
        minor: 0,
        build: 3,
        revision: 2,
    };

    #[inline]
    pub const fn new(major: u8, minor: u8, build: u8, revision: u8) -> Self {
        Self {
            major,
            minor,
            build,
            revision,
        }
    }
}
