//! FileHeader 스트림 생성기
//!
//! HWP 5.0 FileHeader는 256바이트 고정 크기의 스트림입니다.

use crate::header::{FILE_HEADER_SIZE, HWP_SIGNATURE};
use primitive::Version;

/// FileHeader 스트림 생성기
pub struct FileHeaderWriter {
    /// 문서 버전
    version: Version,
    /// 압축 여부
    compressed: bool,
    /// 암호화 여부
    encrypted: bool,
    /// 배포용 문서 여부
    distribution: bool,
    /// 스크립트 포함 여부
    has_script: bool,
    /// DRM 보호 여부
    drm_protected: bool,
    /// XML 템플릿 존재 여부
    has_xml_template: bool,
    /// 문서 이력 존재 여부
    has_document_history: bool,
    /// 전자 서명 존재 여부
    has_digital_signature: bool,
    /// 인증서 암호화 여부
    certificate_encrypted: bool,
    /// 전자 서명 예약 여부
    signature_reserve: bool,
    /// 인증서 DRM 여부
    certificate_drm: bool,
    /// CCL 문서 여부
    ccl_document: bool,
    /// 모바일 최적화 여부
    mobile_optimized: bool,
    /// 개인정보 보호 여부
    privacy_protected: bool,
    /// 변경 추적 여부
    track_changes: bool,
    /// KOGL 문서 여부
    kogl_document: bool,
    /// 비디오 컨트롤 포함 여부
    has_video_control: bool,
    /// 목차 필드 포함 여부
    has_toc_field: bool,
    /// 암호화 버전
    encryption_version: u32,
    /// KOGL 국가 코드
    kogl_country: u8,
}

impl FileHeaderWriter {
    /// 새 FileHeaderWriter를 생성합니다.
    pub const fn new(version: Version, compressed: bool) -> Self {
        Self {
            version,
            compressed,
            encrypted: false,
            distribution: false,
            has_script: false,
            drm_protected: false,
            has_xml_template: false,
            has_document_history: false,
            has_digital_signature: false,
            certificate_encrypted: false,
            signature_reserve: false,
            certificate_drm: false,
            ccl_document: false,
            mobile_optimized: false,
            privacy_protected: false,
            track_changes: false,
            kogl_document: false,
            has_video_control: false,
            has_toc_field: false,
            encryption_version: 0,
            kogl_country: 6, // Korea
        }
    }

    /// 배포용 문서로 설정합니다.
    pub const fn with_distribution(mut self, distribution: bool) -> Self {
        self.distribution = distribution;
        self
    }

    /// 스크립트 포함 여부를 설정합니다.
    pub const fn with_script(mut self, has_script: bool) -> Self {
        self.has_script = has_script;
        self
    }

    /// 문서 이력 존재 여부를 설정합니다.
    pub const fn with_document_history(mut self, has_history: bool) -> Self {
        self.has_document_history = has_history;
        self
    }

    /// FileHeader 데이터를 생성합니다.
    pub fn build(&self) -> Vec<u8> {
        let mut data = vec![0u8; FILE_HEADER_SIZE];

        // 시그니처 (32 바이트)
        data[..32].copy_from_slice(HWP_SIGNATURE);

        // 버전 (4 바이트, offset 32)
        let version_bytes = self.version.to_le_bytes();
        data[32..36].copy_from_slice(&version_bytes);

        // 파일 속성 (4 바이트, offset 36)
        let properties = self.build_properties();
        data[36..40].copy_from_slice(&properties.to_le_bytes());

        // 라이선스 정보 (4 바이트, offset 40)
        let license_info: u32 = 0;
        data[40..44].copy_from_slice(&license_info.to_le_bytes());

        // 암호화 버전 (4 바이트, offset 44)
        data[44..48].copy_from_slice(&self.encryption_version.to_le_bytes());

        // KOGL 국가 코드 (1 바이트, offset 48)
        data[48] = self.kogl_country;

        // 나머지는 예약 영역 (0으로 이미 초기화됨)

        data
    }

    const fn build_properties(&self) -> u32 {
        let mut props = 0;

        props |= (self.compressed as u32) << 0;
        props |= (self.encrypted as u32) << 1;
        props |= (self.distribution as u32) << 2;
        props |= (self.has_script as u32) << 3;
        props |= (self.drm_protected as u32) << 4;
        props |= (self.has_xml_template as u32) << 5;
        props |= (self.has_document_history as u32) << 6;
        props |= (self.has_digital_signature as u32) << 7;
        props |= (self.certificate_encrypted as u32) << 8;
        props |= (self.signature_reserve as u32) << 9;
        props |= (self.certificate_drm as u32) << 10;
        props |= (self.ccl_document as u32) << 11;
        props |= (self.mobile_optimized as u32) << 12;
        props |= (self.privacy_protected as u32) << 13;
        props |= (self.track_changes as u32) << 14;
        props |= (self.kogl_document as u32) << 15;
        props |= (self.has_video_control as u32) << 16;
        props |= (self.has_toc_field as u32) << 17;

        props
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_header_size() {
        let writer = FileHeaderWriter::new(Version::V5_0_3_0, true);
        let data = writer.build();
        assert_eq!(data.len(), FILE_HEADER_SIZE);
    }

    #[test]
    fn test_file_header_signature() {
        let writer = FileHeaderWriter::new(Version::V5_0_3_0, true);
        let data = writer.build();
        assert_eq!(&data[..32], HWP_SIGNATURE);
    }

    #[test]
    fn test_file_header_version() {
        let writer = FileHeaderWriter::new(Version::V5_0_3_0, true);
        let data = writer.build();
        let version = u32::from_le_bytes([data[32], data[33], data[34], data[35]]);
        assert_eq!(Version::from_raw(version), Version::V5_0_3_0);
    }

    #[test]
    fn test_file_header_compressed() {
        let writer = FileHeaderWriter::new(Version::V5_0_3_0, true);
        let data = writer.build();
        let properties = u32::from_le_bytes([data[36], data[37], data[38], data[39]]);
        assert!(properties & 1 != 0); // compressed bit
    }

    #[test]
    fn test_file_header_uncompressed() {
        let writer = FileHeaderWriter::new(Version::V5_0_3_0, false);
        let data = writer.build();
        let properties = u32::from_le_bytes([data[36], data[37], data[38], data[39]]);
        assert_eq!(properties & 1, 0); // compressed bit not set
    }
}
