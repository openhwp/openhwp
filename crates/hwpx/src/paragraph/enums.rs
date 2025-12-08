//! [AI 생성] 문단 전역 열거형 모음
//!
//! 문단/도형/표 주변에서 재사용되는 속성들의 코드 값을 한 곳에 모았습니다. KS X 6101:2024 `paralist.xsd` 기준이며, 실제 문서 맥락(가로/세로 지면, 교차 참조, 메일 머지 등)을 떠올리며 읽어야 더 잘 이해됩니다.

use serde::{Deserialize, Serialize};

/// [AI 생성] 텍스트 방향
///
/// 원본: `textDirection` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TextDirection {
    /// 가로
    #[default]
    #[serde(rename = "HORIZONTAL")]
    Horizontal,
    /// 세로
    #[serde(rename = "VERTICAL")]
    Vertical,
    /// 세로 (영문 세움)
    #[serde(rename = "VERTICALALL")]
    VerticalAll,
}

/// [AI 생성] 줄바꿈 유형
///
/// 원본: `lineWrap` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ParagraphLineWrap {
    /// 줄바꿈
    #[default]
    #[serde(rename = "BREAK")]
    Break,
    /// 자간 조정
    #[serde(rename = "SQUEEZE")]
    Squeeze,
    /// 유지
    #[serde(rename = "KEEP")]
    Keep,
}

/// [AI 생성] 세로 정렬
///
/// 원본: `vertAlign` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ParagraphVerticalAlignment {
    /// 위
    #[default]
    #[serde(rename = "TOP")]
    Top,
    /// 가운데
    #[serde(rename = "CENTER")]
    Center,
    /// 아래
    #[serde(rename = "BOTTOM")]
    Bottom,
}

/// [AI 생성] 필드 유형
///
/// 원본: `FieldType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
    /// 클릭 여기 필드
    #[serde(rename = "CLICK_HERE")]
    ClickHere,
    /// 하이퍼링크 필드
    #[serde(rename = "HYPERLINK")]
    Hyperlink,
    /// 책갈피 필드
    #[serde(rename = "BOOKMARK")]
    Bookmark,
    /// 공식 필드
    #[serde(rename = "FORMULA")]
    Formula,
    /// 요약 정보 필드
    #[serde(rename = "SUMMERY")]
    Summary,
    /// 사용자 정보 필드
    #[serde(rename = "USER_INFO")]
    UserInfo,
    /// 날짜 필드
    #[serde(rename = "DATE")]
    Date,
    /// 문서 날짜 필드
    #[serde(rename = "DOC_DATE")]
    DocumentDate,
    /// 경로 필드
    #[serde(rename = "PATH")]
    Path,
    /// 상호 참조 필드
    #[serde(rename = "CROSSREF")]
    CrossReference,
    /// 메일 병합 필드
    #[serde(rename = "MAILMERGE")]
    MailMerge,
    /// 메모 필드
    #[serde(rename = "MEMO")]
    Memo,
    /// 교정 부호 필드
    #[serde(rename = "PROOFREADING_MARKS")]
    ProofreadingMarks,
    /// 개인 정보 보호 필드
    #[serde(rename = "PRIVATE_INFO")]
    PrivateInfo,
    /// 메타 태그 필드
    #[serde(rename = "METATAG")]
    MetaTag,
}

/// [AI 생성] 페이지 시작 위치
///
/// 원본: `pageStartsOn` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PageStartsOn {
    /// 양쪽
    #[default]
    #[serde(rename = "BOTH")]
    Both,
    /// 짝수쪽
    #[serde(rename = "EVEN")]
    Even,
    /// 홀수쪽
    #[serde(rename = "ODD")]
    Odd,
}

/// [AI 생성] 페이지 번호 위치
///
/// 원본: `pageNum.pos` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PageNumberPosition {
    /// 없음
    #[serde(rename = "NONE")]
    None,
    /// 위쪽 왼쪽
    #[default]
    #[serde(rename = "TOP_LEFT")]
    TopLeft,
    /// 위쪽 가운데
    #[serde(rename = "TOP_CENTER")]
    TopCenter,
    /// 위쪽 오른쪽
    #[serde(rename = "TOP_RIGHT")]
    TopRight,
    /// 아래쪽 왼쪽
    #[serde(rename = "BOTTOM_LEFT")]
    BottomLeft,
    /// 아래쪽 가운데
    #[serde(rename = "BOTTOM_CENTER")]
    BottomCenter,
    /// 아래쪽 오른쪽
    #[serde(rename = "BOTTOM_RIGHT")]
    BottomRight,
    /// 바깥쪽 위
    #[serde(rename = "OUTSIDE_TOP")]
    OutsideTop,
    /// 바깥쪽 아래
    #[serde(rename = "OUTSIDE_BOTTOM")]
    OutsideBottom,
    /// 안쪽 위
    #[serde(rename = "INSIDE_TOP")]
    InsideTop,
    /// 안쪽 아래
    #[serde(rename = "INSIDE_BOTTOM")]
    InsideBottom,
}

/// [AI 생성] 탭 유형 (문단 내)
///
/// 원본: `tab.type` 속성의 익명 타입
///
/// # 값 매핑
/// - 0 = LEFT (왼쪽)
/// - 1 = RIGHT (오른쪽)
/// - 2 = CENTER (가운데)
/// - 3 = DECIMAL (소수점)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum InlineTabType {
    /// 왼쪽
    #[default]
    #[serde(rename = "LEFT", alias = "0")]
    Left,
    /// 오른쪽
    #[serde(rename = "RIGHT", alias = "1")]
    Right,
    /// 가운데
    #[serde(rename = "CENTER", alias = "2")]
    Center,
    /// 소수점
    #[serde(rename = "DECIMAL", alias = "3")]
    Decimal,
}

