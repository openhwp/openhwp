//! IR → HWP 변환
//!
//! IR 문서를 HWP로 변환합니다.
//!
//! 이 모듈은 두 가지 기능을 제공합니다:
//! 1. IR → HWP 데이터 변환 (HwpBuildData)
//! 2. IR → HWP 파일 바이트 변환 (to_hwp_bytes)

use ir::{
    ConversionError, ConversionResult, Document as IrDocument, StyleStore,
    border_fill::{Border as IrBorder, BorderFill as IrBorderFill},
    control::{
        AutoNumber as IrAutoNumber, AutoNumberType as IrAutoNumberType, Bookmark as IrBookmark,
        Caption as IrCaption, CaptionPosition as IrCaptionPosition, Chart as IrChart,
        ChartType as IrChartType, Control as IrControl, Equation as IrEquation,
        FormObject as IrFormObject, FormObjectType as IrFormObjectType,
        HeaderFooterControl as IrHeaderFooter, HiddenComment as IrHiddenComment,
        Hyperlink as IrHyperlink, NewNumber as IrNewNumber, Note as IrNote,
        ObjectCommon as IrObjectCommon, OleObject as IrOleObject, TextArt as IrTextArt,
        TextArtAlignment as IrTextArtAlignment, TextArtFontStyle as IrTextArtFontStyle,
        TextArtShapeType as IrTextArtShapeType, TextBox as IrTextBox, TextWrap as IrTextWrap,
        Video as IrVideo, VideoType as IrVideoType,
    },
    para_shape::{LineSpacingValue, ParaShape as IrParaShape, TabDef as IrTabDef},
    paragraph::{
        FieldStart as IrFieldStart, Paragraph as IrParagraph, Run as IrRun,
        RunContent as IrRunContent,
    },
    picture::Picture as IrPicture,
    shape::{
        ArcType as IrArcType, CurvePointType as IrCurvePointType, Shape as IrShape,
        ShapeType as IrShapeType,
    },
    table::{Table as IrTable, TableCell as IrTableCell},
};
use primitive::{
    Alignment, FieldType as IrFieldType, HeaderFooterApplyTo,
    HorizontalRelativeTo as IrHorizontalRelativeTo, LineType as IrLineType, NumberFormat,
    StyleType as IrStyleType, TabLeader as IrTabLeader, TabType as IrTabType,
    TextWrapSide as IrTextWrapSide, TextWrapType as IrTextWrapType,
    VerticalRelativeTo as IrVerticalRelativeTo,
};

use crate::writer::{
    BodyWriter, DocInfoWriter, HwpWriter,
    body_writer::{
        ArcShapeData, AutoNumberData, BookmarkData, CaptionData as HwpCaptionData,
        CaptionDirection as HwpCaptionDirection, CharShapeRef, ChartData as HwpChartData,
        ColumnDefinitionData, ContainerShapeData, ControlData, CurveShapeData, EllipseShapeData,
        EndnoteShapeData, EquationData, FieldData as HwpFieldData,
        FieldTypeData as HwpFieldTypeData, FootnoteShapeData, FormObjectData as HwpFormObjectData,
        HeaderFooterData, HiddenCommentData, HyperlinkData, LineShapeData, NewNumberData, NoteData,
        ObjectCommonData, OleData as HwpOleData, PageBorderFillData, PageDefinitionData,
        ParagraphData, PictureData, PolygonShapeData, RectangleShapeData, SectionData,
        SectionDefinitionData, ShapeData, ShapeTypeData, TableCellData, TableData,
        TextArtData as HwpTextArtData, TextBoxData, VideoData as HwpVideoData,
    },
    doc_info_writer::{
        BinaryDataInfo, BorderFillData, BorderLine, BulletData, CharShapeData, FontCounts,
        FontData, NumberingData, NumberingLevelData, PanoseData, ParaShapeData, StyleData,
        TabDefinitionData, TabItem,
    },
};

use crate::doc_info::{
    FillInfo as HwpFillInfo, GradientFill as HwpGradientFill, GradientType as HwpGradientType,
    ImageFill as HwpImageFill, ImageFillType as HwpImageFillType, ImageInfo as HwpImageInfo,
    PatternFill as HwpPatternFill, PatternType as HwpPatternType,
};
use crate::primitive::ColorReference;

use super::{ColorConvert, FromIrContext};

/// IR → HWP 변환 트레이트
pub trait IrToHwp {
    /// HwpDocument 생성을 위한 데이터 추출
    ///
    /// 반환값은 HWP 파일 생성에 필요한 중간 데이터입니다.
    fn to_hwp_data(&self) -> Result<ConversionResult<HwpBuildData>, ConversionError>;

    /// IR 문서를 HWP 파일 바이트로 변환합니다.
    fn to_hwp_bytes(&self) -> Result<ConversionResult<Vec<u8>>, ConversionError>;
}

/// HWP 파일 생성을 위한 중간 데이터
///
/// HwpDocument를 직접 생성할 수 없으므로 (파싱 전용),
/// HWP 파일 생성에 필요한 데이터를 별도 구조체로 제공합니다.
#[derive(Debug, Clone, Default)]
pub struct HwpBuildData {
    /// 문서 제목
    pub title: Option<String>,
    /// 저자
    pub author: Option<String>,
    /// 섹션 데이터
    pub sections: Vec<HwpSectionData>,
    /// 바이너리 데이터 (이미지 등)
    pub binary_data: Vec<(String, Vec<u8>)>,
}

/// HWP 섹션 데이터
#[derive(Debug, Clone, Default)]
pub struct HwpSectionData {
    /// 페이지 너비 (HwpUnit)
    pub page_width: i32,
    /// 페이지 높이 (HwpUnit)
    pub page_height: i32,
    /// 문단 텍스트 목록
    pub paragraphs: Vec<String>,
}

impl IrToHwp for IrDocument {
    fn to_hwp_data(&self) -> Result<ConversionResult<HwpBuildData>, ConversionError> {
        let mut ctx = FromIrContext::new();
        let data = convert_to_hwp_data(self, &mut ctx)?;
        Ok(ctx.warnings.into_result(data))
    }

    fn to_hwp_bytes(&self) -> Result<ConversionResult<Vec<u8>>, ConversionError> {
        let mut ctx = FromIrContext::new();
        let bytes = convert_to_hwp_bytes(self, &mut ctx)?;
        Ok(ctx.warnings.into_result(bytes))
    }
}

/// IR → HWP 데이터 변환
fn convert_to_hwp_data(
    ir: &IrDocument,
    ctx: &mut FromIrContext,
) -> Result<HwpBuildData, ConversionError> {
    let mut data = HwpBuildData {
        title: ir.metadata.title.clone(),
        author: ir.metadata.author.clone(),
        ..Default::default()
    };

    // 섹션 변환
    for section in &ir.sections {
        let section_data = HwpSectionData {
            page_width: section.page.width.value(),
            page_height: section.page.height.value(),
            paragraphs: section
                .paragraphs
                .iter()
                .map(|p| p.to_plain_text())
                .collect(),
        };
        data.sections.push(section_data);
    }

    // 바이너리 데이터 변환
    for (id, binary) in ir.binary_data.iter() {
        data.binary_data
            .push((id.value().to_string(), binary.data.clone()));
    }

    // HWPX 확장 데이터 경고
    if ir.extensions.hwpx.is_some() {
        ctx.warnings
            .data_loss("HWPX 확장 데이터는 HWP로 변환 시 손실됩니다");
    }

    Ok(data)
}

/// IR → HWP 파일 바이트 변환
fn convert_to_hwp_bytes(
    ir: &IrDocument,
    ctx: &mut FromIrContext,
) -> Result<Vec<u8>, ConversionError> {
    let section_count = ir.sections.len().max(1) as u16;

    // DocInfo 빌드
    let doc_info_data = build_doc_info(ir, section_count, ctx)?;

    // BodyText 빌드
    let sections_data = build_body_sections(ir, ctx)?;

    // HwpWriter로 파일 생성
    let mut writer = HwpWriter::new();
    writer.set_doc_info(doc_info_data);

    for section_data in sections_data {
        writer.add_section(section_data);
    }

    // 바이너리 데이터 추가
    for (id, binary) in ir.binary_data.iter() {
        let ext = detect_extension(&binary.data);
        let name = format!("BIN{:04X}.{}", id.value().parse::<u16>().unwrap_or(0), ext);
        writer.add_binary_data(name, binary.data.clone());
    }

    writer
        .write_to_bytes()
        .map_err(|e| ConversionError::unsupported(e.to_string()))
}

/// DocInfo 스트림 빌드
fn build_doc_info(
    ir: &IrDocument,
    section_count: u16,
    ctx: &mut FromIrContext,
) -> Result<Vec<u8>, ConversionError> {
    let mut doc_info = DocInfoWriter::new(section_count);

    // 폰트 설정
    let fonts = build_fonts(&ir.styles, ctx);
    doc_info.set_fonts(fonts);

    // 글자 모양
    for char_shape in &ir.styles.char_shapes {
        let data = convert_char_shape(char_shape);
        doc_info.add_char_shape(data);
    }

    // 문단 모양
    for para_shape in &ir.styles.para_shapes {
        let data = convert_para_shape(para_shape);
        doc_info.add_para_shape(data);
    }

    // 스타일
    for style in &ir.styles.styles {
        let data = convert_style(style);
        doc_info.add_style(data);
    }

    // BorderFill 변환
    for border_fill in &ir.styles.border_fills {
        let data = convert_border_fill_to_hwp(border_fill);
        doc_info.add_border_fill(data);
    }

    // TabDef 변환
    for tab_def in &ir.styles.tab_defs {
        let data = convert_tab_def_to_hwp(tab_def);
        doc_info.add_tab_definition(data);
    }

    // Numbering 변환
    for numbering in &ir.styles.numberings {
        let data = convert_numbering_to_hwp(numbering);
        doc_info.add_numbering(data);
    }

    // Bullet 변환
    for bullet in &ir.styles.bullets {
        let data = convert_bullet_to_hwp(bullet);
        doc_info.add_bullet(data);
    }

    // 바이너리 데이터 정보 추가
    for (id, binary) in ir.binary_data.iter() {
        let bin_id = id.value().parse::<u16>().unwrap_or_else(|_| {
            // "BIN0001" 형식에서 숫자 추출
            id.value()
                .trim_start_matches("BIN")
                .parse::<u16>()
                .unwrap_or(0)
        });

        let extension = binary.format.extension().to_string();

        let info = BinaryDataInfo {
            // 타입: 0=링크, 1=임베드, 2=스토리지
            // IR에서 임베드된 데이터만 지원하므로 1로 설정
            data_type: 1,
            abs_path: String::new(),
            rel_path: String::new(),
            bin_data_id: bin_id,
            extension,
        };
        doc_info.add_binary_data_info(info);
    }

    Ok(doc_info.build())
}

