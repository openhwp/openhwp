//! 버전 정보 파싱 테스트

use quick_xml::de::from_str;
use hwpx::version::{HcfVersion, TargetApplication};

const VERSION_XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes" ?><hv:HCFVersion xmlns:hv="http://www.hancom.co.kr/hwpml/2011/version" tagetApplication="WORDPROCESSOR" major="5" minor="1" micro="1" buildNumber="0" os="1" xmlVersion="1.5" application="Hancom Office Hangul" appVersion="12, 0, 0, 3650 WIN32LEWindows_10"/>"#;

#[test]
fn parse_version_xml_from_string() {
    let result: Result<HcfVersion, _> = from_str(VERSION_XML);

    assert!(
        result.is_ok(),
        "version.xml 파싱 실패: {:?}",
        result.err()
    );

    let version = result.unwrap();
    assert_eq!(version.target_application, TargetApplication::WordProcessor);
    assert_eq!(version.major, 5);
    assert_eq!(version.minor, 1);
    assert_eq!(version.micro, 1);
    assert_eq!(version.build_number, 0);
    assert_eq!(version.os, 1);
    assert_eq!(version.xml_version, Some("1.5".to_string()));
    assert_eq!(
        version.application,
        Some("Hancom Office Hangul".to_string())
    );
    assert_eq!(
        version.application_version,
        Some("12, 0, 0, 3650 WIN32LEWindows_10".to_string())
    );
}

#[test]
fn version_presentation() {
    let xml = r#"<hv:HCFVersion xmlns:hv="http://www.hancom.co.kr/hwpml/2011/version" tagetApplication="PRESENTATION" major="1" minor="0" micro="0" buildNumber="0"/>"#;
    
    let result: Result<HcfVersion, _> = from_str(xml);
    assert!(result.is_ok());
    
    let version = result.unwrap();
    assert_eq!(version.target_application, TargetApplication::Presentation);
}

#[test]
fn version_spreadsheet() {
    let xml = r#"<hv:HCFVersion xmlns:hv="http://www.hancom.co.kr/hwpml/2011/version" tagetApplication="SPREADSHEET" major="1" minor="0" micro="0" buildNumber="0"/>"#;
    
    let result: Result<HcfVersion, _> = from_str(xml);
    assert!(result.is_ok());
    
    let version = result.unwrap();
    assert_eq!(version.target_application, TargetApplication::Spreadsheet);
}
