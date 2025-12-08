//! Core 열거형 타입 테스트
//!
//! core/enums 모듈의 열거형 타입들에 대한 단위 테스트

use hwpx::core::enums::{
    GradationType, HatchStyle, ImageBrushMode, ImageEffect, LineStyleType1, LineStyleType2,
    NumberFormatType1, NumberFormatType2, ValueUnit,
};
use quick_xml::de::from_str;
use serde::Deserialize;

/// 테스트용 래퍼 구조체
#[derive(Debug, Deserialize)]
struct EnumWrapper<T> {
    #[serde(rename = "@value")]
    value: T,
}

mod number_format_type1_tests {
    use super::*;

    #[test]
    fn parse_digit() {
        let xml = r#"<test value="DIGIT"/>"#;
        let result: EnumWrapper<NumberFormatType1> = from_str(xml).unwrap();
        assert_eq!(result.value, NumberFormatType1::Digit);
    }

    #[test]
    fn parse_circled_digit() {
        let xml = r#"<test value="CIRCLED_DIGIT"/>"#;
        let result: EnumWrapper<NumberFormatType1> = from_str(xml).unwrap();
        assert_eq!(result.value, NumberFormatType1::CircledDigit);
    }

    #[test]
    fn parse_roman_capital() {
        let xml = r#"<test value="ROMAN_CAPITAL"/>"#;
        let result: EnumWrapper<NumberFormatType1> = from_str(xml).unwrap();
        assert_eq!(result.value, NumberFormatType1::RomanCapital);
    }

    #[test]
    fn parse_roman_small() {
        let xml = r#"<test value="ROMAN_SMALL"/>"#;
        let result: EnumWrapper<NumberFormatType1> = from_str(xml).unwrap();
        assert_eq!(result.value, NumberFormatType1::RomanSmall);
    }

    #[test]
    fn parse_latin_capital() {
        let xml = r#"<test value="LATIN_CAPITAL"/>"#;
        let result: EnumWrapper<NumberFormatType1> = from_str(xml).unwrap();
        assert_eq!(result.value, NumberFormatType1::LatinCapital);
    }

    #[test]
    fn parse_latin_small() {
        let xml = r#"<test value="LATIN_SMALL"/>"#;
        let result: EnumWrapper<NumberFormatType1> = from_str(xml).unwrap();
        assert_eq!(result.value, NumberFormatType1::LatinSmall);
    }

    #[test]
    fn parse_hangul_syllable() {
        let xml = r#"<test value="HANGUL_SYLLABLE"/>"#;
        let result: EnumWrapper<NumberFormatType1> = from_str(xml).unwrap();
        assert_eq!(result.value, NumberFormatType1::HangulSyllable);
    }

    #[test]
    fn parse_hangul_jamo() {
        let xml = r#"<test value="HANGUL_JAMO"/>"#;
        let result: EnumWrapper<NumberFormatType1> = from_str(xml).unwrap();
        assert_eq!(result.value, NumberFormatType1::HangulJamo);
    }

    #[test]
    fn parse_ideograph() {
        let xml = r#"<test value="IDEOGRAPH"/>"#;
        let result: EnumWrapper<NumberFormatType1> = from_str(xml).unwrap();
        assert_eq!(result.value, NumberFormatType1::Ideograph);
    }

    #[test]
    fn default_is_digit() {
        assert_eq!(NumberFormatType1::default(), NumberFormatType1::Digit);
    }
}

mod number_format_type2_tests {
    use super::*;

    #[test]
    fn parse_symbol() {
        let xml = r#"<test value="SYMBOL"/>"#;
        let result: EnumWrapper<NumberFormatType2> = from_str(xml).unwrap();
        assert_eq!(result.value, NumberFormatType2::Symbol);
    }

    #[test]
    fn parse_user_char() {
        let xml = r#"<test value="USER_CHAR"/>"#;
        let result: EnumWrapper<NumberFormatType2> = from_str(xml).unwrap();
        assert_eq!(result.value, NumberFormatType2::UserCharacter);
    }

    #[test]
    fn parse_decagon_circle() {
        let xml = r#"<test value="DECAGON_CIRCLE"/>"#;
        let result: EnumWrapper<NumberFormatType2> = from_str(xml).unwrap();
        assert_eq!(result.value, NumberFormatType2::DecagonCircle);
    }

    #[test]
    fn default_is_digit() {
        assert_eq!(NumberFormatType2::default(), NumberFormatType2::Digit);
    }
}

mod line_style_type1_tests {
    use super::*;

    #[test]
    fn parse_none() {
        let xml = r#"<test value="NONE"/>"#;
        let result: EnumWrapper<LineStyleType1> = from_str(xml).unwrap();
        assert_eq!(result.value, LineStyleType1::None);
    }

    #[test]
    fn parse_solid() {
        let xml = r#"<test value="SOLID"/>"#;
        let result: EnumWrapper<LineStyleType1> = from_str(xml).unwrap();
        assert_eq!(result.value, LineStyleType1::Solid);
    }