/// 폰트 목록 빌드
fn build_fonts(styles: &StyleStore, _ctx: &mut FromIrContext) -> FontCounts {
    use ir::char_shape::FontType as IrFontType;

    let mut fonts = FontCounts::default();

    // 폰트 수집 (기본 폰트 추가)
    let mut korean_fonts: Vec<FontData> = Vec::new();
    let mut english_fonts: Vec<FontData> = Vec::new();

    for font in &styles.fonts {
        // 한글 폰트와 영문 폰트 구분 (간단한 휴리스틱)
        let is_korean = font
            .name
            .chars()
            .any(|c| ('\u{AC00}'..='\u{D7A3}').contains(&c));

        // FontData 생성
        let font_data = FontData {
            name: font.name.clone(),
            alternate_type: font.alternate_name.as_ref().map(|_| {
                // FontType을 HWP 대체 폰트 타입으로 변환
                match font.font_type {
                    IrFontType::TrueType => 1,
                    IrFontType::HangeulOnly => 2,
                    IrFontType::Representative => 0,
                }
            }),
            alternate_name: font.alternate_name.clone(),
            panose_info: font.panose.map(|p| {
                let bytes = p.to_bytes();
                PanoseData {
                    family_kind: bytes[0],
                    serif_style: bytes[1],
                    weight: bytes[2],
                    proportion: bytes[3],
                    contrast: bytes[4],
                    stroke_variation: bytes[5],
                    arm_style: bytes[6],
                    letterform: bytes[7],
                    midline: bytes[8],
                    x_height: bytes[9],
                }
            }),
            default_font_name: font.default_font_name.clone(),
        };

        if is_korean {
            if !korean_fonts.iter().any(|f| f.name == font.name) {
                korean_fonts.push(font_data);
            }
        } else if !english_fonts.iter().any(|f| f.name == font.name) {
            english_fonts.push(font_data);
        }
    }

    // 기본 폰트 추가 (없으면)
    if korean_fonts.is_empty() {
        korean_fonts.push(FontData {
            name: "함초롬돋움".to_string(),
            alternate_type: None,
            alternate_name: None,
            panose_info: None,
            default_font_name: None,
        });
    }
    if english_fonts.is_empty() {
        english_fonts.push(FontData {
            name: "함초롬돋움".to_string(),
            alternate_type: None,
            alternate_name: None,
            panose_info: None,
            default_font_name: None,
        });
    }

    fonts.korean = korean_fonts;
    fonts.english = english_fonts;

    fonts
}

/// IR CharShape → HWP CharShapeData 변환
fn convert_char_shape(shape: &ir::char_shape::CharShape) -> CharShapeData {
    use super::font_helper::extract_font_field;

    // 언어별 폰트 데이터 추출 (한글, 영문, 한자, 일본어, 기타, 기호, 사용자 순)
    let font_ids = extract_font_field(&shape.fonts, |f| f.id.value() as u16, 0);
    let font_ratios = extract_font_field(&shape.fonts, |f| f.width_ratio.0 as u8, 100);
    let font_spacings = extract_font_field(
        &shape.fonts,
        |f| f.spacing.0 as i8,
        shape.char_spacing.0 as i8,
    );
    let relative_sizes = extract_font_field(&shape.fonts, |f| f.relative_size.0 as u8, 100);
    let positions = extract_font_field(&shape.fonts, |f| f.offset.0 as i8, 0);

    // 속성 플래그 구성 (HWP 명세 표 35)
    use crate::doc_info::{CharShapeFlags, CharShapePropsBuilder};

    // 밑줄 위치 (0=없음, 1=아래, 3=위)
    let underline_pos = if shape.underline.line_type == primitive::UnderlineType::None {
        0
    } else {
        match shape.underline.position {
            primitive::UnderlinePosition::Bottom => 1,
            primitive::UnderlinePosition::Top => 3,
        }
    };

    // 밑줄 모양
    let underline_shape = match shape.underline.line_type {
        primitive::UnderlineType::None => 0,
        primitive::UnderlineType::Single => 1,
        primitive::UnderlineType::Double => 2,
        primitive::UnderlineType::Thick => 3,
        primitive::UnderlineType::Dotted => 4,
        primitive::UnderlineType::Dash => 5,
        primitive::UnderlineType::DashDot => 6,
        primitive::UnderlineType::DashDotDot => 7,
        primitive::UnderlineType::Wave => 8,
    };

    // 외곽선 종류
    let outline_val = match shape.outline {
        primitive::OutlineType::None => 0,
        primitive::OutlineType::Outline => 1,
        primitive::OutlineType::Shadow => 2,
        primitive::OutlineType::Emboss => 3,
        primitive::OutlineType::Engrave => 4,
    };

    // 그림자 종류
    let shadow_val = convert_shadow_type_to_bits(&shape.shadow.shadow_type);

    // 취소선
    let strikethrough_val = match shape.strikethrough {
        primitive::StrikethroughType::None => 0,
        primitive::StrikethroughType::Single => 1,
        primitive::StrikethroughType::Double => 2,
    };

    // 강조점
    let emphasis_val = match shape.emphasis.emphasis_type {
        primitive::EmphasisType::None => 0,
        primitive::EmphasisType::Dot => 1,
        primitive::EmphasisType::Circle => 2,
        primitive::EmphasisType::CircleOpen => 3,
        primitive::EmphasisType::Comma => 4,
        primitive::EmphasisType::Colon => 5,
        primitive::EmphasisType::Tilde => 6,
        primitive::EmphasisType::Caron => 7,
    };

    // 속성 플래그 빌더로 구성
    let properties = CharShapePropsBuilder::new()
        .with_flag_if(shape.italic, CharShapeFlags::ITALIC)
        .with_flag_if(shape.bold, CharShapeFlags::BOLD)
        .with_underline_pos(underline_pos)
        .with_underline_shape(underline_shape)
        .with_outline(outline_val)
        .with_shadow(shadow_val)
        .with_flag_if(shape.emboss, CharShapeFlags::EMBOSS)
        .with_flag_if(shape.engrave, CharShapeFlags::ENGRAVE)
        .with_flag_if(shape.superscript, CharShapeFlags::SUPERSCRIPT)
        .with_flag_if(shape.subscript, CharShapeFlags::SUBSCRIPT)
        .with_strikethrough(strikethrough_val)
        .with_emphasis(emphasis_val)
        .with_flag_if(shape.use_kerning, CharShapeFlags::KERNING)
        .build();

    // shade_color (음영 색상, IR의 shade_color 우선 사용, 없으면 background_color 사용)
    let shade_color = shape
        .shade_color
        .as_ref()
        .or(shape.background_color.as_ref())
        .map(ColorConvert::to_bgr_u32)
        .unwrap_or(0xFFFFFF);

    // 밑줄 색상 (None이면 글자 색상 사용)
    let underline_color = shape
        .underline
        .color
        .as_ref()
        .map(ColorConvert::to_bgr_u32)
        .unwrap_or_else(|| ColorConvert::to_bgr_u32(&shape.color));

    // 그림자 색상 및 오프셋
    // HWP shadow offset은 i8 퍼센트 단위 (-100% ~ 100%)
    let shadow_color = shape
        .shadow
        .color
        .as_ref()
        .map(ColorConvert::to_bgr_u32)
        .unwrap_or(0x808080); // 기본값: 회색
    // IR에서는 퍼센트 값을 그대로 저장하므로 i8 범위로 클램프
    let shadow_offset_x = shape.shadow.offset_x.value().clamp(-100, 100) as i8;
    let shadow_offset_y = shape.shadow.offset_y.value().clamp(-100, 100) as i8;

    // 테두리/배경 ID
    let border_fill_id = shape.border_fill_id_ref.map(|id| id.value() as u16);

    // 취소선 색상
    let strikethrough_color = None; // TODO: IR에서 취소선 색상 지원 필요

    CharShapeData {
        font_ids,
        font_ratios,
        font_spacings,
        relative_sizes,
        positions,
        font_size: shape.font_size.value(),
        properties,
        shadow_offset_x,
        shadow_offset_y,
        text_color: ColorConvert::to_bgr_u32(&shape.color),
        underline_color,
        shade_color,
        shadow_color,
        border_fill_id,
        strikethrough_color,
    }
}

/// IR ParaShape → HWP ParaShapeData 변환
fn convert_para_shape(shape: &IrParaShape) -> ParaShapeData {
    // 정렬 값 변환 (bit 2~4)
    let alignment = match shape.alignment {
        Alignment::Left => 1,
        Alignment::Center => 3,
        Alignment::Right => 2,
        Alignment::Justify => 0,
        Alignment::Distribute => 4,
        Alignment::Divide => 5, // HWP Divide alignment
    };

    // properties1 구성
    let mut properties1: u32 = (alignment & 0x07) << 2;

    // snap_to_grid (bit 8)
    if shape.snap_to_grid {
        properties1 |= 1 << 8;
    }

    // widow_orphan_control (bit 16)
    if shape.widow_orphan_control {
        properties1 |= 1 << 16;
    }

    // keep_with_next (bit 17)
    if shape.keep_with_next {
        properties1 |= 1 << 17;
    }

    // keep_lines (bit 18)
    if shape.keep_lines {
        properties1 |= 1 << 18;
    }

    // page_break_before (bit 19)
    if shape.page_break_before {
        properties1 |= 1 << 19;
    }

    // auto_line_height (bit 22) - 100%가 아니면 활성화
    if shape.auto_line_height_ratio.0 != 100.0 {
        properties1 |= 1 << 22;
    }

    // 문단 테두리 관련 속성
    if let Some(ref border) = shape.border {
        // connect (bit 28)
        if border.connect {
            properties1 |= 1 << 28;
        }
        // ignore_margin (bit 29)
        if border.ignore_margin {
            properties1 |= 1 << 29;
        }
    }

    // properties2 구성
    let mut properties2: u32 = 0;

    // suppress_line_numbers (bit 0~1)
    if shape.suppress_line_numbers {
        properties2 |= 1;
    }

    // 줄 간격 값 추출
    let line_spacing_val = match &shape.line_spacing.value {
        LineSpacingValue::Percent(p) => (p.0 * 100.0) as i32, // 160% -> 16000
        LineSpacingValue::Fixed(u) => u.value(),
    };

    // 문단 테두리 정보 추출
    let (
        border_fill_id,
        border_space_left,
        border_space_right,
        border_space_top,
        border_space_bottom,
    ) = if let Some(ref border) = shape.border {
        (
            border.border_fill_id_ref.value() as u16,
            border.offset_left.value() as i16,
            border.offset_right.value() as i16,
            border.offset_top.value() as i16,
            border.offset_bottom.value() as i16,
        )
    } else {
        // border_fill_id만 ParaShape에서 가져오기
        let bf_id = shape
            .border_fill_id
            .map(|id| id.value() as u16)
            .unwrap_or(0);
        (bf_id, 0, 0, 0, 0)
    };

    // Tab 정의 ID
    let tab_def_id = shape.tab_def_id.map(|id| id.value() as u16).unwrap_or(0);

    // Numbering/Bullet ID (번호 매기기 정의 ID)
    let numbering_bullet_id = shape
        .numbering
        .as_ref()
        .and_then(|n| n.numbering_id.or(n.bullet_id))
        .map(|id| id as u16)
        .unwrap_or(0);

    ParaShapeData {
        properties1,
        margin_left: shape.margin_left.value(),
        margin_right: shape.margin_right.value(),
        indent: shape.first_line_indent.value(),
        space_before: shape.space_before.value(),
        space_after: shape.space_after.value(),
        line_spacing: (line_spacing_val / 100) as i16, // HwpUnit -> 일반값
        tab_def_id,
        numbering_bullet_id,
        border_fill_id,
        border_space_left,
        border_space_right,
        border_space_top,
        border_space_bottom,
        properties2,
        properties3: 0,
        line_spacing2: line_spacing_val as u32,
    }
}

/// IR Style → HWP StyleData 변환
fn convert_style(style: &ir::style::Style) -> StyleData {
    let style_type = match style.style_type {
        IrStyleType::Paragraph => 0,
        IrStyleType::Character => 1,
    };

    StyleData {
        name: style.name.clone(),
        english_name: style.english_name.clone().unwrap_or_default(),
        style_type,
        next_style_id: style.next_style_id.map(|id| id.value() as u8).unwrap_or(0),
        lang_id: 0x0412, // Korean
        para_shape_id: style.para_shape_id.map(|id| id.value() as u16).unwrap_or(0),
        char_shape_id: style.char_shape_id.map(|id| id.value() as u16).unwrap_or(0),
    }
}

