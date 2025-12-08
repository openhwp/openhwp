//! Fixture 파일 파싱 테스트
//!
//! 실제 HWPX 파일의 XML 파일들을 파싱하여 구조체로 변환되는지 확인합니다.
//! 스냅샷 테스트를 사용하여 파싱 결과의 일관성을 검증합니다.

use quick_xml::de::from_str;

mod version_tests {
    use super::*;
    use hwpx::version::HcfVersion;

    const VERSION_XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes" ?><hv:HCFVersion xmlns:hv="http://www.hancom.co.kr/hwpml/2011/version" tagetApplication="WORDPROCESSOR" major="5" minor="1" micro="1" buildNumber="0" os="1" xmlVersion="1.5" application="Hancom Office Hangul" appVersion="12, 0, 0, 3650 WIN32LEWindows_10"/>"#;

    #[test]
    fn parse_version_xml_from_fixture() {
        let result: Result<HcfVersion, _> = from_str(VERSION_XML);
        assert!(result.is_ok(), "version.xml 파싱 실패: {:?}", result.err());

        let version = result.unwrap();
        insta::assert_yaml_snapshot!(version);
    }
}

mod paragraph_tests {
    use super::*;
    use hwpx::paragraph::Paragraph;

    #[test]
    fn parse_simple_paragraph() {
        let xml = r#"<hp:p xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="0" paraPrIDRef="44" styleIDRef="0" pageBreak="0" columnBreak="0" merged="0"></hp:p>"#;

        let result: Result<Paragraph, _> = from_str(xml);
        assert!(result.is_ok(), "간단한 문단 파싱 실패: {:?}", result.err());

        let paragraph = result.unwrap();
        insta::assert_yaml_snapshot!(paragraph);
    }

    #[test]
    fn parse_paragraph_with_text() {
        // 일반적인 샘플 텍스트 사용
        let xml = r#"<hp:p xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="2147483648" paraPrIDRef="39" styleIDRef="0" pageBreak="0" columnBreak="0" merged="0"><hp:run charPrIDRef="39"><hp:t>Sample Title</hp:t></hp:run></hp:p>"#;

        let result: Result<Paragraph, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "텍스트가 있는 문단 파싱 실패: {:?}",
            result.err()
        );

        let paragraph = result.unwrap();
        insta::assert_yaml_snapshot!(paragraph);
    }
}

mod run_tests {
    use super::*;
    use hwpx::paragraph::Run;

    #[test]
    fn parse_simple_run_with_text() {
        let xml = r#"<hp:run xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" charPrIDRef="39"><hp:t>Sample Text</hp:t></hp:run>"#;

        let result: Result<Run, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "텍스트가 있는 런 파싱 실패: {:?}",
            result.err()
        );

        let run = result.unwrap();
        insta::assert_yaml_snapshot!(run);
    }

    #[test]
    fn parse_empty_run() {
        let xml = r#"<hp:run xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" charPrIDRef="30"/>"#;

        let result: Result<Run, _> = from_str(xml);
        assert!(result.is_ok(), "빈 런 파싱 실패: {:?}", result.err());

        let run = result.unwrap();
        insta::assert_yaml_snapshot!(run);
    }
}

mod text_tests {
    use super::*;
    use hwpx::paragraph::TextElement;

    #[test]
    fn parse_text_element() {
        let xml = r#"<hp:t xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">Sample Text</hp:t>"#;

        let result: Result<TextElement, _> = from_str(xml);
        assert!(result.is_ok(), "텍스트 요소 파싱 실패: {:?}", result.err());

        let text = result.unwrap();
        insta::assert_yaml_snapshot!(text);
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

        let text = result.unwrap();
        insta::assert_yaml_snapshot!(text);
    }
}

mod table_tests {
    use super::*;
    use hwpx::paragraph::Table;

