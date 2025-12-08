//! Core 타입 테스트
//!
//! core 모듈의 타입들에 대한 단위 테스트

use hwpx::core::types::{FillBrush, HwpUnit, OptionalRgbColor, RgbColor, WindowsBrush};
use quick_xml::de::from_str;

mod hwp_unit_tests {
    use super::*;

    #[test]
    fn hwp_unit_basic() {
        let unit = HwpUnit::new(1000);
        assert_eq!(unit.value(), 1000);
    }

    #[test]
    fn hwp_unit_to_pt() {
        // 10pt = 1000 hwpunit
        let unit = HwpUnit::new(1000);
        assert!((unit.to_pt() - 10.0).abs() < 0.001);
    }

    #[test]
    fn hwp_unit_from_pt() {
        let unit = HwpUnit::from_pt(10.0);
        assert_eq!(unit.value(), 1000);
    }

    #[test]
    fn hwp_unit_to_mm() {
        // 1 inch = 7200 hwpunit, 1 inch = 25.4 mm
        let unit = HwpUnit::new(7200);
        assert!((unit.to_mm() - 25.4).abs() < 0.001);
    }

    #[test]
    fn hwp_unit_from_mm() {
        let unit = HwpUnit::from_mm(25.4);
        assert_eq!(unit.value(), 7200);
    }

    #[test]
    fn hwp_unit_roundtrip_pt() {
        let original = 15.5;
        let unit = HwpUnit::from_pt(original);
        let result = unit.to_pt();
        assert!((result - original).abs() < 0.01);
    }

    #[test]
    fn hwp_unit_roundtrip_mm() {
        let original = 10.0;
        let unit = HwpUnit::from_mm(original);
        let result = unit.to_mm();
        assert!((result - original).abs() < 0.01);
    }
}

mod rgb_color_tests {
    use super::*;

