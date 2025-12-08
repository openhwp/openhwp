//! [AI 생성 문서화] 헤더 정보
//!
//! KS X 6101:2024 - header.xsd에 기반한 설명입니다. 이 모듈의 타입·필드 문서화는 AI가 작성했으므로, 필요 시 `docs/hwpx/schemas/header.xsd`와 대조해 주세요.

pub mod begin_number;
pub mod border_fill;
pub mod bullet;
pub mod character_shape;
pub mod compatible_document;
pub mod document_option;
pub mod font;
pub mod forbidden_word;
pub mod mapping_table;
pub mod memo_shape;
pub mod numbering;
pub mod paragraph_head;
pub mod paragraph_shape;
pub mod style;
pub mod tab_definition;
pub mod track_change;

use serde::{Deserialize, Serialize};

use self::{
    begin_number::BeginNumber, compatible_document::CompatibleDocument,
    document_option::DocumentOption, forbidden_word::ForbiddenWordList,
    mapping_table::MappingTable, track_change::TrackChangeConfig,
};

/// [AI 생성] 헤더 루트 요소
///
/// 원본: `head` 요소
///
/// 참고: 일부 문서에서 `metaTag` 요소가 여러 번 나타날 수 있으며,
/// `trackchangeConfig` 전후로 흩어져 있을 수 있습니다.
/// quick-xml/serde는 이런 경우를 처리하지 못하므로, `metaTag`는 파싱하지 않습니다.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "head")]
pub struct Head {
    /// [AI 생성] 시작 번호 (`beginNum` 요소)
    #[serde(rename = "beginNum")]
    pub begin_number: BeginNumber,

    /// [AI 생성] 매핑 테이블 (`refList` 요소)
    #[serde(rename = "refList")]
    pub mapping_table: MappingTable,

    /// [AI 생성] 금칙 문자 목록 (`forbiddenWordList` 요소)
    #[serde(rename = "forbiddenWordList", skip_serializing_if = "Option::is_none")]
    pub forbidden_word_list: Option<ForbiddenWordList>,

    /// [AI 생성] 문서 호환성 정보 (`compatibleDocument` 요소)
    #[serde(rename = "compatibleDocument", skip_serializing_if = "Option::is_none")]
    pub compatible_document: Option<CompatibleDocument>,

    /// [AI 생성] 문서 옵션 (`docOption` 요소)
    #[serde(rename = "docOption", skip_serializing_if = "Option::is_none")]
    pub document_option: Option<DocumentOption>,

    /// [AI 생성] 변경 추적 설정 (`trackchangeConfig` 요소, 일부 문서에서는 `trackchageConfig` 오타로 사용됨)
    #[serde(
        rename = "trackchangeConfig",
        alias = "trackchageConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub track_change_config: Option<TrackChangeConfig>,

    /// [AI 생성] 버전 (`version` 속성)
    #[serde(rename = "@version")]
    pub version: String,

    /// [AI 생성] 구역 개수 (`secCnt` 속성)
    #[serde(rename = "@secCnt")]
    pub section_count: u32,
}