/// BodyText 섹션들 빌드
fn build_body_sections(
    ir: &IrDocument,
    ctx: &mut FromIrContext,
) -> Result<Vec<Vec<u8>>, ConversionError> {
    let mut body_writer = BodyWriter::new();

    for section in &ir.sections {
        let section_data = build_section(section, ir, ctx)?;
        body_writer.add_section(section_data);
    }

    // 섹션이 없으면 빈 섹션 추가
    if ir.sections.is_empty() {
        body_writer.add_section(SectionData::default());
    }

    Ok(body_writer.build())
}

/// 섹션 빌드
fn build_section(
    section: &ir::Section,
    ir_doc: &IrDocument,
    _ctx: &mut FromIrContext,
) -> Result<SectionData, ConversionError> {
    let mut data = SectionData::default();

    // 페이지 정의
    let orientation_flag = match section.page.orientation {
        primitive::PageOrientation::Portrait => 0,
        primitive::PageOrientation::Landscape => 1,
    };
    let gutter_flag = section.page.gutter_position.to_raw();
    let properties = orientation_flag | ((gutter_flag as u32) << 1);

    data.page_definition = Some(PageDefinitionData {
        paper_width: section.page.width.value() as u32,
        paper_height: section.page.height.value() as u32,
        margin_left: section.page.margins.left.value() as u32,
        margin_right: section.page.margins.right.value() as u32,
        margin_top: section.page.margins.top.value() as u32,
        margin_bottom: section.page.margins.bottom.value() as u32,
        margin_header: section.page.margins.header.value() as u32,
        margin_footer: section.page.margins.footer.value() as u32,
        margin_gutter: section.page.margins.gutter.value() as u32,
        properties,
    });

    // 페이지 테두리/배경
    if let Some(ref page_border) = section.page_border_fill {
        // properties 비트 구성:
        // - 비트 0: 위치 (0=용지, 1=본문)
        // - 비트 1: 머리글 포함
        // - 비트 2: 바닥글 포함
        // - 비트 3: 텍스트 뒤에 채우기
        let position_flag = match page_border.position {
            ir::section::PageBorderPosition::Paper => 0,
            ir::section::PageBorderPosition::Body => 1,
        };
        let header_flag = if page_border.header_inside { 0x2 } else { 0 };
        let footer_flag = if page_border.footer_inside { 0x4 } else { 0 };
        let fill_behind_flag = if page_border.fill_behind { 0x8 } else { 0 };
        let properties = position_flag | header_flag | footer_flag | fill_behind_flag;

        data.page_border_fill = Some(PageBorderFillData {
            border_fill_id: page_border.border_fill_id.value() as u16,
            properties,
            offset_left: page_border.offsets.left.value(),
            offset_right: page_border.offsets.right.value(),
            offset_top: page_border.offsets.top.value(),
            offset_bottom: page_border.offsets.bottom.value(),
        });
    }

    // 각주 모양
    if let Some(ref footnote_shape) = section.footnote_shape {
        let base = &footnote_shape.base;
        let number_format_flag = convert_note_number_format_to_hwp(base.number_format);
        let superscript_flag = if base.superscript { 0x100 } else { 0 };
        let properties = number_format_flag | superscript_flag;

        data.footnote_shape = Some(FootnoteShapeData {
            properties,
            prefix: base.prefix.clone().unwrap_or_default(),
            suffix: base.suffix.clone().unwrap_or_default(),
            start_number: base.start_number as u16,
            separator_length: base.separator_length.value() as u16, // HwpUnit → u16
            separator_position: base
                .separator_position
                .map(|p| p.value() as u16)
                .unwrap_or(0),
            space_above: base.space_above.value() as u16,
            space_below: base.space_below.value() as u16,
            space_between: base.space_between.value() as u16,
            separator_line_type: match base.separator_line_type {
                primitive::LineType::None => 0,
                primitive::LineType::Solid => 1,
                primitive::LineType::Dash => 2,
                primitive::LineType::Dot => 3,
                primitive::LineType::DashDot => 4,
                primitive::LineType::DashDotDot => 5,
                primitive::LineType::LongDash => 6,
                primitive::LineType::Double => 7,
                primitive::LineType::Triple => 8,
                primitive::LineType::Wave => 9,
                primitive::LineType::DoubleWave => 10,
                primitive::LineType::ThickThinLarge => 11,
                primitive::LineType::ThinThickLarge => 12,
                primitive::LineType::Circle => 13,
            },
            separator_line_thickness: base.separator_line_width,
            // HWP 색상은 BGR 형식 (0x00BBGGRR)
            separator_line_color: ((base.separator_line_color.blue as u32) << 16)
                | ((base.separator_line_color.green as u32) << 8)
                | (base.separator_line_color.red as u32),
        });
    }

    // 미주 모양
    if let Some(ref endnote_shape) = section.endnote_shape {
        let base = &endnote_shape.base;
        let number_format_flag = convert_note_number_format_to_hwp(base.number_format);
        let superscript_flag = if base.superscript { 0x100 } else { 0 };
        let properties = number_format_flag | superscript_flag;

        data.endnote_shape = Some(EndnoteShapeData {
            properties,
            prefix: base.prefix.clone().unwrap_or_default(),
            suffix: base.suffix.clone().unwrap_or_default(),
            start_number: base.start_number as u16,
            separator_position: base
                .separator_position
                .map(|p| p.value() as u16)
                .unwrap_or(0),
        });
    }

    // 구역 정의 (시작 페이지 번호 및 가시성 설정)
    // properties 비트 구성 (표 130):
    // - 비트 0: 머리글 숨김
    // - 비트 1: 바닥글 숨김
    // - 비트 2: 마스터페이지 숨김
    // - 비트 3: 테두리 숨김
    // - 비트 4: 배경 숨김
    // - 비트 5: 쪽 번호 위치 숨김
    let visibility = &section.extensions.visibility;
    let hide_header_flag = if visibility.hide_header { 1 << 0 } else { 0 };
    let hide_footer_flag = if visibility.hide_footer { 1 << 1 } else { 0 };
    let hide_master_page_flag = if visibility.hide_master_page {
        1 << 2
    } else {
        0
    };
    let hide_border_flag = if visibility.hide_border { 1 << 3 } else { 0 };
    let hide_background_flag = if visibility.hide_background {
        1 << 4
    } else {
        0
    };
    let hide_page_number_flag = if visibility.hide_page_number {
        1 << 5
    } else {
        0
    };
    let properties = hide_header_flag
        | hide_footer_flag
        | hide_master_page_flag
        | hide_border_flag
        | hide_background_flag
        | hide_page_number_flag;

    // 가시성 설정이 있거나 시작 페이지 번호, 그리드 설정이 있으면 섹션 정의 생성
    let has_visibility = visibility.hide_header
        || visibility.hide_footer
        || visibility.hide_master_page
        || visibility.hide_border
        || visibility.hide_background
        || visibility.hide_page_number;

    let grid = &section.extensions.grid;
    let has_grid = grid.line_grid > 0 || grid.character_grid > 0;

    // representative_language 가져오기
    let language = ir_doc.settings.representative_language.unwrap_or(0);

    if section.start_number.page > 0 || has_visibility || has_grid || language > 0 {
        data.section_definition = Some(SectionDefinitionData {
            properties,
            starting_page_number: section.start_number.page as u16,
            vertical_grid: grid.line_grid as u16,
            horizontal_grid: grid.character_grid as u16,
            language,
            ..Default::default()
        });
    }

    // 줄 번호 모양
    // 참고: HWP 5.0 바이너리 형식에서는 줄 번호 모양을 지원하지 않으므로
    // section.extensions.line_number_shape는 무시됩니다.
    // 이 기능은 HWPX 전용입니다.

    // 단 정의 (다단 설정)
    if section.columns.count > 1 || !section.columns.widths.is_empty() {
        let direction_flag: u16 = match section.columns.direction {
            ir::section::ColumnDirection::LeftToRight => 0,
            ir::section::ColumnDirection::RightToLeft => 1,
            ir::section::ColumnDirection::FacingPages => 2, // 맞쪽 페이지
        };
        let same_width_flag: u16 = if section.columns.widths.is_empty() {
            1 << 12
        } else {
            0
        };
        let properties1 = section.columns.count << 2 | direction_flag << 10 | same_width_flag;

        let separator_style = match section.columns.separator {
            ir::section::ColumnSeparator::None => 0,
            ir::section::ColumnSeparator::Solid => 1,
            ir::section::ColumnSeparator::Dash => 2,
            ir::section::ColumnSeparator::Dot => 3,
        };

        data.column_definition = Some(ColumnDefinitionData {
            properties1,
            column_gap: section.columns.gap.value() as u16,
            column_widths: section
                .columns
                .widths
                .iter()
                .map(|w| w.value() as u16)
                .collect(),
            properties2: 0,
            separator_style,
            separator_thickness: section.columns.separator_thickness,
            separator_color: ColorConvert::to_bgr_u32(&section.columns.separator_color),
        });
    }

    // 문단들
    for para in &section.paragraphs {
        let para_data = build_paragraph(para)?;
        data.paragraphs.push(para_data);
    }

    Ok(data)
}

/// NumberFormat을 HWP 번호 형식 플래그로 변환 (각주/미주용)
/// 각주/미주는 0-7만 지원하므로 나머지는 가장 유사한 형식으로 매핑
fn convert_note_number_format_to_hwp(format: NumberFormat) -> u32 {
    match format {
        NumberFormat::Digit => 0,
        NumberFormat::CircledDigit => 1,
        NumberFormat::RomanUpper => 2,
        NumberFormat::RomanLower => 3,
        NumberFormat::LatinUpper => 4,
        NumberFormat::LatinLower => 5,
        NumberFormat::HangulSyllable => 6,
        NumberFormat::HangulJamo => 7,
        // 각주/미주에서 지원하지 않는 형식들은 가장 유사한 것으로 매핑
        NumberFormat::CircledLatinUpper => 1, // CircledDigit으로 fallback
        NumberFormat::CircledLatinLower => 1, // CircledDigit으로 fallback
        NumberFormat::CircledHangul => 1,     // CircledDigit으로 fallback
        NumberFormat::CircledHangulJamo => 1, // CircledDigit으로 fallback
        NumberFormat::HangulIdeograph => 6,   // HangulSyllable으로 fallback
        NumberFormat::Ideograph => 0,         // Digit으로 fallback
        NumberFormat::CircledIdeograph => 1,  // CircledDigit으로 fallback
        NumberFormat::Ganji => 0,             // Digit으로 fallback
    }
}

/// RangeTag 변환 (IR → HWP)
///
/// IR RangeTag를 HWP RangeTagData로 변환합니다.
fn convert_range_tag_to_hwp(
    range_tag: &ir::paragraph::RangeTag,
) -> crate::writer::body_writer::RangeTagData {
    use crate::writer::body_writer::RangeTagData;
    use ir::paragraph::RangeTagType;

    // 태그 종류를 상위 바이트로 변환
    let tag_type_byte = match range_tag.tag_type {
        RangeTagType::Bookmark => 0,
        RangeTagType::Hyperlink => 1,
        RangeTagType::TrackChangeInsert => 2,
        RangeTagType::TrackChangeDelete => 3,
        RangeTagType::Highlight => 4,
        RangeTagType::Other(byte) => byte,
    };

    // 태그 데이터 추출
    let (tag_data_low, tag_data_mid) = if let Some(track_info) = &range_tag.track_change_info {
        // 변경 추적 ID를 하위 16비트로 저장
        let id_bytes = (track_info.track_change_id as u16).to_le_bytes();
        (id_bytes[0], id_bytes[1])
    } else if let Some(data_str) = &range_tag.data {
        // 16진수 문자열을 바이트로 파싱
        let bytes: Vec<u8> = (0..data_str.len())
            .step_by(2)
            .filter_map(|i| {
                let end = (i + 2).min(data_str.len());
                u8::from_str_radix(&data_str[i..end], 16).ok()
            })
            .collect();
        (
            bytes.get(1).copied().unwrap_or(0),
            bytes.first().copied().unwrap_or(0),
        )
    } else {
        (0, 0)
    };

    RangeTagData {
        start_position: range_tag.start,
        end_position: range_tag.end,
        tag: [tag_data_low, tag_data_mid, tag_type_byte],
    }
}

