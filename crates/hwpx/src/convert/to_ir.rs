//! HWPX → IR 변환
//!
//! HWPX 문서를 IR로 변환합니다.
//! 스타일 정보(폰트, 글자 모양, 문단 모양, 스타일)를 모두 IR로 변환합니다.

use crate::Document as HwpxDocument;
use crate::header::{
    bullet::Bullet as HwpxBullet,
    character_shape::{
        CharacterShape as HwpxCharShape, EmphasisMarkType, ShadowType as HwpxShadowType,
        UnderlinePosition as HwpxUnderlinePosition,
    },
    font::Font as HwpxFont,
    numbering::Numbering as HwpxNumbering,
    paragraph_shape::{
        HorizontalAlignment, LatinWordBreak, LineSpacingType as HwpxLineSpacingType,
        NonLatinWordBreak, ParagraphShape as HwpxParaShape,
        VerticalAlignment as HwpxVerticalAlignment,
    },
    style::Style as HwpxStyle,
};
use ir::{
    BinaryData, BinaryDataId, BinaryDataStore, BinaryFormat, ConversionError, ConversionResult,
    Document as IrDocument, Extensions, HwpxExtensions, Metadata, Paragraph as IrParagraph,
    Run as IrRun, RunContent as IrRunContent, Section as IrSection,
    char_shape::{
        CharShape, EmphasisStyle, Font, FontFamily, FontRef, FontSet, ShadowStyle, UnderlineStyle,
    },
    control::{
        Caption as IrCaption, CaptionPosition as IrCaptionPosition, Chart as IrChart,
        ChartType as IrChartType, Control as IrControl, Equation as IrEquation,
        FormObject as IrFormObject, FormObjectType as IrFormObjectType, HeaderFooterControl,
        Hyperlink as IrHyperlink, HyperlinkTarget, Note as IrNote, ObjectCommon,
        ObjectNumberingType as IrObjectNumberingType, OleObject as IrOleObject,
        TextWrap as IrTextWrap, Video as IrVideo, VideoType as IrVideoType,
    },
    para_shape::{LineSpacing, LineSpacingType, LineSpacingValue, ParaShape, TabDef},
    paragraph::{
        FieldEnd as IrFieldEnd, FieldParameter as IrFieldParameter,
        FieldParameters as IrFieldParameters, FieldStart as IrFieldStart,
        LineSegment as IrLineSegment, RangeTag as IrRangeTag, RangeTagType as IrRangeTagType,
        TabChar, Text as IrText,
    },
    picture::{ImageCrop as IrImageCrop, Picture as IrPicture, PictureBorder},
    shape::{
        ArcShape as IrArcShape, ArcType as IrArcType, CurvePoint as IrCurvePoint,
        CurvePointType as IrCurvePointType, CurveShape as IrCurveShape,
        EllipseShape as IrEllipseShape, LineShape as IrLineShape, LineStyle as IrLineStyle,
        PolygonShape as IrPolygonShape, RectangleShape as IrRectangleShape, Shape as IrShape,
        ShapeText as IrShapeText, ShapeType as IrShapeType,
    },
    style::{
        Bullet as IrBullet, Numbering as IrNumbering, NumberingLevel as IrNumberingLevel, Style,
        StyleStore,
    },
    table::{
        Table as IrTable, TableCell as IrTableCell, TableRow as IrTableRow,
        TableZone as IrTableZone,
    },
};
use primitive::{
    Alignment, BorderFillId, CharShapeId, Color, EmphasisType, FieldType as IrFieldType, FontId,
    HeaderFooterApplyTo, HorizontalRelativeTo as IrHorizontalRelativeTo, HwpUnit, ImageFlip,
    Insets, LineBreakKorean, LineBreakLatin, LineCap as IrLineCap, LineType as IrLineType,
    NumberFormat, OutlineType, ParaShapeId, Percent, Point as IrPoint, ShadowType, Size,
    StrikethroughType, StyleId, StyleType, TextWrapSide as IrTextWrapSide,
    TextWrapType as IrTextWrapType, UnderlinePosition, VerticalAlignment,
    VerticalRelativeTo as IrVerticalRelativeTo,
};

use super::ToIrContext;

/// HWPX → IR 변환 트레이트
pub trait HwpxToIr {
    /// IR 문서로 변환
    fn to_ir(&self) -> Result<ConversionResult<IrDocument>, ConversionError>;
}

impl HwpxToIr for HwpxDocument {
    fn to_ir(&self) -> Result<ConversionResult<IrDocument>, ConversionError> {
        let mut ctx = ToIrContext::new();
        let doc = convert_document(self, &mut ctx)?;
        Ok(ctx.warnings.into_result(doc))
    }
}

/// 문서 변환
fn convert_document(
    hwpx: &HwpxDocument,
    ctx: &mut ToIrContext,
) -> Result<IrDocument, ConversionError> {
    let mut doc = IrDocument::new();

    // 메타데이터 변환
    doc.metadata = convert_metadata(hwpx);

    // 스타일 변환 (완전한 변환)
    doc.styles = convert_styles(hwpx, ctx)?;

    // 섹션 변환
    for section in &hwpx.sections {
        let ir_section = convert_section(section)?;
        doc.sections.push(ir_section);
    }

    // 바이너리 데이터 변환
    doc.binary_data = convert_binary_data(hwpx)?;

    // 확장 데이터 설정
    doc.extensions = convert_extensions(hwpx, ctx);

    Ok(doc)
}

/// 메타데이터 변환
fn convert_metadata(hwpx: &HwpxDocument) -> Metadata {
    let mut metadata = Metadata::new();

    // 버전 정보
    let version = &hwpx.version;
    metadata.version = Some(ir::DocumentVersion::new(
        version.major,
        version.minor,
        version.micro as u32,
        version.build_number,
    ));

    metadata
}

/// 스타일 저장소 변환
fn convert_styles(
    hwpx: &HwpxDocument,
    _ctx: &mut ToIrContext,
) -> Result<StyleStore, ConversionError> {
    let mut store = StyleStore::new();

    let mapping_table = &hwpx.header.mapping_table;

    // 폰트 변환
    if let Some(ref fontfaces) = mapping_table.fontfaces {
        for fontface in &fontfaces.fontfaces {
            for font in &fontface.fonts {
                let ir_font = convert_font(font);
                store.fonts.push(ir_font);
            }
        }
    }

    // 글자 모양 변환
    if let Some(ref char_shapes) = mapping_table.character_shapes {
        for shape in &char_shapes.character_shapes {
            let ir_shape = convert_char_shape(shape);
            store.char_shapes.push(ir_shape);
        }
    }

    // 문단 모양 변환
    if let Some(ref para_shapes) = mapping_table.paragraph_shapes {
        for shape in &para_shapes.paragraph_shapes {
            let ir_shape = convert_para_shape(shape);
            store.para_shapes.push(ir_shape);
        }
    }

    // 테두리/채우기 변환
    if let Some(ref border_fills) = mapping_table.border_fills {
        for bf in &border_fills.border_fills {
            let ir_bf = convert_border_fill(bf);
            store.border_fills.push(ir_bf);
        }
    }

    // 스타일 변환
    if let Some(ref styles) = mapping_table.styles {
        for style in &styles.styles {
            let ir_style = convert_style(style);
            store.styles.push(ir_style);
        }
    }

    // 탭 정의 변환
    if let Some(ref tab_defs) = mapping_table.tab_definitions {
        for tab_def in &tab_defs.tab_definitions {
            store.tab_defs.push(convert_tab_def(tab_def));
        }
    }

    // 번호 매기기 변환
    if let Some(ref numberings) = mapping_table.numberings {
        for numbering in &numberings.numberings {
            store.numberings.push(convert_numbering(numbering));
        }
    }

    // 글머리 기호 변환
    if let Some(ref bullets) = mapping_table.bullets {
        for bullet in &bullets.bullets {
            store.bullets.push(convert_bullet(bullet));
        }
    }

    Ok(store)
}

/// 폰트 변환
fn convert_font(font: &HwpxFont) -> Font {
    use crate::header::font::FontType as HwpxFontType;
    use ir::char_shape::SubstituteFont as IrSubstituteFont;

    let mut ir_font = Font::new(&font.face);

    // 폰트 타입 변환
    ir_font.font_type = match font.font_type {
        HwpxFontType::Representative => ir::char_shape::FontType::Representative,
        HwpxFontType::TrueType => ir::char_shape::FontType::TrueType,
        HwpxFontType::HangeulFont => ir::char_shape::FontType::HangeulOnly,
    };

    // 임베디드 폰트 정보
    ir_font.is_embedded = font.is_embedded;
    if let Some(ref bin_ref) = font.binary_item_id_reference {
        ir_font.binary_item_id_ref = Some(primitive::BinaryDataId::new(bin_ref.0.clone()));
    }

    // 대체 폰트가 있으면 상세 정보 저장
    if let Some(ref substitute) = font.substitute_font {
        ir_font.alternate_name = Some(substitute.face.clone());

        // HWPX 대체 폰트 상세 정보
        let subst_font_type = match substitute.font_type {
            HwpxFontType::Representative => ir::char_shape::FontType::Representative,
            HwpxFontType::TrueType => ir::char_shape::FontType::TrueType,
            HwpxFontType::HangeulFont => ir::char_shape::FontType::HangeulOnly,
        };

        ir_font.substitute_font = Some(IrSubstituteFont {
            face: substitute.face.clone(),
            font_type: subst_font_type,
            is_embedded: substitute.is_embedded,
            binary_item_id_ref: substitute
                .binary_item_id_reference
                .as_ref()
                .map(|r| r.0.clone()),
        });
    }

    // 폰트 패밀리 정보가 있으면 변환
    if let Some(ref type_info) = font.type_info {
        use crate::header::font::FontFamilyType;
        ir_font.family = match type_info.family_type {
            FontFamilyType::Myungjo | FontFamilyType::NonRectMyungjo => FontFamily::Serif,
            FontFamilyType::Gothic | FontFamilyType::NonRectGothic | FontFamilyType::SansSerif => {
                FontFamily::SansSerif
            }
            FontFamilyType::Decorative => FontFamily::Decorative,
            FontFamilyType::BrushScript => FontFamily::Script,
            FontFamilyType::Unknown => FontFamily::Unknown,
        };

        // PANOSE 정보 저장 (enum을 u8로 변환)
        ir_font.panose = Some(primitive::Panose::from_bytes([
            type_info.family_type as u8,
            type_info.serif_style.map(|s| s as u8).unwrap_or(0),
            type_info.weight as u8,
            type_info.proportion as u8,
            type_info.contrast as u8,
            type_info.stroke_variation as u8,
            type_info.arm_style as u8,
            type_info.letterform as u8,
            type_info.midline as u8,
            type_info.x_height as u8,
        ]));
    }

    ir_font
}

/// 글자 모양 변환
fn convert_char_shape(shape: &HwpxCharShape) -> CharShape {
    let mut ir_shape = CharShape::new();

    // 글꼴 크기
    ir_shape.font_size = HwpUnit::new(shape.height);

    // 굵게/기울임
    ir_shape.bold = shape.bold.is_some();
    ir_shape.italic = shape.italic.is_some();

    // 위/아래 첨자
    ir_shape.superscript = shape.superscript.is_some();
    ir_shape.subscript = shape.subscript.is_some();

    // 양각/음각
    ir_shape.emboss = shape.emboss.is_some();
    ir_shape.engrave = shape.engrave.is_some();

    // 텍스트 색상
    ir_shape.color = Color::rgb(shape.text_color.r, shape.text_color.g, shape.text_color.b);

    // 밑줄
    if let Some(ref underline) = shape.underline {
        ir_shape.underline = UnderlineStyle {
            line_type: convert_hwpx_underline_type(&underline.shape),
            position: convert_hwpx_underline_position(&underline.position),
            color: Some(Color::rgb(
                underline.color.r,
                underline.color.g,
                underline.color.b,
            )),
        };
    }

    // 취소선
    if let Some(ref strikeout) = shape.strikeout {
        ir_shape.strikethrough = convert_hwpx_strikethrough_type(&strikeout.shape);
    }

    // 외곽선
    if let Some(ref outline) = shape.outline {
        ir_shape.outline = convert_hwpx_outline_type(&outline.outline_type);
    }

    // 그림자
    // HWPX shadow offset도 i8 퍼센트 단위 (-100% ~ 100%)
    if let Some(ref shadow) = shape.shadow {
        ir_shape.shadow = ShadowStyle {
            shadow_type: convert_hwpx_shadow_type(&shadow.shadow_type),
            color: Some(Color::rgb(shadow.color.r, shadow.color.g, shadow.color.b)),
            // 퍼센트 값 그대로 저장
            offset_x: HwpUnit::new(shadow.offset_x as i32),
            offset_y: HwpUnit::new(shadow.offset_y as i32),
        };
    }

    // 강조점
    ir_shape.emphasis = EmphasisStyle {
        emphasis_type: convert_hwpx_emphasis_type(&shape.emphasis_mark),
        color: None,
    };

    // 장평 (한글 기준)
    ir_shape.char_scale = Percent::new(shape.ratio.hangul as f64);

    // 자간 (한글 기준)
    ir_shape.char_spacing = Percent::new(shape.spacing.hangul as f64);

    // 커닝
    ir_shape.use_kerning = shape.use_kerning;

    // 폰트 설정 (각 언어별)
    let font_ref = &shape.font_reference;
    let ratio = &shape.ratio;
    let spacing = &shape.spacing;
    let offset = &shape.offset;
    let relative = &shape.relative_size;

    ir_shape.fonts = FontSet {
        korean: Some(FontRef {
            id: FontId::new(font_ref.hangul),
            width_ratio: Percent::new(ratio.hangul as f64),
            spacing: Percent::new(spacing.hangul as f64),
            offset: Percent::new(offset.hangul as f64),
            relative_size: Percent::new(relative.hangul as f64),
        }),
        english: Some(FontRef {
            id: FontId::new(font_ref.latin),
            width_ratio: Percent::new(ratio.latin as f64),
            spacing: Percent::new(spacing.latin as f64),
            offset: Percent::new(offset.latin as f64),
            relative_size: Percent::new(relative.latin as f64),
        }),
        hanja: Some(FontRef {
            id: FontId::new(font_ref.hanja),
            width_ratio: Percent::new(ratio.hanja as f64),
            spacing: Percent::new(spacing.hanja as f64),
            offset: Percent::new(offset.hanja as f64),
            relative_size: Percent::new(relative.hanja as f64),
        }),
        japanese: Some(FontRef {
            id: FontId::new(font_ref.japanese),
            width_ratio: Percent::new(ratio.japanese as f64),
            spacing: Percent::new(spacing.japanese as f64),
            offset: Percent::new(offset.japanese as f64),
            relative_size: Percent::new(relative.japanese as f64),
        }),
        other: Some(FontRef {
            id: FontId::new(font_ref.other),
            width_ratio: Percent::new(ratio.other as f64),
            spacing: Percent::new(spacing.other as f64),
            offset: Percent::new(offset.other as f64),
            relative_size: Percent::new(relative.other as f64),
        }),
        symbol: Some(FontRef {
            id: FontId::new(font_ref.symbol),
            width_ratio: Percent::new(ratio.symbol as f64),
            spacing: Percent::new(spacing.symbol as f64),
            offset: Percent::new(offset.symbol as f64),
            relative_size: Percent::new(relative.symbol as f64),
        }),
        user: Some(FontRef {
            id: FontId::new(font_ref.user),
            width_ratio: Percent::new(ratio.user as f64),
            spacing: Percent::new(spacing.user as f64),
            offset: Percent::new(offset.user as f64),
            relative_size: Percent::new(relative.user as f64),
        }),
    };

    // 글자 배경색 (shade_color가 흰색이 아닌 경우에만 설정)
    if shape.shade_color.r != 255 || shape.shade_color.g != 255 || shape.shade_color.b != 255 {
        ir_shape.background_color = Some(Color::rgb(
            shape.shade_color.r,
            shape.shade_color.g,
            shape.shade_color.b,
        ));
    }

    // 음영 색상 (shade_color를 그대로 저장)
    ir_shape.shade_color = Some(Color::rgb(
        shape.shade_color.r,
        shape.shade_color.g,
        shape.shade_color.b,
    ));

    // 테두리/배경 참조 ID
    if let Some(border_fill_ref) = shape.border_fill_id_reference {
        ir_shape.border_fill_id_ref = Some(primitive::BorderFillId::new(border_fill_ref.0));
    }

    ir_shape
}

/// 문단 모양 변환
fn convert_para_shape(shape: &HwpxParaShape) -> ParaShape {
    let mut ir_shape = ParaShape::new();

    // 정렬
    ir_shape.alignment = convert_hwpx_alignment(&shape.alignment.horizontal);

    // 세로 정렬
    ir_shape.vertical_alignment = convert_hwpx_vertical_alignment(&shape.alignment.vertical);

    // 여백
    if let Some(ref margin) = shape.margin {
        ir_shape.margin_left = HwpUnit::new(margin.left.value);
        ir_shape.margin_right = HwpUnit::new(margin.right.value);
        ir_shape.first_line_indent = HwpUnit::new(margin.indent.value);
        ir_shape.space_before = HwpUnit::new(margin.previous.value);
        ir_shape.space_after = HwpUnit::new(margin.next.value);
    }

    // 줄 간격
    if let Some(ref line_spacing) = shape.line_spacing {
        ir_shape.line_spacing = LineSpacing {
            spacing_type: convert_hwpx_line_spacing_type(&line_spacing.spacing_type),
            value: match line_spacing.spacing_type {
                HwpxLineSpacingType::Percent => {
                    LineSpacingValue::Percent(Percent::new(line_spacing.value as f64))
                }
                _ => LineSpacingValue::Fixed(HwpUnit::new(line_spacing.value)),
            },
        };
    }

    // 줄 나눔 규칙
    ir_shape.line_break_korean =
        convert_hwpx_break_korean(&shape.break_setting.break_non_latin_word);
    ir_shape.line_break_latin = convert_hwpx_break_latin(&shape.break_setting.break_latin_word);

    // 문단 보호 옵션
    ir_shape.widow_orphan_control = shape.break_setting.widow_orphan;
    ir_shape.keep_with_next = shape.break_setting.keep_with_next;
    ir_shape.keep_lines = shape.break_setting.keep_lines;
    ir_shape.page_break_before = shape.break_setting.page_break_before;

    // 테두리/채우기 참조
    let bf_id = shape.border.border_fill_id_reference.0;
    if bf_id > 0 {
        ir_shape.border_fill_id = Some(BorderFillId::new(bf_id));
    }

    // 탭 정의 참조
    if let Some(ref tab_ref) = shape.tab_definition_id_reference {
        ir_shape.tab_def_id = Some(primitive::TabDefId::new(tab_ref.0));
    }

    // 추가 속성
    ir_shape.snap_to_grid = shape.snap_to_grid;
    ir_shape.suppress_line_numbers = shape.suppress_line_numbers;

    // 글꼴에 어울리는 줄 높이
    if shape.font_line_height {
        ir_shape.auto_line_height_ratio = Percent::new(100.0);
    }

    // 자동 간격
    ir_shape.auto_spacing_east_asian_english = shape.auto_spacing.east_asian_english;
    ir_shape.auto_spacing_east_asian_number = shape.auto_spacing.east_asian_number;

    // 문단 머리 (번호/글머리표) 설정
    use crate::header::paragraph_shape::HeadingType as HwpxHeadingType;
    use primitive::HeadingType as IrHeadingType;
    let heading_type_ir = match shape.heading.heading_type {
        HwpxHeadingType::None => IrHeadingType::None,
        HwpxHeadingType::Outline => IrHeadingType::Outline,
        HwpxHeadingType::Number => IrHeadingType::Number,
        HwpxHeadingType::Bullet => IrHeadingType::Bullet,
    };

    if heading_type_ir != IrHeadingType::None {
        let numbering_shape_id = shape.heading.id_reference.0;
        ir_shape.numbering = Some(ir::para_shape::ParagraphNumbering {
            heading_type: heading_type_ir,
            numbering_id: if heading_type_ir == IrHeadingType::Number
                || heading_type_ir == IrHeadingType::Outline
            {
                if numbering_shape_id > 0 {
                    Some(numbering_shape_id)
                } else {
                    None
                }
            } else {
                None
            },
            bullet_id: if heading_type_ir == IrHeadingType::Bullet {
                if numbering_shape_id > 0 {
                    Some(numbering_shape_id)
                } else {
                    None
                }
            } else {
                None
            },
            level: shape.heading.level as u8,
        });
    }

    // 문단 테두리 오프셋 정보 추가
    if shape.border.border_fill_id_reference.0 > 0 {
        ir_shape.border = Some(ir::para_shape::ParagraphBorder {
            border_fill_id_ref: BorderFillId::new(shape.border.border_fill_id_reference.0),
            offset_left: HwpUnit::new(shape.border.offset_left),
            offset_right: HwpUnit::new(shape.border.offset_right),
            offset_top: HwpUnit::new(shape.border.offset_top),
            offset_bottom: HwpUnit::new(shape.border.offset_bottom),
            connect: shape.border.connect,
            ignore_margin: shape.border.ignore_margin,
        });
    }

    ir_shape
}

/// 탭 정의 변환
fn convert_tab_def(tab_def: &crate::header::tab_definition::TabDefinition) -> TabDef {
    use crate::core::enums::LineStyleType2;
    use crate::header::tab_definition::TabType as HwpxTabType;
    use ir::para_shape::Tab;
    use primitive::{TabLeader, TabType as IrTabType};

    let mut ir_tab_def = TabDef::default();

    // 탭 항목 변환 (HWPX는 단일 tab_item만 가짐)
    if let Some(ref tab_item) = tab_def.tab_item {
        let ir_tab = Tab {
            position: primitive::HwpUnit::new(tab_item.position),
            tab_type: match tab_item.tab_type {
                HwpxTabType::Left => IrTabType::Left,
                HwpxTabType::Right => IrTabType::Right,
                HwpxTabType::Center => IrTabType::Center,
                HwpxTabType::Decimal => IrTabType::Decimal,
            },
            leader: match tab_item.leader {
                LineStyleType2::None => TabLeader::None,
                LineStyleType2::Solid => TabLeader::Underscore,
                LineStyleType2::Dot => TabLeader::Dot,
                LineStyleType2::Dash => TabLeader::Dash,
                LineStyleType2::DashDot => TabLeader::Dash,
                LineStyleType2::DashDotDot => TabLeader::LongDash,
                LineStyleType2::LongDash => TabLeader::LongDash,
                LineStyleType2::Circle => TabLeader::Dot,
                LineStyleType2::DoubleSlim => TabLeader::Underscore,
                LineStyleType2::SlimThick => TabLeader::Underscore,
                LineStyleType2::ThickSlim => TabLeader::Underscore,
                LineStyleType2::SlimThickSlim => TabLeader::Underscore,
            },
        };
        ir_tab_def.tabs.push(ir_tab);
    }

    // 자동 탭 설정
    if tab_def.auto_tab_left || tab_def.auto_tab_right {
        ir_tab_def.auto_tab_interval = Some(primitive::HwpUnit::new(800)); // 8mm 기본값
    } else {
        ir_tab_def.auto_tab_interval = None;
    }

    ir_tab_def
}

