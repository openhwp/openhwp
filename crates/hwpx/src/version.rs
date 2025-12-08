//! [AI 생성 문서화] 버전 정보
//!
//! 패키지 내 `version.xml`에 기록되는 HCF/HWPX 버전 메타데이터입니다. 뷰어·편집기가 호환성을 판단할 때 참고하므로 대상 애플리케이션과 버전 문자열을 일관되게 유지해야 합니다. KS X 6101:2024 `version.xsd`를 근거로 합니다.

use serde::{Deserialize, Serialize};

/// [AI 생성] 대상 애플리케이션 종류
///
/// 원본: `tagetApplication` 속성의 익명 타입. 패키지를 주로 소비하는 제품군(워드/프레젠테이션/스프레드시트)을 표시해 호환 기능을 안내합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetApplication {
    /// 워드프로세서
    #[serde(rename = "WORDPROCESSOR")]
    WordProcessor,
    /// 프레젠테이션
    #[serde(rename = "PRESENTATION")]
    Presentation,
    /// 스프레드시트
    #[serde(rename = "SPREADSHEET")]
    Spreadsheet,
}

/// [AI 생성] HCF 버전 정보
///
/// 한글 파일 컨테이너(HCF)의 버전 정보를 표현하며 `version.xml` 루트와 매핑됩니다. 대상 제품군, 버전·빌드, OS 플래그, 작성 앱 문자열을 묶어 재현성 있는 패키징을 돕습니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "HCFVersion")]
pub struct HcfVersion {
    /// [AI 생성] 대상 애플리케이션 (`tagetApplication` 속성, 제품군 식별)
    #[serde(rename = "@tagetApplication")]
    pub target_application: TargetApplication,

    /// [AI 생성] 메이저 버전 번호 (`major` 속성). 호환성 단절 여부를 판단하는 핵심 값입니다.
    #[serde(rename = "@major")]
    pub major: u32,

    /// [AI 생성] 마이너 버전 번호 (`minor` 속성). 메이저 내부의 사양 변화에 대응합니다.
    #[serde(rename = "@minor")]
    pub minor: u32,

    /// [AI 생성] 마이크로/패치 버전 (`micro` 속성, 기본 0). 단순 패치 수준을 나타냅니다.
    #[serde(rename = "@micro", default)]
    pub micro: i32,

    /// [AI 생성] 빌드 번호 (`buildNumber` 속성). 생성 환경을 추적하는 보조 지표입니다.
    #[serde(rename = "@buildNumber", default)]
    pub build_number: u32,

    /// [AI 생성] OS 플래그 (`os` 속성). 작성 시점 실행 환경을 구분하는 비트마스크입니다.
    #[serde(rename = "@os", default)]
    pub os: u32,

    /// [AI 생성] XML 버전 문자열 (`xmlVersion` 속성, 선택). 패키지 내부 XML 문서 버전.
    #[serde(rename = "@xmlVersion", skip_serializing_if = "Option::is_none")]
    pub xml_version: Option<String>,

    /// [AI 생성] 애플리케이션 이름 (`application` 속성, 선택). 예: "Hancom Word".
    #[serde(rename = "@application", skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,

    /// [AI 생성] 애플리케이션 버전 문자열 (`appVersion` 속성, 선택). 빌드 번호와 함께 생성 환경을 복원하는 데 사용합니다.
    #[serde(rename = "@appVersion", skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
}