/// 문단 빌드
fn build_paragraph(para: &IrParagraph) -> Result<ParagraphData, ConversionError> {
    let text = para.to_plain_text();
    let utf16: Vec<u16> = text.encode_utf16().collect();

    // 컨트롤 추출 (표 등)
    let controls = extract_controls(para)?;

    // RangeTag 변환 (IR → HWP)
    let range_tags = para
        .range_tags
        .iter()
        .map(convert_range_tag_to_hwp)
        .collect();

    // CharShapeRef 생성 (Run별 char_shape_id 추적)
    let char_shape_refs = build_char_shape_refs(para);

    Ok(ParagraphData {
        para_shape_id: para.para_shape_id.map(|id| id.value() as u16).unwrap_or(0),
        style_id: para.style_id.map(|id| id.value() as u8).unwrap_or(0),
        text: utf16,
        char_shape_refs,
        range_tags,
        controls,
    })
}

/// Run을 순회하면서 CharShapeRef 목록 생성
fn build_char_shape_refs(para: &IrParagraph) -> Vec<CharShapeRef> {
    let mut refs = Vec::new();
    let mut position: u32 = 0;
    let mut current_char_shape_id: Option<u32> = None;

    for run in &para.runs {
        // char_shape_id 변경 감지
        let run_char_shape_id = run.char_shape_id.map(|id| id.value()).unwrap_or(0);

        if current_char_shape_id != Some(run_char_shape_id) {
            refs.push(CharShapeRef {
                position,
                char_shape_id: run_char_shape_id,
            });
            current_char_shape_id = Some(run_char_shape_id);
        }

        // Run 길이 계산 (UTF-16 코드 유닛 기준)
        position += get_run_utf16_length(run) as u32;
    }

    // 비어있으면 기본값 추가
    if refs.is_empty() {
        refs.push(CharShapeRef {
            position: 0,
            char_shape_id: 0,
        });
    }

    refs
}

/// Run 내용의 UTF-16 길이 계산
fn get_run_utf16_length(run: &IrRun) -> usize {
    run.contents
        .iter()
        .map(|content| {
            match content {
                IrRunContent::Text(text) => text.text.encode_utf16().count(),
                IrRunContent::Tab(_) => 1,
                IrRunContent::LineBreak => 1,
                IrRunContent::Hyphen => 1,
                IrRunContent::NonBreakingSpace => 1,
                IrRunContent::FixedWidthSpace => 1,
                IrRunContent::Control(_) => 1, // 컨트롤은 확장문자 1글자
                IrRunContent::FieldStart(_) => 1,
                IrRunContent::FieldEnd(_) => 1,
                IrRunContent::BookmarkStart(_) => 1,
                IrRunContent::BookmarkEnd(_) => 1,
                IrRunContent::Compose(c) => c.compose_text.encode_utf16().count(),
                IrRunContent::Dutmal(d) => {
                    d.main_text.encode_utf16().count() + d.sub_text.encode_utf16().count()
                }
            }
        })
        .sum()
}

/// 문단에서 컨트롤 추출
fn extract_controls(para: &IrParagraph) -> Result<Vec<ControlData>, ConversionError> {
    let mut controls = Vec::new();

    for run in &para.runs {
        for content in &run.contents {
            match content {
                IrRunContent::Control(ctrl) => {
                    if let Some(control_data) = convert_control_to_hwp(ctrl.as_ref())? {
                        controls.push(control_data);
                    }
                }
                IrRunContent::FieldStart(field_start) => {
                    // 필드를 ControlData::Field로 변환
                    let field_data = convert_field_start_to_hwp(field_start);
                    controls.push(ControlData::Field(field_data));
                }
                _ => {}
            }
        }
    }

    Ok(controls)
}

/// IR FieldStart → HWP FieldData 변환
fn convert_field_start_to_hwp(field_start: &IrFieldStart) -> HwpFieldData {
    let field_type = match field_start.field_type {
        IrFieldType::Date => HwpFieldTypeData::Date,
        IrFieldType::Time => HwpFieldTypeData::Time,
        IrFieldType::FilePath => HwpFieldTypeData::FilePath,
        IrFieldType::FileName => HwpFieldTypeData::FilePath, // FileName은 FilePath로 매핑
        IrFieldType::Title => HwpFieldTypeData::DocTitle,
        IrFieldType::Author => HwpFieldTypeData::Author,
        IrFieldType::PageNumber => HwpFieldTypeData::PageNumber,
        IrFieldType::PageCount => HwpFieldTypeData::TotalPages,
        IrFieldType::Summary => HwpFieldTypeData::Summary,
        IrFieldType::CrossReference => HwpFieldTypeData::CrossReference,
        IrFieldType::Hyperlink => HwpFieldTypeData::Hyperlink,
        IrFieldType::ClickHere => HwpFieldTypeData::ClickHere,
        IrFieldType::UserInfo => HwpFieldTypeData::UserInfo,
        IrFieldType::Formula => HwpFieldTypeData::Formula,
        IrFieldType::Memo => HwpFieldTypeData::Memo,
        IrFieldType::PrivateInfo => HwpFieldTypeData::PrivateInfo,
        IrFieldType::MetaTag => HwpFieldTypeData::MetaTag,
        IrFieldType::MailMerge => HwpFieldTypeData::MailMerge,
        IrFieldType::TableOfContents => HwpFieldTypeData::TableOfContents,
        IrFieldType::Bookmark | IrFieldType::Unknown => HwpFieldTypeData::Unknown,
    };

    // Note: parameters는 write_field에서 별도로 처리되거나 FieldData 구조 확장 필요
    HwpFieldData {
        field_type,
        instruction: field_start.instruction.clone().unwrap_or_default(),
    }
}

/// IR 컨트롤 → HWP ControlData 변환
fn convert_control_to_hwp(ctrl: &IrControl) -> Result<Option<ControlData>, ConversionError> {
    match ctrl {
        IrControl::Table(table) => {
            let table_data = convert_table_to_hwp(table)?;
            Ok(Some(ControlData::Table(table_data)))
        }
        IrControl::Picture(picture) => {
            let picture_data = convert_picture_to_hwp(picture)?;
            Ok(Some(ControlData::Picture(picture_data)))
        }
        IrControl::Equation(equation) => {
            let equation_data = convert_equation_to_hwp(equation)?;
            Ok(Some(ControlData::Equation(equation_data)))
        }
        IrControl::Header(header) => {
            let header_data = convert_header_footer_to_hwp(header)?;
            Ok(Some(ControlData::Header(header_data)))
        }
        IrControl::Footer(footer) => {
            let footer_data = convert_header_footer_to_hwp(footer)?;
            Ok(Some(ControlData::Footer(footer_data)))
        }
        IrControl::Footnote(note) => {
            let note_data = convert_note_to_hwp(note)?;
            Ok(Some(ControlData::Footnote(note_data)))
        }
        IrControl::Endnote(note) => {
            let note_data = convert_note_to_hwp(note)?;
            Ok(Some(ControlData::Endnote(note_data)))
        }
        IrControl::Hyperlink(link) => {
            let link_data = convert_hyperlink_to_hwp(link)?;
            Ok(Some(ControlData::Hyperlink(link_data)))
        }
        IrControl::Bookmark(bookmark) => {
            let bookmark_data = convert_bookmark_to_hwp(bookmark)?;
            Ok(Some(ControlData::Bookmark(bookmark_data)))
        }
        IrControl::AutoNumber(auto_num) => {
            // HWPX PageNumber (위치 정보 포함)는 PageNumberData로 변환
            if auto_num.number_type == IrAutoNumberType::Page
                && let Some(fmt) = auto_num.auto_number_format.as_ref()
                && let Some(position) = fmt.position
            {
                // PageNumberData로 변환
                use crate::writer::body_writer::PageNumberData;
                use primitive::PageNumberPosition;

                let position_value = match position {
                    PageNumberPosition::None => 0,
                    PageNumberPosition::TopLeft => 1,
                    PageNumberPosition::TopCenter => 2,
                    PageNumberPosition::TopRight => 3,
                    PageNumberPosition::BottomLeft => 4,
                    PageNumberPosition::BottomCenter => 5,
                    PageNumberPosition::BottomRight => 6,
                    PageNumberPosition::OutsideTop => 7,
                    PageNumberPosition::OutsideBottom => 8,
                    PageNumberPosition::InsideTop => 9,
                    PageNumberPosition::InsideBottom => 10,
                };

                let format_type = fmt.format_type.as_ref().unwrap_or(&auto_num.number_format);
                let number_format = convert_auto_number_format_to_hwp(format_type);

                let side_character = fmt
                    .side_character
                    .clone()
                    .unwrap_or_else(|| "-".to_string());

                return Ok(Some(ControlData::PageNumber(PageNumberData {
                    position: position_value,
                    number_format,
                    side_character,
                })));
            }

            // 일반 AutoNumber로 변환
            let auto_num_data = convert_auto_number_to_hwp(auto_num)?;
            Ok(Some(ControlData::AutoNumber(auto_num_data)))
        }
        IrControl::NewNumber(new_num) => {
            let new_num_data = convert_new_number_to_hwp(new_num)?;
            Ok(Some(ControlData::NewNumber(new_num_data)))
        }
        IrControl::HiddenComment(comment) => {
            let comment_data = convert_hidden_comment_to_hwp(comment)?;
            Ok(Some(ControlData::HiddenComment(comment_data)))
        }
        IrControl::Shape(shape) => {
            let shape_data = convert_shape_to_hwp(shape)?;
            Ok(Some(ControlData::Shape(shape_data)))
        }
        IrControl::TextBox(text_box) => {
            let textbox_data = convert_textbox_to_hwp(text_box)?;
            Ok(Some(ControlData::TextBox(textbox_data)))
        }
        IrControl::Video(video) => {
            let video_data = convert_video_to_hwp(video)?;
            Ok(Some(ControlData::Video(video_data)))
        }
        IrControl::Ole(ole) => {
            let ole_data = convert_ole_to_hwp(ole)?;
            Ok(Some(ControlData::Ole(ole_data)))
        }
        IrControl::Chart(chart) => {
            let chart_data = convert_chart_to_hwp(chart)?;
            Ok(Some(ControlData::Chart(chart_data)))
        }
        IrControl::FormObject(form) => {
            let form_data = convert_form_object_to_hwp(form)?;
            Ok(Some(ControlData::FormObject(form_data)))
        }
        IrControl::TextArt(text_art) => {
            let text_art_data = convert_text_art_to_hwp(text_art)?;
            Ok(Some(ControlData::TextArt(text_art_data)))
        }
        IrControl::Memo(_memo) => {
            // Memo는 HWP에서 HiddenComment와 유사하게 처리
            // 현재 HWP ControlData에 Memo variant가 없으므로 None 반환
            // TODO: HWP Memo 지원 필요 시 ControlData 확장 필요
            Ok(None)
        }
        IrControl::IndexMark(_index_mark) => {
            // IndexMark는 HWP에서 직접 지원하지 않음
            // TODO: HWP IndexMark 지원 필요 시 ControlData 확장 필요
            Ok(None)
        }
        // 기타 컨트롤 타입들
        IrControl::Unknown(_) => Ok(None),
    }
}

