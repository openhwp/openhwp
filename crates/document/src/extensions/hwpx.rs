//! HWPX 형식 확장
//!
//! HWPX (KS X 6101:2024) XML 형식에 특화된 데이터를 관리합니다.

/// HWPX 확장 데이터
#[derive(Debug, Clone, Default)]
pub struct HwpxExtension {
    /// 버전 정보
    pub version: Option<HcfVersion>,
    /// 마스터 페이지 목록
    pub master_pages: Vec<MasterPage>,
    /// 변경 이력
    pub change_history: Option<ChangeHistory>,
    /// 변경 추적 설정
    pub track_change_config: Option<TrackChangeConfig>,
    /// 시작 번호 설정
    pub begin_numbers: Option<BeginNumbers>,
    /// 양식 데이터
    pub form_data: Option<FormData>,
    /// 메모 목록
    pub memos: Vec<Memo>,
}

impl HwpxExtension {
    /// 새 HWPX 확장 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 버전 문자열
    pub fn version_string(&self) -> Option<String> {
        self.version.as_ref().map(|v| v.to_string())
    }

    /// 마스터 페이지 수
    pub fn master_page_count(&self) -> usize {
        self.master_pages.len()
    }

    /// 변경 추적 활성화 여부
    pub fn is_track_change_enabled(&self) -> bool {
        self.track_change_config
            .as_ref()
            .map(|c| c.enabled)
            .unwrap_or(false)
    }

    /// 메모 수
    pub fn memo_count(&self) -> usize {
        self.memos.len()
    }
}

/// HCF (HWPX Container Format) 버전
#[derive(Debug, Clone, Default)]
pub struct HcfVersion {
    /// 타이틀
    pub title: Option<String>,
    /// 언어
    pub language: Option<String>,
    /// 메타데이터
    pub meta: Option<String>,
    /// 버전 (major.minor.build)
    pub version: (u8, u8, u16),
}

impl HcfVersion {
    /// 1.0.0 이상인지 확인
    pub fn is_1_0_or_later(&self) -> bool {
        self.version.0 >= 1
    }
}

impl std::fmt::Display for HcfVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.version.0, self.version.1, self.version.2)
    }
}

/// 마스터 페이지
#[derive(Debug, Clone)]
pub struct MasterPage {
    /// 마스터 페이지 ID
    pub id: String,
    /// 마스터 페이지 타입
    pub page_type: MasterPageType,
    /// 페이지 크기
    pub width: ir::HwpUnit,
    pub height: ir::HwpUnit,
    /// 머리말 참조 ID
    pub header_ref: Option<String>,
    /// 바닥글 참조 ID
    pub footer_ref: Option<String>,
    /// 쪽 번호 위치
    pub page_number_position: Option<PageNumberPosition>,
}

/// 마스터 페이지 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MasterPageType {
    /// 기본
    #[default]
    Default,
    /// 첫 페이지
    FirstPage,
    /// 홀수 페이지
    OddPage,
    /// 짝수 페이지
    EvenPage,
}

/// 쪽 번호 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PageNumberPosition {
    /// 없음
    None,
    /// 위쪽 왼쪽
    TopLeft,
    /// 위쪽 가운데
    #[default]
    TopCenter,
    /// 위쪽 오른쪽
    TopRight,
    /// 아래쪽 왼쪽
    BottomLeft,
    /// 아래쪽 가운데
    BottomCenter,
    /// 아래쪽 오른쪽
    BottomRight,
    /// 바깥쪽 (짝수:왼쪽, 홀수:오른쪽)
    Outside,
    /// 안쪽 (짝수:오른쪽, 홀수:왼쪽)
    Inside,
}

/// 변경 이력
#[derive(Debug, Clone, Default)]
pub struct ChangeHistory {
    /// 이력 목록
    pub entries: Vec<ChangeHistoryEntry>,
}

/// 변경 이력 항목
#[derive(Debug, Clone)]
pub struct ChangeHistoryEntry {
    /// 버전 번호
    pub version: u32,
    /// 변경 날짜/시간
    pub date: String,
    /// 변경 사용자
    pub author: Option<String>,
    /// 변경 설명
    pub description: Option<String>,
}

