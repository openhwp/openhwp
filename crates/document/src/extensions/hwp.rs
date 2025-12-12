//! HWP 5.0 형식 확장
//!
//! HWP 5.0 바이너리 형식에 특화된 데이터를 관리합니다.

/// HWP 확장 데이터
#[derive(Debug, Clone, Default)]
pub struct HwpExtension {
    /// 파일 헤더 정보
    pub file_header: Option<FileHeader>,
    /// 요약 정보
    pub summary_info: Option<SummaryInfo>,
    /// 스크립트
    pub scripts: Option<Scripts>,
    /// 배포용 문서 데이터
    pub distribute_doc_data: Option<DistributeDocData>,
    /// 변경 추적 정보
    pub track_change_info: Option<TrackChangeInfo>,
    /// 미리보기 텍스트
    pub preview_text: Option<String>,
    /// 미리보기 이미지 (PNG)
    pub preview_image: Option<Vec<u8>>,
}

impl HwpExtension {
    /// 새 HWP 확장 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 암호화 여부
    pub fn is_encrypted(&self) -> bool {
        self.file_header
            .as_ref()
            .map(|h| h.flags.encrypted)
            .unwrap_or(false)
    }

    /// 배포용 문서 여부
    pub fn is_distributed(&self) -> bool {
        self.file_header
            .as_ref()
            .map(|h| h.flags.distributed)
            .unwrap_or(false)
    }

    /// 스크립트 포함 여부
    pub fn has_scripts(&self) -> bool {
        self.file_header
            .as_ref()
            .map(|h| h.flags.has_script)
            .unwrap_or(false)
    }
}

/// 파일 헤더 정보
#[derive(Debug, Clone, Default)]
pub struct FileHeader {
    /// 파일 버전
    pub version: HwpVersion,
    /// 플래그
    pub flags: FileHeaderFlags,
    /// 라이선스
    pub license: Option<String>,
    /// 암호화 버전
    pub encryption_version: Option<u32>,
    /// KO 글자 인코딩 타입
    pub kogl_ccl_type: Option<u32>,
}

/// HWP 버전
#[derive(Debug, Clone, Default)]
pub struct HwpVersion {
    /// 메이저 버전
    pub major: u8,
    /// 마이너 버전
    pub minor: u8,
    /// 빌드 번호
    pub build: u16,
    /// 리비전
    pub revision: u8,
}

impl HwpVersion {
    /// 5.0.0.0 이상인지 확인
    pub fn is_5_0_or_later(&self) -> bool {
        self.major >= 5
    }

    /// 5.1.0.0 이상인지 확인 (HWPX 지원 시작)
    pub fn is_5_1_or_later(&self) -> bool {
        self.major > 5 || (self.major == 5 && self.minor >= 1)
    }
}

impl std::fmt::Display for HwpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.major, self.minor, self.build, self.revision)
    }
}

/// 파일 헤더 플래그
#[derive(Debug, Clone, Default)]
pub struct FileHeaderFlags {
    /// 압축 여부
    pub compressed: bool,
    /// 암호화 여부
    pub encrypted: bool,
    /// 배포용 문서 여부
    pub distributed: bool,
    /// 스크립트 저장 여부
    pub has_script: bool,
    /// DRM 보안 여부
    pub drm: bool,
    /// XML 템플릿 저장 여부
    pub xml_template: bool,
    /// 문서 이력 관리 여부
    pub document_history: bool,
    /// 전자 서명 정보 여부
    pub digital_signature: bool,
    /// 공인 인증서 암호화 여부
    pub certificate_encryption: bool,
    /// 전자 서명 예비 저장 여부
    pub signature_reserve: bool,
    /// 공인 인증서 DRM 여부
    pub certificate_drm: bool,
    /// CCL 문서 여부
    pub ccl: bool,
    /// 모바일 최적화 여부
    pub mobile_optimized: bool,
    /// 개인 정보 보호 여부
    pub privacy_security: bool,
    /// 변경 추적 여부
    pub track_changes: bool,
    /// KOGL 저작권 여부
    pub kogl: bool,
    /// 비디오 컨트롤 포함 여부
    pub has_video: bool,
    /// 순서 정보 필드 순서 방식
    pub order_field_order: bool,
}

