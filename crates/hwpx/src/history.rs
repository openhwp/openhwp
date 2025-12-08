//! [AI 생성 문서화] 히스토리 정보
//!
//! 패키지 내 변경 이력(`history.xml`)을 표현합니다. 리비전 번호·작성자·자동저장 여부와 패키지/헤더/본문/꼬리의 Diff 경로를 함께 보관해 뷰어가 특정 버전을 복원할 수 있게 합니다. KS X 6101:2024 `history.xsd` 기반.

use serde::{Deserialize, Serialize};

/// [AI 생성] 히스토리 루트 요소
///
/// 원본: `history` 요소. 수정 이력 레코드와 히스토리 스키마 버전을 묶어 제공합니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "history")]
pub struct History {
    /// [AI 생성] 히스토리 항목 목록 (`historyEntry` 요소). 시간순 리비전 스택.
    #[serde(rename = "historyEntry")]
    pub entries: Vec<HistoryEntry>,

    /// [AI 생성] 이력 파일 버전 (`version` 속성). `history.xsd` 호환성을 확인할 때 사용합니다.
    #[serde(rename = "@version")]
    pub version: String,
}

/// [AI 생성] 히스토리 항목
///
/// 원본: `HistoryEntryType`. 패키지/헤더/본문/꼬리 변경을 분리 기록하며, 리비전 메타데이터를 함께 보관합니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// [AI 생성] 패키지 차이 (`packageDiff` 요소). 압축 패키지 레벨 변경 사항.
    #[serde(rename = "packageDiff", skip_serializing_if = "Option::is_none")]
    pub package_diff: Option<DiffEntry>,

    /// [AI 생성] 헤더 차이 (`headDiff` 요소). DocInfo 영역 변경.
    #[serde(rename = "headDiff", skip_serializing_if = "Option::is_none")]
    pub head_diff: Option<DiffEntry>,

    /// [AI 생성] 본문 차이 목록 (`bodyDiff` 요소). 섹션별 Diff가 다수일 수 있습니다.
    #[serde(rename = "bodyDiff", default)]
    pub body_diffs: Vec<DiffEntry>,

    /// [AI 생성] 꼬리 차이 (`tailDiff` 요소). Tail 영역 변경.
    #[serde(rename = "tailDiff", skip_serializing_if = "Option::is_none")]
    pub tail_diff: Option<DiffEntry>,

    /// [AI 생성] 리비전 번호 (`revisionNumber` 속성). 시간 순 정렬 키.
    #[serde(rename = "@revisionNumber", skip_serializing_if = "Option::is_none")]
    pub revision_number: Option<u32>,

    /// [AI 생성] 리비전 날짜 (`revisionDate` 속성). 패턴: `YYYY-MM-DD hh:mm:ss mmm`.
    #[serde(rename = "@revisionDate", skip_serializing_if = "Option::is_none")]
    pub revision_date: Option<String>,

    /// [AI 생성] 리비전 작성자 (`revisionAuthor` 속성). 사람이 읽을 수 있는 표기.
    #[serde(rename = "@revisionAuthor", skip_serializing_if = "Option::is_none")]
    pub revision_author: Option<String>,

    /// [AI 생성] 리비전 설명 (`revisionDesc` 속성). 변경 요약.
    #[serde(rename = "@revisionDesc", skip_serializing_if = "Option::is_none")]
    pub revision_description: Option<String>,

    /// [AI 생성] 리비전 잠금 여부 (`revisionLock` 속성). 참이면 해당 리비전 수정 금지.
    #[serde(rename = "@revisionLock", default)]
    pub revision_lock: bool,

    /// [AI 생성] 자동 저장 여부 (`autoSave` 속성). 자동 저장으로 생성된 리비전인지 표시.
    #[serde(rename = "@autoSave", default)]
    pub auto_save: bool,
}

/// [AI 생성] 차이 데이터 (추상 타입의 인라인)
///
/// 원본: `DiffEntryType`. 삽입·갱신·삭제·위치 변경을 혼합하여 기록하며, 대상 파일 경로를 함께 둡니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffEntry {
    /// [AI 생성] 차이 작업 목록 (`insert`/`update`/`delete`/`position`). 순서대로 적용됩니다.
    #[serde(rename = "$value")]
    pub operations: Vec<DiffOperation>,

    /// [AI 생성] 변경 추적 대상 파일 경로 (`href` 속성, 컨테이너 내 절대 경로)
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

/// [AI 생성] 차이 작업 종류
///
/// 원본: `insert`, `update`, `delete`, `position` 요소.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffOperation {
    /// [AI 생성] 삽입 (`InsertType`)
    #[serde(rename = "insert")]
    Insert(DiffInsert),

    /// [AI 생성] 업데이트 (`UpdateType`)
    #[serde(rename = "update")]
    Update(DiffUpdate),

    /// [AI 생성] 삭제 (`DeleteType`)
    #[serde(rename = "delete")]
    Delete(DiffDelete),

    /// [AI 생성] 위치 정보 (`PositionType`)
    #[serde(rename = "position")]
    Position(DiffPosition),
}

/// [AI 생성] 삽입 차이 데이터
///
/// 원본: `InsertType`. 삽입 위치를 가리키며, 실제 삽입 내용은 별도 리소스에 존재할 수 있습니다.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DiffInsert {
    /// [AI 생성] 경로 (`path` 속성). 컨테이너 내 대상 위치.
    #[serde(rename = "@path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// [AI 생성] 업데이트 차이 데이터
///
/// 원본: `UpdateType`. 중첩 Diff를 포함할 수 있으며 이전 값을 함께 남깁니다.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DiffUpdate {
    /// [AI 생성] 중첩된 차이 작업 목록. 부분 업데이트를 더 세분화합니다.
    #[serde(rename = "$value", default)]
    pub operations: Vec<DiffOperation>,

    /// [AI 생성] 대상 경로 (`path` 속성)
    #[serde(rename = "@path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// [AI 생성] 이전 값 (`oldValue` 속성). 패치 전 상태.
    #[serde(rename = "@oldValue", skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
}

/// [AI 생성] 삭제 차이 데이터
///
/// 원본: `DeleteType`. 삭제된 텍스트와 대상 경로를 포함합니다.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DiffDelete {
    /// [AI 생성] 삭제된 내용 (mixed content). 텍스트 덩어리 그대로 보관.
    #[serde(rename = "$text", default)]
    pub content: String,

    /// [AI 생성] 대상 경로 (`path` 속성)
    #[serde(rename = "@path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// [AI 생성] 위치 차이 데이터
///
/// 원본: `PositionType`. 재배치가 필요할 때 위치만 기록합니다.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DiffPosition {
    /// [AI 생성] 대상 경로 (`path` 속성)
    #[serde(rename = "@path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
