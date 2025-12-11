//! HWP → IR 변환
//!
//! HWP 문서를 IR로 변환합니다.
//!
//! 스타일 정보(폰트, 글자 모양, 문단 모양, 테두리/채우기, 스타일)를
//! 모두 IR로 변환하여 완전한 문서 표현을 제공합니다.

use crate::body::{
    Control, ControlContent, ControlType, Table as HwpTable, TableCell as HwpTableCell, CellPadding,
    Picture as HwpPicture, ImageFlip as HwpImageFlip, PictureEffectType,
    Equation as HwpEquation,
    Header as HwpHeader, Footer as HwpFooter, Footnote as HwpFootnote, Endnote as HwpEndnote,
    Hyperlink as HwpHyperlink, HyperlinkType as HwpHyperlinkType, NoteNumberingType,
    Shape as HwpShape, ShapeType as HwpShapeType,
    ArcType as HwpArcType, CurveSegmentType as HwpCurveSegmentType,
    Point as HwpPoint, ShapeBorderLine, ArrowType as HwpArrowType, ArrowSize as HwpArrowSize,
    VideoData as HwpVideoData, OleObject as HwpOleObject, ChartData as HwpChartData,
    FormObject as HwpFormObject, FormObjectType as HwpFormObjectType,
    TextArt as HwpTextArt, TextArtShape as HwpTextArtShape, TextArtAlignment as HwpTextArtAlignment,
    Field as HwpField, FieldType as HwpFieldType,
    ShapeContainer as HwpShapeContainer,
    Caption as HwpCaption, CaptionDirection as HwpCaptionDirection,
};
use crate::body::control_data::ControlData;
use crate::doc_info::{
    Alignment as HwpAlignment, BorderFill as HwpBorderFill, BreakLatinWord, BreakNonLatinWord,
    CharacterShape, FaceName, LineSpacingType as HwpLineSpacingType, ParagraphShape,
    Style as HwpStyle, VerticalAlignment as HwpVerticalAlignment,
};
use crate::HwpDocument;
use primitive::{
    Alignment, ArrowSize as IrArrowSize, ArrowType as IrArrowType, EmphasisType,
    FieldType as IrFieldType, HeaderFooterApplyTo, HeadingType as IrHeadingType,
    HorizontalRelativeTo as IrHorizontalRelativeTo,
    ImageEffect, LineCap as IrLineCap, LineBreakKorean, LineBreakLatin, LineType as IrLineType,
    NumberFormat, OutlineType, ShadowType, StrikethroughType, StyleType as IrStyleType,
    TabLeader as IrTabLeader, TabType as IrTabType,
    TextWrapSide as IrTextWrapSide, TextWrapType as IrTextWrapType,
    UnderlinePosition, UnderlineType, VerticalAlignment,
    VerticalRelativeTo as IrVerticalRelativeTo,
    BorderFillId, CharShapeId, Color, FontId, HwpUnit, Insets, ParaShapeId, Percent,
    Point as IrPoint, Size, StyleId,
};
use ir::{
    char_shape::{CharShape, EmphasisStyle, Font, FontFamily, FontRef, FontSet, ShadowStyle, UnderlineStyle},
    control::{
        AutoNumber as IrAutoNumber, AutoNumberType as IrAutoNumberType,
        Bookmark as IrBookmark, Control as IrControl, Equation as IrEquation, EquationFormat,
        HeaderFooterControl, HiddenComment as IrHiddenComment, Hyperlink as IrHyperlink,
        HyperlinkTarget, NewNumber as IrNewNumber, Note as IrNote, ObjectCommon,
        TextWrap as IrTextWrap,
        Video as IrVideo, VideoType as IrVideoType, OleObject as IrOleObject, Chart as IrChart, ChartType as IrChartType,
        FormObject as IrFormObject, FormObjectType as IrFormObjectType,
        TextArt as IrTextArt, TextArtFontStyle as IrTextArtFontStyle,
        TextArtShapeType as IrTextArtShapeType, TextArtAlignment as IrTextArtAlignment,
        Caption as IrCaption, CaptionPosition as IrCaptionPosition,
    },
    paragraph::{FieldStart as IrFieldStart, FieldEnd as IrFieldEnd},
    para_shape::{LineSpacing, LineSpacingType, LineSpacingValue, ParaShape, TabDef},
    picture::{ImageCrop as IrImageCrop, Picture as IrPicture, PictureBorder},
    shape::{
        Shape as IrShape, ShapeType as IrShapeType, LineShape as IrLineShape,
        RectangleShape as IrRectangleShape, EllipseShape as IrEllipseShape,
        ArcShape as IrArcShape, ArcType as IrArcType, PolygonShape as IrPolygonShape,
        CurveShape as IrCurveShape, CurvePoint as IrCurvePoint, CurvePointType as IrCurvePointType,
        LineStyle as IrLineStyle, Arrow as IrArrow, ShapeShadow as IrShapeShadow,
    },
    style::{Bullet as IrBullet, Numbering as IrNumbering, NumberingLevel as IrNumberingLevel, Style, StyleStore},
    table::{Table as IrTable, TableRow as IrTableRow, TableCell as IrTableCell, TableZone as IrTableZone},
    BinaryData, BinaryDataId, BinaryDataStore, BinaryFormat, ConversionError, ConversionResult,
    Document as IrDocument, Extensions, HwpExtensions, Metadata, Paragraph as IrParagraph,
    Run as IrRun, Section as IrSection,
};

use super::{ColorConvert, ToIrContext};

/// HWP → IR 변환 트레이트
pub trait HwpToIr {
    /// IR 문서로 변환
    fn to_ir(&self) -> Result<ConversionResult<IrDocument>, ConversionError>;
}

impl HwpToIr for HwpDocument {
    fn to_ir(&self) -> Result<ConversionResult<IrDocument>, ConversionError> {
        let mut ctx = ToIrContext::new();
        let doc = convert_document(self, &mut ctx)?;
        Ok(ctx.warnings.into_result(doc))
    }
}

/// 문서 변환
fn convert_document(
    hwp: &HwpDocument,
    ctx: &mut ToIrContext,
) -> Result<IrDocument, ConversionError> {
    let mut doc = IrDocument::new();

    // 메타데이터 변환
    doc.metadata = convert_metadata(hwp);

    // 스타일 변환 (완전한 변환)
    doc.styles = convert_styles(hwp, ctx)?;

    // 섹션 변환
    for section in hwp.sections() {
        let ir_section = convert_section(section, &mut doc)?;
        doc.sections.push(ir_section);
    }

    // 바이너리 데이터 변환
    doc.binary_data = convert_binary_data(hwp)?;

    // 확장 데이터 설정
    doc.extensions = convert_extensions(hwp, ctx);

    Ok(doc)
}

/// 메타데이터 변환
fn convert_metadata(hwp: &HwpDocument) -> Metadata {
    let mut metadata = Metadata::new();

    // Public API를 통해 접근 가능한 메타데이터
    if let Some(title) = hwp.title() {
        metadata.title = Some(title.to_string());
    }
    if let Some(author) = hwp.author() {
        metadata.author = Some(author.to_string());
    }
    if let Some(subject) = hwp.subject() {
        metadata.subject = Some(subject.to_string());
    }
    if let Some(keywords) = hwp.keywords() {
        metadata.keywords = keywords.split(',').map(|s| s.trim().to_string()).collect();
    }

    // 버전 정보
    let version = hwp.header().version();
    metadata.version = Some(ir::DocumentVersion::new(
        version.major() as u32,
        version.minor() as u32,
        version.build() as u32,
        version.revision() as u32,
    ));

    metadata
}

/// 스타일 저장소 변환
fn convert_styles(hwp: &HwpDocument, _ctx: &mut ToIrContext) -> Result<StyleStore, ConversionError> {
    let mut store = StyleStore::new();

    // 폰트 변환
    for face in hwp.font_faces() {
        let ir_font = convert_font_face(face);
        store.fonts.push(ir_font);
    }

    // 글자 모양 변환
    for shape in hwp.character_shapes() {
        let ir_shape = convert_char_shape(shape);
        store.char_shapes.push(ir_shape);
    }

    // 문단 모양 변환
    for shape in hwp.paragraph_shapes() {
        let ir_shape = convert_para_shape(shape);
        store.para_shapes.push(ir_shape);
    }

    // 테두리/채우기 변환
    for bf in hwp.border_fills() {
        let ir_bf = convert_border_fill(bf);
        store.border_fills.push(ir_bf);
    }

    // 스타일 변환
    for style in hwp.styles() {
        let ir_style = convert_style(style);
        store.styles.push(ir_style);
    }

    // 탭 정의 변환
    for tab_def in hwp.tab_definitions() {
        store.tab_defs.push(convert_tab_def(tab_def));
    }

    // 번호 매기기 변환
    for numbering in hwp.numberings() {
        store.numberings.push(convert_numbering(numbering));
    }

    // 글머리표 변환
    for bullet in hwp.bullets() {
        store.bullets.push(convert_bullet(bullet));
    }

    Ok(store)
}

/// 폰트 변환
fn convert_font_face(face: &FaceName) -> Font {
    use crate::doc_info::AlternateFontType;

    let mut font = Font::new(face.name());

    // 대체 폰트 이름
    if let Some(alt_name) = face.alternate_font_name() {
        font.alternate_name = Some(alt_name.to_string());
    }

    // 폰트 타입 변환
    if let Some(alt_type) = face.alternate_font_type() {
        font.font_type = match alt_type {
            AlternateFontType::TrueType => ir::char_shape::FontType::TrueType,
            AlternateFontType::HangulFont => ir::char_shape::FontType::HangeulOnly,
            AlternateFontType::Unknown => ir::char_shape::FontType::Representative,
        };
    }

    // 기본 폰트 이름
    if let Some(default_name) = face.default_font_name() {
        font.default_font_name = Some(default_name.to_string());
    }

    // 폰트 타입 정보가 있으면 변환
    if let Some(type_info) = face.font_type_info() {
        // 패밀리 설정
        font.family = match type_info.family_kind {
            1 => FontFamily::Serif,
            2 => FontFamily::SansSerif,
            3 => FontFamily::Monospace,
            4 => FontFamily::Decorative,
            5 => FontFamily::Script,
            _ => FontFamily::Unknown,
        };

        // PANOSE 정보 저장
        font.panose = Some(primitive::Panose::from_bytes([
            type_info.family_kind,
            type_info.serif_style,
            type_info.weight,
            type_info.proportion,
            type_info.contrast,
            type_info.stroke_variation,
            type_info.arm_style,
            type_info.letterform,
            type_info.midline,
            type_info.x_height,
        ]));
    }

    font
}

/// 글자 모양 변환
fn convert_char_shape(shape: &CharacterShape) -> CharShape {
    let mut ir_shape = CharShape::new();

    // 글꼴 크기
    ir_shape.font_size = HwpUnit::new(shape.base_size());

    // 굵게/기울임
    ir_shape.bold = shape.is_bold();
    ir_shape.italic = shape.is_italic();

    // 위/아래 첨자
    ir_shape.superscript = shape.is_superscript();
    ir_shape.subscript = shape.is_subscript();

    // 텍스트 색상
    let text_color = shape.text_color();
    ir_shape.color = Color::rgb(text_color.red(), text_color.green(), text_color.blue());

    // 밑줄
    ir_shape.underline = UnderlineStyle {
        line_type: convert_underline_type(shape.underline_shape()),
        position: convert_underline_position(shape.underline_position()),
        color: {
            let color = shape.underline_color();
            Some(Color::rgb(color.red(), color.green(), color.blue()))
        },
    };

    // 취소선
    ir_shape.strikethrough = convert_strikethrough_type(shape.strikethrough_shape());

    // 외곽선
    ir_shape.outline = convert_outline_type(shape.outline_type());

    // 양각/음각 효과 (별도 필드로 변환)
    ir_shape.emboss = shape.is_emboss();
    ir_shape.engrave = shape.is_engrave();

    // 강조점
    ir_shape.emphasis = EmphasisStyle {
        emphasis_type: convert_emphasis_type(shape.emphasis_type()),
        color: None,
    };

    // 그림자
    // HWP shadow offset은 i8 퍼센트 단위 (-100% ~ 100%)
    // 폰트 크기의 백분율을 나타냄. IR에서는 HwpUnit으로 저장하되,
    // 원본 퍼센트 값을 보존하기 위해 그대로 저장함.
    let (shadow_x, shadow_y) = shape.shadow_offset();
    ir_shape.shadow = ShadowStyle {
        shadow_type: convert_shadow_type(shape.shadow_type()),
        color: {
            let color = shape.shadow_color();
            Some(Color::rgb(color.red(), color.green(), color.blue()))
        },
        // 퍼센트 값을 그대로 저장 (폰트 크기 기준 상대값)
        offset_x: HwpUnit::new(shadow_x as i32),
        offset_y: HwpUnit::new(shadow_y as i32),
    };

    // 장평 (첫 번째 언어의 width_ratio 사용)
    ir_shape.char_scale = Percent::new(shape.width_ratio(crate::doc_info::LanguageType::Korean) as f64);

    // 자간 (첫 번째 언어의 spacing 사용)
    ir_shape.char_spacing = Percent::new(shape.spacing(crate::doc_info::LanguageType::Korean) as f64);

    // 폰트 설정 (각 언어별)
    let mut fonts = FontSet::default();
    for (i, &lang) in [
        crate::doc_info::LanguageType::Korean,
        crate::doc_info::LanguageType::English,
        crate::doc_info::LanguageType::Chinese,
        crate::doc_info::LanguageType::Japanese,
        crate::doc_info::LanguageType::Other,
        crate::doc_info::LanguageType::Symbol,
        crate::doc_info::LanguageType::User,
    ].iter().enumerate() {
        let font_id = shape.font_id(lang);
        let font_ref = FontRef {
            id: FontId::new(font_id as u32),
            width_ratio: Percent::new(shape.width_ratio(lang) as f64),
            spacing: Percent::new(shape.spacing(lang) as f64),
            offset: Percent::new(shape.position(lang) as f64),
            relative_size: Percent::new(shape.relative_size(lang) as f64),
        };

        match i {
            0 => fonts.korean = Some(font_ref),
            1 => fonts.english = Some(font_ref),
            2 => fonts.hanja = Some(font_ref),
            3 => fonts.japanese = Some(font_ref),
            4 => fonts.other = Some(font_ref),
            5 => fonts.symbol = Some(font_ref),
            6 => fonts.user = Some(font_ref),
            _ => {}
        }
    }
    ir_shape.fonts = fonts;

    // 글자 배경색 (shade_color가 흰색이 아닌 경우에만 설정)
    let shade_color = shape.shade_color();
    if shade_color.red() != 255 || shade_color.green() != 255 || shade_color.blue() != 255 {
        ir_shape.background_color = Some(Color::rgb(shade_color.red(), shade_color.green(), shade_color.blue()));
    }

    // 음영 색상 (shade_color를 그대로 저장)
    ir_shape.shade_color = Some(Color::rgb(shade_color.red(), shade_color.green(), shade_color.blue()));

    // 테두리/배경 참조 ID
    if let Some(border_fill_id) = shape.border_fill_id() {
        ir_shape.border_fill_id_ref = Some(primitive::BorderFillId::new(border_fill_id as u32));
    }

    // 커닝
    ir_shape.use_kerning = shape.is_kerning();

    ir_shape
}