/// 요약 정보
#[derive(Debug, Clone, Default)]
pub struct SummaryInfo {
    /// 제목
    pub title: Option<String>,
    /// 주제
    pub subject: Option<String>,
    /// 저자
    pub author: Option<String>,
    /// 날짜
    pub date: Option<String>,
    /// 키워드
    pub keywords: Option<String>,
    /// 설명
    pub comments: Option<String>,
    /// 마지막 저장한 사용자
    pub last_saved_by: Option<String>,
    /// 리비전 번호
    pub revision_number: Option<String>,
    /// 편집 시간 (분)
    pub edit_time: Option<u32>,
    /// 마지막 인쇄 날짜
    pub last_printed: Option<String>,
    /// 생성 날짜
    pub create_date: Option<String>,
    /// 마지막 저장 날짜
    pub save_date: Option<String>,
    /// 페이지 수
    pub page_count: Option<u32>,
    /// 단어 수
    pub word_count: Option<u32>,
    /// 문자 수
    pub char_count: Option<u32>,
}

/// 스크립트 정보
#[derive(Debug, Clone, Default)]
pub struct Scripts {
    /// 기본 스크립트 버전
    pub default_version: Option<String>,
    /// 스크립트 소스 목록
    pub sources: Vec<ScriptSource>,
}

/// 스크립트 소스
#[derive(Debug, Clone)]
pub struct ScriptSource {
    /// 스크립트 타입
    pub script_type: ScriptType,
    /// 스크립트 버전
    pub version: String,
    /// 스크립트 코드
    pub code: String,
}

/// 스크립트 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScriptType {
    /// 매크로
    #[default]
    Macro,
    /// 문서 스크립트
    Document,
    /// 글로벌 스크립트
    Global,
}

/// 배포용 문서 데이터
#[derive(Debug, Clone, Default)]
pub struct DistributeDocData {
    /// 배포 날짜
    pub date: Option<String>,
    /// 배포 제한 정보
    pub restrictions: DistributeRestrictions,
    /// 암호화된 데이터
    pub encrypted_data: Option<Vec<u8>>,
}

/// 배포 제한 정보
#[derive(Debug, Clone, Default)]
pub struct DistributeRestrictions {
    /// 인쇄 허용
    pub allow_print: bool,
    /// 복사 허용
    pub allow_copy: bool,
    /// 인쇄 횟수 제한
    pub print_count_limit: Option<u32>,
    /// 유효 기간 (일)
    pub valid_days: Option<u32>,
}

/// 변경 추적 정보
#[derive(Debug, Clone, Default)]
pub struct TrackChangeInfo {
    /// 변경 추적 활성화 여부
    pub enabled: bool,
    /// 변경 목록
    pub changes: Vec<TrackChangeRecord>,
    /// 사용자 목록
    pub users: Vec<TrackChangeUser>,
}

/// 변경 추적 레코드
#[derive(Debug, Clone)]
pub struct TrackChangeRecord {
    /// 변경 ID
    pub id: u32,
    /// 변경 타입
    pub change_type: TrackChangeRecordType,
    /// 사용자 ID
    pub user_id: u32,
    /// 변경 날짜
    pub date: String,
    /// 변경 내용 설명
    pub description: Option<String>,
}

/// 변경 추적 레코드 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrackChangeRecordType {
    /// 삽입
    #[default]
    Insert,
    /// 삭제
    Delete,
    /// 서식 변경
    Format,
    /// 이동
    Move,
    /// 표 변경
    Table,
}

/// 변경 추적 사용자
#[derive(Debug, Clone)]
pub struct TrackChangeUser {
    /// 사용자 ID
    pub id: u32,
    /// 사용자 이름
    pub name: String,
    /// 색상 (RGB)
    pub color: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hwp_extension_creation() {
        let ext = HwpExtension::new();
        assert!(!ext.is_encrypted());
        assert!(!ext.is_distributed());
        assert!(!ext.has_scripts());
    }

    #[test]
    fn test_hwp_version() {
        let version = HwpVersion {
            major: 5,
            minor: 1,
            build: 0,
            revision: 0,
        };
        assert!(version.is_5_0_or_later());
        assert!(version.is_5_1_or_later());
        assert_eq!(version.to_string(), "5.1.0.0");
    }

    #[test]
    fn test_file_header_flags() {
        let mut flags = FileHeaderFlags::default();
        flags.encrypted = true;
        flags.compressed = true;

        let ext = HwpExtension {
            file_header: Some(FileHeader {
                flags,
                ..Default::default()
            }),
            ..Default::default()
        };

        assert!(ext.is_encrypted());
    }
}