/// IR 수식 → HWP EquationData 변환
fn convert_equation_to_hwp(equation: &IrEquation) -> Result<EquationData, ConversionError> {
    // 캡션 변환 (있는 경우)
    let caption = equation
        .common
        .caption
        .as_ref()
        .and_then(|c| convert_caption_to_hwp(c).ok());

    Ok(EquationData {
        common: convert_object_common_to_hwp(&equation.common),
        script: equation.script.clone(),
        base_size: equation.font_size.value() as u32,
        caption,
    })
}

/// IR 표 → HWP TableData 변환
fn convert_table_to_hwp(table: &IrTable) -> Result<TableData, ConversionError> {
    // 열 너비 계산
    let column_widths: Vec<u16> = if !table.rows.is_empty() && !table.rows[0].cells.is_empty() {
        table.rows[0]
            .cells
            .iter()
            .map(|cell| cell.width.value() as u16)
            .collect()
    } else {
        vec![4252; table.column_count as usize] // 기본 15mm
    };

    // 셀 변환
    let mut cells = Vec::new();
    for row in &table.rows {
        for cell in &row.cells {
            let cell_data = convert_table_cell_to_hwp(cell)?;
            cells.push(cell_data);
        }
    }

    // 캡션 변환 (있는 경우)
    let caption = table
        .common
        .caption
        .as_ref()
        .and_then(|c| convert_caption_to_hwp(c).ok());

    // 표 속성 비트 필드 생성
    let mut properties = 0u32;

    // bit 0-1: 페이지 나눔 설정
    match table.page_break {
        ir::table::TablePageBreak::None => {
            properties |= 0; // 0 = 나누지 않음
        }
        ir::table::TablePageBreak::Cell => {
            properties |= 1; // 1 = 셀 단위로 나눔
        }
        ir::table::TablePageBreak::Table => {
            // TablePageBreak::Table은 HWP에서 지원하지 않으므로 Cell로 대체
            properties |= 1;
        }
    }

    // bit 2: 제목 줄 자동 반복
    if table.repeat_header {
        properties |= 1 << 2;
    }

    Ok(TableData {
        common: convert_object_common_to_hwp(&table.common),
        rows: table.row_count,
        columns: table.column_count,
        column_widths,
        cells,
        border_fill_id: table
            .border_fill_id
            .map(|id| id.value() as u16)
            .unwrap_or(0),
        caption,
        properties,
    })
}

/// IR 표 셀 → HWP TableCellData 변환
fn convert_table_cell_to_hwp(cell: &IrTableCell) -> Result<TableCellData, ConversionError> {
    // 셀 내용 (문단들) 변환
    let paragraphs: Vec<ParagraphData> = cell
        .paragraphs
        .iter()
        .map(build_paragraph)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(TableCellData {
        col: cell.column,
        row: cell.row,
        col_span: cell.column_span,
        row_span: cell.row_span,
        width: cell.width.value() as u32,
        height: cell.height.value() as u32,
        paragraphs,
        border_fill_id: cell.border_fill_id.map(|id| id.value() as u16).unwrap_or(0),
    })
}

/// 바이너리 데이터 확장자 감지
fn detect_extension(data: &[u8]) -> &'static str {
    if data.starts_with(&[0x89, b'P', b'N', b'G']) {
        "png"
    } else if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        "jpg"
    } else if data.starts_with(b"GIF") {
        "gif"
    } else if data.starts_with(b"BM") {
        "bmp"
    } else if data.starts_with(&[0x00, 0x00, 0x01, 0x00]) {
        "ico"
    } else {
        "dat"
    }
}

// ColorConvert::to_bgr_u32을 사용합니다 (색상 변환 통합)

/// IR ShadowType → HWP 비트 값 (0=없음, 1=discrete, 2=continuous)
fn convert_shadow_type_to_bits(shadow_type: &primitive::ShadowType) -> u32 {
    match shadow_type {
        primitive::ShadowType::None => 0,
        // 기본 방향 (스타일 구분 없음) - discrete로 처리
        primitive::ShadowType::TopLeft
        | primitive::ShadowType::TopRight
        | primitive::ShadowType::BottomLeft
        | primitive::ShadowType::BottomRight => 1,
        // Discrete 스타일 (모든 방향)
        primitive::ShadowType::TopLeftDiscrete
        | primitive::ShadowType::TopRightDiscrete
        | primitive::ShadowType::BottomLeftDiscrete
        | primitive::ShadowType::BottomRightDiscrete => 1,
        // Continuous 스타일 (모든 방향)
        primitive::ShadowType::TopLeftContinuous
        | primitive::ShadowType::TopRightContinuous
        | primitive::ShadowType::BottomLeftContinuous
        | primitive::ShadowType::BottomRightContinuous => 2,
    }
}

/// IR ObjectCommon → HWP ObjectCommonData 변환
fn convert_object_common_to_hwp(common: &IrObjectCommon) -> ObjectCommonData {
    // TextWrap을 properties 비트 필드로 변환
    let properties = convert_text_wrap_to_properties(&common.text_wrap);

    ObjectCommonData {
        properties,
        vertical_offset: common.position.y.value(),
        horizontal_offset: common.position.x.value(),
        width: common.size.width.value(),
        height: common.size.height.value(),
        z_order: common.z_order,
        margin_left: common.margin.left.value() as u16,
        margin_right: common.margin.right.value() as u16,
        margin_top: common.margin.top.value() as u16,
        margin_bottom: common.margin.bottom.value() as u16,
    }
}

/// IR TextWrap → HWP properties 비트 필드로 변환
fn convert_text_wrap_to_properties(text_wrap: &IrTextWrap) -> u32 {
    let mut props: u32 = 0;

    // Bit 0: 글자처럼 취급
    if text_wrap.treat_as_char {
        props |= 1 << 0;
    }

    // Bits 3-4: 세로 기준
    let vertical_rel = match text_wrap.vertical_rel {
        IrVerticalRelativeTo::Paper => 0,
        IrVerticalRelativeTo::Page => 1,
        IrVerticalRelativeTo::Paragraph => 2,
    };
    props |= (vertical_rel & 0x3) << 3;

    // Bits 8-9: 가로 기준
    let horizontal_rel = match text_wrap.horizontal_rel {
        IrHorizontalRelativeTo::Paper => 0,
        IrHorizontalRelativeTo::Page => 1,
        IrHorizontalRelativeTo::Column => 2,
        IrHorizontalRelativeTo::Paragraph => 3,
    };
    props |= (horizontal_rel & 0x3) << 8;

    // Bit 14: 다른 개체 겹침 허용
    if text_wrap.allow_overlap {
        props |= 1 << 14;
    }

    // Bits 21-23: 배치 종류
    let wrap_type = match text_wrap.wrap_type {
        IrTextWrapType::Inline => 0,
        IrTextWrapType::Square => 0,
        IrTextWrapType::Tight => 1,
        IrTextWrapType::Behind => 4,
        IrTextWrapType::InFront => 5,
    };
    props |= (wrap_type & 0x7) << 21;

    // Bits 24-25: 배치 방향
    let wrap_side = match text_wrap.wrap_side {
        IrTextWrapSide::Both => 0,
        IrTextWrapSide::Left => 1,
        IrTextWrapSide::Right => 2,
        IrTextWrapSide::Largest => 3,
    };
    props |= (wrap_side & 0x3) << 24;

    props
}

/// IR Caption → HWP CaptionData 변환
fn convert_caption_to_hwp(caption: &IrCaption) -> Result<HwpCaptionData, ConversionError> {
    // 캡션 방향 변환
    let direction = match caption.position {
        IrCaptionPosition::Bottom => HwpCaptionDirection::Below,
        IrCaptionPosition::Top => HwpCaptionDirection::Above,
        IrCaptionPosition::Left => HwpCaptionDirection::Left,
        IrCaptionPosition::Right => HwpCaptionDirection::Right,
    };

    // 캡션 문단 변환
    let paragraphs: Vec<ParagraphData> = caption
        .paragraphs
        .iter()
        .filter_map(|para| build_paragraph(para).ok())
        .collect();

    Ok(HwpCaptionData {
        direction,
        gap: caption.gap.value(),
        paragraphs,
    })
}

/// IR 그림 → HWP PictureData 변환
fn convert_picture_to_hwp(picture: &IrPicture) -> Result<PictureData, ConversionError> {
    // 바이너리 데이터 ID 파싱
    let binary_id_str = picture.binary_id.value();
    let binary_data_id = binary_id_str.parse::<u16>().unwrap_or_else(|_| {
        // BINxxxx 형식에서 숫자 추출 시도
        binary_id_str
            .strip_prefix("BIN")
            .and_then(|hex_str| u16::from_str_radix(hex_str, 16).ok())
            .unwrap_or(0)
    });

    // 캡션 변환 (있는 경우)
    let caption = picture
        .common
        .caption
        .as_ref()
        .and_then(|c| convert_caption_to_hwp(c).ok());

    Ok(PictureData {
        common: convert_object_common_to_hwp(&picture.common),
        binary_data_id,
        width: picture.original_size.width.value() as u32,
        height: picture.original_size.height.value() as u32,
        caption,
    })
}

/// IR 머리글/바닥글 → HWP HeaderFooterData 변환
fn convert_header_footer_to_hwp(hf: &IrHeaderFooter) -> Result<HeaderFooterData, ConversionError> {
    let apply_to = match hf.apply_to {
        HeaderFooterApplyTo::Both => 0,
        HeaderFooterApplyTo::Even => 1,
        HeaderFooterApplyTo::Odd => 2,
        HeaderFooterApplyTo::First => 0, // HWP doesn't have First, use Both
    };

    let paragraphs: Vec<ParagraphData> = hf
        .paragraphs
        .iter()
        .map(build_paragraph)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(HeaderFooterData {
        apply_to,
        paragraphs,
    })
}

/// IR 각주/미주 → HWP NoteData 변환
fn convert_note_to_hwp(note: &IrNote) -> Result<NoteData, ConversionError> {
    let paragraphs: Vec<ParagraphData> = note
        .paragraphs
        .iter()
        .map(build_paragraph)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(NoteData {
        number: note.number as u16,
        paragraphs,
    })
}

/// IR 하이퍼링크 → HWP HyperlinkData 변환
fn convert_hyperlink_to_hwp(link: &IrHyperlink) -> Result<HyperlinkData, ConversionError> {
    use ir::control::HyperlinkTarget;

    let url = match &link.target {
        HyperlinkTarget::Url(url) => url.clone(),
        HyperlinkTarget::Email(email) => format!("mailto:{}", email),
        HyperlinkTarget::File(path) => path.clone(),
        HyperlinkTarget::Bookmark(name) => format!("#{}", name),
    };

    Ok(HyperlinkData {
        url,
        tooltip: link.tooltip.clone(),
    })
}

/// IR 책갈피 → HWP BookmarkData 변환
fn convert_bookmark_to_hwp(bookmark: &IrBookmark) -> Result<BookmarkData, ConversionError> {
    Ok(BookmarkData {
        name: bookmark.name.clone(),
    })
}