/// 문단 모양 변환
fn convert_para_shape(shape: &ParagraphShape) -> ParaShape {
    let mut ir_shape = ParaShape::new();

    // 정렬
    ir_shape.alignment = convert_alignment(shape.alignment());

    // 여백
    ir_shape.margin_left = HwpUnit::new(shape.left_margin());
    ir_shape.margin_right = HwpUnit::new(shape.right_margin());

    // 들여쓰기
    ir_shape.first_line_indent = HwpUnit::new(shape.indent());

    // 문단 간격
    ir_shape.space_before = HwpUnit::new(shape.space_before());
    ir_shape.space_after = HwpUnit::new(shape.space_after());

    // 줄 간격
    let line_spacing_value = shape.line_spacing_value();
    ir_shape.line_spacing = match shape.line_spacing_type() {
        HwpLineSpacingType::Percent => LineSpacing {
            spacing_type: LineSpacingType::Percent,
            value: LineSpacingValue::Percent(Percent::new(line_spacing_value as f64)),
        },
        HwpLineSpacingType::Fixed => LineSpacing {
            spacing_type: LineSpacingType::Fixed,
            value: LineSpacingValue::Fixed(HwpUnit::new(line_spacing_value)),
        },
        HwpLineSpacingType::BetweenLines => LineSpacing {
            spacing_type: LineSpacingType::FontBased,
            value: LineSpacingValue::Fixed(HwpUnit::new(line_spacing_value)),
        },
        HwpLineSpacingType::Minimum => LineSpacing {
            spacing_type: LineSpacingType::AtLeast,
            value: LineSpacingValue::Fixed(HwpUnit::new(line_spacing_value)),
        },
    };

    // 줄 나눔 규칙
    ir_shape.line_break_korean = convert_break_korean(shape.break_non_latin_word());
    ir_shape.line_break_latin = convert_break_latin(shape.break_latin_word());

    // 문단 보호 옵션
    ir_shape.widow_orphan_control = shape.is_widow_orphan_protected();
    ir_shape.keep_with_next = shape.keep_with_next();
    ir_shape.keep_lines = shape.is_protected();
    ir_shape.page_break_before = shape.page_break_before();

    // 세로 정렬
    ir_shape.vertical_alignment = convert_vertical_alignment(shape.vertical_alignment());

    // 테두리/채우기 참조 및 문단 테두리 설정
    let bf_id = shape.border_fill_id();
    if bf_id > 0 {
        ir_shape.border_fill_id = Some(BorderFillId::new(bf_id as u32));

        // 문단 테두리 설정 추가
        let (border_left, border_right, border_top, border_bottom) = shape.border_margins();
        ir_shape.border = Some(ir::para_shape::ParagraphBorder {
            border_fill_id_ref: BorderFillId::new(bf_id as u32),
            offset_left: primitive::HwpUnit::new(border_left as i32),
            offset_right: primitive::HwpUnit::new(border_right as i32),
            offset_top: primitive::HwpUnit::new(border_top as i32),
            offset_bottom: primitive::HwpUnit::new(border_bottom as i32),
            connect: shape.is_border_connected(),
            ignore_margin: shape.is_margin_ignored(),
        });
    }

    // 탭 정의 참조
    let tab_id = shape.tab_definition_id();
    if tab_id > 0 {
        ir_shape.tab_def_id = Some(primitive::TabDefId::new(tab_id as u32));
    }

    // 추가 속성
    ir_shape.snap_to_grid = shape.snap_to_grid();
    ir_shape.suppress_line_numbers = shape.suppress_line_numbers();
    ir_shape.auto_spacing_east_asian_english = shape.auto_spacing_east_asian_english();
    ir_shape.auto_spacing_east_asian_number = shape.auto_spacing_east_asian_number();

    // 글꼴에 어울리는 줄 높이 - 활성화 시 기본 비율 사용
    if shape.auto_line_height() {
        ir_shape.auto_line_height_ratio = Percent::new(100.0);
    }

    // 문단 머리 (번호/글머리표) 설정
    let heading_type_ir = shape.heading_type();

    if heading_type_ir != IrHeadingType::None {
        let numbering_bullet_id = shape.numbering_bullet_id();
        ir_shape.numbering = Some(ir::para_shape::ParagraphNumbering {
            heading_type: heading_type_ir,
            numbering_id: if heading_type_ir == IrHeadingType::Number || heading_type_ir == IrHeadingType::Outline {
                if numbering_bullet_id > 0 {
                    Some(numbering_bullet_id as u32)
                } else {
                    None
                }
            } else {
                None
            },
            bullet_id: if heading_type_ir == IrHeadingType::Bullet {
                if numbering_bullet_id > 0 {
                    Some(numbering_bullet_id as u32)
                } else {
                    None
                }
            } else {
                None
            },
            level: shape.heading_level(),
        });
    }

    ir_shape
}

/// 탭 정의 변환
fn convert_tab_def(tab_def: &crate::doc_info::TabDefinition) -> TabDef {
    use ir::para_shape::Tab;
    use primitive::TabType as HwpTabType;

    let mut ir_tab_def = TabDef::default();

    // 탭 항목들 변환
    for tab_info in tab_def.tabs() {
        let ir_tab = Tab {
            position: primitive::HwpUnit::new(tab_info.position.value()),
            tab_type: match tab_info.tab_type {
                HwpTabType::Left => IrTabType::Left,
                HwpTabType::Right => IrTabType::Right,
                HwpTabType::Center => IrTabType::Center,
                HwpTabType::Decimal => IrTabType::Decimal,
            },
            leader: match tab_info.fill_type {
                0 => IrTabLeader::None,
                1 => IrTabLeader::Dot,
                2 => IrTabLeader::LongDash,
                3 => IrTabLeader::Dash,
                4 => IrTabLeader::Underscore,
                _ => IrTabLeader::None,
            },
        };
        ir_tab_def.tabs.push(ir_tab);
    }

    // 자동 탭 설정: HWP에는 왼쪽/오른쪽 자동 탭 플래그가 있지만
    // IR에서는 auto_tab_interval로 통합
    // 자동 탭이 활성화되어 있으면 기본 간격 사용
    if tab_def.has_left_auto_tab() || tab_def.has_right_auto_tab() {
        ir_tab_def.auto_tab_interval = Some(primitive::HwpUnit::new(800)); // 8pt 기본값 (1pt = 100 HwpUnit)
    } else {
        ir_tab_def.auto_tab_interval = None;
    }

    ir_tab_def
}

/// 번호 매기기 변환
fn convert_numbering(numbering: &crate::doc_info::Numbering) -> IrNumbering {
    let mut levels = Vec::new();

    for (i, level) in numbering.levels().iter().enumerate() {
        // HWP ParagraphHeadAlignment을 IR Alignment으로 변환
        let alignment = match level.head_info.alignment() {
            crate::doc_info::ParagraphHeadAlignment::Left => primitive::Alignment::Left,
            crate::doc_info::ParagraphHeadAlignment::Center => primitive::Alignment::Center,
            crate::doc_info::ParagraphHeadAlignment::Right => primitive::Alignment::Right,
        };

        // HWP number format을 IR NumberFormat으로 변환
        let number_format = convert_hwp_number_format(level.head_info.number_format());

        levels.push(IrNumberingLevel {
            level: i as u8,
            format: level.format.clone(),
            char_shape_id: if level.head_info.character_shape_id() > 0 {
                Some(CharShapeId::new(level.head_info.character_shape_id()))
            } else {
                None
            },
            text_offset: level.head_info.text_distance().value() as i32,
            number_width: level.head_info.width_correction().value() as i32,
            start_number: level.start_number,
            alignment,
            use_instance_width: level.head_info.use_instance_width(),
            auto_indent: level.head_info.auto_indent(),
            number_format,
        });
    }

    IrNumbering {
        name: None, // HWP 번호 매기기에는 이름이 없음
        levels,
        start_number: numbering.levels().first()
            .map(|l| l.start_number)
            .unwrap_or(1),
    }
}

/// 글머리표 변환
fn convert_bullet(bullet: &crate::doc_info::Bullet) -> IrBullet {
    IrBullet {
        char: bullet.bullet_char(),
        char_shape_id: if bullet.head_info().character_shape_id() > 0 {
            Some(CharShapeId::new(bullet.head_info().character_shape_id()))
        } else {
            None
        },
        is_checkbox: bullet.check_bullet_char() != '\0' && bullet.check_bullet_char() != bullet.bullet_char(),
    }
}

/// HWP 번호 형식을 IR NumberFormat으로 변환
fn convert_hwp_number_format(format: u8) -> NumberFormat {
    // HWP 스펙 표 41: 문단 번호 형식
    match format {
        0 => NumberFormat::Digit,              // 1, 2, 3
        1 => NumberFormat::CircledDigit,       // ①, ②, ③
        2 => NumberFormat::RomanUpper,         // I, II, III
        3 => NumberFormat::RomanLower,         // i, ii, iii
        4 => NumberFormat::LatinUpper,         // A, B, C
        5 => NumberFormat::LatinLower,         // a, b, c
        6 => NumberFormat::CircledLatinUpper,  // Ⓐ, Ⓑ, Ⓒ
        7 => NumberFormat::CircledLatinLower,  // ⓐ, ⓑ, ⓒ
        8 => NumberFormat::HangulSyllable,     // 가, 나, 다
        9 => NumberFormat::CircledHangul,      // ㉮, ㉯, ㉰
        10 => NumberFormat::HangulJamo,        // ㄱ, ㄴ, ㄷ
        11 => NumberFormat::CircledHangulJamo, // 원 한글 자모
        12 => NumberFormat::HangulIdeograph,   // 일, 이, 삼
        13 => NumberFormat::Ideograph,         // 一, 二, 三
        14 => NumberFormat::CircledIdeograph,  // 원 한자
        _ => NumberFormat::Digit,              // 기본값
    }
}

/// FillInfo를 IR Fill로 변환
fn convert_fill_info(fill_info: &crate::doc_info::FillInfo) -> ir::border_fill::Fill {
    use crate::doc_info::{
        FillInfo, PatternType as HwpPatternType,
        GradientType as HwpGradientType, ImageFillType as HwpImageFillType,
    };
    use ir::border_fill::{
        Fill, SolidFill, GradientFill, ImageFill, PatternFill,
        GradientStop, PatternType as IrPatternType,
    };
    use primitive::{GradientType as IrGradientType, ImageFillMode};
    use primitive::Color;
    use primitive::BinaryDataId;

    // 색상 변환 헬퍼
    let convert_color = |color: crate::primitive::ColorReference| -> Color {
        let value = color.value();
        Color {
            red: (value & 0xFF) as u8,
            green: ((value >> 8) & 0xFF) as u8,
            blue: ((value >> 16) & 0xFF) as u8,
            alpha: 255,
        }
    };

    match fill_info {
        FillInfo::None => Fill::None,
        FillInfo::Pattern(pattern) => {
            // PatternType::None인 경우 단색 채우기로 처리
            if pattern.pattern_type == HwpPatternType::None {
                Fill::Solid(SolidFill {
                    color: convert_color(pattern.background_color),
                })
            } else {
                Fill::Pattern(PatternFill {
                    pattern_type: match pattern.pattern_type {
                        HwpPatternType::None => IrPatternType::Horizontal,
                        HwpPatternType::Horizontal => IrPatternType::Horizontal,
                        HwpPatternType::Vertical => IrPatternType::Vertical,
                        HwpPatternType::BackSlash => IrPatternType::DiagonalDown,
                        HwpPatternType::Slash => IrPatternType::DiagonalUp,
                        HwpPatternType::Cross => IrPatternType::Grid,
                        HwpPatternType::CrossDiagonal => IrPatternType::DiagonalGrid,
                    },
                    foreground: convert_color(pattern.pattern_color),
                    background: convert_color(pattern.background_color),
                })
            }
        }
        FillInfo::Gradient(gradient) => {
            let gradient_type = match gradient.gradient_type {
                HwpGradientType::Linear => IrGradientType::Linear,
                HwpGradientType::Radial => IrGradientType::Radial,
                HwpGradientType::Conical => IrGradientType::Conical,
                HwpGradientType::Rectangular => IrGradientType::Square,
            };

            let stops: Vec<GradientStop> = gradient.colors.iter().enumerate()
                .map(|(i, color)| {
                    let position = if gradient.colors.len() == 1 {
                        0
                    } else {
                        (i * 100 / (gradient.colors.len() - 1)) as u8
                    };
                    GradientStop {
                        position,
                        color: convert_color(*color),
                    }
                })
                .collect();

            Fill::Gradient(GradientFill {
                gradient_type,
                // HWP angle은 i16 (-180~180), IR은 u16 (0-360)
                // 음수 각도를 0-360 범위로 정규화
                angle: if gradient.angle < 0 {
                    (360 + gradient.angle as i32) as u16
                } else {
                    gradient.angle as u16
                },
                // center_x/y는 0-100% 범위, 음수는 0으로 클램프
                center_x: gradient.center_x.clamp(0, 100) as u8,
                center_y: gradient.center_y.clamp(0, 100) as u8,
                stops,
                // blur는 0-255 범위, 음수는 0으로 클램프
                blur: gradient.blur.clamp(0, 255) as u8,
                step_center: 50, // HWP 5.0 doesn't have step_center, use default
            })
        }
        FillInfo::Image(image) => {
            let mode = match image.fill_type {
                HwpImageFillType::TileAll => ImageFillMode::Tile,
                HwpImageFillType::TileHorizontalTop => ImageFillMode::TileHorizontalTop,
                HwpImageFillType::TileHorizontalBottom => ImageFillMode::TileHorizontalBottom,
                HwpImageFillType::TileVerticalLeft => ImageFillMode::TileVerticalLeft,
                HwpImageFillType::TileVerticalRight => ImageFillMode::TileVerticalRight,
                HwpImageFillType::FitToSize => ImageFillMode::Stretch,
                HwpImageFillType::Center => ImageFillMode::Center,
                HwpImageFillType::CenterTop => ImageFillMode::CenterTop,
                HwpImageFillType::CenterBottom => ImageFillMode::CenterBottom,
                HwpImageFillType::CenterLeft => ImageFillMode::CenterLeft,
                HwpImageFillType::TopLeft => ImageFillMode::TopLeft,
                HwpImageFillType::BottomLeft => ImageFillMode::BottomLeft,
                HwpImageFillType::CenterRight => ImageFillMode::CenterRight,
                HwpImageFillType::TopRight => ImageFillMode::TopRight,
                HwpImageFillType::BottomRight => ImageFillMode::BottomRight,
                HwpImageFillType::None => ImageFillMode::Original,
            };

            let effect = match image.image_info.effect {
                0 => primitive::ImageEffect::Original,
                1 => primitive::ImageEffect::Grayscale,
                2 => primitive::ImageEffect::BlackWhite,
                _ => primitive::ImageEffect::Original,
            };

            Fill::Image(ImageFill {
                binary_id: BinaryDataId::new(image.image_info.binary_data_id.to_string()),
                mode,
                brightness: image.image_info.brightness,
                contrast: image.image_info.contrast,
                effect,
            })
        }
    }
}

/// 테두리/채우기 변환
fn convert_border_fill(bf: &HwpBorderFill) -> ir::border_fill::BorderFill {
    use ir::border_fill::{Border, BorderFill as IrBorderFill};
    use primitive::{BorderLineStyle, Color, HwpUnit, LineType};

    // 테두리 스타일 변환 헬퍼
    let convert_line_type = |style: BorderLineStyle| -> LineType {
        match style {
            BorderLineStyle::Solid => LineType::Solid,
            BorderLineStyle::LongDash | BorderLineStyle::LongDashAlt => LineType::LongDash,
            BorderLineStyle::Dash => LineType::Dash,
            BorderLineStyle::DashDot => LineType::DashDot,
            BorderLineStyle::DashDotDot => LineType::DashDotDot,
            BorderLineStyle::Circle => LineType::Circle,
            BorderLineStyle::Double => LineType::Double,
            BorderLineStyle::ThinThick => LineType::ThinThickLarge,
            BorderLineStyle::ThickThin => LineType::ThickThinLarge,
            BorderLineStyle::ThinThickThin => LineType::Triple,
            BorderLineStyle::Wave => LineType::Wave,
            BorderLineStyle::DoubleWave => LineType::DoubleWave,
            BorderLineStyle::Thick3D | BorderLineStyle::Thick3DReversed |
            BorderLineStyle::Single3D | BorderLineStyle::Single3DReversed => LineType::Solid,
        }
    };

    // 색상 변환 헬퍼
    let convert_color = |color: crate::primitive::ColorReference| -> Color {
        let value = color.value();
        Color {
            red: (value & 0xFF) as u8,
            green: ((value >> 8) & 0xFF) as u8,
            blue: ((value >> 16) & 0xFF) as u8,
            alpha: 255,
        }
    };

    // 테두리 변환 헬퍼
    let convert_border = |index: usize| -> Border {
        let styles = bf.border_styles();
        let thicknesses = bf.border_thicknesses();
        let colors = bf.border_colors();

        Border {
            line_type: convert_line_type(styles[index]),
            width: HwpUnit::from_mm(thicknesses[index].value_mm()),
            color: convert_color(colors[index]),
        }
    };

    // 4개 테두리 변환 (left=0, right=1, top=2, bottom=3)
    let left = convert_border(0);
    let right = convert_border(1);
    let top = convert_border(2);
    let bottom = convert_border(3);

    // 대각선 변환
    let diagonal_down = if bf.diagonal_style() != BorderLineStyle::Solid || bf.diagonal_thickness().value_hundredths_mm() > 0 {
        Some(Border {
            line_type: convert_line_type(bf.diagonal_style()),
            width: HwpUnit::from_mm(bf.diagonal_thickness().value_mm()),
            color: convert_color(bf.diagonal_color()),
        })
    } else {
        None
    };

    // 채우기 변환 (별도 함수 사용)
    let fill = convert_fill_info(bf.fill_info());

    IrBorderFill {
        left,
        right,
        top,
        bottom,
        diagonal_down,
        diagonal_up: None, // HWP doesn't have separate diagonal_up
        fill,
        is_3d: bf.has_3d_effect(),
        has_shadow: bf.has_shadow(),
    }
}

/// 스타일 변환
fn convert_style(style: &HwpStyle) -> Style {
    use crate::doc_info::StyleType as HwpStyleType;
    let style_type = match style.style_type() {
        HwpStyleType::Paragraph => IrStyleType::Paragraph,
        HwpStyleType::Character => IrStyleType::Character,
    };

    Style {
        name: style.local_name().to_string(),
        english_name: if style.english_name().is_empty() {
            None
        } else {
            Some(style.english_name().to_string())
        },
        style_type,
        para_shape_id: Some(ParaShapeId::new(style.paragraph_shape_id() as u32)),
        char_shape_id: Some(CharShapeId::new(style.character_shape_id() as u32)),
        next_style_id: if style.next_style_id() > 0 {
            Some(StyleId::new(style.next_style_id() as u32))
        } else {
            None
        },
    }
}