/// 번호 매기기 변환 (HWPX → IR)
fn convert_numbering(numbering: &HwpxNumbering) -> IrNumbering {
    let mut levels = Vec::new();

    for para_head in &numbering.paragraph_heads {
        // HWPX ParagraphHeadAlignment을 IR Alignment으로 변환
        let alignment = match para_head.alignment {
            crate::header::paragraph_head::ParagraphHeadAlignment::Left => {
                primitive::Alignment::Left
            }
            crate::header::paragraph_head::ParagraphHeadAlignment::Center => {
                primitive::Alignment::Center
            }
            crate::header::paragraph_head::ParagraphHeadAlignment::Right => {
                primitive::Alignment::Right
            }
        };

        // HWPX NumberFormatType1을 IR NumberFormat으로 변환
        let number_format = convert_number_format_type1_to_ir(para_head.number_format);

        levels.push(IrNumberingLevel {
            level: para_head.level as u8,
            format: para_head.text.clone(),
            char_shape_id: para_head
                .character_shape_id_reference
                .as_ref()
                .map(|id| CharShapeId::new(id.0)),
            text_offset: para_head.text_offset,
            number_width: para_head.width_adjust,
            start_number: para_head.start,
            alignment,
            use_instance_width: para_head.use_instance_width,
            auto_indent: para_head.auto_indent,
            number_format,
        });
    }

    IrNumbering {
        name: None,
        levels,
        start_number: numbering.start as u32,
    }
}

/// 글머리 기호 변환 (HWPX → IR)
fn convert_bullet(bullet: &HwpxBullet) -> IrBullet {
    IrBullet {
        char: bullet.character.chars().next().unwrap_or('•'),
        char_shape_id: bullet
            .paragraph_head
            .character_shape_id_reference
            .as_ref()
            .map(|id| CharShapeId::new(id.0)),
        is_checkbox: bullet.paragraph_head.checkable.unwrap_or(false),
    }
}

/// HWPX NumberFormatType1을 IR NumberFormat으로 변환
const fn convert_number_format_type1_to_ir(
    format: crate::core::enums::NumberFormatType1,
) -> primitive::NumberFormat {
    use crate::core::enums::NumberFormatType1;
    use primitive::NumberFormat;

    match format {
        NumberFormatType1::Digit => NumberFormat::Digit,
        NumberFormatType1::CircledDigit => NumberFormat::CircledDigit,
        NumberFormatType1::RomanCapital => NumberFormat::RomanUpper,
        NumberFormatType1::RomanSmall => NumberFormat::RomanLower,
        NumberFormatType1::LatinCapital => NumberFormat::LatinUpper,
        NumberFormatType1::LatinSmall => NumberFormat::LatinLower,
        NumberFormatType1::CircledLatinCapital => NumberFormat::CircledLatinUpper,
        NumberFormatType1::CircledLatinSmall => NumberFormat::CircledLatinLower,
        NumberFormatType1::HangulSyllable => NumberFormat::HangulSyllable,
        NumberFormatType1::CircledHangulSyllable => NumberFormat::CircledHangul,
        NumberFormatType1::HangulJamo => NumberFormat::HangulJamo,
        NumberFormatType1::CircledHangulJamo => NumberFormat::CircledHangulJamo,
        NumberFormatType1::HangulPhonetic => NumberFormat::HangulIdeograph,
        NumberFormatType1::Ideograph => NumberFormat::Ideograph,
        NumberFormatType1::CircledIdeograph => NumberFormat::CircledIdeograph,
    }
}

/// FillBrush → IR Fill 변환
fn convert_fill_brush(fill_brush: Option<&crate::core::types::FillBrush>) -> ir::border_fill::Fill {
    use crate::core::enums::{GradationType, HatchStyle, ImageBrushMode};
    use crate::core::types::RgbColor;
    use ir::border_fill::{
        Fill, GradientFill, GradientStop, ImageFill, PatternFill, PatternType as IrPatternType,
        SolidFill,
    };
    use primitive::BinaryDataId;
    use primitive::Color;
    use primitive::{GradientType as IrGradientType, ImageFillMode};

    // 색상 변환 헬퍼
    let convert_color = |color: &RgbColor| -> Color {
        Color {
            red: color.r,
            green: color.g,
            blue: color.b,
            alpha: color.a,
        }
    };

    let Some(fill_brush) = fill_brush else {
        return Fill::None;
    };

    // 먼저 이미지 브러시 확인
    if let Some(ref img_brush) = fill_brush.image_brush {
        let mode = match img_brush.mode {
            ImageBrushMode::Tile => ImageFillMode::Tile,
            ImageBrushMode::TileHorizontalTop => ImageFillMode::TileHorizontalTop,
            ImageBrushMode::TileHorizontalBottom => ImageFillMode::TileHorizontalBottom,
            ImageBrushMode::TileVerticalLeft => ImageFillMode::TileVerticalLeft,
            ImageBrushMode::TileVerticalRight => ImageFillMode::TileVerticalRight,
            ImageBrushMode::Total => ImageFillMode::Stretch,
            ImageBrushMode::Center => ImageFillMode::Center,
            ImageBrushMode::CenterTop => ImageFillMode::CenterTop,
            ImageBrushMode::CenterBottom => ImageFillMode::CenterBottom,
            ImageBrushMode::LeftCenter => ImageFillMode::CenterLeft,
            ImageBrushMode::LeftTop => ImageFillMode::TopLeft,
            ImageBrushMode::LeftBottom => ImageFillMode::BottomLeft,
            ImageBrushMode::RightCenter => ImageFillMode::CenterRight,
            ImageBrushMode::RightTop => ImageFillMode::TopRight,
            ImageBrushMode::RightBottom => ImageFillMode::BottomRight,
            ImageBrushMode::Zoom => ImageFillMode::Stretch,
        };

        let effect = match img_brush.image.effect {
            crate::core::enums::ImageEffect::RealPicture => primitive::ImageEffect::Original,
            crate::core::enums::ImageEffect::GrayScale => primitive::ImageEffect::Grayscale,
            crate::core::enums::ImageEffect::BlackWhite => primitive::ImageEffect::BlackWhite,
        };

        return Fill::Image(ImageFill {
            binary_id: BinaryDataId::new(img_brush.image.binary_item_id_reference.0.clone()),
            mode,
            // brightness/contrast는 -100~100 범위, 클램프 적용
            brightness: img_brush.image.brightness.clamp(-100, 100) as i8,
            contrast: img_brush.image.contrast.clamp(-100, 100) as i8,
            effect,
            offset_x: primitive::HwpUnit::ZERO,
            offset_y: primitive::HwpUnit::ZERO,
            size: None,
        });
    }

    // 그라데이션 확인
    if let Some(ref grad) = fill_brush.gradation {
        let gradient_type = match grad.gradation_type {
            Some(GradationType::Linear) => IrGradientType::Linear,
            Some(GradationType::Radial) => IrGradientType::Radial,
            Some(GradationType::Conical) => IrGradientType::Conical,
            Some(GradationType::Square) => IrGradientType::Square,
            None => IrGradientType::Linear,
        };

        let stops: Vec<GradientStop> = grad
            .colors
            .iter()
            .enumerate()
            .map(|(i, color)| {
                let position = if grad.colors.len() == 1 {
                    0
                } else {
                    (i * 100 / (grad.colors.len() - 1)) as u8
                };
                GradientStop {
                    position,
                    color: convert_color(&color.value),
                }
            })
            .collect();

        return Fill::Gradient(GradientFill {
            gradient_type,
            // HWPX angle은 i32, IR angle은 u16 (0-360)
            // 음수 각도를 양수로 정규화
            angle: if grad.angle < 0 {
                ((grad.angle % 360) + 360) as u16
            } else {
                (grad.angle % 360) as u16
            },
            // center_x/y는 퍼센트 (0-100), 범위 클램프
            center_x: grad.center_x.clamp(0, 100) as u8,
            center_y: grad.center_y.clamp(0, 100) as u8,
            stops,
            blur: grad.step.value(),
            step_center: grad.step_center.value(),
        });
    }

    // 윈도우 브러시 (단색/패턴)
    if let Some(ref win_brush) = fill_brush.windows_brush {
        if let Some(ref hatch) = win_brush.hatch_style {
            // 패턴 채우기
            let pattern_type = match hatch {
                HatchStyle::Horizontal => IrPatternType::Horizontal,
                HatchStyle::Vertical => IrPatternType::Vertical,
                HatchStyle::BackSlash => IrPatternType::BackSlash,
                HatchStyle::Slash => IrPatternType::Slash,
                HatchStyle::Cross => IrPatternType::Cross,
                HatchStyle::CrossDiagonal => IrPatternType::CrossDiagonal,
            };
            let foreground = match win_brush.hatch_color.0 {
                Some(ref c) => convert_color(c),
                None => Color::BLACK,
            };
            let background = match win_brush.face_color.0 {
                Some(ref c) => convert_color(c),
                None => Color::WHITE,
            };
            return Fill::Pattern(PatternFill {
                pattern_type,
                foreground,
                background,
            });
        } else {
            // 단색 채우기
            let color = match win_brush.face_color.0 {
                Some(ref c) => convert_color(c),
                None => Color::WHITE,
            };
            return Fill::Solid(SolidFill { color, alpha: 255 });
        }
    }

    Fill::None
}

/// 테두리/채우기 변환
fn convert_border_fill(bf: &crate::header::border_fill::BorderFill) -> ir::border_fill::BorderFill {
    use crate::core::enums::LineStyleType2;
    use crate::core::types::RgbColor;
    use ir::border_fill::{Border, BorderFill as IrBorderFill};
    use primitive::Color;
    use primitive::HwpUnit;
    use primitive::LineType;

    // 색상 변환 헬퍼
    let convert_color = |color: &RgbColor| -> Color {
        Color {
            red: color.r,
            green: color.g,
            blue: color.b,
            alpha: color.a,
        }
    };

    // 선 스타일 변환 헬퍼
    let convert_line_type = |style: LineStyleType2| -> LineType {
        match style {
            LineStyleType2::None => LineType::None,
            LineStyleType2::Solid => LineType::Solid,
            LineStyleType2::Dash => LineType::Dash,
            LineStyleType2::Dot => LineType::Dot,
            LineStyleType2::DashDot => LineType::DashDot,
            LineStyleType2::DashDotDot => LineType::DashDotDot,
            LineStyleType2::LongDash => LineType::LongDash,
            LineStyleType2::Circle => LineType::Circle,
            LineStyleType2::DoubleSlim => LineType::Double,
            LineStyleType2::SlimThick
            | LineStyleType2::ThickSlim
            | LineStyleType2::SlimThickSlim => LineType::Double,
        }
    };

    // 테두리 변환 헬퍼
    let convert_border_item = |border: &Option<crate::header::border_fill::Border>| -> Border {
        match border {
            Some(b) => {
                // LineWidth를 HwpUnit으로 변환 (0.1mm 단위)
                let width_mm = match b.width {
                    crate::core::enums::LineWidth::Mm0_1 => 0.1,
                    crate::core::enums::LineWidth::Mm0_12 => 0.12,
                    crate::core::enums::LineWidth::Mm0_15 => 0.15,
                    crate::core::enums::LineWidth::Mm0_2 => 0.2,
                    crate::core::enums::LineWidth::Mm0_25 => 0.25,
                    crate::core::enums::LineWidth::Mm0_3 => 0.3,
                    crate::core::enums::LineWidth::Mm0_4 => 0.4,
                    crate::core::enums::LineWidth::Mm0_5 => 0.5,
                    crate::core::enums::LineWidth::Mm0_6 => 0.6,
                    crate::core::enums::LineWidth::Mm0_7 => 0.7,
                    crate::core::enums::LineWidth::Mm1_0 => 1.0,
                    crate::core::enums::LineWidth::Mm1_5 => 1.5,
                    crate::core::enums::LineWidth::Mm2_0 => 2.0,
                    crate::core::enums::LineWidth::Mm3_0 => 3.0,
                    crate::core::enums::LineWidth::Mm4_0 => 4.0,
                    crate::core::enums::LineWidth::Mm5_0 => 5.0,
                };
                Border {
                    line_type: convert_line_type(b.line_type),
                    width: HwpUnit::from_mm(width_mm),
                    color: convert_color(&b.color),
                }
            }
            None => Border::none(),
        }
    };

    // 4개 테두리 변환
    let left = convert_border_item(&bf.left_border);
    let right = convert_border_item(&bf.right_border);
    let top = convert_border_item(&bf.top_border);
    let bottom = convert_border_item(&bf.bottom_border);

    // 대각선 변환 (단순 diagonal만 지원, slash/backSlash는 미지원)
    let diagonal_down = bf.diagonal.as_ref().map(|d| {
        let width_mm = match d.width {
            crate::core::enums::LineWidth::Mm0_1 => 0.1,
            crate::core::enums::LineWidth::Mm0_12 => 0.12,
            crate::core::enums::LineWidth::Mm0_15 => 0.15,
            crate::core::enums::LineWidth::Mm0_2 => 0.2,
            crate::core::enums::LineWidth::Mm0_25 => 0.25,
            crate::core::enums::LineWidth::Mm0_3 => 0.3,
            crate::core::enums::LineWidth::Mm0_4 => 0.4,
            crate::core::enums::LineWidth::Mm0_5 => 0.5,
            crate::core::enums::LineWidth::Mm0_6 => 0.6,
            crate::core::enums::LineWidth::Mm0_7 => 0.7,
            crate::core::enums::LineWidth::Mm1_0 => 1.0,
            crate::core::enums::LineWidth::Mm1_5 => 1.5,
            crate::core::enums::LineWidth::Mm2_0 => 2.0,
            crate::core::enums::LineWidth::Mm3_0 => 3.0,
            crate::core::enums::LineWidth::Mm4_0 => 4.0,
            crate::core::enums::LineWidth::Mm5_0 => 5.0,
        };
        Border {
            line_type: convert_line_type(d.line_type),
            width: HwpUnit::from_mm(width_mm),
            color: convert_color(&d.color),
        }
    });

    // 채우기 변환 (별도 함수 사용)
    let fill = convert_fill_brush(bf.fill_brush.as_ref());

    IrBorderFill {
        left,
        right,
        top,
        bottom,
        diagonal_down,
        diagonal_up: None, // HWPX에서 분리된 대각선 없음
        fill,
        is_3d: bf.three_dimensional,
        has_shadow: bf.shadow,
    }
}

/// 스타일 변환
fn convert_style(style: &HwpxStyle) -> Style {
    use crate::header::style::StyleKind;
    let style_type = match style.style_type {
        StyleKind::Paragraph => StyleType::Paragraph,
        StyleKind::Character => StyleType::Character,
    };

    Style {
        name: style.name.clone(),
        english_name: style.english_name.clone(),
        style_type,
        para_shape_id: style
            .paragraph_shape_id_reference
            .as_ref()
            .map(|id| ParaShapeId::new(id.0)),
        char_shape_id: style
            .character_shape_id_reference
            .as_ref()
            .map(|id| CharShapeId::new(id.0)),
        next_style_id: style
            .next_style_id_reference
            .as_ref()
            .map(|id| StyleId::new(id.0)),
    }
}

/// 섹션 변환
fn convert_section(section: &crate::paragraph::Section) -> Result<IrSection, ConversionError> {
    let mut ir_section = IrSection::default();

    // SectionDefinition 찾기 (보통 첫 문단의 첫 런에 있음)
    if let Some(section_def) = find_section_definition(section) {
        convert_section_definition(&section_def, &mut ir_section);
    }

    // ColumnDefinition 찾기
    if let Some(col_def) = find_column_definition(section) {
        ir_section.columns = convert_hwpx_column_definition(&col_def);
    }

    // 문단 변환
    for para in &section.paragraphs {
        let ir_para = convert_paragraph(para)?;
        ir_section.paragraphs.push(ir_para);
    }

    Ok(ir_section)
}

/// 섹션에서 SectionDefinition 찾기
fn find_section_definition(
    section: &crate::paragraph::Section,
) -> Option<crate::paragraph::SectionDefinition> {
    for para in &section.paragraphs {
        for run in &para.runs {
            for content in &run.contents {
                if let crate::paragraph::RunContent::SectionDefinition(sec_def) = content {
                    return Some(sec_def.as_ref().clone());
                }
            }
        }
    }
    None
}

/// SectionDefinition → IR Section 변환
fn convert_section_definition(
    sec_def: &crate::paragraph::SectionDefinition,
    ir_section: &mut IrSection,
) {
    // 페이지 설정 변환
    if let Some(ref page_pr) = sec_def.page_property {
        ir_section.page = convert_page_property(page_pr);
    }

    // 각주 모양 변환
    if let Some(ref footnote_shape) = sec_def.footnote_shape {
        ir_section.footnote_shape = Some(convert_hwpx_footnote_shape(footnote_shape));
    }

    // 미주 모양 변환
    if let Some(ref endnote_shape) = sec_def.endnote_shape {
        ir_section.endnote_shape = Some(convert_hwpx_endnote_shape(endnote_shape));
    }

    // 페이지 테두리/배경 변환
    if !sec_def.page_border_fills.is_empty() {
        // 첫 번째 것만 사용 (IR은 하나만 지원)
        ir_section.page_border_fill =
            Some(convert_hwpx_page_border_fill(&sec_def.page_border_fills[0]));
    }

    // 시작 번호 정보
    if let Some(ref start_num) = sec_def.start_number {
        use crate::paragraph::enums::PageStartsOn as HwpxPageStartsOn;
        use primitive::PageStartsOn as IrPageStartsOn;

        let page_starts_on = match start_num.page_starts_on {
            HwpxPageStartsOn::Both => IrPageStartsOn::Both,
            HwpxPageStartsOn::Even => IrPageStartsOn::Even,
            HwpxPageStartsOn::Odd => IrPageStartsOn::Odd,
        };

        ir_section.start_number = ir::section::SectionStartNumber {
            page_starts_on,
            page: start_num.page,
            picture: start_num.picture,
            table: start_num.table,
            equation: start_num.equation,
        };
    }

    // 단 간격 (SectionDefinition에서)
    if sec_def.space_columns != 0 {
        ir_section.columns.gap = HwpUnit(sec_def.space_columns);
    }

    // 그리드 설정 변환
    if let Some(ref grid) = sec_def.grid {
        ir_section.extensions.grid = ir::section::SectionGrid {
            line_grid: grid.line_grid,
            character_grid: grid.character_grid,
            manuscript_format: grid.manuscript_format,
        };
    }

    // 가시성 설정 변환
    if let Some(ref visibility) = sec_def.visibility {
        use crate::paragraph::enums::VisibilityValue;

        // HWPX 가시성 enum을 IR VisibilityOption으로 변환
        let convert_visibility_value =
            |val: &Option<VisibilityValue>| -> Option<ir::section::VisibilityOption> {
                match val {
                    Some(VisibilityValue::HideFirst) => {
                        Some(ir::section::VisibilityOption::HideFirstPage)
                    }
                    Some(VisibilityValue::ShowFirst) => {
                        Some(ir::section::VisibilityOption::ShowFirstPage)
                    }
                    Some(VisibilityValue::ShowAll) => Some(ir::section::VisibilityOption::Show),
                    None => None,
                }
            };

        ir_section.extensions.visibility = ir::section::SectionVisibility {
            hide_first_header: visibility.hide_first_header,
            hide_first_footer: visibility.hide_first_footer,
            hide_first_master_page: visibility.hide_first_master_page,
            hide_first_page_number: visibility.hide_first_page_number,
            hide_first_empty_line: visibility.hide_first_empty_line,
            show_line_number: visibility.show_line_number,
            border_visibility: convert_visibility_value(&visibility.border),
            fill_visibility: convert_visibility_value(&visibility.fill),
            ..Default::default()
        };
    }

    // 줄 번호 모양 변환
    if let Some(ref line_number_shape) = sec_def.line_number_shape {
        use primitive::LineNumberRestartType;

        let restart_type = match line_number_shape.restart_type {
            Some(0) => LineNumberRestartType::Continuous,
            Some(1) => LineNumberRestartType::RestartSection,
            Some(2) => LineNumberRestartType::RestartPage,
            _ => LineNumberRestartType::Continuous,
        };

        ir_section.extensions.line_number_shape = Some(ir::section::LineNumberShape {
            restart_type,
            count_by: line_number_shape.count_by.unwrap_or(1),
            distance: HwpUnit(line_number_shape.distance.unwrap_or(0) as i32),
            start_number: line_number_shape.start_number.unwrap_or(1),
        });
    }
}

/// HWPX ColumnDefinition → IR ColumnDefinition 변환
fn convert_hwpx_column_definition(
    col_def: &crate::paragraph::ColumnDefinition,
) -> ir::section::ColumnDefinition {
    use crate::paragraph::enums::ColumnLayout;
    use ir::section::{ColumnDefinition as IrColumnDef, ColumnDirection, ColumnSeparator};

    // 단 방향
    let direction = match col_def.layout {
        ColumnLayout::Left => ColumnDirection::LeftToRight,
        ColumnLayout::Right => ColumnDirection::RightToLeft,
        ColumnLayout::Mirror => ColumnDirection::FacingPages,
    };

    // 단 구분선
    let separator = if let Some(ref line) = col_def.column_line {
        use crate::core::enums::LineStyleType2;
        match line.line_type {
            LineStyleType2::None => ColumnSeparator::None,
            LineStyleType2::Solid => ColumnSeparator::Solid,
            LineStyleType2::Dash | LineStyleType2::LongDash => ColumnSeparator::Dash,
            LineStyleType2::Dot => ColumnSeparator::Dot,
            _ => ColumnSeparator::Solid,
        }
    } else {
        ColumnSeparator::None
    };

    // 개별 단 너비
    let widths: Vec<HwpUnit> = col_def
        .column_sizes
        .iter()
        .filter_map(|cs| cs.width.map(|w| HwpUnit(w as i32)))
        .collect();

    // 구분선 두께와 색상 변환
    let (separator_thickness, separator_color) = if let Some(ref line) = col_def.column_line {
        use crate::core::enums::LineWidth;
        let thickness = match line.width {
            LineWidth::Mm0_1 => 1,
            LineWidth::Mm0_12 => 1,
            LineWidth::Mm0_15 => 2,
            LineWidth::Mm0_2 => 2,
            LineWidth::Mm0_25 => 3,
            LineWidth::Mm0_3 => 3,
            LineWidth::Mm0_4 => 4,
            LineWidth::Mm0_5 => 5,
            LineWidth::Mm0_6 => 6,
            LineWidth::Mm0_7 => 7,
            LineWidth::Mm1_0 => 10,
            LineWidth::Mm1_5 => 15,
            LineWidth::Mm2_0 => 20,
            LineWidth::Mm3_0 => 30,
            LineWidth::Mm4_0 => 40,
            LineWidth::Mm5_0 => 50,
        };
        let color = primitive::Color::rgb(line.color.r, line.color.g, line.color.b);
        (thickness, color)
    } else {
        (0, primitive::Color::BLACK)
    };

    IrColumnDef {
        count: col_def.column_count as u16,
        direction,
        gap: HwpUnit(col_def.same_gap as i32),
        separator,
        separator_thickness,
        separator_color,
        widths,
    }
}

