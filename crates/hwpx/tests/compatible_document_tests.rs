//! 문서 호환성 관련 타입 테스트
//!
//! - CompatibleDocument 테스트
//! - LayoutCompatibility 테스트
//! - TargetProgram 테스트

use hwpx::header::compatible_document::{CompatibleDocument, LayoutCompatibility, TargetProgram};

mod target_program_tests {
    use super::*;

    #[test]
    fn default_is_hwp201x() {
        let program = TargetProgram::default();
        assert_eq!(program, TargetProgram::Hwp201X);
    }

    #[test]
    fn deserialize_hwp201x() {
        let xml = r#"<compatibleDocument targetProgram="HWP201X"><layoutCompatibility/></compatibleDocument>"#;
        let doc: CompatibleDocument = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(doc.target_program, TargetProgram::Hwp201X);
    }

    #[test]
    fn deserialize_hwp200x() {
        let xml = r#"<compatibleDocument targetProgram="HWP200X"><layoutCompatibility/></compatibleDocument>"#;
        let doc: CompatibleDocument = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(doc.target_program, TargetProgram::Hwp200X);
    }

    #[test]
    fn deserialize_ms_word() {
        let xml = r#"<compatibleDocument targetProgram="MS_WORD"><layoutCompatibility/></compatibleDocument>"#;
        let doc: CompatibleDocument = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(doc.target_program, TargetProgram::MsWord);
    }

    #[test]
    fn serialize_variants() {
        let variants = vec![
            TargetProgram::Hwp201X,
            TargetProgram::Hwp200X,
            TargetProgram::MsWord,
        ];
        insta::assert_yaml_snapshot!("target_program_variants", variants);
    }
}