/// 섹션 변환 컨텍스트 (각주/미주 모양 정보)
struct SectionContext<'a> {
    footnote_shape: Option<&'a crate::body::FootnoteShape>,
    endnote_shape: Option<&'a crate::body::EndnoteShape>,
}

/// 섹션 변환
fn convert_section(section: &crate::Section, ir_doc: &mut IrDocument) -> Result<IrSection, ConversionError> {
    let mut ir_section = IrSection::default();

    // 페이지 정의 변환
    if let Some(page_def) = section.page_definition() {
        ir_section.page = convert_page_definition(page_def);
    }

    // 페이지 테두리/배경 변환
    if let Some(border_fill) = section.page_border_fill() {
        ir_section.page_border_fill = Some(convert_page_border_fill(border_fill));
    }

    // 각주 모양 변환
    if let Some(footnote_shape) = section.footnote_shape() {
        ir_section.footnote_shape = Some(convert_footnote_shape(footnote_shape));
    }

    // 미주 모양 변환
    if let Some(endnote_shape) = section.endnote_shape() {
        ir_section.endnote_shape = Some(convert_endnote_shape(endnote_shape));
    }

    // 섹션/단 정의 컨트롤에서 정보 추출
    // HWP에서는 섹션 정의와 단 정의가 컨트롤로 첫 문단에 포함됨
    for para in section.paragraphs() {
        for control in para.controls() {
            // 섹션 정의에서 시작 번호, 가시성, 그리드 설정 추출
            if let Some(secd) = control.as_section_definition() {
                // 시작 번호 정보 추출
                let page_starts_on = match secd.page_starts_on() {
                    0 => primitive::PageStartsOn::Both,
                    1 => primitive::PageStartsOn::Even,
                    2 => primitive::PageStartsOn::Odd,
                    _ => primitive::PageStartsOn::Both,
                };
                ir_section.start_number = ir::section::SectionStartNumber {
                    page_starts_on,
                    page: secd.starting_page_number as u32,
                    picture: secd.starting_figure_number as u32,
                    table: secd.starting_table_number as u32,
                    equation: secd.starting_equation_number as u32,
                };
                // 가시성 설정 추출
                ir_section.extensions.visibility = ir::section::SectionVisibility {
                    hide_header: secd.hide_header(),
                    hide_footer: secd.hide_footer(),
                    hide_master_page: secd.hide_master_page(),
                    hide_border: secd.hide_border(),
                    hide_background: secd.hide_background(),
                    hide_page_number: secd.hide_page_number(),
                    ..Default::default()
                };
                // 그리드 설정 추출
                ir_section.extensions.grid = ir::section::SectionGrid {
                    line_grid: secd.vertical_grid as u32,
                    character_grid: secd.horizontal_grid as u32,
                    manuscript_format: false, // HWP에는 원고지 형식 플래그가 직접 없음
                };
                // representative_language 저장 (첫 번째 섹션의 값을 문서 설정에 반영)
                if ir_doc.settings.representative_language.is_none() {
                    ir_doc.settings.representative_language = Some(secd.language);
                }
            }
            // 단 정의에서 단 설정 추출
            if let Some(cold) = control.as_column_definition() {
                ir_section.columns = convert_column_definition(cold);
            }
        }
    }

    // 섹션 컨텍스트 생성 (각주/미주 모양 정보 포함)
    let section_context = SectionContext {
        footnote_shape: section.footnote_shape(),
        endnote_shape: section.endnote_shape(),
    };

    // 문단 변환
    for para in section.paragraphs() {
        let ir_para = convert_paragraph_with_context(para, &section_context)?;
        ir_section.paragraphs.push(ir_para);
    }

    Ok(ir_section)
}

/// 단 정의 변환
fn convert_column_definition(cold: &crate::body::ColumnDefinition) -> ir::section::ColumnDefinition {
    use crate::body::ColumnDirection as HwpColumnDirection;
    use ir::section::{ColumnDefinition as IrColumnDefinition, ColumnDirection as IrColumnDirection, ColumnSeparator};

    // 단 방향 변환
    let direction = match cold.direction() {
        HwpColumnDirection::LeftToRight => IrColumnDirection::LeftToRight,
        HwpColumnDirection::RightToLeft => IrColumnDirection::RightToLeft,
        HwpColumnDirection::FacingPages => IrColumnDirection::FacingPages,
    };

    // 단 구분선 스타일 변환
    let separator = match cold.separator_style {
        0 => ColumnSeparator::None,
        1 => ColumnSeparator::Solid,
        2 => ColumnSeparator::Dash,
        3 => ColumnSeparator::Dot,
        _ => ColumnSeparator::None,
    };

    // 단 너비 변환 (HWP 단위 → IR HwpUnit)
    let widths: Vec<HwpUnit> = cold.column_widths
        .iter()
        .map(|&w| HwpUnit::new(w as i32))
        .collect();

    // 구분선 색상 변환
    let (r, g, b) = cold.separator_color.to_rgb();
    let separator_color = primitive::Color::rgb(r, g, b);

    IrColumnDefinition {
        count: cold.column_count() as u16,
        direction,
        gap: HwpUnit::new(cold.column_gap as i32),
        separator,
        separator_thickness: cold.separator_thickness,
        separator_color,
        widths,
    }
}

/// 페이지 정의 변환
fn convert_page_definition(page_def: &crate::PageDefinition) -> ir::section::PageDefinition {
    // 용지 방향
    let orientation = match page_def.orientation {
        crate::PageOrientation::Portrait => primitive::PageOrientation::Portrait,
        crate::PageOrientation::Landscape => primitive::PageOrientation::Landscape,
    };

    // 제본 여백 위치 및 여백 - From trait 사용
    let gutter_position: primitive::GutterPosition = page_def.gutter_position.into();
    let margins: primitive::PageMargins = page_def.margins.into();

    ir::section::PageDefinition {
        width: page_def.width,
        height: page_def.height,
        margins,
        orientation,
        gutter_position,
    }
}

/// 페이지 테두리/배경 변환
fn convert_page_border_fill(border_fill: &crate::PageBorderFill) -> ir::section::PageBorderFill {
    use ir::section::{PageBorderFill as IrPageBorderFill, PageBorderPosition};

    let position = match border_fill.position {
        crate::PageBorderFillPosition::Paper => PageBorderPosition::Paper,
        crate::PageBorderFillPosition::Body => PageBorderPosition::Body,
    };

    IrPageBorderFill {
        border_fill_id: primitive::BorderFillId::new(border_fill.border_fill_id as u32),
        position,
        offsets: Insets {
            left: HwpUnit::new(border_fill.offset_left.value()),
            right: HwpUnit::new(border_fill.offset_right.value()),
            top: HwpUnit::new(border_fill.offset_top.value()),
            bottom: HwpUnit::new(border_fill.offset_bottom.value()),
        },
        first_page_only: false, // HWP에는 이 속성이 없음
        header_inside: border_fill.include_header,
        footer_inside: border_fill.include_footer,
        fill_behind: border_fill.fill_behind,
        page_type: ir::section::PageBorderPageType::Both, // HWP에는 없음
        fill_area: ir::section::PageBorderFillArea::Paper, // HWP에는 없음
    }
}

/// HWP 선 종류를 IR LineType으로 변환
fn convert_line_type(line_type: u8) -> primitive::LineType {
    match line_type {
        0 => primitive::LineType::None,
        1 => primitive::LineType::Solid,
        2 => primitive::LineType::Dash,
        3 => primitive::LineType::Dot,
        4 => primitive::LineType::DashDot,
        5 => primitive::LineType::DashDotDot,
        6 => primitive::LineType::LongDash,
        _ => primitive::LineType::Solid, // 기본값
    }
}

// ColorConvert::to_ir을 사용합니다 (색상 변환 통합)

/// 각주 모양 변환
fn convert_footnote_shape(footnote_shape: &crate::FootnoteShape) -> ir::section::FootnoteShape {
    use primitive::{FootnotePlacement, NoteNumbering};

    // 번호 매김 방식 결정: placement와 continue_numbering 조합
    let numbering = match (footnote_shape.placement, footnote_shape.continue_numbering) {
        (_, true) => NoteNumbering::Continuous, // 문서 전체 연속
        (crate::body::NotePlacement::EndOfPage, false) => NoteNumbering::RestartPage, // 페이지당 새로
        (crate::body::NotePlacement::EndOfSection, false) => NoteNumbering::RestartSection, // 섹션당 새로
        _ => NoteNumbering::Continuous, // 기본값
    };

    // HWP placement를 IR FootnotePlacement로 변환
    // HWP에서는 단(column) 배치 개념이 없으므로 기본값 사용
    let placement = FootnotePlacement::EachColumn;

    ir::section::FootnoteShape {
        base: ir::section::NoteShape {
            number_format: convert_note_numbering_type_to_format(footnote_shape.numbering_type),
            numbering,
            superscript: footnote_shape.superscript,
            prefix: footnote_shape.prefix.clone(),
            suffix: footnote_shape.suffix.clone(),
            start_number: footnote_shape.start_number as u32,
            user_character: footnote_shape.custom_symbol.clone(),
            separator_length: HwpUnit::new(footnote_shape.separator_length.value()), // HwpUnit 직접 사용
            separator_position: Some(HwpUnit::new(footnote_shape.separator_position.value())),
            separator_line_type: convert_line_type(footnote_shape.separator_line_type),
            separator_line_width: footnote_shape.separator_line_thickness,
            separator_line_color: ColorConvert::to_ir(footnote_shape.separator_line_color),
            space_above: HwpUnit::new(footnote_shape.space_above.value()),
            space_below: HwpUnit::new(footnote_shape.space_below.value()),
            space_between: HwpUnit::new(footnote_shape.space_between.value()),
            beneath_text: false, // HWP에서는 지원하지 않음
        },
        placement,
    }
}

/// 미주 모양 변환
fn convert_endnote_shape(endnote_shape: &crate::EndnoteShape) -> ir::section::EndnoteShape {
    use primitive::{EndnotePlacement, NoteNumbering};

    // 미주 번호 매김: 대부분 연속 또는 섹션별
    let numbering = if endnote_shape.continue_numbering {
        NoteNumbering::Continuous
    } else {
        NoteNumbering::RestartSection
    };

    // HWP placement를 IR EndnotePlacement로 변환
    let placement = match endnote_shape.placement {
        crate::body::NotePlacement::EndOfDocument => EndnotePlacement::EndOfDocument,
        crate::body::NotePlacement::EndOfSection => EndnotePlacement::EndOfSection,
        _ => EndnotePlacement::EndOfDocument,
    };

    ir::section::EndnoteShape {
        base: ir::section::NoteShape {
            number_format: convert_note_numbering_type_to_format(endnote_shape.numbering_type),
            numbering,
            superscript: endnote_shape.superscript,
            prefix: endnote_shape.prefix.clone(),
            suffix: endnote_shape.suffix.clone(),
            start_number: endnote_shape.start_number as u32,
            user_character: endnote_shape.custom_symbol.clone(),
            separator_length: HwpUnit::ZERO, // 미주는 구분선 없음
            separator_position: None, // 미주는 구분선 위치 없음
            separator_line_type: primitive::LineType::None,
            separator_line_width: 0,
            separator_line_color: primitive::Color::BLACK,
            space_above: HwpUnit::default(),
            space_below: HwpUnit::default(),
            space_between: HwpUnit::new(endnote_shape.space_between.value()),
            beneath_text: false,
        },
        placement,
    }
}

/// 각주/미주 번호 형식 변환
fn convert_note_numbering_type_to_format(numbering_type: NoteNumberingType) -> NumberFormat {
    match numbering_type {
        NoteNumberingType::Arabic => NumberFormat::Digit,
        NoteNumberingType::CircledNumbers => NumberFormat::CircledDigit,
        NoteNumberingType::UpperRoman => NumberFormat::RomanUpper,
        NoteNumberingType::LowerRoman => NumberFormat::RomanLower,
        NoteNumberingType::UpperAlpha => NumberFormat::LatinUpper,
        NoteNumberingType::LowerAlpha => NumberFormat::LatinLower,
        NoteNumberingType::Custom => NumberFormat::Digit,
    }
}


/// RangeTag 변환 (HWP → IR)
///
/// HWP의 RangeTag를 IR RangeTag로 변환합니다.
/// 태그의 상위 바이트(tag[2])가 태그 종류를 나타냅니다:
/// - 0: 책갈피 (Bookmark)
/// - 1: 하이퍼링크 (Hyperlink)
/// - 2: 변경 추적 - 삽입 (TrackChangeInsert)
/// - 3: 변경 추적 - 삭제 (TrackChangeDelete)
/// - 4: 형광펜 (Highlight)
/// - 기타: Other
fn convert_range_tag(range_tag: &crate::body::RangeTag) -> ir::paragraph::RangeTag {
    use ir::paragraph::{RangeTag as IrRangeTag, RangeTagType, TrackChangeInfo};

    let tag = range_tag.tag;
    let tag_type_byte = tag[2]; // 상위 바이트가 태그 종류
    let tag_data_low = tag[0];
    let tag_data_mid = tag[1];

    // 태그 종류 결정
    let tag_type = match tag_type_byte {
        0 => RangeTagType::Bookmark,
        1 => RangeTagType::Hyperlink,
        2 => RangeTagType::TrackChangeInsert,
        3 => RangeTagType::TrackChangeDelete,
        4 => RangeTagType::Highlight,
        other => RangeTagType::Other(other),
    };

    // 변경 추적 정보 추출 (TrackChangeInsert/Delete의 경우)
    let track_change_info = if matches!(tag_type, RangeTagType::TrackChangeInsert | RangeTagType::TrackChangeDelete) {
        // 하위 16비트를 변경 추적 ID로 사용
        let track_change_id = u16::from_le_bytes([tag_data_low, tag_data_mid]) as u32;
        Some(TrackChangeInfo {
            track_change_id,
            tag_id: None,
            paragraph_end: false,
        })
    } else {
        None
    };

    // 태그 데이터 문자열 생성 (16진수 표현)
    let data = if tag_data_low != 0 || tag_data_mid != 0 {
        Some(format!("{:02x}{:02x}", tag_data_mid, tag_data_low))
    } else {
        None
    };

    IrRangeTag {
        start: range_tag.start_position,
        end: range_tag.end_position,
        tag_type,
        data,
        track_change_info,
    }
}

/// 문단 변환 (컨텍스트 포함)
fn convert_paragraph_with_context(para: &crate::Paragraph, ctx: &SectionContext) -> Result<IrParagraph, ConversionError> {
    let mut ir_para = IrParagraph::new();

    // 문단 모양 ID
    ir_para.para_shape_id = Some(ParaShapeId::new(para.paragraph_shape_id() as u32));

    // 스타일 ID
    ir_para.style_id = Some(StyleId::new(para.style_id() as u32));

    // 인스턴스 ID
    ir_para.instance_id = Some(para.instance_id());

    // Break Type 변환
    ir_para.break_type = convert_paragraph_break_type(para.break_type());

    // RangeTag 변환 (범위 태그 - 변경 추적, 형광펜 등)
    for range_tag in para.range_tags() {
        let ir_range_tag = convert_range_tag(range_tag);
        ir_para.range_tags.push(ir_range_tag);
    }

    // 텍스트 변환 - plain_text() 메서드 사용
    let plain_text = para.plain_text();
    if !plain_text.is_empty() {
        let run = IrRun::text(plain_text);
        ir_para.runs.push(run);
    }

    // 컨트롤 (표, 그림 등) 변환 및 Caption 연결
    // Caption은 이전 Table/Picture/Shape 등에 연결해야 함
    let mut pending_caption: Option<IrCaption> = None;
    let mut field_id_counter: u32 = 0;

    for control in para.controls() {
        // Caption 컨트롤 먼저 체크
        if let Some(ControlContent::Caption(caption)) = control.content() {
            // Caption을 IR Caption으로 변환하고 보류
            if let Ok(ir_caption) = convert_caption(caption) {
                pending_caption = Some(ir_caption);
            }
            continue;
        }

        // Field 컨트롤 특별 처리 (Hyperlink 제외 - FieldStart/FieldEnd로 변환)
        if let Some(ControlContent::Field(field)) = control.content()
            && field.field_type() != HwpFieldType::Hyperlink
        {
            // 비-하이퍼링크 필드는 FieldStart/FieldEnd로 변환
            let field_id = field_id_counter;
            field_id_counter += 1;

            let field_start = convert_field_to_field_start(field, field_id, control.data());
            let field_end = IrFieldEnd { id: field_id };

            // 필드 값(표시 텍스트)도 포함
            let mut run = IrRun::new();
            run.contents.push(ir::paragraph::RunContent::FieldStart(field_start));

            // 필드의 표시 텍스트 추가 (있는 경우)
            let display_text = field.display_text();
            if !display_text.is_empty() {
                run.contents.push(ir::paragraph::RunContent::Text(ir::paragraph::Text::new(display_text)));
            }

            run.contents.push(ir::paragraph::RunContent::FieldEnd(field_end));
            ir_para.runs.push(run);
            continue;
            // Hyperlink 필드는 아래 일반 컨트롤 변환으로 처리
        }

        // 일반 컨트롤 변환 (섹션 컨텍스트 전달)
        if let Some(mut ir_control) = convert_control_with_context(control, ctx)? {
            // 보류된 Caption이 있으면 해당 컨트롤에 연결
            if let Some(caption) = pending_caption.take() {
                match &mut ir_control {
                    IrControl::Table(table) => {
                        table.common.caption = Some(caption);
                    }
                    IrControl::Picture(picture) => {
                        picture.common.caption = Some(caption);
                    }
                    IrControl::Equation(equation) => {
                        equation.common.caption = Some(caption);
                    }
                    IrControl::Shape(shape) => {
                        shape.common.caption = Some(caption);
                    }
                    _ => {
                        // 다른 컨트롤 타입에는 Caption을 연결할 수 없음
                    }
                }
            }
            let run = IrRun::control(ir_control);
            ir_para.runs.push(run);
        }
    }

    // 남은 Caption이 있고, runs에 캡션을 연결할 수 있는 컨트롤이 있으면 연결
    if let Some(caption) = pending_caption {
        // 뒤에서부터 검색하여 가장 최근의 Table/Picture 등에 연결
        'outer: for run in ir_para.runs.iter_mut().rev() {
            for content in run.contents.iter_mut().rev() {
                if let ir::paragraph::RunContent::Control(ctrl) = content {
                    let connected = match ctrl.as_mut() {
                        IrControl::Table(table) => {
                            table.common.caption = Some(caption.clone());
                            true
                        }
                        IrControl::Picture(picture) => {
                            picture.common.caption = Some(caption.clone());
                            true
                        }
                        IrControl::Equation(equation) => {
                            equation.common.caption = Some(caption.clone());
                            true
                        }
                        IrControl::Shape(shape) => {
                            shape.common.caption = Some(caption.clone());
                            true
                        }
                        _ => false,
                    };
                    if connected {
                        break 'outer;
                    }
                }
            }
        }
    }

    Ok(ir_para)
}