/// 섹션에서 ColumnDefinition 컨트롤 찾기
fn find_column_definition(
    section: &crate::paragraph::Section,
) -> Option<crate::paragraph::ColumnDefinition> {
    use crate::paragraph::{ControlItem, RunContent};

    for para in &section.paragraphs {
        for run in &para.runs {
            for content in &run.contents {
                if let RunContent::Control(ctrl) = content {
                    for item in &ctrl.items {
                        if let ControlItem::ColumnDefinition(col_def) = item {
                            return Some(col_def.clone());
                        }
                    }
                }
            }
        }
    }
    None
}

/// PageProperty → IR PageDefinition 변환
const fn convert_page_property(
    page_pr: &crate::paragraph::PageProperty,
) -> ir::section::PageDefinition {
    use crate::paragraph::enums::{GutterType, PaperOrientation};
    use ir::section::PageDefinition;
    use primitive::{GutterPosition, PageMargins, PageOrientation};

    // 용지 방향
    let orientation = match page_pr.orientation {
        PaperOrientation::Portrait => PageOrientation::Portrait,
        PaperOrientation::Landscape => PageOrientation::Landscape,
    };

    // 제본 여백 위치
    let gutter_position = match page_pr.gutter_type {
        GutterType::LeftOnly => GutterPosition::Left,
        GutterType::LeftRight => GutterPosition::Left,
        GutterType::TopBottom => GutterPosition::Top,
    };

    // 여백
    let margins = PageMargins {
        left: HwpUnit(page_pr.margin.left as i32),
        right: HwpUnit(page_pr.margin.right as i32),
        top: HwpUnit(page_pr.margin.top as i32),
        bottom: HwpUnit(page_pr.margin.bottom as i32),
        header: HwpUnit(page_pr.margin.header as i32),
        footer: HwpUnit(page_pr.margin.footer as i32),
        gutter: HwpUnit(page_pr.margin.gutter as i32),
    };

    PageDefinition {
        width: HwpUnit(page_pr.width as i32),
        height: HwpUnit(page_pr.height as i32),
        margins,
        orientation,
        gutter_position,
    }
}

/// HWPX 각주 모양 → IR FootnoteShape 변환
fn convert_hwpx_footnote_shape(
    footnote: &crate::paragraph::FootnoteShape,
) -> ir::section::FootnoteShape {
    use crate::paragraph::FootnotePlacement as HwpxFootnotePlacement;
    use crate::paragraph::enums::FootnoteNumberingType;

    // HWPX FootnoteNumberingType → IR NoteNumbering 변환
    let numbering = match footnote.numbering.numbering_type {
        FootnoteNumberingType::Continuous => primitive::NoteNumbering::Continuous,
        FootnoteNumberingType::OnSection => primitive::NoteNumbering::RestartSection,
        FootnoteNumberingType::OnPage => primitive::NoteNumbering::RestartPage,
    };

    // HWPX FootnotePlacement → IR FootnotePlacement 변환
    let placement = match footnote.placement.place {
        HwpxFootnotePlacement::EachColumn => primitive::FootnotePlacement::EachColumn,
        HwpxFootnotePlacement::MergedColumn => primitive::FootnotePlacement::MergedColumn,
        HwpxFootnotePlacement::RightMostColumn => primitive::FootnotePlacement::RightMostColumn,
    };

    // NoteLine → IR 변환
    let separator_line_type = convert_line_style_to_ir(&footnote.note_line.line_type);
    let separator_line_width = convert_line_width_to_ir(&footnote.note_line.width);
    let separator_line_color = primitive::Color::rgb(
        footnote.note_line.color.r,
        footnote.note_line.color.g,
        footnote.note_line.color.b,
    );

    ir::section::FootnoteShape {
        base: ir::section::NoteShape {
            number_format: convert_hwpx_number_format_type(
                &footnote.auto_number_format.number_type,
            ),
            numbering,
            superscript: footnote.auto_number_format.superscript,
            prefix: footnote.auto_number_format.prefix_character.clone(),
            suffix: Some(footnote.auto_number_format.suffix_character.clone()),
            start_number: footnote.numbering.new_number,
            user_character: footnote.auto_number_format.user_character.clone(),
            separator_length: convert_note_line_length(footnote.note_line.length),
            separator_position: None, // HWPX에서는 지원하지 않음
            separator_line_type,
            separator_line_width,
            separator_line_color,
            space_above: HwpUnit(footnote.note_spacing.above_line as i32),
            space_below: HwpUnit(footnote.note_spacing.below_line as i32),
            space_between: HwpUnit(footnote.note_spacing.between_notes as i32),
            beneath_text: footnote.placement.beneath_text,
        },
        placement,
    }
}

/// HWPX 미주 모양 → IR EndnoteShape 변환
fn convert_hwpx_endnote_shape(
    endnote: &crate::paragraph::EndnoteShape,
) -> ir::section::EndnoteShape {
    use crate::paragraph::EndnotePlacement as HwpxEndnotePlacement;
    use crate::paragraph::enums::EndnoteNumberingType;

    // HWPX EndnoteNumberingType → IR NoteNumbering 변환
    let numbering = match endnote.numbering.numbering_type {
        EndnoteNumberingType::Continuous => primitive::NoteNumbering::Continuous,
        EndnoteNumberingType::OnSection => primitive::NoteNumbering::RestartSection,
    };

    // HWPX EndnotePlacement → IR EndnotePlacement 변환
    let placement = match endnote.placement.place {
        HwpxEndnotePlacement::EndOfDocument => primitive::EndnotePlacement::EndOfDocument,
        HwpxEndnotePlacement::EndOfSection => primitive::EndnotePlacement::EndOfSection,
    };

    // NoteLine → IR 변환
    let separator_line_type = convert_line_style_to_ir(&endnote.note_line.line_type);
    let separator_line_width = convert_line_width_to_ir(&endnote.note_line.width);
    let separator_line_color = primitive::Color::rgb(
        endnote.note_line.color.r,
        endnote.note_line.color.g,
        endnote.note_line.color.b,
    );

    ir::section::EndnoteShape {
        base: ir::section::NoteShape {
            number_format: convert_hwpx_number_format_type(&endnote.auto_number_format.number_type),
            numbering,
            superscript: endnote.auto_number_format.superscript,
            prefix: endnote.auto_number_format.prefix_character.clone(),
            suffix: Some(endnote.auto_number_format.suffix_character.clone()),
            start_number: endnote.numbering.new_number,
            user_character: endnote.auto_number_format.user_character.clone(),
            separator_length: convert_note_line_length(endnote.note_line.length),
            separator_position: None, // HWPX에서는 지원하지 않음
            separator_line_type,
            separator_line_width,
            separator_line_color,
            space_above: HwpUnit(endnote.note_spacing.above_line as i32),
            space_below: HwpUnit(endnote.note_spacing.below_line as i32),
            space_between: HwpUnit(endnote.note_spacing.between_notes as i32),
            beneath_text: endnote.placement.beneath_text,
        },
        placement,
    }
}

/// HWPX LineStyleType2 → IR LineType 변환
const fn convert_line_style_to_ir(
    style: &crate::core::enums::LineStyleType2,
) -> primitive::LineType {
    use crate::core::enums::LineStyleType2;

    match style {
        LineStyleType2::None => primitive::LineType::None,
        LineStyleType2::Solid => primitive::LineType::Solid,
        LineStyleType2::Dash => primitive::LineType::Dash,
        LineStyleType2::Dot => primitive::LineType::Dot,
        LineStyleType2::DashDot => primitive::LineType::DashDot,
        LineStyleType2::DashDotDot => primitive::LineType::DashDotDot,
        LineStyleType2::LongDash => primitive::LineType::LongDash,
        LineStyleType2::Circle => primitive::LineType::Circle,
        _ => primitive::LineType::Solid,
    }
}

/// HWPX LineWidth → IR line width (0.1mm 단위) 변환
const fn convert_line_width_to_ir(width: &crate::core::enums::LineWidth) -> u8 {
    use crate::core::enums::LineWidth;

    match width {
        LineWidth::Mm0_1 => 1,
        LineWidth::Mm0_12 => 1,
        LineWidth::Mm0_15 => 2,
        LineWidth::Mm0_2 => 2,
        LineWidth::Mm0_25 => 3,
        LineWidth::Mm0_3 => 3,
        LineWidth::Mm0_4 => 4,
        LineWidth::Mm0_5 => 5,
        LineWidth::Mm0_6 => 6,
        LineWidth::Mm0_7 => 7,
        LineWidth::Mm1_0 => 10,
        LineWidth::Mm1_5 => 15,
        LineWidth::Mm2_0 => 20,
        LineWidth::Mm3_0 => 30,
        LineWidth::Mm4_0 => 40,
        LineWidth::Mm5_0 => 50,
    }
}

/// 주석 구분선 길이 변환 (HWPX → IR HwpUnit)
const fn convert_note_line_length(length: i32) -> primitive::HwpUnit {
    match length {
        0 => primitive::HwpUnit::ZERO,                      // 구분선 없음
        -1 => primitive::HwpUnit::from_mm(50.0),            // 5cm
        -2 => primitive::HwpUnit::from_mm(20.0),            // 2cm
        -3 => primitive::HwpUnit::from_mm(56.0),            // 단 크기의 1/3 (약 170mm/3)
        -4 => primitive::HwpUnit::from_mm(170.0),           // 단 크기 전체 (A4 기준 약 170mm)
        _ if length > 0 => primitive::HwpUnit::new(length), // HwpUnit 절대값 그대로 사용
        _ => primitive::HwpUnit::from_mm(20.0),             // 기본값 2cm
    }
}

/// HWPX NumberFormatType2 → IR NumberFormat 변환
const fn convert_hwpx_number_format_type(
    format_type: &crate::core::enums::NumberFormatType2,
) -> primitive::NumberFormat {
    use crate::core::enums::NumberFormatType2;
    use primitive::NumberFormat;

    match format_type {
        NumberFormatType2::Digit => NumberFormat::Digit,
        NumberFormatType2::CircledDigit => NumberFormat::CircledDigit,
        NumberFormatType2::RomanCapital => NumberFormat::RomanUpper,
        NumberFormatType2::RomanSmall => NumberFormat::RomanLower,
        NumberFormatType2::LatinCapital => NumberFormat::LatinUpper,
        NumberFormatType2::LatinSmall => NumberFormat::LatinLower,
        NumberFormatType2::HangulSyllable => NumberFormat::HangulSyllable,
        NumberFormatType2::HangulJamo => NumberFormat::HangulJamo,
        NumberFormatType2::HangulPhonetic => NumberFormat::HangulIdeograph,
        NumberFormatType2::Ideograph => NumberFormat::Ideograph,
        NumberFormatType2::CircledIdeograph => NumberFormat::CircledIdeograph,
        NumberFormatType2::CircledLatinCapital => NumberFormat::CircledLatinUpper,
        NumberFormatType2::CircledLatinSmall => NumberFormat::CircledLatinLower,
        NumberFormatType2::CircledHangulSyllable => NumberFormat::CircledHangul,
        NumberFormatType2::CircledHangulJamo => NumberFormat::CircledHangulJamo,
        NumberFormatType2::DecagonCircle => NumberFormat::CircledDigit,
        NumberFormatType2::DecagonCircleHanja => NumberFormat::CircledIdeograph,
        NumberFormatType2::Symbol => NumberFormat::Digit,
        NumberFormatType2::UserCharacter => NumberFormat::Digit,
    }
}

/// HWPX PageBorderFill → IR PageBorderFill 변환
fn convert_hwpx_page_border_fill(
    border_fill: &crate::paragraph::PageBorderFill,
) -> ir::section::PageBorderFill {
    use crate::paragraph::enums::PageBorderPosition as HwpxBorderPosition;
    use ir::section::{PageBorderFill as IrPageBorderFill, PageBorderPosition};

    // 테두리 위치 기준
    let position = match border_fill.text_border {
        Some(HwpxBorderPosition::Paper) => PageBorderPosition::Paper,
        Some(HwpxBorderPosition::Content) => PageBorderPosition::Body,
        None => PageBorderPosition::Paper,
    };

    // 테두리/채우기 ID
    let border_fill_id = border_fill
        .border_fill_id_reference
        .as_ref()
        .map(|id| BorderFillId::new(id.0))
        .unwrap_or(BorderFillId::new(0));

    // 페이지 타입 변환
    let page_type = match border_fill.page_type {
        Some(crate::paragraph::enums::PageBorderType::Both) => {
            ir::section::PageBorderPageType::Both
        }
        Some(crate::paragraph::enums::PageBorderType::Even) => {
            ir::section::PageBorderPageType::Even
        }
        Some(crate::paragraph::enums::PageBorderType::Odd) => ir::section::PageBorderPageType::Odd,
        None => ir::section::PageBorderPageType::Both,
    };

    // 채우기 영역 변환
    let fill_area = match border_fill.fill_area {
        Some(crate::paragraph::enums::FillAreaType::Paper) => {
            ir::section::PageBorderFillArea::Paper
        }
        Some(crate::paragraph::enums::FillAreaType::Page) => ir::section::PageBorderFillArea::Body,
        Some(crate::paragraph::enums::FillAreaType::Border) => {
            ir::section::PageBorderFillArea::Content
        }
        None => ir::section::PageBorderFillArea::Paper,
    };

    IrPageBorderFill {
        border_fill_id,
        position,
        offsets: Insets {
            left: HwpUnit(border_fill.offset.left as i32),
            right: HwpUnit(border_fill.offset.right as i32),
            top: HwpUnit(border_fill.offset.top as i32),
            bottom: HwpUnit(border_fill.offset.bottom as i32),
        },
        first_page_only: false, // HWPX에는 이 속성이 없음 (page_type으로 대체)
        header_inside: border_fill.header_inside,
        footer_inside: border_fill.footer_inside,
        fill_behind: false, // HWPX에는 이 속성이 없음
        page_type,
        fill_area,
    }
}

/// 문단 변환
fn convert_paragraph(para: &crate::paragraph::Paragraph) -> Result<IrParagraph, ConversionError> {
    let mut ir_para = IrParagraph::new();

    // 문단 모양 ID
    if let Some(ref para_pr_id) = para.paragraph_property_id_reference {
        ir_para.para_shape_id = Some(ParaShapeId::new(para_pr_id.0));
    }

    // 스타일 ID
    if let Some(ref style_id) = para.style_id_reference {
        ir_para.style_id = Some(StyleId::new(style_id.0));
    }

    // 인스턴스 ID
    ir_para.instance_id = Some(para.id);

    // 런 변환
    for run in &para.runs {
        let ir_run = convert_run(run)?;
        ir_para.runs.push(ir_run);
    }

    // 형광펜 범위 태그 추출
    extract_highlight_tags(para, &mut ir_para);

    // 줄 세그먼트 변환
    if let Some(ref line_segs) = para.line_segments {
        let segments: Vec<IrLineSegment> = line_segs
            .segments
            .iter()
            .map(|seg| IrLineSegment {
                text_start: seg.text_position as u32,
                vertical_position: HwpUnit(seg.vertical_position),
                line_height: HwpUnit(seg.vertical_size),
                text_height: HwpUnit(seg.text_height),
                baseline_distance: HwpUnit(seg.baseline),
                line_spacing: HwpUnit(seg.spacing),
                column_start: HwpUnit(seg.horizontal_position),
                segment_width: HwpUnit(seg.horizontal_size),
            })
            .collect();
        ir_para.line_segments = Some(segments);
    }

    // Break type
    if para.page_break {
        ir_para.break_type = primitive::BreakType::Page;
    } else if para.column_break {
        ir_para.break_type = primitive::BreakType::Column;
    }

    Ok(ir_para)
}

/// 런 변환
fn convert_run(run: &crate::paragraph::Run) -> Result<IrRun, ConversionError> {
    let mut ir_run = IrRun::new();

    // 글자 모양 ID
    if let Some(ref char_pr_id) = run.character_property_id_reference {
        ir_run.char_shape_id = Some(CharShapeId::new(char_pr_id.0));
    }

    // 런 내용 변환
    for content in &run.contents {
        ir_run.contents.extend(convert_run_content(content)?);
    }

    Ok(ir_run)
}

