//! 폰트 관련 테스트
//!
//! - FontLanguage, FontType, FontFamilyType 열거형 테스트
//! - Font, Fontface, FontfaceList 구조체 테스트
//! - PANOSE 분류 체계 테스트

use hwpx::header::font::{
    Font, FontFamilyType, FontLanguage, FontType, FontTypeInfo, Fontface, FontfaceList,
    PanoseContrast, PanoseProportion, PanoseSerifStyle, PanoseStrokeVariation, PanoseWeight,
    PanoseXHeight, SubstituteFont,
};

mod font_language_tests {
    use super::*;

    #[test]
    fn all_variants() {
        let variants = vec![
            FontLanguage::Hangul,
            FontLanguage::Latin,
            FontLanguage::Hanja,
            FontLanguage::Japanese,
            FontLanguage::Other,
            FontLanguage::Symbol,
            FontLanguage::User,
        ];
        insta::assert_yaml_snapshot!("font_languages", variants);
    }
}

mod font_type_tests {
    use super::*;

    #[test]
    fn all_variants() {
        let variants = vec![
            FontType::Representative,
            FontType::TrueType,
            FontType::HangeulFont,
        ];
        insta::assert_yaml_snapshot!("font_types", variants);
    }
}

mod font_family_type_tests {
    use super::*;

    #[test]
    fn all_variants() {
        let variants = vec![
            FontFamilyType::Unknown,
            FontFamilyType::Myungjo,
            FontFamilyType::Gothic,
            FontFamilyType::SansSerif,
            FontFamilyType::BrushScript,
            FontFamilyType::Decorative,
            FontFamilyType::NonRectMyungjo,
            FontFamilyType::NonRectGothic,
        ];
        insta::assert_yaml_snapshot!("font_family_types", variants);
    }
}

mod panose_tests {
    use super::*;

    #[test]
    fn serif_style_defaults() {
        let default = PanoseSerifStyle::default();
        assert_eq!(default, PanoseSerifStyle::Any);
    }

    #[test]
    fn weight_defaults() {
        let default = PanoseWeight::default();
        assert_eq!(default, PanoseWeight::Any);
    }

    #[test]
    fn proportion_defaults() {
        let default = PanoseProportion::default();
        assert_eq!(default, PanoseProportion::Any);
    }

    #[test]
    fn contrast_defaults() {
        let default = PanoseContrast::default();
        assert_eq!(default, PanoseContrast::Any);
    }

    #[test]
    fn stroke_variation_defaults() {
        let default = PanoseStrokeVariation::default();
        assert_eq!(default, PanoseStrokeVariation::Any);
    }

    #[test]
    fn x_height_defaults() {
        let default = PanoseXHeight::default();
        assert_eq!(default, PanoseXHeight::Any);
    }
}

mod font_type_info_tests {
    use super::*;

    #[test]
    fn deserialize_basic() {
        // PANOSE 값: Medium=6, Modern=3, Low=4, NoVariation=2, armStyle=0, letterform=0, midline=0, ConstantStandard=3
        let xml = r#"<typeInfo familyType="FCAT_GOTHIC" weight="6" proportion="3" contrast="4" strokeVariation="2" armStyle="0" letterform="0" midline="0" xHeight="3"/>"#;
        let info: FontTypeInfo = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("font_type_info_basic", info);
    }

    #[test]
    fn deserialize_with_serif() {
        // PANOSE 값: Cove=2, DemiBold=7, EvenWidth=4, MediumLow=5, GradualDiagonal=3, armStyle=5, letterform=8, midline=0, ConstantLarge=4
        let xml = r#"<typeInfo familyType="FCAT_MYUNGJO" serifStyle="2" weight="7" proportion="4" contrast="5" strokeVariation="3" armStyle="5" letterform="8" midline="0" xHeight="4"/>"#;
        let info: FontTypeInfo = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("font_type_info_serif", info);
    }
}

mod substitute_font_tests {
    use super::*;

    #[test]
    fn deserialize_basic() {
        let xml = r#"<substFont face="Sample Sans" type="TTF" isEmbedded="false"/>"#;
        let subst: SubstituteFont = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("subst_font_basic", subst);
    }

    #[test]
    fn deserialize_embedded() {
        let xml = r#"<substFont face="Test Serif" type="TTF" isEmbedded="true" binaryItemIDRef="FONT01"/>"#;
        let subst: SubstituteFont = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("subst_font_embedded", subst);
    }

    #[test]
    fn deserialize_hft() {
        let xml = r#"<substFont face="Sample Korean" type="HFT" isEmbedded="false"/>"#;
        let subst: SubstituteFont = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("subst_font_hft", subst);
    }
}

mod font_tests {
    use super::*;

    #[test]
    fn deserialize_minimal() {
        let xml = r#"<font id="0" face="Test Gothic" type="TTF" isEmbedded="false"/>"#;
        let font: Font = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("font_minimal", font);
    }

    #[test]
    fn deserialize_with_type_info() {
        // PANOSE 값: Medium=6, Modern=3, Low=4, NoVariation=2, armStyle=0, letterform=0, midline=0, ConstantStandard=3
        let xml = r#"<font id="1" face="Sample Serif" type="TTF" isEmbedded="false">
            <typeInfo familyType="FCAT_MYUNGJO" weight="6" proportion="3" contrast="4" strokeVariation="2" armStyle="0" letterform="0" midline="0" xHeight="3"/>
        </font>"#;
        let font: Font = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("font_with_type_info", font);
    }