/// 컨트롤 변환 (섹션 컨텍스트 포함)
fn convert_control_with_context(control: &Control, ctx: &SectionContext) -> Result<Option<IrControl>, ConversionError> {
    // 개체 공통 속성이 있는 컨트롤에서 ObjectCommon 파싱
    let object_common = parse_object_common(control.data());

    match control.content() {
        Some(ControlContent::Table(table)) => {
            let mut ir_table = convert_table(table)?;
            if let Some(common) = object_common {
                ir_table.common = common;
            }
            Ok(Some(IrControl::Table(Box::new(ir_table))))
        }
        Some(ControlContent::Picture(picture)) => {
            let mut ir_picture = convert_picture(picture)?;
            if let Some(common) = object_common {
                ir_picture.common = common;
            }
            Ok(Some(IrControl::Picture(Box::new(ir_picture))))
        }
        Some(ControlContent::Equation(equation)) => {
            let mut ir_equation = convert_equation(equation)?;
            if let Some(common) = object_common {
                ir_equation.common = common;
            }
            Ok(Some(IrControl::Equation(Box::new(ir_equation))))
        }
        Some(ControlContent::Header(header)) => {
            let ir_header = convert_header(header)?;
            Ok(Some(IrControl::Header(Box::new(ir_header))))
        }
        Some(ControlContent::Footer(footer)) => {
            let ir_footer = convert_footer(footer)?;
            Ok(Some(IrControl::Footer(Box::new(ir_footer))))
        }
        Some(ControlContent::Footnote(footnote)) => {
            // 섹션 컨텍스트에서 각주 모양 정보 사용
            let ir_footnote = convert_footnote_with_shape(footnote, ctx.footnote_shape)?;
            Ok(Some(IrControl::Footnote(Box::new(ir_footnote))))
        }
        Some(ControlContent::Endnote(endnote)) => {
            // 섹션 컨텍스트에서 미주 모양 정보 사용
            let ir_endnote = convert_endnote_with_shape(endnote, ctx.endnote_shape)?;
            Ok(Some(IrControl::Endnote(Box::new(ir_endnote))))
        }
        Some(ControlContent::Hyperlink(hyperlink)) => {
            let ir_hyperlink = convert_hyperlink(hyperlink)?;
            Ok(Some(IrControl::Hyperlink(Box::new(ir_hyperlink))))
        }
        Some(ControlContent::Shape(shape)) => {
            let mut ir_shape = convert_shape(shape, ctx)?;
            if let Some(common) = object_common {
                ir_shape.common = common;
            }
            Ok(Some(IrControl::Shape(Box::new(ir_shape))))
        }
        Some(ControlContent::TextBox(text_box)) => {
            let mut ir_text_box = convert_text_box(text_box, control)?;
            if let Some(common) = object_common {
                ir_text_box.common = common;
            }
            Ok(Some(IrControl::TextBox(Box::new(ir_text_box))))
        }
        Some(ControlContent::Video(video)) => {
            let mut ir_video = convert_video(video, control)?;
            if let Some(common) = object_common {
                ir_video.common = common;
            }
            Ok(Some(IrControl::Video(Box::new(ir_video))))
        }
        Some(ControlContent::OleObject(ole)) => {
            let mut ir_ole = convert_ole(ole, control)?;
            if let Some(common) = object_common {
                ir_ole.common = common;
            }
            Ok(Some(IrControl::Ole(Box::new(ir_ole))))
        }
        Some(ControlContent::Chart(chart)) => {
            let mut ir_chart = convert_chart(chart, control)?;
            if let Some(common) = object_common {
                ir_chart.common = common;
            }
            Ok(Some(IrControl::Chart(Box::new(ir_chart))))
        }
        Some(ControlContent::FormObject(form)) => {
            let mut ir_form = convert_form_object(form, control)?;
            if let Some(common) = object_common {
                ir_form.common = common;
            }
            Ok(Some(IrControl::FormObject(Box::new(ir_form))))
        }
        Some(ControlContent::TextArt(text_art)) => {
            let mut ir_text_art = convert_text_art(text_art, control)?;
            if let Some(common) = object_common {
                ir_text_art.common = common;
            }
            Ok(Some(IrControl::TextArt(Box::new(ir_text_art))))
        }
        Some(ControlContent::Field(field)) => {
            // 하이퍼링크 필드는 IR Hyperlink로 변환
            if field.field_type() == HwpFieldType::Hyperlink {
                let ir_hyperlink = convert_field_to_hyperlink(field)?;
                Ok(Some(IrControl::Hyperlink(Box::new(ir_hyperlink))))
            } else {
                // 기타 필드 타입은 convert_paragraph에서 FieldStart/FieldEnd로 변환됨
                Ok(None)
            }
        }
        Some(ControlContent::Caption(caption)) => {
            // Caption은 표/그림의 캡션이므로 별도 처리 필요
            // 현재는 무시
            let _ = caption;
            Ok(None)
        }
        Some(ControlContent::Container(container)) => {
            // 그룹 도형 - ShapeType::Group으로 변환
            let mut ir_shape = convert_container(container, ctx)?;
            if let Some(common) = object_common {
                ir_shape.common = common;
            }
            Ok(Some(IrControl::Shape(Box::new(ir_shape))))
        }
        Some(ControlContent::Memo(memo)) => {
            // 메모를 IR Memo로 변환
            // 메모 내부에서는 각주/미주가 없으므로 기본 컨텍스트 사용
            let empty_ctx = SectionContext {
                footnote_shape: None,
                endnote_shape: None,
            };
            let ir_paragraphs: Vec<IrParagraph> = memo.paragraphs()
                .iter()
                .map(|p| convert_paragraph_with_context(p, &empty_ctx))
                .collect::<Result<Vec<_>, _>>()?;

            let ir_memo = ir::control::Memo {
                paragraphs: ir_paragraphs,
                author: None, // HWP 바이너리에는 작성자 정보가 없음
                date: None, // HWP 바이너리에는 작성일 정보가 없음
                width: Some(HwpUnit::new(memo.shape().width() as i32)),
                line_width: None, // HWP에서 추출 불가
                line_color: None, // TODO: MemoShape에서 border_color 파싱 필요
                fill_color: None, // TODO: MemoShape에서 fill_color 파싱 필요
                active_color: None, // HWP에는 없음
                memo_type: None, // HWP에는 없음 (HWPX 전용)
            };
            Ok(Some(IrControl::Memo(Box::new(ir_memo))))
        }
        Some(ControlContent::SectionDefinition(_secd)) => {
            // 구역 정의는 Section 레벨에서 처리되므로 컨트롤로는 변환하지 않음
            // Section의 columns, starting_page_number 등에 반영됨
            Ok(None)
        }
        Some(ControlContent::ColumnDefinition(_cold)) => {
            // 단 정의는 Section 레벨에서 columns로 처리되므로 컨트롤로는 변환하지 않음
            Ok(None)
        }
        // 컨트롤 타입으로 처리하는 경우 (ControlContent가 없는 컨트롤들)
        None => {
            match control.control_type() {
                ControlType::Bookmark => {
                    let ir_bookmark = convert_bookmark(control)?;
                    Ok(Some(IrControl::Bookmark(Box::new(ir_bookmark))))
                }
                ControlType::AutoNumber => {
                    let ir_auto_number = convert_auto_number(control)?;
                    Ok(Some(IrControl::AutoNumber(Box::new(ir_auto_number))))
                }
                ControlType::NewNumber => {
                    let ir_new_number = convert_new_number(control)?;
                    Ok(Some(IrControl::NewNumber(Box::new(ir_new_number))))
                }
                ControlType::HiddenComment => {
                    let ir_hidden_comment = convert_hidden_comment(control)?;
                    Ok(Some(IrControl::HiddenComment(Box::new(ir_hidden_comment))))
                }
                ControlType::PageNumberPosition => {
                    let ir_auto_number = convert_page_number_to_auto_number(control)?;
                    Ok(Some(IrControl::AutoNumber(Box::new(ir_auto_number))))
                }
                ControlType::IndexMark => {
                    let ir_index_mark = convert_index_mark(control)?;
                    Ok(Some(IrControl::IndexMark(Box::new(ir_index_mark))))
                }
                // 아직 구현되지 않은 컨트롤 타입들
                _ => Ok(None),
            }
        }
    }
}

/// 캡션 변환
fn convert_caption(caption: &HwpCaption) -> Result<IrCaption, ConversionError> {
    // 캡션 위치 변환
    let position = match caption.direction() {
        HwpCaptionDirection::Below => IrCaptionPosition::Bottom,
        HwpCaptionDirection::Above => IrCaptionPosition::Top,
        HwpCaptionDirection::Left => IrCaptionPosition::Left,
        HwpCaptionDirection::Right => IrCaptionPosition::Right,
    };

    // 캡션 문단 변환 (캡션 내부에서는 각주/미주가 없으므로 빈 컨텍스트 사용)
    let empty_ctx = SectionContext {
        footnote_shape: None,
        endnote_shape: None,
    };
    let paragraphs: Vec<IrParagraph> = caption.paragraphs()
        .iter()
        .filter_map(|para| convert_paragraph_with_context(para, &empty_ctx).ok())
        .collect();

    Ok(IrCaption {
        position,
        width: HwpUnit::default(),  // 캡션 너비는 HWP에서 따로 지정하지 않음
        gap: HwpUnit::new(caption.gap()),
        paragraphs,
    })
}

/// 표 변환
fn convert_table(table: &HwpTable) -> Result<IrTable, ConversionError> {
    let props = &table.properties;

    let mut ir_table = IrTable::new(props.row_count, props.column_count);
    ir_table.cell_spacing = HwpUnit::new(props.cell_spacing.value() as i32);
    ir_table.border_fill_id = if props.border_fill_id > 0 {
        Some(BorderFillId::new(props.border_fill_id as u32))
    } else {
        None
    };

    // 반복 제목행 설정
    if props.auto_repeat_title_row() {
        ir_table.header_row_count = 1;
        ir_table.repeat_header = true;
    }

    // 페이지 나눔 설정 변환
    ir_table.page_break = match props.page_border_split() {
        crate::body::table::PageBorderSplit::NoSplit => ir::table::TablePageBreak::None,
        crate::body::table::PageBorderSplit::SplitByCell => ir::table::TablePageBreak::Cell,
        crate::body::table::PageBorderSplit::NoSplitAlt => ir::table::TablePageBreak::None,
    };

    // 표 안쪽 여백 변환
    ir_table.inside_margin = Some(Insets {
        left: HwpUnit::new(props.padding.left.value() as i32),
        right: HwpUnit::new(props.padding.right.value() as i32),
        top: HwpUnit::new(props.padding.top.value() as i32),
        bottom: HwpUnit::new(props.padding.bottom.value() as i32),
    });

    // 영역 정보 (병합 영역) 변환
    for zone in &props.zones {
        ir_table.zones.push(IrTableZone {
            start_row: zone.start_row,
            start_column: zone.start_column,
            end_row: zone.end_row,
            end_column: zone.end_column,
            border_fill_id: if zone.border_fill_id > 0 {
                Some(BorderFillId::new(zone.border_fill_id as u32))
            } else {
                None
            },
        });
    }

    // 행별로 셀 변환
    for row_idx in 0..props.row_count {
        let mut ir_row = IrTableRow::new();

        // 행 높이 설정
        if let Some(height) = props.row_sizes.get(row_idx as usize) {
            ir_row.height = HwpUnit::new(height.value() as i32);
        }

        // 해당 행의 셀들 변환
        for cell in table.cells_in_row(row_idx) {
            let ir_cell = convert_table_cell(cell)?;
            ir_row.cells.push(ir_cell);
        }

        ir_table.rows.push(ir_row);
    }

    Ok(ir_table)
}

/// 표 셀 변환
fn convert_table_cell(cell: &HwpTableCell) -> Result<IrTableCell, ConversionError> {
    let mut ir_cell = IrTableCell::new(cell.row, cell.column);

    ir_cell.column_span = cell.column_span;
    ir_cell.row_span = cell.row_span;
    ir_cell.width = HwpUnit::new(cell.width.value());
    ir_cell.height = HwpUnit::new(cell.height.value());
    ir_cell.padding = convert_cell_padding(&cell.padding);

    ir_cell.border_fill_id = if cell.border_fill_id > 0 {
        Some(BorderFillId::new(cell.border_fill_id as u32))
    } else {
        None
    };

    // 셀 내용 (문단들) 변환 (표 셀 내부에서는 각주/미주가 없으므로 빈 컨텍스트 사용)
    let empty_ctx = SectionContext {
        footnote_shape: None,
        endnote_shape: None,
    };
    for para in &cell.paragraphs {
        ir_cell.paragraphs.push(convert_paragraph_with_context(para, &empty_ctx)?);
    }

    Ok(ir_cell)
}

/// 셀 여백 변환
fn convert_cell_padding(padding: &CellPadding) -> Insets {
    Insets {
        left: HwpUnit::new(padding.left.value() as i32),
        right: HwpUnit::new(padding.right.value() as i32),
        top: HwpUnit::new(padding.top.value() as i32),
        bottom: HwpUnit::new(padding.bottom.value() as i32),
    }
}

/// 바이너리 데이터 변환
fn convert_binary_data(hwp: &HwpDocument) -> Result<BinaryDataStore, ConversionError> {
    let mut store = BinaryDataStore::new();

    for id in hwp.binary_data_ids() {
        if let Some(data) = hwp.get_binary_data(id) {
            let format = detect_binary_format(data);
            let ir_data = BinaryData::new(format, data.to_vec());
            let ir_id = BinaryDataId::from_numeric(id);
            store.add(ir_id, ir_data);
        }
    }

    Ok(store)
}

/// 바이너리 데이터 형식 감지
fn detect_binary_format(data: &[u8]) -> BinaryFormat {
    if data.len() < 8 {
        return BinaryFormat::Unknown;
    }

    // 매직 바이트로 형식 감지
    match &data[0..4] {
        [0x89, b'P', b'N', b'G'] => BinaryFormat::Png,
        [0xFF, 0xD8, 0xFF, _] => BinaryFormat::Jpg,
        [b'G', b'I', b'F', b'8'] => BinaryFormat::Gif,
        [b'B', b'M', _, _] => BinaryFormat::Bmp,
        [0xD0, 0xCF, 0x11, 0xE0] => BinaryFormat::Ole,
        _ => {
            // TIFF 체크
            if (data[0..4] == [0x49, 0x49, 0x2A, 0x00])
                || (data[0..4] == [0x4D, 0x4D, 0x00, 0x2A])
            {
                BinaryFormat::Tiff
            } else {
                BinaryFormat::Unknown
            }
        }
    }
}

/// 확장 데이터 변환
fn convert_extensions(hwp: &HwpDocument, ctx: &mut ToIrContext) -> Extensions {
    let mut ext = Extensions::new();

    // HWP 고유 데이터
    let hwp_ext = HwpExtensions::default();

    // 배포용 문서 확인
    if hwp.is_distribution_document() {
        ctx.warnings
            .data_loss("배포용 문서 데이터는 HWPX로 변환 시 손실됩니다");
    }

    // 스크립트 확인
    if hwp.has_scripts() {
        ctx.warnings.data_loss("스크립트는 HWPX로 변환 시 손실됩니다");
    }

    ext.hwp = Some(hwp_ext);

    ext
}

// 열거형 변환 헬퍼 함수들

