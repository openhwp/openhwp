//! 헤더 관련 타입 테스트
//!
//! - BeginNumber 테스트
//! - Border/BorderFill 테스트

use hwpx::header::begin_number::BeginNumber;
use hwpx::header::border_fill::{
    Border, BorderFill, BorderFillList, CenterLineType, Slash, SlashDiagonalType,
};

mod begin_number_tests {
    use super::*;

    #[test]
    fn default_values() {
        let begin = BeginNumber::default();
        insta::assert_yaml_snapshot!("begin_number_default", begin);
    }

    #[test]
    fn deserialize_from_xml() {
        let xml = r#"<beginNum page="1" footnote="1" endnote="1" pic="1" tbl="1" equation="1"/>"#;
        let begin: BeginNumber = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("begin_number_parsed", begin);
    }

    #[test]
    fn deserialize_with_custom_values() {
        let xml = r#"<beginNum page="5" footnote="3" endnote="2" pic="10" tbl="7" equation="4"/>"#;
        let begin: BeginNumber = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("begin_number_custom", begin);
    }

    #[test]
    fn serialize_to_xml() {
        let begin = BeginNumber::default();
        let xml = quick_xml::se::to_string(&begin).unwrap();
        insta::assert_snapshot!("begin_number_serialized", xml);
    }

    #[test]
    fn roundtrip() {
        let original = BeginNumber::default();
        let xml = quick_xml::se::to_string(&original).unwrap();
        let parsed: BeginNumber = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(original, parsed);
    }
}

mod slash_diagonal_type_tests {
    use super::*;

    #[test]
    fn all_variants() {
        let variants = vec![
            SlashDiagonalType::None,
            SlashDiagonalType::Center,
            SlashDiagonalType::CenterBelow,
            SlashDiagonalType::CenterAbove,
            SlashDiagonalType::All,
        ];
        insta::assert_yaml_snapshot!("slash_diagonal_types", variants);
    }

    #[test]
    fn deserialize_none() {
        let xml = r#"<slash type="NONE" Crooked="false" isCounter="false"/>"#;
        let slash: Slash = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(slash.diagonal_type, SlashDiagonalType::None);
    }

    #[test]
    fn deserialize_center() {
        let xml = r#"<slash type="CENTER" Crooked="true" isCounter="false"/>"#;
        let slash: Slash = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(slash.diagonal_type, SlashDiagonalType::Center);
        assert!(slash.crooked);
    }

    #[test]
    fn deserialize_all() {
        let xml = r#"<slash type="ALL" Crooked="false" isCounter="true"/>"#;
        let slash: Slash = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(slash.diagonal_type, SlashDiagonalType::All);
        assert!(slash.is_counter);
    }
}

mod center_line_type_tests {
    use super::*;

    #[test]
    fn all_variants() {
        let variants = vec![
            CenterLineType::None,
            CenterLineType::Vertical,
            CenterLineType::Horizontal,
            CenterLineType::Cross,
        ];
        insta::assert_yaml_snapshot!("center_line_types", variants);
    }

    #[test]
    fn default_is_none() {
        let default = CenterLineType::default();
        assert_eq!(default, CenterLineType::None);
    }
}

mod slash_tests {
    use super::*;

    #[test]
    fn deserialize_basic() {
        let xml = r#"<slash type="NONE" Crooked="false" isCounter="false"/>"#;
        let slash: Slash = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("slash_basic", slash);
    }

    #[test]
    fn deserialize_with_crooked() {
        let xml = r#"<slash type="CENTER_BELOW" Crooked="true" isCounter="false"/>"#;
        let slash: Slash = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("slash_crooked", slash);
    }

    #[test]
    fn deserialize_counter_slash() {
        let xml = r#"<slash type="CENTER_ABOVE" Crooked="false" isCounter="true"/>"#;
        let slash: Slash = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("slash_counter", slash);
    }

    #[test]
    fn serialize_roundtrip() {
        let slash = Slash {
            diagonal_type: SlashDiagonalType::CenterBelow,
            crooked: true,
            is_counter: false,
        };
        let xml = quick_xml::se::to_string(&slash).unwrap();
        let parsed: Slash = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(slash, parsed);
    }
}