    #[test]
    fn parse_dot() {
        let xml = r#"<test value="DOT"/>"#;
        let result: EnumWrapper<LineStyleType1> = from_str(xml).unwrap();
        assert_eq!(result.value, LineStyleType1::Dot);
    }

    #[test]
    fn parse_dash() {
        let xml = r#"<test value="DASH"/>"#;
        let result: EnumWrapper<LineStyleType1> = from_str(xml).unwrap();
        assert_eq!(result.value, LineStyleType1::Dash);
    }

    #[test]
    fn parse_dash_dot() {
        let xml = r#"<test value="DASH_DOT"/>"#;
        let result: EnumWrapper<LineStyleType1> = from_str(xml).unwrap();
        assert_eq!(result.value, LineStyleType1::DashDot);
    }

    #[test]
    fn parse_dash_dot_dot() {
        let xml = r#"<test value="DASH_DOT_DOT"/>"#;
        let result: EnumWrapper<LineStyleType1> = from_str(xml).unwrap();
        assert_eq!(result.value, LineStyleType1::DashDotDot);
    }

    #[test]
    fn default_is_solid() {
        assert_eq!(LineStyleType1::default(), LineStyleType1::Solid);
    }
}

mod line_style_type2_tests {
    use super::*;

    #[test]
    fn parse_double_slim() {
        let xml = r#"<test value="DOUBLE_SLIM"/>"#;
        let result: EnumWrapper<LineStyleType2> = from_str(xml).unwrap();
        assert_eq!(result.value, LineStyleType2::DoubleSlim);
    }

    #[test]
    fn parse_slim_thick() {
        let xml = r#"<test value="SLIM_THICK"/>"#;
        let result: EnumWrapper<LineStyleType2> = from_str(xml).unwrap();
        assert_eq!(result.value, LineStyleType2::SlimThick);
    }

    #[test]
    fn parse_thick_slim() {
        let xml = r#"<test value="THICK_SLIM"/>"#;
        let result: EnumWrapper<LineStyleType2> = from_str(xml).unwrap();
        assert_eq!(result.value, LineStyleType2::ThickSlim);
    }

    #[test]
    fn parse_slim_thick_slim() {
        let xml = r#"<test value="SLIM_THICK_SLIM"/>"#;
        let result: EnumWrapper<LineStyleType2> = from_str(xml).unwrap();
        assert_eq!(result.value, LineStyleType2::SlimThickSlim);
    }

    #[test]
    fn parse_long_dash() {
        let xml = r#"<test value="LONG_DASH"/>"#;
        let result: EnumWrapper<LineStyleType2> = from_str(xml).unwrap();
        assert_eq!(result.value, LineStyleType2::LongDash);
    }

    #[test]
    fn parse_circle() {
        let xml = r#"<test value="CIRCLE"/>"#;
        let result: EnumWrapper<LineStyleType2> = from_str(xml).unwrap();
        assert_eq!(result.value, LineStyleType2::Circle);
    }

    #[test]
    fn default_is_solid() {
        assert_eq!(LineStyleType2::default(), LineStyleType2::Solid);
    }
}

mod hatch_style_tests {
    use super::*;

    #[test]
    fn parse_horizontal() {
        let xml = r#"<test value="HORIZONTAL"/>"#;
        let result: EnumWrapper<HatchStyle> = from_str(xml).unwrap();
        assert_eq!(result.value, HatchStyle::Horizontal);
    }

    #[test]
    fn parse_vertical() {
        let xml = r#"<test value="VERTICAL"/>"#;
        let result: EnumWrapper<HatchStyle> = from_str(xml).unwrap();
        assert_eq!(result.value, HatchStyle::Vertical);
    }

    #[test]
    fn parse_back_slash() {
        let xml = r#"<test value="BACK_SLASH"/>"#;
        let result: EnumWrapper<HatchStyle> = from_str(xml).unwrap();
        assert_eq!(result.value, HatchStyle::BackSlash);
    }

    #[test]
    fn parse_slash() {
        let xml = r#"<test value="SLASH"/>"#;
        let result: EnumWrapper<HatchStyle> = from_str(xml).unwrap();
        assert_eq!(result.value, HatchStyle::Slash);
    }

    #[test]
    fn parse_cross() {
        let xml = r#"<test value="CROSS"/>"#;
        let result: EnumWrapper<HatchStyle> = from_str(xml).unwrap();
        assert_eq!(result.value, HatchStyle::Cross);
    }

    #[test]
    fn parse_cross_diagonal() {
        let xml = r#"<test value="CROSS_DIAGONAL"/>"#;
        let result: EnumWrapper<HatchStyle> = from_str(xml).unwrap();
        assert_eq!(result.value, HatchStyle::CrossDiagonal);
    }
}

mod gradation_type_tests {
    use super::*;