/// 런 내용 변환
fn convert_run_content(
    content: &crate::paragraph::RunContent,
) -> Result<Vec<IrRunContent>, ConversionError> {
    use crate::paragraph::RunContent;

    match content {
        RunContent::Text(text_elem) => {
            let mut results = Vec::new();
            for markup in &text_elem.contents {
                match markup {
                    crate::paragraph::TextMarkup::Text(s) => {
                        results.push(IrRunContent::Text(IrText::new(s.clone())));
                    }
                    crate::paragraph::TextMarkup::Tab(inline_tab) => {
                        use crate::core::enums::LineStyleType2;
                        use crate::paragraph::InlineTabType;

                        let tab_char = TabChar {
                            width: inline_tab.width.map(|w| primitive::HwpUnit::new(w as i32)),
                            leader: inline_tab.leader.and_then(|l| match l {
                                LineStyleType2::None => None,
                                LineStyleType2::Dot => Some('.'),
                                LineStyleType2::Solid => Some('_'),
                                LineStyleType2::Dash => Some('-'),
                                _ => Some('.'),
                            }),
                            tab_type: Some(match inline_tab.tab_type {
                                InlineTabType::Left => primitive::TabType::Left,
                                InlineTabType::Right => primitive::TabType::Right,
                                InlineTabType::Center => primitive::TabType::Center,
                                InlineTabType::Decimal => primitive::TabType::Decimal,
                            }),
                        };
                        results.push(IrRunContent::Tab(tab_char));
                    }
                    crate::paragraph::TextMarkup::LineBreak(_) => {
                        results.push(IrRunContent::LineBreak);
                    }
                    crate::paragraph::TextMarkup::NonBreakingSpace(_) => {
                        results.push(IrRunContent::NonBreakingSpace);
                    }
                    crate::paragraph::TextMarkup::FixedWidthSpace(_) => {
                        results.push(IrRunContent::FixedWidthSpace);
                    }
                    crate::paragraph::TextMarkup::Hyphen(_) => {
                        results.push(IrRunContent::Hyphen);
                    }
                    // 형광펜, 제목 표시, 변경 추적 등은 IR에서 별도 처리
                    _ => {}
                }
            }
            Ok(results)
        }
        RunContent::Table(table) => {
            let ir_table = convert_hwpx_table(table)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Table(
                Box::new(ir_table),
            )))])
        }
        RunContent::Picture(picture) => {
            let ir_picture = convert_hwpx_picture(picture)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Picture(
                Box::new(ir_picture),
            )))])
        }
        RunContent::Equation(equation) => {
            let ir_equation = convert_hwpx_equation(equation)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Equation(
                Box::new(ir_equation),
            )))])
        }
        RunContent::Line(line) => {
            let ir_shape = convert_hwpx_line(line)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Shape(
                Box::new(ir_shape),
            )))])
        }
        RunContent::Rectangle(rect) => {
            let ir_shape = convert_hwpx_rectangle(rect)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Shape(
                Box::new(ir_shape),
            )))])
        }
        RunContent::Ellipse(ellipse) => {
            let ir_shape = convert_hwpx_ellipse(ellipse)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Shape(
                Box::new(ir_shape),
            )))])
        }
        RunContent::Arc(arc) => {
            let ir_shape = convert_hwpx_arc(arc)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Shape(
                Box::new(ir_shape),
            )))])
        }
        RunContent::Polygon(polygon) => {
            let ir_shape = convert_hwpx_polygon(polygon)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Shape(
                Box::new(ir_shape),
            )))])
        }
        RunContent::Curve(curve) => {
            let ir_shape = convert_hwpx_curve(curve)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Shape(
                Box::new(ir_shape),
            )))])
        }
        RunContent::Control(control) => convert_hwpx_control(control),
        RunContent::Video(video) => {
            let ir_video = convert_hwpx_video(video)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Video(
                Box::new(ir_video),
            )))])
        }
        RunContent::Ole(ole) => {
            let ir_ole = convert_hwpx_ole(ole)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Ole(
                Box::new(ir_ole),
            )))])
        }
        RunContent::Chart(chart) => {
            let ir_chart = convert_hwpx_chart(chart)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Chart(
                Box::new(ir_chart),
            )))])
        }
        // 양식 객체들
        RunContent::Button(btn) => {
            let ir_form = convert_hwpx_button(btn, IrFormObjectType::Button)?;
            Ok(vec![IrRunContent::Control(Box::new(
                IrControl::FormObject(Box::new(ir_form)),
            ))])
        }
        RunContent::RadioButton(btn) => {
            let ir_form = convert_hwpx_button(btn, IrFormObjectType::RadioButton)?;
            Ok(vec![IrRunContent::Control(Box::new(
                IrControl::FormObject(Box::new(ir_form)),
            ))])
        }
        RunContent::CheckButton(btn) => {
            let ir_form = convert_hwpx_button(btn, IrFormObjectType::CheckBox)?;
            Ok(vec![IrRunContent::Control(Box::new(
                IrControl::FormObject(Box::new(ir_form)),
            ))])
        }
        RunContent::ComboBox(combo) => {
            let ir_form = convert_hwpx_combo_box(combo)?;
            Ok(vec![IrRunContent::Control(Box::new(
                IrControl::FormObject(Box::new(ir_form)),
            ))])
        }
        RunContent::ListBox(list) => {
            let ir_form = convert_hwpx_list_box(list)?;
            Ok(vec![IrRunContent::Control(Box::new(
                IrControl::FormObject(Box::new(ir_form)),
            ))])
        }
        RunContent::Edit(edit) => {
            let ir_form = convert_hwpx_edit(edit)?;
            Ok(vec![IrRunContent::Control(Box::new(
                IrControl::FormObject(Box::new(ir_form)),
            ))])
        }
        RunContent::ScrollBar(scroll) => {
            let ir_form = convert_hwpx_scroll_bar(scroll)?;
            Ok(vec![IrRunContent::Control(Box::new(
                IrControl::FormObject(Box::new(ir_form)),
            ))])
        }
        // 연결선
        RunContent::ConnectLine(connect_line) => {
            let ir_shape = convert_hwpx_connect_line(connect_line)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Shape(
                Box::new(ir_shape),
            )))])
        }
        // 글맵시
        RunContent::TextArt(text_art) => {
            let ir_text_art = convert_hwpx_text_art(text_art)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::TextArt(
                Box::new(ir_text_art),
            )))])
        }
        // 컨테이너 (그룹)
        RunContent::Container(container) => {
            let ir_shape = convert_hwpx_container(container)?;
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Shape(
                Box::new(ir_shape),
            )))])
        }
        // 글자 겹침 (Compose) - IR Compose로 변환
        RunContent::Compose(compose) => {
            use ir::paragraph::{
                Compose as IrCompose, ComposeCircleType as IrComposeCircleType,
                ComposeType as IrComposeType,
            };

            // ComposeType 변환
            let compose_type = compose.compose_type.map(|ct| match ct {
                crate::paragraph::text_art::ComposeType::Spread => IrComposeType::Spread,
                crate::paragraph::text_art::ComposeType::Overlap => IrComposeType::Overlap,
            });

            // ComposeCircleType 변환
            let circle_type = match compose.circle_type {
                crate::paragraph::text_art::ComposeCircleType::Char => IrComposeCircleType::Char,
                crate::paragraph::text_art::ComposeCircleType::ShapeCircle => {
                    IrComposeCircleType::ShapeCircle
                }
                crate::paragraph::text_art::ComposeCircleType::ShapeReversalCircle => {
                    IrComposeCircleType::ShapeReversalCircle
                }
                crate::paragraph::text_art::ComposeCircleType::ShapeRectangle => {
                    IrComposeCircleType::ShapeRectangle
                }
                crate::paragraph::text_art::ComposeCircleType::ShapeReversalRectangle => {
                    IrComposeCircleType::ShapeReversalRectangle
                }
                crate::paragraph::text_art::ComposeCircleType::ShapeTriangle => {
                    IrComposeCircleType::ShapeTriangle
                }
                crate::paragraph::text_art::ComposeCircleType::ShapeReversalTriangle => {
                    IrComposeCircleType::ShapeReversalTriangle
                }
                crate::paragraph::text_art::ComposeCircleType::ShapeLight => {
                    IrComposeCircleType::ShapeLight
                }
                crate::paragraph::text_art::ComposeCircleType::ShapeRhombus => {
                    IrComposeCircleType::ShapeRhombus
                }
                crate::paragraph::text_art::ComposeCircleType::ShapeReversalRhombus => {
                    IrComposeCircleType::ShapeReversalRhombus
                }
                crate::paragraph::text_art::ComposeCircleType::ShapeRoundedRectangle => {
                    IrComposeCircleType::ShapeRoundedRectangle
                }
                crate::paragraph::text_art::ComposeCircleType::ShapeEmptyCirculateTriangle => {
                    IrComposeCircleType::ShapeEmptyCirculateTriangle
                }
                crate::paragraph::text_art::ComposeCircleType::ShapeThinCirculateTriangle => {
                    IrComposeCircleType::ShapeThinCirculateTriangle
                }
                crate::paragraph::text_art::ComposeCircleType::ShapeThickCirculateTriangle => {
                    IrComposeCircleType::ShapeThickCirculateTriangle
                }
            };

            // 글자 속성 변환
            let char_shape_ids: Vec<Option<primitive::CharShapeId>> = compose
                .char_properties
                .iter()
                .map(|prop| {
                    prop.property_id_ref
                        .map(|id| primitive::CharShapeId::new(id.0))
                })
                .collect();

            let ir_compose = IrCompose {
                compose_text: compose.compose_text.clone().unwrap_or_default(),
                compose_type,
                circle_type,
                char_size: compose.char_size,
                char_shape_ids,
            };

            Ok(vec![IrRunContent::Compose(ir_compose)])
        }
        // 덧말 (Dutmal/Ruby) - IR Dutmal로 변환
        RunContent::Dutmal(dutmal) => {
            use ir::paragraph::{
                Dutmal as IrDutmal, DutmalAlignment as IrDutmalAlignment,
                DutmalPosition as IrDutmalPosition,
            };

            // DutmalPosition 변환
            let position_type = match dutmal.position_type {
                crate::paragraph::text_art::DutmalPosition::Top => IrDutmalPosition::Top,
                crate::paragraph::text_art::DutmalPosition::Bottom => IrDutmalPosition::Bottom,
            };

            // DutmalAlignment 변환
            let alignment = match dutmal.alignment {
                crate::paragraph::text_art::DutmalAlignment::Justify => IrDutmalAlignment::Justify,
                crate::paragraph::text_art::DutmalAlignment::Left => IrDutmalAlignment::Left,
                crate::paragraph::text_art::DutmalAlignment::Right => IrDutmalAlignment::Right,
                crate::paragraph::text_art::DutmalAlignment::Center => IrDutmalAlignment::Center,
                crate::paragraph::text_art::DutmalAlignment::Distribute => {
                    IrDutmalAlignment::Distribute
                }
                crate::paragraph::text_art::DutmalAlignment::DistributeSpace => {
                    IrDutmalAlignment::DistributeSpace
                }
            };

            let ir_dutmal = IrDutmal {
                main_text: dutmal.main_text.clone(),
                sub_text: dutmal.sub_text.clone(),
                position_type,
                size_ratio: dutmal.size_ratio,
                option: dutmal.option,
                style_id_ref: dutmal.style_id_ref.map(|id| primitive::StyleId::new(id.0)),
                alignment,
            };

            Ok(vec![IrRunContent::Dutmal(ir_dutmal)])
        }
        // 알 수 없는 개체 - IR에 Unknown 컨트롤로 보존
        RunContent::UnknownObject(_unknown_obj) => {
            let ir_unknown = ir::control::UnknownControl {
                ctrl_id: [b'u', b'n', b'k', b'n'],
                data: Vec::new(), // UnknownObject의 세부 데이터는 보존하지 않음
            };
            Ok(vec![IrRunContent::Control(Box::new(IrControl::Unknown(
                Box::new(ir_unknown),
            )))])
        }
        // 기타 알 수 없는 컨트롤
        _ => Ok(vec![]),
    }
}

/// HWPX 컨트롤 변환
fn convert_hwpx_control(
    control: &crate::paragraph::Control,
) -> Result<Vec<IrRunContent>, ConversionError> {
    use crate::paragraph::ControlItem;

    let mut results = Vec::new();
    for item in &control.items {
        match item {
            ControlItem::Header(header_footer) => {
                let ir_header = convert_hwpx_header(header_footer)?;
                results.push(IrRunContent::Control(Box::new(IrControl::Header(
                    Box::new(ir_header),
                ))));
            }
            ControlItem::Footer(header_footer) => {
                let ir_footer = convert_hwpx_footer(header_footer)?;
                results.push(IrRunContent::Control(Box::new(IrControl::Footer(
                    Box::new(ir_footer),
                ))));
            }
            ControlItem::Footnote(note) => {
                let ir_footnote = convert_hwpx_footnote(note)?;
                results.push(IrRunContent::Control(Box::new(IrControl::Footnote(
                    Box::new(ir_footnote),
                ))));
            }
            ControlItem::Endnote(note) => {
                let ir_endnote = convert_hwpx_endnote(note)?;
                results.push(IrRunContent::Control(Box::new(IrControl::Endnote(
                    Box::new(ir_endnote),
                ))));
            }
            ControlItem::Bookmark(bookmark) => {
                let ir_bookmark = convert_hwpx_bookmark(bookmark)?;
                results.push(IrRunContent::Control(Box::new(IrControl::Bookmark(
                    Box::new(ir_bookmark),
                ))));
            }
            ControlItem::IndexMark(index_mark) => {
                let ir_index_mark = convert_hwpx_index_mark(index_mark)?;
                results.push(IrRunContent::Control(Box::new(IrControl::IndexMark(
                    Box::new(ir_index_mark),
                ))));
            }
            ControlItem::FieldBegin(field_begin) => {
                // 하이퍼링크 필드는 Hyperlink로 변환
                if let Some(ir_hyperlink) = convert_hwpx_hyperlink_field(field_begin)? {
                    results.push(IrRunContent::Control(Box::new(IrControl::Hyperlink(
                        Box::new(ir_hyperlink),
                    ))));
                } else {
                    // 다른 필드 타입은 FieldStart로 변환
                    let ir_field_start = convert_hwpx_field_to_field_start(field_begin);
                    results.push(IrRunContent::FieldStart(ir_field_start));
                }
            }
            ControlItem::FieldEnd(field_end) => {
                // FieldEnd 변환 - begin_id_reference로 시작 필드와 매칭
                let ir_field_end = IrFieldEnd {
                    id: field_end.begin_id_reference.0,
                };
                results.push(IrRunContent::FieldEnd(ir_field_end));
            }
            ControlItem::HiddenComment(hidden_comment) => {
                let ir_comment = convert_hwpx_hidden_comment(hidden_comment)?;
                results.push(IrRunContent::Control(Box::new(IrControl::HiddenComment(
                    Box::new(ir_comment),
                ))));
            }
            ControlItem::AutoNumber(auto_num) => {
                let ir_auto_num = convert_hwpx_auto_number(auto_num)?;
                results.push(IrRunContent::Control(Box::new(IrControl::AutoNumber(
                    Box::new(ir_auto_num),
                ))));
            }
            ControlItem::NewNumber(new_num) => {
                let ir_new_num = convert_hwpx_new_number(new_num)?;
                results.push(IrRunContent::Control(Box::new(IrControl::NewNumber(
                    Box::new(ir_new_num),
                ))));
            }
            ControlItem::PageNumber(page_num) => {
                let ir_auto_num = convert_hwpx_page_number(page_num)?;
                results.push(IrRunContent::Control(Box::new(IrControl::AutoNumber(
                    Box::new(ir_auto_num),
                ))));
            }
            // 기타 컨트롤은 무시 (PageNumberControl, PageHiding, IndexMark 등)
            _ => {}
        }
    }
    Ok(results)
}

/// 바이너리 데이터 변환
fn convert_binary_data(hwpx: &HwpxDocument) -> Result<BinaryDataStore, ConversionError> {
    let mut store = BinaryDataStore::new();

    for (path, data) in &hwpx.binary_data {
        let format = detect_binary_format(data, path);
        let ir_data = BinaryData::new(format, data.clone());
        let ir_id = BinaryDataId::new(path.clone());
        store.add(ir_id, ir_data);
    }

    Ok(store)
}

/// 바이너리 데이터 형식 감지
fn detect_binary_format(data: &[u8], path: &str) -> BinaryFormat {
    // 확장자로 먼저 판단
    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "png" => return BinaryFormat::Png,
        "jpg" | "jpeg" => return BinaryFormat::Jpg,
        "gif" => return BinaryFormat::Gif,
        "bmp" => return BinaryFormat::Bmp,
        "tif" | "tiff" => return BinaryFormat::Tiff,
        "wmf" => return BinaryFormat::Wmf,
        "emf" => return BinaryFormat::Emf,
        _ => {}
    }

    // 매직 바이트로 판단
    if data.len() < 8 {
        return BinaryFormat::Unknown;
    }

    match &data[0..4] {
        [0x89, b'P', b'N', b'G'] => BinaryFormat::Png,
        [0xFF, 0xD8, 0xFF, _] => BinaryFormat::Jpg,
        [b'G', b'I', b'F', b'8'] => BinaryFormat::Gif,
        [b'B', b'M', _, _] => BinaryFormat::Bmp,
        [0xD0, 0xCF, 0x11, 0xE0] => BinaryFormat::Ole,
        [0x49, 0x49, 0x2A, 0x00] | [0x4D, 0x4D, 0x00, 0x2A] => BinaryFormat::Tiff,
        _ => BinaryFormat::Unknown,
    }
}

/// 확장 데이터 변환
fn convert_extensions(hwpx: &HwpxDocument, _ctx: &mut ToIrContext) -> Extensions {
    let mut ext = Extensions::new();

    // HWPX 고유 데이터
    // 마스터 페이지 정보 (내용 포함)
    let master_pages = hwpx
        .master_pages
        .iter()
        .map(|mp| {
            // 마스터페이지 적용 유형 변환
            let app_type = match mp.application_type {
                crate::master_page::MasterPageApplicationType::Both => {
                    ir::extensions::MasterPageApplicationType::Both
                }
                crate::master_page::MasterPageApplicationType::Even => {
                    ir::extensions::MasterPageApplicationType::Even
                }
                crate::master_page::MasterPageApplicationType::Odd => {
                    ir::extensions::MasterPageApplicationType::Odd
                }
                crate::master_page::MasterPageApplicationType::LastPage => {
                    ir::extensions::MasterPageApplicationType::Last
                }
                crate::master_page::MasterPageApplicationType::OptionalPage => {
                    ir::extensions::MasterPageApplicationType::Optional
                }
            };

            // 마스터페이지 문단들 변환
            let paragraphs = mp
                .paragraph_list
                .paragraphs
                .iter()
                .filter_map(|p| convert_paragraph(p).ok())
                .collect();

            ir::extensions::MasterPageInfo {
                id: mp.id.clone(),
                application_type: app_type,
                paragraphs,
                page_number: mp.page_number,
                page_duplicate: mp.page_duplicate,
                page_front: mp.page_front,
            }
        })
        .collect();

    let mut hwpx_ext = HwpxExtensions {
        master_pages,
        ..Default::default()
    };

    // 금칙 문자 목록
    if let Some(ref forbidden_list) = hwpx.header.forbidden_word_list {
        hwpx_ext.forbidden_words = forbidden_list.forbidden_words.clone();
    }

    // 레이아웃 호환성 설정
    if let Some(ref compat) = hwpx.header.compatible_document {
        use crate::header::compatible_document::TargetProgram;
        let target = match compat.target_program {
            TargetProgram::Hwp201X => ir::extensions::HwpxTargetProgram::Hwp201X,
            TargetProgram::Hwp200X => ir::extensions::HwpxTargetProgram::Hwp200X,
            TargetProgram::MsWord => ir::extensions::HwpxTargetProgram::MsWord,
        };
        hwpx_ext.layout_compatibility = Some(ir::extensions::HwpxLayoutCompatibility {
            target_program: target,
            flags: 0, // 개별 플래그는 필요시 추가
        });
    }

    // 변경 추적 설정
    if let Some(ref track_config) = hwpx.header.track_change_config {
        // TrackChangeConfig는 authors와 changes만 있으므로 존재 여부로 판단
        let enabled = track_config.authors.is_some() || track_config.changes.is_some();
        hwpx_ext.track_change_config = Some(ir::extensions::TrackChangeConfig {
            enabled,
            insert_color: None,
            delete_color: None,
        });
    }

    // 문서 옵션 (link_document_path)
    if let Some(ref doc_option) = hwpx.header.document_option
        && doc_option.link_document_path.is_some()
    {
        hwpx_ext.document_option = Some(ir::extensions::HwpxDocumentOption {
            link_document_path: doc_option.link_document_path.clone(),
            flags: 0,
        });
    }

    ext.hwpx = Some(hwpx_ext);

    ext
}

// 열거형 변환 헬퍼 함수들

const fn convert_hwpx_underline_type(
    shape: &crate::core::enums::LineStyleType2,
) -> primitive::UnderlineType {
    use crate::core::enums::LineStyleType2;
    use primitive::UnderlineType;

    match shape {
        LineStyleType2::Solid => UnderlineType::Single,
        LineStyleType2::DoubleSlim
        | LineStyleType2::SlimThick
        | LineStyleType2::ThickSlim
        | LineStyleType2::SlimThickSlim => UnderlineType::Double,
        LineStyleType2::Dash | LineStyleType2::LongDash => UnderlineType::Dash,
        LineStyleType2::DashDot => UnderlineType::DashDot,
        LineStyleType2::DashDotDot => UnderlineType::DashDotDot,
        LineStyleType2::Dot | LineStyleType2::Circle => UnderlineType::Dotted,
        LineStyleType2::None => UnderlineType::None,
    }
}

const fn convert_hwpx_underline_position(pos: &HwpxUnderlinePosition) -> UnderlinePosition {
    match pos {
        HwpxUnderlinePosition::Bottom => UnderlinePosition::Bottom,
        HwpxUnderlinePosition::Top => UnderlinePosition::Top,
        HwpxUnderlinePosition::Center => UnderlinePosition::Bottom, // IR에서는 Center 없음
        HwpxUnderlinePosition::None => UnderlinePosition::Bottom,
    }
}

const fn convert_hwpx_strikethrough_type(
    shape: &crate::core::enums::LineStyleType2,
) -> StrikethroughType {
    use crate::core::enums::LineStyleType2;

    match shape {
        LineStyleType2::None => StrikethroughType::None,
        LineStyleType2::Solid
        | LineStyleType2::Dash
        | LineStyleType2::Dot
        | LineStyleType2::DashDot
        | LineStyleType2::DashDotDot
        | LineStyleType2::LongDash
        | LineStyleType2::Circle => StrikethroughType::Single,
        LineStyleType2::DoubleSlim
        | LineStyleType2::SlimThick
        | LineStyleType2::ThickSlim
        | LineStyleType2::SlimThickSlim => StrikethroughType::Double,
    }
}

const fn convert_hwpx_outline_type(outline: &crate::core::enums::LineStyleType1) -> OutlineType {
    use crate::core::enums::LineStyleType1;

    match outline {
        LineStyleType1::None => OutlineType::None,
        LineStyleType1::Solid => OutlineType::Outline,
        _ => OutlineType::None,
    }
}

const fn convert_hwpx_emphasis_type(emphasis: &EmphasisMarkType) -> EmphasisType {
    match emphasis {
        EmphasisMarkType::None => EmphasisType::None,
        EmphasisMarkType::DotAbove => EmphasisType::Dot,
        EmphasisMarkType::RingAbove => EmphasisType::CircleOpen,
        EmphasisMarkType::Tilde => EmphasisType::Tilde,
        EmphasisMarkType::Caron => EmphasisType::Caron,
        EmphasisMarkType::Side => EmphasisType::Circle,
        EmphasisMarkType::Colon => EmphasisType::Colon,
        // GraveAccent, AcuteAccent, Circumflex, Macron, HookAbove, DotBelow 등은
        // IR에 대응하는 타입이 없으므로 None으로 fallback
        _ => EmphasisType::None,
    }
}

const fn convert_hwpx_shadow_type(shadow: &HwpxShadowType) -> ShadowType {
    match shadow {
        HwpxShadowType::None => ShadowType::None,
        HwpxShadowType::Drop => ShadowType::BottomRightDiscrete,
        HwpxShadowType::Continuous => ShadowType::BottomRightContinuous,
    }
}

const fn convert_hwpx_alignment(align: &HorizontalAlignment) -> Alignment {
    match align {
        HorizontalAlignment::Justify => Alignment::Justify,
        HorizontalAlignment::Left => Alignment::Left,
        HorizontalAlignment::Right => Alignment::Right,
        HorizontalAlignment::Center => Alignment::Center,
        HorizontalAlignment::Distribute => Alignment::Distribute,
        HorizontalAlignment::DistributeSpace => Alignment::Divide,
    }
}

const fn convert_hwpx_vertical_alignment(align: &HwpxVerticalAlignment) -> VerticalAlignment {
    match align {
        HwpxVerticalAlignment::Baseline => VerticalAlignment::Baseline,
        HwpxVerticalAlignment::Top => VerticalAlignment::Top,
        HwpxVerticalAlignment::Center => VerticalAlignment::Middle,
        HwpxVerticalAlignment::Bottom => VerticalAlignment::Bottom,
    }
}

const fn convert_hwpx_line_spacing_type(spacing_type: &HwpxLineSpacingType) -> LineSpacingType {
    match spacing_type {
        HwpxLineSpacingType::Percent => LineSpacingType::Percent,
        HwpxLineSpacingType::Fixed => LineSpacingType::Fixed,
        HwpxLineSpacingType::BetweenLines => LineSpacingType::FontBased,
        HwpxLineSpacingType::AtLeast => LineSpacingType::AtLeast,
    }
}

const fn convert_hwpx_break_korean(break_type: &NonLatinWordBreak) -> LineBreakKorean {
    match break_type {
        NonLatinWordBreak::KeepWord => LineBreakKorean::Word,
        NonLatinWordBreak::BreakWord => LineBreakKorean::Character,
    }
}

const fn convert_hwpx_break_latin(break_type: &LatinWordBreak) -> LineBreakLatin {
    match break_type {
        LatinWordBreak::KeepWord => LineBreakLatin::Word,
        LatinWordBreak::Hyphenation => LineBreakLatin::Hyphenation,
        LatinWordBreak::BreakWord => LineBreakLatin::Character,
    }
}

