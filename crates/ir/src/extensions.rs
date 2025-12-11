//! 형식별 확장 데이터
//!
//! HWP나 HWPX 고유의 기능을 위한 확장 구조입니다.
//! 다른 형식으로 변환 시 이 데이터는 손실될 수 있습니다.

/// 형식별 확장 데이터 컨테이너
#[derive(Debug, Clone, Default)]
pub struct Extensions {
    /// HWP 5.0 고유 데이터
    pub hwp: Option<HwpExtensions>,
    /// HWPX 고유 데이터
    pub hwpx: Option<HwpxExtensions>,
}

impl Extensions {
    /// 빈 확장 데이터 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// HWP 확장 데이터 설정
    pub fn with_hwp(mut self, hwp: HwpExtensions) -> Self {
        self.hwp = Some(hwp);
        self
    }

    /// HWPX 확장 데이터 설정
    pub fn with_hwpx(mut self, hwpx: HwpxExtensions) -> Self {
        self.hwpx = Some(hwpx);
        self
    }
}

/// HWP 5.0 고유 확장 데이터
///
/// hwpx로 변환 시 손실됩니다.
#[derive(Debug, Clone, Default)]
pub struct HwpExtensions {
    /// 배포용 문서 데이터
    ///
    /// 문서 배포/보안 관련 정보. HWPX에는 해당 개념이 없음.
    pub distribute_doc_data: Option<DistributeDocData>,

    /// 스크립트 코드
    ///
    /// 문서에 포함된 JavaScript. HWPX에서는 지원하지 않음.
    pub scripts: Option<String>,

    /// 레이아웃 호환성 설정
    pub layout_compatibility: Option<LayoutCompatibility>,

    /// 문서 데이터 (임의 데이터)
    pub document_data: Option<Vec<u8>>,
}

/// 배포용 문서 데이터
#[derive(Debug, Clone, Default)]
pub struct DistributeDocData {
    /// 배포 설정 플래그
    pub flags: u32,
    /// 암호화된 데이터
    pub data: Vec<u8>,
}

/// 레이아웃 호환성 설정
#[derive(Debug, Clone, Default)]
pub struct LayoutCompatibility {
    /// 호환 대상 버전
    pub target_version: Option<String>,
    /// 레이아웃 플래그
    pub flags: u32,
}

/// HWPX 고유 확장 데이터
///
/// HWP로 변환 시 손실됩니다.
#[derive(Debug, Clone, Default)]
pub struct HwpxExtensions {
    /// 변경 이력
    ///
    /// 문서 변경 히스토리. HWP에는 TrackChange만 있고 전체 이력은 없음.
    pub change_history: Option<ChangeHistory>,

    /// 마스터 페이지 정보
    ///
    /// HWPX의 마스터 페이지 시스템. HWP에는 해당 개념이 없음.
    pub master_pages: Vec<MasterPageInfo>,

    /// 프레젠테이션 설정
    ///
    /// 슬라이드 쇼 관련 설정. HWP에서는 지원하지 않음.
    pub presentation: Option<PresentationSettings>,

    /// 금칙 문자 목록
    ///
    /// 줄 끝에 올 수 없는 문자들. HWP에서는 다른 방식으로 처리됨.
    pub forbidden_words: Vec<String>,

    /// 레이아웃 호환성 설정
    ///
    /// HWPX 고유의 레이아웃 호환성 플래그들.
    pub layout_compatibility: Option<HwpxLayoutCompatibility>,

    /// 문서 옵션
    ///
    /// HWPX 문서 옵션 설정.
    pub document_option: Option<HwpxDocumentOption>,

    /// 변경 추적 설정
    ///
    /// 변경 추적 표시 옵션.
    pub track_change_config: Option<TrackChangeConfig>,
}

/// 변경 이력
#[derive(Debug, Clone, Default)]
pub struct ChangeHistory {
    /// 이력 항목들
    pub entries: Vec<ChangeHistoryEntry>,
}

/// 변경 이력 항목
#[derive(Debug, Clone)]
pub struct ChangeHistoryEntry {
    /// 변경일시
    pub timestamp: String,
    /// 저자
    pub author: Option<String>,
    /// 변경 설명
    pub description: Option<String>,
}

/// 마스터 페이지 정보
#[derive(Debug, Clone)]
pub struct MasterPageInfo {
    /// 마스터 페이지 ID
    pub id: String,
    /// 적용 대상 (양면, 홀수, 짝수 등)
    pub application_type: MasterPageApplicationType,
    /// 마스터 페이지 내용 (문단들)
    pub paragraphs: Vec<crate::Paragraph>,
    /// 특정 페이지 번호 (Optional인 경우)
    pub page_number: Option<u32>,
    /// 페이지 복제 여부
    pub page_duplicate: bool,
    /// 앞면 여부
    pub page_front: bool,
}

/// 마스터 페이지 적용 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MasterPageApplicationType {
    /// 모든 페이지
    #[default]
    Both,
    /// 짝수 페이지
    Even,
    /// 홀수 페이지
    Odd,
    /// 마지막 페이지
    Last,
    /// 특정 페이지
    Optional,
}

/// 프레젠테이션 설정
#[derive(Debug, Clone, Default)]
pub struct PresentationSettings {
    /// 슬라이드 전환 효과
    pub transition: Option<String>,
    /// 자동 재생 간격 (초)
    pub auto_advance_seconds: Option<u32>,
}

/// HWPX 레이아웃 호환성 설정
#[derive(Debug, Clone, Default)]
pub struct HwpxLayoutCompatibility {
    /// 대상 프로그램
    pub target_program: HwpxTargetProgram,
    /// 호환성 플래그들 (HWPX에서 정의된 다양한 호환성 옵션)
    pub flags: u64,
}

/// HWPX 대상 프로그램
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HwpxTargetProgram {
    /// HWP 201X
    #[default]
    Hwp201X,
    /// HWP 200X
    Hwp200X,
    /// MS Word
    MsWord,
}

/// HWPX 문서 옵션
#[derive(Debug, Clone, Default)]
pub struct HwpxDocumentOption {
    /// 링크된 문서 경로 (상대/절대)
    pub link_document_path: Option<String>,
    /// 기타 옵션 플래그
    pub flags: u32,
}

/// 변경 추적 설정
#[derive(Debug, Clone, Default)]
pub struct TrackChangeConfig {
    /// 추적 활성화 여부
    pub enabled: bool,
    /// 삽입 색상
    pub insert_color: Option<String>,
    /// 삭제 색상
    pub delete_color: Option<String>,
}