    #[test]
    fn parse_simple_table_attributes() {
        // 기본 테이블 속성만 테스트
        let xml = r#"<hp:tbl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="1854889781" zOrder="2" numberingType="TABLE" textWrap="TOP_AND_BOTTOM" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" pageBreak="CELL" repeatHeader="1" rowCnt="1" colCnt="3" cellSpacing="0" borderFillIDRef="2" noAdjust="1">
            <hp:sz width="47697" widthRelTo="ABSOLUTE" height="2994" heightRelTo="ABSOLUTE" protect="0"/>
            <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="0" holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
            <hp:outMargin left="283" right="283" top="283" bottom="283"/>
            <hp:inMargin left="141" right="141" top="141" bottom="141"/>
        </hp:tbl>"#;

        let result: Result<Table, _> = from_str(xml);
        assert!(result.is_ok(), "간단한 테이블 파싱 실패: {:?}", result.err());

        let table = result.unwrap();
        insta::assert_yaml_snapshot!(table);
    }
}

mod section_definition_tests {
    use super::*;
    use hwpx::paragraph::SectionDefinition;

    #[test]
    fn parse_section_definition_attributes() {
        let xml = r#"<hp:secPr xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="" textDirection="HORIZONTAL" spaceColumns="1134" tabStop="8000" tabStopVal="4000" tabStopUnit="HWPUNIT" outlineShapeIDRef="1" memoShapeIDRef="1" textVerticalWidthHead="0" masterPageCnt="0"></hp:secPr>"#;

        let result: Result<SectionDefinition, _> = from_str(xml);
        assert!(result.is_ok(), "구역 정의 파싱 실패: {:?}", result.err());

        let sec_def = result.unwrap();
        insta::assert_yaml_snapshot!(sec_def);
    }
}

mod picture_tests {
    use super::*;
    use hwpx::paragraph::Picture;

    #[test]
    fn parse_picture_attributes() {
        let xml = r#"<hp:pic xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core" id="1475691556" zOrder="3" numberingType="PICTURE" textWrap="TOP_AND_BOTTOM" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="0" instid="401949733" reverse="0">
            <hp:offset x="4294937695" y="4294967295"/>
            <hp:orgSz width="59880" height="11400"/>
            <hp:curSz width="9749" height="2567"/>
            <hp:flip horizontal="0" vertical="0"/>
            <hp:rotationInfo angle="0" centerX="4874" centerY="1283" rotateimage="0"/>
            <hp:renderingInfo>
                <hc:transMatrix e1="1" e2="0" e3="-29601" e4="0" e5="1" e6="-1"/>
                <hc:scaMatrix e1="0.162809" e2="0" e3="29601" e4="0" e5="0.225175" e6="1"/>
                <hc:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
            </hp:renderingInfo>
            <hc:img binaryItemIDRef="image1" bright="0" contrast="0" effect="REAL_PIC" alpha="0"/>
        </hp:pic>"#;

        let result: Result<Picture, _> = from_str(xml);
        assert!(result.is_ok(), "그림 파싱 실패: {:?}", result.err());

        let picture = result.unwrap();
        insta::assert_yaml_snapshot!(picture);
    }