mod layout_compatibility_tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        let layout = LayoutCompatibility::default();
        assert!(layout.apply_font_weight_to_bold.is_none());
        assert!(layout.use_inner_underline.is_none());
        assert!(layout.fixed_underline_width.is_none());
    }

    #[test]
    fn deserialize_empty() {
        let xml = r#"<layoutCompatibility/>"#;
        let layout: LayoutCompatibility = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(layout, LayoutCompatibility::default());
    }

    #[test]
    fn deserialize_with_flags() {
        let xml = r#"<layoutCompatibility>
            <applyFontWeightToBold/>
            <useInnerUnderline/>
            <fixedUnderlineWidth/>
        </layoutCompatibility>"#;
        let layout: LayoutCompatibility = quick_xml::de::from_str(xml).unwrap();

        assert!(layout.apply_font_weight_to_bold.is_some());
        assert!(layout.use_inner_underline.is_some());
        assert!(layout.fixed_underline_width.is_some());
        // 나머지는 None
        assert!(layout.do_not_apply_strikeout_with_underline.is_none());
    }

    #[test]
    fn deserialize_underline_flags() {
        let xml = r#"<layoutCompatibility>
            <doNotApplyStrikeoutWithUnderline/>
            <useLowercaseStrikeout/>
        </layoutCompatibility>"#;
        let layout: LayoutCompatibility = quick_xml::de::from_str(xml).unwrap();

        assert!(layout.do_not_apply_strikeout_with_underline.is_some());
        assert!(layout.use_lowercase_strikeout.is_some());
    }

    #[test]
    fn deserialize_spacing_flags() {
        let xml = r#"<layoutCompatibility>
            <extendLineheightToOffset/>
            <applyFontspaceToLatin/>
            <treatQuotationAsLatin/>
        </layoutCompatibility>"#;
        let layout: LayoutCompatibility = quick_xml::de::from_str(xml).unwrap();

        assert!(layout.extend_lineheight_to_offset.is_some());
        assert!(layout.apply_fontspace_to_latin.is_some());
        assert!(layout.treat_quotation_as_latin.is_some());
    }

    #[test]
    fn deserialize_alignment_flags() {
        let xml = r#"<layoutCompatibility>
            <doNotAlignWhitespaceOnRight/>
            <doNotAdjustWordInJustify/>
            <baseCharUnitOnEAsian/>
        </layoutCompatibility>"#;
        let layout: LayoutCompatibility = quick_xml::de::from_str(xml).unwrap();

        assert!(layout.do_not_align_whitespace_on_right.is_some());
        assert!(layout.do_not_adjust_word_in_justify.is_some());
        assert!(layout.base_char_unit_on_east_asian.is_some());
    }

    #[test]
    fn deserialize_paragraph_flags() {
        let xml = r#"<layoutCompatibility>
            <adjustParaBorderfillToSpacing/>
            <connectParaBorderfillOfEqualBorder/>
            <applyParaBorderToOutside/>
        </layoutCompatibility>"#;
        let layout: LayoutCompatibility = quick_xml::de::from_str(xml).unwrap();

        assert!(layout.adjust_para_borderfill_to_spacing.is_some());
        assert!(layout.connect_para_borderfill_of_equal_border.is_some());
        assert!(layout.apply_para_border_to_outside.is_some());
    }

    #[test]
    fn deserialize_grid_flags() {
        let xml = r#"<layoutCompatibility>
            <baseLineSpacingOnLineGrid/>
            <applyCharSpacingToCharGrid/>
            <doNotApplyGridInHeaderFooter/>
        </layoutCompatibility>"#;
        let layout: LayoutCompatibility = quick_xml::de::from_str(xml).unwrap();

        assert!(layout.base_line_spacing_on_line_grid.is_some());
        assert!(layout.apply_char_spacing_to_char_grid.is_some());
        assert!(layout.do_not_apply_grid_in_header_footer.is_some());
    }

    #[test]
    fn deserialize_table_and_object_flags() {
        let xml = r#"<layoutCompatibility>
            <applyMinColumnWidthTo1mm/>
            <doNotHoldAnchorOfTable/>
            <doNotApplyImageEffect/>
            <doNotApplyShapeComment/>
        </layoutCompatibility>"#;
        let layout: LayoutCompatibility = quick_xml::de::from_str(xml).unwrap();

        assert!(layout.apply_min_column_width_to_1mm.is_some());
        assert!(layout.do_not_hold_anchor_of_table.is_some());
        assert!(layout.do_not_apply_image_effect.is_some());
        assert!(layout.do_not_apply_shape_comment.is_some());
    }

    #[test]
    fn serialize_empty() {
        let layout = LayoutCompatibility::default();
        let xml = quick_xml::se::to_string(&layout).unwrap();
        insta::assert_snapshot!("layout_compatibility_empty_serialized", xml);
    }

    #[test]
    fn serialize_with_flags() {
        let mut layout = LayoutCompatibility::default();
        layout.apply_font_weight_to_bold = Some(());
        layout.use_inner_underline = Some(());

        let xml = quick_xml::se::to_string(&layout).unwrap();
        insta::assert_snapshot!("layout_compatibility_with_flags_serialized", xml);
    }

    #[test]
    fn roundtrip_empty() {
        let original = LayoutCompatibility::default();
        let xml = quick_xml::se::to_string(&original).unwrap();
        let parsed: LayoutCompatibility = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn roundtrip_with_flags() {
        let mut original = LayoutCompatibility::default();
        original.apply_font_weight_to_bold = Some(());
        original.use_inner_underline = Some(());
        original.do_not_apply_strikeout_with_underline = Some(());

        let xml = quick_xml::se::to_string(&original).unwrap();
        let parsed: LayoutCompatibility = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(original, parsed);
    }
}

mod compatible_document_tests {
    use super::*;

    #[test]
    fn deserialize_minimal() {
        let xml = r#"<compatibleDocument targetProgram="HWP201X"><layoutCompatibility/></compatibleDocument>"#;
        let doc: CompatibleDocument = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(doc.target_program, TargetProgram::Hwp201X);
        assert_eq!(doc.layout_compatibility, LayoutCompatibility::default());
    }

    #[test]
    fn deserialize_with_namespace() {
        let xml = r#"<hh:compatibleDocument xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head" targetProgram="HWP201X"><hh:layoutCompatibility/></hh:compatibleDocument>"#;
        let doc: CompatibleDocument = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(doc.target_program, TargetProgram::Hwp201X);
    }

