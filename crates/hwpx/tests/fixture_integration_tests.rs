//! Fixture 파일 통합 테스트
//!
//! 실제 fixture 디렉토리에서 XML 파일들을 읽어 파싱합니다.
//! 모든 fixture 파일들이 오류 없이 파싱되는지 확인합니다.

use quick_xml::de::from_str;
use std::fs;
use std::path::Path;

/// fixture 디렉토리 경로를 반환
fn fixtures_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .leak()
}

mod version_file_tests {
    use super::*;
    use hwpx::version::HcfVersion;

    #[test]
    fn parse_all_version_files() {
        let fixtures = fixtures_dir();

        for i in 0..=12 {
            let path = fixtures.join(format!("sample{}/version.xml", i));
            if !path.exists() {
                continue;
            }

            let content = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("sample{}/version.xml 읽기 실패: {}", i, e));

            let result: Result<HcfVersion, _> = from_str(&content);
            assert!(
                result.is_ok(),
                "sample{}/version.xml 파싱 실패: {:?}",
                i,
                result.err()
            );
        }
    }
}

mod header_file_tests {
    use super::*;
    use hwpx::header::Head;

    #[test]
    fn parse_all_header_files() {
        let fixtures = fixtures_dir();

        for i in 0..=12 {
            let path = fixtures.join(format!("sample{}/Contents/header.xml", i));
            if !path.exists() {
                continue;
            }

            let content = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("sample{}/Contents/header.xml 읽기 실패: {}", i, e));

            let result: Result<Head, _> = from_str(&content);
            assert!(
                result.is_ok(),
                "sample{}/Contents/header.xml 파싱 실패: {:?}",
                i,
                result.err()
            );
        }
    }
}

mod color_format_tests {
    use hwpx::core::types::RgbColor;

    /// fixture에서 발견된 ARGB 형식 색상 테스트
    #[test]
    fn argb_color_from_fixture() {
        // fixture에서 발견된 색상들
        let colors = [
            ("#FF000000", 0, 0, 0, 255),     // 불투명한 검은색
            ("#000000", 0, 0, 0, 255),       // 검은색 (6자리)
            ("#FFFFFF", 255, 255, 255, 255), // 흰색
            ("#999999", 153, 153, 153, 255), // 회색
            ("#B2B2B2", 178, 178, 178, 255), // 밝은 회색
            ("#E5E5CB", 229, 229, 203, 255), // 베이지
        ];

        for (hex, expected_r, expected_g, expected_b, expected_a) in colors {
            let color =
                RgbColor::from_hex(hex).unwrap_or_else(|| panic!("색상 파싱 실패: {}", hex));

            assert_eq!(
                (color.r, color.g, color.b, color.a),
                (expected_r, expected_g, expected_b, expected_a),
                "색상 {} 파싱 결과가 예상과 다름",
                hex
            );
        }
    }
}

mod line_segment_tests {
    use super::*;
    use hwpx::paragraph::{LineSegment, LineSegmentArray};

    /// fixture에서 발견된 linesegarray 파싱 테스트
    #[test]
    fn parse_lineseg_from_fixture() {
        // sample0/section0.xml에서 추출한 실제 데이터
        let xml = r#"<hp:lineseg xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" textpos="0" vertpos="0" vertsize="1500" textheight="1500" baseline="1275" spacing="-1500" horzpos="0" horzsize="48188" flags="393216"/>"#;

        let result: Result<LineSegment, _> = from_str(xml);
        assert!(result.is_ok(), "LineSegment 파싱 실패: {:?}", result.err());

        let seg = result.unwrap();
        assert_eq!(seg.vertical_size, 1500);
        assert_eq!(seg.text_height, 1500);
        assert_eq!(seg.baseline, 1275);
        assert_eq!(seg.spacing, -1500);
        assert_eq!(seg.horizontal_size, 48188);
        assert_eq!(seg.flags, 393216);
    }

    #[test]
    fn parse_linesegarray_multiple_segments() {
        // 여러 줄이 있는 문단의 linesegarray
        let xml = r#"<hp:linesegarray xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">
            <hp:lineseg textpos="0" vertpos="0" vertsize="1400" textheight="1400" baseline="1190" spacing="840" horzpos="0" horzsize="48188" flags="393216"/>
            <hp:lineseg textpos="40" vertpos="2240" vertsize="1400" textheight="1400" baseline="1190" spacing="840" horzpos="0" horzsize="48188" flags="393216"/>
            <hp:lineseg textpos="78" vertpos="4480" vertsize="1400" textheight="1400" baseline="1190" spacing="840" horzpos="0" horzsize="48188" flags="393216"/>
        </hp:linesegarray>"#;

        let result: Result<LineSegmentArray, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "LineSegmentArray 파싱 실패: {:?}",
            result.err()
        );

        let arr = result.unwrap();
        assert_eq!(arr.segments.len(), 3);

        // 각 줄의 text_position 검증
        assert_eq!(arr.segments[0].text_position, 0);
        assert_eq!(arr.segments[1].text_position, 40);
        assert_eq!(arr.segments[2].text_position, 78);

        // 각 줄의 vertical_position 검증 (줄 간격)
        assert_eq!(arr.segments[0].vertical_position, 0);
        assert_eq!(arr.segments[1].vertical_position, 2240);
        assert_eq!(arr.segments[2].vertical_position, 4480);
    }
}

mod section_file_tests {
    use super::*;
    use hwpx::paragraph::Section;