    #[test]
    fn parse_full_picture_from_fixture() {
        // fixture에서 추출한 전체 그림 요소 - 일반적인 이미지 설명 사용
        let xml = r#"<hp:pic xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core" id="1475691556" zOrder="3" numberingType="PICTURE" textWrap="TOP_AND_BOTTOM" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="0" instid="401949733" reverse="0">
            <hp:offset x="4294937695" y="4294967295"/>
            <hp:orgSz width="59880" height="11400"/>
            <hp:curSz width="9749" height="2567"/>
            <hp:flip horizontal="0" vertical="0"/>
            <hp:rotationInfo angle="0" centerX="4874" centerY="1283" rotateimage="0"/>
            <hp:renderingInfo>
                <hc:transMatrix e1="1" e2="0" e3="-29601" e4="0" e5="1" e6="-1"/>
                <hc:scaMatrix e1="0.162809" e2="0" e3="29601" e4="0" e5="0.225175" e6="1"/>
                <hc:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
            </hp:renderingInfo>
            <hc:img binaryItemIDRef="image1" bright="0" contrast="0" effect="REAL_PIC" alpha="0"/>
            <hp:imgRect>
                <hc:pt0 x="0" y="0"/>
                <hc:pt1 x="59880" y="0"/>
                <hc:pt2 x="59880" y="11400"/>
                <hc:pt3 x="0" y="11400"/>
            </hp:imgRect>
            <hp:imgClip left="524" right="15726" top="0" bottom="3960"/>
            <hp:inMargin left="0" right="0" top="0" bottom="0"/>
            <hp:imgDim dimwidth="16260" dimheight="3960"/>
            <hp:effects/>
            <hp:sz width="9749" widthRelTo="ABSOLUTE" height="2567" heightRelTo="ABSOLUTE" protect="0"/>
            <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="0" holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
            <hp:outMargin left="0" right="0" top="0" bottom="0"/>
            <hp:shapeComment>This is an image.
Original name: sample_logo.jpg
Original size: 598x114 pixels</hp:shapeComment>
        </hp:pic>"#;

        let result: Result<Picture, _> = from_str(xml);
        assert!(result.is_ok(), "전체 그림 파싱 실패: {:?}", result.err());

        let picture = result.unwrap();
        insta::assert_yaml_snapshot!(picture);
    }
}

mod drawing_tests {
    use super::*;
    use hwpx::paragraph::Rectangle;

    #[test]
    fn parse_rectangle() {
        let xml = r##"<hp:rect xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core" id="1712398788" zOrder="4" numberingType="PICTURE" textWrap="TOP_AND_BOTTOM" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="0" instid="638656965" ratio="0">
            <hp:offset x="612" y="4294966253"/>
            <hp:orgSz width="48150" height="3825"/>
            <hp:curSz width="48899" height="4905"/>
            <hp:flip horizontal="0" vertical="0"/>
            <hp:rotationInfo angle="0" centerX="24449" centerY="2452" rotateimage="0"/>
            <hp:renderingInfo>
                <hc:transMatrix e1="1" e2="0" e3="612" e4="0" e5="1" e6="-1043"/>
                <hc:scaMatrix e1="1.015556" e2="0" e3="-612" e4="0" e5="1.282353" e6="1043"/>
                <hc:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
            </hp:renderingInfo>
            <hp:lineShape color="#000000" width="33" style="NONE" endCap="FLAT" headStyle="NORMAL" tailStyle="NORMAL" headfill="1" tailfill="1" headSz="SMALL_SMALL" tailSz="SMALL_SMALL" outlineStyle="NORMAL" alpha="0"/>
            <hc:fillBrush>
                <hc:winBrush faceColor="#FFFFFF" hatchColor="#000000" alpha="0"/>
            </hc:fillBrush>
            <hp:shadow type="NONE" color="#B2B2B2" offsetX="0" offsetY="0" alpha="0"/>
            <hc:pt0 x="0" y="0"/>
            <hc:pt1 x="48150" y="0"/>
            <hc:pt2 x="48150" y="3825"/>
            <hc:pt3 x="0" y="3825"/>
            <hp:sz width="48899" widthRelTo="ABSOLUTE" height="4905" heightRelTo="ABSOLUTE" protect="0"/>
            <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="0" allowOverlap="1" holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
            <hp:outMargin left="0" right="0" top="0" bottom="0"/>
            <hp:shapeComment>This is a rectangle.</hp:shapeComment>
        </hp:rect>"##;

        let result: Result<Rectangle, _> = from_str(xml);
        assert!(result.is_ok(), "사각형 파싱 실패: {:?}", result.err());

        let rect = result.unwrap();
        insta::assert_yaml_snapshot!(rect);
    }
}

mod control_tests {
    use super::*;
    use hwpx::paragraph::Control;