fn convert_underline_type(shape: primitive::UnderlineShape) -> UnderlineType {
    use primitive::UnderlineShape;
    match shape {
        UnderlineShape::Solid => UnderlineType::Single,
        UnderlineShape::Double => UnderlineType::Double,
        UnderlineShape::Wave | UnderlineShape::DoubleWave => UnderlineType::Wave,
        UnderlineShape::Dash => UnderlineType::Dash,
        UnderlineShape::LongDash | UnderlineShape::LongDashAlt => UnderlineType::Dash,
        UnderlineShape::DashDot => UnderlineType::DashDot,
        UnderlineShape::DashDotDot => UnderlineType::DashDotDot,
        UnderlineShape::Circle => UnderlineType::Dotted,
        // 두께 관련 스타일은 Thick으로 매핑
        UnderlineShape::ThinThick | UnderlineShape::ThickThin | UnderlineShape::ThinThickThin => UnderlineType::Thick,
        // 3D 스타일은 가장 유사한 것으로 매핑
        UnderlineShape::Thick3D | UnderlineShape::Thick3DReversed => UnderlineType::Thick,
        UnderlineShape::Single3D | UnderlineShape::Single3DReversed => UnderlineType::Single,
    }
}

fn convert_underline_position(pos: crate::doc_info::UnderlinePosition) -> UnderlinePosition {
    match pos {
        crate::doc_info::UnderlinePosition::Bottom => UnderlinePosition::Bottom,
        crate::doc_info::UnderlinePosition::Top => UnderlinePosition::Top,
        crate::doc_info::UnderlinePosition::None => UnderlinePosition::Bottom,
    }
}

fn convert_strikethrough_type(shape: crate::doc_info::StrikethroughShape) -> StrikethroughType {
    match shape {
        crate::doc_info::StrikethroughShape::None => StrikethroughType::None,
        crate::doc_info::StrikethroughShape::Single => StrikethroughType::Single,
        crate::doc_info::StrikethroughShape::Double => StrikethroughType::Double,
    }
}

fn convert_outline_type(outline: crate::doc_info::OutlineType) -> OutlineType {
    use crate::doc_info::OutlineType as HwpOutlineType;
    match outline {
        HwpOutlineType::None => OutlineType::None,
        // 모든 스타일의 외곽선은 Outline으로 매핑 (선 스타일 정보는 손실됨)
        HwpOutlineType::Solid
        | HwpOutlineType::Dotted
        | HwpOutlineType::Thick
        | HwpOutlineType::Dashed
        | HwpOutlineType::DashDot
        | HwpOutlineType::DashDotDot => OutlineType::Outline,
    }
}

fn convert_emphasis_type(emphasis: crate::doc_info::EmphasisType) -> EmphasisType {
    use crate::doc_info::EmphasisType as HwpEmphasisType;
    match emphasis {
        HwpEmphasisType::None => EmphasisType::None,
        HwpEmphasisType::FilledCircle => EmphasisType::Circle,
        HwpEmphasisType::OpenCircle => EmphasisType::CircleOpen,
        HwpEmphasisType::Dot => EmphasisType::Dot,
        HwpEmphasisType::Colon => EmphasisType::Colon,
        HwpEmphasisType::Caron => EmphasisType::Caron,
        HwpEmphasisType::Tilde => EmphasisType::Tilde,
    }
}

fn convert_shadow_type(shadow: crate::doc_info::ShadowType) -> ShadowType {
    match shadow {
        crate::doc_info::ShadowType::None => ShadowType::None,
        crate::doc_info::ShadowType::Discrete => ShadowType::BottomRightDiscrete,
        crate::doc_info::ShadowType::Continuous => ShadowType::BottomRightContinuous,
    }
}

fn convert_alignment(align: HwpAlignment) -> Alignment {
    match align {
        HwpAlignment::Justify => Alignment::Justify,
        HwpAlignment::Left => Alignment::Left,
        HwpAlignment::Right => Alignment::Right,
        HwpAlignment::Center => Alignment::Center,
        HwpAlignment::Distribute => Alignment::Distribute,
        HwpAlignment::Divide => Alignment::Divide,
    }
}

fn convert_vertical_alignment(align: HwpVerticalAlignment) -> VerticalAlignment {
    match align {
        HwpVerticalAlignment::Baseline => VerticalAlignment::Baseline,
        HwpVerticalAlignment::Top => VerticalAlignment::Top,
        HwpVerticalAlignment::Center => VerticalAlignment::Middle,
        HwpVerticalAlignment::Bottom => VerticalAlignment::Bottom,
    }
}

fn convert_break_korean(break_type: BreakNonLatinWord) -> LineBreakKorean {
    match break_type {
        BreakNonLatinWord::Word => LineBreakKorean::Word,
        BreakNonLatinWord::Character => LineBreakKorean::Character,
    }
}

fn convert_break_latin(break_type: BreakLatinWord) -> LineBreakLatin {
    match break_type {
        BreakLatinWord::Word => LineBreakLatin::Word,
        BreakLatinWord::Hyphen => LineBreakLatin::Hyphenation,
        BreakLatinWord::Character => LineBreakLatin::Character,
    }
}

/// 문단 나누기 타입 변환
fn convert_paragraph_break_type(break_type: crate::body::BreakType) -> primitive::BreakType {
    // HWP BreakType은 비트 플래그 조합 (SECTION=0x01, MULTI_COLUMN=0x02, PAGE=0x04, COLUMN=0x08)
    // IR BreakType은 단일 값 (None, Page, Column, Section)
    // 우선순위: Section > Page > Column > None
    if break_type.is_section_break() {
        primitive::BreakType::Section
    } else if break_type.is_page_break() {
        primitive::BreakType::Page
    } else if break_type.is_column_break() {
        primitive::BreakType::Column
    } else {
        primitive::BreakType::None
    }
}

// =============================================================================
// 컨트롤 변환 함수들
// =============================================================================

/// 그림 변환
fn convert_picture(picture: &HwpPicture) -> Result<IrPicture, ConversionError> {
    let props = &picture.properties;
    let binary_id = BinaryDataId::from_numeric(props.binary_data_id);

    let mut ir_picture = IrPicture::new(binary_id);

    // 원본 크기 (픽셀 → HwpUnit 변환, 대략적 계산: 96 DPI 가정)
    let (width, height) = props.image_dimension;
    if width > 0 && height > 0 {
        // 픽셀을 HwpUnit으로 변환 (1 inch = 96 pixels = 7200 HwpUnits)
        ir_picture.original_size = Size {
            width: HwpUnit::new((width as i32 * 7200) / 96),
            height: HwpUnit::new((height as i32 * 7200) / 96),
        };
    }

    // 자르기 변환
    ir_picture.crop = IrImageCrop {
        left: HwpUnit::new(props.crop.left.value()),
        right: HwpUnit::new(props.crop.right.value()),
        top: HwpUnit::new(props.crop.top.value()),
        bottom: HwpUnit::new(props.crop.bottom.value()),
    };

    // 안쪽 여백 변환
    ir_picture.inside_margin = Insets {
        left: HwpUnit::new(props.inner_margin.left.value() as i32),
        right: HwpUnit::new(props.inner_margin.right.value() as i32),
        top: HwpUnit::new(props.inner_margin.top.value() as i32),
        bottom: HwpUnit::new(props.inner_margin.bottom.value() as i32),
    };

    // 이미지 효과 변환
    ir_picture.effect = convert_picture_effect(props.effect.effect_type);
    ir_picture.brightness = props.effect.brightness;
    ir_picture.contrast = props.effect.contrast;

    // 테두리 변환
    if props.has_border() {
        let border_color = props.border_color;
        // HWP border_transparency: 0 = 불투명, 100 = 완전투명
        // IR alpha: 0 = 투명, 255 = 불투명
        let alpha = if props.border_transparency >= 100 {
            0
        } else {
            ((100 - props.border_transparency as u16) * 255 / 100) as u8
        };
        ir_picture.border = Some(PictureBorder {
            line_type: IrLineType::Solid,
            width: HwpUnit::new(props.border_thickness),
            // COLORREF는 0x00BBGGRR 형식 (BGR 순서)
            color: Color::argb(
                alpha,
                (border_color & 0xFF) as u8,
                ((border_color >> 8) & 0xFF) as u8,
                ((border_color >> 16) & 0xFF) as u8,
            ),
        });
    }

    // 뒤집기 변환
    ir_picture.flip = convert_image_flip(picture.flip);

    // 회전 변환
    ir_picture.rotation = picture.rotation as f64;

    // 투명 색상 변환 (COLORREF는 0x00BBGGRR 형식)
    if let Some(transparent_color) = props.transparent_color {
        ir_picture.transparent_color = Some(Color::rgb(
            (transparent_color & 0xFF) as u8,
            ((transparent_color >> 8) & 0xFF) as u8,
            ((transparent_color >> 16) & 0xFF) as u8,
        ));
    }

    Ok(ir_picture)
}

/// 이미지 효과 타입 변환
fn convert_picture_effect(effect: PictureEffectType) -> ImageEffect {
    match effect {
        PictureEffectType::RealPicture => ImageEffect::Original,
        PictureEffectType::Grayscale => ImageEffect::Grayscale,
        PictureEffectType::BlackWhite => ImageEffect::BlackWhite,
        PictureEffectType::Pattern8x8 => ImageEffect::Pattern,
    }
}

/// 이미지 뒤집기 변환
fn convert_image_flip(flip: HwpImageFlip) -> primitive::ImageFlip {
    match flip {
        HwpImageFlip::None => primitive::ImageFlip::None,
        HwpImageFlip::Horizontal => primitive::ImageFlip::Horizontal,
        HwpImageFlip::Vertical => primitive::ImageFlip::Vertical,
        HwpImageFlip::Both => primitive::ImageFlip::Both,
    }
}

/// 수식 변환
fn convert_equation(equation: &HwpEquation) -> Result<IrEquation, ConversionError> {
    use crate::body::EquationLineMode as HwpLineMode;
    use ir::control::EquationLineMode as IrLineMode;

    // 라인 모드 변환
    let line_mode = match equation.properties.line_mode {
        HwpLineMode::Baseline => IrLineMode::Baseline,
        HwpLineMode::Center => IrLineMode::Center,
        HwpLineMode::Bottom => IrLineMode::Bottom,
        HwpLineMode::Top => IrLineMode::Top,
    };

    Ok(IrEquation {
        common: ObjectCommon::default(),
        script: equation.properties.script.clone(),
        format: EquationFormat::HwpScript,
        baseline_offset: HwpUnit::new(equation.properties.baseline_offset as i32),
        font_size: HwpUnit::new(equation.properties.base_font_size as i32), // 이미 points * 100 형식
        // COLORREF는 0x00BBGGRR 형식 (BGR 순서)
        color: {
            let color = equation.properties.text_color;
            Some(Color::rgb(
                (color & 0xFF) as u8,
                ((color >> 8) & 0xFF) as u8,
                ((color >> 16) & 0xFF) as u8,
            ))
        },
        line_mode: Some(line_mode),
        version: if !equation.properties.version.is_empty() {
            Some(equation.properties.version.clone())
        } else {
            None
        },
        font_name: if !equation.properties.font_name.is_empty() {
            Some(equation.properties.font_name.clone())
        } else {
            None
        },
        properties: Some(equation.properties.properties),
    })
}

/// 머리글 변환
fn convert_header(header: &HwpHeader) -> Result<HeaderFooterControl, ConversionError> {
    let apply_to = convert_header_footer_target(header.target());

    // 머리글 내부에서는 각주/미주가 없으므로 빈 컨텍스트 사용
    let empty_ctx = SectionContext {
        footnote_shape: None,
        endnote_shape: None,
    };

    let mut paragraphs = Vec::new();
    for para in header.paragraphs() {
        paragraphs.push(convert_paragraph_with_context(para, &empty_ctx)?);
    }

    Ok(HeaderFooterControl {
        apply_to,
        paragraphs,
    })
}

/// 바닥글 변환
fn convert_footer(footer: &HwpFooter) -> Result<HeaderFooterControl, ConversionError> {
    let apply_to = convert_header_footer_target(footer.target());

    // 바닥글 내부에서는 각주/미주가 없으므로 빈 컨텍스트 사용
    let empty_ctx = SectionContext {
        footnote_shape: None,
        endnote_shape: None,
    };

    let mut paragraphs = Vec::new();
    for para in footer.paragraphs() {
        paragraphs.push(convert_paragraph_with_context(para, &empty_ctx)?);
    }

    Ok(HeaderFooterControl {
        apply_to,
        paragraphs,
    })
}

/// 머리글/바닥글 대상 변환
fn convert_header_footer_target(target: crate::body::HeaderFooterTarget) -> HeaderFooterApplyTo {
    match target {
        crate::body::HeaderFooterTarget::BothPages => HeaderFooterApplyTo::Both,
        crate::body::HeaderFooterTarget::EvenPages => HeaderFooterApplyTo::Even,
        crate::body::HeaderFooterTarget::OddPages => HeaderFooterApplyTo::Odd,
    }
}

/// 각주 변환 (섹션의 각주 모양 정보 활용)
fn convert_footnote_with_shape(
    footnote: &HwpFootnote,
    footnote_shape: Option<&crate::body::FootnoteShape>
) -> Result<IrNote, ConversionError> {
    // 각주 내부 문단에서는 각주/미주가 없으므로 빈 컨텍스트 사용
    let empty_ctx = SectionContext {
        footnote_shape: None,
        endnote_shape: None,
    };

    let mut paragraphs = Vec::new();
    for para in footnote.paragraphs() {
        paragraphs.push(convert_paragraph_with_context(para, &empty_ctx)?);
    }

    // 섹션의 각주 모양에서 번호 형식과 위치 정보 가져오기
    let (number_format, number_position) = if let Some(shape) = footnote_shape {
        let format = convert_note_numbering_type_to_format(shape.numbering_type);
        let position = if shape.superscript {
            primitive::NoteNumberPosition::Superscript
        } else {
            primitive::NoteNumberPosition::Subscript
        };
        (format, position)
    } else {
        (NumberFormat::Digit, primitive::NoteNumberPosition::Superscript)
    };

    Ok(IrNote {
        number: footnote.number() as u32,
        number_format,
        number_position,
        paragraphs,
        instance_id: None,
    })
}

/// 미주 변환 (섹션의 미주 모양 정보 활용)
fn convert_endnote_with_shape(
    endnote: &HwpEndnote,
    endnote_shape: Option<&crate::body::EndnoteShape>
) -> Result<IrNote, ConversionError> {
    // 미주 내부 문단에서는 각주/미주가 없으므로 빈 컨텍스트 사용
    let empty_ctx = SectionContext {
        footnote_shape: None,
        endnote_shape: None,
    };

    let mut paragraphs = Vec::new();
    for para in endnote.paragraphs() {
        paragraphs.push(convert_paragraph_with_context(para, &empty_ctx)?);
    }

    // 섹션의 미주 모양에서 번호 형식과 위치 정보 가져오기
    let (number_format, number_position) = if let Some(shape) = endnote_shape {
        let format = convert_note_numbering_type_to_format(shape.numbering_type);
        let position = if shape.superscript {
            primitive::NoteNumberPosition::Superscript
        } else {
            primitive::NoteNumberPosition::Subscript
        };
        (format, position)
    } else {
        (NumberFormat::Digit, primitive::NoteNumberPosition::Superscript)
    };

    Ok(IrNote {
        number: endnote.number() as u32,
        number_format,
        number_position,
        paragraphs,
        instance_id: None,
    })
}

/// 하이퍼링크 변환
fn convert_hyperlink(hyperlink: &HwpHyperlink) -> Result<IrHyperlink, ConversionError> {
    let target_str = hyperlink.target().to_string();
    let target = match hyperlink.link_type() {
        HwpHyperlinkType::Url => HyperlinkTarget::Url(target_str),
        HwpHyperlinkType::File => HyperlinkTarget::File(target_str),
        HwpHyperlinkType::Bookmark => HyperlinkTarget::Bookmark(target_str),
        HwpHyperlinkType::Email => HyperlinkTarget::Email(target_str),
    };

    Ok(IrHyperlink {
        target,
        tooltip: None,
        display_text: None, // HWP에서 display_text 추출은 별도 구현 필요
    })
}