/// IR 자동 번호 → HWP AutoNumberData/PageNumberData 변환
///
/// HWPX PageNumber (위치 정보 포함)는 HWP PageNumberData로 변환하고,
/// 나머지는 HWP AutoNumberData로 변환합니다.
fn convert_auto_number_to_hwp(auto_num: &IrAutoNumber) -> Result<AutoNumberData, ConversionError> {
    let number_type = match auto_num.number_type {
        IrAutoNumberType::Page => 0,
        IrAutoNumberType::Footnote => 1,
        IrAutoNumberType::Endnote => 2,
        IrAutoNumberType::Picture => 3,
        IrAutoNumberType::Table => 4,
        IrAutoNumberType::Equation => 5,
        IrAutoNumberType::TotalPages => 6,
    };

    let number_format = convert_auto_number_format_to_hwp(&auto_num.number_format);

    Ok(AutoNumberData {
        number_type,
        number_format,
    })
}

/// IR 새 번호 → HWP NewNumberData 변환
fn convert_new_number_to_hwp(new_num: &IrNewNumber) -> Result<NewNumberData, ConversionError> {
    let number_type = match new_num.number_type {
        IrAutoNumberType::Page => 0,
        IrAutoNumberType::Footnote => 1,
        IrAutoNumberType::Endnote => 2,
        IrAutoNumberType::Picture => 3,
        IrAutoNumberType::Table => 4,
        IrAutoNumberType::Equation => 5,
        IrAutoNumberType::TotalPages => 6,
    };

    Ok(NewNumberData {
        number_type,
        number: new_num.number as u16,
    })
}

/// IR 숨은 설명 → HWP HiddenCommentData 변환
fn convert_hidden_comment_to_hwp(
    comment: &IrHiddenComment,
) -> Result<HiddenCommentData, ConversionError> {
    let paragraphs: Vec<ParagraphData> = comment
        .paragraphs
        .iter()
        .map(build_paragraph)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(HiddenCommentData { paragraphs })
}

/// IR 번호 형식 → HWP 번호 형식 변환 (자동 번호용)
fn convert_auto_number_format_to_hwp(format: &NumberFormat) -> u16 {
    // HWP 스펙 표 41: 문단 번호 형식
    match format {
        NumberFormat::Digit => 0,              // 1, 2, 3
        NumberFormat::CircledDigit => 1,       // ①, ②, ③
        NumberFormat::RomanUpper => 2,         // I, II, III
        NumberFormat::RomanLower => 3,         // i, ii, iii
        NumberFormat::LatinUpper => 4,         // A, B, C
        NumberFormat::LatinLower => 5,         // a, b, c
        NumberFormat::CircledLatinUpper => 6,  // Ⓐ, Ⓑ, Ⓒ
        NumberFormat::CircledLatinLower => 7,  // ⓐ, ⓑ, ⓒ
        NumberFormat::HangulSyllable => 8,     // 가, 나, 다
        NumberFormat::CircledHangul => 9,      // ㉮, ㉯, ㉰
        NumberFormat::HangulJamo => 10,        // ㄱ, ㄴ, ㄷ
        NumberFormat::CircledHangulJamo => 11, // 원 한글 자모
        NumberFormat::HangulIdeograph => 12,   // 일, 이, 삼
        NumberFormat::Ideograph => 13,         // 一, 二, 三
        NumberFormat::CircledIdeograph => 14,  // 원 한자
        NumberFormat::Ganji => 13,             // 간지 → 한자로 fallback
    }
}

/// IR 도형 → HWP ShapeData 변환
fn convert_shape_to_hwp(shape: &IrShape) -> Result<ShapeData, ConversionError> {
    // 기본 도형 크기 및 위치
    let width = shape.common.size.width.value() as u32;
    let height = shape.common.size.height.value() as u32;
    let offset_x = shape.common.position.x.value();
    let offset_y = shape.common.position.y.value();

    // 선 색상 및 두께
    let line_color = ColorConvert::to_bgr_u32(&shape.line.color);
    let line_thickness = shape.line.width.value() as u16;

    // 채우기 정보 변환
    let fill = convert_fill_to_hwp(&shape.fill);

    // 도형 타입 변환
    let shape_type = convert_shape_type_to_hwp(&shape.shape_type)?;

    // 캡션 변환 (있는 경우)
    let caption = shape
        .common
        .caption
        .as_ref()
        .and_then(|c| convert_caption_to_hwp(c).ok());

    // 변환 행렬 변환 (IR TransformMatrix → HWP 행렬)
    let translation_matrix = shape
        .translation_matrix
        .map(|m| [m.e1, m.e2, m.e3, m.e4, m.e5, m.e6]);
    let scale_matrix = shape
        .scale_matrix
        .map(|m| [m.e1, m.e2, m.e3, m.e4, m.e5, m.e6]);
    let rotation_matrix = shape
        .rotation_matrix
        .map(|m| [m.e1, m.e2, m.e3, m.e4, m.e5, m.e6]);

    Ok(ShapeData {
        common: convert_object_common_to_hwp(&shape.common),
        shape_type,
        width,
        height,
        offset_x,
        offset_y,
        line_color,
        line_thickness,
        fill,
        caption,
        translation_matrix,
        scale_matrix,
        rotation_matrix,
        rotation: shape.rotation,
    })
}

/// IR Fill → HWP FillInfo 변환
fn convert_fill_to_hwp(fill: &ir::border_fill::Fill) -> HwpFillInfo {
    match fill {
        ir::border_fill::Fill::None => HwpFillInfo::None,
        ir::border_fill::Fill::Solid(solid) => {
            // Solid fill을 Pattern으로 변환 (HWP는 Solid를 패턴의 None으로 표현)
            HwpFillInfo::Pattern(HwpPatternFill {
                background_color: ColorConvert::from_ir(&solid.color),
                pattern_color: ColorConvert::from_ir(&solid.color),
                pattern_type: HwpPatternType::None,
            })
        }
        ir::border_fill::Fill::Gradient(gradient) => {
            // 그라데이션 변환 - 모든 색상 정보 보존
            let gradient_type = match gradient.gradient_type {
                primitive::GradientType::Linear => HwpGradientType::Linear,
                primitive::GradientType::Radial => HwpGradientType::Radial,
                primitive::GradientType::Conical => HwpGradientType::Conical,
                primitive::GradientType::Square => HwpGradientType::Rectangular,
            };

            let colors: Vec<ColorReference> = gradient
                .stops
                .iter()
                .map(|stop| ColorConvert::from_ir(&stop.color))
                .collect();

            HwpFillInfo::Gradient(HwpGradientFill {
                gradient_type,
                // IR angle은 u16 (0-360), HWP angle은 i16 (-180~180)
                // 180도 초과는 음수로 변환
                angle: {
                    let a = gradient.angle as i32;
                    if a > 180 { (a - 360) as i16 } else { a as i16 }
                },
                center_x: gradient.center_x as i16,
                center_y: gradient.center_y as i16,
                blur: gradient.blur as i16,
                colors,
            })
        }
        ir::border_fill::Fill::Pattern(pattern) => {
            let pattern_type = match pattern.pattern_type {
                ir::border_fill::PatternType::None => HwpPatternType::None,
                ir::border_fill::PatternType::Horizontal => HwpPatternType::Horizontal,
                ir::border_fill::PatternType::Vertical => HwpPatternType::Vertical,
                ir::border_fill::PatternType::BackSlash => HwpPatternType::BackSlash,
                ir::border_fill::PatternType::Slash => HwpPatternType::Slash,
                ir::border_fill::PatternType::Cross => HwpPatternType::Cross,
                ir::border_fill::PatternType::CrossDiagonal => HwpPatternType::CrossDiagonal,
            };

            HwpFillInfo::Pattern(HwpPatternFill {
                background_color: ColorConvert::from_ir(&pattern.background),
                pattern_color: ColorConvert::from_ir(&pattern.foreground),
                pattern_type,
            })
        }
        ir::border_fill::Fill::Image(image) => {
            // IR의 ImageFillMode → HWP의 ImageFillType 변환
            let fill_type = match image.mode {
                primitive::ImageFillMode::Tile => HwpImageFillType::TileAll,
                primitive::ImageFillMode::TileHorizontalTop => HwpImageFillType::TileHorizontalTop,
                primitive::ImageFillMode::TileHorizontalBottom => {
                    HwpImageFillType::TileHorizontalBottom
                }
                primitive::ImageFillMode::TileVerticalLeft => HwpImageFillType::TileVerticalLeft,
                primitive::ImageFillMode::TileVerticalRight => HwpImageFillType::TileVerticalRight,
                primitive::ImageFillMode::Stretch => HwpImageFillType::FitToSize,
                primitive::ImageFillMode::Center => HwpImageFillType::Center,
                primitive::ImageFillMode::CenterTop => HwpImageFillType::CenterTop,
                primitive::ImageFillMode::CenterBottom => HwpImageFillType::CenterBottom,
                primitive::ImageFillMode::CenterLeft => HwpImageFillType::CenterLeft,
                primitive::ImageFillMode::TopLeft => HwpImageFillType::TopLeft,
                primitive::ImageFillMode::BottomLeft => HwpImageFillType::BottomLeft,
                primitive::ImageFillMode::CenterRight => HwpImageFillType::CenterRight,
                primitive::ImageFillMode::TopRight => HwpImageFillType::TopRight,
                primitive::ImageFillMode::BottomRight => HwpImageFillType::BottomRight,
                primitive::ImageFillMode::Original => HwpImageFillType::None,
            };

            let effect = match image.effect {
                primitive::ImageEffect::Original => 0,
                primitive::ImageEffect::Grayscale => 1,
                primitive::ImageEffect::BlackWhite => 2,
                primitive::ImageEffect::Pattern => 3,
            };

            HwpFillInfo::Image(HwpImageFill {
                fill_type,
                image_info: HwpImageInfo {
                    brightness: image.brightness,
                    contrast: image.contrast,
                    effect,
                    binary_data_id: image.binary_id.value().parse::<u16>().unwrap_or(0),
                },
            })
        }
    }
}

// ColorConvert::from_ir을 사용합니다 (색상 변환 통합)

/// IR Fill → HWP (fill_type, background_color) 변환 (BorderFillData용)
fn convert_fill_to_hwp_simple(fill: &ir::border_fill::Fill) -> (u32, u32) {
    match fill {
        ir::border_fill::Fill::None => (0, 0xFFFFFF),
        ir::border_fill::Fill::Solid(solid) => {
            // fill_type bit 0 = 단색 채우기
            // HWP는 0x00RRGGBB 형식 (알파 없음)
            (1, solid.color.to_rgb_u32())
        }
        ir::border_fill::Fill::Gradient(gradient) => {
            // fill_type bit 2 = 그라데이션
            // HWP는 0x00RRGGBB 형식 (알파 없음)
            let bg = gradient
                .stops
                .first()
                .map(|s| s.color.to_rgb_u32())
                .unwrap_or(0xFFFFFF);
            (4, bg)
        }
        ir::border_fill::Fill::Image(_) => {
            // fill_type bit 1 = 이미지 채우기
            (2, 0xFFFFFF)
        }
        ir::border_fill::Fill::Pattern(pattern) => {
            // fill_type bit 0 = 단색 (패턴은 HWP에서 별도 처리)
            // HWP는 0x00RRGGBB 형식 (알파 없음)
            (1, pattern.background.to_rgb_u32())
        }
    }
}