    #[test]
    fn parse_column_control() {
        let xml = r#"<hp:ctrl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">
            <hp:colPr id="" type="NEWSPAPER" layout="LEFT" colCount="1" sameSz="1" sameGap="0"/>
        </hp:ctrl>"#;

        let result: Result<Control, _> = from_str(xml);
        assert!(result.is_ok(), "단 컨트롤 파싱 실패: {:?}", result.err());

        let control = result.unwrap();
        insta::assert_yaml_snapshot!(control);
    }

    #[test]
    fn parse_page_num_control() {
        let xml = r#"<hp:ctrl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">
            <hp:pageNum pos="BOTTOM_CENTER" formatType="DIGIT" sideChar="-"/>
        </hp:ctrl>"#;

        let result: Result<Control, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "페이지 번호 컨트롤 파싱 실패: {:?}",
            result.err()
        );

        let control = result.unwrap();
        insta::assert_yaml_snapshot!(control);
    }

    #[test]
    fn parse_field_begin_control() {
        // 일반적인 필드명 사용
        let xml = r##"<hp:ctrl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">
            <hp:fieldBegin id="1999385510" type="CLICK_HERE" name="content" editable="1" dirty="1" zorder="-1" fieldid="627272811" metaTag="">
                <hp:parameters cnt="4" name="">
                    <hp:integerParam name="Prop">9</hp:integerParam>
                </hp:parameters>
                <hp:metaTag>{"name":"#content"}</hp:metaTag>
            </hp:fieldBegin>
        </hp:ctrl>"##;

        let result: Result<Control, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "필드 시작 컨트롤 파싱 실패: {:?}",
            result.err()
        );

        let control = result.unwrap();
        insta::assert_yaml_snapshot!(control);
    }
}

mod table_with_content_tests {
    use super::*;
    use hwpx::paragraph::Table;

    #[test]
    fn parse_table_with_rows() {
        // 일반적인 샘플 텍스트 사용
        let xml = r#"<hp:tbl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="1854889786" zOrder="1" numberingType="TABLE" textWrap="TOP_AND_BOTTOM" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" pageBreak="CELL" repeatHeader="1" rowCnt="1" colCnt="4" cellSpacing="0" borderFillIDRef="2" noAdjust="1">
            <hp:sz width="32981" widthRelTo="ABSOLUTE" height="2582" heightRelTo="ABSOLUTE" protect="0"/>
            <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="0" holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
            <hp:outMargin left="283" right="283" top="283" bottom="283"/>
            <hp:inMargin left="141" right="141" top="141" bottom="141"/>
            <hp:tr>
                <hp:tc name="" header="0" hasMargin="0" protect="0" editable="0" dirty="0" borderFillIDRef="12">
                    <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="CENTER" linkListIDRef="0" linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0">
                        <hp:p id="2147483648" paraPrIDRef="51" styleIDRef="0" pageBreak="0" columnBreak="0" merged="0">
                            <hp:run charPrIDRef="37">
                                <hp:t>Release Date</hp:t>
                            </hp:run>
                        </hp:p>
                    </hp:subList>
                    <hp:cellAddr colAddr="0" rowAddr="0"/>
                    <hp:cellSpan colSpan="1" rowSpan="1"/>
                    <hp:cellSz width="5670" height="1282"/>
                    <hp:cellMargin left="510" right="510" top="141" bottom="141"/>
                </hp:tc>
            </hp:tr>
        </hp:tbl>"#;

        let result: Result<Table, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "행이 있는 테이블 파싱 실패: {:?}",
            result.err()
        );

        let table = result.unwrap();
        insta::assert_yaml_snapshot!(table);
    }
}

mod fill_brush_tests {
    use super::*;
    use hwpx::core::types::FillBrush;

    #[test]
    fn parse_fill_brush_with_windows_brush() {
        let xml = r##"<hc:fillBrush xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core">
            <hc:winBrush faceColor="#FFFFFF" hatchColor="#000000" alpha="0"/>
        </hc:fillBrush>"##;

        let result: Result<FillBrush, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "FillBrush 파싱 실패: {:?}",
            result.err()
        );