mod border_tests {
    use super::*;
    use hwpx::core::enums::{LineStyleType2, LineWidth};
    use hwpx::core::types::RgbColor;

    #[test]
    fn deserialize_solid_border() {
        let xml = r##"<leftBorder type="SOLID" width="0.12 mm" color="#000000"/>"##;
        let border: Border = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("border_solid", border);
    }

    #[test]
    fn deserialize_dashed_border() {
        let xml = r##"<topBorder type="DASH" width="0.4 mm" color="#FF0000"/>"##;
        let border: Border = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("border_dashed", border);
    }

    #[test]
    fn deserialize_dotted_border() {
        let xml = r##"<rightBorder type="DOT" width="0.3 mm" color="#0000FF"/>"##;
        let border: Border = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("border_dotted", border);
    }

    #[test]
    fn serialize_roundtrip() {
        let border = Border {
            line_type: LineStyleType2::Solid,
            width: LineWidth::Mm0_12,
            color: RgbColor::BLACK,
        };
        let xml = quick_xml::se::to_string(&border).unwrap();
        let parsed: Border = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(border, parsed);
    }
}

mod border_fill_tests {
    use super::*;

    #[test]
    fn deserialize_minimal() {
        let xml = r#"<borderFill id="1" threeD="false" shadow="false" breakCellSeparateLine="false"/>"#;
        let bf: BorderFill = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("border_fill_minimal", bf);
    }

    #[test]
    fn deserialize_with_3d_shadow() {
        let xml = r#"<borderFill id="2" threeD="true" shadow="true" breakCellSeparateLine="false"/>"#;
        let bf: BorderFill = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("border_fill_3d_shadow", bf);
    }

    #[test]
    fn deserialize_with_center_line() {
        let xml = r#"<borderFill id="3" threeD="false" shadow="false" centerLine="CROSS" breakCellSeparateLine="true"/>"#;
        let bf: BorderFill = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("border_fill_center_line", bf);
    }

    #[test]
    fn deserialize_with_borders() {
        let xml = r##"<borderFill id="4" threeD="false" shadow="false" breakCellSeparateLine="false">
            <leftBorder type="SOLID" width="0.12 mm" color="#000000"/>
            <rightBorder type="SOLID" width="0.12 mm" color="#000000"/>
            <topBorder type="SOLID" width="0.12 mm" color="#000000"/>
            <bottomBorder type="SOLID" width="0.12 mm" color="#000000"/>
        </borderFill>"##;
        let bf: BorderFill = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("border_fill_with_borders", bf);
    }

    #[test]
    fn deserialize_with_slash() {
        let xml = r#"<borderFill id="5" threeD="false" shadow="false" breakCellSeparateLine="false">
            <slash type="CENTER" Crooked="false" isCounter="false"/>
            <backSlash type="CENTER" Crooked="false" isCounter="true"/>
        </borderFill>"#;
        let bf: BorderFill = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("border_fill_with_slash", bf);
    }

    #[test]
    fn deserialize_with_diagonal() {
        let xml = r##"<borderFill id="6" threeD="false" shadow="false" breakCellSeparateLine="false">
            <diagonal type="DASH" width="0.4 mm" color="#FF0000"/>
        </borderFill>"##;
        let bf: BorderFill = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("border_fill_with_diagonal", bf);
    }
}

mod border_fill_list_tests {
    use super::*;

    #[test]
    fn deserialize_single_item() {
        let xml = r#"<borderFills itemCnt="1">
            <borderFill id="1" threeD="false" shadow="false" breakCellSeparateLine="false"/>
        </borderFills>"#;
        let list: BorderFillList = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("border_fill_list_single", list);
    }

    #[test]
    fn deserialize_multiple_items() {
        let xml = r#"<borderFills itemCnt="3">
            <borderFill id="1" threeD="false" shadow="false" breakCellSeparateLine="false"/>
            <borderFill id="2" threeD="true" shadow="false" breakCellSeparateLine="false"/>
            <borderFill id="3" threeD="false" shadow="true" breakCellSeparateLine="true"/>
        </borderFills>"#;
        let list: BorderFillList = quick_xml::de::from_str(xml).unwrap();
        insta::assert_yaml_snapshot!("border_fill_list_multiple", list);
    }
}
