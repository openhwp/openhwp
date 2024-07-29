#[derive(Debug)]
pub struct FileHeader {
    /// 파일 버전 정보
    pub file_version: FileVersion,
    /// 파일 속성 정보
    pub properties: Properties,
    /// 라이선스 정보
    pub license: License,
    /// 암호화 버전 정보
    pub encrypted_version: EncryptedVersion,
    /// 공공누리(KOGL) 라이선스 지원 국가 정보
    pub kogl_license_support_country: KoglLicenseSupportCountry,
}

#[derive(Debug, Clone)]
pub struct FileVersion {
    pub major: u8,
    pub minor: u8,
    pub build: u8,
    pub revision: u8,
}

#[derive(Debug, Clone)]
pub struct Properties {
    /// 압축 여부
    pub compressed: bool,
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
#[derive(Debug, Clone)]
pub enum KoglLicenseSupportCountry {
    NONE = 0,
    KOR = 6,
    US = 15,
}

#[derive(Debug, Error)]
pub enum FileHeaderError {
    #[error("FileHeader must be 256 bytes, Received: {0}")]
    InvalidSize(usize),
    #[error("Invalid signature. Received version: {0:?}")]
    InvalidSignature([u8; 32]),
    #[error("Only support 5.1.0.0 format. Received version: {0:?}")]
    UnsupportedVersion(FileVersion),
    #[error("Unknown encrypted version: {0}")]
    UnknownEncryptedVersion(u8),
    #[error("Unknown Kogl license support country: {0}")]
    UnknownKoglLicenseSupportCountry(u8),
}

impl FileHeader {
    pub fn from_vec(bytes: Vec<u8>) -> Result<Self, FileHeaderError> {
        let bytes = match <[u8; 256]>::try_from(bytes.as_slice()) {
            Ok(bytes) if !bytes.starts_with(b"HWP Document File") => {
                return Err(FileHeaderError::InvalidSignature(
                    bytes[0..32].try_into().unwrap(),
                ))
            }
            Ok(bytes) => bytes,
            Err(_) => return Err(FileHeaderError::InvalidSize(bytes.len())),
        };

        let file_version = FileVersion {
            major: bytes[35],
            minor: bytes[34],
            build: bytes[33],
            revision: bytes[32],
        };
        if !file_version.is_compatible() {
            return Err(FileHeaderError::UnsupportedVersion(file_version));
        };

        let properties = Properties {
            compressed: bytes[36] & 0x01 != 0,
            encrypted: bytes[36] & 0x02 != 0,
            distribution: bytes[36] & 0x04 != 0,
            script: bytes[36] & 0x08 != 0,
            drm: bytes[36] & 0x10 != 0,
            has_xml_template_storage: bytes[36] & 0x20 != 0,
            vcs: bytes[36] & 0x40 != 0,
            has_electronic_signature_information: bytes[36] & 0x80 != 0,
            certificate_encryption: bytes[37] & 0x01 != 0,
            prepare_signature: bytes[37] & 0x02 != 0,
            certificate_drm: bytes[37] & 0x04 != 0,
            ccl: bytes[37] & 0x08 != 0,
            mobile: bytes[37] & 0x010 != 0,
            is_privacy_security_document: bytes[37] & 0x020 != 0,
            track_changes: bytes[37] & 0x040 != 0,
            kogl: bytes[37] & 0x080 != 0,
            has_video_control: bytes[38] & 0x01 != 0,
            has_order_field_control: bytes[38] & 0x02 != 0,
        };
        let license = License {
            ccl: bytes[40] & 0x01 != 0,
            copy_limit: bytes[40] & 0x02 != 0,
            copy_same: bytes[40] & 0x04 != 0,
        };
        let encrypted_version = match bytes[44] {
            0 => EncryptedVersion::None,
            1 => EncryptedVersion::LessThan2_5,
            2 => EncryptedVersion::Enhanced3_0,
            3 => EncryptedVersion::Old3_0,
            4 => EncryptedVersion::GreaterThan7_0,
            byte => return Err(FileHeaderError::UnknownEncryptedVersion(byte)),
        };
        let kogl_license_support_country = match bytes[48] {
            0 => KoglLicenseSupportCountry::NONE,
            6 => KoglLicenseSupportCountry::KOR,
            15 => KoglLicenseSupportCountry::US,
            byte => return Err(FileHeaderError::UnknownKoglLicenseSupportCountry(byte)),
        };

        Ok(Self {
            file_version,
            properties,
            license,
            encrypted_version,
            kogl_license_support_country,
        })
    }
}

impl FileVersion {
    pub const COMPATIBLE: Self = Self {
        major: 5,
        minor: 1,
        build: 0,
        revision: 0,
    };

    pub const fn is_compatible(&self) -> bool {
        self.major == Self::COMPATIBLE.major && self.minor <= Self::COMPATIBLE.minor
    }
}