    #[test]
    fn deserialize_with_substitute() {
        let xml = r#"<font id="2" face="Primary Font" type="TTF" isEmbedded="false">
            <substFont face="Fallback Font" type="TTF" isEmbedded="false"/>
        </font>"#;
        let font: Font = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("font_with_substitute", font);
    }

    #[test]
    fn deserialize_embedded() {
        let xml = r#"<font id="3" face="Embedded Font" type="TTF" isEmbedded="true" binaryItemIDRef="BIN0001"/>"#;
        let font: Font = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("font_embedded", font);
    }

    #[test]
    fn deserialize_representative() {
        let xml = r#"<font id="4" face="Representative Font" type="REP" isEmbedded="false"/>"#;
        let font: Font = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("font_representative", font);
    }

    #[test]
    fn deserialize_complete() {
        // PANOSE 값: DemiBold=7, EvenWidth=4, MediumLow=5, GradualDiagonal=3, armStyle=5, letterform=8, midline=3, ConstantLarge=4
        let xml = r#"<font id="5" face="Complete Font" type="TTF" isEmbedded="true" binaryItemIDRef="BIN0002">
            <substFont face="Substitute Complete" type="TTF" isEmbedded="false"/>
            <typeInfo familyType="FCAT_GOTHIC" weight="7" proportion="4" contrast="5" strokeVariation="3" armStyle="5" letterform="8" midline="3" xHeight="4"/>
        </font>"#;
        let font: Font = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("font_complete", font);
    }
}

mod fontface_tests {
    use super::*;

    #[test]
    fn deserialize_hangul() {
        let xml = r#"<fontface lang="HANGUL" fontCnt="1">
            <font id="0" face="Sample Korean Font" type="TTF" isEmbedded="false"/>
        </fontface>"#;
        let fontface: Fontface = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("fontface_hangul", fontface);
    }

    #[test]
    fn deserialize_latin() {
        let xml = r#"<fontface lang="LATIN" fontCnt="2">
            <font id="0" face="Sample Sans" type="TTF" isEmbedded="false"/>
            <font id="1" face="Sample Serif" type="TTF" isEmbedded="false"/>
        </fontface>"#;
        let fontface: Fontface = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("fontface_latin", fontface);
    }

    #[test]
    fn deserialize_hanja() {
        let xml = r#"<fontface lang="HANJA" fontCnt="1">
            <font id="0" face="Sample CJK Font" type="TTF" isEmbedded="false"/>
        </fontface>"#;
        let fontface: Fontface = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("fontface_hanja", fontface);
    }

    #[test]
    fn deserialize_symbol() {
        let xml = r#"<fontface lang="SYMBOL" fontCnt="1">
            <font id="0" face="Sample Symbols" type="TTF" isEmbedded="false"/>
        </fontface>"#;
        let fontface: Fontface = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("fontface_symbol", fontface);
    }
}

mod fontface_list_tests {
    use super::*;

    #[test]
    fn deserialize_single_language() {
        let xml = r#"<fontfaces itemCnt="1">
            <fontface lang="HANGUL" fontCnt="1">
                <font id="0" face="Sample Korean" type="TTF" isEmbedded="false"/>
            </fontface>
        </fontfaces>"#;
        let list: FontfaceList = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("fontface_list_single", list);
    }

    #[test]
    fn deserialize_multi_language() {
        let xml = r#"<fontfaces itemCnt="3">
            <fontface lang="HANGUL" fontCnt="1">
                <font id="0" face="Sample Korean" type="TTF" isEmbedded="false"/>
            </fontface>
            <fontface lang="LATIN" fontCnt="1">
                <font id="0" face="Sample Latin" type="TTF" isEmbedded="false"/>
            </fontface>
            <fontface lang="HANJA" fontCnt="1">
                <font id="0" face="Sample Hanja" type="TTF" isEmbedded="false"/>
            </fontface>
        </fontfaces>"#;
        let list: FontfaceList = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("fontface_list_multi", list);
    }

    #[test]
    fn deserialize_complete_setup() {
        let xml = r#"<fontfaces itemCnt="7">
            <fontface lang="HANGUL" fontCnt="2">
                <font id="0" face="Sample Gothic" type="TTF" isEmbedded="false"/>
                <font id="1" face="Sample Myungjo" type="TTF" isEmbedded="false"/>
            </fontface>
            <fontface lang="LATIN" fontCnt="2">
                <font id="0" face="Sample Sans" type="TTF" isEmbedded="false"/>
                <font id="1" face="Sample Serif" type="TTF" isEmbedded="false"/>
            </fontface>
            <fontface lang="HANJA" fontCnt="1">
                <font id="0" face="Sample CJK" type="TTF" isEmbedded="false"/>
            </fontface>
            <fontface lang="JAPANESE" fontCnt="1">
                <font id="0" face="Sample Japanese" type="TTF" isEmbedded="false"/>
            </fontface>
            <fontface lang="OTHER" fontCnt="1">
                <font id="0" face="Sample Other" type="TTF" isEmbedded="false"/>
            </fontface>
            <fontface lang="SYMBOL" fontCnt="1">
                <font id="0" face="Sample Symbols" type="TTF" isEmbedded="false"/>
            </fontface>
            <fontface lang="USER" fontCnt="1">
                <font id="0" face="Sample User Font" type="TTF" isEmbedded="false"/>
            </fontface>
        </fontfaces>"#;
        let list: FontfaceList = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("fontface_list_complete", list);
    }
}