    #[test]
    fn parse_linear() {
        let xml = r#"<test value="LINEAR"/>"#;
        let result: EnumWrapper<GradationType> = from_str(xml).unwrap();
        assert_eq!(result.value, GradationType::Linear);
    }

    #[test]
    fn parse_radial() {
        let xml = r#"<test value="RADIAL"/>"#;
        let result: EnumWrapper<GradationType> = from_str(xml).unwrap();
        assert_eq!(result.value, GradationType::Radial);
    }

    #[test]
    fn parse_conical() {
        let xml = r#"<test value="CONICAL"/>"#;
        let result: EnumWrapper<GradationType> = from_str(xml).unwrap();
        assert_eq!(result.value, GradationType::Conical);
    }

    #[test]
    fn parse_square() {
        let xml = r#"<test value="SQUARE"/>"#;
        let result: EnumWrapper<GradationType> = from_str(xml).unwrap();
        assert_eq!(result.value, GradationType::Square);
    }

    #[test]
    fn serialize_linear() {
        let gt = GradationType::Linear;
        insta::assert_yaml_snapshot!("gradation_type_linear", gt);
    }
}

mod image_effect_tests {
    use super::*;

    #[test]
    fn parse_real_pic() {
        let xml = r#"<test value="REAL_PIC"/>"#;
        let result: EnumWrapper<ImageEffect> = from_str(xml).unwrap();
        assert_eq!(result.value, ImageEffect::RealPicture);
    }

    #[test]
    fn parse_gray_scale() {
        let xml = r#"<test value="GRAY_SCALE"/>"#;
        let result: EnumWrapper<ImageEffect> = from_str(xml).unwrap();
        assert_eq!(result.value, ImageEffect::GrayScale);
    }

    #[test]
    fn parse_black_white() {
        let xml = r#"<test value="BLACK_WHITE"/>"#;
        let result: EnumWrapper<ImageEffect> = from_str(xml).unwrap();
        assert_eq!(result.value, ImageEffect::BlackWhite);
    }

    #[test]
    fn default_is_real_picture() {
        assert_eq!(ImageEffect::default(), ImageEffect::RealPicture);
    }
}

mod image_brush_mode_tests {
    use super::*;

    #[test]
    fn parse_tile() {
        let xml = r#"<test value="TILE"/>"#;
        let result: EnumWrapper<ImageBrushMode> = from_str(xml).unwrap();
        assert_eq!(result.value, ImageBrushMode::Tile);
    }

    #[test]
    fn parse_tile_horz_top() {
        let xml = r#"<test value="TILE_HORZ_TOP"/>"#;
        let result: EnumWrapper<ImageBrushMode> = from_str(xml).unwrap();
        assert_eq!(result.value, ImageBrushMode::TileHorizontalTop);
    }

    #[test]
    fn parse_tile_horz_bottom() {
        let xml = r#"<test value="TILE_HORZ_BOTTOM"/>"#;
        let result: EnumWrapper<ImageBrushMode> = from_str(xml).unwrap();
        assert_eq!(result.value, ImageBrushMode::TileHorizontalBottom);
    }

    #[test]
    fn parse_tile_vert_left() {
        let xml = r#"<test value="TILE_VERT_LEFT"/>"#;
        let result: EnumWrapper<ImageBrushMode> = from_str(xml).unwrap();
        assert_eq!(result.value, ImageBrushMode::TileVerticalLeft);
    }

    #[test]
    fn parse_tile_vert_right() {
        let xml = r#"<test value="TILE_VERT_RIGHT"/>"#;
        let result: EnumWrapper<ImageBrushMode> = from_str(xml).unwrap();
        assert_eq!(result.value, ImageBrushMode::TileVerticalRight);
    }

    #[test]
    fn parse_total() {
        let xml = r#"<test value="TOTAL"/>"#;
        let result: EnumWrapper<ImageBrushMode> = from_str(xml).unwrap();
        assert_eq!(result.value, ImageBrushMode::Total);
    }

    #[test]
    fn parse_center() {
        let xml = r#"<test value="CENTER"/>"#;
        let result: EnumWrapper<ImageBrushMode> = from_str(xml).unwrap();
        assert_eq!(result.value, ImageBrushMode::Center);
    }

    #[test]
    fn default_is_tile() {
        assert_eq!(ImageBrushMode::default(), ImageBrushMode::Tile);
    }
}

mod value_unit_tests {
    use super::*;

    #[test]
    fn parse_hwpunit() {
        let xml = r#"<test value="HWPUNIT"/>"#;
        let result: EnumWrapper<ValueUnit> = from_str(xml).unwrap();
        assert_eq!(result.value, ValueUnit::HwpUnit);
    }

    #[test]
    fn parse_char() {
        let xml = r#"<test value="CHAR"/>"#;
        let result: EnumWrapper<ValueUnit> = from_str(xml).unwrap();
        assert_eq!(result.value, ValueUnit::Character);
    }

    #[test]
    fn default_is_hwpunit() {
        assert_eq!(ValueUnit::default(), ValueUnit::HwpUnit);
    }
}