/// 도형 변환
fn convert_shape(shape: &HwpShape, ctx: &SectionContext) -> Result<IrShape, ConversionError> {
    use primitive::Point as IrPoint;

    // 도형 타입 변환 (화살표 정보도 전달)
    let shape_type = convert_shape_type(&shape.shape_type, &shape.border_line, ctx)?;

    // 선 스타일 변환
    let line = convert_border_line(&shape.border_line);

    // 회전 각도 변환 (i16 → f64)
    let rotation = shape.element_properties.rotation as f64;

    // 도형 크기 계산
    let size = calculate_shape_size(&shape.shape_type);

    // 공통 속성 설정 - 중심점 정보 활용
    let center_x = shape.element_properties.center_x.value();
    let center_y = shape.element_properties.center_y.value();
    let common = ObjectCommon {
        id: None,
        position: IrPoint {
            x: HwpUnit::new(center_x),
            y: HwpUnit::new(center_y),
        },
        size,
        z_order: 0,
        text_wrap: ir::control::TextWrap::default(),
        caption: None,
        numbering_type: None,
        shape_comment: None,
        meta_tag: None,
        dirty: false,
        width_relative_to: primitive::WidthRelativeTo::Absolute,
        height_relative_to: primitive::HeightRelativeTo::Absolute,
        margin: ir::control::ObjectMargin::default(),
    };

    // 채우기 변환 (파서에서 추출한 정보 사용)
    let fill = convert_fill_info(&shape.fill);

    // 도형 내부 텍스트 정보 변환
    let text = if shape.text_box.is_some() || !shape.paragraphs.is_empty() {
        use ir::shape::ShapeText;
        use primitive::Insets;
        use primitive::{VerticalAlignment, TextDirection};

        // 문단 변환
        let paragraphs: Vec<IrParagraph> = shape.paragraphs.iter()
            .filter_map(|para| convert_paragraph_with_context(para, ctx).ok())
            .collect();

        // 텍스트박스 속성이 있으면 사용, 없으면 기본값
        let (padding, vertical_alignment, text_direction) = if let Some(tb) = &shape.text_box {
            (
                Insets {
                    left: HwpUnit::new(tb.margin_left as i32),
                    right: HwpUnit::new(tb.margin_right as i32),
                    top: HwpUnit::new(tb.margin_top as i32),
                    bottom: HwpUnit::new(tb.margin_bottom as i32),
                },
                VerticalAlignment::Top,
                TextDirection::Horizontal,
            )
        } else {
            (Insets::default(), VerticalAlignment::Top, TextDirection::Horizontal)
        };

        Some(ShapeText {
            paragraphs,
            padding,
            vertical_alignment,
            text_direction,
            editable: true,
        })
    } else {
        None
    };

    // 변환 행렬 추출
    // HWP에서는 matrix 배열에 6개 이상의 값을 저장
    // 첫 6개: translation, 다음 6개: scale, 마지막 6개: rotation (예상)
    let (translation_matrix, scale_matrix, rotation_matrix) = {
        use ir::shape::TransformMatrix;
        let matrix = &shape.element_properties.matrix;

        let translation = if matrix.len() >= 6 {
            TransformMatrix::from_hwp_matrix(&matrix[0..6])
        } else {
            None
        };

        let scale = if matrix.len() >= 12 {
            TransformMatrix::from_hwp_matrix(&matrix[6..12])
        } else {
            None
        };

        let rotation_mat = if matrix.len() >= 18 {
            TransformMatrix::from_hwp_matrix(&matrix[12..18])
        } else {
            None
        };

        (translation, scale, rotation_mat)
    };

    Ok(IrShape {
        common,
        shape_type,
        line,
        fill,
        shadow: None,
        rotation,
        text,
        translation_matrix,
        scale_matrix,
        rotation_matrix,
    })
}

/// 도형 타입에서 크기 계산
fn calculate_shape_size(shape_type: &HwpShapeType) -> Size {
    match shape_type {
        HwpShapeType::Line(line) => {
            // 선의 경우 시작점과 끝점의 바운딩 박스
            let width = (line.end.x.value() - line.start.x.value()).abs();
            let height = (line.end.y.value() - line.start.y.value()).abs();
            Size {
                width: HwpUnit::new(width.max(1)), // 최소 1
                height: HwpUnit::new(height.max(1)),
            }
        }
        HwpShapeType::Rectangle(rect) => {
            // 4개 꼭짓점에서 바운딩 박스 계산
            let xs: Vec<i32> = rect.corners.iter().map(|p| p.x.value()).collect();
            let ys: Vec<i32> = rect.corners.iter().map(|p| p.y.value()).collect();
            let min_x = xs.iter().copied().min().unwrap_or(0);
            let max_x = xs.iter().copied().max().unwrap_or(0);
            let min_y = ys.iter().copied().min().unwrap_or(0);
            let max_y = ys.iter().copied().max().unwrap_or(0);
            Size {
                width: HwpUnit::new((max_x - min_x).max(1)),
                height: HwpUnit::new((max_y - min_y).max(1)),
            }
        }
        HwpShapeType::Ellipse(ellipse) => {
            // axis1, axis2에서 크기 계산 (중심에서 축까지의 거리 * 2)
            let dx1 = (ellipse.axis1.x.value() - ellipse.center.x.value()).abs();
            let dy1 = (ellipse.axis1.y.value() - ellipse.center.y.value()).abs();
            let dx2 = (ellipse.axis2.x.value() - ellipse.center.x.value()).abs();
            let dy2 = (ellipse.axis2.y.value() - ellipse.center.y.value()).abs();
            // 두 축 중 더 큰 것 선택
            let width = (dx1.max(dx2) * 2).max(1);
            let height = (dy1.max(dy2) * 2).max(1);
            Size {
                width: HwpUnit::new(width),
                height: HwpUnit::new(height),
            }
        }
        HwpShapeType::Arc(arc) => {
            // Arc도 Ellipse와 유사
            let dx1 = (arc.axis1.x.value() - arc.center.x.value()).abs();
            let dy1 = (arc.axis1.y.value() - arc.center.y.value()).abs();
            let dx2 = (arc.axis2.x.value() - arc.center.x.value()).abs();
            let dy2 = (arc.axis2.y.value() - arc.center.y.value()).abs();
            let width = (dx1.max(dx2) * 2).max(1);
            let height = (dy1.max(dy2) * 2).max(1);
            Size {
                width: HwpUnit::new(width),
                height: HwpUnit::new(height),
            }
        }
        HwpShapeType::Polygon(polygon) => {
            // 모든 점의 바운딩 박스
            if polygon.points.is_empty() {
                return Size::default();
            }
            let xs: Vec<i32> = polygon.points.iter().map(|p| p.x.value()).collect();
            let ys: Vec<i32> = polygon.points.iter().map(|p| p.y.value()).collect();
            let min_x = xs.iter().copied().min().unwrap_or(0);
            let max_x = xs.iter().copied().max().unwrap_or(0);
            let min_y = ys.iter().copied().min().unwrap_or(0);
            let max_y = ys.iter().copied().max().unwrap_or(0);
            Size {
                width: HwpUnit::new((max_x - min_x).max(1)),
                height: HwpUnit::new((max_y - min_y).max(1)),
            }
        }
        HwpShapeType::Curve(curve) => {
            // 모든 점의 바운딩 박스
            if curve.points.is_empty() {
                return Size::default();
            }
            let xs: Vec<i32> = curve.points.iter().map(|p| p.x.value()).collect();
            let ys: Vec<i32> = curve.points.iter().map(|p| p.y.value()).collect();
            let min_x = xs.iter().copied().min().unwrap_or(0);
            let max_x = xs.iter().copied().max().unwrap_or(0);
            let min_y = ys.iter().copied().min().unwrap_or(0);
            let max_y = ys.iter().copied().max().unwrap_or(0);
            Size {
                width: HwpUnit::new((max_x - min_x).max(1)),
                height: HwpUnit::new((max_y - min_y).max(1)),
            }
        }
        HwpShapeType::Container(shapes) => {
            // 그룹의 경우 모든 자식 도형의 바운딩 박스
            if shapes.is_empty() {
                return Size::default();
            }
            let child_sizes: Vec<Size> = shapes.iter()
                .map(|s| calculate_shape_size(&s.shape_type))
                .collect();
            let max_width = child_sizes.iter().map(|s| s.width.value()).max().unwrap_or(0);
            let max_height = child_sizes.iter().map(|s| s.height.value()).max().unwrap_or(0);
            Size {
                width: HwpUnit::new(max_width),
                height: HwpUnit::new(max_height),
            }
        }
        HwpShapeType::Unknown => Size::default(),
    }
}

/// HWP 도형 타입 → IR 도형 타입 변환
fn convert_shape_type(hwp_type: &HwpShapeType, border_line: &ShapeBorderLine, ctx: &SectionContext) -> Result<IrShapeType, ConversionError> {
    match hwp_type {
        HwpShapeType::Line(line) => {
            Ok(IrShapeType::Line(IrLineShape {
                start: convert_hwp_point(&line.start),
                end: convert_hwp_point(&line.end),
                start_arrow: convert_hwp_arrow(border_line.start_arrow(), border_line.start_arrow_size()),
                end_arrow: convert_hwp_arrow(border_line.end_arrow(), border_line.end_arrow_size()),
            }))
        }
        HwpShapeType::Rectangle(rect) => {
            // 모서리 반지름을 HwpUnit으로 변환 (비율 → 실제 값)
            Ok(IrShapeType::Rectangle(IrRectangleShape {
                corner_radius: HwpUnit::new(rect.round_ratio as i32 * 100),
            }))
        }
        HwpShapeType::Ellipse(ellipse) => {
            // 중심점과 start/end 점에서 각도 계산
            let (start_angle, end_angle) = calculate_arc_angles(
                &ellipse.center,
                &ellipse.start,
                &ellipse.end,
            );
            Ok(IrShapeType::Ellipse(IrEllipseShape {
                arc_type: convert_hwp_arc_type(ellipse.arc_type()),
                start_angle,
                end_angle,
            }))
        }
        HwpShapeType::Arc(arc) => {
            // Arc는 start/end 점이 없어서 axis에서 유추
            let (start_angle, end_angle) = calculate_arc_angles(
                &arc.center,
                &arc.axis1,
                &arc.axis2,
            );
            Ok(IrShapeType::Arc(IrArcShape {
                arc_type: convert_hwp_arc_type(arc.arc_type),
                start_angle,
                end_angle,
            }))
        }
        HwpShapeType::Polygon(polygon) => {
            let points = polygon.points.iter()
                .map(convert_hwp_point)
                .collect();
            Ok(IrShapeType::Polygon(IrPolygonShape { points }))
        }
        HwpShapeType::Curve(curve) => {
            let points: Vec<IrCurvePoint> = curve.points.iter()
                .zip(curve.segment_types.iter())
                .map(|(p, t)| IrCurvePoint {
                    point: convert_hwp_point(p),
                    point_type: match t {
                        HwpCurveSegmentType::Line => IrCurvePointType::Normal,
                        HwpCurveSegmentType::Curve => IrCurvePointType::Control1,
                    },
                })
                .collect();
            // 첫 번째 점과 마지막 점이 같으면 닫힌 곡선으로 판단
            let closed = if points.len() >= 2 {
                let first = &points[0].point;
                let last = &points[points.len() - 1].point;
                (first.x.value() - last.x.value()).abs() < 10 &&
                (first.y.value() - last.y.value()).abs() < 10
            } else {
                false
            };
            Ok(IrShapeType::Curve(IrCurveShape {
                points,
                closed,
            }))
        }
        HwpShapeType::Container(shapes) => {
            let ir_shapes = shapes.iter()
                .map(|s| convert_shape(s, ctx))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(IrShapeType::Group(ir_shapes))
        }
        HwpShapeType::Unknown => {
            // Unknown 타입은 빈 사각형으로 처리
            Ok(IrShapeType::Rectangle(IrRectangleShape::default()))
        }
    }
}

/// HWP 점 → IR 점 변환
fn convert_hwp_point(point: &HwpPoint) -> IrPoint {
    IrPoint {
        x: HwpUnit::new(point.x.value()),
        y: HwpUnit::new(point.y.value()),
    }
}

/// HWP 호 타입 → IR 호 타입 변환
fn convert_hwp_arc_type(hwp_type: HwpArcType) -> IrArcType {
    match hwp_type {
        HwpArcType::Arc => IrArcType::Arc,
        HwpArcType::Pie => IrArcType::Pie,
        HwpArcType::Chord => IrArcType::Chord,
    }
}

/// 중심점과 start/end 점에서 호의 시작/종료 각도 계산 (도 단위)
fn calculate_arc_angles(center: &HwpPoint, start: &HwpPoint, end: &HwpPoint) -> (f64, f64) {
    // 중심점 기준 상대 좌표
    let dx_start = start.x.value() as f64 - center.x.value() as f64;
    let dy_start = start.y.value() as f64 - center.y.value() as f64;
    let dx_end = end.x.value() as f64 - center.x.value() as f64;
    let dy_end = end.y.value() as f64 - center.y.value() as f64;

    // atan2로 각도 계산 (라디안 → 도)
    let start_angle = dy_start.atan2(dx_start).to_degrees();
    let end_angle = dy_end.atan2(dx_end).to_degrees();

    // 음수 각도를 0-360 범위로 정규화
    let start_angle = if start_angle < 0.0 { start_angle + 360.0 } else { start_angle };
    let end_angle = if end_angle < 0.0 { end_angle + 360.0 } else { end_angle };

    // 시작점과 끝점이 같으면 완전한 원/타원
    if (dx_start - dx_end).abs() < 1.0 && (dy_start - dy_end).abs() < 1.0 {
        return (0.0, 360.0);
    }

    (start_angle, end_angle)
}

/// HWP 테두리 선 → IR 선 스타일 변환
fn convert_border_line(border: &ShapeBorderLine) -> IrLineStyle {
    let color = Color::rgb(
        border.color.red(),
        border.color.green(),
        border.color.blue(),
    );

    // 선 종류: properties의 bit 0~5 (표 87 참조)
    let line_style_value = (border.properties & 0x3F) as u8;
    let line_type = convert_hwp_line_style(line_style_value);

    // 선 끝 모양: properties의 bit 6~9 (표 87 참조)
    let cap_value = ((border.properties >> 6) & 0x0F) as u8;
    let cap = match cap_value {
        0 => IrLineCap::Round,
        1 => IrLineCap::Flat,
        _ => IrLineCap::Flat,
    };

    IrLineStyle {
        line_type,
        width: HwpUnit::new(border.thickness),
        color,
        cap,
        outline_style: primitive::LineOutlineStyle::Normal,
        alpha: None,
    }
}

/// HWP 선 종류 값 → IR LineType 변환 (표 25 참조)
fn convert_hwp_line_style(value: u8) -> IrLineType {
    match value {
        0 => IrLineType::Solid,        // 실선
        1 => IrLineType::LongDash,     // 긴 점선
        2 => IrLineType::Dot,          // 점선
        3 => IrLineType::DashDot,      // -.-.-.-
        4 => IrLineType::DashDotDot,   // -..-..-..-
        5 => IrLineType::Dash,         // Dash보다 긴 선분의 반복
        6 => IrLineType::Circle,       // 큰 동그라미의 반복
        7 => IrLineType::Double,       // 2중선
        8 | 9 => IrLineType::Double,   // 가는선 + 굵은선 / 굵은선 + 가는선
        10 => IrLineType::Triple,      // 가는선 + 굵은선 + 가는선 3중선
        11 => IrLineType::Wave,        // 물결
        12 => IrLineType::DoubleWave,  // 물결 2중선
        13 | 14 => IrLineType::ThickThinLarge,  // 두꺼운 3D
        15 | 16 => IrLineType::ThinThickLarge,  // 3D 단선
        _ => IrLineType::Solid,        // 기본값
    }
}

/// HWP 화살표 → IR 화살표 변환 (ArrowType에서 filled 여부 추론)
fn convert_hwp_arrow(hwp_type: HwpArrowType, hwp_size: HwpArrowSize) -> IrArrow {
    let arrow_type = convert_hwp_arrow_type(hwp_type);
    let filled = matches!(
        hwp_type,
        HwpArrowType::Arrow
            | HwpArrowType::Spear
            | HwpArrowType::ConcaveArrow
            | HwpArrowType::FilledDiamond
            | HwpArrowType::FilledCircle
            | HwpArrowType::FilledBox
    );

    IrArrow {
        arrow_type,
        size: convert_hwp_arrow_size(hwp_size),
        filled,
    }
}

/// HWP 화살표 타입 → IR 화살표 타입 변환
fn convert_hwp_arrow_type(hwp_type: HwpArrowType) -> IrArrowType {
    match hwp_type {
        HwpArrowType::None => IrArrowType::None,
        HwpArrowType::Arrow => IrArrowType::Arrow,
        HwpArrowType::Spear => IrArrowType::Arrow,
        HwpArrowType::ConcaveArrow => IrArrowType::Stealth,
        HwpArrowType::EmptyDiamond => IrArrowType::Diamond,
        HwpArrowType::EmptyCircle => IrArrowType::Circle,
        HwpArrowType::EmptyBox => IrArrowType::Square,
        HwpArrowType::FilledDiamond => IrArrowType::Diamond,
        HwpArrowType::FilledCircle => IrArrowType::Circle,
        HwpArrowType::FilledBox => IrArrowType::Square,
    }
}

/// HWP 화살표 크기 → IR 화살표 크기 변환
fn convert_hwp_arrow_size(hwp_size: HwpArrowSize) -> IrArrowSize {
    match hwp_size {
        HwpArrowSize::Smallest => IrArrowSize::Small,
        HwpArrowSize::Small => IrArrowSize::Small,
        HwpArrowSize::Medium => IrArrowSize::Medium,
        HwpArrowSize::Large => IrArrowSize::Large,
        HwpArrowSize::Largest => IrArrowSize::Large,
    }
}

// =============================================================================
// 누락된 컨트롤 변환 함수들 (Phase A)
// =============================================================================

