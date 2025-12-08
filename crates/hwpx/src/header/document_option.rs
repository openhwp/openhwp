//! [AI 생성 문서화] 문서 옵션
//!
//! 전역 문서 옵션을 담습니다. 현재는 링크 문서 경로만 포함하지만 확장 여지를 고려해 별도 요소로 유지합니다. KS X 6101:2024 `header.xsd` 참조.

use serde::{Deserialize, Serialize};

/// [AI 생성] 문서 옵션
///
/// 원본: 문서 전역 옵션 설정. 링크 문서 경로 등 외부 참조 정보를 포함합니다.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "docOption")]
pub struct DocumentOption {
    /// [AI 생성] 링크 문서 경로 (`linkDocPath` 요소). 외부 문서와 연동할 때 사용됩니다.
    #[serde(rename = "linkDocPath", skip_serializing_if = "Option::is_none")]
    pub link_document_path: Option<String>,
}
