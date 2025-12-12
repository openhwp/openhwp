//! HWPX 통합 문서 타입
//!
//! HWPX 패키지의 여러 XML 파일을 하나의 문서 타입으로 통합합니다.

use crate::header::Head;
use crate::master_page::MasterPage;
use crate::paragraph::Section;
use crate::version::HcfVersion;

/// HWPX 문서
///
/// HWPX 패키지 내의 모든 구성 요소를 통합하는 문서 타입입니다.
/// - `version.xml` → `version`
/// - `Contents/header.xml` → `header`
/// - `Contents/section*.xml` → `sections`
/// - `MasterPage/master*.xml` → `master_pages` (선택적)
#[derive(Debug, Clone)]
pub struct Document {
    /// 버전 정보 (version.xml)
    pub version: HcfVersion,

    /// 헤더 정보 (Contents/header.xml)
    pub header: Head,

    /// 섹션 목록 (Contents/section*.xml)
    pub sections: Vec<Section>,

    /// 마스터 페이지 목록 (MasterPage/master*.xml)
    pub master_pages: Vec<MasterPage>,

    /// 바이너리 데이터 (BinData/*)
    /// 키: 파일 경로 (예: "BinData/BIN0001.png")
    pub binary_data: std::collections::HashMap<String, Vec<u8>>,
}

impl Document {
    /// 기본 문서 생성
    pub fn new(version: HcfVersion, header: Head) -> Self {
        Self {
            version,
            header,
            sections: Vec::new(),
            master_pages: Vec::new(),
            binary_data: std::collections::HashMap::new(),
        }
    }

    /// 섹션 추가
    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }

    /// 마스터 페이지 추가
    pub fn add_master_page(&mut self, master_page: MasterPage) {
        self.master_pages.push(master_page);
    }

    /// 바이너리 데이터 추가
    pub fn add_binary_data(&mut self, path: String, data: Vec<u8>) {
        self.binary_data.insert(path, data);
    }
}