/// 책갈피 변환
fn convert_bookmark(control: &Control) -> Result<IrBookmark, ConversionError> {
    // 책갈피 이름은 ControlData에서 가져옴
    let name = if let Ok(ctrl_data) = ControlData::from_bytes(control.data()) {
        ctrl_data
            .get_string(0x4000) // NAME item
            .map(|s| s.to_string())
            .unwrap_or_default()
    } else {
        String::new()
    };

    Ok(IrBookmark { name })
}

/// 자동 번호 변환
fn convert_auto_number(control: &Control) -> Result<IrAutoNumber, ConversionError> {
    let data = control.data();

    // HWP 스펙 표 143: 자동번호
    // - UINT32: 속성 (bit 0~3: 번호 종류, bit 4~11: 번호 모양)
    let properties = if data.len() >= 4 {
        u32::from_le_bytes([data[0], data[1], data[2], data[3]])
    } else {
        0
    };

    // 번호 종류 추출 (bit 0~3)
    let number_kind = (properties & 0x0F) as u8;
    let number_type = convert_auto_number_type(number_kind);

    // 번호 모양 추출 (bit 4~11)
    let number_shape = ((properties >> 4) & 0xFF) as u8;
    let number_format = convert_number_format(number_shape);

    Ok(IrAutoNumber {
        number_type,
        number_format,
        auto_number_format: None,
    })
}

/// 새 번호 변환
fn convert_new_number(control: &Control) -> Result<IrNewNumber, ConversionError> {
    let data = control.data();

    // HWP 스펙 표 144: 새 번호 지정
    // - UINT32: 속성 (bit 0~3: 번호 종류)
    // - UINT16: 번호
    let properties = if data.len() >= 4 {
        u32::from_le_bytes([data[0], data[1], data[2], data[3]])
    } else {
        0
    };

    let number = if data.len() >= 6 {
        u16::from_le_bytes([data[4], data[5]]) as u32
    } else {
        1
    };

    // 번호 종류 추출 (bit 0~3)
    let number_kind = (properties & 0x0F) as u8;
    let number_type = convert_auto_number_type(number_kind);

    Ok(IrNewNumber { number_type, number })
}

/// 페이지 번호 변환 (PageNumber → AutoNumber)
fn convert_page_number_to_auto_number(control: &Control) -> Result<IrAutoNumber, ConversionError> {
    let data = control.data();

    // HWP 스펙 표 145: 쪽 번호 위치
    // - UINT32: 속성 (bit 0~3: 위치, bit 4~11: 번호 모양)
    // - WCHAR[]: 줄표 문자
    let properties = if data.len() >= 4 {
        u32::from_le_bytes([data[0], data[1], data[2], data[3]])
    } else {
        0
    };

    // 번호 모양 추출 (bit 4~11)
    let number_shape = ((properties >> 4) & 0xFF) as u8;
    let number_format = convert_number_format(number_shape);

    Ok(IrAutoNumber {
        number_type: IrAutoNumberType::Page,
        number_format,
        auto_number_format: None,
    })
}

/// 숨은 설명 변환
fn convert_hidden_comment(control: &Control) -> Result<IrHiddenComment, ConversionError> {
    // 숨은 설명은 문단 리스트를 포함 - children에서 가져옴
    // 숨은 설명 내부에서는 각주/미주가 없으므로 빈 컨텍스트 사용
    let empty_ctx = SectionContext {
        footnote_shape: None,
        endnote_shape: None,
    };
    let paragraphs = control
        .children()
        .iter()
        .filter_map(|p| convert_paragraph_with_context(p, &empty_ctx).ok())
        .collect();

    Ok(IrHiddenComment { paragraphs })
}

/// 색인 표시 변환
fn convert_index_mark(control: &Control) -> Result<ir::control::IndexMark, ConversionError> {
    use crate::body::control_data::ControlData;

    // 색인 표시 데이터는 ControlData에서 가져옴
    // HWP 스펙 표 146: 색인 표시 (idxm)
    // - 첫 번째 키 (item 0x4000)
    // - 두 번째 키 (item 0x4001)
    let (first_key, second_key) = if let Ok(ctrl_data) = ControlData::from_bytes(control.data()) {
        let first = ctrl_data
            .get_string(0x4000)
            .map(|s| s.to_string())
            .unwrap_or_default();

        let second = ctrl_data
            .get_string(0x4001)
            .map(|s| s.to_string())
            .unwrap_or_default();

        (first, second)
    } else {
        (String::new(), String::new())
    };

    Ok(ir::control::IndexMark {
        first_key,
        second_key,
    })
}

/// 텍스트 박스 변환
fn convert_text_box(
    text_box: &crate::body::TextBox,
    control: &Control,
) -> Result<ir::control::TextBox, ConversionError> {
    use crate::body::VerticalAlignment as HwpVertAlign;
    use ir::control::TextBox as IrTextBox;
    use primitive::VerticalAlignment as IrVertAlign;

    // 문단 변환 - children에서 가져옴
    // 텍스트 박스 내부에서는 각주/미주가 없으므로 빈 컨텍스트 사용
    let empty_ctx = SectionContext {
        footnote_shape: None,
        endnote_shape: None,
    };
    let paragraphs = control
        .children()
        .iter()
        .filter_map(|p| convert_paragraph_with_context(p, &empty_ctx).ok())
        .collect();

    // 세로 정렬 변환
    let vertical_alignment = match text_box.vertical_align() {
        HwpVertAlign::Top => IrVertAlign::Top,
        HwpVertAlign::Center => IrVertAlign::Middle,
        HwpVertAlign::Bottom => IrVertAlign::Bottom,
    };

    Ok(IrTextBox {
        common: ir::control::ObjectCommon::default(),
        paragraphs,
        text_direction: primitive::TextDirection::Horizontal,
        vertical_alignment,
        padding: primitive::Insets::default(),
        editable: true,
        name: None,
        last_width: None,
        line_wrap: primitive::LineWrap::Break,
        link_list_id_reference: None,
        link_list_next_id_reference: None,
        text_width: None,
        text_height: None,
        has_text_reference: false,
        has_number_reference: false,
    })
}

/// 자동 번호 종류 변환
fn convert_auto_number_type(kind: u8) -> IrAutoNumberType {
    match kind {
        0 => IrAutoNumberType::Page,
        1 => IrAutoNumberType::Footnote,
        2 => IrAutoNumberType::Endnote,
        3 => IrAutoNumberType::Picture,
        4 => IrAutoNumberType::Table,
        5 => IrAutoNumberType::Equation,
        _ => IrAutoNumberType::Page,
    }
}

/// 번호 모양 변환 (표 134 참조)
fn convert_number_format(shape: u8) -> NumberFormat {
    match shape {
        0 => NumberFormat::Digit,         // 1, 2, 3
        1 => NumberFormat::CircledDigit,  // ①, ②, ③
        2 => NumberFormat::RomanUpper,    // I, II, III
        3 => NumberFormat::RomanLower,    // i, ii, iii
        4 => NumberFormat::LatinUpper,    // A, B, C
        5 => NumberFormat::LatinLower,    // a, b, c
        6 => NumberFormat::CircledHangul, // ㉮, ㉯, ㉰
        7 => NumberFormat::Ideograph,     // 一, 二, 三
        _ => NumberFormat::Digit,
    }
}

/// 비디오 변환
fn convert_video(video: &HwpVideoData, control: &Control) -> Result<IrVideo, ConversionError> {
    use crate::VideoType;

    // 공통 속성 파싱 (Control 데이터에서 추출)
    let common = parse_object_common(control.data()).unwrap_or_default();

    // 비디오 종류 변환
    let video_type = match video.video_type {
        VideoType::Embedded => IrVideoType::Embedded,
        VideoType::Linked => IrVideoType::Linked,
        VideoType::YouTube => IrVideoType::YouTube,
        VideoType::Unknown => IrVideoType::Embedded, // 기본값
    };

    // 비디오 파일 ID (임베디드인 경우)
    let video_id = video.bin_data_id.map(|id| BinaryDataId::from_numeric(id as u16));

    // 소스 URL (링크인 경우)
    let source_url = if !video.source.is_empty() {
        Some(video.source.clone())
    } else {
        None
    };

    // 미리보기 이미지 ID
    let preview_image_id = video.poster_bin_id.map(|id| BinaryDataId::from_numeric(id as u16));

    // 포스터 바이너리 ID
    let poster_binary_id = video.poster_bin_id.map(|id| BinaryDataId::from_numeric(id as u16));

    // 비디오 너비와 높이 (HWP 단위)
    let width = if video.width > 0 {
        Some(HwpUnit::new(video.width as i32))
    } else {
        None
    };
    let height = if video.height > 0 {
        Some(HwpUnit::new(video.height as i32))
    } else {
        None
    };

    Ok(IrVideo {
        common,
        video_type,
        video_id,
        source_url,
        preview_image_id,
        poster_binary_id,
        width,
        height,
    })
}

/// OLE 객체 변환
fn convert_ole(ole: &HwpOleObject, control: &Control) -> Result<IrOleObject, ConversionError> {
    // 공통 속성 파싱 (Control 데이터에서 추출)
    let common = parse_object_common(control.data()).unwrap_or_default();

    Ok(IrOleObject {
        common,
        binary_id: BinaryDataId::from_numeric(ole.binary_data_id),
        class_id: None, // HWP에서는 class_id 정보가 별도로 없음
        preview_image_id: None, // 미리보기 이미지는 별도 처리 필요
    })
}

/// 차트 변환
fn convert_chart(chart: &HwpChartData, control: &Control) -> Result<IrChart, ConversionError> {
    // 공통 속성 파싱 (Control 데이터에서 추출)
    let common = parse_object_common(control.data()).unwrap_or_default();

    // 차트 타입 변환
    let chart_type = convert_chart_type(&chart.chart_type);

    Ok(IrChart {
        common,
        chart_id: String::new(), // HWP에서는 차트 ID가 별도로 없음
        chart_type,
    })
}

/// HWP 차트 타입 → IR 차트 타입 변환
fn convert_chart_type(hwp_type: &crate::body::ChartType) -> IrChartType {
    match hwp_type {
        crate::body::ChartType::Bar => IrChartType::Bar,
        crate::body::ChartType::Line => IrChartType::Line,
        crate::body::ChartType::Pie => IrChartType::Pie,
        crate::body::ChartType::Area => IrChartType::Area,
        crate::body::ChartType::Scatter => IrChartType::Scatter,
        crate::body::ChartType::Radar => IrChartType::Radar,
        _ => IrChartType::Bar, // Unknown, Combined 등은 Bar로 기본값
    }
}

/// 양식 객체 변환
fn convert_form_object(form: &HwpFormObject, control: &Control) -> Result<IrFormObject, ConversionError> {
    use ir::control::{FormCharProperty, FormListItem};

    // 공통 속성 파싱 (Control 데이터에서 추출)
    let common = parse_object_common(control.data()).unwrap_or_default();

    // 양식 타입 변환
    let form_type = convert_form_type(form.object_type());

    let ext = form.extended_properties();

    // Convert back_color from HWP BGR format to IR Color
    let back_color = ext.back_color.map(|bgr| Color {
        alpha: 255,
        red: (bgr & 0xFF) as u8,
        green: ((bgr >> 8) & 0xFF) as u8,
        blue: ((bgr >> 16) & 0xFF) as u8,
    });

    // Convert back_style
    let back_style = ext.back_style.map(|val| match val {
        0 => ir::control::ButtonBackStyle::Transparent,
        1 => ir::control::ButtonBackStyle::Opaque,
        _ => ir::control::ButtonBackStyle::Transparent,
    });

    // Convert scroll_bars
    let scroll_bars = ext.scroll_bars.map(|val| match val {
        0 => ir::control::EditScrollBars::None,
        1 => ir::control::EditScrollBars::Vertical,
        2 => ir::control::EditScrollBars::Horizontal,
        3 => ir::control::EditScrollBars::Both,
        _ => ir::control::EditScrollBars::None,
    });

    // Convert tab_key_behavior
    let tab_key_behavior = ext.tab_key_behavior.map(|val| match val {
        0 => ir::control::EditTabKeyBehavior::NextObject,
        1 => ir::control::EditTabKeyBehavior::InsertTab,
        _ => ir::control::EditTabKeyBehavior::NextObject,
    });

    // Convert alignment
    let alignment = ext.alignment.map(|val| match val {
        0 => ir::control::EditTextAlignment::Left,
        1 => ir::control::EditTextAlignment::Center,
        2 => ir::control::EditTextAlignment::Right,
        _ => ir::control::EditTextAlignment::Left,
    });

    // Convert password_char
    let password_char = ext.password_char.and_then(|ch| {
        char::from_u32(ch as u32).map(|c| c.to_string())
    });

    // Build items list from items_text and items_value
    let items: Vec<FormListItem> = ext.items_text.iter()
        .zip(ext.items_value.iter())
        .map(|(text, value)| FormListItem {
            display_text: if text.is_empty() { None } else { Some(text.clone()) },
            value: if value.is_empty() { None } else { Some(value.clone()) },
        })
        .collect();

    Ok(IrFormObject {
        common,
        form_type,
        name: if form.name().is_empty() { None } else { Some(form.name().to_string()) },
        value: if form.default_value().is_empty() { None } else { Some(form.default_value().to_string()) },
        char_property: FormCharProperty::default(),
        items,
        // HWPX 전용 필드 - HWP에서는 추출 어려움
        fore_color: None,
        back_color,
        group_name: None,
        tab_stop: true,
        enabled: true,
        editable: true,
        border_type_id_ref: None,
        draw_frame: true,
        printable: true,
        tab_order: None,
        // Button 속성
        caption: ext.caption.clone(),
        button_value: None, // TODO: Parse from properties or control data
        radio_group_name: ext.radio_group_name.clone(),
        back_style,
        tri_state: ext.tri_state,
        gradient_fill: ext.gradient_fill,
        image_fill: ext.image_fill,
        // Edit 속성
        multiline: ext.multiline,
        password_char,
        max_length: ext.max_length,
        scroll_bars,
        tab_key_behavior,
        num_only: ext.num_only,
        read_only: ext.read_only,
        alignment,
        // ComboBox/ListBox 속성
        edit_enable: ext.edit_enable,
        selected_value: ext.selected_value.clone(),
        list_box_rows: ext.list_box_rows,
        list_box_width: ext.list_box_width,
        item_height: ext.item_height,
        top_index: ext.top_index,
        // ScrollBar 속성
        bar_type: None, // TODO: Parse from properties
        min: ext.min,
        max: ext.max,
        scroll_value: ext.value,
        small_change: ext.small_change,
        large_change: ext.large_change,
        page: ext.page,
        delay: ext.delay,
    })
}
/// HWP 양식 타입 → IR 양식 타입 변환
fn convert_form_type(hwp_type: HwpFormObjectType) -> IrFormObjectType {
    match hwp_type {
        HwpFormObjectType::TextField => IrFormObjectType::Edit,
        HwpFormObjectType::CheckBox => IrFormObjectType::CheckBox,
        HwpFormObjectType::RadioButton => IrFormObjectType::RadioButton,
        HwpFormObjectType::ComboBox => IrFormObjectType::ComboBox,
        HwpFormObjectType::ListBox => IrFormObjectType::ListBox,
        HwpFormObjectType::Button => IrFormObjectType::Button,
        HwpFormObjectType::Unknown => IrFormObjectType::Button, // 기본값
    }
}