        let fill_brush = result.unwrap();
        insta::assert_yaml_snapshot!(fill_brush);
    }

    // TODO: "none" faceColor 테스트는 Color 타입이 "none" 값을 지원한 후 추가
    // parse_fill_brush_with_none_face_color, parse_fill_brush_with_alpha_color
}

mod border_fill_tests {
    use super::*;
    use hwpx::header::border_fill::BorderFill;

    #[test]
    fn parse_simple_border_fill() {
        let xml = r##"<hh:borderFill xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head" id="1" threeD="0" shadow="0" centerLine="NONE" breakCellSeparateLine="0">
            <hh:slash type="NONE" Crooked="0" isCounter="0"/>
            <hh:backSlash type="NONE" Crooked="0" isCounter="0"/>
            <hh:leftBorder type="NONE" width="0.1 mm" color="#000000"/>
            <hh:rightBorder type="NONE" width="0.1 mm" color="#000000"/>
            <hh:topBorder type="NONE" width="0.1 mm" color="#000000"/>
            <hh:bottomBorder type="NONE" width="0.1 mm" color="#000000"/>
            <hh:diagonal type="SOLID" width="0.1 mm" color="#000000"/>
        </hh:borderFill>"##;

        let result: Result<BorderFill, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "BorderFill 파싱 실패: {:?}",
            result.err()
        );

        let border_fill = result.unwrap();
        insta::assert_yaml_snapshot!(border_fill);
    }

    #[test]
    fn parse_border_fill_with_fill_brush() {
        let xml = r##"<hh:borderFill xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head" xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core" id="3" threeD="0" shadow="0" centerLine="NONE" breakCellSeparateLine="0">
            <hh:slash type="NONE" Crooked="0" isCounter="0"/>
            <hh:backSlash type="NONE" Crooked="0" isCounter="0"/>
            <hh:leftBorder type="NONE" width="0.1 mm" color="#000000"/>
            <hh:rightBorder type="NONE" width="0.1 mm" color="#000000"/>
            <hh:topBorder type="NONE" width="0.1 mm" color="#000000"/>
            <hh:bottomBorder type="NONE" width="0.1 mm" color="#000000"/>
            <hh:diagonal type="SOLID" width="0.1 mm" color="#000000"/>
            <hc:fillBrush>
                <hc:winBrush faceColor="#FFFFFF" hatchColor="#000000" alpha="0"/>
            </hc:fillBrush>
        </hh:borderFill>"##;

        let result: Result<BorderFill, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "FillBrush가 있는 BorderFill 파싱 실패: {:?}",
            result.err()
        );

        let border_fill = result.unwrap();
        insta::assert_yaml_snapshot!(border_fill);
    }

    #[test]
    fn parse_border_fill_with_solid_borders() {
        let xml = r##"<hh:borderFill xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head" id="2" threeD="0" shadow="0" centerLine="NONE" breakCellSeparateLine="0">
            <hh:slash type="NONE" Crooked="0" isCounter="0"/>
            <hh:backSlash type="NONE" Crooked="0" isCounter="0"/>
            <hh:leftBorder type="SOLID" width="0.12 mm" color="#000000"/>
            <hh:rightBorder type="SOLID" width="0.12 mm" color="#000000"/>
            <hh:topBorder type="SOLID" width="0.12 mm" color="#000000"/>
            <hh:bottomBorder type="SOLID" width="0.12 mm" color="#000000"/>
            <hh:diagonal type="SOLID" width="0.1 mm" color="#000000"/>
        </hh:borderFill>"##;

        let result: Result<BorderFill, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "실선 테두리 BorderFill 파싱 실패: {:?}",
            result.err()
        );

        let border_fill = result.unwrap();
        insta::assert_yaml_snapshot!(border_fill);
    }
}