/// HWPX 표 변환
fn convert_hwpx_table(table: &crate::paragraph::Table) -> Result<IrTable, ConversionError> {
    let row_count = table.row_count.unwrap_or(table.rows.len() as u32) as u16;
    let col_count = table.column_count.unwrap_or_else(|| {
        table
            .rows
            .first()
            .map(|r| r.cells.len() as u32)
            .unwrap_or(0)
    }) as u16;

    let mut ir_table = IrTable::new(row_count, col_count);
    ir_table.cell_spacing = HwpUnit::new(table.cell_spacing as i32);

    // 테두리/채우기 ID
    if let Some(ref bf_id) = table.border_fill_id_ref {
        ir_table.border_fill_id = Some(BorderFillId::new(bf_id.0));
    }

    // 반복 제목행 설정
    ir_table.repeat_header = table.repeat_header;
    if table.repeat_header {
        ir_table.header_row_count = 1;
    }

    // 페이지 나눔 설정
    ir_table.page_break = match table.page_break {
        crate::paragraph::enums::TablePageBreak::None => ir::table::TablePageBreak::None,
        crate::paragraph::enums::TablePageBreak::Cell => ir::table::TablePageBreak::Cell,
        crate::paragraph::enums::TablePageBreak::Table => ir::table::TablePageBreak::Table,
    };

    // 자동 조정 안함
    ir_table.no_adjust = table.no_adjust;

    // 잠금 여부
    ir_table.lock = table.lock;

    // 안쪽 여백
    if let Some(ref margin) = table.inside_margin {
        ir_table.inside_margin = Some(Insets {
            left: HwpUnit::new(margin.left as i32),
            right: HwpUnit::new(margin.right as i32),
            top: HwpUnit::new(margin.top as i32),
            bottom: HwpUnit::new(margin.bottom as i32),
        });
    }

    // 셀존 (병합 영역) 변환
    if let Some(ref zone_list) = table.cell_zone_list {
        for zone in &zone_list.cell_zones {
            ir_table.zones.push(IrTableZone {
                start_row: zone.start_row_address.unwrap_or(0) as u16,
                start_column: zone.start_column_address.unwrap_or(0) as u16,
                end_row: zone.end_row_address.unwrap_or(0) as u16,
                end_column: zone.end_column_address.unwrap_or(0) as u16,
                border_fill_id: zone.border_fill_id_ref.map(|id| BorderFillId::new(id.0)),
            });
        }
    }

    // 행/셀 변환
    for hwpx_row in &table.rows {
        let mut ir_row = IrTableRow::new();

        for hwpx_cell in &hwpx_row.cells {
            let ir_cell = convert_hwpx_table_cell(hwpx_cell)?;
            ir_row.cells.push(ir_cell);
        }

        ir_table.rows.push(ir_row);
    }

    // ObjectCommon 설정
    ir_table.common.id = table.id;
    ir_table.common.z_order = table.z_order;

    // 크기 설정
    if let Some(ref sz) = table.size {
        ir_table.common.size = Size {
            width: HwpUnit::new(sz.width.unwrap_or(0) as i32),
            height: HwpUnit::new(sz.height.unwrap_or(0) as i32),
        };
    }

    // 위치 설정
    if let Some(ref pos) = table.position {
        ir_table.common.position = IrPoint {
            x: HwpUnit::new(pos.horizontal_offset as i32),
            y: HwpUnit::new(pos.vertical_offset as i32),
        };
        ir_table.common.text_wrap = convert_hwpx_position_to_text_wrap(
            Some(pos),
            table.text_wrap,
            Some(table.text_flow),
            table.outside_margin.as_ref(),
        );
    }

    // 캡션 설정
    if let Some(ref caption) = table.caption {
        ir_table.common.caption = Some(convert_hwpx_caption(caption)?);
    }

    // HWPX 확장 필드 설정
    ir_table.common.numbering_type = convert_numbering_type(table.numbering_type);
    ir_table.common.shape_comment = table.shape_comment.clone();
    ir_table.common.meta_tag = convert_meta_tag(&table.meta_tag);
    ir_table.common.dirty = false; // Table은 dirty 필드를 가지지 않음

    Ok(ir_table)
}

/// HWPX 표 셀 변환
fn convert_hwpx_table_cell(
    cell: &crate::paragraph::TableCell,
) -> Result<IrTableCell, ConversionError> {
    let row = cell.cell_address.row_address.unwrap_or(0) as u16;
    let column = cell.cell_address.column_address.unwrap_or(0) as u16;

    let mut ir_cell = IrTableCell::new(row, column);

    ir_cell.column_span = cell.cell_span.column_span as u16;
    ir_cell.row_span = cell.cell_span.row_span as u16;
    ir_cell.width = HwpUnit::new(cell.cell_size.width.unwrap_or(0) as i32);
    ir_cell.height = HwpUnit::new(cell.cell_size.height.unwrap_or(0) as i32);

    // 셀 여백
    ir_cell.padding = Insets {
        left: HwpUnit::new(cell.cell_margin.left as i32),
        right: HwpUnit::new(cell.cell_margin.right as i32),
        top: HwpUnit::new(cell.cell_margin.top as i32),
        bottom: HwpUnit::new(cell.cell_margin.bottom as i32),
    };

    // 테두리/채우기 ID
    if let Some(ref bf_id) = cell.border_fill_id_ref {
        ir_cell.border_fill_id = Some(BorderFillId::new(bf_id.0));
    }

    // HWPX 확장 필드
    ir_cell.is_header = cell.header;
    ir_cell.protect = cell.protect;
    ir_cell.name = cell.name.clone();
    ir_cell.editable = cell.editable;

    // 셀 내용 (문단들) 변환
    for para in &cell.paragraph_list.paragraphs {
        ir_cell.paragraphs.push(convert_paragraph(para)?);
    }

    Ok(ir_cell)
}

// =============================================================================
// 컨트롤 변환 함수들
// =============================================================================

/// HWPX 이미지 뒤집기 변환
const fn convert_hwpx_image_flip(flip: &crate::paragraph::shape_common::Flip) -> ImageFlip {
    match (flip.horizontal, flip.vertical) {
        (true, true) => ImageFlip::Both,
        (true, false) => ImageFlip::Horizontal,
        (false, true) => ImageFlip::Vertical,
        (false, false) => ImageFlip::None,
    }
}

/// HWPX 이미지 효과 변환
const fn convert_hwpx_image_effect(
    effect: &crate::core::enums::ImageEffect,
) -> primitive::ImageEffect {
    use crate::core::enums::ImageEffect as HwpxEffect;
    use primitive::ImageEffect as IrEffect;

    match effect {
        HwpxEffect::RealPicture => IrEffect::Original,
        HwpxEffect::GrayScale => IrEffect::Grayscale,
        HwpxEffect::BlackWhite => IrEffect::BlackWhite,
    }
}

/// HWPX 그림 변환
fn convert_hwpx_picture(picture: &crate::paragraph::Picture) -> Result<IrPicture, ConversionError> {
    // 바이너리 ID 추출
    let binary_id = picture
        .image
        .as_ref()
        .map(|img| BinaryDataId::new(img.binary_item_id_reference.0.clone()))
        .unwrap_or_else(|| BinaryDataId::new("unknown"));

    let mut ir_picture = IrPicture::new(binary_id);

    // 원본 크기 (Option 필드 처리)
    ir_picture.original_size = Size {
        width: HwpUnit::new(picture.original_size.width.unwrap_or(0) as i32),
        height: HwpUnit::new(picture.original_size.height.unwrap_or(0) as i32),
    };

    // 뒤집기 변환
    ir_picture.flip = convert_hwpx_image_flip(&picture.flip);

    // 회전 변환 (HWPX는 0.1도 단위)
    ir_picture.rotation = picture.rotation_info.angle as f64 / 10.0;

    // 이미지 클리핑 변환
    if let Some(ref clip) = picture.image_clip {
        ir_picture.crop = IrImageCrop {
            left: HwpUnit::new(clip.left.unwrap_or(0)),
            right: HwpUnit::new(clip.right.unwrap_or(0)),
            top: HwpUnit::new(clip.top.unwrap_or(0)),
            bottom: HwpUnit::new(clip.bottom.unwrap_or(0)),
        };
    }

    // 안쪽 여백 변환
    if let Some(ref margin) = picture.inside_margin {
        ir_picture.inside_margin = Insets {
            left: HwpUnit::new(margin.left as i32),
            right: HwpUnit::new(margin.right as i32),
            top: HwpUnit::new(margin.top as i32),
            bottom: HwpUnit::new(margin.bottom as i32),
        };
    }

    // 밝기/대비/효과/투명도 변환
    if let Some(ref img) = picture.image {
        // brightness/contrast는 -100~100 범위, 클램프 적용
        ir_picture.brightness = img.brightness.clamp(-100, 100) as i8;
        ir_picture.contrast = img.contrast.clamp(-100, 100) as i8;
        ir_picture.alpha = img.alpha.unwrap_or(1.0) as f64;
        ir_picture.effect = convert_hwpx_image_effect(&img.effect);
    }

    // 테두리 (라인 모양) 변환
    if let Some(ref line_shape) = picture.line_shape
        && let Some(width) = line_shape.width
        && width > 0
    {
        let color = line_shape
            .color
            .as_ref()
            .map(|c| Color::rgb(c.r, c.g, c.b))
            .unwrap_or(Color::rgb(0, 0, 0));
        ir_picture.border = Some(PictureBorder {
            line_type: convert_hwpx_line_style(&line_shape.style),
            width: HwpUnit::new(width as i32),
            color,
        });
    }

    // ObjectCommon 설정
    ir_picture.common.id = picture.id;
    ir_picture.common.z_order = picture.z_order;

    // 크기 설정
    if let Some(ref sz) = picture.size {
        ir_picture.common.size = Size {
            width: HwpUnit::new(sz.width.unwrap_or(0) as i32),
            height: HwpUnit::new(sz.height.unwrap_or(0) as i32),
        };
    }

    // 위치 설정
    if let Some(ref pos) = picture.position {
        ir_picture.common.position = IrPoint {
            x: HwpUnit::new(pos.horizontal_offset as i32),
            y: HwpUnit::new(pos.vertical_offset as i32),
        };
        ir_picture.common.text_wrap = convert_hwpx_position_to_text_wrap(
            Some(pos),
            picture.text_wrap,
            Some(picture.text_flow),
            picture.outside_margin.as_ref(),
        );
    }

    // 캡션 설정
    if let Some(ref caption) = picture.caption {
        ir_picture.common.caption = Some(convert_hwpx_caption(caption)?);
    }

    // 효과에서 그림자 추출
    if let Some(ref effects) = picture.effects
        && let Some(ref shadow) = effects.shadow
    {
        ir_picture.shadow = Some(convert_hwpx_shadow_to_ir(shadow));
    }

    // HWPX 확장 필드 설정
    ir_picture.common.numbering_type = convert_numbering_type(picture.numbering_type);
    ir_picture.common.shape_comment = picture.shape_comment.clone();
    ir_picture.common.meta_tag = convert_meta_tag(&picture.meta_tag);
    ir_picture.common.dirty = false; // Picture는 dirty 필드를 가지지 않음

    Ok(ir_picture)
}

/// HWPX AdvancedShadowEffect → IR PictureShadow 변환
const fn convert_hwpx_shadow_to_ir(
    shadow: &crate::paragraph::effects::AdvancedShadowEffect,
) -> ir::picture::PictureShadow {
    use ir::picture::{PictureShadow, PictureShadowType};

    // 방향(각도)에서 그림자 위치 결정
    let shadow_type = if let Some(direction) = shadow.direction {
        match direction {
            0..=44 | 316..=360 => PictureShadowType::BottomRight, // 동쪽(오른쪽)
            45..=134 => PictureShadowType::TopRight,              // 북쪽(위)
            135..=224 => PictureShadowType::TopLeft,              // 서쪽(왼쪽)
            225..=315 => PictureShadowType::BottomLeft,           // 남쪽(아래)
            _ => PictureShadowType::BottomRight,
        }
    } else {
        PictureShadowType::BottomRight
    };

    // 거리로 오프셋 계산
    let distance = match shadow.distance {
        Some(d) => d as i32,
        None => 100, // 기본값 100
    };
    let (offset_x, offset_y) = match shadow_type {
        PictureShadowType::TopLeft => (-distance, -distance),
        PictureShadowType::TopRight => (distance, -distance),
        PictureShadowType::BottomLeft => (-distance, distance),
        PictureShadowType::BottomRight => (distance, distance),
        PictureShadowType::None => (0, 0),
    };

    // 색상 추출 (RGB만 지원)
    let color = if let Some(ref value) = shadow.effects_color.value {
        match value {
            crate::paragraph::effects::EffectsColorValue::Rgb(rgb) => {
                Color::rgb(rgb.red as u8, rgb.green as u8, rgb.blue as u8)
            }
            _ => Color::rgb(128, 128, 128), // 기본 회색
        }
    } else {
        Color::rgb(128, 128, 128)
    };

    PictureShadow {
        shadow_type,
        color,
        offset_x: HwpUnit::new(offset_x),
        offset_y: HwpUnit::new(offset_y),
        alpha: match shadow.alpha {
            Some(alpha) => alpha as f64,
            None => 0.5,
        },
    }
}

/// HWPX 라인 스타일 변환
const fn convert_hwpx_line_style(style: &crate::core::enums::LineStyleType2) -> IrLineType {
    use crate::core::enums::LineStyleType2;

    match style {
        LineStyleType2::None => IrLineType::None,
        LineStyleType2::Solid => IrLineType::Solid,
        LineStyleType2::Dot => IrLineType::Dot,
        LineStyleType2::Dash => IrLineType::Dash,
        LineStyleType2::DashDot => IrLineType::DashDot,
        LineStyleType2::DashDotDot => IrLineType::DashDotDot,
        LineStyleType2::LongDash => IrLineType::LongDash,
        LineStyleType2::Circle => IrLineType::Solid,
        LineStyleType2::DoubleSlim
        | LineStyleType2::SlimThick
        | LineStyleType2::ThickSlim
        | LineStyleType2::SlimThickSlim => IrLineType::Double,
    }
}

/// HWPX 수식 변환
fn convert_hwpx_equation(
    equation: &crate::paragraph::Equation,
) -> Result<IrEquation, ConversionError> {
    use ir::control::EquationFormat;

    // 크기 추출 (size가 있으면 사용)
    let (width, height) = if let Some(sz) = &equation.size {
        (sz.width.unwrap_or(0), sz.height.unwrap_or(0))
    } else {
        (0, 0)
    };

    let common = create_object_common_from_hwpx(ObjectCommonParams {
        id: equation.id,
        offset_x: 0, // Equation에는 offset이 없음
        offset_y: 0,
        width,
        height,
        z_order: equation.z_order,
        position: equation.position.as_ref(),
        text_wrap_mode: equation.text_wrap,
        text_flow_mode: Some(equation.text_flow),
        outside_margin: equation.outside_margin.as_ref(),
    });

    Ok(IrEquation {
        common,
        script: equation.script.clone(),
        format: EquationFormat::HwpScript,
        baseline_offset: HwpUnit::new(equation.baseline as i32),
        font_size: HwpUnit::new(equation.base_unit as i32),
        color: Some(Color::rgb(
            equation.text_color.r,
            equation.text_color.g,
            equation.text_color.b,
        )),
        line_mode: None,
        version: None,
        font_name: None,
        properties: None,
    })
}

// =============================================================================
// 도형 변환 함수들
// =============================================================================

/// HWPX 선 변환
fn convert_hwpx_line(line: &crate::paragraph::Line) -> Result<IrShape, ConversionError> {
    let start = IrPoint {
        x: HwpUnit::new(line.start_point.x.unwrap_or(0)),
        y: HwpUnit::new(line.start_point.y.unwrap_or(0)),
    };
    let end = IrPoint {
        x: HwpUnit::new(line.end_point.x.unwrap_or(0)),
        y: HwpUnit::new(line.end_point.y.unwrap_or(0)),
    };

    // line_shape에서 화살표 정보 추출
    let start_arrow = convert_arrow_style(
        &line.line_shape.head_style,
        line.line_shape.head_fill,
        &line.line_shape.head_size,
    );
    let end_arrow = convert_arrow_style(
        &line.line_shape.tail_style,
        line.line_shape.tail_fill,
        &line.line_shape.tail_size,
    );

    let shape_type = IrShapeType::Line(IrLineShape {
        start,
        end,
        start_arrow,
        end_arrow,
    });

    // draw_text가 있으면 ShapeText로 변환
    let text = line.draw_text.as_ref().map(convert_draw_text_to_shape_text);

    // ObjectCommon 생성
    let common = create_drawing_object_common(DrawingObjectCommonParams {
        size: line.size.as_ref(),
        position: line.position.as_ref(),
        offset: &line.offset,
        current_size: &line.current_size,
        z_order: line.z_order,
        text_wrap_mode: None,
        text_flow_mode: None,
        outside_margin: None,
    });

    Ok(IrShape {
        translation_matrix: None,
        scale_matrix: None,
        rotation_matrix: None,
        common,
        shape_type,
        line: convert_hwpx_shape_line_style(&line.line_shape),
        fill: convert_fill_brush(line.fill_brush.as_ref()),
        shadow: line.shadow.as_ref().map(convert_hwpx_shape_shadow),
        rotation: line.rotation_info.angle as f64 / 10.0, // HWPX는 0.1도 단위
        text,
    })
}

/// HWPX 사각형 변환
fn convert_hwpx_rectangle(rect: &crate::paragraph::Rectangle) -> Result<IrShape, ConversionError> {
    let corner_radius = rect.ratio.unwrap_or(0);

    let shape_type = IrShapeType::Rectangle(IrRectangleShape {
        corner_radius: HwpUnit::new(corner_radius as i32),
    });

    // draw_text가 있으면 ShapeText로 변환
    let text = rect.draw_text.as_ref().map(convert_draw_text_to_shape_text);

    // ObjectCommon 생성
    let common = create_drawing_object_common(DrawingObjectCommonParams {
        size: rect.size.as_ref(),
        position: rect.position.as_ref(),
        offset: &rect.offset,
        current_size: &rect.current_size,
        z_order: rect.z_order,
        text_wrap_mode: None,
        text_flow_mode: None,
        outside_margin: None,
    });

    Ok(IrShape {
        translation_matrix: None,
        scale_matrix: None,
        rotation_matrix: None,
        common,
        shape_type,
        line: convert_hwpx_shape_line_style(&rect.line_shape),
        fill: convert_fill_brush(rect.fill_brush.as_ref()),
        shadow: rect.shadow.as_ref().map(convert_hwpx_shape_shadow),
        rotation: rect.rotation_info.angle as f64 / 10.0, // HWPX는 0.1도 단위
        text,
    })
}

/// HWPX 타원 변환
fn convert_hwpx_ellipse(ellipse: &crate::paragraph::Ellipse) -> Result<IrShape, ConversionError> {
    use crate::paragraph::shape_common::ArcStyle;

    let arc_type = match ellipse.arc_type {
        ArcStyle::Normal => IrArcType::Full,
        ArcStyle::Pie => IrArcType::Pie,
        ArcStyle::Chord => IrArcType::Chord,
    };

    // 중심점과 start1/end1에서 각도 계산
    let (start_angle, end_angle) =
        calculate_hwpx_arc_angles(&ellipse.center, &ellipse.start1, &ellipse.end1);

    let shape_type = IrShapeType::Ellipse(IrEllipseShape {
        arc_type,
        start_angle,
        end_angle,
    });

    // draw_text가 있으면 ShapeText로 변환
    let text = ellipse
        .draw_text
        .as_ref()
        .map(convert_draw_text_to_shape_text);

    // ObjectCommon 생성
    let common = create_drawing_object_common(DrawingObjectCommonParams {
        size: ellipse.size.as_ref(),
        position: ellipse.position.as_ref(),
        offset: &ellipse.offset,
        current_size: &ellipse.current_size,
        z_order: ellipse.z_order,
        text_wrap_mode: None,
        text_flow_mode: None,
        outside_margin: None,
    });

    Ok(IrShape {
        translation_matrix: None,
        scale_matrix: None,
        rotation_matrix: None,
        common,
        shape_type,
        line: convert_hwpx_shape_line_style(&ellipse.line_shape),
        fill: convert_fill_brush(ellipse.fill_brush.as_ref()),
        shadow: ellipse.shadow.as_ref().map(convert_hwpx_shape_shadow),
        rotation: ellipse.rotation_info.angle as f64 / 10.0, // HWPX는 0.1도 단위
        text,
    })
}

/// HWPX 호 변환
fn convert_hwpx_arc(arc: &crate::paragraph::Arc) -> Result<IrShape, ConversionError> {
    use crate::paragraph::shape_common::ArcStyle;

    let arc_type = match arc.arc_type {
        ArcStyle::Normal => IrArcType::Arc,
        ArcStyle::Pie => IrArcType::Pie,
        ArcStyle::Chord => IrArcType::Chord,
    };

    // 중심점과 axis1/axis2에서 각도 계산
    let (start_angle, end_angle) = calculate_hwpx_arc_angles(&arc.center, &arc.axis1, &arc.axis2);

    let shape_type = IrShapeType::Arc(IrArcShape {
        arc_type,
        start_angle,
        end_angle,
    });

    // draw_text가 있으면 ShapeText로 변환
    let text = arc.draw_text.as_ref().map(convert_draw_text_to_shape_text);

    // ObjectCommon 생성
    let common = create_drawing_object_common(DrawingObjectCommonParams {
        size: arc.size.as_ref(),
        position: arc.position.as_ref(),
        offset: &arc.offset,
        current_size: &arc.current_size,
        z_order: arc.z_order,
        text_wrap_mode: None,
        text_flow_mode: None,
        outside_margin: None,
    });

    Ok(IrShape {
        translation_matrix: None,
        scale_matrix: None,
        rotation_matrix: None,
        common,
        shape_type,
        line: convert_hwpx_shape_line_style(&arc.line_shape),
        fill: convert_fill_brush(arc.fill_brush.as_ref()),
        shadow: arc.shadow.as_ref().map(convert_hwpx_shape_shadow),
        rotation: arc.rotation_info.angle as f64 / 10.0, // HWPX는 0.1도 단위
        text,
    })
}

/// HWPX 다각형 변환
fn convert_hwpx_polygon(polygon: &crate::paragraph::Polygon) -> Result<IrShape, ConversionError> {
    let points: Vec<IrPoint> = polygon
        .points
        .iter()
        .map(|p| IrPoint {
            x: HwpUnit::new(p.x.unwrap_or(0)),
            y: HwpUnit::new(p.y.unwrap_or(0)),
        })
        .collect();

    let shape_type = IrShapeType::Polygon(IrPolygonShape { points });

    // draw_text가 있으면 ShapeText로 변환
    let text = polygon
        .draw_text
        .as_ref()
        .map(convert_draw_text_to_shape_text);

    // ObjectCommon 생성
    let common = create_drawing_object_common(DrawingObjectCommonParams {
        size: polygon.size.as_ref(),
        position: polygon.position.as_ref(),
        offset: &polygon.offset,
        current_size: &polygon.current_size,
        z_order: polygon.z_order,
        text_wrap_mode: None,
        text_flow_mode: None,
        outside_margin: None,
    });

    Ok(IrShape {
        translation_matrix: None,
        scale_matrix: None,
        rotation_matrix: None,
        common,
        shape_type,
        line: convert_hwpx_shape_line_style(&polygon.line_shape),
        fill: convert_fill_brush(polygon.fill_brush.as_ref()),
        shadow: polygon.shadow.as_ref().map(convert_hwpx_shape_shadow),
        rotation: polygon.rotation_info.angle as f64 / 10.0, // HWPX는 0.1도 단위
        text,
    })
}

