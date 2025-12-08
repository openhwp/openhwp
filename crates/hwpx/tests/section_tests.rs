//! 섹션 정의 파싱 테스트

use quick_xml::de::from_str;
use hwpx::paragraph::SectionDefinition;

mod section_definition_tests {
    use super::*;

    #[test]
    fn parse_section_definition_attributes() {
        let xml = r#"<hp:secPr xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="" textDirection="HORIZONTAL" spaceColumns="1134" tabStop="8000" tabStopVal="4000" tabStopUnit="HWPUNIT" outlineShapeIDRef="1" memoShapeIDRef="1" textVerticalWidthHead="0" masterPageCnt="0"></hp:secPr>"#;

        let result: Result<SectionDefinition, _> = from_str(xml);

        assert!(
            result.is_ok(),
            "구역 정의 파싱 실패: {:?}",
            result.err()
        );

        let sec_def = result.unwrap();
        assert_eq!(sec_def.space_columns, 1134);
    }

    #[test]
    fn parse_section_definition_minimal() {
        let xml = r#"<hp:secPr xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="" textDirection="HORIZONTAL" spaceColumns="0" tabStopVal="4000"></hp:secPr>"#;

        let result: Result<SectionDefinition, _> = from_str(xml);

        assert!(
            result.is_ok(),
            "최소 구역 정의 파싱 실패: {:?}",
            result.err()
        );
    }

    #[test]
    fn parse_section_definition_with_various_attributes() {
        let xml = r#"<hp:secPr xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="section1" textDirection="VERTICAL" spaceColumns="500" tabStopVal="8000" tabStopUnit="HWPUNIT" outlineShapeIDRef="2" memoShapeIDRef="3" textVerticalWidthHead="1" masterPageCnt="2"></hp:secPr>"#;

        let result: Result<SectionDefinition, _> = from_str(xml);

        assert!(
            result.is_ok(),
            "다양한 속성 구역 정의 파싱 실패: {:?}",
            result.err()
        );

        let sec_def = result.unwrap();
        assert_eq!(sec_def.space_columns, 500);
        assert_eq!(sec_def.tab_stop_value, 8000);
    }
}
