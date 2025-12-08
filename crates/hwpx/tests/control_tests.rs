//! 컨트롤 및 필드 파싱 테스트

use quick_xml::de::from_str;
use hwpx::paragraph::Control;

mod column_control_tests {
    use super::*;

    #[test]
    fn parse_column_control() {
        let xml = r#"<hp:ctrl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">
            <hp:colPr id="" type="NEWSPAPER" layout="LEFT" colCount="1" sameSz="1" sameGap="0"/>
        </hp:ctrl>"#;

        let result: Result<Control, _> = from_str(xml);

        assert!(
            result.is_ok(),
            "단 컨트롤 파싱 실패: {:?}",
            result.err()
        );

        let ctrl = result.unwrap();
        assert!(!ctrl.items.is_empty());
    }

    #[test]
    fn parse_column_control_with_sizes() {
        let xml = r#"<hp:ctrl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">
            <hp:colPr id="0" type="NEWSPAPER" layout="LEFT" colCount="2" sameSz="1" sameGap="1">
                <hp:colSz width="20000" gap="500"/>
                <hp:colSz width="20000" gap="500"/>
            </hp:colPr>
        </hp:ctrl>"#;

        let result: Result<Control, _> = from_str(xml);
        assert!(result.is_ok());
        
        let ctrl = result.unwrap();
        assert_eq!(ctrl.items.len(), 1);
    }
}

mod field_control_tests {
    use super::*;

    #[test]
    fn parse_field_begin_control() {
        let xml = r##"<hp:ctrl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">
            <hp:fieldBegin id="1999385510" type="CLICK_HERE" name="본문" editable="1" dirty="1" zorder="-1" fieldid="627272811" metaTag="">
                <hp:parameters cnt="4" name="">
                    <hp:integerParam name="Prop">9</hp:integerParam>
                </hp:parameters>
                <hp:metaTag>{"name":"#본문"}</hp:metaTag>
            </hp:fieldBegin>
        </hp:ctrl>"##;

        let result: Result<Control, _> = from_str(xml);

        assert!(
            result.is_ok(),
            "필드 시작 컨트롤 파싱 실패: {:?}",
            result.err()
        );

        let ctrl = result.unwrap();
        assert!(!ctrl.items.is_empty());
    }

    #[test]
    fn parse_field_end_control() {
        let xml = r#"<hp:ctrl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">
            <hp:fieldEnd id="0" type="UNKNOWN" editable="0" dirty="0" zorder="0" fieldid="100" beginIDRef="1"/>
        </hp:ctrl>"#;

        let result: Result<Control, _> = from_str(xml);

        assert!(
            result.is_ok(),
            "필드 끝 컨트롤 파싱 실패: {:?}",
            result.err()
        );
    }

    #[test]
    fn parse_bookmark_control() {
        let xml = r#"<hp:ctrl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">
            <hp:bookmark id="0" name="TestBookmark"/>
        </hp:ctrl>"#;

        let result: Result<Control, _> = from_str(xml);

        assert!(
            result.is_ok(),
            "북마크 컨트롤 파싱 실패: {:?}",
            result.err()
        );
    }
}

// 각주/미주는 복잡한 구조로 인해 별도 테스트 파일에서 fixture 기반으로 테스트
