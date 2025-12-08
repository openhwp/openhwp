//! 문단 관련 파싱 테스트

use quick_xml::de::from_str;
use hwpx::paragraph::{LineSegment, LineSegmentArray, Paragraph, Run, TextElement, TextMarkup};

mod paragraph_tests {
    use super::*;

    #[test]
    fn parse_simple_paragraph() {
        let xml = r#"<hp:p xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="0" paraPrIDRef="44" styleIDRef="0" pageBreak="0" columnBreak="0" merged="0"></hp:p>"#;

        let result: Result<Paragraph, _> = from_str(xml);

        assert!(result.is_ok(), "간단한 문단 파싱 실패: {:?}", result.err());

        let paragraph = result.unwrap();
        assert_eq!(paragraph.id, 0);
        assert!(!paragraph.page_break);
        assert!(!paragraph.column_break);
        assert!(!paragraph.merged);
    }

    #[test]
    fn parse_paragraph_with_text() {
        let xml = r#"<hp:p xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="2147483648" paraPrIDRef="39" styleIDRef="0" pageBreak="0" columnBreak="0" merged="0"><hp:run charPrIDRef="39"><hp:t>보도자료</hp:t></hp:run></hp:p>"#;

        let result: Result<Paragraph, _> = from_str(xml);

        assert!(
            result.is_ok(),
            "텍스트가 있는 문단 파싱 실패: {:?}",
            result.err()
        );

        let paragraph = result.unwrap();
        assert_eq!(paragraph.id, 2147483648);
        assert_eq!(paragraph.runs.len(), 1);
    }

    #[test]
    fn parse_paragraph_with_page_break() {
        let xml = r#"<hp:p xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="1" pageBreak="1" columnBreak="0" merged="0"></hp:p>"#;

        let result: Result<Paragraph, _> = from_str(xml);
        assert!(result.is_ok());

        let paragraph = result.unwrap();
        assert!(paragraph.page_break);
    }

    #[test]
    fn parse_paragraph_with_column_break() {
        let xml = r#"<hp:p xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="1" pageBreak="0" columnBreak="1" merged="0"></hp:p>"#;

        let result: Result<Paragraph, _> = from_str(xml);
        assert!(result.is_ok());

        let paragraph = result.unwrap();
        assert!(paragraph.column_break);
    }
}

mod run_tests {
    use super::*;

    #[test]
    fn parse_simple_run_with_text() {
        let xml = r#"<hp:run xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" charPrIDRef="39"><hp:t>보도자료</hp:t></hp:run>"#;

        let result: Result<Run, _> = from_str(xml);

        assert!(
            result.is_ok(),
            "텍스트가 있는 런 파싱 실패: {:?}",
            result.err()
        );

        let run = result.unwrap();
        assert!(run.character_property_id_reference.is_some());
    }

    #[test]
    fn parse_empty_run() {
        let xml = r#"<hp:run xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" charPrIDRef="30"/>"#;

        let result: Result<Run, _> = from_str(xml);

        assert!(result.is_ok(), "빈 런 파싱 실패: {:?}", result.err());
    }

    #[test]
    fn parse_run_with_multiple_contents() {
        let xml = r#"<hp:run xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" charPrIDRef="39">
            <hp:t>첫 번째 텍스트</hp:t>
            <hp:t>두 번째 텍스트</hp:t>
        </hp:run>"#;

        let result: Result<Run, _> = from_str(xml);
        assert!(result.is_ok(), "여러 텍스트가 있는 런 파싱 실패: {:?}", result.err());
    }
}

mod text_tests {
    use super::*;

    #[test]
    fn parse_text_element() {
        let xml = r#"<hp:t xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">보도자료</hp:t>"#;

        let result: Result<TextElement, _> = from_str(xml);

        assert!(result.is_ok(), "텍스트 요소 파싱 실패: {:?}", result.err());

        let text = result.unwrap();
        assert_eq!(text.text(), "보도자료".to_string());
    }