/// HWPX 곡선 변환
fn convert_hwpx_curve(curve: &crate::paragraph::Curve) -> Result<IrShape, ConversionError> {
    use crate::paragraph::shape_common::CurveSegmentType;

    let points: Vec<IrCurvePoint> = curve
        .segments
        .iter()
        .flat_map(|seg| {
            let point_type = match seg.segment_type {
                CurveSegmentType::Line => IrCurvePointType::Normal,
                CurveSegmentType::Curve => IrCurvePointType::Control1,
            };
            vec![IrCurvePoint {
                point: IrPoint {
                    x: HwpUnit::new(seg.x1.unwrap_or(0)),
                    y: HwpUnit::new(seg.y1.unwrap_or(0)),
                },
                point_type,
            }]
        })
        .collect();

    // 첫 번째 점과 마지막 점이 같으면 닫힌 곡선으로 판단
    // 또는 채우기가 있으면 닫힌 곡선으로 추정
    let closed = if points.len() >= 2 {
        let first = &points[0].point;
        let last = &points[points.len() - 1].point;
        let points_match = (first.x.value() - last.x.value()).abs() < 10
            && (first.y.value() - last.y.value()).abs() < 10;
        let has_fill = curve.fill_brush.is_some();
        points_match || has_fill
    } else {
        false
    };

    let shape_type = IrShapeType::Curve(IrCurveShape { points, closed });

    // draw_text가 있으면 ShapeText로 변환
    let text = curve
        .draw_text
        .as_ref()
        .map(convert_draw_text_to_shape_text);

    // ObjectCommon 생성
    let common = create_drawing_object_common(DrawingObjectCommonParams {
        size: curve.size.as_ref(),
        position: curve.position.as_ref(),
        offset: &curve.offset,
        current_size: &curve.current_size,
        z_order: curve.z_order,
        text_wrap_mode: None,
        text_flow_mode: None,
        outside_margin: None, // Curve에는 outside_margin 없음
    });

    Ok(IrShape {
        translation_matrix: None,
        scale_matrix: None,
        rotation_matrix: None,
        common,
        shape_type,
        line: convert_hwpx_shape_line_style(&curve.line_shape),
        fill: convert_fill_brush(curve.fill_brush.as_ref()),
        shadow: curve.shadow.as_ref().map(convert_hwpx_shape_shadow),
        rotation: curve.rotation_info.angle as f64 / 10.0, // HWPX는 0.1도 단위
        text,
    })
}

/// HWPX 도형 선 스타일 변환
fn convert_hwpx_shape_line_style(line_shape: &crate::paragraph::LineShape) -> IrLineStyle {
    use primitive::LineOutlineStyle;

    let color = line_shape
        .color
        .as_ref()
        .map(|c| Color::rgb(c.r, c.g, c.b))
        .unwrap_or(Color::rgb(0, 0, 0));

    let width = line_shape.width.unwrap_or(0) as i32;

    let outline_style = match line_shape.outline_style {
        crate::paragraph::line_shape::OutlineStyle::Normal => LineOutlineStyle::Normal,
        crate::paragraph::line_shape::OutlineStyle::Outer => LineOutlineStyle::Outer,
        crate::paragraph::line_shape::OutlineStyle::Inner => LineOutlineStyle::Inner,
    };

    IrLineStyle {
        line_type: convert_hwpx_line_style(&line_shape.style),
        width: HwpUnit::new(width),
        color,
        cap: IrLineCap::Flat,
        outline_style,
        alpha: line_shape.alpha,
    }
}

// =============================================================================
// 컨트롤 아이템 변환 함수들 (Header, Footer, Notes, etc.)
// =============================================================================

/// HWPX 머리글 변환
fn convert_hwpx_header(
    header: &crate::paragraph::HeaderFooter,
) -> Result<HeaderFooterControl, ConversionError> {
    let apply_to = convert_hwpx_page_apply_type(&header.apply_page_type);
    let mut paragraphs = Vec::new();
    for para in &header.sub_list.paragraphs {
        paragraphs.push(convert_paragraph(para)?);
    }
    Ok(HeaderFooterControl {
        apply_to,
        paragraphs,
    })
}

/// HWPX 바닥글 변환
fn convert_hwpx_footer(
    footer: &crate::paragraph::HeaderFooter,
) -> Result<HeaderFooterControl, ConversionError> {
    let apply_to = convert_hwpx_page_apply_type(&footer.apply_page_type);
    let mut paragraphs = Vec::new();
    for para in &footer.sub_list.paragraphs {
        paragraphs.push(convert_paragraph(para)?);
    }
    Ok(HeaderFooterControl {
        apply_to,
        paragraphs,
    })
}

/// HWPX 페이지 적용 타입 변환
const fn convert_hwpx_page_apply_type(
    apply_type: &crate::paragraph::PageStartsOn,
) -> HeaderFooterApplyTo {
    use crate::paragraph::PageStartsOn;

    match apply_type {
        PageStartsOn::Both => HeaderFooterApplyTo::Both,
        PageStartsOn::Even => HeaderFooterApplyTo::Even,
        PageStartsOn::Odd => HeaderFooterApplyTo::Odd,
    }
}

/// HWPX 각주 변환
fn convert_hwpx_footnote(note: &crate::paragraph::Note) -> Result<IrNote, ConversionError> {
    let mut paragraphs = Vec::new();
    for para in &note.sub_list.paragraphs {
        paragraphs.push(convert_paragraph(para)?);
    }
    Ok(IrNote {
        number: 0, // HWPX에서는 자동 번호 매김
        number_format: NumberFormat::Digit,
        number_position: primitive::NoteNumberPosition::Superscript,
        paragraphs,
        instance_id: note.instance_id,
    })
}

/// HWPX 미주 변환
fn convert_hwpx_endnote(note: &crate::paragraph::Note) -> Result<IrNote, ConversionError> {
    let mut paragraphs = Vec::new();
    for para in &note.sub_list.paragraphs {
        paragraphs.push(convert_paragraph(para)?);
    }
    Ok(IrNote {
        number: 0, // HWPX에서는 자동 번호 매김
        number_format: NumberFormat::Digit,
        number_position: primitive::NoteNumberPosition::Superscript,
        paragraphs,
        instance_id: note.instance_id,
    })
}

/// HWPX 책갈피 변환
fn convert_hwpx_bookmark(
    bookmark: &crate::paragraph::Bookmark,
) -> Result<ir::control::Bookmark, ConversionError> {
    Ok(ir::control::Bookmark {
        name: bookmark.name.clone().unwrap_or_default(),
    })
}

/// HWPX IndexMark → IR IndexMark 변환
fn convert_hwpx_index_mark(
    index_mark: &crate::paragraph::IndexMark,
) -> Result<ir::control::IndexMark, ConversionError> {
    Ok(ir::control::IndexMark {
        first_key: index_mark.first_key.clone(),
        second_key: index_mark.second_key.clone(),
    })
}

/// HWPX 하이퍼링크 필드 변환
fn convert_hwpx_hyperlink_field(
    field: &crate::paragraph::FieldBegin,
) -> Result<Option<IrHyperlink>, ConversionError> {
    use crate::paragraph::FieldType;

    // 하이퍼링크 필드인 경우에만 변환
    if field.field_type != FieldType::Hyperlink {
        return Ok(None);
    }

    // 파라미터에서 URL 추출
    let target = field
        .parameters
        .as_ref()
        .and_then(|params| params.items.first())
        .and_then(|first_item| {
            if let crate::paragraph::ParameterItem::String(string_param) = first_item {
                Some(string_param.value.clone())
            } else {
                None
            }
        })
        .unwrap_or_default();

    // URL 타입 결정
    let hyperlink_target = if target.starts_with("mailto:") {
        HyperlinkTarget::Email(target.trim_start_matches("mailto:").to_string())
    } else if target.starts_with("http://") || target.starts_with("https://") {
        HyperlinkTarget::Url(target)
    } else if target.starts_with('#') {
        HyperlinkTarget::Bookmark(target.trim_start_matches('#').to_string())
    } else {
        HyperlinkTarget::File(target)
    };

    // subList에서 display_text 추출 (필드 내용의 텍스트)
    let display_text = if let Some(ref sub_list) = field.sub_list {
        // 첫 번째 문단의 첫 번째 런에서 텍스트 추출
        sub_list.paragraphs.first().and_then(|para| {
            para.runs.first().and_then(|run| {
                run.contents.first().and_then(|run_content| {
                    if let crate::paragraph::RunContent::Text(text_elem) = run_content {
                        Some(text_elem.text())
                    } else {
                        None
                    }
                })
            })
        })
    } else {
        None
    };

    Ok(Some(IrHyperlink {
        target: hyperlink_target,
        tooltip: field.name.clone(),
        display_text,
    }))
}

/// HWPX 숨은 주석 변환
fn convert_hwpx_hidden_comment(
    comment: &crate::paragraph::HiddenComment,
) -> Result<ir::control::HiddenComment, ConversionError> {
    let mut paragraphs = Vec::new();
    for para in &comment.sub_list.paragraphs {
        paragraphs.push(convert_paragraph(para)?);
    }
    Ok(ir::control::HiddenComment { paragraphs })
}

/// HWPX 자동 번호 변환
fn convert_hwpx_auto_number(
    auto_num: &crate::paragraph::AutoNumberNewNumber,
) -> Result<ir::control::AutoNumber, ConversionError> {
    use crate::paragraph::AutoNumberKind;
    use ir::control::AutoNumberType;

    let number_type = match auto_num.number_type {
        Some(AutoNumberKind::Page) => AutoNumberType::Page,
        Some(AutoNumberKind::Footnote) => AutoNumberType::Footnote,
        Some(AutoNumberKind::Endnote) => AutoNumberType::Endnote,
        Some(AutoNumberKind::Picture) => AutoNumberType::Picture,
        Some(AutoNumberKind::Table) => AutoNumberType::Table,
        Some(AutoNumberKind::Equation) => AutoNumberType::Equation,
        Some(AutoNumberKind::TotalPage) => AutoNumberType::TotalPages,
        None => AutoNumberType::Page,
    };

    let number_format = if let Some(ref format) = auto_num.auto_number_format {
        convert_hwpx_number_format_type(&format.number_type)
    } else {
        NumberFormat::Digit
    };

    Ok(ir::control::AutoNumber {
        number_type,
        number_format,
        auto_number_format: None, // TODO: HWPX에서 추출 필요
    })
}

/// HWPX 새 번호 변환
const fn convert_hwpx_new_number(
    new_num: &crate::paragraph::AutoNumberNewNumber,
) -> Result<ir::control::NewNumber, ConversionError> {
    use crate::paragraph::AutoNumberKind;
    use ir::control::AutoNumberType;

    let number_type = match new_num.number_type {
        Some(AutoNumberKind::Page) => AutoNumberType::Page,
        Some(AutoNumberKind::Footnote) => AutoNumberType::Footnote,
        Some(AutoNumberKind::Endnote) => AutoNumberType::Endnote,
        Some(AutoNumberKind::Picture) => AutoNumberType::Picture,
        Some(AutoNumberKind::Table) => AutoNumberType::Table,
        Some(AutoNumberKind::Equation) => AutoNumberType::Equation,
        Some(AutoNumberKind::TotalPage) => AutoNumberType::TotalPages,
        None => AutoNumberType::Page,
    };

    Ok(ir::control::NewNumber {
        number_type,
        number: new_num.number as u32,
    })
}

/// HWPX 페이지 번호 변환
fn convert_hwpx_page_number(
    page_num: &crate::paragraph::PageNumber,
) -> Result<ir::control::AutoNumber, ConversionError> {
    use crate::core::enums::NumberFormatType1;
    use ir::control::{AutoNumber, AutoNumberFormat, AutoNumberType};

    // NumberFormatType1을 IR NumberFormat으로 변환
    let number_format = match page_num.format_type {
        NumberFormatType1::Digit => NumberFormat::Digit,
        NumberFormatType1::CircledDigit => NumberFormat::CircledDigit,
        NumberFormatType1::RomanCapital => NumberFormat::RomanUpper,
        NumberFormatType1::RomanSmall => NumberFormat::RomanLower,
        NumberFormatType1::LatinCapital => NumberFormat::LatinUpper,
        NumberFormatType1::LatinSmall => NumberFormat::LatinLower,
        NumberFormatType1::HangulSyllable => NumberFormat::HangulSyllable,
        NumberFormatType1::HangulJamo => NumberFormat::HangulJamo,
        NumberFormatType1::CircledHangulSyllable => NumberFormat::CircledHangul,
        NumberFormatType1::Ideograph => NumberFormat::Ideograph,
        NumberFormatType1::CircledIdeograph => NumberFormat::CircledIdeograph,
        NumberFormatType1::CircledHangulJamo => NumberFormat::CircledHangulJamo,
        NumberFormatType1::CircledLatinCapital => NumberFormat::CircledLatinUpper,
        NumberFormatType1::CircledLatinSmall => NumberFormat::CircledLatinLower,
        NumberFormatType1::HangulPhonetic => NumberFormat::HangulIdeograph,
    };

    // AutoNumberFormat 구성 (HWPX PageNumber의 정보를 IR AutoNumberFormat으로 변환)
    let auto_number_format = Some(AutoNumberFormat {
        user_character: None,
        prefix_character: Some(page_num.side_character.clone()),
        suffix_character: page_num.side_character.clone(),
        superscript: false,
        position: Some(convert_hwpx_page_number_position(&page_num.position)),
        format_type: Some(number_format),
        side_character: Some(page_num.side_character.clone()),
    });

    Ok(AutoNumber {
        number_type: AutoNumberType::Page,
        number_format,
        auto_number_format,
    })
}

/// HWPX PageNumberPosition → IR PageNumberPosition 변환
const fn convert_hwpx_page_number_position(
    pos: &crate::paragraph::enums::PageNumberPosition,
) -> primitive::PageNumberPosition {
    use crate::paragraph::enums::PageNumberPosition as HwpxPos;
    use primitive::PageNumberPosition as IrPos;

    match pos {
        HwpxPos::TopLeft => IrPos::TopLeft,
        HwpxPos::TopCenter => IrPos::TopCenter,
        HwpxPos::TopRight => IrPos::TopRight,
        HwpxPos::BottomLeft => IrPos::BottomLeft,
        HwpxPos::BottomCenter => IrPos::BottomCenter,
        HwpxPos::BottomRight => IrPos::BottomRight,
        HwpxPos::OutsideTop => IrPos::OutsideTop,
        HwpxPos::OutsideBottom => IrPos::OutsideBottom,
        HwpxPos::InsideTop => IrPos::InsideTop,
        HwpxPos::InsideBottom => IrPos::InsideBottom,
        HwpxPos::None => IrPos::None,
    }
}

/// HWPX 호 각도 계산
///
/// 중심점과 start/end 점에서 호의 시작/종료 각도를 계산합니다 (도 단위).
/// atan2를 사용하여 점의 방향 각도를 계산합니다.
fn calculate_hwpx_arc_angles(
    center: &crate::core::types::Point,
    start: &crate::core::types::Point,
    end: &crate::core::types::Point,
) -> (f64, f64) {
    let cx = center.x.unwrap_or(0) as f64;
    let cy = center.y.unwrap_or(0) as f64;

    let dx_start = start.x.unwrap_or(0) as f64 - cx;
    let dy_start = start.y.unwrap_or(0) as f64 - cy;
    let dx_end = end.x.unwrap_or(0) as f64 - cx;
    let dy_end = end.y.unwrap_or(0) as f64 - cy;

    // atan2로 각도 계산 (라디안 → 도)
    let mut start_angle = dy_start.atan2(dx_start).to_degrees();
    let mut end_angle = dy_end.atan2(dx_end).to_degrees();

    // 음수 각도를 양수로 변환 (0~360 범위)
    if start_angle < 0.0 {
        start_angle += 360.0;
    }
    if end_angle < 0.0 {
        end_angle += 360.0;
    }

    // 시작점과 끝점이 거의 같으면 완전한 원/타원으로 처리
    if (dx_start - dx_end).abs() < 1.0 && (dy_start - dy_end).abs() < 1.0 {
        return (0.0, 360.0);
    }

    (start_angle, end_angle)
}

// =============================================================================
// DrawText → ShapeText 변환
// =============================================================================

/// HWPX DrawText → IR ShapeText 변환
fn convert_draw_text_to_shape_text(draw_text: &crate::paragraph::DrawText) -> IrShapeText {
    // 문단 변환
    let paragraphs: Vec<IrParagraph> = draw_text
        .paragraph_list
        .paragraphs
        .iter()
        .filter_map(|para| convert_paragraph(para).ok())
        .collect();

    // 여백 변환
    let padding = draw_text
        .text_margin
        .as_ref()
        .map(|margin| Insets {
            left: HwpUnit::new(margin.left as i32),
            right: HwpUnit::new(margin.right as i32),
            top: HwpUnit::new(margin.top as i32),
            bottom: HwpUnit::new(margin.bottom as i32),
        })
        .unwrap_or_default();

    // 세로 정렬 변환
    let vertical_alignment =
        convert_paragraph_list_vert_align(&draw_text.paragraph_list.vertical_alignment);

    // 텍스트 방향 변환
    let text_direction = convert_text_direction(&draw_text.paragraph_list.text_direction);

    IrShapeText {
        paragraphs,
        padding,
        vertical_alignment,
        text_direction,
        editable: draw_text.editable,
    }
}

/// HWPX ParagraphVerticalAlignment → IR VerticalAlignment 변환
const fn convert_paragraph_list_vert_align(
    align: &crate::paragraph::ParagraphVerticalAlignment,
) -> VerticalAlignment {
    use crate::paragraph::ParagraphVerticalAlignment;

    match align {
        ParagraphVerticalAlignment::Top => VerticalAlignment::Top,
        ParagraphVerticalAlignment::Center => VerticalAlignment::Middle,
        ParagraphVerticalAlignment::Bottom => VerticalAlignment::Bottom,
    }
}

/// HWPX TextDirection → IR TextDirection 변환
const fn convert_text_direction(
    direction: &crate::paragraph::TextDirection,
) -> primitive::TextDirection {
    use crate::paragraph::TextDirection;
    use primitive::TextDirection as IrTextDir;

    match direction {
        TextDirection::Horizontal => IrTextDir::Horizontal,
        TextDirection::Vertical => IrTextDir::Vertical,
        TextDirection::VerticalAll => IrTextDir::VerticalRightToLeft,
    }
}

/// HWPX Video → IR Video 변환
fn convert_hwpx_video(video: &crate::paragraph::Video) -> Result<IrVideo, ConversionError> {
    use crate::paragraph::video_chart::VideoType;

    // 비디오 종류 변환
    let video_type = match video.video_type {
        VideoType::Local => IrVideoType::Embedded,
        VideoType::Web => IrVideoType::Linked,
    };

    // 비디오 ID (로컬인 경우)
    let video_id = video
        .file_id_ref
        .as_ref()
        .map(|id| BinaryDataId::new(id.0.clone()));

    // 미리보기 이미지 ID
    let preview_image_id = video
        .image_id_ref
        .as_ref()
        .map(|id| BinaryDataId::new(id.0.clone()));

    // ObjectCommon 생성 (Video는 AbstractShapeComponentType을 포함)
    let common = create_object_common_from_hwpx(ObjectCommonParams {
        id: video.id,
        offset_x: video.offset.x,
        offset_y: video.offset.y,
        width: video.current_size.width.unwrap_or(0),
        height: video.current_size.height.unwrap_or(0),
        z_order: video.z_order,
        position: video.position.as_ref(),
        text_wrap_mode: video.text_wrap,
        text_flow_mode: Some(video.text_flow),
        outside_margin: video.outside_margin.as_ref(),
    });

    // 포스터 바이너리 ID (HWPX에서는 imageIDRef와 같음)
    let poster_binary_id = video
        .image_id_ref
        .as_ref()
        .map(|id| BinaryDataId::new(id.0.clone()));

    Ok(IrVideo {
        common,
        video_type,
        video_id,
        source_url: None, // HWPX에서는 웹 비디오 URL을 별도 필드로 저장하지 않음
        preview_image_id,
        poster_binary_id,
        width: video.original_size.width.map(|w| HwpUnit::new(w as i32)),
        height: video.original_size.height.map(|h| HwpUnit::new(h as i32)),
    })
}

/// HWPX Ole → IR OleObject 변환
fn convert_hwpx_ole(ole: &crate::paragraph::Ole) -> Result<IrOleObject, ConversionError> {
    // 바이너리 ID
    let binary_id = ole
        .binary_item_id_ref
        .as_ref()
        .map(|id| BinaryDataId::new(id.0.clone()))
        .unwrap_or_else(|| BinaryDataId::new("0".to_string()));

    // ObjectCommon 생성 (Ole는 AbstractShapeComponentType을 포함)
    let common = create_object_common_from_hwpx(ObjectCommonParams {
        id: ole.id,
        offset_x: ole.offset.x,
        offset_y: ole.offset.y,
        width: ole.current_size.width.unwrap_or(0),
        height: ole.current_size.height.unwrap_or(0),
        z_order: ole.z_order,
        position: ole.position.as_ref(),
        text_wrap_mode: ole.text_wrap,
        text_flow_mode: Some(ole.text_flow),
        outside_margin: ole.outside_margin.as_ref(),
    });

    Ok(IrOleObject {
        common,
        binary_id,
        class_id: None,
        preview_image_id: None,
    })
}

/// HWPX Chart → IR Chart 변환
fn convert_hwpx_chart(chart: &crate::paragraph::Chart) -> Result<IrChart, ConversionError> {
    // 차트 ID
    let chart_id = chart
        .chart_id_ref
        .as_ref()
        .map(|id| id.0.clone())
        .unwrap_or_default();

    // 크기 추출 (size가 있으면 사용)
    let (width, height) = if let Some(sz) = &chart.size {
        (sz.width.unwrap_or(0), sz.height.unwrap_or(0))
    } else {
        (0, 0)
    };

    // ObjectCommon 생성 (Chart는 AbstractShapeObjectType만 - offset/current_size 없음)
    let common = create_object_common_from_hwpx(ObjectCommonParams {
        id: chart.id,
        offset_x: 0, // Chart에는 offset이 없음
        offset_y: 0,
        width,
        height,
        z_order: chart.z_order,
        position: chart.position.as_ref(),
        text_wrap_mode: chart.text_wrap,
        text_flow_mode: Some(chart.text_flow),
        outside_margin: chart.outside_margin.as_ref(),
    });

    Ok(IrChart {
        common,
        chart_id,
        chart_type: IrChartType::Bar, // 기본값
    })
}