/// IR 도형 타입 → HWP ShapeTypeData 변환
fn convert_shape_type_to_hwp(shape_type: &IrShapeType) -> Result<ShapeTypeData, ConversionError> {
    match shape_type {
        IrShapeType::Line(line) => Ok(ShapeTypeData::Line(LineShapeData {
            start_x: line.start.x.value(),
            start_y: line.start.y.value(),
            end_x: line.end.x.value(),
            end_y: line.end.y.value(),
            start_arrow_type: convert_arrow_type_with_filled(
                &line.start_arrow.arrow_type,
                line.start_arrow.filled,
            ),
            start_arrow_size: convert_arrow_size(&line.start_arrow.size),
            end_arrow_type: convert_arrow_type_with_filled(
                &line.end_arrow.arrow_type,
                line.end_arrow.filled,
            ),
            end_arrow_size: convert_arrow_size(&line.end_arrow.size),
        })),
        IrShapeType::Rectangle(rect) => Ok(ShapeTypeData::Rectangle(RectangleShapeData {
            corner_radius: rect.corner_radius.value() as u32,
        })),
        IrShapeType::Ellipse(ellipse) => Ok(ShapeTypeData::Ellipse(EllipseShapeData {
            arc_type: convert_arc_type(&ellipse.arc_type),
            start_angle: (ellipse.start_angle * 100.0) as i32,
            end_angle: (ellipse.end_angle * 100.0) as i32,
        })),
        IrShapeType::Arc(arc) => Ok(ShapeTypeData::Arc(ArcShapeData {
            arc_type: convert_arc_type(&arc.arc_type),
            start_angle: (arc.start_angle * 100.0) as i32,
            end_angle: (arc.end_angle * 100.0) as i32,
        })),
        IrShapeType::Polygon(poly) => {
            let points = poly
                .points
                .iter()
                .map(|p| (p.x.value(), p.y.value()))
                .collect();
            Ok(ShapeTypeData::Polygon(PolygonShapeData { points }))
        }
        IrShapeType::Curve(curve) => {
            let points = curve
                .points
                .iter()
                .map(|p| {
                    (
                        p.point.x.value(),
                        p.point.y.value(),
                        convert_curve_point_type(&p.point_type),
                    )
                })
                .collect();
            Ok(ShapeTypeData::Curve(CurveShapeData {
                points,
                closed: curve.closed,
            }))
        }
        IrShapeType::Connector(connector) => {
            use crate::writer::body_writer::ConnectorShapeData;

            // ConnectorType을 HWP line_type으로 변환
            let line_type = match connector.connector_type {
                ir::shape::ConnectorType::Straight => 0, // 직선
                ir::shape::ConnectorType::Elbow
                | ir::shape::ConnectorType::VerticalHorizontal
                | ir::shape::ConnectorType::HorizontalVertical => 1, // 꺾인선
                ir::shape::ConnectorType::Curved => 2,   // 곡선
            };

            // 제어점 변환
            let control_points: Vec<(i32, i32)> = connector
                .control_points
                .iter()
                .map(|cp| (cp.point.x.value(), cp.point.y.value()))
                .collect();

            Ok(ShapeTypeData::Connector(ConnectorShapeData {
                line_type,
                start_x: connector.start.point.x.value(),
                start_y: connector.start.point.y.value(),
                start_subject_id: connector.start.subject_id_ref,
                start_subject_index: connector.start.subject_index,
                end_x: connector.end.point.x.value(),
                end_y: connector.end.point.y.value(),
                end_subject_id: connector.end.subject_id_ref,
                end_subject_index: connector.end.subject_index,
                control_points,
                start_arrow_type: convert_arrow_type_with_filled(
                    &connector.start_arrow.arrow_type,
                    connector.start_arrow.filled,
                ),
                start_arrow_size: convert_arrow_size(&connector.start_arrow.size),
                end_arrow_type: convert_arrow_type_with_filled(
                    &connector.end_arrow.arrow_type,
                    connector.end_arrow.filled,
                ),
                end_arrow_size: convert_arrow_size(&connector.end_arrow.size),
            }))
        }
        IrShapeType::Group(shapes) => {
            // 그룹 도형의 모든 자식을 재귀적으로 변환
            let children: Vec<ShapeData> = shapes
                .iter()
                .filter_map(|child_shape| convert_shape_to_hwp(child_shape).ok())
                .collect();

            Ok(ShapeTypeData::Container(ContainerShapeData { children }))
        }
    }
}

/// IR 화살표 → HWP 화살표 타입 (filled 속성 반영)
fn convert_arrow_type_with_filled(arrow_type: &primitive::ArrowType, filled: bool) -> u8 {
    match arrow_type {
        primitive::ArrowType::None => 0,
        primitive::ArrowType::Arrow => 1,     // Arrow (채워진)
        primitive::ArrowType::ArrowOpen => 2, // Spear (열린)
        primitive::ArrowType::Stealth => 3,   // ConcaveArrow
        primitive::ArrowType::Diamond => {
            if filled {
                7
            } else {
                4
            }
        } // FilledDiamond : EmptyDiamond
        primitive::ArrowType::Circle => {
            if filled {
                8
            } else {
                5
            }
        } // FilledCircle : EmptyCircle
        primitive::ArrowType::Square => {
            if filled {
                9
            } else {
                6
            }
        } // FilledBox : EmptyBox
    }
}

/// IR 화살표 크기 → HWP 화살표 크기
fn convert_arrow_size(size: &primitive::ArrowSize) -> u8 {
    match size {
        primitive::ArrowSize::Small => 0,
        primitive::ArrowSize::Medium => 1,
        primitive::ArrowSize::Large => 2,
    }
}

/// IR 호 타입 → HWP 호 타입
fn convert_arc_type(arc_type: &IrArcType) -> u8 {
    match arc_type {
        IrArcType::Full => 0,
        IrArcType::Arc => 1,
        IrArcType::Pie => 2,
        IrArcType::Chord => 3,
    }
}

/// IR 곡선 점 타입 → HWP 곡선 점 타입
fn convert_curve_point_type(pt_type: &IrCurvePointType) -> u8 {
    match pt_type {
        IrCurvePointType::Normal => 0,
        IrCurvePointType::Control1 => 1,
        IrCurvePointType::Control2 => 2,
    }
}

/// IR 텍스트 박스 → HWP TextBoxData 변환
fn convert_textbox_to_hwp(text_box: &IrTextBox) -> Result<TextBoxData, ConversionError> {
    // 문단들 변환
    let paragraphs: Vec<ParagraphData> = text_box
        .paragraphs
        .iter()
        .map(build_paragraph)
        .collect::<Result<Vec<_>, _>>()?;

    // 크기 추출
    let width = text_box.common.size.width.value().max(0) as u32;
    let height = text_box.common.size.height.value().max(0) as u32;

    Ok(TextBoxData {
        common: convert_object_common_to_hwp(&text_box.common),
        width,
        height,
        paragraphs,
    })
}

/// IR BorderFill → HWP BorderFillData 변환
fn convert_border_fill_to_hwp(border_fill: &IrBorderFill) -> BorderFillData {
    // 속성 계산: 3D 효과(bit 0), 그림자(bit 1)
    let mut properties: u16 = 0;
    if border_fill.is_3d {
        properties |= 1;
    }
    if border_fill.has_shadow {
        properties |= 2;
    }

    // 대각선 처리
    let diagonal = if let Some(ref diag) = border_fill.diagonal_down {
        convert_border_to_line(diag)
    } else if let Some(ref diag) = border_fill.diagonal_up {
        convert_border_to_line(diag)
    } else {
        BorderLine::default()
    };

    // 대각선 타입 설정 (properties에 포함)
    // bit 2-3: 대각선 타입 (0=없음, 1=왼쪽위→오른쪽아래, 2=오른쪽위→왼쪽아래, 3=십자)
    if border_fill.diagonal_down.is_some() && border_fill.diagonal_up.is_some() {
        properties |= 3 << 2; // 십자
    } else if border_fill.diagonal_down.is_some() {
        properties |= 1 << 2; // 왼쪽위→오른쪽아래
    } else if border_fill.diagonal_up.is_some() {
        properties |= 2 << 2; // 오른쪽위→왼쪽아래
    }

    // 채우기 타입 및 배경색
    let (fill_type, background_color) = convert_fill_to_hwp_simple(&border_fill.fill);

    BorderFillData {
        properties,
        left_border: convert_border_to_line(&border_fill.left),
        right_border: convert_border_to_line(&border_fill.right),
        top_border: convert_border_to_line(&border_fill.top),
        bottom_border: convert_border_to_line(&border_fill.bottom),
        diagonal,
        fill_type,
        background_color,
    }
}

/// IR Border → HWP BorderLine 변환
fn convert_border_to_line(border: &IrBorder) -> BorderLine {
    BorderLine {
        style: convert_line_type_to_hwp(&border.line_type),
        thickness: convert_line_width_to_thickness(border.width.value()),
        // HWP는 0x00RRGGBB 형식 (알파 없음)
        color: border.color.to_rgb_u32(),
    }
}

/// IR LineType → HWP 선 스타일 변환
fn convert_line_type_to_hwp(line_type: &IrLineType) -> u8 {
    match line_type {
        IrLineType::None => 0,
        IrLineType::Solid => 1,
        IrLineType::Dash => 2,
        IrLineType::Dot => 3,
        IrLineType::DashDot => 4,
        IrLineType::DashDotDot => 5,
        IrLineType::LongDash => 6,
        IrLineType::Double => 7,
        IrLineType::Triple => 8,
        IrLineType::Wave => 9,
        IrLineType::DoubleWave => 10,
        IrLineType::ThickThinLarge => 11,
        IrLineType::ThinThickLarge => 12,
        IrLineType::Circle => 13,
    }
}

/// 선 두께(HwpUnit) → HWP 두께 코드 변환
fn convert_line_width_to_thickness(width: i32) -> u8 {
    // HWP 두께 코드: 0=0.1mm, 1=0.12mm, 2=0.15mm, 3=0.2mm, 4=0.25mm,
    // 5=0.3mm, 6=0.4mm, 7=0.5mm, 8=0.6mm, 9=0.7mm, 10=1.0mm, 11=1.5mm, 12=2.0mm 등
    // HwpUnit 100 = 1mm
    match width {
        0..=9 => 0,      // ~0.1mm
        10..=11 => 1,    // ~0.12mm
        12..=14 => 2,    // ~0.15mm
        15..=22 => 3,    // ~0.2mm
        23..=27 => 4,    // ~0.25mm
        28..=34 => 5,    // ~0.3mm
        35..=44 => 6,    // ~0.4mm
        45..=54 => 7,    // ~0.5mm
        55..=64 => 8,    // ~0.6mm
        65..=84 => 9,    // ~0.7mm
        85..=124 => 10,  // ~1.0mm
        125..=174 => 11, // ~1.5mm
        _ => 12,         // 2.0mm 이상
    }
}

/// IR TabDef → HWP TabDefinitionData 변환
fn convert_tab_def_to_hwp(tab_def: &IrTabDef) -> TabDefinitionData {
    // 속성: 자동 탭 간격이 있으면 bit 0 설정
    let properties = if tab_def.auto_tab_interval.is_some() {
        // 자동 탭 간격 활성화
        0
    } else {
        // 자동 탭 비활성화
        1
    };

    let items: Vec<TabItem> = tab_def
        .tabs
        .iter()
        .map(|tab| TabItem {
            position: tab.position.value().max(0) as u32,
            tab_type: convert_tab_type_to_hwp(&tab.tab_type),
            fill_char: convert_tab_leader_to_fill_char(&tab.leader),
        })
        .collect();

    TabDefinitionData { properties, items }
}

/// IR TabType → HWP 탭 종류 변환
fn convert_tab_type_to_hwp(tab_type: &IrTabType) -> u8 {
    match tab_type {
        IrTabType::Left => 0,
        IrTabType::Center => 1,
        IrTabType::Right => 2,
        IrTabType::Decimal => 3,
    }
}