    #[test]
    fn parse_empty_text_element() {
        let xml = r#"<hp:t xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph"/>"#;

        let result: Result<TextElement, _> = from_str(xml);

        assert!(
            result.is_ok(),
            "빈 텍스트 요소 파싱 실패: {:?}",
            result.err()
        );
    }

    #[test]
    fn parse_text_with_whitespace() {
        let xml = r#"<hp:t xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">  공백 포함 텍스트  </hp:t>"#;

        let result: Result<TextElement, _> = from_str(xml);
        assert!(result.is_ok());

        let text = result.unwrap();
        assert!(text.text().contains("공백 포함 텍스트"));
    }

    #[test]
    fn text_markup_extraction() {
        // TextMarkup::Text 변환 테스트
        let text_markup = TextMarkup::Text("테스트 텍스트".to_string());
        
        if let TextMarkup::Text(s) = text_markup {
            assert_eq!(s, "테스트 텍스트");
        } else {
            panic!("TextMarkup::Text 매칭 실패");
        }
    }
}

mod line_segment_tests {
    use super::*;

    #[test]
    fn parse_line_segment() {
        let xml = r#"<hp:lineseg xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" textpos="0" vertpos="0" vertsize="1500" textheight="1500" baseline="1275" spacing="-1500" horzpos="0" horzsize="48188" flags="393216"/>"#;

        let result: Result<LineSegment, _> = from_str(xml);
        assert!(result.is_ok(), "LineSegment 파싱 실패: {:?}", result.err());

        let seg = result.unwrap();
        assert_eq!(seg.text_position, 0);
        assert_eq!(seg.vertical_position, 0);
        assert_eq!(seg.vertical_size, 1500);
        assert_eq!(seg.text_height, 1500);
        assert_eq!(seg.baseline, 1275);
        assert_eq!(seg.spacing, -1500);
        assert_eq!(seg.horizontal_position, 0);
        assert_eq!(seg.horizontal_size, 48188);
        assert_eq!(seg.flags, 393216);
    }

    #[test]
    fn parse_line_segment_array() {
        let xml = r#"<hp:linesegarray xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">
            <hp:lineseg textpos="0" vertpos="0" vertsize="1400" textheight="1400" baseline="1190" spacing="840" horzpos="0" horzsize="48188" flags="393216"/>
            <hp:lineseg textpos="54" vertpos="2240" vertsize="1400" textheight="1400" baseline="1190" spacing="840" horzpos="0" horzsize="48188" flags="393216"/>
        </hp:linesegarray>"#;

        let result: Result<LineSegmentArray, _> = from_str(xml);
        assert!(result.is_ok(), "LineSegmentArray 파싱 실패: {:?}", result.err());

        let arr = result.unwrap();
        assert_eq!(arr.segments.len(), 2);
        assert_eq!(arr.segments[0].text_position, 0);
        assert_eq!(arr.segments[1].text_position, 54);
    }

    #[test]
    fn parse_paragraph_with_linesegarray() {
        let xml = r#"<hp:p xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="0" pageBreak="0" columnBreak="0" merged="0">
            <hp:run charPrIDRef="30">
                <hp:t>테스트</hp:t>
            </hp:run>
            <hp:linesegarray>
                <hp:lineseg textpos="0" vertpos="0" vertsize="1500" textheight="1500" baseline="1275" spacing="-1500" horzpos="0" horzsize="48188" flags="393216"/>
            </hp:linesegarray>
        </hp:p>"#;

        let result: Result<Paragraph, _> = from_str(xml);
        assert!(result.is_ok(), "linesegarray가 있는 문단 파싱 실패: {:?}", result.err());

        let para = result.unwrap();
        assert!(para.line_segments.is_some());
        
        let segments = para.line_segments.unwrap();
        assert_eq!(segments.segments.len(), 1);
        assert_eq!(segments.segments[0].horizontal_size, 48188);
    }
}