/// HWPX RgbColor → IR Color 변환 헬퍼
const fn rgb_color_to_ir(color: &crate::core::types::RgbColor) -> primitive::Color {
    primitive::Color {
        red: color.r,
        green: color.g,
        blue: color.b,
        alpha: color.a,
    }
}

/// FormObject용 ObjectCommon 생성 헬퍼 (AbstractShapeObjectType만 있는 경우)
fn create_form_object_common(
    id: Option<u32>,
    size: Option<&crate::paragraph::shape_common::ShapeObjectSize>,
    position: Option<&crate::paragraph::shape_common::ShapeObjectPosition>,
    z_order: i32,
    text_wrap: Option<crate::paragraph::shape_common::TextWrapMode>,
    text_flow: crate::paragraph::shape_common::TextFlowMode,
) -> ObjectCommon {
    let (width, height) = if let Some(sz) = size {
        (sz.width.unwrap_or(0), sz.height.unwrap_or(0))
    } else {
        (0, 0)
    };

    create_object_common_from_hwpx(ObjectCommonParams {
        id,
        offset_x: 0, // FormObject에는 offset이 없음
        offset_y: 0,
        width,
        height,
        z_order,
        position,
        text_wrap_mode: text_wrap,
        text_flow_mode: Some(text_flow),
        outside_margin: None, // FormObject에는 outside_margin이 없음
    })
}

/// HWPX FormCharacterProperty → IR FormCharProperty 변환
fn convert_hwpx_form_char_property(
    prop: &crate::paragraph::form_control::FormCharacterProperty,
) -> ir::control::FormCharProperty {
    ir::control::FormCharProperty {
        char_shape_id: prop.char_property_id_ref.map(|id| id.0),
        follow_context: prop.follow_context,
        auto_size: prop.auto_size,
        word_wrap: prop.word_wrap,
    }
}

/// HWPX ListItem → IR FormListItem 변환
fn convert_hwpx_list_items(
    items: &[crate::paragraph::form_control::ListItem],
) -> Vec<ir::control::FormListItem> {
    items
        .iter()
        .map(|item| ir::control::FormListItem {
            display_text: item.display_text.clone(),
            value: item.value.clone(),
        })
        .collect()
}

/// HWPX Button → IR FormObject 변환
fn convert_hwpx_button(
    btn: &crate::paragraph::Button,
    form_type: IrFormObjectType,
) -> Result<IrFormObject, ConversionError> {
    use crate::paragraph::form_control::{ButtonBackStyle, ButtonValue};
    use ir::control::{ButtonBackStyle as IrButtonBackStyle, ButtonValue as IrButtonValue};

    let common = create_form_object_common(
        btn.id,
        btn.size.as_ref(),
        btn.position.as_ref(),
        btn.z_order,
        btn.text_wrap,
        btn.text_flow,
    );

    // ButtonValue 변환
    let button_value = btn.value.as_ref().map(|v| match v {
        ButtonValue::Unchecked => IrButtonValue::Unchecked,
        ButtonValue::Checked => IrButtonValue::Checked,
        ButtonValue::Indeterminate => IrButtonValue::Indeterminate,
    });

    // ButtonBackStyle 변환
    let back_style = btn.back_style.as_ref().map(|bs| match bs {
        ButtonBackStyle::Transparent => IrButtonBackStyle::Transparent,
        ButtonBackStyle::Opaque => IrButtonBackStyle::Opaque,
    });

    Ok(IrFormObject {
        common,
        form_type,
        name: btn.name.clone(),
        value: None,
        char_property: convert_hwpx_form_char_property(&btn.form_char_property),
        items: Vec::new(),
        fore_color: btn.fore_color.as_ref().map(rgb_color_to_ir),
        back_color: btn.back_color.as_ref().map(rgb_color_to_ir),
        group_name: btn.group_name.clone(),
        tab_stop: btn.tab_stop,
        enabled: btn.enabled,
        editable: btn.editable,
        border_type_id_ref: btn.border_type_id_ref.as_ref().map(|r| r.0),
        draw_frame: btn.draw_frame,
        printable: btn.printable,
        tab_order: btn.tab_order,
        caption: btn.caption_text.clone(),
        button_value,
        radio_group_name: btn.radio_group_name.clone(),
        back_style,
        tri_state: btn.tri_state,
        gradient_fill: false,
        image_fill: false,
        multiline: false,
        password_char: None,
        max_length: None,
        scroll_bars: None,
        tab_key_behavior: None,
        num_only: false,
        read_only: false,
        alignment: None,
        edit_enable: false,
        selected_value: None,
        list_box_rows: None,
        list_box_width: None,
        item_height: None,
        top_index: None,
        bar_type: None,
        min: None,
        max: None,
        scroll_value: None,
        small_change: None,
        large_change: None,
        page: None,
        delay: None,
    })
}

/// HWPX ComboBox → IR FormObject 변환
fn convert_hwpx_combo_box(
    combo: &crate::paragraph::ComboBox,
) -> Result<IrFormObject, ConversionError> {
    let common = create_form_object_common(
        combo.id,
        combo.size.as_ref(),
        combo.position.as_ref(),
        combo.z_order,
        combo.text_wrap,
        combo.text_flow,
    );

    Ok(IrFormObject {
        common,
        form_type: IrFormObjectType::ComboBox,
        name: combo.name.clone(),
        value: None,
        char_property: convert_hwpx_form_char_property(&combo.form_char_property),
        items: convert_hwpx_list_items(&combo.list_items),
        fore_color: combo.fore_color.as_ref().map(rgb_color_to_ir),
        back_color: combo.back_color.as_ref().map(rgb_color_to_ir),
        group_name: combo.group_name.clone(),
        tab_stop: combo.tab_stop,
        enabled: combo.enabled,
        editable: combo.form_editable,
        border_type_id_ref: combo.border_type_id_ref.as_ref().map(|r| r.0),
        draw_frame: combo.draw_frame,
        printable: combo.printable,
        tab_order: combo.tab_order,
        caption: None,
        button_value: None,
        radio_group_name: None,
        back_style: None,
        tri_state: false,
        gradient_fill: false,
        image_fill: false,
        multiline: false,
        password_char: None,
        max_length: None,
        scroll_bars: None,
        tab_key_behavior: None,
        num_only: false,
        read_only: false,
        alignment: None,
        edit_enable: combo.edit_enable,
        selected_value: combo.selected_value.clone(),
        list_box_rows: combo.list_box_rows,
        list_box_width: combo.list_box_width,
        item_height: None,
        top_index: None,
        bar_type: None,
        min: None,
        max: None,
        scroll_value: None,
        small_change: None,
        large_change: None,
        page: None,
        delay: None,
    })
}

/// HWPX ListBox → IR FormObject 변환
fn convert_hwpx_list_box(
    list: &crate::paragraph::ListBox,
) -> Result<IrFormObject, ConversionError> {
    let common = create_form_object_common(
        list.id,
        list.size.as_ref(),
        list.position.as_ref(),
        list.z_order,
        list.text_wrap,
        list.text_flow,
    );

    Ok(IrFormObject {
        common,
        form_type: IrFormObjectType::ListBox,
        name: list.name.clone(),
        value: None,
        char_property: convert_hwpx_form_char_property(&list.form_char_property),
        items: convert_hwpx_list_items(&list.list_items),
        fore_color: list.fore_color.as_ref().map(rgb_color_to_ir),
        back_color: list.back_color.as_ref().map(rgb_color_to_ir),
        group_name: list.group_name.clone(),
        tab_stop: list.tab_stop,
        enabled: list.enabled,
        editable: list.editable,
        border_type_id_ref: list.border_type_id_ref.as_ref().map(|r| r.0),
        draw_frame: list.draw_frame,
        printable: list.printable,
        tab_order: list.tab_order,
        caption: None,
        button_value: None,
        radio_group_name: None,
        back_style: None,
        tri_state: false,
        gradient_fill: false,
        image_fill: false,
        multiline: false,
        password_char: None,
        max_length: None,
        scroll_bars: None,
        tab_key_behavior: None,
        num_only: false,
        read_only: false,
        alignment: None,
        edit_enable: false,
        selected_value: list.selected_value.clone(),
        list_box_rows: None,
        list_box_width: None,
        item_height: list.item_height,
        top_index: list.top_index,
        bar_type: None,
        min: None,
        max: None,
        scroll_value: None,
        small_change: None,
        large_change: None,
        page: None,
        delay: None,
    })
}

/// HWPX Edit → IR FormObject 변환
fn convert_hwpx_edit(edit: &crate::paragraph::Edit) -> Result<IrFormObject, ConversionError> {
    let common = create_form_object_common(
        edit.id,
        edit.size.as_ref(),
        edit.position.as_ref(),
        edit.z_order,
        edit.text_wrap,
        edit.text_flow,
    );

    Ok(IrFormObject {
        common,
        form_type: IrFormObjectType::Edit,
        name: edit.name.clone(),
        value: None,
        char_property: convert_hwpx_form_char_property(&edit.form_char_property),
        items: Vec::new(),
        fore_color: edit.fore_color.as_ref().map(rgb_color_to_ir),
        back_color: edit.back_color.as_ref().map(rgb_color_to_ir),
        group_name: edit.group_name.clone(),
        tab_stop: edit.tab_stop,
        enabled: edit.enabled,
        editable: true,
        border_type_id_ref: edit.border_type_id_ref.as_ref().map(|r| r.0),
        draw_frame: edit.draw_frame,
        printable: edit.printable,
        tab_order: None,
        caption: None,
        button_value: None,
        radio_group_name: None,
        back_style: None,
        tri_state: false,
        gradient_fill: false,
        image_fill: false,
        multiline: false,
        password_char: None,
        max_length: None,
        scroll_bars: None,
        tab_key_behavior: None,
        num_only: false,
        read_only: false,
        alignment: None,
        edit_enable: false,
        selected_value: None,
        list_box_rows: None,
        list_box_width: None,
        item_height: None,
        top_index: None,
        bar_type: None,
        min: None,
        max: None,
        scroll_value: None,
        small_change: None,
        large_change: None,
        page: None,
        delay: None,
    })
}

/// HWPX ScrollBar → IR FormObject 변환
fn convert_hwpx_scroll_bar(
    scroll: &crate::paragraph::ScrollBar,
) -> Result<IrFormObject, ConversionError> {
    // i32 value를 문자열로 변환
    let value_str = scroll.value.map(|v| v.to_string());

    let common = create_form_object_common(
        scroll.id,
        scroll.size.as_ref(),
        scroll.position.as_ref(),
        scroll.z_order,
        scroll.text_wrap,
        scroll.text_flow,
    );

    // bar_type 변환 (HWPX ScrollBar 전용)
    let bar_type = scroll.bar_type.as_ref().map(|bt| match bt {
        crate::paragraph::form_control::ScrollBarType::Horizontal => {
            ir::control::ScrollBarType::Horizontal
        }
        crate::paragraph::form_control::ScrollBarType::Vertical => {
            ir::control::ScrollBarType::Vertical
        }
    });

    Ok(IrFormObject {
        common,
        form_type: IrFormObjectType::ScrollBar,
        name: scroll.name.clone(),
        value: value_str,
        char_property: convert_hwpx_form_char_property(&scroll.form_char_property),
        items: Vec::new(),
        fore_color: scroll.fore_color.as_ref().map(rgb_color_to_ir),
        back_color: scroll.back_color.as_ref().map(rgb_color_to_ir),
        group_name: scroll.group_name.clone(),
        tab_stop: scroll.tab_stop,
        enabled: scroll.enabled,
        editable: true,
        border_type_id_ref: scroll.border_type_id_ref.as_ref().map(|r| r.0),
        draw_frame: scroll.draw_frame,
        printable: scroll.printable,
        tab_order: None,
        caption: None,
        button_value: None,
        radio_group_name: None,
        back_style: None,
        tri_state: false,
        gradient_fill: false,
        image_fill: false,
        multiline: false,
        password_char: None,
        max_length: None,
        scroll_bars: None,
        tab_key_behavior: None,
        num_only: false,
        read_only: false,
        alignment: None,
        edit_enable: false,
        selected_value: None,
        list_box_rows: None,
        list_box_width: None,
        item_height: None,
        top_index: None,
        bar_type,
        min: scroll.min,
        max: scroll.max,
        scroll_value: scroll.value,
        small_change: scroll.small_change,
        large_change: scroll.large_change,
        page: scroll.page,
        delay: scroll.delay,
    })
}