    #[test]
    fn deserialize_with_layout_flags() {
        let xml = r#"<compatibleDocument targetProgram="HWP200X">
            <layoutCompatibility>
                <applyFontWeightToBold/>
                <useInnerUnderline/>
            </layoutCompatibility>
        </compatibleDocument>"#;
        let doc: CompatibleDocument = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(doc.target_program, TargetProgram::Hwp200X);
        assert!(doc.layout_compatibility.apply_font_weight_to_bold.is_some());
        assert!(doc.layout_compatibility.use_inner_underline.is_some());
    }

    #[test]
    fn serialize_minimal() {
        let doc = CompatibleDocument {
            target_program: TargetProgram::Hwp201X,
            layout_compatibility: LayoutCompatibility::default(),
        };
        let xml = quick_xml::se::to_string(&doc).unwrap();
        insta::assert_snapshot!("compatible_document_minimal_serialized", xml);
    }

    #[test]
    fn serialize_with_flags() {
        let mut layout = LayoutCompatibility::default();
        layout.apply_font_weight_to_bold = Some(());

        let doc = CompatibleDocument {
            target_program: TargetProgram::MsWord,
            layout_compatibility: layout,
        };
        let xml = quick_xml::se::to_string(&doc).unwrap();
        insta::assert_snapshot!("compatible_document_with_flags_serialized", xml);
    }

    #[test]
    fn roundtrip() {
        let mut layout = LayoutCompatibility::default();
        layout.apply_font_weight_to_bold = Some(());
        layout.base_char_unit_on_east_asian = Some(());

        let original = CompatibleDocument {
            target_program: TargetProgram::Hwp201X,
            layout_compatibility: layout,
        };

        let xml = quick_xml::se::to_string(&original).unwrap();
        let parsed: CompatibleDocument = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn all_target_programs_roundtrip() {
        for program in [
            TargetProgram::Hwp201X,
            TargetProgram::Hwp200X,
            TargetProgram::MsWord,
        ] {
            let original = CompatibleDocument {
                target_program: program,
                layout_compatibility: LayoutCompatibility::default(),
            };

            let xml = quick_xml::se::to_string(&original).unwrap();
            let parsed: CompatibleDocument = quick_xml::de::from_str(&xml).unwrap();
            assert_eq!(original, parsed);
        }
    }
}

mod fixture_tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    /// 샘플 header.xml에서 compatibleDocument 추출하여 파싱
    fn extract_compatible_document(header_content: &str) -> Option<String> {
        // 네임스페이스 없는 버전으로 추출
        let start_with_ns = "<hh:compatibleDocument";
        let end_with_ns = "</hh:compatibleDocument>";

        if let Some(start_idx) = header_content.find(start_with_ns) {
            if let Some(end_idx) = header_content.find(end_with_ns) {
                let end_pos = end_idx + end_with_ns.len();
                return Some(header_content[start_idx..end_pos].to_string());
            }
        }
        None
    }

    #[test]
    fn parse_sample0_compatible_document() {
        let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
        let header_path = fixtures_dir.join("sample0/Contents/header.xml");

        if header_path.exists() {
            let content = fs::read_to_string(&header_path).unwrap();
            if let Some(doc_xml) = extract_compatible_document(&content) {
                let doc: CompatibleDocument = quick_xml::de::from_str(&doc_xml).unwrap();
                assert_eq!(doc.target_program, TargetProgram::Hwp201X);
            }
        }
    }

    #[test]
    #[ignore]
    fn parse_all_fixture_compatible_documents() {
        let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");

        let mut results = Vec::new();

        for i in 0.. {
            let sample_name = format!("sample{}", i);
            let header_path = fixtures_dir.join(&sample_name).join("Contents/header.xml");
            if !header_path.exists() {
                break;
            }

            let content = fs::read_to_string(&header_path).unwrap();
            if let Some(doc_xml) = extract_compatible_document(&content) {
                match quick_xml::de::from_str::<CompatibleDocument>(&doc_xml) {
                    Ok(doc) => {
                        results.push((sample_name, "OK", format!("{:?}", doc.target_program)));
                    }
                    Err(e) => {
                        panic!("{} 파싱 실패: {:?}", sample_name, e);
                    }
                }
            }
        }

        // 모든 샘플이 성공적으로 파싱되었는지 확인
        assert!(!results.is_empty(), "No fixtures found");
        insta::assert_yaml_snapshot!("all_fixture_compatible_documents", results);
    }
}