/// 글맵시 변환
fn convert_text_art(text_art: &HwpTextArt, control: &Control) -> Result<IrTextArt, ConversionError> {
    // 공통 속성 파싱 (Control 데이터에서 추출)
    let common = parse_object_common(control.data()).unwrap_or_default();

    // 글꼴 스타일 변환
    let font_style = {
        let is_bold = text_art.is_bold();
        let is_italic = text_art.is_italic();
        match (is_bold, is_italic) {
            (false, false) => IrTextArtFontStyle::Regular,
            (true, false) => IrTextArtFontStyle::Bold,
            (false, true) => IrTextArtFontStyle::Italic,
            (true, true) => IrTextArtFontStyle::BoldItalic,
        }
    };

    // 모양 변환
    let shape = match text_art.shape() {
        HwpTextArtShape::Plain => IrTextArtShapeType::Rectangle,
        HwpTextArtShape::Wave => IrTextArtShapeType::Wave,
        HwpTextArtShape::ArcUp => IrTextArtShapeType::ArchUp,
        HwpTextArtShape::ArcDown => IrTextArtShapeType::ArchDown,
        HwpTextArtShape::Circle => IrTextArtShapeType::Circle,
        HwpTextArtShape::Button => IrTextArtShapeType::Rectangle,
        HwpTextArtShape::Inflate => IrTextArtShapeType::Inflate,
        HwpTextArtShape::Deflate => IrTextArtShapeType::Deflate,
        HwpTextArtShape::FadeRight | HwpTextArtShape::FadeLeft => IrTextArtShapeType::Rectangle,
        HwpTextArtShape::SlantUp | HwpTextArtShape::SlantDown => IrTextArtShapeType::Rectangle,
        HwpTextArtShape::Unknown => IrTextArtShapeType::Rectangle,
    };

    // 정렬 변환
    let alignment = match text_art.alignment() {
        HwpTextArtAlignment::Left => IrTextArtAlignment::Left,
        HwpTextArtAlignment::Center => IrTextArtAlignment::Center,
        HwpTextArtAlignment::Right => IrTextArtAlignment::Right,
        HwpTextArtAlignment::Justify => IrTextArtAlignment::Full,
    };

    // 글꼴 이름
    let font_name = if text_art.font_name().is_empty() {
        None
    } else {
        Some(text_art.font_name().to_string())
    };

    // HWP 색상은 BGR 형식 (0xBBGGRR)
    let bgr_to_color = |bgr: u32| -> Color {
        Color {
            alpha: 255,
            red: (bgr & 0xFF) as u8,
            green: ((bgr >> 8) & 0xFF) as u8,
            blue: ((bgr >> 16) & 0xFF) as u8,
        }
    };

    // 외곽선 색상 → LineStyle
    let outline_color = text_art.outline_color();
    let line = IrLineStyle {
        line_type: IrLineType::Solid,
        width: HwpUnit::new(10), // 기본 두께 0.1pt
        color: bgr_to_color(outline_color),
        cap: IrLineCap::Flat,
        outline_style: primitive::LineOutlineStyle::Normal,
        alpha: None,
    };

    // 텍스트 색상 → Fill (Solid)
    let text_color = text_art.text_color();
    let fill = ir::border_fill::Fill::Solid(ir::border_fill::SolidFill {
        color: bgr_to_color(text_color),
    });

    // 그림자 색상 → ShapeShadow (색상이 0이 아니면 그림자 있음)
    let shadow_color = text_art.shadow_color();
    let shadow = if shadow_color != 0 {
        Some(IrShapeShadow {
            color: bgr_to_color(shadow_color),
            offset_x: HwpUnit::new(100), // 기본 오프셋 1pt
            offset_y: HwpUnit::new(100),
            alpha: 0.5, // 기본 투명도
            blur: None,
            direction: None,
            distance: None,
        })
    } else {
        None
    };

    Ok(IrTextArt {
        common,
        text: text_art.text().to_string(),
        font_name,
        font_style,
        font_type: None, // HWP에는 font_type 정보가 없음 (HWPX 전용)
        shape,
        line_spacing: 120, // 기본값
        char_spacing: 100, // 기본값
        alignment,
        line,
        fill,
        shadow,
        text_art_pr: None, // HWP에는 text_art_pr 정보가 없음 (HWPX 전용)
    })
}

/// HWP 필드 → IR 하이퍼링크 변환
fn convert_field_to_hyperlink(field: &HwpField) -> Result<IrHyperlink, ConversionError> {
    // 필드 instruction에서 URL 추출
    let target = field.instruction().to_string();

    // URL 타입 결정
    let hyperlink_target = if target.starts_with("mailto:") {
        HyperlinkTarget::Email(target.trim_start_matches("mailto:").to_string())
    } else if target.starts_with("http://") || target.starts_with("https://") {
        HyperlinkTarget::Url(target)
    } else if target.starts_with('#') {
        HyperlinkTarget::Bookmark(target.trim_start_matches('#').to_string())
    } else if target.is_empty() {
        // 값(value)을 대신 사용
        let value = field.value().to_string();
        if value.starts_with("mailto:") {
            HyperlinkTarget::Email(value.trim_start_matches("mailto:").to_string())
        } else if value.starts_with("http://") || value.starts_with("https://") {
            HyperlinkTarget::Url(value)
        } else {
            HyperlinkTarget::File(value)
        }
    } else {
        HyperlinkTarget::File(target)
    };

    Ok(IrHyperlink {
        target: hyperlink_target,
        tooltip: None,
        display_text: None, // HWP에서 display_text 추출은 별도 구현 필요
    })
}

/// HWP 필드 타입 → IR 필드 타입 변환
fn convert_field_type(hwp_type: HwpFieldType) -> IrFieldType {
    match hwp_type {
        HwpFieldType::Date => IrFieldType::Date,
        HwpFieldType::Time => IrFieldType::Time,
        HwpFieldType::FilePath => IrFieldType::FilePath,
        HwpFieldType::DocTitle => IrFieldType::Title,
        HwpFieldType::Author => IrFieldType::Author,
        HwpFieldType::PageNumber => IrFieldType::PageNumber,
        HwpFieldType::TotalPages => IrFieldType::PageCount,
        HwpFieldType::Summary => IrFieldType::Summary,
        HwpFieldType::CrossReference => IrFieldType::CrossReference,
        HwpFieldType::Hyperlink => IrFieldType::Hyperlink,
        HwpFieldType::ClickHere => IrFieldType::ClickHere,
        HwpFieldType::UserInfo => IrFieldType::UserInfo,
        HwpFieldType::Formula => IrFieldType::Formula,
        HwpFieldType::Memo => IrFieldType::Memo,
        HwpFieldType::PrivateInfo => IrFieldType::PrivateInfo,
        HwpFieldType::MetaTag => IrFieldType::MetaTag,
        HwpFieldType::MailMerge => IrFieldType::MailMerge,
        HwpFieldType::TableOfContents => IrFieldType::TableOfContents,
        HwpFieldType::Unknown => IrFieldType::Unknown,
    }
}

/// HWP 필드 → IR FieldStart 변환 (Hyperlink 제외)
fn convert_field_to_field_start(field: &HwpField, field_id: u32, control_data: &[u8]) -> IrFieldStart {
    // ControlData에서 ParameterSet 추출
    let parameters = if !control_data.is_empty() {
        match ControlData::from_bytes(control_data) {
            Ok(ctrl_data) => convert_hwp_parameters_to_ir(&ctrl_data),
            Err(_) => None,
        }
    } else {
        None
    };

    IrFieldStart {
        id: field_id,
        field_type: convert_field_type(field.field_type()),
        instruction: if field.instruction().is_empty() {
            None
        } else {
            Some(field.instruction().to_string())
        },
        parameters,
        sub_paragraphs: None,
        editable: true,
        dirty: false,
        z_order: None,
        field_id: None,
    }
}

/// HWP ParameterSet → IR FieldParameters 변환
fn convert_hwp_parameters_to_ir(ctrl_data: &ControlData) -> Option<ir::paragraph::FieldParameters> {
    // 첫 번째 ParameterSet 가져오기
    let param_set = ctrl_data.first_set()?;

    let mut items = Vec::new();

    // 모든 ParameterItem을 IR FieldParameter로 변환
    for item in param_set.items() {
        if let Some(ir_param) = convert_hwp_parameter_item_to_ir(item) {
            items.push(ir_param);
        }
    }

    if items.is_empty() {
        return None;
    }

    Some(ir::paragraph::FieldParameters {
        items,
        name: None, // HWP ParameterSet에는 name이 없음
    })
}

/// HWP ParameterItem → IR FieldParameter 변환
fn convert_hwp_parameter_item_to_ir(
    item: &crate::body::control_data::ParameterItem,
) -> Option<ir::paragraph::FieldParameter> {
    use crate::body::control_data::ParameterValue;

    // item.id를 name으로 사용 (16진수 문자열)
    let name = Some(format!("0x{:04X}", item.id));

    match &item.value {
        ParameterValue::Boolean(b) => Some(ir::paragraph::FieldParameter::Boolean {
            name,
            value: *b,
        }),
        ParameterValue::Integer(i) => Some(ir::paragraph::FieldParameter::Integer {
            name,
            value: *i,
        }),
        ParameterValue::UnsignedInteger(u) => Some(ir::paragraph::FieldParameter::Integer {
            name,
            value: *u as i64,
        }),
        ParameterValue::String(s) => Some(ir::paragraph::FieldParameter::String {
            name,
            value: s.clone(),
        }),
        ParameterValue::Null | ParameterValue::Binary(_) => None,
    }
}

/// HWP 그룹 도형(Container) → IR Shape(Group) 변환
fn convert_container(container: &HwpShapeContainer, ctx: &SectionContext) -> Result<IrShape, ConversionError> {
    let mut children = Vec::new();

    // 컨테이너의 모든 자식 Control을 재귀적으로 변환
    for child_control in container.children() {
        // 자식 Control의 ObjectCommon 파싱
        let child_common = parse_object_common(child_control.data());

        // Control에서 Shape 컨텐츠 추출
        match child_control.content() {
            Some(ControlContent::Shape(shape)) => {
                let mut ir_child = convert_shape(shape, ctx)?;
                // 자식의 ObjectCommon 적용
                if let Some(common) = child_common {
                    ir_child.common = common;
                }
                children.push(ir_child);
            }
            Some(ControlContent::Container(nested_container)) => {
                // 중첩된 컨테이너도 재귀 처리
                let mut ir_child = convert_container(nested_container, ctx)?;
                // 중첩 컨테이너의 ObjectCommon 적용
                if let Some(common) = child_common {
                    ir_child.common = common;
                }
                children.push(ir_child);
            }
            Some(ControlContent::Picture(_)) => {
                // 그룹 내 그림은 IR Shape가 아니므로 현재 변환 불가
                // TODO: IR 타입 확장 후 지원 필요 (Group이 Shape 외에 Picture도 포함하도록)
            }
            Some(ControlContent::TextBox(text_box)) => {
                // 그룹 내 텍스트 박스를 사각형 + 텍스트로 변환
                if let Ok(mut ir_text_box) = convert_text_box(text_box, child_control) {
                    // 자식의 ObjectCommon 적용
                    if let Some(common) = child_common {
                        ir_text_box.common = common.clone();
                    }
                    // TextBox를 Shape로 래핑 (사각형 도형 + 텍스트)
                    let shape = IrShape {
                        common: ir_text_box.common.clone(),
                        shape_type: IrShapeType::Rectangle(IrRectangleShape::default()),
                        line: IrLineStyle::default(),
                        fill: ir::border_fill::Fill::None,
                        shadow: None,
                        rotation: 0.0,
                        text: Some(ir::shape::ShapeText {
                            paragraphs: ir_text_box.paragraphs,
                            padding: ir_text_box.padding,
                            vertical_alignment: ir_text_box.vertical_alignment,
                            text_direction: ir_text_box.text_direction,
                            editable: ir_text_box.editable,
                        }),
                        translation_matrix: None,
                        scale_matrix: None,
                        rotation_matrix: None,
                    };
                    children.push(shape);
                }
            }
            _ => {
                // 그 외 컨텐츠는 그룹 도형에서 무시
            }
        }
    }

    // 그룹 자체의 ObjectCommon은 호출 측에서 설정됨 (convert_control_with_context)
    Ok(IrShape {
        translation_matrix: None,
        scale_matrix: None,
        rotation_matrix: None,
        common: ObjectCommon::default(),
        shape_type: IrShapeType::Group(children),
        line: IrLineStyle::default(),
        fill: ir::border_fill::Fill::None,
        shadow: None,
        rotation: 0.0,
        text: None,
    })
}

// =============================================================================
// ObjectCommon 파싱 (HWP 개체 공통 속성)
// =============================================================================

/// HWP Control 데이터에서 개체 공통 속성(ObjectCommon) 파싱
///
/// HWP 스펙 표 69에 따른 구조:
/// - UINT32: ctrl ID (4 bytes)
/// - UINT32: 속성 (4 bytes)
/// - HWPUNIT: 세로 오프셋 (4 bytes)
/// - HWPUNIT: 가로 오프셋 (4 bytes)
/// - HWPUNIT: width (4 bytes)
/// - HWPUNIT: height (4 bytes)
/// - INT32: z-order (4 bytes)
/// - HWPUNIT16 array[4]: 바깥 여백 (8 bytes)
/// - UINT32: instance ID (4 bytes)
/// - INT32: 쪽나눔 방지 (4 bytes)
/// - WORD: 개체 설명문 길이 (2 bytes)
/// - WCHAR array: 개체 설명문
fn parse_object_common(data: &[u8]) -> Option<ObjectCommon> {
    // 최소 크기 확인 (46 bytes 기본)
    if data.len() < 46 {
        return None;
    }

    // ctrl ID skip (4 bytes)
    let properties = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    let offset_y = i32::from_le_bytes([data[8], data[9], data[10], data[11]]);
    let offset_x = i32::from_le_bytes([data[12], data[13], data[14], data[15]]);
    let width = i32::from_le_bytes([data[16], data[17], data[18], data[19]]);
    let height = i32::from_le_bytes([data[20], data[21], data[22], data[23]]);
    let z_order = i32::from_le_bytes([data[24], data[25], data[26], data[27]]);
    // 바깥 여백 4개 (HWPUNIT16 x 4 = 8 bytes: 28-35)
    // 순서: left, right, top, bottom
    let margin_left = i16::from_le_bytes([data[28], data[29]]);
    let margin_right = i16::from_le_bytes([data[30], data[31]]);
    let margin_top = i16::from_le_bytes([data[32], data[33]]);
    let margin_bottom = i16::from_le_bytes([data[34], data[35]]);
    // 평균을 사용하거나 가장 큰 값을 사용 (IR은 단일 margin만 지원)
    let avg_margin = (margin_left as i32 + margin_right as i32 + margin_top as i32 + margin_bottom as i32) / 4;
    let instance_id = u32::from_le_bytes([data[36], data[37], data[38], data[39]]);
    // 쪽나눔 방지 skip (4 bytes: 40-43)
    // 개체 설명문 길이/내용 skip

    // 속성 비트 파싱 (표 70 참조)
    let text_wrap = parse_text_wrap_from_properties(properties, avg_margin);

    Some(ObjectCommon {
        id: if instance_id > 0 { Some(instance_id) } else { None },
        position: IrPoint {
            x: HwpUnit::new(offset_x),
            y: HwpUnit::new(offset_y),
        },
        size: Size {
            width: HwpUnit::new(width),
            height: HwpUnit::new(height),
        },
        z_order,
        text_wrap,
        caption: None, // Caption은 별도 처리 (Phase 2)
        numbering_type: None,
        shape_comment: None,
        meta_tag: None,
        dirty: false,
        width_relative_to: primitive::WidthRelativeTo::default(),
        height_relative_to: primitive::HeightRelativeTo::default(),
        margin: ir::control::ObjectMargin {
            left: HwpUnit::new(margin_left as i32),
            right: HwpUnit::new(margin_right as i32),
            top: HwpUnit::new(margin_top as i32),
            bottom: HwpUnit::new(margin_bottom as i32),
        },
    })
}

/// 속성 비트에서 TextWrap 정보 파싱 (HWP 스펙 표 70)
fn parse_text_wrap_from_properties(properties: u32, margin: i32) -> IrTextWrap {
    // bit 0: 글자처럼 취급 여부
    let treat_as_char = (properties & 0x01) != 0;

    // bit 3~4: 세로 위치의 기준 (VertRelTo)
    let vert_rel_to = (properties >> 3) & 0x03;
    let vertical_rel = match vert_rel_to {
        0 => IrVerticalRelativeTo::Paper,
        1 => IrVerticalRelativeTo::Page,
        2 => IrVerticalRelativeTo::Paragraph,
        _ => IrVerticalRelativeTo::Paper,
    };

    // bit 8~9: 가로 위치의 기준 (HorzRelTo)
    let horz_rel_to = (properties >> 8) & 0x03;
    let horizontal_rel = match horz_rel_to {
        0 => IrHorizontalRelativeTo::Paper,
        1 => IrHorizontalRelativeTo::Page,
        2 => IrHorizontalRelativeTo::Column,
        3 => IrHorizontalRelativeTo::Paragraph,
        _ => IrHorizontalRelativeTo::Paper,
    };

    // bit 14: 다른 오브젝트와 겹치는 것을 허용할지 여부
    let allow_overlap = (properties >> 14) & 0x01 != 0;

    // bit 21~23: 텍스트 감싸기 종류
    let wrap_type_bits = (properties >> 21) & 0x07;
    let wrap_type = match wrap_type_bits {
        0 => IrTextWrapType::Square,      // bound rect를 따라
        1 => IrTextWrapType::Tight,       // 오브젝트의 outline을 따라
        2 => IrTextWrapType::Tight,       // through (빈 공간까지)
        3 => IrTextWrapType::Square,      // TopAndBottom
        4 => IrTextWrapType::Behind,      // BehindText
        5 => IrTextWrapType::InFront,     // InFrontOfText
        _ => IrTextWrapType::Square,
    };

    // bit 24~25: 좌/우 어느 쪽에 글 배치
    let wrap_side_bits = (properties >> 24) & 0x03;
    let wrap_side = match wrap_side_bits {
        0 => IrTextWrapSide::Both,
        1 => IrTextWrapSide::Left,
        2 => IrTextWrapSide::Right,
        3 => IrTextWrapSide::Largest,
        _ => IrTextWrapSide::Both,
    };

    // 글자처럼 취급인 경우 Inline으로 처리
    let final_wrap_type = if treat_as_char {
        IrTextWrapType::Inline
    } else {
        wrap_type
    };

    IrTextWrap {
        wrap_type: final_wrap_type,
        wrap_side,
        margin: HwpUnit::new(margin),
        vertical_rel,
        horizontal_rel,
        vertical_offset_type: primitive::VerticalOffsetType::default(),
        horizontal_offset_type: primitive::HorizontalOffsetType::default(),
        treat_as_char,
        flow_with_text: false, // HWP 5.0 스펙에서 명시적으로 정의되지 않음
        allow_overlap,
    }
}