    #[test]
    fn rgb_color_from_hex() {
        let color = RgbColor::from_hex("#FF0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn rgb_color_from_hex_without_hash() {
        let color = RgbColor::from_hex("00FF00").unwrap();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn rgb_color_to_hex() {
        let color = RgbColor::rgb(0, 0, 255);
        assert_eq!(color.to_hex(), "#0000FF");
    }

    #[test]
    fn rgb_color_constants() {
        assert_eq!(RgbColor::BLACK, RgbColor::rgb(0, 0, 0));
        assert_eq!(RgbColor::WHITE, RgbColor::rgb(255, 255, 255));
        assert_eq!(RgbColor::RED, RgbColor::rgb(255, 0, 0));
        assert_eq!(RgbColor::GREEN, RgbColor::rgb(0, 255, 0));
        assert_eq!(RgbColor::BLUE, RgbColor::rgb(0, 0, 255));
    }

    #[test]
    fn rgb_color_to_u32() {
        let color = RgbColor::rgb(0x12, 0x34, 0x56);
        assert_eq!(color.to_u32(), 0x123456);
    }

    #[test]
    fn rgb_color_from_u32() {
        let color = RgbColor::from_u32(0xABCDEF);
        assert_eq!(color.r, 0xAB);
        assert_eq!(color.g, 0xCD);
        assert_eq!(color.b, 0xEF);
    }

    #[test]
    fn rgb_color_invalid_hex() {
        assert!(RgbColor::from_hex("#FFF").is_none()); // too short
        assert!(RgbColor::from_hex("#FFFFFFF").is_none()); // too long
        assert!(RgbColor::from_hex("#GGGGGG").is_none()); // invalid chars
    }

    /// ARGB 8자리 형식 테스트 (#AARRGGBB)
    /// fixture에서 발견: hatchColor="#FF000000" (불투명한 검은색)
    #[test]
    fn rgb_color_from_hex_argb_opaque_black() {
        // #FF000000 = A=FF(255), R=00, G=00, B=00 = 불투명한 검은색
        let color = RgbColor::from_hex("#FF000000").unwrap();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn rgb_color_from_hex_argb_semitransparent() {
        // #80FF0000 = A=80(128), R=FF, G=00, B=00 = 반투명 빨간색
        let color = RgbColor::from_hex("#80FF0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 128);
    }

    #[test]
    fn rgb_color_from_hex_argb_transparent() {
        // #00FFFFFF = A=00(0), R=FF, G=FF, B=FF = 완전 투명한 흰색
        let color = RgbColor::from_hex("#00FFFFFF").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
        assert_eq!(color.a, 0);
    }

    #[test]
    fn rgb_color_to_hex_argb() {
        // alpha != 255인 경우 ARGB 형식으로 출력
        let color = RgbColor::rgba(255, 0, 0, 128);
        assert_eq!(color.to_hex(), "#80FF0000");
    }

    #[test]
    fn rgb_color_roundtrip_argb() {
        let original = "#80123456";
        let color = RgbColor::from_hex(original).unwrap();
        assert_eq!(color.to_hex(), original);
    }
}

mod optional_rgb_color_tests {
    use super::*;

    #[test]
    fn optional_rgb_color_none() {
        let color = OptionalRgbColor::none();
        assert_eq!(color.value(), None);
    }

    #[test]
    fn optional_rgb_color_some() {
        let color = OptionalRgbColor::some(RgbColor::RED);
        assert_eq!(color.value(), Some(RgbColor::RED));
    }

    #[test]
    fn optional_rgb_color_from_str_none() {
        let color = OptionalRgbColor::from_str("none").unwrap();
        assert_eq!(color.value(), None);
    }

    #[test]
    fn optional_rgb_color_from_str_none_uppercase() {
        let color = OptionalRgbColor::from_str("NONE").unwrap();
        assert_eq!(color.value(), None);
    }

    #[test]
    fn optional_rgb_color_from_str_hex() {
        let color = OptionalRgbColor::from_str("#FF0000").unwrap();
        assert_eq!(color.value(), Some(RgbColor::RED));
    }

    #[test]
    fn optional_rgb_color_default_is_none() {
        let color = OptionalRgbColor::default();
        assert_eq!(color.value(), None);
    }
}

mod fill_brush_tests {
    use super::*;

    #[test]
    fn parse_fill_brush_with_windows_brush() {
        let xml = r##"<hc:fillBrush xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core">
            <hc:winBrush faceColor="#FFFFFF" hatchColor="#000000" alpha="0"/>
        </hc:fillBrush>"##;

        let result: Result<FillBrush, _> = from_str(xml);
        assert!(result.is_ok(), "FillBrush 파싱 실패: {:?}", result.err());

        let fill_brush = result.unwrap();
        insta::assert_yaml_snapshot!(fill_brush);
    }

    #[test]
    fn parse_fill_brush_with_none_face_color() {
        let xml = r##"<hc:fillBrush xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core">
            <hc:winBrush faceColor="none" hatchColor="#000000" alpha="0"/>
        </hc:fillBrush>"##;

        let result: Result<FillBrush, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "none faceColor FillBrush 파싱 실패: {:?}",
            result.err()
        );

        let fill_brush = result.unwrap();
        insta::assert_yaml_snapshot!(fill_brush);
    }

    #[test]
    fn parse_fill_brush_with_both_none_colors() {
        let xml = r##"<hc:fillBrush xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core">
            <hc:winBrush faceColor="none" hatchColor="none" alpha="0.5"/>
        </hc:fillBrush>"##;

        let result: Result<FillBrush, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "both none FillBrush 파싱 실패: {:?}",
            result.err()
        );

        let fill_brush = result.unwrap();
        insta::assert_yaml_snapshot!(fill_brush);
    }

    #[test]
    fn parse_empty_fill_brush() {
        let xml = r##"<hc:fillBrush xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core"/>"##;

        let result: Result<FillBrush, _> = from_str(xml);
        assert!(result.is_ok(), "빈 FillBrush 파싱 실패: {:?}", result.err());

        let fill_brush = result.unwrap();
        insta::assert_yaml_snapshot!(fill_brush);
    }
}

mod windows_brush_tests {
    use super::*;

    #[test]
    fn parse_windows_brush_default_colors() {
        let xml = r##"<hc:winBrush xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core" faceColor="#FFFFFF" hatchColor="#000000"/>"##;

        let result: Result<WindowsBrush, _> = from_str(xml);
        assert!(result.is_ok(), "WindowsBrush 파싱 실패: {:?}", result.err());

        let brush = result.unwrap();
        insta::assert_yaml_snapshot!(brush);
    }

    #[test]
    fn parse_windows_brush_with_alpha() {
        let xml = r##"<hc:winBrush xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core" faceColor="#FF0000" hatchColor="#00FF00" alpha="0.5"/>"##;

        let result: Result<WindowsBrush, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "alpha 있는 WindowsBrush 파싱 실패: {:?}",
            result.err()
        );

        let brush = result.unwrap();
        insta::assert_yaml_snapshot!(brush);
    }

    #[test]
    fn parse_windows_brush_with_none_colors() {
        let xml = r##"<hc:winBrush xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core" faceColor="none" hatchColor="none" alpha="0"/>"##;

        let result: Result<WindowsBrush, _> = from_str(xml);
        assert!(
            result.is_ok(),
            "none 색상 WindowsBrush 파싱 실패: {:?}",
            result.err()
        );

        let brush = result.unwrap();
        assert!(brush.face_color.value().is_none());
        assert!(brush.hatch_color.value().is_none());
        insta::assert_yaml_snapshot!(brush);
    }

    #[test]
    fn windows_brush_default() {
        let brush = WindowsBrush::default();
        assert_eq!(brush.face_color.value(), Some(RgbColor::WHITE));
        assert_eq!(brush.hatch_color.value(), Some(RgbColor::BLACK));
        assert!(brush.hatch_style.is_none());
        assert!(brush.alpha.is_none());
    }
}
