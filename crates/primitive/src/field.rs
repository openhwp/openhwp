//! 필드 종류 관련 열거형

/// 필드 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FieldType {
    /// 알 수 없음
    #[default]
    Unknown,
    /// 날짜
    Date,
    /// 시간
    Time,
    /// 파일 이름
    FileName,
    /// 파일 경로
    FilePath,
    /// 페이지 번호
    PageNumber,
    /// 총 페이지 수
    PageCount,
    /// 제목
    Title,
    /// 저자
    Author,
    /// 요약
    Summary,
    /// 상호 참조
    CrossReference,
    /// 메일 머지
    MailMerge,
    /// 목차
    TableOfContents,
    /// 책갈피
    Bookmark,
    /// 하이퍼링크
    Hyperlink,
    /// 클릭하세요 (입력 필드)
    ClickHere,
    /// 사용자 정보
    UserInfo,
    /// 수식 (계산)
    Formula,
    /// 메모
    Memo,
    /// 개인정보
    PrivateInfo,
    /// 메타태그
    MetaTag,
}

/// 바이너리 데이터 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BinaryDataType {
    /// 외부 파일 링크
    #[default]
    Link,
    /// 내장 바이너리 데이터
    Embedding,
    /// OLE 저장소
    Storage,
}

/// 바이너리 데이터 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BinaryDataState {
    /// 아직 접근 안 함
    #[default]
    NotAccessed,
    /// 접근 성공
    AccessSuccess,
    /// 접근 실패
    AccessFailed,
    /// 링크 접근 실패 (무시됨)
    AccessIgnored,
}
