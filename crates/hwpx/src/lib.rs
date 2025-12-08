//! [AI 생성 문서화] HWPX 파일 형식 지원
//!
//! KS X 6101:2024 표준에 따른 HWPX 문서 형식 구현. 본 문서화는 AI가 생성했으니 실제 표준과의 차이는 원문 스펙을 참고하세요.

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
