//! 런 내용 (RunContent)
//!
//! 런 내부에 포함될 수 있는 다양한 콘텐츠 타입입니다.

use crate::id::ControlId;

/// 런 내용
#[derive(Debug, Clone)]
pub enum RunContent {
    /// 텍스트
    Text(String),
    /// 탭
    Tab,
    /// 줄바꿈 (Shift+Enter)
    LineBreak,
    /// 하이픈
    Hyphen,
    /// 줄바꿈 안 되는 공백
    NonBreakingSpace,
    /// 고정폭 공백
    FixedWidthSpace,
    /// 컨트롤 (표, 그림, 도형 등)
    Control(ControlId),
    /// 필드 시작
    FieldStart(FieldStart),
    /// 필드 끝
    FieldEnd,
    /// 책갈피 시작
    BookmarkStart(BookmarkStart),
    /// 책갈피 끝
    BookmarkEnd,
}

/// 필드 시작
#[derive(Debug, Clone)]
pub struct FieldStart {
    /// 필드 타입
    pub field_type: FieldType,
    /// 필드 ID
    pub field_id: Option<String>,
    /// 편집 가능 여부
    pub editable: bool,
    /// 변경 여부
    pub dirty: bool,
}

/// 필드 타입
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    /// 하이퍼링크
    Hyperlink {
        /// 대상 URL
        target: String,
        /// 툴팁
        tooltip: Option<String>,
    },
    /// 책갈피
    Bookmark {
        /// 책갈피 이름
        name: String,
    },
    /// 날짜
    Date {
        /// 형식 문자열
        format: Option<String>,
    },
    /// 시간
    Time {
        /// 형식 문자열
        format: Option<String>,
    },
    /// 페이지 번호
    PageNumber,
    /// 전체 페이지 수
    TotalPages,
    /// 파일 이름
    FileName,
    /// 파일 경로
    FilePath,
    /// 제목
    Title,
    /// 저자
    Author,
    /// 요약
    Summary,
    /// 상호 참조
    CrossReference {
        /// 참조 대상 ID
        target_id: String,
    },
    /// 메일 머지
    MailMerge {
        /// 필드 이름
        field_name: String,
    },
    /// 목차
    TableOfContents,
    /// 수식 필드
    Formula {
        /// 수식 문자열
        expression: String,
    },
    /// 사용자 정보
    UserInfo {
        /// 정보 종류
        info_type: String,
    },
    /// 메모
    Memo,
    /// 누름틀 (ClickHere)
    ClickHere {
        /// 기본 텍스트
        placeholder: Option<String>,
    },
    /// 개인정보
    PrivateInfo,
    /// 메타태그
    MetaTag {
        /// 태그 이름
        name: String,
    },
    /// 알 수 없는 필드
    Unknown(String),
}

/// 책갈피 시작
#[derive(Debug, Clone)]
pub struct BookmarkStart {
    /// 책갈피 ID
    pub id: u32,
    /// 책갈피 이름
    pub name: String,
}

impl Default for FieldStart {
    fn default() -> Self {
        Self {
            field_type: FieldType::Unknown(String::new()),
            field_id: None,
            editable: true,
            dirty: false,
        }
    }
}