/// 변경 추적 설정
#[derive(Debug, Clone, Default)]
pub struct TrackChangeConfig {
    /// 변경 추적 활성화 여부
    pub enabled: bool,
    /// 변경 추적 표시 여부
    pub show_changes: bool,
    /// 삽입 표시 방식
    pub insert_display: TrackChangeDisplay,
    /// 삭제 표시 방식
    pub delete_display: TrackChangeDisplay,
    /// 변경 색상 사용자 목록
    pub user_colors: Vec<UserColor>,
}

/// 변경 추적 표시 방식
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrackChangeDisplay {
    /// 밑줄
    #[default]
    Underline,
    /// 취소선
    Strikethrough,
    /// 색상 변경
    ColorChange,
    /// 표시 안 함
    Hidden,
}

/// 사용자 색상
#[derive(Debug, Clone)]
pub struct UserColor {
    /// 사용자 ID
    pub user_id: String,
    /// 사용자 이름
    pub name: String,
    /// 색상 (RGB)
    pub color: u32,
}

/// 시작 번호 설정
#[derive(Debug, Clone, Default)]
pub struct BeginNumbers {
    /// 페이지 시작 번호
    pub page: Option<u32>,
    /// 각주 시작 번호
    pub footnote: Option<u32>,
    /// 미주 시작 번호
    pub endnote: Option<u32>,
    /// 그림 시작 번호
    pub picture: Option<u32>,
    /// 표 시작 번호
    pub table: Option<u32>,
    /// 수식 시작 번호
    pub equation: Option<u32>,
}

/// 양식 데이터
#[derive(Debug, Clone, Default)]
pub struct FormData {
    /// 양식 필드 목록
    pub fields: Vec<FormField>,
}

/// 양식 필드
#[derive(Debug, Clone)]
pub struct FormField {
    /// 필드 ID
    pub id: String,
    /// 필드 이름
    pub name: String,
    /// 필드 타입
    pub field_type: FormFieldType,
    /// 필드 값
    pub value: Option<String>,
    /// 기본값
    pub default_value: Option<String>,
    /// 필수 여부
    pub required: bool,
    /// 읽기 전용 여부
    pub readonly: bool,
}

/// 양식 필드 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormFieldType {
    /// 텍스트
    #[default]
    Text,
    /// 숫자
    Number,
    /// 날짜
    Date,
    /// 체크박스
    Checkbox,
    /// 라디오
    Radio,
    /// 콤보박스
    Combo,
    /// 리스트
    List,
}

/// 메모
#[derive(Debug, Clone)]
pub struct Memo {
    /// 메모 ID
    pub id: String,
    /// 메모 위치 (문단 ID)
    pub paragraph_ref: String,
    /// 메모 위치 (문자 오프셋)
    pub char_offset: u32,
    /// 메모 내용
    pub content: String,
    /// 작성자
    pub author: Option<String>,
    /// 작성 날짜
    pub date: Option<String>,
    /// 답글 목록
    pub replies: Vec<MemoReply>,
}

/// 메모 답글
#[derive(Debug, Clone)]
pub struct MemoReply {
    /// 답글 ID
    pub id: String,
    /// 답글 내용
    pub content: String,
    /// 작성자
    pub author: Option<String>,
    /// 작성 날짜
    pub date: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hwpx_extension_creation() {
        let ext = HwpxExtension::new();
        assert_eq!(ext.master_page_count(), 0);
        assert!(!ext.is_track_change_enabled());
        assert_eq!(ext.memo_count(), 0);
    }

    #[test]
    fn test_hcf_version() {
        let version = HcfVersion {
            title: Some("Test Document".into()),
            language: Some("ko".into()),
            meta: None,
            version: (1, 0, 0),
        };
        assert!(version.is_1_0_or_later());
        assert_eq!(version.to_string(), "1.0.0");
    }

    #[test]
    fn test_track_change_config() {
        let ext = HwpxExtension {
            track_change_config: Some(TrackChangeConfig {
                enabled: true,
                show_changes: true,
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(ext.is_track_change_enabled());
    }

    #[test]
    fn test_master_page() {
        let mut ext = HwpxExtension::new();
        ext.master_pages.push(MasterPage {
            id: "mp1".into(),
            page_type: MasterPageType::Default,
            width: ir::HwpUnit::from_mm(210.0),
            height: ir::HwpUnit::from_mm(297.0),
            header_ref: None,
            footer_ref: None,
            page_number_position: Some(PageNumberPosition::BottomCenter),
        });
        assert_eq!(ext.master_page_count(), 1);
    }
}