/// IR TabLeader → HWP 채움 종류 변환
/// HWP 명세: 채움 종류는 테두리선 종류(표 25)를 참조
/// 0: 실선, 1: 긴 점선, 2: 점선, 3: -.-.-., 4: -..-..-, 5: Dash보다 긴 선분, 6: Dot보다 큰 동그라미
fn convert_tab_leader_to_fill_char(leader: &IrTabLeader) -> u16 {
    match leader {
        IrTabLeader::None => 0,       // 없음 (실선으로 표시되지만 실제로는 채움 없음)
        IrTabLeader::Dot => 2,        // 점선
        IrTabLeader::LongDash => 1,   // 긴 점선
        IrTabLeader::Dash => 3,       // -.-.-. (파선)
        IrTabLeader::Underscore => 0, // 밑줄 (실선)
    }
}

/// IR Numbering → HWP NumberingData 변환
fn convert_numbering_to_hwp(numbering: &ir::style::Numbering) -> NumberingData {
    let levels: Vec<NumberingLevelData> = numbering
        .levels
        .iter()
        .map(|level| {
            // Alignment 변환 (IR → HWP: Left=0, Center=1, Right=2)
            let alignment = match level.alignment {
                primitive::Alignment::Left => 0,
                primitive::Alignment::Center => 1,
                primitive::Alignment::Right => 2,
                primitive::Alignment::Justify
                | primitive::Alignment::Distribute
                | primitive::Alignment::Divide => 0,
            };

            // NumberFormat 변환 (IR → HWP)
            let number_format = match level.number_format {
                primitive::NumberFormat::Digit => 0,
                primitive::NumberFormat::CircledDigit => 1,
                primitive::NumberFormat::RomanUpper => 2,
                primitive::NumberFormat::RomanLower => 3,
                primitive::NumberFormat::LatinUpper => 4,
                primitive::NumberFormat::LatinLower => 5,
                primitive::NumberFormat::CircledLatinUpper => 6,
                primitive::NumberFormat::CircledLatinLower => 7,
                primitive::NumberFormat::HangulSyllable => 8,
                primitive::NumberFormat::CircledHangul => 9,
                primitive::NumberFormat::HangulJamo => 10,
                primitive::NumberFormat::CircledHangulJamo => 11,
                primitive::NumberFormat::HangulIdeograph => 12,
                primitive::NumberFormat::Ideograph => 13,
                primitive::NumberFormat::CircledIdeograph => 14,
                primitive::NumberFormat::Ganji => 15,
            };

            NumberingLevelData {
                alignment,
                use_instance_width: level.use_instance_width,
                auto_indent: level.auto_indent,
                number_format,
                width_correction: level.number_width as i16,
                text_distance: level.text_offset as i16,
                char_shape_id: level.char_shape_id.map(|id| id.value()).unwrap_or(0),
                format: level.format.clone(),
                start_number: level.start_number,
            }
        })
        .collect();

    NumberingData {
        levels,
        start_number: numbering.start_number as u16,
    }
}

/// IR Bullet → HWP BulletData 변환
fn convert_bullet_to_hwp(bullet: &ir::style::Bullet) -> BulletData {
    let para_head_info = bullet.char_shape_id.map(|id| id.value()).unwrap_or(0);

    BulletData {
        para_head_info,
        bullet_char: bullet.char,
    }
}

/// BinaryDataId에서 숫자 ID 추출 (BIN{XXXX} 형식에서 숫자 추출)
fn extract_bin_data_id(id: &primitive::BinaryDataId) -> u16 {
    let value = id.value();
    // "BIN{XXXX}" 형식에서 숫자 부분 추출
    if let Some(hex_part) = value.strip_prefix("BIN") {
        u16::from_str_radix(hex_part, 16).unwrap_or(0)
    } else {
        // 단순 숫자인 경우
        value.parse::<u16>().unwrap_or(0)
    }
}

/// IR Video → HWP VideoData 변환
fn convert_video_to_hwp(video: &IrVideo) -> Result<HwpVideoData, ConversionError> {
    // 비디오 종류 변환
    let video_type = match video.video_type {
        IrVideoType::Embedded => 0,
        IrVideoType::Linked => 1,
        IrVideoType::YouTube | IrVideoType::Web => 2,
    };

    // 바이너리 데이터 ID (임베디드인 경우)
    let bin_data_id = video
        .video_id
        .as_ref()
        .map(extract_bin_data_id)
        .unwrap_or(0);

    // 포스터 바이너리 데이터 ID (poster_binary_id 우선, 없으면 preview_image_id 사용)
    let poster_bin_id = video
        .poster_binary_id
        .as_ref()
        .map(extract_bin_data_id)
        .or_else(|| video.preview_image_id.as_ref().map(extract_bin_data_id))
        .unwrap_or(0);

    // 소스 URL (링크인 경우)
    let source_url = video.source_url.clone().unwrap_or_default();

    // 비디오 너비와 높이
    let width = video.width.map(|w| w.value() as u32).unwrap_or(0);
    let height = video.height.map(|h| h.value() as u32).unwrap_or(0);

    Ok(HwpVideoData {
        common: convert_object_common_to_hwp(&video.common),
        video_type,
        bin_data_id,
        poster_bin_id,
        source_url,
        width,
        height,
    })
}

/// IR OleObject → HWP OleData 변환
fn convert_ole_to_hwp(ole: &IrOleObject) -> Result<HwpOleData, ConversionError> {
    let bin_data_id = extract_bin_data_id(&ole.binary_id);

    Ok(HwpOleData {
        common: convert_object_common_to_hwp(&ole.common),
        properties: 0,
        extent_width: ole.common.size.width.value(),
        extent_height: ole.common.size.height.value(),
        bin_data_id,
        border_color: 0,
        border_thickness: 0,
    })
}

/// IR Chart → HWP ChartData 변환
fn convert_chart_to_hwp(chart: &IrChart) -> Result<HwpChartData, ConversionError> {
    let chart_type = match chart.chart_type {
        IrChartType::Bar | IrChartType::Column => 0,
        IrChartType::Line | IrChartType::Stock => 1,
        IrChartType::Pie | IrChartType::Doughnut => 2,
        IrChartType::Area | IrChartType::Surface => 3,
        IrChartType::Scatter | IrChartType::Bubble => 4,
        IrChartType::Radar => 6,
    };

    Ok(HwpChartData {
        common: convert_object_common_to_hwp(&chart.common),
        chart_type,
    })
}

/// IR FormObject → HWP FormObjectData 변환
/// IR FormObject → HWP FormObjectData 변환
fn convert_form_object_to_hwp(form: &IrFormObject) -> Result<HwpFormObjectData, ConversionError> {
    let form_type = match form.form_type {
        IrFormObjectType::Button | IrFormObjectType::Signature => 5,
        IrFormObjectType::CheckBox => 1,
        IrFormObjectType::RadioButton => 2,
        IrFormObjectType::ComboBox => 3,
        IrFormObjectType::ListBox => 4,
        IrFormObjectType::Edit => 0,
        IrFormObjectType::ScrollBar => 6,
    };

    // Build properties flags
    let mut properties = form_type as u32;

    // Button properties
    if form.tri_state {
        properties |= 0x0100;
    }
    if form.gradient_fill {
        properties |= 0x0200;
    }
    if form.image_fill {
        properties |= 0x0400;
    }

    // Back style (bits 10-11)
    if let Some(back_style) = &form.back_style {
        let style_val = match back_style {
            ir::control::ButtonBackStyle::Transparent => 0,
            ir::control::ButtonBackStyle::Opaque => 1,
        };
        properties |= (style_val as u32) << 10;
    }

    // Edit properties
    if form.multiline {
        properties |= 0x0100;
    }
    if form.num_only {
        properties |= 0x0200;
    }
    if form.read_only {
        properties |= 0x0400;
    }

    // Scroll bars (bits 12-13)
    if let Some(scroll_bars) = &form.scroll_bars {
        let scroll_val = match scroll_bars {
            ir::control::EditScrollBars::None => 0,
            ir::control::EditScrollBars::Vertical => 1,
            ir::control::EditScrollBars::Horizontal => 2,
            ir::control::EditScrollBars::Both => 3,
        };
        properties |= (scroll_val as u32) << 12;
    }

    // Tab key behavior (bit 14)
    if let Some(tab_key) = &form.tab_key_behavior {
        let tab_val = match tab_key {
            ir::control::EditTabKeyBehavior::NextObject => 0,
            ir::control::EditTabKeyBehavior::InsertTab => 1,
        };
        properties |= (tab_val as u32) << 14;
    }

    // Alignment (bits 16-17)
    if let Some(alignment) = &form.alignment {
        let align_val = match alignment {
            ir::control::EditTextAlignment::Left => 0,
            ir::control::EditTextAlignment::Center => 1,
            ir::control::EditTextAlignment::Right => 2,
        };
        properties |= (align_val as u32) << 16;
    }

    // ComboBox/ListBox properties
    if form.edit_enable {
        properties |= 0x0100;
    }

    // Convert back_color from IR Color to HWP BGR format
    let back_color = form.back_color.as_ref().map(|color| {
        ((color.blue as u32) << 16) | ((color.green as u32) << 8) | (color.red as u32)
    });

    // Convert password_char
    let password_char = form
        .password_char
        .as_ref()
        .and_then(|s| s.chars().next().map(|c| c as u16));

    // Build items lists
    let (items_text, items_value): (Vec<String>, Vec<String>) = form
        .items
        .iter()
        .map(|item| {
            let text = item.display_text.clone().unwrap_or_default();
            let value = item.value.clone().unwrap_or_default();
            (text, value)
        })
        .unzip();

    Ok(HwpFormObjectData {
        common: convert_object_common_to_hwp(&form.common),
        form_type,
        name: form.name.clone().unwrap_or_default(),
        value: form.value.clone().unwrap_or_default(),
        properties,
        caption: form.caption.clone(),
        radio_group_name: form.radio_group_name.clone(),
        back_color,
        password_char,
        max_length: form.max_length,
        selected_value: form.selected_value.clone(),
        list_box_rows: form.list_box_rows,
        list_box_width: form.list_box_width,
        item_height: form.item_height,
        top_index: form.top_index,
        items_text,
        items_value,
        min: form.min,
        max: form.max,
        scroll_value: form.scroll_value,
        small_change: form.small_change,
        large_change: form.large_change,
        page: form.page,
        delay: form.delay,
    })
}

/// IR TextArt → HWP TextArtData 변환
fn convert_text_art_to_hwp(text_art: &IrTextArt) -> Result<HwpTextArtData, ConversionError> {
    let font_style = match text_art.font_style {
        IrTextArtFontStyle::Regular => 0,
        IrTextArtFontStyle::Bold => 1,
        IrTextArtFontStyle::Italic => 2,
        IrTextArtFontStyle::BoldItalic => 3,
    };

    let shape_type = match text_art.shape {
        IrTextArtShapeType::Rectangle => 0,
        IrTextArtShapeType::Circle => 1,
        IrTextArtShapeType::ArchUp => 2,
        IrTextArtShapeType::ArchDown => 3,
        IrTextArtShapeType::Wave => 4,
        IrTextArtShapeType::Cylinder => 5,
        IrTextArtShapeType::Inflate => 6,
        IrTextArtShapeType::Deflate => 7,
        IrTextArtShapeType::Other(n) => n as u8,
    };

    let alignment = match text_art.alignment {
        IrTextArtAlignment::Left => 0,
        IrTextArtAlignment::Center => 1,
        IrTextArtAlignment::Right => 2,
        IrTextArtAlignment::Full => 3,
    };

    Ok(HwpTextArtData {
        common: convert_object_common_to_hwp(&text_art.common),
        text: text_art.text.clone(),
        font_name: text_art.font_name.clone(),
        font_style,
        shape_type,
        line_spacing: text_art.line_spacing,
        char_spacing: text_art.char_spacing,
        alignment,
        width: text_art.common.size.width.value() as u32,
        height: text_art.common.size.height.value() as u32,
    })
}