/// [AI 생성] 가시성 값
///
/// 원본: `VisibilityValue`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum VisibilityValue {
    /// 첫 번째 숨기기
    #[serde(rename = "HIDE_FIRST")]
    HideFirst,
    /// 첫 번째 보이기
    #[serde(rename = "SHOW_FIRST")]
    ShowFirst,
    /// 모두 보이기
    #[default]
    #[serde(rename = "SHOW_ALL")]
    ShowAll,
}

/// [AI 생성] 단 종류
///
/// 원본: `ColumnDefType.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ColumnType {
    /// 신문형
    #[default]
    #[serde(rename = "NEWSPAPER")]
    Newspaper,
    /// 균형 신문형
    #[serde(rename = "BALANCED_NEWSPAPER")]
    BalancedNewspaper,
    /// 평행형
    #[serde(rename = "PARALLEL")]
    Parallel,
}

/// [AI 생성] 단 배치 방향
///
/// 원본: `ColumnDefType.layout` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ColumnLayout {
    /// 왼쪽
    #[default]
    #[serde(rename = "LEFT")]
    Left,
    /// 오른쪽
    #[serde(rename = "RIGHT")]
    Right,
    /// 거울
    #[serde(rename = "MIRROR")]
    Mirror,
}

/// [AI 생성] 용지 방향
///
/// 원본: `pagePr.landscape` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PaperOrientation {
    /// 가로
    #[serde(rename = "WIDELY")]
    Landscape,
    /// 세로
    #[default]
    #[serde(rename = "NARROWLY")]
    Portrait,
}

/// [AI 생성] 제본 유형
///
/// 원본: `pagePr.gutterType` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GutterType {
    /// 왼쪽만
    #[default]
    #[serde(rename = "LEFT_ONLY")]
    LeftOnly,
    /// 좌우
    #[serde(rename = "LEFT_RIGHT")]
    LeftRight,
    /// 상하
    #[serde(rename = "TOP_BOTTOM")]
    TopBottom,
}

/// [AI 생성] 각주 번호 매기기 형식
///
/// 원본: `FootNoteShapeType.numbering.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FootnoteNumberingType {
    /// 앞 구역에 이어서
    #[default]
    #[serde(rename = "CONTINUOUS")]
    Continuous,
    /// 현재 구역부터 새로 시작
    #[serde(rename = "ON_SECTION")]
    OnSection,
    /// 쪽마다 새로 시작 (각주 전용)
    #[serde(rename = "ON_PAGE")]
    OnPage,
}

/// [AI 생성] 미주 번호 매기기 형식
///
/// 원본: `EndNoteShapeType.numbering.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EndnoteNumberingType {
    /// 앞 구역에 이어서
    #[default]
    #[serde(rename = "CONTINUOUS")]
    Continuous,
    /// 현재 구역부터 새로 시작
    #[serde(rename = "ON_SECTION")]
    OnSection,
}

/// [AI 생성] 각주 배치
///
/// 원본: `FootNoteShapeType.placement.place` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FootnotePlacement {
    /// 각 단마다 따로 배열
    #[default]
    #[serde(rename = "EACH_COLUMN")]
    EachColumn,
    /// 통단으로 배열
    #[serde(rename = "MERGED_COLUMN")]
    MergedColumn,
    /// 가장 오른쪽 단에 배열
    #[serde(rename = "RIGHT_MOST_COLUMN")]
    RightMostColumn,
}

/// [AI 생성] 미주 배치
///
/// 원본: `EndNoteShapeType.placement.place` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EndnotePlacement {
    /// 문서의 마지막
    #[default]
    #[serde(rename = "END_OF_DOCUMENT")]
    EndOfDocument,
    /// 구역의 마지막
    #[serde(rename = "END_OF_SECTION")]
    EndOfSection,
}

/// [AI 생성] 쪽 테두리 유형
///
/// 원본: `pageBorderFill.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PageBorderType {
    /// 양쪽
    #[default]
    #[serde(rename = "BOTH")]
    Both,
    /// 짝수쪽
    #[serde(rename = "EVEN")]
    Even,
    /// 홀수쪽
    #[serde(rename = "ODD")]
    Odd,
}

/// [AI 생성] 쪽 테두리 위치 기준
///
/// 원본: `pageBorderFill.textBorder` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PageBorderPosition {
    /// 본문 기준
    #[default]
    #[serde(rename = "CONTENT")]
    Content,
    /// 종이 기준
    #[serde(rename = "PAPER")]
    Paper,
}

/// [AI 생성] 채울 영역
///
/// 원본: `pageBorderFill.fillArea` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FillAreaType {
    /// 종이
    #[default]
    #[serde(rename = "PAPER")]
    Paper,
    /// 쪽
    #[serde(rename = "PAGE")]
    Page,
    /// 테두리
    #[serde(rename = "BORDER")]
    Border,
}

/// [AI 생성] 기본 탭 간격 단위
///
/// 원본: `tabStopUnit` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TabStopUnit {
    /// 글자 단위
    #[serde(rename = "CHAR")]
    Character,
    /// HWP 유닛
    #[default]
    #[serde(rename = "HWPUNIT")]
    HwpUnit,
}

/// [AI 생성] 표 쪽 나눔 설정
///
/// 원본: `TableType.pageBreak` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TablePageBreak {
    /// 표 단위
    #[serde(rename = "TABLE")]
    Table,
    /// 셀 단위
    #[default]
    #[serde(rename = "CELL")]
    Cell,
    /// 나눔 안함
    #[serde(rename = "NONE")]
    None,
}
