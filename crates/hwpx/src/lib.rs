//! # HWPX 파서 라이브러리
//!
//! HWPX (KS X 6101:2024) 문서를 파싱하고 생성하기 위한 Rust 라이브러리.
//!
//! ## 개요
//!
//! 이 라이브러리는 KS X 6101:2024 표준에 따른 HWPX XML 문서 형식을 지원합니다.
//! `serde`와 `quick-xml`을 사용하여 XML 직렬화/역직렬화를 제공합니다.
//!
//! ## 기능
//!
//! - KS X 6101:2024 스키마 지원
//! - 강타입 Rust 모델
//! - XML 직렬화/역직렬화
//!
//! ## 사용 예시
//!
//! ```ignore
//! use hwpx::version::HcfVersion;
//! use hwpx::header::Head;
//! use hwpx::paragraph::Section;
//!
//! // version.xml 파싱
//! let xml = std::fs::read_to_string("version.xml")?;
//! let version: HcfVersion = hwpx::from_str(&xml)?;
//!
//! // header.xml 파싱
//! let xml = std::fs::read_to_string("Contents/header.xml")?;
//! let head: Head = hwpx::from_str(&xml)?;
//!
//! // section.xml 파싱
//! let xml = std::fs::read_to_string("Contents/section0.xml")?;
//! let section: Section = hwpx::from_str(&xml)?;
//! ```
//!
//! ## 주요 모듈
//!
//! - [`core`]: 공통 속성, 열거형, 타입
//! - [`header`]: 문서 헤더 (글꼴, 스타일, 문단 모양 등)
//! - [`paragraph`]: 본문 (문단, 표, 그림, 컨트롤 등)
//! - [`master_page`]: 바탕쪽
//! - [`history`]: 변경 이력
//! - [`version`]: 버전 정보

#![deny(missing_docs)]

pub mod core;
pub mod header;
pub mod history;
pub mod master_page;
pub mod paragraph;
pub mod version;

#[inline]
/// 문자열 HWPX/XML을 지정 타입으로 역직렬화합니다. `quick_xml::de`를 래핑합니다.
pub fn from_str<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, quick_xml::DeError> {
    quick_xml::de::from_str(s)
}

#[inline]
/// 지정 값을 HWPX/XML 문자열로 직렬화합니다. `quick_xml::se`를 래핑합니다.
pub fn to_string<T: serde::ser::Serialize>(value: &T) -> Result<String, quick_xml::SeError> {
    quick_xml::se::to_string(value)
}
