//! 문서 메타데이터
//!
//! 문서의 제목, 저자, 생성일 등 부가 정보를 표현합니다.

/// 문서 메타데이터
#[derive(Debug, Clone, Default)]
pub struct Metadata {
    /// 문서 제목
    pub title: Option<String>,
    /// 저자
    pub author: Option<String>,
    /// 주제
    pub subject: Option<String>,
    /// 키워드
    pub keywords: Vec<String>,
    /// 설명
    pub description: Option<String>,
    /// 생성일시 (ISO 8601 형식)
    pub created: Option<String>,
    /// 수정일시 (ISO 8601 형식)
    pub modified: Option<String>,
    /// 최종 저장자
    pub last_saved_by: Option<String>,
    /// 버전 정보
    pub version: Option<DocumentVersion>,
}

impl Metadata {
    /// 빈 메타데이터 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 제목 설정
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 저자 설정
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }
}

/// 문서 버전 정보
#[derive(Debug, Clone, Default)]
pub struct DocumentVersion {
    /// 주 버전
    pub major: u32,
    /// 부 버전
    pub minor: u32,
    /// 패치 버전
    pub patch: u32,
    /// 빌드 번호
    pub build: u32,
}

impl DocumentVersion {
    /// 버전 생성
    pub const fn new(major: u32, minor: u32, patch: u32, build: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            build,
        }
    }

    /// 문자열로 변환 (예: "5.1.0.0")
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}.{}", self.major, self.minor, self.patch, self.build)
    }
}