/// HWPX ConnectLine → IR Shape 변환
fn convert_hwpx_connect_line(
    connect_line: &crate::paragraph::ConnectLine,
) -> Result<IrShape, ConversionError> {
    use crate::paragraph::shape_common::ConnectLineStyle;
    use ir::shape::{ConnectorShape, ConnectorType, ShapeType};

    // 연결선 종류 변환 (스타일 이름에서 추론)
    let connector_type = match connect_line.line_type {
        Some(ConnectLineStyle::StraightNoArrow)
        | Some(ConnectLineStyle::StraightOneWay)
        | Some(ConnectLineStyle::StraightBoth) => ConnectorType::Straight,
        Some(ConnectLineStyle::StrokeNoArrow)
        | Some(ConnectLineStyle::StrokeOneWay)
        | Some(ConnectLineStyle::StrokeBoth) => ConnectorType::Elbow,
        Some(ConnectLineStyle::ArcNoArrow)
        | Some(ConnectLineStyle::ArcOneWay)
        | Some(ConnectLineStyle::ArcBoth) => ConnectorType::Curved,
        None => ConnectorType::Straight,
    };

    // 시작점/끝점 좌표 (subject 참조 포함)
    let start = ir::shape::ConnectorPoint {
        point: primitive::Point {
            x: HwpUnit::new(connect_line.start_point.x.unwrap_or(0)),
            y: HwpUnit::new(connect_line.start_point.y.unwrap_or(0)),
        },
        subject_id_ref: connect_line.start_point.subject_id_reference.map(|r| r.0),
        subject_index: connect_line.start_point.subject_index,
    };
    let end = ir::shape::ConnectorPoint {
        point: primitive::Point {
            x: HwpUnit::new(connect_line.end_point.x.unwrap_or(0)),
            y: HwpUnit::new(connect_line.end_point.y.unwrap_or(0)),
        },
        subject_id_ref: connect_line.end_point.subject_id_reference.map(|r| r.0),
        subject_index: connect_line.end_point.subject_index,
    };

    // 제어점 변환 (point_type 포함)
    let control_points = connect_line
        .control_points
        .as_ref()
        .map(|cp| {
            cp.points
                .iter()
                .map(|pt| {
                    use ir::shape::CurvePoint;

                    // point_type 변환: 0=Normal, 1=Control1, 2=Control2
                    let point_type = match pt.point_type.unwrap_or(0) {
                        0 => ir::shape::CurvePointType::Normal,
                        1 => ir::shape::CurvePointType::Control1,
                        2 => ir::shape::CurvePointType::Control2,
                        _ => ir::shape::CurvePointType::Normal,
                    };

                    CurvePoint {
                        point: primitive::Point {
                            x: HwpUnit::new(pt.x.unwrap_or(0)),
                            y: HwpUnit::new(pt.y.unwrap_or(0)),
                        },
                        point_type,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    // 화살표 변환 (LineShape의 head_style, tail_style, head_size, tail_size 사용)
    let start_arrow = convert_arrow_style(
        &connect_line.line_shape.head_style,
        connect_line.line_shape.head_fill,
        &connect_line.line_shape.head_size,
    );
    let end_arrow = convert_arrow_style(
        &connect_line.line_shape.tail_style,
        connect_line.line_shape.tail_fill,
        &connect_line.line_shape.tail_size,
    );

    let connector = ConnectorShape {
        connector_type,
        start,
        end,
        start_arrow,
        end_arrow,
        control_points,
    };

    // 선 스타일 변환
    let line = convert_hwpx_drawing_line_shape(&connect_line.line_shape);

    // 채우기 변환 (연결선은 대부분 채우기 없음)
    let fill = convert_fill_brush(connect_line.fill_brush.as_ref());

    // 공통 속성
    let common = create_object_common_from_hwpx(ObjectCommonParams {
        id: connect_line.id,
        offset_x: connect_line.offset.x,
        offset_y: connect_line.offset.y,
        width: connect_line.current_size.width.unwrap_or(0),
        height: connect_line.current_size.height.unwrap_or(0),
        z_order: connect_line.z_order,
        position: connect_line.position.as_ref(),
        text_wrap_mode: connect_line.text_wrap,
        text_flow_mode: Some(connect_line.text_flow),
        outside_margin: None, // 연결선에는 outside_margin 없음
    });

    // 회전 각도 (HWPX는 0.1도 단위)
    let rotation = connect_line.rotation_info.angle as f64 / 10.0;

    // 텍스트
    let text = connect_line
        .draw_text
        .as_ref()
        .map(convert_draw_text_to_shape_text);

    // 그림자
    let shadow = connect_line.shadow.as_ref().map(convert_hwpx_shape_shadow);

    Ok(IrShape {
        translation_matrix: None,
        scale_matrix: None,
        rotation_matrix: None,
        common,
        shape_type: ShapeType::Connector(connector),
        line,
        fill,
        shadow,
        rotation,
        text,
    })
}

/// HWPX ArrowStyle → IR Arrow 변환
fn convert_arrow_style(
    style: &crate::core::enums::ArrowStyle,
    filled: bool,
    hwpx_size: &crate::core::enums::ArrowSize,
) -> ir::shape::Arrow {
    use crate::core::enums::ArrowStyle;
    use primitive::ArrowType;

    let arrow_type = match style {
        ArrowStyle::Normal => ArrowType::None,
        ArrowStyle::Arrow => ArrowType::Arrow,
        ArrowStyle::Spear => ArrowType::Stealth,
        ArrowStyle::ConcaveArrow => ArrowType::ArrowOpen,
        ArrowStyle::EmptyDiamond | ArrowStyle::FilledDiamond => ArrowType::Diamond,
        ArrowStyle::EmptyCircle | ArrowStyle::FilledCircle => ArrowType::Circle,
        ArrowStyle::EmptyBox | ArrowStyle::FilledBox => ArrowType::Square,
    };

    let size = convert_hwpx_arrow_size(hwpx_size);

    ir::shape::Arrow {
        arrow_type,
        size,
        filled,
    }
}

/// HWPX ArrowSize → IR ArrowSize 변환
const fn convert_hwpx_arrow_size(
    hwpx_size: &crate::core::enums::ArrowSize,
) -> primitive::ArrowSize {
    use crate::core::enums::ArrowSize as HwpxArrowSize;
    use primitive::ArrowSize;

    // HWPX는 WIDTH_HEIGHT 형식 (예: SmallMedium = 폭 Small, 높이 Medium)
    // IR은 단일 크기만 지원하므로 높이(두 번째) 값을 우선 사용
    match hwpx_size {
        HwpxArrowSize::SmallSmall | HwpxArrowSize::MediumSmall | HwpxArrowSize::LargeSmall => {
            ArrowSize::Small
        }
        HwpxArrowSize::SmallMedium | HwpxArrowSize::MediumMedium | HwpxArrowSize::LargeMedium => {
            ArrowSize::Medium
        }
        HwpxArrowSize::SmallLarge | HwpxArrowSize::MediumLarge | HwpxArrowSize::LargeLarge => {
            ArrowSize::Large
        }
    }
}

/// HWPX LineShape (drawing) → IR LineStyle 변환
fn convert_hwpx_drawing_line_shape(
    line_shape: &crate::paragraph::line_shape::LineShape,
) -> ir::shape::LineStyle {
    use primitive::{LineCap, LineOutlineStyle};

    let line_type = convert_hwpx_line_style(&line_shape.style);
    let width = HwpUnit::new(line_shape.width.unwrap_or(10) as i32);
    let color = line_shape
        .color
        .as_ref()
        .map(|c| Color::rgb(c.r, c.g, c.b))
        .unwrap_or(Color::BLACK);

    let cap = match line_shape.end_cap {
        crate::paragraph::line_shape::LineEndCapStyle::Flat => LineCap::Flat,
        crate::paragraph::line_shape::LineEndCapStyle::Round => LineCap::Round,
    };

    let outline_style = match line_shape.outline_style {
        crate::paragraph::line_shape::OutlineStyle::Normal => LineOutlineStyle::Normal,
        crate::paragraph::line_shape::OutlineStyle::Outer => LineOutlineStyle::Outer,
        crate::paragraph::line_shape::OutlineStyle::Inner => LineOutlineStyle::Inner,
    };

    ir::shape::LineStyle {
        line_type,
        width,
        color,
        cap,
        outline_style,
        alpha: line_shape.alpha,
    }
}

/// HWPX TextArt → IR TextArt 변환
fn convert_hwpx_text_art(
    text_art: &crate::paragraph::TextArt,
) -> Result<ir::control::TextArt, ConversionError> {
    use crate::paragraph::text_art::{
        TextArtAlignment as HwpxAlignment, TextArtShape as HwpxShape,
    };
    use ir::control::{TextArt as IrTextArt, TextArtAlignment, TextArtFontStyle, TextArtShapeType};

    // 텍스트
    let text = text_art.text.clone().unwrap_or_default();

    // 글꼴 이름
    let font_name = text_art.properties.font_name.clone();

    // 글꼴 스타일 변환
    let font_style = match text_art.properties.font_style.as_str() {
        "BOLD" => TextArtFontStyle::Bold,
        "ITALIC" => TextArtFontStyle::Italic,
        "BOLD_ITALIC" => TextArtFontStyle::BoldItalic,
        _ => TextArtFontStyle::Regular,
    };

    // 글맵시 모양 변환
    let shape = text_art
        .properties
        .text_shape
        .as_ref()
        .map(|s| match s {
            HwpxShape::Rectangle => TextArtShapeType::Rectangle,
            HwpxShape::Circle
            | HwpxShape::SingleLineCircle1
            | HwpxShape::SingleLineCircle2
            | HwpxShape::DoubleLineCircle
            | HwpxShape::TripleLineCircle1
            | HwpxShape::TripleLineCircle2 => TextArtShapeType::Circle,
            HwpxShape::ArchUp => TextArtShapeType::ArchUp,
            HwpxShape::ArchDown => TextArtShapeType::ArchDown,
            HwpxShape::Wave1 | HwpxShape::Wave2 | HwpxShape::Wave3 | HwpxShape::Wave4 => {
                TextArtShapeType::Wave
            }
            HwpxShape::Cylinder
            | HwpxShape::LeftCylinder
            | HwpxShape::RightCylinder
            | HwpxShape::LeftTiltCylinder
            | HwpxShape::RightTiltCylinder
            | HwpxShape::BottomWideCylinder
            | HwpxShape::TopWideCylinder => TextArtShapeType::Cylinder,
            HwpxShape::Inflate
            | HwpxShape::InflateTop
            | HwpxShape::InflateBottom
            | HwpxShape::InflateRight
            | HwpxShape::InflateLeft
            | HwpxShape::InflateUpConvex
            | HwpxShape::InflateBottomConvex => TextArtShapeType::Inflate,
            HwpxShape::Deflate | HwpxShape::DeflateTop | HwpxShape::DeflateBottom => {
                TextArtShapeType::Deflate
            }
            _ => TextArtShapeType::Rectangle,
        })
        .unwrap_or_default();

    // 정렬 변환
    let alignment = match text_art.properties.alignment {
        HwpxAlignment::Left => TextArtAlignment::Left,
        HwpxAlignment::Center => TextArtAlignment::Center,
        HwpxAlignment::Right => TextArtAlignment::Right,
        HwpxAlignment::Full | HwpxAlignment::Table => TextArtAlignment::Full,
    };

    // 선 스타일 변환
    let line = convert_hwpx_drawing_line_shape(&text_art.line_shape);

    // 채우기 변환
    let fill = convert_fill_brush(text_art.fill_brush.as_ref());

    // 그림자 변환
    let shadow = text_art
        .shape_shadow
        .as_ref()
        .map(convert_hwpx_shape_shadow);

    // 공통 속성
    let common = create_object_common_from_hwpx(ObjectCommonParams {
        id: text_art.id,
        offset_x: text_art.offset.x,
        offset_y: text_art.offset.y,
        width: text_art.current_size.width.unwrap_or(0),
        height: text_art.current_size.height.unwrap_or(0),
        z_order: text_art.z_order,
        position: text_art.position.as_ref(),
        text_wrap_mode: text_art.text_wrap,
        text_flow_mode: Some(text_art.text_flow),
        outside_margin: None, // TextArt에는 outside_margin 없음
    });

    // font_type 변환 (HWPX TextArt 전용)
    let font_type = Some(match text_art.properties.font_type {
        crate::paragraph::text_art::TextArtFontType::Ttf => ir::control::TextArtFontType::TTF,
        crate::paragraph::text_art::TextArtFontType::Htf => ir::control::TextArtFontType::HTF,
    });

    // text_art_pr (HWPX 추가 속성) - 향후 확장 가능
    let text_art_pr = None; // TODO: HWPX text_art_pr 속성 파싱 필요

    Ok(IrTextArt {
        common,
        text,
        font_name,
        font_style,
        font_type,
        shape,
        line_spacing: text_art.properties.line_spacing,
        char_spacing: text_art.properties.char_spacing,
        alignment,
        line,
        fill,
        shadow,
        text_art_pr,
    })
}

/// HWPX ShapeShadow → IR ShapeShadow 변환
fn convert_hwpx_shape_shadow(shadow: &crate::paragraph::ShapeShadow) -> ir::shape::ShapeShadow {
    let color = Color::rgb(shadow.color.r, shadow.color.g, shadow.color.b);

    ir::shape::ShapeShadow {
        color,
        offset_x: HwpUnit::new(shadow.offset_x.unwrap_or(0)),
        offset_y: HwpUnit::new(shadow.offset_y.unwrap_or(0)),
        // HWPX ShapeShadow.alpha는 0~1 범위 (f32)
        alpha: shadow.alpha.map(|a| a as f64).unwrap_or(0.5),
        blur: None, // ShapeShadow는 단순 그림자, blur는 AdvancedShadowEffect에 있음
        direction: None,
        distance: None,
    }
}

/// HWPX Container → IR Shape (Group) 변환
fn convert_hwpx_container(
    container: &crate::paragraph::ole_equation::Container,
) -> Result<IrShape, ConversionError> {
    use ir::shape::ShapeType;

    // 자식 도형들을 재귀적으로 변환
    let mut child_shapes = Vec::new();
    for child in &container.children {
        if let Some(shape) = convert_container_child_to_shape(child)? {
            child_shapes.push(shape);
        }
    }

    // 공통 속성
    let common = create_object_common_from_hwpx(ObjectCommonParams {
        id: container.id,
        offset_x: container.offset.x,
        offset_y: container.offset.y,
        width: container.current_size.width.unwrap_or(0),
        height: container.current_size.height.unwrap_or(0),
        z_order: container.z_order,
        position: container.position.as_ref(),
        text_wrap_mode: container.text_wrap,
        text_flow_mode: Some(container.text_flow),
        outside_margin: None, // Container에는 outside_margin 없음
    });

    // 회전 각도 (HWPX는 0.1도 단위)
    let rotation = container.rotation_info.angle as f64 / 10.0;

    Ok(IrShape {
        translation_matrix: None,
        scale_matrix: None,
        rotation_matrix: None,
        common,
        shape_type: ShapeType::Group(child_shapes),
        line: ir::shape::LineStyle::default(),
        fill: ir::border_fill::Fill::None,
        shadow: None,
        rotation,
        text: None,
    })
}

/// ContainerChild → IR Shape 변환
fn convert_container_child_to_shape(
    child: &crate::paragraph::ole_equation::ContainerChild,
) -> Result<Option<IrShape>, ConversionError> {
    use crate::paragraph::ole_equation::ContainerChild;

    match child {
        ContainerChild::Container(container) => {
            // 중첩 컨테이너 재귀 변환
            Ok(Some(convert_hwpx_container(container)?))
        }
        ContainerChild::Line(line) => Ok(Some(convert_hwpx_line(line)?)),
        ContainerChild::Rectangle(rect) => Ok(Some(convert_hwpx_rectangle(rect)?)),
        ContainerChild::Ellipse(ellipse) => Ok(Some(convert_hwpx_ellipse(ellipse)?)),
        ContainerChild::Arc(arc) => Ok(Some(convert_hwpx_arc(arc)?)),
        ContainerChild::Polygon(polygon) => Ok(Some(convert_hwpx_polygon(polygon)?)),
        ContainerChild::Curve(curve) => Ok(Some(convert_hwpx_curve(curve)?)),
        ContainerChild::ConnectLine(connect_line) => {
            Ok(Some(convert_hwpx_connect_line(connect_line)?))
        }
        // Picture, Ole 등은 Shape이 아니므로 생략
        _ => Ok(None),
    }
}

/// HWPX FieldBegin → IR FieldStart 변환
fn convert_hwpx_field_to_field_start(field: &crate::paragraph::FieldBegin) -> IrFieldStart {
    use crate::paragraph::FieldType as HwpxFieldType;

    let field_type = match field.field_type {
        HwpxFieldType::Date | HwpxFieldType::DocumentDate => IrFieldType::Date,
        HwpxFieldType::Path => IrFieldType::FilePath,
        HwpxFieldType::Summary => IrFieldType::Summary,
        HwpxFieldType::UserInfo => IrFieldType::UserInfo,
        HwpxFieldType::CrossReference => IrFieldType::CrossReference,
        HwpxFieldType::MailMerge => IrFieldType::MailMerge,
        HwpxFieldType::Bookmark => IrFieldType::Bookmark,
        HwpxFieldType::Hyperlink => IrFieldType::Hyperlink,
        HwpxFieldType::Formula => IrFieldType::Formula,
        HwpxFieldType::ClickHere => IrFieldType::ClickHere,
        HwpxFieldType::Memo => IrFieldType::Memo,
        HwpxFieldType::ProofreadingMarks => IrFieldType::Unknown,
        HwpxFieldType::PrivateInfo => IrFieldType::PrivateInfo,
        HwpxFieldType::MetaTag => IrFieldType::MetaTag,
    };

    // instruction 추출 - parameters에서 첫 번째 문자열 항목 또는 name 사용
    let instruction = if let Some(ref params) = field.parameters {
        params.items.first().and_then(|item| {
            if let crate::paragraph::ParameterItem::String(s) = item {
                Some(s.value.clone())
            } else {
                None
            }
        })
    } else {
        field.name.clone()
    };

    // HWPX ParameterList → IR FieldParameters 변환
    let parameters = field
        .parameters
        .as_ref()
        .and_then(convert_hwpx_parameter_list_to_ir);

    // HWPX subList → IR sub_paragraphs 변환
    let sub_paragraphs = field.sub_list.as_ref().and_then(|sub_list| {
        let mut ir_paragraphs = Vec::new();
        for hwpx_para in &sub_list.paragraphs {
            if let Ok(ir_para) = convert_paragraph(hwpx_para) {
                ir_paragraphs.push(ir_para);
            }
        }
        if ir_paragraphs.is_empty() {
            None
        } else {
            Some(ir_paragraphs)
        }
    });

    IrFieldStart {
        id: field.id,
        field_type,
        instruction,
        parameters,
        sub_paragraphs,
        editable: field.editable,
        dirty: field.dirty,
        z_order: field.z_order,
        field_id: field.field_id,
    }
}

/// HWPX ParameterList → IR FieldParameters 변환
fn convert_hwpx_parameter_list_to_ir(
    params: &crate::paragraph::ParameterList,
) -> Option<IrFieldParameters> {
    let mut ir_items = Vec::new();

    for item in &params.items {
        if let Some(ir_param) = convert_hwpx_parameter_item_to_ir(item) {
            ir_items.push(ir_param);
        }
    }

    if ir_items.is_empty() {
        return None;
    }

    Some(IrFieldParameters {
        items: ir_items,
        name: params.name.clone(),
    })
}

/// HWPX ParameterItem → IR FieldParameter 변환
fn convert_hwpx_parameter_item_to_ir(
    item: &crate::paragraph::ParameterItem,
) -> Option<IrFieldParameter> {
    match item {
        crate::paragraph::ParameterItem::Boolean(b) => Some(IrFieldParameter::Boolean {
            name: b.name.clone(),
            value: b.value,
        }),
        crate::paragraph::ParameterItem::Integer(i) => Some(IrFieldParameter::Integer {
            name: i.name.clone(),
            value: i.value,
        }),
        crate::paragraph::ParameterItem::Float(f) => Some(IrFieldParameter::Float {
            name: f.name.clone(),
            value: f.value,
        }),
        crate::paragraph::ParameterItem::String(s) => Some(IrFieldParameter::String {
            name: s.name.clone(),
            value: s.value.clone(),
        }),
        crate::paragraph::ParameterItem::List(list) => {
            // 재귀적으로 중첩된 리스트 변환
            convert_hwpx_parameter_list_to_ir(list).map(IrFieldParameter::List)
        }
    }
}

// =============================================================================
// ObjectCommon / TextWrap 변환 (HWPX → IR)
// =============================================================================

/// HWPX ShapeObjectPosition → IR TextWrap 변환
fn convert_hwpx_position_to_text_wrap(
    position: Option<&crate::paragraph::shape_common::ShapeObjectPosition>,
    text_wrap_mode: Option<crate::paragraph::shape_common::TextWrapMode>,
    text_flow_mode: Option<crate::paragraph::shape_common::TextFlowMode>,
    outside_margin: Option<&crate::paragraph::shape_common::OutsideMargin>,
) -> IrTextWrap {
    use crate::paragraph::shape_common::{
        HorizontalRelativeTo as HwpxHorzRelTo, TextFlowMode, TextWrapMode,
        VerticalRelativeTo as HwpxVertRelTo,
    };

    // text_wrap_mode 변환
    let (wrap_type, treat_as_char) = if let Some(mode) = text_wrap_mode {
        let wt = match mode {
            TextWrapMode::Square => IrTextWrapType::Square,
            TextWrapMode::Tight => IrTextWrapType::Tight,
            TextWrapMode::Through => IrTextWrapType::Tight, // Through도 Tight로
            TextWrapMode::TopAndBottom => IrTextWrapType::Square,
            TextWrapMode::BehindText => IrTextWrapType::Behind,
            TextWrapMode::InFrontOfText => IrTextWrapType::InFront,
        };
        (wt, false)
    } else if let Some(pos) = position {
        if pos.treat_as_character {
            (IrTextWrapType::Inline, true)
        } else {
            (IrTextWrapType::Square, false)
        }
    } else {
        (IrTextWrapType::Square, false)
    };

    // text_flow_mode 변환
    let wrap_side = if let Some(flow) = text_flow_mode {
        match flow {
            TextFlowMode::BothSides => IrTextWrapSide::Both,
            TextFlowMode::LeftOnly => IrTextWrapSide::Left,
            TextFlowMode::RightOnly => IrTextWrapSide::Right,
            TextFlowMode::LargestOnly => IrTextWrapSide::Largest,
        }
    } else {
        IrTextWrapSide::Both
    };

    // position에서 나머지 속성 추출
    let (vertical_rel, horizontal_rel, flow_with_text, allow_overlap) = if let Some(pos) = position
    {
        let vert = pos
            .vertical_relative_to
            .map(|v| match v {
                HwpxVertRelTo::Paper => IrVerticalRelativeTo::Paper,
                HwpxVertRelTo::Page => IrVerticalRelativeTo::Page,
                HwpxVertRelTo::Paragraph => IrVerticalRelativeTo::Paragraph,
            })
            .unwrap_or(IrVerticalRelativeTo::Paper);

        let horz = pos
            .horizontal_relative_to
            .map(|h| match h {
                HwpxHorzRelTo::Paper => IrHorizontalRelativeTo::Paper,
                HwpxHorzRelTo::Page => IrHorizontalRelativeTo::Page,
                HwpxHorzRelTo::Column => IrHorizontalRelativeTo::Column,
                HwpxHorzRelTo::Paragraph => IrHorizontalRelativeTo::Paragraph,
            })
            .unwrap_or(IrHorizontalRelativeTo::Paper);

        (vert, horz, pos.flow_with_text, pos.allow_overlap)
    } else {
        (
            IrVerticalRelativeTo::Paper,
            IrHorizontalRelativeTo::Paper,
            false,
            false,
        )
    };

    // 바깥 여백 계산 (4방향 평균)
    let margin = if let Some(m) = outside_margin {
        let avg = (m.left as i32 + m.right as i32 + m.top as i32 + m.bottom as i32) / 4;
        HwpUnit::new(avg)
    } else {
        HwpUnit::new(0)
    };

    IrTextWrap {
        wrap_type,
        wrap_side,
        margin,
        vertical_rel,
        horizontal_rel,
        vertical_offset_type: Default::default(), // TODO: HWPX에서 추출 필요
        horizontal_offset_type: Default::default(), // TODO: HWPX에서 추출 필요
        treat_as_char,
        flow_with_text,
        allow_overlap,
    }
}

/// HWPX ObjectCommon 생성용 파라미터
struct ObjectCommonParams<'a> {
    id: Option<u32>,
    offset_x: u32,
    offset_y: u32,
    width: u32,
    height: u32,
    z_order: i32,
    position: Option<&'a crate::paragraph::shape_common::ShapeObjectPosition>,
    text_wrap_mode: Option<crate::paragraph::shape_common::TextWrapMode>,
    text_flow_mode: Option<crate::paragraph::shape_common::TextFlowMode>,
    outside_margin: Option<&'a crate::paragraph::shape_common::OutsideMargin>,
}

/// HWPX 개체에서 ObjectCommon 생성 (공통 헬퍼)
fn create_object_common_from_hwpx(params: ObjectCommonParams<'_>) -> ObjectCommon {
    ObjectCommon {
        id: params.id,
        position: IrPoint {
            x: HwpUnit::new(params.offset_x as i32),
            y: HwpUnit::new(params.offset_y as i32),
        },
        size: Size {
            width: HwpUnit::new(params.width as i32),
            height: HwpUnit::new(params.height as i32),
        },
        z_order: params.z_order,
        text_wrap: convert_hwpx_position_to_text_wrap(
            params.position,
            params.text_wrap_mode,
            params.text_flow_mode,
            params.outside_margin,
        ),
        caption: None, // Caption은 별도 처리 - convert_hwpx_caption 함수 사용
        numbering_type: None,
        shape_comment: None,
        meta_tag: None,
        dirty: false,
        width_relative_to: Default::default(), // TODO: HWPX에서 추출 필요
        height_relative_to: Default::default(), // TODO: HWPX에서 추출 필요
        margin: Default::default(),            // TODO: HWPX outside_margin에서 변환 필요
    }
}

/// HWPX 도형 ObjectCommon 생성용 파라미터
struct DrawingObjectCommonParams<'a> {
    size: Option<&'a crate::paragraph::shape_common::ShapeObjectSize>,
    position: Option<&'a crate::paragraph::shape_common::ShapeObjectPosition>,
    offset: &'a crate::paragraph::shape_common::ShapeComponentOffset,
    current_size: &'a crate::paragraph::shape_common::CurrentSize,
    z_order: i32,
    text_wrap_mode: Option<crate::paragraph::shape_common::TextWrapMode>,
    text_flow_mode: Option<crate::paragraph::shape_common::TextFlowMode>,
    outside_margin: Option<&'a crate::paragraph::shape_common::OutsideMargin>,
}

/// HWPX 도형 객체에서 ObjectCommon 생성 (그리기 개체용)
fn create_drawing_object_common(params: DrawingObjectCommonParams<'_>) -> ObjectCommon {
    // 크기: size가 있으면 사용, 없으면 current_size 사용
    let (width, height) = if let Some(sz) = params.size {
        (sz.width.unwrap_or(0), sz.height.unwrap_or(0))
    } else {
        (
            params.current_size.width.unwrap_or(0),
            params.current_size.height.unwrap_or(0),
        )
    };

    // 위치: offset 사용 (도형 컴포넌트의 상대 위치)
    let offset_x = params.offset.x as i32;
    let offset_y = params.offset.y as i32;

    ObjectCommon {
        id: None, // 그리기 개체에는 보통 별도 ID가 없음
        position: IrPoint {
            x: HwpUnit::new(offset_x),
            y: HwpUnit::new(offset_y),
        },
        size: Size {
            width: HwpUnit::new(width as i32),
            height: HwpUnit::new(height as i32),
        },
        z_order: params.z_order,
        text_wrap: convert_hwpx_position_to_text_wrap(
            params.position,
            params.text_wrap_mode,
            params.text_flow_mode,
            params.outside_margin,
        ),
        caption: None,
        numbering_type: None,
        shape_comment: None,
        meta_tag: None,
        dirty: false,
        width_relative_to: Default::default(), // TODO: HWPX에서 추출 필요
        height_relative_to: Default::default(), // TODO: HWPX에서 추출 필요
        margin: Default::default(),            // TODO: HWPX outside_margin에서 변환 필요
    }
}

/// HWPX ShapeNumberingType → IR ObjectNumberingType 변환
const fn convert_numbering_type(
    hwpx_type: crate::paragraph::shape_common::ShapeNumberingType,
) -> Option<IrObjectNumberingType> {
    use crate::paragraph::shape_common::ShapeNumberingType;

    match hwpx_type {
        ShapeNumberingType::None => None,
        ShapeNumberingType::Picture => Some(IrObjectNumberingType::Picture),
        ShapeNumberingType::Table => Some(IrObjectNumberingType::Table),
        ShapeNumberingType::Equation => Some(IrObjectNumberingType::Equation),
    }
}

/// HWPX MetaTag를 String으로 변환
fn convert_meta_tag(meta_tag: &Option<crate::core::types::MetaTag>) -> Option<String> {
    meta_tag.as_ref().map(|mt| mt.content.clone())
}

/// HWPX 캡션 변환
fn convert_hwpx_caption(
    caption: &crate::paragraph::table::Caption,
) -> Result<IrCaption, ConversionError> {
    use crate::paragraph::shape_common::CaptionSide;

    // 캡션 위치 변환
    let position = match caption.side {
        CaptionSide::Left => IrCaptionPosition::Left,
        CaptionSide::Right => IrCaptionPosition::Right,
        CaptionSide::Top => IrCaptionPosition::Top,
        CaptionSide::Bottom => IrCaptionPosition::Bottom,
    };

    // 캡션 내용 (문단들) 변환
    let paragraphs: Vec<IrParagraph> = caption
        .paragraph_list
        .paragraphs
        .iter()
        .filter_map(|para| convert_paragraph(para).ok())
        .collect();

    Ok(IrCaption {
        position,
        width: HwpUnit::new(caption.width.unwrap_or(0)),
        gap: HwpUnit::new(caption.gap),
        paragraphs,
    })
}

/// 형광펜 범위 태그 추출
///
/// HWPX의 MarkPenBegin/MarkPenEnd를 IR의 RangeTag로 변환합니다.
fn extract_highlight_tags(para: &crate::paragraph::Paragraph, ir_para: &mut IrParagraph) {
    use crate::paragraph::{RunContent, TextMarkup};

    let mut highlight_stack: Vec<(u32, Option<String>)> = Vec::new(); // (start_pos, color)
    let mut current_pos = 0u32;

    // 각 런을 순회하며 MarkPen 태그와 텍스트 위치 추적
    for run in &para.runs {
        for content in &run.contents {
            if let RunContent::Text(text_elem) = content {
                for markup in &text_elem.contents {
                    match markup {
                        TextMarkup::MarkPenBegin(markpen) => {
                            // 형광펜 시작: 현재 위치와 색상을 스택에 저장
                            let color = markpen
                                .color
                                .as_ref()
                                .map(|c| format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b));
                            highlight_stack.push((current_pos, color));
                        }
                        TextMarkup::MarkPenEnd(_) => {
                            // 형광펜 끝: 스택에서 시작 위치를 꺼내 RangeTag 생성
                            if let Some((start_pos, color)) = highlight_stack.pop() {
                                let tag = IrRangeTag {
                                    start: start_pos,
                                    end: current_pos,
                                    tag_type: IrRangeTagType::Highlight,
                                    data: color,
                                    track_change_info: None,
                                };
                                ir_para.range_tags.push(tag);
                            }
                        }
                        TextMarkup::Text(text) => {
                            // 텍스트: 위치 증가
                            current_pos += text.chars().count() as u32;
                        }
                        TextMarkup::Tab(_)
                        | TextMarkup::LineBreak(_)
                        | TextMarkup::NonBreakingSpace(_)
                        | TextMarkup::FixedWidthSpace(_)
                        | TextMarkup::Hyphen(_) => {
                            // 특수 문자: 위치 1 증가
                            current_pos += 1;
                        }
                        _ => {
                            // 기타 마크업은 무시
                        }
                    }
                }
            } else {
                // Control (Table, Picture 등): 위치 1 증가
                current_pos += 1;
            }
        }
    }
}