    /// 모든 sample의 section 파일 파싱 테스트
    ///
    /// 각 sample 디렉토리에서 모든 section*.xml 파일을 파싱합니다.
    /// 모든 파일이 오류 없이 파싱되어야 합니다.
    #[test]
    fn parse_all_section_files() {
        let fixtures = fixtures_dir();

        for i in 0.. {
            let contents_dir = fixtures.join(format!("sample{}/Contents", i));
            if !contents_dir.exists() {
                break;
            }

            // section0.xml, section1.xml, ... 모두 찾기
            for entry in fs::read_dir(&contents_dir).unwrap() {
                let entry = entry.unwrap();
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                if file_name_str.starts_with("section") && file_name_str.ends_with(".xml") {
                    let path = entry.path();
                    let content = fs::read_to_string(&path).unwrap_or_else(|e| {
                        panic!("sample{}/{} 읽기 실패: {}", i, file_name_str, e)
                    });

                    let result: Result<Section, _> = from_str(&content);
                    assert!(
                        result.is_ok(),
                        "sample{}/{} 파싱 실패: {:?}",
                        i,
                        file_name_str,
                        result.err()
                    );
                }
            }
        }
    }
}

mod section_definition_tests {
    use super::*;
    use hwpx::paragraph::SectionDefinition;

    /// sample5에서 추출한 실제 secPr 파싱 테스트
    #[test]
    fn parse_section_definition_from_fixture() {
        // sample5/section0.xml에서 추출한 실제 데이터
        let xml = r##"<hp:secPr xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="" textDirection="HORIZONTAL" spaceColumns="1134" tabStop="8000" tabStopVal="4000" tabStopUnit="HWPUNIT" outlineShapeIDRef="1" memoShapeIDRef="0" textVerticalWidthHead="0" masterPageCnt="2">
            <hp:grid lineGrid="0" charGrid="0" wonggojiFormat="0"/>
            <hp:startNum pageStartsOn="BOTH" page="0" pic="0" tbl="0" equation="0"/>
            <hp:visibility hideFirstHeader="0" hideFirstFooter="0" hideFirstMasterPage="1" border="SHOW_ALL" fill="SHOW_ALL" hideFirstPageNum="0" hideFirstEmptyLine="0" showLineNumber="0"/>
            <hp:lineNumberShape restartType="0" countBy="0" distance="0" startNumber="0"/>
            <hp:pagePr landscape="WIDELY" width="59528" height="84188" gutterType="LEFT_ONLY">
                <hp:margin header="4251" footer="4251" gutter="0" left="9921" right="9921" top="9921" bottom="9921"/>
            </hp:pagePr>
            <hp:footNotePr>
                <hp:autoNumFormat type="DIGIT" userChar="" prefixChar="" suffixChar=")" supscript="0"/>
                <hp:noteLine length="-1" type="SOLID" width="0.12 mm" color="#000000"/>
                <hp:noteSpacing betweenNotes="283" belowLine="567" aboveLine="850"/>
                <hp:numbering type="CONTINUOUS" newNum="1"/>
                <hp:placement place="EACH_COLUMN" beneathText="0"/>
            </hp:footNotePr>
            <hp:endNotePr>
                <hp:autoNumFormat type="DIGIT" userChar="" prefixChar="" suffixChar=")" supscript="0"/>
                <hp:noteLine length="14692344" type="SOLID" width="0.12 mm" color="#000000"/>
                <hp:noteSpacing betweenNotes="0" belowLine="567" aboveLine="850"/>
                <hp:numbering type="CONTINUOUS" newNum="1"/>
                <hp:placement place="END_OF_DOCUMENT" beneathText="0"/>
            </hp:endNotePr>
            <hp:pageBorderFill type="BOTH" borderFillIDRef="0" textBorder="PAPER" headerInside="0" footerInside="0" fillArea="PAPER">
                <hp:offset left="1417" right="1417" top="1417" bottom="1417"/>
            </hp:pageBorderFill>
            <hp:masterPage idRef="masterpage0"/>
            <hp:masterPage idRef="masterpage1"/>
        </hp:secPr>"##;

        let result: Result<SectionDefinition, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "fixture 기반 secPr 파싱 실패: {:?}",
            result.err()
        );

        let sec_def = result.unwrap();

        // 속성 검증
        assert_eq!(sec_def.space_columns, 1134);
        assert_eq!(sec_def.tab_stop_value, 4000);
        assert_eq!(sec_def.master_page_count, 2);

        // 용지 설정 검증
        assert!(sec_def.page_property.is_some());
        let page_prop = sec_def.page_property.as_ref().unwrap();
        assert_eq!(page_prop.width, 59528);
        assert_eq!(page_prop.height, 84188);
        assert_eq!(page_prop.margin.left, 9921);
        assert_eq!(page_prop.margin.header, 4251);

        // 각주 설정 검증
        assert!(sec_def.footnote_shape.is_some());

        // 미주 설정 검증
        assert!(sec_def.endnote_shape.is_some());

        // 바탕쪽 참조 검증
        assert_eq!(sec_def.master_pages.len(), 2);

        // 페이지 테두리 검증
        assert!(!sec_def.page_border_fills.is_empty());
    }
}

mod page_number_control_tests {
    use super::*;
    use hwpx::paragraph::Control;

    /// sample0에서 발견한 페이지 번호 컨트롤 파싱 테스트
    #[test]
    fn parse_page_number_control() {
        let xml = r#"<hp:ctrl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph">
            <hp:pageNum pos="BOTTOM_CENTER" formatType="DIGIT" sideChar="-"/>
        </hp:ctrl>"#;

        let result: Result<Control, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "pageNum 컨트롤 파싱 실패: {:?}",
            result.err()
        );
    }
}
