//! IR → HWPX 변환
//!
//! IR 문서를 HWPX로 변환합니다.
//! 스타일 정보(폰트, 글자 모양, 문단 모양, 스타일)를 모두 HWPX로 변환합니다.

use crate::Document as HwpxDocument;
use crate::core::enums::{LineStyleType1, LineStyleType2};
use crate::core::types::BeginIdRef;
use crate::core::types::RgbColor;
use crate::header::Head;
use crate::header::begin_number::BeginNumber;
use crate::header::bullet::{Bullet as HwpxBullet, BulletList};
use crate::header::character_shape::{
    CharacterOutline, CharacterShadow, CharacterShape as HwpxCharShape, CharacterShapeList,
    CharacterStrikeout, CharacterUnderline, EmphasisMarkType, LanguageFontReference,
    LanguageOffset, LanguageRatio, LanguageRelativeSize, LanguageSpacing,
    ShadowType as HwpxShadowType, UnderlinePosition as HwpxUnderlinePosition,
};
use crate::header::font::{Font as HwpxFont, FontLanguage, FontType, Fontface, FontfaceList};
use crate::header::mapping_table::MappingTable;
use crate::header::numbering::{Numbering as HwpxNumbering, NumberingList};
use crate::header::paragraph_head::ParagraphHead;
use crate::header::paragraph_shape::{
    HeadingType, HorizontalAlignment, LatinWordBreak, LineSpacing as HwpxLineSpacing,
    LineSpacingType as HwpxLineSpacingType, LineSpacingUnit, LineWrapType, NonLatinWordBreak,
    ParagraphAlignment, ParagraphAutoSpacing, ParagraphBorder, ParagraphBreakSetting,
    ParagraphHeading, ParagraphMargin, ParagraphShape as HwpxParaShape, ParagraphShapeList,
    VerticalAlignment as HwpxVerticalAlignment,
};
use crate::header::style::{Style as HwpxStyle, StyleKind, StyleList};
use crate::paragraph::column::{ColumnDefinition as HwpxColumnDef, ColumnLine, ColumnSize};
use crate::paragraph::control::{
    Control as HwpxControl, ControlItem, ControlItem as HwpxControlItem,
    FieldBegin as HwpxFieldBegin, FieldEnd as HwpxFieldEnd,
};
use crate::paragraph::enums::FieldType as HwpxFieldType;
use crate::paragraph::enums::{
    ColumnLayout, ColumnType, EndnoteNumberingType, FootnoteNumberingType, GutterType,
    PageBorderPosition as HwpxPageBorderPosition, PaperOrientation,
};
use crate::paragraph::para_list::ParagraphList;
use crate::paragraph::section_definition::{
    AutoNumberFormat, EndnoteNumbering, EndnotePlacementSettings, EndnoteShape, FootnoteNumbering,
    FootnotePlacementSettings, FootnoteShape, NoteLine, NoteSpacing,
    PageBorderFill as HwpxPageBorderFill, PageBorderOffset, PageMargin, PageProperty,
    SectionDefinition, SectionStartNumber,
};
use crate::paragraph::shape_common::{
    HorizontalRelativeTo as HwpxHorzRelTo, ShapeObjectPosition, ShapeObjectSize,
    VerticalRelativeTo as HwpxVertRelTo,
};
use crate::paragraph::table::{
    CellAddress, CellMargin, CellSize, CellSpan, CellZone, CellZoneList, Table as HwpxTable,
    TableCell as HwpxTableCell, TableRow as HwpxTableRow,
};
use crate::paragraph::{
    LineSegment as HwpxLineSegment, LineSegmentArray, Paragraph as HwpxParagraph, Run as HwpxRun,
    RunContent as HwpxRunContent, Section as HwpxSection, TextElement, TextMarkup,
};
use crate::paragraph::{TextFlowMode, TextWrapMode};
use crate::version::{HcfVersion, TargetApplication};
use ir::{
    ConversionError, ConversionResult, Document as IrDocument, RunContent as IrRunContent,
    char_shape::{CharShape as IrCharShape, Font as IrFont},
    control::{
        AutoNumber as IrAutoNumber, AutoNumberType as IrAutoNumberType, Bookmark as IrBookmark,
        Chart as IrChart, Control as IrControl, Equation as IrEquation, FormObject as IrFormObject,
        FormObjectType as IrFormObjectType, HeaderFooterControl as IrHeaderFooter,
        HiddenComment as IrHiddenComment, Hyperlink as IrHyperlink, HyperlinkTarget,
        NewNumber as IrNewNumber, Note as IrNote, ObjectCommon as IrObjectCommon,
        OleObject as IrOleObject, TextWrap as IrTextWrap, Video as IrVideo,
        VideoType as IrVideoType,
    },
    extensions::HwpxExtensions as IrHwpxExtensions,
    para_shape::{
        LineSpacingType as IrLineSpacingType, LineSpacingValue, ParaShape as IrParaShape,
    },
    paragraph::{
        FieldParameter as IrFieldParameter, FieldParameters as IrFieldParameters,
        FieldStart as IrFieldStart,
    },
    picture::Picture as IrPicture,
    shape::{ArcType as IrArcType, Shape as IrShape, ShapeType as IrShapeType},
    style::{Bullet as IrBullet, Numbering as IrNumbering, Style as IrStyle, StyleStore},
    table::{Table as IrTable, TableCell as IrTableCell, TableZone as IrTableZone},
};
use primitive::{
    Alignment as IrAlignment, BreakType, EmphasisType as IrEmphasisType, FieldType as IrFieldType,
    HeaderFooterApplyTo, HorizontalRelativeTo as IrHorizontalRelativeTo,
    ImageEffect as IrImageEffect, ImageFlip as IrImageFlip, LineBreakKorean, LineBreakLatin,
    NumberFormat as IrNumberFormat, OutlineType as IrOutlineType, ShadowType as IrShadowType,
    StrikethroughType as IrStrikethroughType, StyleType as IrStyleType,
    TextWrapSide as IrTextWrapSide, TextWrapType as IrTextWrapType,
    UnderlinePosition as IrUnderlinePosition, UnderlineType as IrUnderlineType,
    VerticalAlignment as IrVerticalAlignment, VerticalRelativeTo as IrVerticalRelativeTo,
};

use super::FromIrContext;

/// IR → HWPX 변환 트레이트
pub trait IrToHwpx {
    /// HWPX 문서로 변환
    fn to_hwpx(&self) -> Result<ConversionResult<HwpxDocument>, ConversionError>;
}

impl IrToHwpx for IrDocument {
    fn to_hwpx(&self) -> Result<ConversionResult<HwpxDocument>, ConversionError> {
        let mut ctx = FromIrContext::new();
        let doc = convert_document(self, &mut ctx)?;
        Ok(ctx.warnings.into_result(doc))
    }
}

/// 문서 변환
fn convert_document(
    ir: &IrDocument,
    ctx: &mut FromIrContext,
) -> Result<HwpxDocument, ConversionError> {
    // 버전 정보 변환
    let version = if let Some(ref ver) = ir.metadata.version {
        HcfVersion {
            target_application: TargetApplication::WordProcessor,
            major: ver.major,
            minor: ver.minor,
            micro: ver.patch as i32,
            build_number: ver.build,
            os: 0,
            xml_version: None,
            application: None,
            application_version: None,
        }
    } else {
        HcfVersion {
            target_application: TargetApplication::WordProcessor,
            major: 1,
            minor: 0,
            micro: 0,
            build_number: 0,
            os: 0,
            xml_version: None,
            application: None,
            application_version: None,
        }
    };

    // 헤더 생성 (스타일 포함)
    let hwpx_ext = ir.extensions.hwpx.as_ref();
    let header = convert_head(&ir.styles, ir.sections.len() as u32, hwpx_ext, ctx)?;

    let mut doc = HwpxDocument::new(version, header);

    // 섹션 변환
    for (i, section) in ir.sections.iter().enumerate() {
        let hwpx_section = convert_section(section, i as u32)?;
        doc.sections.push(hwpx_section);
    }

    // 바이너리 데이터 변환
    for (id, binary) in ir.binary_data.iter() {
        doc.binary_data
            .insert(id.value().to_string(), binary.data.clone());
    }

    // 마스터 페이지 변환
    if let Some(ref hwpx_ext) = ir.extensions.hwpx {
        for mp_info in &hwpx_ext.master_pages {
            if let Ok(master_page) = convert_master_page_to_hwpx(mp_info) {
                doc.master_pages.push(master_page);
            }
        }
    }

    // HWP 확장 데이터 경고
    if ir.extensions.hwp.is_some() {
        ctx.warnings
            .data_loss("HWP 확장 데이터는 HWPX로 변환 시 손실됩니다");
    }

    Ok(doc)
}

/// 섹션 변환
fn convert_section(section: &ir::Section, section_id: u32) -> Result<HwpxSection, ConversionError> {
    let mut hwpx_section = HwpxSection {
        paragraphs: Vec::new(),
    };

    // SectionDefinition 생성
    let section_def = create_section_definition(section, section_id);

    // ColumnDefinition 생성 (설정이 있을 경우에만)
    let column_control = if has_column_settings(&section.columns) {
        let col_def = convert_ir_column_definition(&section.columns, section_id);
        Some(HwpxControl {
            items: vec![ControlItem::ColumnDefinition(col_def)],
        })
    } else {
        None
    };

    // 문단 변환
    for (i, para) in section.paragraphs.iter().enumerate() {
        let mut hwpx_para = convert_paragraph(para, i as u32)?;

        // 첫 번째 문단의 첫 번째 run에 SectionDefinition과 ColumnDefinition 삽입
        if i == 0 {
            let mut first_contents = vec![HwpxRunContent::SectionDefinition(Box::new(
                section_def.clone(),
            ))];

            // ColumnDefinition 추가
            if let Some(ref col_ctrl) = column_control {
                first_contents.push(HwpxRunContent::Control(col_ctrl.clone()));
            }

            if hwpx_para.runs.is_empty() {
                // 빈 run 생성
                hwpx_para.runs.push(HwpxRun {
                    contents: first_contents,
                    character_property_id_reference: None,
                    character_track_change_id: None,
                });
            } else {
                // 첫 번째 run 맨 앞에 삽입 (역순으로 삽입)
                for content in first_contents.into_iter().rev() {
                    hwpx_para.runs[0].contents.insert(0, content);
                }
            }
        }

        hwpx_section.paragraphs.push(hwpx_para);
    }

    // 문단이 없는 경우 빈 문단 생성 후 SectionDefinition 삽입
    if hwpx_section.paragraphs.is_empty() {
        let mut contents = vec![HwpxRunContent::SectionDefinition(Box::new(section_def))];
        if let Some(col_ctrl) = column_control {
            contents.push(HwpxRunContent::Control(col_ctrl));
        }

        let hwpx_para = HwpxParagraph {
            id: 0,
            runs: vec![HwpxRun {
                contents,
                character_property_id_reference: None,
                character_track_change_id: None,
            }],
            line_segments: None,
            paragraph_property_id_reference: None,
            style_id_reference: None,
            page_break: false,
            column_break: false,
            merged: false,
            paragraph_track_change_id: None,
        };
        hwpx_section.paragraphs.push(hwpx_para);
    }

    Ok(hwpx_section)
}

/// SectionDefinition 생성
fn create_section_definition(section: &ir::Section, section_id: u32) -> SectionDefinition {
    use primitive::{GutterPosition as IrGutterPosition, PageOrientation as IrPageOrientation};

    // 용지 설정
    let page_property = PageProperty {
        margin: PageMargin {
            left: section.page.margins.left.value() as u32,
            right: section.page.margins.right.value() as u32,
            top: section.page.margins.top.value() as u32,
            bottom: section.page.margins.bottom.value() as u32,
            header: section.page.margins.header.value() as u32,
            footer: section.page.margins.footer.value() as u32,
            gutter: section.page.margins.gutter.value() as u32,
        },
        orientation: match section.page.orientation {
            IrPageOrientation::Portrait => PaperOrientation::Portrait,
            IrPageOrientation::Landscape => PaperOrientation::Landscape,
        },
        width: section.page.width.value() as u32,
        height: section.page.height.value() as u32,
        // GutterType in HWPX: LeftOnly, LeftRight, TopBottom
        gutter_type: match section.page.gutter_position {
            IrGutterPosition::Left => GutterType::LeftOnly,
            IrGutterPosition::Right => GutterType::LeftRight, // 가장 가까운 매핑
            IrGutterPosition::Top | IrGutterPosition::Bottom => GutterType::TopBottom,
        },
    };

    // 각주 모양
    let footnote_shape = section.footnote_shape.as_ref().map(|footnote| {
        let base = &footnote.base;
        // IR NoteNumbering → HWPX FootnoteNumberingType 변환
        let footnote_numbering_type = match base.numbering {
            primitive::NoteNumbering::Continuous => FootnoteNumberingType::Continuous,
            primitive::NoteNumbering::RestartSection => FootnoteNumberingType::OnSection,
            primitive::NoteNumbering::RestartPage => FootnoteNumberingType::OnPage,
        };

        // IR FootnotePlacement → HWPX FootnotePlacement 변환
        let placement = match footnote.placement {
            primitive::FootnotePlacement::EachColumn => {
                crate::paragraph::FootnotePlacement::EachColumn
            }
            primitive::FootnotePlacement::MergedColumn => {
                crate::paragraph::FootnotePlacement::MergedColumn
            }
            primitive::FootnotePlacement::RightMostColumn => {
                crate::paragraph::FootnotePlacement::RightMostColumn
            }
        };

        // 구분선 종류 변환
        let line_type = convert_line_type_to_hwpx(base.separator_line_type);
        let line_width = convert_line_width_to_hwpx(base.separator_line_width);
        let line_color = crate::core::types::RgbColor {
            r: base.separator_line_color.red,
            g: base.separator_line_color.green,
            b: base.separator_line_color.blue,
            a: 255,
        };

        FootnoteShape {
            auto_number_format: convert_note_auto_number_format(base),
            note_line: NoteLine {
                length: convert_separator_length_to_hwpx(base.separator_length),
                line_type,
                width: line_width,
                color: line_color,
            },
            note_spacing: NoteSpacing {
                between_notes: base.space_between.value() as u32,
                below_line: base.space_below.value() as u32,
                above_line: base.space_above.value() as u32,
            },
            numbering: FootnoteNumbering {
                numbering_type: footnote_numbering_type,
                new_number: base.start_number,
            },
            placement: FootnotePlacementSettings {
                place: placement,
                beneath_text: base.beneath_text,
            },
        }
    });

    // 미주 모양
    let endnote_shape = section.endnote_shape.as_ref().map(|endnote| {
        let base = &endnote.base;
        // IR NoteNumbering → HWPX EndnoteNumberingType 변환
        let endnote_numbering_type = match base.numbering {
            primitive::NoteNumbering::Continuous => EndnoteNumberingType::Continuous,
            primitive::NoteNumbering::RestartSection => EndnoteNumberingType::OnSection,
            primitive::NoteNumbering::RestartPage => EndnoteNumberingType::OnSection, // 미주는 페이지별 없음
        };

        // IR EndnotePlacement → HWPX EndnotePlacement 변환
        let placement = match endnote.placement {
            primitive::EndnotePlacement::EndOfDocument => {
                crate::paragraph::EndnotePlacement::EndOfDocument
            }
            primitive::EndnotePlacement::EndOfSection => {
                crate::paragraph::EndnotePlacement::EndOfSection
            }
        };

        // 구분선 종류 변환
        let line_type = convert_line_type_to_hwpx(base.separator_line_type);
        let line_width = convert_line_width_to_hwpx(base.separator_line_width);
        let line_color = crate::core::types::RgbColor {
            r: base.separator_line_color.red,
            g: base.separator_line_color.green,
            b: base.separator_line_color.blue,
            a: 255,
        };

        EndnoteShape {
            auto_number_format: convert_note_auto_number_format(base),
            note_line: NoteLine {
                length: convert_separator_length_to_hwpx(base.separator_length),
                line_type,
                width: line_width,
                color: line_color,
            },
            note_spacing: NoteSpacing {
                between_notes: base.space_between.value() as u32,
                below_line: base.space_below.value() as u32,
                above_line: base.space_above.value() as u32,
            },
            numbering: EndnoteNumbering {
                numbering_type: endnote_numbering_type,
                new_number: base.start_number,
            },
            placement: EndnotePlacementSettings {
                place: placement,
                beneath_text: base.beneath_text,
            },
        }
    });

    // 페이지 테두리/배경
    let page_border_fills = section
        .page_border_fill
        .as_ref()
        .map(|pf| {
            use crate::core::types::BorderFillIdRef;
            use crate::paragraph::enums::{FillAreaType, PageBorderType};

            // 페이지 타입 변환
            let page_type = match pf.page_type {
                ir::section::PageBorderPageType::Both => Some(PageBorderType::Both),
                ir::section::PageBorderPageType::Even => Some(PageBorderType::Even),
                ir::section::PageBorderPageType::Odd => Some(PageBorderType::Odd),
                ir::section::PageBorderPageType::First => Some(PageBorderType::Both), // HWPX에는 First가 없음
            };

            // 채우기 영역 변환
            let fill_area = match pf.fill_area {
                ir::section::PageBorderFillArea::Paper => Some(FillAreaType::Paper),
                ir::section::PageBorderFillArea::Body => Some(FillAreaType::Page),
                ir::section::PageBorderFillArea::Content => Some(FillAreaType::Border),
            };

            vec![HwpxPageBorderFill {
                offset: PageBorderOffset {
                    left: pf.offsets.left.value() as u32,
                    right: pf.offsets.right.value() as u32,
                    top: pf.offsets.top.value() as u32,
                    bottom: pf.offsets.bottom.value() as u32,
                },
                page_type,
                border_fill_id_reference: Some(BorderFillIdRef(pf.border_fill_id.value())),
                text_border: Some(match pf.position {
                    ir::section::PageBorderPosition::Paper => HwpxPageBorderPosition::Paper,
                    ir::section::PageBorderPosition::Body => HwpxPageBorderPosition::Content,
                }),
                header_inside: pf.header_inside,
                footer_inside: pf.footer_inside,
                fill_area,
            }]
        })
        .unwrap_or_default();

    // 시작 페이지 번호
    let start_number = if section.start_number.page > 0
        || section.start_number.picture > 0
        || section.start_number.table > 0
        || section.start_number.equation > 0
    {
        use primitive::PageStartsOn as IrPageStartsOn;
        let page_starts_on = match section.start_number.page_starts_on {
            IrPageStartsOn::Both => crate::paragraph::enums::PageStartsOn::Both,
            IrPageStartsOn::Even => crate::paragraph::enums::PageStartsOn::Even,
            IrPageStartsOn::Odd => crate::paragraph::enums::PageStartsOn::Odd,
        };
        Some(SectionStartNumber {
            page_starts_on,
            page: section.start_number.page,
            picture: section.start_number.picture,
            table: section.start_number.table,
            equation: section.start_number.equation,
        })
    } else {
        None
    };

    // 그리드 설정 변환
    let grid =
        if section.extensions.grid.line_grid > 0 || section.extensions.grid.character_grid > 0 {
            Some(crate::paragraph::SectionGrid {
                line_grid: section.extensions.grid.line_grid,
                character_grid: section.extensions.grid.character_grid,
                manuscript_format: section.extensions.grid.manuscript_format,
            })
        } else {
            None
        };

    // 가시성 설정 변환
    let visibility = {
        let vis = &section.extensions.visibility;
        let has_visibility = vis.hide_first_header
            || vis.hide_first_footer
            || vis.hide_first_master_page
            || vis.hide_first_page_number
            || vis.hide_first_empty_line
            || vis.show_line_number
            || vis.border_visibility.is_some()
            || vis.fill_visibility.is_some();

        if has_visibility {
            use crate::paragraph::enums::VisibilityValue;

            let convert_to_visibility_value =
                |opt: Option<ir::section::VisibilityOption>| -> Option<VisibilityValue> {
                    match opt {
                        Some(ir::section::VisibilityOption::Hide) => {
                            Some(VisibilityValue::HideFirst)
                        }
                        Some(ir::section::VisibilityOption::HideFirstPage) => {
                            Some(VisibilityValue::HideFirst)
                        }
                        Some(ir::section::VisibilityOption::Show) => Some(VisibilityValue::ShowAll),
                        Some(ir::section::VisibilityOption::ShowFirstPage) => {
                            Some(VisibilityValue::ShowFirst)
                        }
                        None => None,
                    }
                };

            Some(crate::paragraph::SectionVisibility {
                hide_first_header: vis.hide_first_header,
                hide_first_footer: vis.hide_first_footer,
                hide_first_master_page: vis.hide_first_master_page,
                hide_first_page_number: vis.hide_first_page_number,
                hide_first_empty_line: vis.hide_first_empty_line,
                show_line_number: vis.show_line_number,
                border: convert_to_visibility_value(vis.border_visibility),
                fill: convert_to_visibility_value(vis.fill_visibility),
            })
        } else {
            None
        }
    };

    // 줄 번호 모양 변환
    let line_number_shape = section.extensions.line_number_shape.as_ref().map(|shape| {
        use primitive::LineNumberRestartType;

        let restart_type = match shape.restart_type {
            LineNumberRestartType::Continuous => Some(0),
            LineNumberRestartType::RestartSection => Some(1),
            LineNumberRestartType::RestartPage => Some(2),
        };

        crate::paragraph::LineNumberShape {
            restart_type,
            count_by: Some(shape.count_by),
            distance: Some(shape.distance.value() as u32),
            start_number: Some(shape.start_number),
        }
    });

    SectionDefinition {
        start_number,
        grid,
        visibility,
        line_number_shape,
        page_property: Some(page_property),
        footnote_shape,
        endnote_shape,
        page_border_fills,
        master_pages: Vec::new(),
        presentation: None,
        id: format!("{}", section_id),
        text_direction: crate::paragraph::enums::TextDirection::Horizontal,
        space_columns: section.columns.gap.value(),
        tab_stop_value: 8000, // 기본값
        tab_stop_unit: crate::paragraph::enums::TabStopUnit::HwpUnit,
        outline_shape_id_reference: None,
        memo_shape_id_reference: None,
        text_vertical_width_head: false,
        master_page_count: 0,
    }
}

/// IR NoteShape → HWPX AutoNumberFormat 변환
fn convert_note_auto_number_format(note: &ir::section::NoteShape) -> AutoNumberFormat {
    AutoNumberFormat {
        number_type: convert_number_format_to_hwpx(&note.number_format),
        user_character: note.user_character.clone(),
        prefix_character: note.prefix.clone(),
        suffix_character: note.suffix.clone().unwrap_or_else(|| ")".to_string()),
        superscript: note.superscript,
    }
}

/// IR LineType → HWPX LineStyleType2 변환
fn convert_line_type_to_hwpx(line_type: primitive::LineType) -> crate::core::enums::LineStyleType2 {
    use crate::core::enums::LineStyleType2;
    match line_type {
        primitive::LineType::None => LineStyleType2::None,
        primitive::LineType::Solid => LineStyleType2::Solid,
        primitive::LineType::Dash => LineStyleType2::Dash,
        primitive::LineType::Dot => LineStyleType2::Dot,
        primitive::LineType::DashDot => LineStyleType2::DashDot,
        primitive::LineType::DashDotDot => LineStyleType2::DashDotDot,
        primitive::LineType::LongDash => LineStyleType2::LongDash,
        primitive::LineType::Circle => LineStyleType2::Circle,
        _ => LineStyleType2::Solid, // 기본값
    }
}

/// IR separator_line_width → HWPX LineWidth 변환
fn convert_line_width_to_hwpx(width: u8) -> crate::core::enums::LineWidth {
    use crate::core::enums::LineWidth;
    // 0.1mm 단위
    match width {
        0..=1 => LineWidth::Mm0_12,
        2 => LineWidth::Mm0_2,
        3 => LineWidth::Mm0_3,
        4 => LineWidth::Mm0_4,
        5 => LineWidth::Mm0_5,
        6..=7 => LineWidth::Mm0_7,
        8..=10 => LineWidth::Mm1_0,
        11..=17 => LineWidth::Mm1_5,
        18..=25 => LineWidth::Mm2_0,
        26..=35 => LineWidth::Mm3_0,
        36..=45 => LineWidth::Mm4_0,
        _ => LineWidth::Mm5_0,
    }
}

/// IR separator_length (HwpUnit) → HWPX NoteLine length 변환
fn convert_separator_length_to_hwpx(length: primitive::HwpUnit) -> i32 {
    // HWPX: 0 (없음), -1 (5cm), -2 (2cm), -3 (단 크기의 1/3), -4 (단 크기), 양수 (HWPUNIT)
    let length_val = length.value();

    if length_val == 0 {
        return 0;
    }

    // mm로 변환하여 대략적인 값 매칭
    let mm = length.to_mm();

    if (mm - 50.0).abs() < 5.0 {
        -1 // 5cm
    } else if (mm - 20.0).abs() < 5.0 {
        -2 // 2cm
    } else if (mm - 56.0).abs() < 5.0 {
        -3 // 단 크기의 1/3
    } else if mm >= 160.0 {
        -4 // 단 크기 전체
    } else {
        // HwpUnit 절대값 그대로 반환
        length_val
    }
}

/// IR ColumnDefinition → HWPX ColumnDefinition 변환
fn convert_ir_column_definition(
    col_def: &ir::section::ColumnDefinition,
    section_id: u32,
) -> HwpxColumnDef {
    use ir::section::{ColumnDirection, ColumnSeparator};

    // 단 방향
    let layout = match col_def.direction {
        ColumnDirection::LeftToRight => ColumnLayout::Left,
        ColumnDirection::RightToLeft => ColumnLayout::Right,
        ColumnDirection::FacingPages => ColumnLayout::Mirror,
    };

    // 두께를 LineWidth로 변환 (0.1mm 단위)
    let line_width = match col_def.separator_thickness {
        0..=1 => crate::core::enums::LineWidth::Mm0_12,
        2 => crate::core::enums::LineWidth::Mm0_2,
        3 => crate::core::enums::LineWidth::Mm0_3,
        4 => crate::core::enums::LineWidth::Mm0_4,
        5 => crate::core::enums::LineWidth::Mm0_5,
        6 => crate::core::enums::LineWidth::Mm0_6,
        7 => crate::core::enums::LineWidth::Mm0_7,
        8..=12 => crate::core::enums::LineWidth::Mm1_0,
        13..=17 => crate::core::enums::LineWidth::Mm1_5,
        18..=25 => crate::core::enums::LineWidth::Mm2_0,
        26..=35 => crate::core::enums::LineWidth::Mm3_0,
        36..=45 => crate::core::enums::LineWidth::Mm4_0,
        _ => crate::core::enums::LineWidth::Mm5_0,
    };

    // 색상 변환
    let line_color = crate::core::types::RgbColor {
        r: col_def.separator_color.red,
        g: col_def.separator_color.green,
        b: col_def.separator_color.blue,
        a: 255, // 불투명
    };

    // 단 구분선
    let column_line = match col_def.separator {
        ColumnSeparator::None => None,
        ColumnSeparator::Solid => Some(ColumnLine {
            line_type: crate::core::enums::LineStyleType2::Solid,
            width: line_width,
            color: line_color,
        }),
        ColumnSeparator::Dash => Some(ColumnLine {
            line_type: crate::core::enums::LineStyleType2::Dash,
            width: line_width,
            color: line_color,
        }),
        ColumnSeparator::Dot => Some(ColumnLine {
            line_type: crate::core::enums::LineStyleType2::Dot,
            width: line_width,
            color: line_color,
        }),
    };

    // 개별 단 크기
    let column_sizes: Vec<ColumnSize> = col_def
        .widths
        .iter()
        .map(|w| ColumnSize {
            width: Some(w.value() as u32),
            gap: None,
        })
        .collect();

    HwpxColumnDef {
        column_line,
        column_sizes,
        id: format!("col_{}", section_id),
        column_type: ColumnType::Newspaper, // 기본값
        layout,
        column_count: col_def.count as u8,
        same_size: col_def.widths.is_empty(),
        same_gap: col_def.gap.value() as u32,
    }
}

/// 단 정의가 기본값이 아닌지 확인
fn has_column_settings(col_def: &ir::section::ColumnDefinition) -> bool {
    use ir::section::{ColumnDirection, ColumnSeparator};

    col_def.count > 1
        || col_def.direction != ColumnDirection::LeftToRight
        || col_def.gap.value() != 0
        || col_def.separator != ColumnSeparator::None
        || !col_def.widths.is_empty()
}

/// 문단 변환
fn convert_paragraph(para: &ir::Paragraph, id: u32) -> Result<HwpxParagraph, ConversionError> {
    let mut hwpx_para = HwpxParagraph {
        id,
        runs: Vec::new(),
        line_segments: None,
        paragraph_property_id_reference: para
            .para_shape_id
            .map(|id| crate::core::types::ParaShapeIdRef(id.value())),
        style_id_reference: para
            .style_id
            .map(|id| crate::core::types::StyleIdRef(id.value())),
        page_break: matches!(para.break_type, BreakType::Page),
        column_break: matches!(para.break_type, BreakType::Column),
        merged: false,
        paragraph_track_change_id: None,
    };

    // 런 변환
    for run in &para.runs {
        let hwpx_run = convert_run(run)?;
        hwpx_para.runs.push(hwpx_run);
    }

    // 줄 세그먼트 변환
    if let Some(ref segments) = para.line_segments {
        let hwpx_segments: Vec<HwpxLineSegment> = segments
            .iter()
            .map(|seg| HwpxLineSegment {
                text_position: seg.text_start as i32,
                vertical_position: seg.vertical_position.value(),
                vertical_size: seg.line_height.value(),
                text_height: seg.text_height.value(),
                baseline: seg.baseline_distance.value(),
                spacing: seg.line_spacing.value(),
                horizontal_position: seg.column_start.value(),
                horizontal_size: seg.segment_width.value(),
                flags: 0,
            })
            .collect();

        if !hwpx_segments.is_empty() {
            hwpx_para.line_segments = Some(LineSegmentArray {
                segments: hwpx_segments,
            });
        }
    }

    // 형광펜 범위 태그 삽입
    insert_highlight_tags(para, &mut hwpx_para);

    Ok(hwpx_para)
}

/// 형광펜 범위 태그 삽입
///
/// IR의 RangeTag (Highlight)를 HWPX의 MarkPenBegin/MarkPenEnd로 변환합니다.
fn insert_highlight_tags(para: &ir::Paragraph, hwpx_para: &mut HwpxParagraph) {
    use crate::paragraph::{MarkPenBegin, MarkPenEnd, RunContent as HwpxRunContent, TextMarkup};
    use ir::paragraph::RangeTagType;

    // Highlight 태그만 필터링
    let mut highlights: Vec<&ir::paragraph::RangeTag> = para
        .range_tags
        .iter()
        .filter(|tag| matches!(tag.tag_type, RangeTagType::Highlight))
        .collect();

    if highlights.is_empty() {
        return;
    }

    // 시작 위치 기준으로 정렬
    highlights.sort_by_key(|tag| tag.start);

    // 각 런의 텍스트 내용을 수정하여 MarkPen 태그 삽입
    let mut current_pos = 0u32;

    for run in &mut hwpx_para.runs {
        for content in &mut run.contents {
            if let HwpxRunContent::Text(text_elem) = content {
                let mut new_contents = Vec::new();

                for markup in &text_elem.contents {
                    match markup {
                        TextMarkup::Text(text) => {
                            let text_len = text.chars().count() as u32;
                            let text_start = current_pos;
                            let text_end = current_pos + text_len;

                            // 이 텍스트 범위 내에서 시작하거나 끝나는 형광펜 태그 찾기
                            let mut events = Vec::new();

                            for highlight in &highlights {
                                if highlight.start >= text_start && highlight.start < text_end {
                                    events.push((
                                        highlight.start - text_start,
                                        true,
                                        &highlight.data,
                                    ));
                                }
                                if highlight.end > text_start && highlight.end <= text_end {
                                    events.push((
                                        highlight.end - text_start,
                                        false,
                                        &highlight.data,
                                    ));
                                }
                            }

                            if events.is_empty() {
                                // 태그 없음: 텍스트 그대로
                                new_contents.push(TextMarkup::Text(text.clone()));
                            } else {
                                // 태그 있음: 텍스트를 분할하고 태그 삽입
                                events.sort_by_key(|(pos, _, _)| *pos);

                                let mut last_pos = 0u32;
                                let text_chars: Vec<char> = text.chars().collect();

                                for (pos, is_begin, color_data) in events {
                                    // 이전 위치부터 현재 위치까지의 텍스트
                                    if pos > last_pos {
                                        let segment: String = text_chars
                                            [last_pos as usize..pos as usize]
                                            .iter()
                                            .collect();
                                        new_contents.push(TextMarkup::Text(segment));
                                    }

                                    // 태그 삽입
                                    if is_begin {
                                        let color =
                                            color_data.as_ref().and_then(|s| parse_color(s));
                                        new_contents
                                            .push(TextMarkup::MarkPenBegin(MarkPenBegin { color }));
                                    } else {
                                        new_contents.push(TextMarkup::MarkPenEnd(MarkPenEnd));
                                    }

                                    last_pos = pos;
                                }

                                // 남은 텍스트
                                if last_pos < text_len {
                                    let segment: String =
                                        text_chars[last_pos as usize..].iter().collect();
                                    new_contents.push(TextMarkup::Text(segment));
                                }
                            }

                            current_pos = text_end;
                        }
                        other => {
                            new_contents.push(other.clone());
                            // 특수 문자: 위치 1 증가
                            match other {
                                TextMarkup::Tab(_)
                                | TextMarkup::LineBreak(_)
                                | TextMarkup::NonBreakingSpace(_)
                                | TextMarkup::FixedWidthSpace(_)
                                | TextMarkup::Hyphen(_) => {
                                    current_pos += 1;
                                }
                                _ => {}
                            }
                        }
                    }
                }

                text_elem.contents = new_contents;
            } else {
                // Control: 위치 1 증가
                current_pos += 1;
            }
        }
    }
}

/// 색상 문자열 파싱 (#RRGGBB 형식)
fn parse_color(color_str: &str) -> Option<crate::core::types::RgbColor> {
    if !color_str.starts_with('#') || color_str.len() != 7 {
        return None;
    }

    let r = u8::from_str_radix(&color_str[1..3], 16).ok()?;
    let g = u8::from_str_radix(&color_str[3..5], 16).ok()?;
    let b = u8::from_str_radix(&color_str[5..7], 16).ok()?;

    Some(crate::core::types::RgbColor { r, g, b, a: 255 })
}

/// 런 변환
fn convert_run(run: &ir::Run) -> Result<HwpxRun, ConversionError> {
    let mut hwpx_run = HwpxRun {
        contents: Vec::new(),
        character_property_id_reference: run
            .char_shape_id
            .map(|id| crate::core::types::CharShapeIdRef(id.value())),
        character_track_change_id: None,
    };

    // 런 내용 변환
    for content in &run.contents {
        if let Some(hwpx_content) = convert_run_content(content)? {
            hwpx_run.contents.push(hwpx_content);
        }
    }

    Ok(hwpx_run)
}

/// 런 내용 변환
fn convert_run_content(content: &IrRunContent) -> Result<Option<HwpxRunContent>, ConversionError> {
    match content {
        IrRunContent::Text(text) => {
            let text_elem = TextElement {
                contents: vec![TextMarkup::Text(text.text.clone())],
                character_style_id_reference: None,
            };
            Ok(Some(HwpxRunContent::Text(text_elem)))
        }
        IrRunContent::Tab(tab_char) => {
            use crate::core::enums::LineStyleType2;
            use crate::paragraph::InlineTabType;

            let text_elem = TextElement {
                contents: vec![TextMarkup::Tab(crate::paragraph::InlineTab {
                    width: tab_char.width.map(|w| w.value() as u32),
                    leader: tab_char.leader.map(|c| match c {
                        '.' => LineStyleType2::Dot,
                        '_' => LineStyleType2::Solid,
                        '-' => LineStyleType2::Dash,
                        _ => LineStyleType2::None,
                    }),
                    tab_type: tab_char.tab_type.map_or(InlineTabType::Left, |t| match t {
                        primitive::TabType::Left => InlineTabType::Left,
                        primitive::TabType::Right => InlineTabType::Right,
                        primitive::TabType::Center => InlineTabType::Center,
                        primitive::TabType::Decimal => InlineTabType::Decimal,
                    }),
                })],
                character_style_id_reference: None,
            };
            Ok(Some(HwpxRunContent::Text(text_elem)))
        }
        IrRunContent::LineBreak => {
            let text_elem = TextElement {
                contents: vec![TextMarkup::LineBreak(crate::paragraph::LineBreak)],
                character_style_id_reference: None,
            };
            Ok(Some(HwpxRunContent::Text(text_elem)))
        }
        IrRunContent::NonBreakingSpace => {
            let text_elem = TextElement {
                contents: vec![TextMarkup::NonBreakingSpace(
                    crate::paragraph::NonBreakingSpace,
                )],
                character_style_id_reference: None,
            };
            Ok(Some(HwpxRunContent::Text(text_elem)))
        }
        IrRunContent::FixedWidthSpace => {
            let text_elem = TextElement {
                contents: vec![TextMarkup::FixedWidthSpace(
                    crate::paragraph::FixedWidthSpace,
                )],
                character_style_id_reference: None,
            };
            Ok(Some(HwpxRunContent::Text(text_elem)))
        }
        IrRunContent::Hyphen => {
            let text_elem = TextElement {
                contents: vec![TextMarkup::Hyphen(crate::paragraph::Hyphen)],
                character_style_id_reference: None,
            };
            Ok(Some(HwpxRunContent::Text(text_elem)))
        }
        // 컨트롤 변환
        IrRunContent::Control(ctrl) => convert_control_to_hwpx(ctrl.as_ref()),
        // 필드 시작 변환
        IrRunContent::FieldStart(field_start) => {
            let hwpx_field_begin = convert_field_start_to_hwpx(field_start);
            let control = HwpxControl {
                items: vec![HwpxControlItem::FieldBegin(hwpx_field_begin)],
            };
            Ok(Some(HwpxRunContent::Control(control)))
        }
        // 필드 끝 변환
        IrRunContent::FieldEnd(field_end) => {
            let hwpx_field_end = HwpxFieldEnd {
                begin_id_reference: BeginIdRef(field_end.id),
                field_id: None,
            };
            let control = HwpxControl {
                items: vec![HwpxControlItem::FieldEnd(hwpx_field_end)],
            };
            Ok(Some(HwpxRunContent::Control(control)))
        }
        // 책갈피 시작/끝은 현재 무시 (HWPX의 Bookmark는 단일 요소)
        IrRunContent::BookmarkStart(_) | IrRunContent::BookmarkEnd(_) => Ok(None),
        // 글자 겹침 (Compose) 변환
        IrRunContent::Compose(compose) => {
            use crate::paragraph::text_art::{
                Compose as HwpxCompose, ComposeCharProperty,
                ComposeCircleType as HwpxComposeCircleType, ComposeType as HwpxComposeType,
            };
            use ir::paragraph::{ComposeCircleType, ComposeType};

            // ComposeType 변환
            let compose_type = compose.compose_type.map(|ct| match ct {
                ComposeType::Spread => HwpxComposeType::Spread,
                ComposeType::Overlap => HwpxComposeType::Overlap,
            });

            // ComposeCircleType 변환
            let circle_type = match compose.circle_type {
                ComposeCircleType::Char => HwpxComposeCircleType::Char,
                ComposeCircleType::ShapeCircle => HwpxComposeCircleType::ShapeCircle,
                ComposeCircleType::ShapeReversalCircle => {
                    HwpxComposeCircleType::ShapeReversalCircle
                }
                ComposeCircleType::ShapeRectangle => HwpxComposeCircleType::ShapeRectangle,
                ComposeCircleType::ShapeReversalRectangle => {
                    HwpxComposeCircleType::ShapeReversalRectangle
                }
                ComposeCircleType::ShapeTriangle => HwpxComposeCircleType::ShapeTriangle,
                ComposeCircleType::ShapeReversalTriangle => {
                    HwpxComposeCircleType::ShapeReversalTriangle
                }
                ComposeCircleType::ShapeLight => HwpxComposeCircleType::ShapeLight,
                ComposeCircleType::ShapeRhombus => HwpxComposeCircleType::ShapeRhombus,
                ComposeCircleType::ShapeReversalRhombus => {
                    HwpxComposeCircleType::ShapeReversalRhombus
                }
                ComposeCircleType::ShapeRoundedRectangle => {
                    HwpxComposeCircleType::ShapeRoundedRectangle
                }
                ComposeCircleType::ShapeEmptyCirculateTriangle => {
                    HwpxComposeCircleType::ShapeEmptyCirculateTriangle
                }
                ComposeCircleType::ShapeThinCirculateTriangle => {
                    HwpxComposeCircleType::ShapeThinCirculateTriangle
                }
                ComposeCircleType::ShapeThickCirculateTriangle => {
                    HwpxComposeCircleType::ShapeThickCirculateTriangle
                }
            };

            // 글자 속성 변환
            let char_properties: Vec<ComposeCharProperty> = compose
                .char_shape_ids
                .iter()
                .map(|id| ComposeCharProperty {
                    property_id_ref: id
                        .map(|cs_id| crate::core::types::CharShapeIdRef(cs_id.value())),
                })
                .collect();

            let hwpx_compose = HwpxCompose {
                char_properties,
                circle_type,
                char_size: compose.char_size,
                compose_type,
                char_property_count: Some(compose.char_shape_ids.len() as u32),
                compose_text: Some(compose.compose_text.clone()),
            };

            Ok(Some(HwpxRunContent::Compose(hwpx_compose)))
        }
        // 덧말 (Dutmal) 변환
        IrRunContent::Dutmal(dutmal) => {
            use crate::paragraph::text_art::{
                Dutmal as HwpxDutmal, DutmalAlignment as HwpxDutmalAlignment,
                DutmalPosition as HwpxDutmalPosition,
            };
            use ir::paragraph::{DutmalAlignment, DutmalPosition};

            // DutmalPosition 변환
            let position_type = match dutmal.position_type {
                DutmalPosition::Top => HwpxDutmalPosition::Top,
                DutmalPosition::Bottom => HwpxDutmalPosition::Bottom,
            };

            // DutmalAlignment 변환
            let alignment = match dutmal.alignment {
                DutmalAlignment::Justify => HwpxDutmalAlignment::Justify,
                DutmalAlignment::Left => HwpxDutmalAlignment::Left,
                DutmalAlignment::Right => HwpxDutmalAlignment::Right,
                DutmalAlignment::Center => HwpxDutmalAlignment::Center,
                DutmalAlignment::Distribute => HwpxDutmalAlignment::Distribute,
                DutmalAlignment::DistributeSpace => HwpxDutmalAlignment::DistributeSpace,
            };

            let hwpx_dutmal = HwpxDutmal {
                main_text: dutmal.main_text.clone(),
                sub_text: dutmal.sub_text.clone(),
                position_type,
                size_ratio: dutmal.size_ratio,
                option: dutmal.option,
                style_id_ref: dutmal
                    .style_id_ref
                    .map(|id| crate::core::types::StyleIdRef(id.value())),
                alignment,
            };

            Ok(Some(HwpxRunContent::Dutmal(hwpx_dutmal)))
        }
    }
}

/// Head 변환 (스타일 포함)
fn convert_head(
    styles: &StyleStore,
    section_count: u32,
    hwpx_ext: Option<&IrHwpxExtensions>,
    _ctx: &mut FromIrContext,
) -> Result<Head, ConversionError> {
    use crate::header::compatible_document::{
        CompatibleDocument, LayoutCompatibility, TargetProgram,
    };
    use crate::header::forbidden_word::ForbiddenWordList;
    use crate::header::track_change::TrackChangeConfig;

    // 폰트 변환
    let fontfaces = if !styles.fonts.is_empty() {
        Some(convert_fonts(&styles.fonts))
    } else {
        None
    };

    // 글자 모양 변환
    let character_shapes = if !styles.char_shapes.is_empty() {
        Some(convert_char_shapes(&styles.char_shapes))
    } else {
        None
    };

    // 문단 모양 변환
    let paragraph_shapes = if !styles.para_shapes.is_empty() {
        Some(convert_para_shapes(&styles.para_shapes))
    } else {
        None
    };

    // 스타일 변환
    let style_list = if !styles.styles.is_empty() {
        Some(convert_styles_list(&styles.styles))
    } else {
        None
    };

    // 테두리/채우기 변환
    let border_fills = if !styles.border_fills.is_empty() {
        Some(convert_border_fills(&styles.border_fills))
    } else {
        None
    };

    // 금칙 문자 목록 변환
    let forbidden_word_list = hwpx_ext
        .filter(|ext| !ext.forbidden_words.is_empty())
        .map(|ext| ForbiddenWordList {
            forbidden_words: ext.forbidden_words.clone(),
            item_count: ext.forbidden_words.len() as u32,
        });

    // 레이아웃 호환성 변환
    let compatible_document = hwpx_ext
        .and_then(|ext| ext.layout_compatibility.as_ref())
        .map(|compat| {
            let target_program = match compat.target_program {
                ir::extensions::HwpxTargetProgram::Hwp201X => TargetProgram::Hwp201X,
                ir::extensions::HwpxTargetProgram::Hwp200X => TargetProgram::Hwp200X,
                ir::extensions::HwpxTargetProgram::MsWord => TargetProgram::MsWord,
            };
            CompatibleDocument {
                layout_compatibility: LayoutCompatibility::default(),
                target_program,
            }
        });

    // 변경 추적 설정 변환
    let track_change_config = hwpx_ext
        .and_then(|ext| ext.track_change_config.as_ref())
        .filter(|config| config.enabled)
        .map(|_config| TrackChangeConfig {
            authors: None,
            changes: None,
        });

    Ok(Head {
        begin_number: BeginNumber::default(),
        mapping_table: MappingTable {
            binary_data_list: None,
            fontfaces,
            border_fills,
            character_shapes,
            tab_definitions: if styles.tab_defs.is_empty() {
                None
            } else {
                Some(convert_tab_defs(&styles.tab_defs))
            },
            numberings: if styles.numberings.is_empty() {
                None
            } else {
                Some(convert_numberings(&styles.numberings))
            },
            bullets: if styles.bullets.is_empty() {
                None
            } else {
                Some(convert_bullets(&styles.bullets))
            },
            paragraph_shapes,
            styles: style_list,
            memo_shapes: None,
        },
        forbidden_word_list,
        compatible_document,
        document_option: convert_document_option(hwpx_ext),
        track_change_config,
        version: "1.0".to_string(),
        section_count,
    })
}

/// 문서 옵션 변환
fn convert_document_option(
    hwpx_ext: Option<&IrHwpxExtensions>,
) -> Option<crate::header::document_option::DocumentOption> {
    let doc_opt = hwpx_ext?.document_option.as_ref()?;

    if doc_opt.link_document_path.is_some() {
        Some(crate::header::document_option::DocumentOption {
            link_document_path: doc_opt.link_document_path.clone(),
        })
    } else {
        None
    }
}

/// 폰트 목록 변환
fn convert_fonts(fonts: &[IrFont]) -> FontfaceList {
    // HWPX는 언어별로 폰트를 그룹화
    // IR은 단순 폰트 목록이므로, 한글 언어 그룹으로 통합
    let hwpx_fonts: Vec<HwpxFont> = fonts
        .iter()
        .enumerate()
        .map(|(i, font)| convert_font_to_hwpx(font, i as u32))
        .collect();

    let fontface = Fontface {
        fonts: hwpx_fonts.clone(),
        language: FontLanguage::Hangul,
        font_count: hwpx_fonts.len() as u32,
    };

    FontfaceList {
        fontfaces: vec![fontface],
        item_count: fonts.len() as u32,
    }
}

/// 개별 폰트 변환
fn convert_font_to_hwpx(font: &IrFont, id: u32) -> HwpxFont {
    use crate::core::types::BinaryItemIdRef;
    use crate::header::font::{
        FontFamilyType, FontTypeInfo, PanoseArmStyle, PanoseContrast, PanoseLetterform,
        PanoseMidline, PanoseProportion, PanoseSerifStyle, PanoseStrokeVariation, PanoseWeight,
        PanoseXHeight, SubstituteFont as HwpxSubstituteFont,
    };
    use ir::char_shape::FontType as IrFontType;

    // 폰트 타입 변환
    let font_type = match font.font_type {
        IrFontType::Representative => FontType::Representative,
        IrFontType::TrueType => FontType::TrueType,
        IrFontType::HangeulOnly => FontType::HangeulFont,
    };

    // 대체 폰트 변환
    let substitute_font = font.substitute_font.as_ref().map(|subst| {
        let subst_type = match subst.font_type {
            IrFontType::Representative => FontType::Representative,
            IrFontType::TrueType => FontType::TrueType,
            IrFontType::HangeulOnly => FontType::HangeulFont,
        };

        HwpxSubstituteFont {
            face: subst.face.clone(),
            font_type: subst_type,
            is_embedded: subst.is_embedded,
            binary_item_id_reference: subst
                .binary_item_id_ref
                .as_ref()
                .map(|r| BinaryItemIdRef(r.value().to_string())),
        }
    });

    // PANOSE 정보 변환
    let type_info = font.panose.as_ref().map(|p| {
        let bytes = p.to_bytes();
        // u8 값을 enum으로 변환 (unsafe transmute 사용)
        FontTypeInfo {
            family_type: match bytes[0] {
                1 => FontFamilyType::Myungjo,
                2 => FontFamilyType::Gothic,
                3 => FontFamilyType::SansSerif,
                4 => FontFamilyType::Decorative,
                5 => FontFamilyType::BrushScript,
                _ => FontFamilyType::Unknown,
            },
            serif_style: if bytes[1] > 0 {
                Some(unsafe { std::mem::transmute::<u8, PanoseSerifStyle>(bytes[1]) })
            } else {
                None
            },
            weight: unsafe { std::mem::transmute::<u8, PanoseWeight>(bytes[2]) },
            proportion: unsafe { std::mem::transmute::<u8, PanoseProportion>(bytes[3]) },
            contrast: unsafe { std::mem::transmute::<u8, PanoseContrast>(bytes[4]) },
            stroke_variation: unsafe { std::mem::transmute::<u8, PanoseStrokeVariation>(bytes[5]) },
            arm_style: unsafe { std::mem::transmute::<u8, PanoseArmStyle>(bytes[6]) },
            letterform: unsafe { std::mem::transmute::<u8, PanoseLetterform>(bytes[7]) },
            midline: unsafe { std::mem::transmute::<u8, PanoseMidline>(bytes[8]) },
            x_height: unsafe { std::mem::transmute::<u8, PanoseXHeight>(bytes[9]) },
        }
    });

    HwpxFont {
        substitute_font,
        type_info,
        id,
        face: font.name.clone(),
        font_type,
        is_embedded: font.is_embedded,
        binary_item_id_reference: font
            .binary_item_id_ref
            .as_ref()
            .map(|r| BinaryItemIdRef(r.value().to_string())),
    }
}

/// 글자 모양 목록 변환
fn convert_char_shapes(char_shapes: &[IrCharShape]) -> CharacterShapeList {
    let hwpx_shapes: Vec<HwpxCharShape> = char_shapes
        .iter()
        .enumerate()
        .map(|(i, shape)| convert_char_shape_to_hwpx(shape, i as u32))
        .collect();

    CharacterShapeList {
        item_count: hwpx_shapes.len() as u32,
        character_shapes: hwpx_shapes,
    }
}

/// 개별 글자 모양 변환
fn convert_char_shape_to_hwpx(shape: &IrCharShape, id: u32) -> HwpxCharShape {
    use crate::core::types::BorderFillIdRef;

    // 폰트 참조 (언어별로 개별 설정)
    let font_reference = LanguageFontReference {
        hangul: shape
            .fonts
            .korean
            .as_ref()
            .map(|f| f.id.value())
            .unwrap_or(0),
        latin: shape
            .fonts
            .english
            .as_ref()
            .map(|f| f.id.value())
            .unwrap_or(0),
        hanja: shape
            .fonts
            .hanja
            .as_ref()
            .map(|f| f.id.value())
            .unwrap_or(0),
        japanese: shape
            .fonts
            .japanese
            .as_ref()
            .map(|f| f.id.value())
            .unwrap_or(0),
        other: shape
            .fonts
            .other
            .as_ref()
            .map(|f| f.id.value())
            .unwrap_or(0),
        symbol: shape
            .fonts
            .symbol
            .as_ref()
            .map(|f| f.id.value())
            .unwrap_or(0),
        user: shape.fonts.user.as_ref().map(|f| f.id.value()).unwrap_or(0),
    };

    // 장평 (언어별로 개별 설정)
    let ratio = LanguageRatio {
        hangul: shape
            .fonts
            .korean
            .as_ref()
            .map(|f| f.width_ratio.0 as u8)
            .unwrap_or(100),
        latin: shape
            .fonts
            .english
            .as_ref()
            .map(|f| f.width_ratio.0 as u8)
            .unwrap_or(100),
        hanja: shape
            .fonts
            .hanja
            .as_ref()
            .map(|f| f.width_ratio.0 as u8)
            .unwrap_or(100),
        japanese: shape
            .fonts
            .japanese
            .as_ref()
            .map(|f| f.width_ratio.0 as u8)
            .unwrap_or(100),
        other: shape
            .fonts
            .other
            .as_ref()
            .map(|f| f.width_ratio.0 as u8)
            .unwrap_or(100),
        symbol: shape
            .fonts
            .symbol
            .as_ref()
            .map(|f| f.width_ratio.0 as u8)
            .unwrap_or(100),
        user: shape
            .fonts
            .user
            .as_ref()
            .map(|f| f.width_ratio.0 as u8)
            .unwrap_or(100),
    };

    // 자간 (언어별로 개별 설정)
    let spacing = LanguageSpacing {
        hangul: shape
            .fonts
            .korean
            .as_ref()
            .map(|f| f.spacing.0 as i8)
            .unwrap_or(0),
        latin: shape
            .fonts
            .english
            .as_ref()
            .map(|f| f.spacing.0 as i8)
            .unwrap_or(0),
        hanja: shape
            .fonts
            .hanja
            .as_ref()
            .map(|f| f.spacing.0 as i8)
            .unwrap_or(0),
        japanese: shape
            .fonts
            .japanese
            .as_ref()
            .map(|f| f.spacing.0 as i8)
            .unwrap_or(0),
        other: shape
            .fonts
            .other
            .as_ref()
            .map(|f| f.spacing.0 as i8)
            .unwrap_or(0),
        symbol: shape
            .fonts
            .symbol
            .as_ref()
            .map(|f| f.spacing.0 as i8)
            .unwrap_or(0),
        user: shape
            .fonts
            .user
            .as_ref()
            .map(|f| f.spacing.0 as i8)
            .unwrap_or(0),
    };

    // 상대 크기 (언어별로 개별 설정)
    let relative_size = LanguageRelativeSize {
        hangul: shape
            .fonts
            .korean
            .as_ref()
            .map(|f| f.relative_size.0 as u8)
            .unwrap_or(100),
        latin: shape
            .fonts
            .english
            .as_ref()
            .map(|f| f.relative_size.0 as u8)
            .unwrap_or(100),
        hanja: shape
            .fonts
            .hanja
            .as_ref()
            .map(|f| f.relative_size.0 as u8)
            .unwrap_or(100),
        japanese: shape
            .fonts
            .japanese
            .as_ref()
            .map(|f| f.relative_size.0 as u8)
            .unwrap_or(100),
        other: shape
            .fonts
            .other
            .as_ref()
            .map(|f| f.relative_size.0 as u8)
            .unwrap_or(100),
        symbol: shape
            .fonts
            .symbol
            .as_ref()
            .map(|f| f.relative_size.0 as u8)
            .unwrap_or(100),
        user: shape
            .fonts
            .user
            .as_ref()
            .map(|f| f.relative_size.0 as u8)
            .unwrap_or(100),
    };

    // 오프셋 (언어별로 개별 설정)
    let offset = LanguageOffset {
        hangul: shape
            .fonts
            .korean
            .as_ref()
            .map(|f| f.offset.0 as i8)
            .unwrap_or(0),
        latin: shape
            .fonts
            .english
            .as_ref()
            .map(|f| f.offset.0 as i8)
            .unwrap_or(0),
        hanja: shape
            .fonts
            .hanja
            .as_ref()
            .map(|f| f.offset.0 as i8)
            .unwrap_or(0),
        japanese: shape
            .fonts
            .japanese
            .as_ref()
            .map(|f| f.offset.0 as i8)
            .unwrap_or(0),
        other: shape
            .fonts
            .other
            .as_ref()
            .map(|f| f.offset.0 as i8)
            .unwrap_or(0),
        symbol: shape
            .fonts
            .symbol
            .as_ref()
            .map(|f| f.offset.0 as i8)
            .unwrap_or(0),
        user: shape
            .fonts
            .user
            .as_ref()
            .map(|f| f.offset.0 as i8)
            .unwrap_or(0),
    };

    // 밑줄
    let underline = if shape.underline.line_type != IrUnderlineType::None {
        Some(CharacterUnderline {
            position: convert_underline_position_to_hwpx(&shape.underline.position),
            shape: convert_underline_type_to_hwpx(&shape.underline.line_type),
            color: shape
                .underline
                .color
                .map(|c| RgbColor {
                    r: c.red,
                    g: c.green,
                    b: c.blue,
                    a: 255,
                })
                .unwrap_or(RgbColor::BLACK),
        })
    } else {
        None
    };

    // 취소선
    let strikeout = if shape.strikethrough != IrStrikethroughType::None {
        Some(CharacterStrikeout {
            shape: convert_strikethrough_type_to_hwpx(&shape.strikethrough),
            color: RgbColor::BLACK,
        })
    } else {
        None
    };

    // 외곽선
    let outline = if shape.outline != IrOutlineType::None {
        Some(CharacterOutline {
            outline_type: convert_outline_type_to_hwpx(&shape.outline),
        })
    } else {
        None
    };

    // 그림자
    let shadow = if shape.shadow.shadow_type != IrShadowType::None {
        Some(CharacterShadow {
            shadow_type: convert_shadow_type_to_hwpx(&shape.shadow.shadow_type),
            color: shape
                .shadow
                .color
                .map(|c| RgbColor {
                    r: c.red,
                    g: c.green,
                    b: c.blue,
                    a: 255,
                })
                .unwrap_or(RgbColor::BLACK),
            // 퍼센트 값 그대로 (i8 범위 내로 클램프)
            offset_x: shape.shadow.offset_x.value().clamp(-100, 100) as i8,
            offset_y: shape.shadow.offset_y.value().clamp(-100, 100) as i8,
        })
    } else {
        None
    };

    HwpxCharShape {
        font_reference,
        ratio,
        spacing,
        relative_size,
        offset,
        italic: if shape.italic { Some(()) } else { None },
        bold: if shape.bold { Some(()) } else { None },
        underline,
        strikeout,
        outline,
        shadow,
        emboss: if shape.emboss { Some(()) } else { None },
        engrave: if shape.engrave { Some(()) } else { None },
        superscript: if shape.superscript { Some(()) } else { None },
        subscript: if shape.subscript { Some(()) } else { None },
        id,
        height: shape.font_size.value(),
        text_color: RgbColor {
            r: shape.color.red,
            g: shape.color.green,
            b: shape.color.blue,
            a: 255,
        },
        shade_color: shape
            .shade_color
            .as_ref()
            .or(shape.background_color.as_ref())
            .map(|c| RgbColor {
                r: c.red,
                g: c.green,
                b: c.blue,
                a: 255,
            })
            .unwrap_or(RgbColor::WHITE),
        use_font_space: false,
        use_kerning: shape.use_kerning,
        emphasis_mark: convert_emphasis_type_to_hwpx(&shape.emphasis.emphasis_type),
        border_fill_id_reference: shape
            .border_fill_id_ref
            .map(|id| BorderFillIdRef(id.value())),
    }
}

/// 문단 모양 목록 변환
fn convert_para_shapes(para_shapes: &[IrParaShape]) -> ParagraphShapeList {
    let hwpx_shapes: Vec<HwpxParaShape> = para_shapes
        .iter()
        .enumerate()
        .map(|(i, shape)| convert_para_shape_to_hwpx(shape, i as u32))
        .collect();

    ParagraphShapeList {
        item_count: hwpx_shapes.len() as u32,
        paragraph_shapes: hwpx_shapes,
    }
}

/// 개별 문단 모양 변환
fn convert_para_shape_to_hwpx(shape: &IrParaShape, id: u32) -> HwpxParaShape {
    use crate::core::types::{BorderFillIdRef, HwpValue, ParaShapeIdRef};

    HwpxParaShape {
        id,
        alignment: ParagraphAlignment {
            horizontal: convert_alignment_to_hwpx(&shape.alignment),
            vertical: convert_vertical_alignment_to_hwpx(&shape.vertical_alignment),
        },
        heading: if let Some(ref numbering) = shape.numbering {
            ParagraphHeading {
                heading_type: match numbering.heading_type {
                    primitive::HeadingType::None => HeadingType::None,
                    primitive::HeadingType::Outline => HeadingType::Outline,
                    primitive::HeadingType::Number => HeadingType::Number,
                    primitive::HeadingType::Bullet => HeadingType::Bullet,
                },
                id_reference: ParaShapeIdRef(
                    numbering.numbering_id.or(numbering.bullet_id).unwrap_or(0),
                ),
                level: numbering.level as u32,
            }
        } else {
            ParagraphHeading {
                heading_type: HeadingType::None,
                id_reference: ParaShapeIdRef(0),
                level: 0,
            }
        },
        margin: Some(ParagraphMargin {
            left: HwpValue {
                value: shape.margin_left.value(),
                unit: crate::core::enums::ValueUnit::HwpUnit,
            },
            right: HwpValue {
                value: shape.margin_right.value(),
                unit: crate::core::enums::ValueUnit::HwpUnit,
            },
            indent: HwpValue {
                value: shape.first_line_indent.value(),
                unit: crate::core::enums::ValueUnit::HwpUnit,
            },
            previous: HwpValue {
                value: shape.space_before.value(),
                unit: crate::core::enums::ValueUnit::HwpUnit,
            },
            next: HwpValue {
                value: shape.space_after.value(),
                unit: crate::core::enums::ValueUnit::HwpUnit,
            },
        }),
        line_spacing: Some(HwpxLineSpacing {
            spacing_type: convert_line_spacing_type_to_hwpx(&shape.line_spacing.spacing_type),
            value: match &shape.line_spacing.value {
                LineSpacingValue::Percent(p) => p.0 as i32,
                LineSpacingValue::Fixed(u) => u.value(),
            },
            unit: LineSpacingUnit::HwpUnit,
        }),
        tab_definition_id_reference: shape
            .tab_def_id
            .map(|id| crate::core::types::TabDefIdRef(id.value())),
        auto_spacing: ParagraphAutoSpacing {
            east_asian_english: shape.auto_spacing_east_asian_english,
            east_asian_number: shape.auto_spacing_east_asian_number,
        },
        snap_to_grid: shape.snap_to_grid,
        break_setting: ParagraphBreakSetting {
            break_latin_word: convert_break_latin_to_hwpx(&shape.line_break_latin),
            break_non_latin_word: convert_break_korean_to_hwpx(&shape.line_break_korean),
            widow_orphan: shape.widow_orphan_control,
            keep_with_next: shape.keep_with_next,
            keep_lines: shape.keep_lines,
            page_break_before: shape.page_break_before,
            line_wrap: LineWrapType::Break,
        },
        border: if let Some(ref border) = shape.border {
            ParagraphBorder {
                border_fill_id_reference: BorderFillIdRef(border.border_fill_id_ref.value()),
                offset_left: border.offset_left.value(),
                offset_right: border.offset_right.value(),
                offset_top: border.offset_top.value(),
                offset_bottom: border.offset_bottom.value(),
                connect: border.connect,
                ignore_margin: border.ignore_margin,
            }
        } else {
            ParagraphBorder {
                border_fill_id_reference: shape
                    .border_fill_id
                    .map_or(BorderFillIdRef(0), |id| BorderFillIdRef(id.value())),
                offset_left: 0,
                offset_right: 0,
                offset_top: 0,
                offset_bottom: 0,
                connect: false,
                ignore_margin: false,
            }
        },
        switch: None,
        condense: None,
        font_line_height: shape.auto_line_height_ratio.0 != 100.0,
        suppress_line_numbers: shape.suppress_line_numbers,
        checked: false,
    }
}

/// 스타일 목록 변환
fn convert_styles_list(styles: &[IrStyle]) -> StyleList {
    let hwpx_styles: Vec<HwpxStyle> = styles
        .iter()
        .enumerate()
        .map(|(i, style)| convert_style_to_hwpx(style, i as u32))
        .collect();

    StyleList {
        item_count: hwpx_styles.len() as u32,
        styles: hwpx_styles,
    }
}

/// 개별 스타일 변환
fn convert_style_to_hwpx(style: &IrStyle, id: u32) -> HwpxStyle {
    use crate::core::types::{CharShapeIdRef, ParaShapeIdRef, StyleIdRef};

    HwpxStyle {
        id,
        style_type: match style.style_type {
            IrStyleType::Paragraph => StyleKind::Paragraph,
            IrStyleType::Character => StyleKind::Character,
        },
        name: style.name.clone(),
        english_name: style.english_name.clone(),
        paragraph_shape_id_reference: style.para_shape_id.map(|id| ParaShapeIdRef(id.value())),
        character_shape_id_reference: style.char_shape_id.map(|id| CharShapeIdRef(id.value())),
        next_style_id_reference: style.next_style_id.map(|id| StyleIdRef(id.value())),
        language_id: None,
        lock_form: false,
    }
}

/// 테두리/채우기 목록 변환
fn convert_border_fills(
    border_fills: &[ir::border_fill::BorderFill],
) -> crate::header::border_fill::BorderFillList {
    use crate::header::border_fill::{BorderFill as HwpxBorderFill, BorderFillList};

    let hwpx_border_fills: Vec<HwpxBorderFill> = border_fills
        .iter()
        .enumerate()
        .map(|(i, bf)| convert_border_fill_to_hwpx(bf, i as u32))
        .collect();

    BorderFillList {
        border_fills: hwpx_border_fills,
        item_count: border_fills.len() as u32,
    }
}

/// 개별 테두리/채우기 변환
fn convert_border_fill_to_hwpx(
    bf: &ir::border_fill::BorderFill,
    id: u32,
) -> crate::header::border_fill::BorderFill {
    use crate::core::enums::{
        GradationType, HatchStyle, ImageBrushMode, LineStyleType2, LineWidth,
    };
    use crate::core::types::{
        BinaryItemIdRef, FillBrush, Gradation, GradationColor, GradationStep, GradationStepCenter,
        Image, ImageBrush, OptionalRgbColor, RgbColor, WindowsBrush,
    };
    use crate::header::border_fill::{Border as HwpxBorder, BorderFill as HwpxBorderFill};
    use ir::border_fill::{Fill, PatternType as IrPatternType};
    use primitive::{GradientType as IrGradientType, ImageFillMode, LineType};

    // 색상 변환 헬퍼
    let convert_color = |color: &primitive::Color| -> RgbColor {
        RgbColor {
            r: color.red,
            g: color.green,
            b: color.blue,
            a: color.alpha,
        }
    };

    // 선 스타일 변환 헬퍼
    let convert_line_type = |line_type: &LineType| -> LineStyleType2 {
        match line_type {
            LineType::None => LineStyleType2::None,
            LineType::Solid => LineStyleType2::Solid,
            LineType::Dash => LineStyleType2::Dash,
            LineType::Dot => LineStyleType2::Dot,
            LineType::DashDot => LineStyleType2::DashDot,
            LineType::DashDotDot => LineStyleType2::DashDotDot,
            LineType::LongDash => LineStyleType2::LongDash,
            LineType::Circle => LineStyleType2::Circle,
            LineType::Double => LineStyleType2::DoubleSlim,
            LineType::Triple | LineType::ThickThinLarge | LineType::ThinThickLarge => {
                LineStyleType2::SlimThickSlim
            }
            LineType::Wave | LineType::DoubleWave => LineStyleType2::Solid, // fallback
        }
    };

    // 선 두께 변환 헬퍼 (mm to LineWidth enum)
    let convert_width = |width: primitive::HwpUnit| -> LineWidth {
        let mm = width.to_mm();
        if mm <= 0.1 {
            LineWidth::Mm0_1
        } else if mm <= 0.12 {
            LineWidth::Mm0_12
        } else if mm <= 0.15 {
            LineWidth::Mm0_15
        } else if mm <= 0.2 {
            LineWidth::Mm0_2
        } else if mm <= 0.25 {
            LineWidth::Mm0_25
        } else if mm <= 0.3 {
            LineWidth::Mm0_3
        } else if mm <= 0.4 {
            LineWidth::Mm0_4
        } else if mm <= 0.5 {
            LineWidth::Mm0_5
        } else if mm <= 0.6 {
            LineWidth::Mm0_6
        } else if mm <= 0.7 {
            LineWidth::Mm0_7
        } else if mm <= 1.0 {
            LineWidth::Mm1_0
        } else if mm <= 1.5 {
            LineWidth::Mm1_5
        } else if mm <= 2.0 {
            LineWidth::Mm2_0
        } else if mm <= 3.0 {
            LineWidth::Mm3_0
        } else if mm <= 4.0 {
            LineWidth::Mm4_0
        } else {
            LineWidth::Mm5_0
        }
    };

    // 테두리 변환 헬퍼
    let convert_border = |border: &ir::border_fill::Border| -> Option<HwpxBorder> {
        if border.line_type == LineType::None {
            None
        } else {
            Some(HwpxBorder {
                line_type: convert_line_type(&border.line_type),
                width: convert_width(border.width),
                color: convert_color(&border.color),
            })
        }
    };

    // 4개 테두리 변환
    let left_border = convert_border(&bf.left);
    let right_border = convert_border(&bf.right);
    let top_border = convert_border(&bf.top);
    let bottom_border = convert_border(&bf.bottom);

    // 대각선 변환
    let diagonal = bf.diagonal_down.as_ref().and_then(convert_border);

    // 채우기 변환
    let fill_brush = match &bf.fill {
        Fill::None => None,
        Fill::Solid(solid) => Some(FillBrush {
            windows_brush: Some(WindowsBrush {
                face_color: OptionalRgbColor(Some(convert_color(&solid.color))),
                hatch_color: OptionalRgbColor(Some(RgbColor::default())),
                hatch_style: None,
                alpha: None,
            }),
            gradation: None,
            image_brush: None,
        }),
        Fill::Pattern(pattern) => {
            let hatch = match pattern.pattern_type {
                IrPatternType::Horizontal => HatchStyle::Horizontal,
                IrPatternType::Vertical => HatchStyle::Vertical,
                IrPatternType::DiagonalDown => HatchStyle::BackSlash,
                IrPatternType::DiagonalUp => HatchStyle::Slash,
                IrPatternType::Grid => HatchStyle::Cross,
                IrPatternType::DiagonalGrid => HatchStyle::CrossDiagonal,
            };
            Some(FillBrush {
                windows_brush: Some(WindowsBrush {
                    face_color: OptionalRgbColor(Some(convert_color(&pattern.background))),
                    hatch_color: OptionalRgbColor(Some(convert_color(&pattern.foreground))),
                    hatch_style: Some(hatch),
                    alpha: None,
                }),
                gradation: None,
                image_brush: None,
            })
        }
        Fill::Gradient(grad) => {
            let grad_type = match grad.gradient_type {
                IrGradientType::Linear => GradationType::Linear,
                IrGradientType::Radial => GradationType::Radial,
                IrGradientType::Conical => GradationType::Conical,
                IrGradientType::Square => GradationType::Square,
            };
            let colors: Vec<GradationColor> = grad
                .stops
                .iter()
                .map(|stop| GradationColor {
                    value: convert_color(&stop.color),
                })
                .collect();
            Some(FillBrush {
                windows_brush: None,
                gradation: Some(Gradation {
                    colors: colors.clone(),
                    gradation_type: Some(grad_type),
                    angle: grad.angle as i32,
                    center_x: grad.center_x as i32,
                    center_y: grad.center_y as i32,
                    step: GradationStep::new(grad.blur),
                    color_count: colors.len() as u32,
                    step_center: GradationStepCenter::new(grad.step_center).unwrap_or_default(),
                    alpha: None,
                }),
                image_brush: None,
            })
        }
        Fill::Image(img) => {
            let mode = match img.mode {
                ImageFillMode::Tile => ImageBrushMode::Tile,
                ImageFillMode::TileHorizontalTop => ImageBrushMode::TileHorizontalTop,
                ImageFillMode::TileHorizontalBottom => ImageBrushMode::TileHorizontalBottom,
                ImageFillMode::TileVerticalLeft => ImageBrushMode::TileVerticalLeft,
                ImageFillMode::TileVerticalRight => ImageBrushMode::TileVerticalRight,
                ImageFillMode::Stretch => ImageBrushMode::Total,
                ImageFillMode::Center => ImageBrushMode::Center,
                ImageFillMode::CenterTop => ImageBrushMode::CenterTop,
                ImageFillMode::CenterBottom => ImageBrushMode::CenterBottom,
                ImageFillMode::CenterLeft => ImageBrushMode::LeftCenter,
                ImageFillMode::TopLeft => ImageBrushMode::LeftTop,
                ImageFillMode::BottomLeft => ImageBrushMode::LeftBottom,
                ImageFillMode::CenterRight => ImageBrushMode::RightCenter,
                ImageFillMode::TopRight => ImageBrushMode::RightTop,
                ImageFillMode::BottomRight => ImageBrushMode::RightBottom,
                ImageFillMode::Original => ImageBrushMode::Center,
            };

            let effect = match img.effect {
                primitive::ImageEffect::Original => crate::core::enums::ImageEffect::RealPicture,
                primitive::ImageEffect::Grayscale => crate::core::enums::ImageEffect::GrayScale,
                primitive::ImageEffect::BlackWhite => crate::core::enums::ImageEffect::BlackWhite,
                primitive::ImageEffect::Pattern => crate::core::enums::ImageEffect::RealPicture,
            };

            Some(FillBrush {
                windows_brush: None,
                gradation: None,
                image_brush: Some(ImageBrush {
                    image: Image {
                        binary_item_id_reference: BinaryItemIdRef(
                            img.binary_id.value().to_string(),
                        ),
                        brightness: img.brightness as i32,
                        contrast: img.contrast as i32,
                        effect,
                        alpha: None,
                    },
                    mode,
                }),
            })
        }
    };

    HwpxBorderFill {
        slash: None,
        back_slash: None,
        left_border,
        right_border,
        top_border,
        bottom_border,
        diagonal,
        fill_brush,
        id,
        three_dimensional: bf.is_3d,
        shadow: bf.has_shadow,
        center_line: None,
        break_cell_separate_line: false,
    }
}

/// IR Fill → HWPX FillBrush 변환
fn convert_fill_to_hwpx(fill: &ir::border_fill::Fill) -> Option<crate::core::types::FillBrush> {
    use crate::core::enums::{GradationType, HatchStyle, ImageBrushMode};
    use crate::core::types::{
        BinaryItemIdRef, FillBrush, Gradation, GradationColor, GradationStep, GradationStepCenter,
        Image, ImageBrush, OptionalRgbColor, RgbColor, WindowsBrush,
    };
    use ir::border_fill::{Fill, PatternType as IrPatternType};
    use primitive::{GradientType as IrGradientType, ImageFillMode};

    // 색상 변환 헬퍼
    let convert_color = |color: &primitive::Color| -> RgbColor {
        RgbColor {
            r: color.red,
            g: color.green,
            b: color.blue,
            a: color.alpha,
        }
    };

    match fill {
        Fill::None => None,
        Fill::Solid(solid) => Some(FillBrush {
            windows_brush: Some(WindowsBrush {
                face_color: OptionalRgbColor(Some(convert_color(&solid.color))),
                hatch_color: OptionalRgbColor(Some(RgbColor::default())),
                hatch_style: None,
                alpha: None,
            }),
            gradation: None,
            image_brush: None,
        }),
        Fill::Pattern(pattern) => {
            let hatch = match pattern.pattern_type {
                IrPatternType::Horizontal => HatchStyle::Horizontal,
                IrPatternType::Vertical => HatchStyle::Vertical,
                IrPatternType::DiagonalDown => HatchStyle::BackSlash,
                IrPatternType::DiagonalUp => HatchStyle::Slash,
                IrPatternType::Grid => HatchStyle::Cross,
                IrPatternType::DiagonalGrid => HatchStyle::CrossDiagonal,
            };
            Some(FillBrush {
                windows_brush: Some(WindowsBrush {
                    face_color: OptionalRgbColor(Some(convert_color(&pattern.background))),
                    hatch_color: OptionalRgbColor(Some(convert_color(&pattern.foreground))),
                    hatch_style: Some(hatch),
                    alpha: None,
                }),
                gradation: None,
                image_brush: None,
            })
        }
        Fill::Gradient(grad) => {
            let grad_type = match grad.gradient_type {
                IrGradientType::Linear => GradationType::Linear,
                IrGradientType::Radial => GradationType::Radial,
                IrGradientType::Conical => GradationType::Conical,
                IrGradientType::Square => GradationType::Square,
            };
            let colors: Vec<GradationColor> = grad
                .stops
                .iter()
                .map(|stop| GradationColor {
                    value: convert_color(&stop.color),
                })
                .collect();
            Some(FillBrush {
                windows_brush: None,
                gradation: Some(Gradation {
                    colors: colors.clone(),
                    gradation_type: Some(grad_type),
                    angle: grad.angle as i32,
                    center_x: grad.center_x as i32,
                    center_y: grad.center_y as i32,
                    step: GradationStep::new(grad.blur),
                    color_count: colors.len() as u32,
                    step_center: GradationStepCenter::new(grad.step_center).unwrap_or_default(),
                    alpha: None,
                }),
                image_brush: None,
            })
        }
        Fill::Image(img) => {
            let mode = match img.mode {
                ImageFillMode::Tile => ImageBrushMode::Tile,
                ImageFillMode::TileHorizontalTop => ImageBrushMode::TileHorizontalTop,
                ImageFillMode::TileHorizontalBottom => ImageBrushMode::TileHorizontalBottom,
                ImageFillMode::TileVerticalLeft => ImageBrushMode::TileVerticalLeft,
                ImageFillMode::TileVerticalRight => ImageBrushMode::TileVerticalRight,
                ImageFillMode::Stretch => ImageBrushMode::Total,
                ImageFillMode::Center => ImageBrushMode::Center,
                ImageFillMode::CenterTop => ImageBrushMode::CenterTop,
                ImageFillMode::CenterBottom => ImageBrushMode::CenterBottom,
                ImageFillMode::CenterLeft => ImageBrushMode::LeftCenter,
                ImageFillMode::TopLeft => ImageBrushMode::LeftTop,
                ImageFillMode::BottomLeft => ImageBrushMode::LeftBottom,
                ImageFillMode::CenterRight => ImageBrushMode::RightCenter,
                ImageFillMode::TopRight => ImageBrushMode::RightTop,
                ImageFillMode::BottomRight => ImageBrushMode::RightBottom,
                ImageFillMode::Original => ImageBrushMode::Center,
            };

            let effect = match img.effect {
                primitive::ImageEffect::Original => crate::core::enums::ImageEffect::RealPicture,
                primitive::ImageEffect::Grayscale => crate::core::enums::ImageEffect::GrayScale,
                primitive::ImageEffect::BlackWhite => crate::core::enums::ImageEffect::BlackWhite,
                primitive::ImageEffect::Pattern => crate::core::enums::ImageEffect::RealPicture,
            };

            Some(FillBrush {
                windows_brush: None,
                gradation: None,
                image_brush: Some(ImageBrush {
                    image: Image {
                        binary_item_id_reference: BinaryItemIdRef(
                            img.binary_id.value().to_string(),
                        ),
                        brightness: img.brightness as i32,
                        contrast: img.contrast as i32,
                        effect,
                        alpha: None,
                    },
                    mode,
                }),
            })
        }
    }
}

/// 탭 정의 목록 변환
fn convert_tab_defs(
    tab_defs: &[ir::para_shape::TabDef],
) -> crate::header::tab_definition::TabDefinitionList {
    use crate::core::enums::LineStyleType2;
    use crate::header::tab_definition::{
        TabDefinition as HwpxTabDef, TabDefinitionList, TabItem, TabType as HwpxTabType,
    };
    use primitive::{TabLeader, TabType as IrTabType};

    let hwpx_tab_defs: Vec<HwpxTabDef> = tab_defs
        .iter()
        .enumerate()
        .map(|(i, td)| {
            // HWPX는 단일 탭 항목만 지원하므로 첫 번째만 사용
            let tab_item = td.tabs.first().map(|tab| TabItem {
                position: tab.position.value(),
                tab_type: match tab.tab_type {
                    IrTabType::Left => HwpxTabType::Left,
                    IrTabType::Right => HwpxTabType::Right,
                    IrTabType::Center => HwpxTabType::Center,
                    IrTabType::Decimal => HwpxTabType::Decimal,
                },
                leader: match tab.leader {
                    TabLeader::None => LineStyleType2::None,
                    TabLeader::Dot => LineStyleType2::Dot,
                    TabLeader::LongDash => LineStyleType2::DashDotDot,
                    TabLeader::Dash => LineStyleType2::Dash,
                    TabLeader::Underscore => LineStyleType2::Solid,
                },
            });

            HwpxTabDef {
                tab_item,
                id: i as u32,
                auto_tab_left: td.auto_tab_interval.is_some(),
                auto_tab_right: td.auto_tab_interval.is_some(),
            }
        })
        .collect();

    TabDefinitionList {
        tab_definitions: hwpx_tab_defs,
        item_count: tab_defs.len() as u32,
    }
}

/// 번호 매기기 목록 변환 (IR → HWPX)
fn convert_numberings(numberings: &[IrNumbering]) -> NumberingList {
    use crate::core::types::CharShapeIdRef;
    use crate::header::paragraph_head::{ParagraphHeadAlignment, TextOffsetType};

    let hwpx_numberings: Vec<HwpxNumbering> = numberings
        .iter()
        .enumerate()
        .map(|(i, num)| {
            let paragraph_heads: Vec<ParagraphHead> = num
                .levels
                .iter()
                .map(|level| {
                    // IR Alignment을 HWPX ParagraphHeadAlignment으로 변환
                    let alignment = match level.alignment {
                        primitive::Alignment::Left => ParagraphHeadAlignment::Left,
                        primitive::Alignment::Center => ParagraphHeadAlignment::Center,
                        primitive::Alignment::Right => ParagraphHeadAlignment::Right,
                        // Justify와 Distribute는 번호 매기기에서 지원하지 않으므로 Left로 처리
                        _ => ParagraphHeadAlignment::Left,
                    };

                    // IR NumberFormat을 HWPX NumberFormatType1으로 변환
                    let number_format = convert_ir_number_format_to_hwpx(level.number_format);

                    ParagraphHead {
                        text: level.format.clone(),
                        start: level.start_number,
                        level: level.level as u32,
                        alignment,
                        use_instance_width: level.use_instance_width,
                        auto_indent: level.auto_indent,
                        width_adjust: level.number_width,
                        text_offset_type: TextOffsetType::HwpUnit,
                        text_offset: level.text_offset,
                        number_format,
                        character_shape_id_reference: level
                            .char_shape_id
                            .map(|id| CharShapeIdRef(id.value())),
                        checkable: None,
                    }
                })
                .collect();

            HwpxNumbering {
                paragraph_heads,
                id: i as u32,
                start: num.start_number as i32,
            }
        })
        .collect();

    NumberingList {
        numberings: hwpx_numberings,
        item_count: numberings.len() as u32,
    }
}

/// 글머리 기호 목록 변환 (IR → HWPX)
fn convert_bullets(bullets: &[IrBullet]) -> BulletList {
    use crate::core::enums::NumberFormatType1;
    use crate::core::types::CharShapeIdRef;
    use crate::header::paragraph_head::{ParagraphHeadAlignment, TextOffsetType};

    let hwpx_bullets: Vec<HwpxBullet> = bullets
        .iter()
        .enumerate()
        .map(|(i, bullet)| {
            let paragraph_head = ParagraphHead {
                text: String::new(),
                start: 1,
                level: 1,
                alignment: ParagraphHeadAlignment::Left,
                use_instance_width: true,
                auto_indent: true,
                width_adjust: 0,
                text_offset_type: TextOffsetType::Percent,
                text_offset: 50,
                number_format: NumberFormatType1::Digit, // Bullets don't use number format, but field is required
                character_shape_id_reference: bullet
                    .char_shape_id
                    .map(|id| CharShapeIdRef(id.value())),
                checkable: if bullet.is_checkbox { Some(true) } else { None },
            };

            HwpxBullet {
                image: None,
                paragraph_head,
                id: i as u32,
                character: bullet.char.to_string(),
                checked_character: if bullet.is_checkbox {
                    Some(bullet.char.to_string())
                } else {
                    None
                },
                use_image: false,
            }
        })
        .collect();

    BulletList {
        bullets: hwpx_bullets,
        item_count: bullets.len() as u32,
    }
}

// === 열거형 변환 헬퍼 함수들 ===

/// IR NumberFormat을 HWPX NumberFormatType1으로 변환
fn convert_ir_number_format_to_hwpx(
    format: primitive::NumberFormat,
) -> crate::core::enums::NumberFormatType1 {
    use crate::core::enums::NumberFormatType1;
    use primitive::NumberFormat;

    match format {
        NumberFormat::Digit => NumberFormatType1::Digit,
        NumberFormat::CircledDigit => NumberFormatType1::CircledDigit,
        NumberFormat::RomanUpper => NumberFormatType1::RomanCapital,
        NumberFormat::RomanLower => NumberFormatType1::RomanSmall,
        NumberFormat::LatinUpper => NumberFormatType1::LatinCapital,
        NumberFormat::LatinLower => NumberFormatType1::LatinSmall,
        NumberFormat::CircledLatinUpper => NumberFormatType1::CircledLatinCapital,
        NumberFormat::CircledLatinLower => NumberFormatType1::CircledLatinSmall,
        NumberFormat::HangulSyllable => NumberFormatType1::HangulSyllable,
        NumberFormat::CircledHangul => NumberFormatType1::CircledHangulSyllable,
        NumberFormat::HangulJamo => NumberFormatType1::HangulJamo,
        NumberFormat::CircledHangulJamo => NumberFormatType1::CircledHangulJamo,
        NumberFormat::HangulIdeograph => NumberFormatType1::HangulPhonetic,
        NumberFormat::Ideograph => NumberFormatType1::Ideograph,
        NumberFormat::CircledIdeograph => NumberFormatType1::CircledIdeograph,
        NumberFormat::Ganji => NumberFormatType1::Digit, // Ganji는 HWPX에 정확히 대응되는 값이 없으므로 Digit으로 폴백
    }
}

fn convert_underline_position_to_hwpx(pos: &IrUnderlinePosition) -> HwpxUnderlinePosition {
    match pos {
        IrUnderlinePosition::Bottom => HwpxUnderlinePosition::Bottom,
        IrUnderlinePosition::Top => HwpxUnderlinePosition::Top,
    }
}

fn convert_underline_type_to_hwpx(line_type: &IrUnderlineType) -> LineStyleType2 {
    match line_type {
        IrUnderlineType::None => LineStyleType2::None,
        IrUnderlineType::Single | IrUnderlineType::Thick | IrUnderlineType::Wave => {
            LineStyleType2::Solid
        }
        IrUnderlineType::Double => LineStyleType2::DoubleSlim,
        IrUnderlineType::Dotted => LineStyleType2::Dot,
        IrUnderlineType::Dash => LineStyleType2::Dash,
        IrUnderlineType::DashDot => LineStyleType2::DashDot,
        IrUnderlineType::DashDotDot => LineStyleType2::DashDotDot,
    }
}

fn convert_strikethrough_type_to_hwpx(st: &IrStrikethroughType) -> LineStyleType2 {
    match st {
        IrStrikethroughType::None => LineStyleType2::None,
        IrStrikethroughType::Single => LineStyleType2::Solid,
        IrStrikethroughType::Double => LineStyleType2::DoubleSlim,
    }
}

fn convert_outline_type_to_hwpx(outline: &IrOutlineType) -> LineStyleType1 {
    match outline {
        IrOutlineType::None => LineStyleType1::None,
        IrOutlineType::Outline
        | IrOutlineType::Shadow
        | IrOutlineType::Emboss
        | IrOutlineType::Engrave => LineStyleType1::Solid,
    }
}

fn convert_shadow_type_to_hwpx(shadow: &IrShadowType) -> HwpxShadowType {
    match shadow {
        IrShadowType::None => HwpxShadowType::None,
        // Discrete types map to Drop
        IrShadowType::BottomRight
        | IrShadowType::BottomLeft
        | IrShadowType::TopRight
        | IrShadowType::TopLeft
        | IrShadowType::TopLeftDiscrete
        | IrShadowType::TopRightDiscrete
        | IrShadowType::BottomLeftDiscrete
        | IrShadowType::BottomRightDiscrete => HwpxShadowType::Drop,
        // Continuous types map to Continuous
        IrShadowType::TopLeftContinuous
        | IrShadowType::TopRightContinuous
        | IrShadowType::BottomLeftContinuous
        | IrShadowType::BottomRightContinuous => HwpxShadowType::Continuous,
    }
}

fn convert_emphasis_type_to_hwpx(emphasis: &IrEmphasisType) -> EmphasisMarkType {
    match emphasis {
        IrEmphasisType::None => EmphasisMarkType::None,
        IrEmphasisType::Dot => EmphasisMarkType::DotAbove,
        IrEmphasisType::Circle => EmphasisMarkType::Side,
        IrEmphasisType::CircleOpen => EmphasisMarkType::RingAbove,
        // Comma, Colon, Caron, Tilde all map to Colon in HWPX (no exact variants in HWPX)
        IrEmphasisType::Comma
        | IrEmphasisType::Colon
        | IrEmphasisType::Caron
        | IrEmphasisType::Tilde => EmphasisMarkType::Colon,
    }
}

fn convert_alignment_to_hwpx(align: &IrAlignment) -> HorizontalAlignment {
    match align {
        IrAlignment::Left => HorizontalAlignment::Left,
        IrAlignment::Center => HorizontalAlignment::Center,
        IrAlignment::Right => HorizontalAlignment::Right,
        IrAlignment::Justify => HorizontalAlignment::Justify,
        IrAlignment::Distribute => HorizontalAlignment::Distribute,
        IrAlignment::Divide => HorizontalAlignment::DistributeSpace,
    }
}

fn convert_vertical_alignment_to_hwpx(align: &IrVerticalAlignment) -> HwpxVerticalAlignment {
    match align {
        IrVerticalAlignment::Top => HwpxVerticalAlignment::Top,
        IrVerticalAlignment::Middle => HwpxVerticalAlignment::Center,
        IrVerticalAlignment::Bottom => HwpxVerticalAlignment::Bottom,
        IrVerticalAlignment::Baseline => HwpxVerticalAlignment::Baseline,
    }
}

fn convert_line_spacing_type_to_hwpx(spacing_type: &IrLineSpacingType) -> HwpxLineSpacingType {
    match spacing_type {
        IrLineSpacingType::Percent => HwpxLineSpacingType::Percent,
        IrLineSpacingType::Fixed => HwpxLineSpacingType::Fixed,
        IrLineSpacingType::FontBased => HwpxLineSpacingType::BetweenLines,
        IrLineSpacingType::AtLeast => HwpxLineSpacingType::AtLeast,
    }
}

fn convert_break_latin_to_hwpx(break_type: &LineBreakLatin) -> LatinWordBreak {
    match break_type {
        LineBreakLatin::Word => LatinWordBreak::KeepWord,
        LineBreakLatin::Hyphenation => LatinWordBreak::Hyphenation,
        LineBreakLatin::Character => LatinWordBreak::BreakWord,
    }
}

fn convert_break_korean_to_hwpx(break_type: &LineBreakKorean) -> NonLatinWordBreak {
    match break_type {
        LineBreakKorean::Word => NonLatinWordBreak::KeepWord,
        LineBreakKorean::Character => NonLatinWordBreak::BreakWord,
    }
}

fn convert_text_direction_to_hwpx(
    direction: &primitive::TextDirection,
) -> crate::paragraph::TextDirection {
    use crate::paragraph::TextDirection;
    use primitive::TextDirection as IrTextDir;

    match direction {
        IrTextDir::Horizontal => TextDirection::Horizontal,
        IrTextDir::Vertical => TextDirection::Vertical,
        IrTextDir::VerticalRightToLeft => TextDirection::Vertical, // 근사값
        IrTextDir::RightToLeft => TextDirection::Horizontal,       // 근사값
    }
}

fn convert_line_wrap_to_hwpx(
    line_wrap: &primitive::LineWrap,
) -> crate::paragraph::ParagraphLineWrap {
    use crate::paragraph::ParagraphLineWrap;
    use primitive::LineWrap as IrLineWrap;

    match line_wrap {
        IrLineWrap::Break => ParagraphLineWrap::Break,
        IrLineWrap::Squeeze => ParagraphLineWrap::Squeeze,
        IrLineWrap::Keep => ParagraphLineWrap::Keep,
    }
}

fn convert_paragraph_vertical_alignment_to_hwpx(
    align: &IrVerticalAlignment,
) -> crate::paragraph::ParagraphVerticalAlignment {
    use crate::paragraph::ParagraphVerticalAlignment;

    match align {
        IrVerticalAlignment::Top => ParagraphVerticalAlignment::Top,
        IrVerticalAlignment::Middle => ParagraphVerticalAlignment::Center,
        IrVerticalAlignment::Bottom => ParagraphVerticalAlignment::Bottom,
        // Baseline is mapped to Top for paragraph vertical alignment (closest equivalent)
        IrVerticalAlignment::Baseline => ParagraphVerticalAlignment::Top,
    }
}

// === 컨트롤 변환 (표 등) ===

/// 컨트롤 변환 (표, 그림 등)
fn convert_control_to_hwpx(ctrl: &IrControl) -> Result<Option<HwpxRunContent>, ConversionError> {
    use crate::paragraph::{Control, ControlItem};

    match ctrl {
        IrControl::Table(table) => {
            let hwpx_table = convert_table_to_hwpx(table)?;
            Ok(Some(HwpxRunContent::Table(Box::new(hwpx_table))))
        }
        IrControl::Picture(picture) => {
            let hwpx_picture = convert_picture_to_hwpx(picture)?;
            Ok(Some(HwpxRunContent::Picture(Box::new(hwpx_picture))))
        }
        IrControl::Equation(equation) => {
            let hwpx_equation = convert_equation_to_hwpx(equation)?;
            Ok(Some(HwpxRunContent::Equation(Box::new(hwpx_equation))))
        }
        IrControl::Header(header) => {
            let hwpx_header = convert_header_to_hwpx(header)?;
            let control = Control {
                items: vec![ControlItem::Header(hwpx_header)],
            };
            Ok(Some(HwpxRunContent::Control(control)))
        }
        IrControl::Footer(footer) => {
            let hwpx_footer = convert_footer_to_hwpx(footer)?;
            let control = Control {
                items: vec![ControlItem::Footer(hwpx_footer)],
            };
            Ok(Some(HwpxRunContent::Control(control)))
        }
        IrControl::Footnote(note) => {
            let hwpx_note = convert_note_to_hwpx(note)?;
            let control = Control {
                items: vec![ControlItem::Footnote(hwpx_note)],
            };
            Ok(Some(HwpxRunContent::Control(control)))
        }
        IrControl::Endnote(note) => {
            let hwpx_note = convert_note_to_hwpx(note)?;
            let control = Control {
                items: vec![ControlItem::Endnote(hwpx_note)],
            };
            Ok(Some(HwpxRunContent::Control(control)))
        }
        IrControl::Hyperlink(link) => {
            let hwpx_field = convert_hyperlink_to_hwpx(link)?;
            let control = Control {
                items: vec![ControlItem::FieldBegin(hwpx_field)],
            };
            Ok(Some(HwpxRunContent::Control(control)))
        }
        IrControl::Bookmark(bookmark) => {
            let hwpx_bookmark = convert_bookmark_to_hwpx(bookmark)?;
            let control = Control {
                items: vec![ControlItem::Bookmark(hwpx_bookmark)],
            };
            Ok(Some(HwpxRunContent::Control(control)))
        }
        IrControl::IndexMark(index_mark) => {
            let hwpx_index_mark = convert_index_mark_to_hwpx(index_mark)?;
            let control = Control {
                items: vec![ControlItem::IndexMark(hwpx_index_mark)],
            };
            Ok(Some(HwpxRunContent::Control(control)))
        }
        IrControl::AutoNumber(auto_num) => {
            // Page 타입의 AutoNumber는 HWPX PageNumber로 변환
            if auto_num.number_type == IrAutoNumberType::Page {
                let hwpx_page_num = convert_auto_number_to_page_number(auto_num)?;
                let control = Control {
                    items: vec![ControlItem::PageNumber(hwpx_page_num)],
                };
                Ok(Some(HwpxRunContent::Control(control)))
            } else {
                let hwpx_auto_num = convert_auto_number_to_hwpx(auto_num)?;
                let control = Control {
                    items: vec![ControlItem::AutoNumber(hwpx_auto_num)],
                };
                Ok(Some(HwpxRunContent::Control(control)))
            }
        }
        IrControl::NewNumber(new_num) => {
            let hwpx_new_num = convert_new_number_to_hwpx(new_num)?;
            let control = Control {
                items: vec![ControlItem::NewNumber(hwpx_new_num)],
            };
            Ok(Some(HwpxRunContent::Control(control)))
        }
        IrControl::HiddenComment(comment) => {
            let hwpx_comment = convert_hidden_comment_to_hwpx(comment)?;
            let control = Control {
                items: vec![ControlItem::HiddenComment(hwpx_comment)],
            };
            Ok(Some(HwpxRunContent::Control(control)))
        }
        IrControl::Shape(shape) => {
            let hwpx_shape = convert_shape_to_hwpx(shape)?;
            Ok(hwpx_shape)
        }
        IrControl::TextBox(text_box) => {
            let hwpx_rect = convert_textbox_to_hwpx(text_box)?;
            Ok(Some(HwpxRunContent::Rectangle(Box::new(hwpx_rect))))
        }
        IrControl::Video(video) => {
            let hwpx_video = convert_video_to_hwpx(video)?;
            Ok(Some(HwpxRunContent::Video(Box::new(hwpx_video))))
        }
        IrControl::Ole(ole) => {
            let hwpx_ole = convert_ole_to_hwpx(ole)?;
            Ok(Some(HwpxRunContent::Ole(Box::new(hwpx_ole))))
        }
        IrControl::Chart(chart) => {
            let hwpx_chart = convert_chart_to_hwpx(chart)?;
            Ok(Some(HwpxRunContent::Chart(Box::new(hwpx_chart))))
        }
        IrControl::FormObject(form) => {
            let hwpx_form = convert_form_object_to_hwpx(form)?;
            Ok(hwpx_form)
        }
        IrControl::TextArt(text_art) => {
            let hwpx_text_art = convert_text_art_to_hwpx(text_art)?;
            Ok(Some(HwpxRunContent::TextArt(Box::new(hwpx_text_art))))
        }
        IrControl::Memo(_memo) => {
            // Memo는 HWPX에서 HiddenComment와 유사하게 처리 가능
            // 현재는 무시
            Ok(None)
        }
        IrControl::Unknown(_) => Ok(None),
    }
}

/// IR 표 → HWPX 표 변환
fn convert_table_to_hwpx(table: &IrTable) -> Result<HwpxTable, ConversionError> {
    use crate::core::types::BorderFillIdRef;
    use crate::paragraph::enums::TablePageBreak;

    // 캡션 변환
    let caption = if let Some(ref ir_caption) = table.common.caption {
        Some(convert_ir_caption_to_hwpx(ir_caption)?)
    } else {
        None
    };

    // 페이지 나눔 설정 변환
    let page_break = match table.page_break {
        ir::table::TablePageBreak::None => TablePageBreak::None,
        ir::table::TablePageBreak::Cell => TablePageBreak::Cell,
        ir::table::TablePageBreak::Table => TablePageBreak::Table,
    };

    // 안쪽 여백 변환
    let inside_margin = table.inside_margin.map(|m| crate::paragraph::InsideMargin {
        left: m.left.value() as u32,
        right: m.right.value() as u32,
        top: m.top.value() as u32,
        bottom: m.bottom.value() as u32,
    });

    let mut hwpx_table = HwpxTable {
        // AbstractShapeObjectType 속성들 - ObjectCommon에서 변환
        size: convert_object_common_to_size(&table.common),
        position: convert_object_common_to_position(&table.common),
        outside_margin: None,
        caption,
        shape_comment: None,
        meta_tag: None,
        inside_margin,
        cell_zone_list: None,
        rows: Vec::new(),
        label: None,
        id: table.common.id,
        z_order: table.common.z_order,
        numbering_type: Default::default(),
        text_wrap: Some(convert_text_wrap_to_mode(&table.common.text_wrap)),
        text_flow: convert_text_wrap_to_flow(&table.common.text_wrap),
        lock: table.lock,
        page_break,
        repeat_header: table.repeat_header,
        no_adjust: table.no_adjust,
        row_count: Some(table.row_count as u32),
        column_count: Some(table.column_count as u32),
        cell_spacing: table.cell_spacing.value() as u32,
        border_fill_id_ref: table.border_fill_id.map(|id| BorderFillIdRef(id.value())),
    };

    // 셀존 목록 변환
    if !table.zones.is_empty() {
        let cell_zones: Vec<CellZone> =
            table.zones.iter().map(convert_table_zone_to_hwpx).collect();
        hwpx_table.cell_zone_list = Some(CellZoneList { cell_zones });
    }

    // 행 변환
    for ir_row in &table.rows {
        let mut hwpx_row = HwpxTableRow { cells: Vec::new() };

        for ir_cell in &ir_row.cells {
            let hwpx_cell = convert_table_cell_to_hwpx(ir_cell)?;
            hwpx_row.cells.push(hwpx_cell);
        }

        hwpx_table.rows.push(hwpx_row);
    }

    Ok(hwpx_table)
}

/// IR 표 영역(존) → HWPX CellZone 변환
fn convert_table_zone_to_hwpx(zone: &IrTableZone) -> CellZone {
    use crate::core::types::BorderFillIdRef;

    CellZone {
        start_row_address: Some(zone.start_row as u32),
        start_column_address: Some(zone.start_column as u32),
        end_row_address: Some(zone.end_row as u32),
        end_column_address: Some(zone.end_column as u32),
        border_fill_id_ref: zone.border_fill_id.map(|id| BorderFillIdRef(id.value())),
    }
}

/// IR 표 셀 → HWPX 표 셀 변환
fn convert_table_cell_to_hwpx(cell: &IrTableCell) -> Result<HwpxTableCell, ConversionError> {
    use crate::core::types::BorderFillIdRef;
    use crate::paragraph::enums::{ParagraphLineWrap, ParagraphVerticalAlignment, TextDirection};

    // 셀 내용 (문단들) 변환
    let paragraphs: Vec<HwpxParagraph> = cell
        .paragraphs
        .iter()
        .enumerate()
        .map(|(i, para)| convert_paragraph(para, i as u32))
        .collect::<Result<Vec<_>, _>>()?;

    let paragraph_list = ParagraphList {
        paragraphs,
        id: format!("{}_{}", cell.row, cell.column),
        text_direction: TextDirection::Horizontal,
        line_wrap: ParagraphLineWrap::Break,
        vertical_alignment: match cell.vertical_alignment {
            IrVerticalAlignment::Top => ParagraphVerticalAlignment::Top,
            IrVerticalAlignment::Middle => ParagraphVerticalAlignment::Center,
            IrVerticalAlignment::Bottom => ParagraphVerticalAlignment::Bottom,
            IrVerticalAlignment::Baseline => ParagraphVerticalAlignment::Top,
        },
        link_list_id_reference: None,
        link_list_next_id_reference: None,
        text_width: Some(cell.width.value() as u32),
        text_height: Some(cell.height.value() as u32),
        has_text_reference: false,
        has_number_reference: false,
    };

    let hwpx_cell = HwpxTableCell {
        paragraph_list,
        cell_address: CellAddress {
            column_address: Some(cell.column as u32),
            row_address: Some(cell.row as u32),
        },
        cell_span: CellSpan {
            column_span: cell.column_span as u32,
            row_span: cell.row_span as u32,
        },
        cell_size: CellSize {
            width: Some(cell.width.value() as u32),
            height: Some(cell.height.value() as u32),
        },
        cell_margin: CellMargin {
            left: cell.padding.left.value() as u32,
            right: cell.padding.right.value() as u32,
            top: cell.padding.top.value() as u32,
            bottom: cell.padding.bottom.value() as u32,
        },
        name: cell.name.clone(),
        header: cell.is_header,
        has_margin: cell.padding != primitive::Insets::ZERO,
        protect: cell.protect,
        editable: cell.editable,
        dirty: false,
        border_fill_id_ref: cell.border_fill_id.map(|id| BorderFillIdRef(id.value())),
    };

    Ok(hwpx_cell)
}

/// IR 그림 → HWPX 그림 변환
fn convert_picture_to_hwpx(
    picture: &IrPicture,
) -> Result<crate::paragraph::Picture, ConversionError> {
    use crate::core::types::{BinaryItemIdRef, Image};
    use crate::paragraph::shape_common::{
        CurrentSize, Flip, ImageClip, InsideMargin, OriginalSize, RenderingInfo, RotationInfo,
        ShapeComponentOffset,
    };

    // 원본 크기
    let original_size = OriginalSize {
        width: Some(picture.original_size.width.value() as u32),
        height: Some(picture.original_size.height.value() as u32),
    };

    // 현재 크기 (원본 크기와 동일하게 설정)
    let current_size = CurrentSize {
        width: Some(picture.original_size.width.value() as u32),
        height: Some(picture.original_size.height.value() as u32),
    };

    // 뒤집기 변환
    let flip = Flip {
        horizontal: matches!(picture.flip, IrImageFlip::Horizontal | IrImageFlip::Both),
        vertical: matches!(picture.flip, IrImageFlip::Vertical | IrImageFlip::Both),
    };

    // 회전 정보
    let rotation_info = RotationInfo {
        angle: picture.rotation as i32,
        center_x: Some(0),
        center_y: Some(0),
        rotate_image: Some(false),
    };

    // 이미지 클리핑
    let image_clip = if picture.crop.left.value() != 0
        || picture.crop.right.value() != 0
        || picture.crop.top.value() != 0
        || picture.crop.bottom.value() != 0
    {
        Some(ImageClip {
            left: Some(picture.crop.left.value()),
            right: Some(picture.crop.right.value()),
            top: Some(picture.crop.top.value()),
            bottom: Some(picture.crop.bottom.value()),
        })
    } else {
        None
    };

    // 안쪽 여백
    let inside_margin = if picture.inside_margin != primitive::Insets::ZERO {
        Some(InsideMargin {
            left: picture.inside_margin.left.value() as u32,
            right: picture.inside_margin.right.value() as u32,
            top: picture.inside_margin.top.value() as u32,
            bottom: picture.inside_margin.bottom.value() as u32,
        })
    } else {
        None
    };

    // 이미지 정보
    let image = Some(Image {
        binary_item_id_reference: BinaryItemIdRef(picture.binary_id.value().to_string()),
        brightness: picture.brightness as i32,
        contrast: picture.contrast as i32,
        effect: convert_image_effect_to_hwpx(&picture.effect),
        alpha: Some(picture.alpha as f32),
    });

    // 캡션 변환
    let caption = if let Some(ref ir_caption) = picture.common.caption {
        Some(convert_ir_caption_to_hwpx(ir_caption)?)
    } else {
        None
    };

    // 그림자 효과 변환
    let effects = picture.shadow.as_ref().map(convert_ir_shadow_to_effects);

    let hwpx_picture = crate::paragraph::Picture {
        // ObjectCommon에서 변환
        size: convert_object_common_to_size(&picture.common),
        position: convert_object_common_to_position(&picture.common),
        outside_margin: None,
        caption,
        shape_comment: None,
        meta_tag: None,
        offset: ShapeComponentOffset {
            x: picture.common.position.x.value().max(0) as u32,
            y: picture.common.position.y.value().max(0) as u32,
        },
        original_size,
        current_size,
        flip,
        rotation_info,
        rendering_info: RenderingInfo::default(),
        line_shape: None,
        image_rectangle: None,
        image_clip,
        effects,
        inside_margin,
        image_dimension: None,
        image,
        id: picture.common.id,
        z_order: picture.common.z_order,
        numbering_type: Default::default(),
        text_wrap: Some(convert_text_wrap_to_mode(&picture.common.text_wrap)),
        text_flow: convert_text_wrap_to_flow(&picture.common.text_wrap),
        lock: false,
        href: None,
        group_level: 0,
        instance_id: None,
        reverse: None,
    };

    Ok(hwpx_picture)
}

/// IR 이미지 효과 → HWPX ImageEffect 변환
fn convert_image_effect_to_hwpx(effect: &IrImageEffect) -> crate::core::enums::ImageEffect {
    use crate::core::enums::ImageEffect;
    match effect {
        IrImageEffect::Original => ImageEffect::RealPicture,
        IrImageEffect::Grayscale => ImageEffect::GrayScale,
        IrImageEffect::BlackWhite => ImageEffect::BlackWhite,
        IrImageEffect::Pattern => ImageEffect::RealPicture, // HWPX에는 패턴 효과가 없음
    }
}

/// IR PictureShadow → HWPX Effects 변환
fn convert_ir_shadow_to_effects(
    shadow: &ir::picture::PictureShadow,
) -> crate::paragraph::effects::Effects {
    use crate::paragraph::effects::{
        AdvancedShadowEffect, AdvancedShadowStyle, Effects, EffectsColor, EffectsColorKind,
        EffectsColorValue, EffectsRgbColor, Scale, Skew,
    };
    use ir::picture::PictureShadowType;

    // 그림자 방향 계산 (오프셋에서 각도 추론)
    let direction = match shadow.shadow_type {
        PictureShadowType::TopLeft => Some(135),
        PictureShadowType::TopRight => Some(45),
        PictureShadowType::BottomLeft => Some(225),
        PictureShadowType::BottomRight => Some(315),
        PictureShadowType::None => None,
    };

    // 거리 계산 (오프셋에서)
    let distance =
        ((shadow.offset_x.value().pow(2) + shadow.offset_y.value().pow(2)) as f64).sqrt() as f32;

    // 색상 변환
    let effects_color = EffectsColor {
        value: Some(EffectsColorValue::Rgb(EffectsRgbColor {
            red: shadow.color.red as u32,
            green: shadow.color.green as u32,
            blue: shadow.color.blue as u32,
        })),
        color_type: EffectsColorKind::Rgb,
        scheme_index: None,
        system_index: None,
        preset_index: None,
    };

    let advanced_shadow = AdvancedShadowEffect {
        skew: Skew { x: None, y: None },
        scale: Scale {
            x: Some(1.0),
            y: Some(1.0),
        },
        effects_color,
        style: Some(AdvancedShadowStyle::Outside),
        alpha: Some(shadow.alpha as f32),
        radius: Some(10.0), // 기본 흐림 반경
        direction,
        distance: Some(distance),
        align_style: None,
        rotation_style: Some(false),
    };

    Effects {
        shadow: Some(advanced_shadow),
        glow: None,
        soft_edge: None,
        reflection: None,
    }
}

/// IR 수식 → HWPX 수식 변환
fn convert_equation_to_hwpx(
    equation: &IrEquation,
) -> Result<crate::paragraph::Equation, ConversionError> {
    use crate::paragraph::shape_common::{EquationLineMode, ShapeNumberingType};

    let text_color = equation
        .color
        .as_ref()
        .map(|c| RgbColor {
            r: c.red,
            g: c.green,
            b: c.blue,
            a: c.alpha,
        })
        .unwrap_or_else(RgbColor::black);

    // ObjectCommon → HWPX 속성 변환
    let size = convert_object_common_to_size(&equation.common);
    let position = convert_object_common_to_position(&equation.common);
    let text_wrap = Some(convert_text_wrap_to_mode(&equation.common.text_wrap));
    let text_flow = convert_text_wrap_to_flow(&equation.common.text_wrap);

    // 캡션 변환
    let caption = if let Some(ref ir_caption) = equation.common.caption {
        Some(convert_ir_caption_to_hwpx(ir_caption)?)
    } else {
        None
    };

    Ok(crate::paragraph::Equation {
        size,
        position,
        outside_margin: None,
        caption,
        shape_comment: None,
        meta_tag: None,
        script: equation.script.clone(),
        id: equation.common.id,
        z_order: equation.common.z_order,
        numbering_type: ShapeNumberingType::default(),
        text_wrap,
        text_flow,
        lock: false,
        version: "Equation Version 60".to_string(),
        baseline: equation.baseline_offset.value() as u32,
        text_color,
        base_unit: equation.font_size.value() as u32,
        line_mode: EquationLineMode::default(),
        font: "HYhwpEQ".to_string(),
    })
}

// =============================================================================
// 컨트롤 아이템 변환 함수들 (Header, Footer, Notes, etc.)
// =============================================================================

/// IR 머리글 → HWPX HeaderFooter 변환
fn convert_header_to_hwpx(
    header: &IrHeaderFooter,
) -> Result<crate::paragraph::HeaderFooter, ConversionError> {
    use crate::paragraph::enums::{
        PageStartsOn, ParagraphLineWrap, ParagraphVerticalAlignment, TextDirection,
    };

    let apply_page_type = match header.apply_to {
        HeaderFooterApplyTo::Both => PageStartsOn::Both,
        HeaderFooterApplyTo::Even => PageStartsOn::Even,
        HeaderFooterApplyTo::Odd => PageStartsOn::Odd,
        HeaderFooterApplyTo::First => PageStartsOn::Both, // HWPX doesn't have First
    };

    let paragraphs: Vec<HwpxParagraph> = header
        .paragraphs
        .iter()
        .enumerate()
        .map(|(i, para)| convert_paragraph(para, i as u32))
        .collect::<Result<Vec<_>, _>>()?;

    let sub_list = ParagraphList {
        paragraphs,
        id: "header".to_string(),
        text_direction: TextDirection::Horizontal,
        line_wrap: ParagraphLineWrap::Break,
        vertical_alignment: ParagraphVerticalAlignment::Top,
        link_list_id_reference: None,
        link_list_next_id_reference: None,
        text_width: None,
        text_height: None,
        has_text_reference: false,
        has_number_reference: false,
    };

    Ok(crate::paragraph::HeaderFooter {
        sub_list,
        id: None,
        apply_page_type,
    })
}

/// IR 바닥글 → HWPX HeaderFooter 변환
fn convert_footer_to_hwpx(
    footer: &IrHeaderFooter,
) -> Result<crate::paragraph::HeaderFooter, ConversionError> {
    use crate::paragraph::enums::{
        PageStartsOn, ParagraphLineWrap, ParagraphVerticalAlignment, TextDirection,
    };

    let apply_page_type = match footer.apply_to {
        HeaderFooterApplyTo::Both => PageStartsOn::Both,
        HeaderFooterApplyTo::Even => PageStartsOn::Even,
        HeaderFooterApplyTo::Odd => PageStartsOn::Odd,
        HeaderFooterApplyTo::First => PageStartsOn::Both, // HWPX doesn't have First
    };

    let paragraphs: Vec<HwpxParagraph> = footer
        .paragraphs
        .iter()
        .enumerate()
        .map(|(i, para)| convert_paragraph(para, i as u32))
        .collect::<Result<Vec<_>, _>>()?;

    let sub_list = ParagraphList {
        paragraphs,
        id: "footer".to_string(),
        text_direction: TextDirection::Horizontal,
        line_wrap: ParagraphLineWrap::Break,
        vertical_alignment: ParagraphVerticalAlignment::Top,
        link_list_id_reference: None,
        link_list_next_id_reference: None,
        text_width: None,
        text_height: None,
        has_text_reference: false,
        has_number_reference: false,
    };

    Ok(crate::paragraph::HeaderFooter {
        sub_list,
        id: None,
        apply_page_type,
    })
}

/// IR 각주/미주 → HWPX Note 변환
fn convert_note_to_hwpx(note: &IrNote) -> Result<crate::paragraph::Note, ConversionError> {
    use crate::paragraph::enums::{ParagraphLineWrap, ParagraphVerticalAlignment, TextDirection};

    let paragraphs: Vec<HwpxParagraph> = note
        .paragraphs
        .iter()
        .enumerate()
        .map(|(i, para)| convert_paragraph(para, i as u32))
        .collect::<Result<Vec<_>, _>>()?;

    let sub_list = ParagraphList {
        paragraphs,
        id: "note".to_string(),
        text_direction: TextDirection::Horizontal,
        line_wrap: ParagraphLineWrap::Break,
        vertical_alignment: ParagraphVerticalAlignment::Top,
        link_list_id_reference: None,
        link_list_next_id_reference: None,
        text_width: None,
        text_height: None,
        has_text_reference: false,
        has_number_reference: false,
    };

    Ok(crate::paragraph::Note {
        sub_list,
        instance_id: note.instance_id,
    })
}

/// IR 하이퍼링크 → HWPX FieldBegin 변환
fn convert_hyperlink_to_hwpx(
    link: &IrHyperlink,
) -> Result<crate::paragraph::FieldBegin, ConversionError> {
    use crate::paragraph::{FieldType, ParameterItem, ParameterList, StringParameter};

    let url = match &link.target {
        HyperlinkTarget::Url(url) => url.clone(),
        HyperlinkTarget::Email(email) => format!("mailto:{}", email),
        HyperlinkTarget::File(path) => path.clone(),
        HyperlinkTarget::Bookmark(name) => format!("#{}", name),
    };

    let parameters = Some(ParameterList {
        items: vec![ParameterItem::String(StringParameter {
            value: url,
            name: Some("url".to_string()),
        })],
        count: 1,
        name: None,
    });

    // display_text는 FieldBegin의 name 필드에 저장하거나 별도 처리
    // HWPX에서 실제로 display_text를 저장하는 위치는 필드 내부 텍스트로 처리됨
    // subList는 복잡한 내용이 있을 때만 사용하므로 일반적으로는 None
    let sub_list = None;

    Ok(crate::paragraph::FieldBegin {
        parameters,
        sub_list,
        meta_tag: None,
        id: 0,
        field_type: FieldType::Hyperlink,
        name: link.tooltip.clone(),
        editable: true,
        dirty: false,
        z_order: None,
        field_id: None,
    })
}

/// IR 책갈피 → HWPX Bookmark 변환
fn convert_bookmark_to_hwpx(
    bookmark: &IrBookmark,
) -> Result<crate::paragraph::Bookmark, ConversionError> {
    Ok(crate::paragraph::Bookmark {
        name: Some(bookmark.name.clone()),
    })
}

/// IR 색인 표시 → HWPX IndexMark 변환
fn convert_index_mark_to_hwpx(
    index_mark: &ir::control::IndexMark,
) -> Result<crate::paragraph::IndexMark, ConversionError> {
    Ok(crate::paragraph::IndexMark {
        first_key: index_mark.first_key.clone(),
        second_key: index_mark.second_key.clone(),
    })
}

/// IR 자동 번호 → HWPX AutoNumberNewNumber 변환
fn convert_auto_number_to_hwpx(
    auto_num: &IrAutoNumber,
) -> Result<crate::paragraph::AutoNumberNewNumber, ConversionError> {
    use crate::paragraph::AutoNumberKind;
    use crate::paragraph::section_definition::AutoNumberFormat;

    let number_type = match auto_num.number_type {
        IrAutoNumberType::Page => Some(AutoNumberKind::Page),
        IrAutoNumberType::Footnote => Some(AutoNumberKind::Footnote),
        IrAutoNumberType::Endnote => Some(AutoNumberKind::Endnote),
        IrAutoNumberType::Picture => Some(AutoNumberKind::Picture),
        IrAutoNumberType::Table => Some(AutoNumberKind::Table),
        IrAutoNumberType::Equation => Some(AutoNumberKind::Equation),
        IrAutoNumberType::TotalPages => Some(AutoNumberKind::TotalPage),
    };

    let auto_number_format = if let Some(ref fmt) = auto_num.auto_number_format {
        Some(AutoNumberFormat {
            number_type: convert_number_format_to_hwpx(&auto_num.number_format),
            user_character: fmt.user_character.clone(),
            prefix_character: fmt.prefix_character.clone(),
            suffix_character: fmt.suffix_character.clone(),
            superscript: fmt.superscript,
        })
    } else {
        Some(AutoNumberFormat {
            number_type: convert_number_format_to_hwpx(&auto_num.number_format),
            user_character: None,
            prefix_character: None,
            suffix_character: ")".to_string(),
            superscript: false,
        })
    };

    Ok(crate::paragraph::AutoNumberNewNumber {
        auto_number_format,
        number: 1,
        number_type,
    })
}

/// IR 자동 번호(페이지) → HWPX PageNumber 변환
fn convert_auto_number_to_page_number(
    auto_num: &IrAutoNumber,
) -> Result<crate::paragraph::PageNumber, ConversionError> {
    use crate::paragraph::PageNumberPosition;

    // auto_number_format에서 position, format_type, side_character 추출
    let (position, format_type, side_character) = if let Some(ref fmt) = auto_num.auto_number_format
    {
        let pos = fmt
            .position
            .unwrap_or(primitive::PageNumberPosition::TopLeft);
        let hwpx_pos = match pos {
            primitive::PageNumberPosition::None => PageNumberPosition::None,
            primitive::PageNumberPosition::TopLeft => PageNumberPosition::TopLeft,
            primitive::PageNumberPosition::TopCenter => PageNumberPosition::TopCenter,
            primitive::PageNumberPosition::TopRight => PageNumberPosition::TopRight,
            primitive::PageNumberPosition::BottomLeft => PageNumberPosition::BottomLeft,
            primitive::PageNumberPosition::BottomCenter => PageNumberPosition::BottomCenter,
            primitive::PageNumberPosition::BottomRight => PageNumberPosition::BottomRight,
            primitive::PageNumberPosition::OutsideTop => PageNumberPosition::OutsideTop,
            primitive::PageNumberPosition::OutsideBottom => PageNumberPosition::OutsideBottom,
            primitive::PageNumberPosition::InsideTop => PageNumberPosition::InsideTop,
            primitive::PageNumberPosition::InsideBottom => PageNumberPosition::InsideBottom,
        };

        let format = if let Some(ref f) = fmt.format_type {
            convert_number_format_to_hwpx_type1(f)
        } else {
            convert_number_format_to_hwpx_type1(&auto_num.number_format)
        };

        let side_ch = fmt
            .side_character
            .clone()
            .unwrap_or_else(|| "-".to_string());
        (hwpx_pos, format, side_ch)
    } else {
        // 기본값 사용
        let format_type = convert_number_format_to_hwpx_type1(&auto_num.number_format);
        (PageNumberPosition::TopLeft, format_type, "-".to_string())
    };

    Ok(crate::paragraph::PageNumber {
        position,
        format_type,
        side_character,
    })
}

fn convert_number_format_to_hwpx_type1(
    format: &IrNumberFormat,
) -> crate::core::enums::NumberFormatType1 {
    use crate::core::enums::NumberFormatType1;

    match format {
        IrNumberFormat::Digit => NumberFormatType1::Digit,
        IrNumberFormat::CircledDigit => NumberFormatType1::CircledDigit,
        IrNumberFormat::RomanUpper => NumberFormatType1::RomanCapital,
        IrNumberFormat::RomanLower => NumberFormatType1::RomanSmall,
        IrNumberFormat::LatinUpper => NumberFormatType1::LatinCapital,
        IrNumberFormat::LatinLower => NumberFormatType1::LatinSmall,
        IrNumberFormat::CircledLatinUpper => NumberFormatType1::CircledLatinCapital,
        IrNumberFormat::CircledLatinLower => NumberFormatType1::CircledLatinSmall,
        IrNumberFormat::HangulSyllable => NumberFormatType1::HangulSyllable,
        IrNumberFormat::CircledHangul => NumberFormatType1::CircledHangulSyllable,
        IrNumberFormat::HangulJamo => NumberFormatType1::HangulJamo,
        IrNumberFormat::CircledHangulJamo => NumberFormatType1::CircledHangulJamo,
        IrNumberFormat::HangulIdeograph => NumberFormatType1::HangulPhonetic,
        IrNumberFormat::Ideograph => NumberFormatType1::Ideograph,
        IrNumberFormat::CircledIdeograph => NumberFormatType1::CircledIdeograph,
        IrNumberFormat::Ganji => NumberFormatType1::HangulPhonetic, // Ganji는 Type1에 없으므로 가장 유사한 것으로 변환
    }
}

/// IR 새 번호 → HWPX AutoNumberNewNumber 변환
fn convert_new_number_to_hwpx(
    new_num: &IrNewNumber,
) -> Result<crate::paragraph::AutoNumberNewNumber, ConversionError> {
    use crate::paragraph::AutoNumberKind;

    let number_type = match new_num.number_type {
        IrAutoNumberType::Page => Some(AutoNumberKind::Page),
        IrAutoNumberType::Footnote => Some(AutoNumberKind::Footnote),
        IrAutoNumberType::Endnote => Some(AutoNumberKind::Endnote),
        IrAutoNumberType::Picture => Some(AutoNumberKind::Picture),
        IrAutoNumberType::Table => Some(AutoNumberKind::Table),
        IrAutoNumberType::Equation => Some(AutoNumberKind::Equation),
        IrAutoNumberType::TotalPages => Some(AutoNumberKind::TotalPage),
    };

    Ok(crate::paragraph::AutoNumberNewNumber {
        auto_number_format: None,
        number: new_num.number as i32,
        number_type,
    })
}

/// IR 숨은 설명 → HWPX HiddenComment 변환
fn convert_hidden_comment_to_hwpx(
    comment: &IrHiddenComment,
) -> Result<crate::paragraph::HiddenComment, ConversionError> {
    use crate::paragraph::enums::{ParagraphLineWrap, ParagraphVerticalAlignment, TextDirection};

    let paragraphs: Vec<HwpxParagraph> = comment
        .paragraphs
        .iter()
        .enumerate()
        .map(|(i, para)| convert_paragraph(para, i as u32))
        .collect::<Result<Vec<_>, _>>()?;

    let sub_list = ParagraphList {
        paragraphs,
        id: "comment".to_string(),
        text_direction: TextDirection::Horizontal,
        line_wrap: ParagraphLineWrap::Break,
        vertical_alignment: ParagraphVerticalAlignment::Top,
        link_list_id_reference: None,
        link_list_next_id_reference: None,
        text_width: None,
        text_height: None,
        has_text_reference: false,
        has_number_reference: false,
    };

    Ok(crate::paragraph::HiddenComment { sub_list })
}

/// IR 번호 형식 → HWPX NumberFormatType2 변환
fn convert_number_format_to_hwpx(format: &IrNumberFormat) -> crate::core::enums::NumberFormatType2 {
    use crate::core::enums::NumberFormatType2;

    match format {
        IrNumberFormat::Digit => NumberFormatType2::Digit,
        IrNumberFormat::CircledDigit => NumberFormatType2::CircledDigit,
        IrNumberFormat::RomanUpper => NumberFormatType2::RomanCapital,
        IrNumberFormat::RomanLower => NumberFormatType2::RomanSmall,
        IrNumberFormat::LatinUpper => NumberFormatType2::LatinCapital,
        IrNumberFormat::LatinLower => NumberFormatType2::LatinSmall,
        IrNumberFormat::CircledLatinUpper => NumberFormatType2::CircledLatinCapital,
        IrNumberFormat::CircledLatinLower => NumberFormatType2::CircledLatinSmall,
        IrNumberFormat::HangulSyllable => NumberFormatType2::HangulSyllable,
        IrNumberFormat::CircledHangul => NumberFormatType2::CircledHangulSyllable,
        IrNumberFormat::HangulJamo => NumberFormatType2::HangulJamo,
        IrNumberFormat::CircledHangulJamo => NumberFormatType2::CircledHangulJamo,
        IrNumberFormat::HangulIdeograph => NumberFormatType2::HangulPhonetic,
        IrNumberFormat::Ideograph => NumberFormatType2::Ideograph,
        IrNumberFormat::CircledIdeograph => NumberFormatType2::CircledIdeograph,
        IrNumberFormat::Ganji => NumberFormatType2::DecagonCircle,
    }
}

// =============================================================================
// TextBox 변환 (IR → HWPX)
// =============================================================================

/// IR 텍스트 박스 → HWPX 도형 (Rectangle with DrawText) 변환
fn convert_textbox_to_hwpx(
    text_box: &ir::control::TextBox,
) -> Result<crate::paragraph::drawing::Rectangle, ConversionError> {
    use crate::core::types::Point;
    use crate::paragraph::drawing::{DrawText, Rectangle, TextMargin};
    use crate::paragraph::line_shape::LineShape;
    use crate::paragraph::para_list::ParagraphList;
    use crate::paragraph::shape_common::{
        CurrentSize, Flip, OriginalSize, RenderingInfo, RotationInfo, ShapeComponentOffset,
    };

    // 크기/위치 변환
    let width = text_box.common.size.width.value().max(0);
    let height = text_box.common.size.height.value().max(0);
    let offset_x = text_box.common.position.x.value().max(0) as u32;
    let offset_y = text_box.common.position.y.value().max(0) as u32;

    let offset = ShapeComponentOffset {
        x: offset_x,
        y: offset_y,
    };
    let original_size = OriginalSize {
        width: Some(width as u32),
        height: Some(height as u32),
    };
    let current_size = CurrentSize {
        width: Some(width as u32),
        height: Some(height as u32),
    };

    // ObjectCommon → HWPX 속성 변환
    let size = convert_object_common_to_size(&text_box.common);
    let position = convert_object_common_to_position(&text_box.common);
    let text_wrap = Some(convert_text_wrap_to_mode(&text_box.common.text_wrap));
    let text_flow = convert_text_wrap_to_flow(&text_box.common.text_wrap);

    // 캡션 변환
    let caption = if let Some(ref ir_caption) = text_box.common.caption {
        Some(convert_ir_caption_to_hwpx(ir_caption)?)
    } else {
        None
    };

    // 문단 변환
    let paragraphs = text_box
        .paragraphs
        .iter()
        .enumerate()
        .filter_map(|(i, p)| convert_paragraph(p, i as u32).ok())
        .collect::<Vec<_>>();

    // DrawText 생성
    let draw_text = DrawText {
        paragraph_list: ParagraphList {
            paragraphs,
            id: "0".to_string(),
            text_direction: convert_text_direction_to_hwpx(&text_box.text_direction),
            line_wrap: convert_line_wrap_to_hwpx(&text_box.line_wrap),
            vertical_alignment: convert_paragraph_vertical_alignment_to_hwpx(
                &text_box.vertical_alignment,
            ),
            link_list_id_reference: text_box
                .link_list_id_reference
                .map(crate::core::types::LinkListIdRef),
            link_list_next_id_reference: text_box
                .link_list_next_id_reference
                .map(crate::core::types::LinkListIdRef),
            text_width: text_box.text_width.map(|w| w.value().max(0) as u32),
            text_height: text_box.text_height.map(|h| h.value().max(0) as u32),
            has_text_reference: text_box.has_text_reference,
            has_number_reference: text_box.has_number_reference,
        },
        text_margin: Some(TextMargin {
            left: text_box.padding.left.value().max(0) as u32,
            right: text_box.padding.right.value().max(0) as u32,
            top: text_box.padding.top.value().max(0) as u32,
            bottom: text_box.padding.bottom.value().max(0) as u32,
        }),
        last_width: text_box.last_width.map(|w| w.value().max(0) as u32),
        name: text_box.name.clone(),
        editable: text_box.editable,
    };

    // 사각형 네 꼭짓점 (텍스트 박스 크기에 맞게)
    let point0 = Point {
        x: Some(0),
        y: Some(0),
    };
    let point1 = Point {
        x: Some(width),
        y: Some(0),
    };
    let point2 = Point {
        x: Some(width),
        y: Some(height),
    };
    let point3 = Point {
        x: Some(0),
        y: Some(height),
    };

    // Rectangle으로 TextBox 표현
    Ok(Rectangle {
        size,
        position,
        outside_margin: None,
        caption,
        shape_comment: None,
        meta_tag: None,
        offset,
        original_size,
        current_size,
        flip: Flip {
            horizontal: false,
            vertical: false,
        },
        rotation_info: RotationInfo {
            angle: 0,
            center_x: Some(0),
            center_y: Some(0),
            rotate_image: Some(false),
        },
        rendering_info: RenderingInfo::default(),
        line_shape: LineShape::default(), // 기본 선 모양 (없음 상태)
        fill_brush: None,
        draw_text: Some(draw_text),
        shadow: None,
        point0,
        point1,
        point2,
        point3,
        id: text_box.common.id,
        z_order: text_box.common.z_order,
        numbering_type: Default::default(),
        text_wrap,
        text_flow,
        lock: false,
        href: None,
        group_level: 0,
        instance_id: None,
        ratio: None,
    })
}

// =============================================================================
// Shape 변환 (IR → HWPX)
// =============================================================================

/// IR 도형 → HWPX 도형 변환
fn convert_shape_to_hwpx(shape: &IrShape) -> Result<Option<HwpxRunContent>, ConversionError> {
    use crate::core::types::Point;
    use crate::paragraph::drawing::{Arc, Curve, Ellipse, Line, Polygon, Rectangle};
    use crate::paragraph::shape_common::{
        CurrentSize, Flip, OriginalSize, RenderingInfo, RotationInfo, ShapeComponentOffset,
    };

    // 공통 속성 변환
    let width = shape.common.size.width.value() as u32;
    let height = shape.common.size.height.value() as u32;
    // ShapeComponentOffset uses u32, so convert with abs or default to 0
    let offset_x = shape.common.position.x.value().max(0) as u32;
    let offset_y = shape.common.position.y.value().max(0) as u32;

    let offset = ShapeComponentOffset {
        x: offset_x,
        y: offset_y,
    };
    let original_size = OriginalSize {
        width: Some(width),
        height: Some(height),
    };
    let current_size = CurrentSize {
        width: Some(width),
        height: Some(height),
    };
    let flip = Flip {
        horizontal: false,
        vertical: false,
    };
    let rotation_info = RotationInfo {
        angle: (shape.rotation as i32) * 10,
        center_x: Some(0),
        center_y: Some(0),
        rotate_image: Some(false),
    };
    let rendering_info = RenderingInfo::default();

    // ObjectCommon → HWPX 속성 변환
    let size = convert_object_common_to_size(&shape.common);
    let position = convert_object_common_to_position(&shape.common);
    let text_wrap = Some(convert_text_wrap_to_mode(&shape.common.text_wrap));
    let text_flow = convert_text_wrap_to_flow(&shape.common.text_wrap);

    // 선 모양 변환 (화살표 미포함 기본 스타일)
    let line_shape = convert_line_style_to_hwpx(&shape.line);

    // 캡션 변환
    let caption = if let Some(ref ir_caption) = shape.common.caption {
        Some(convert_ir_caption_to_hwpx(ir_caption)?)
    } else {
        None
    };

    // 채우기 변환
    let fill_brush = convert_fill_to_hwpx(&shape.fill);

    // 도형 내 텍스트 변환
    let draw_text = shape.text.as_ref().map(|text| {
        use crate::paragraph::drawing::{DrawText, TextMargin};
        use crate::paragraph::enums::{
            ParagraphLineWrap, ParagraphVerticalAlignment, TextDirection,
        };
        use crate::paragraph::para_list::ParagraphList;

        let paragraphs = text
            .paragraphs
            .iter()
            .enumerate()
            .filter_map(|(i, p)| convert_paragraph(p, i as u32).ok())
            .collect::<Vec<_>>();

        DrawText {
            paragraph_list: ParagraphList {
                paragraphs,
                id: "0".to_string(),
                text_direction: match text.text_direction {
                    primitive::TextDirection::Horizontal => TextDirection::Horizontal,
                    primitive::TextDirection::Vertical => TextDirection::Vertical,
                    primitive::TextDirection::VerticalRightToLeft => TextDirection::Vertical,
                    primitive::TextDirection::RightToLeft => TextDirection::Horizontal,
                },
                line_wrap: ParagraphLineWrap::Break,
                vertical_alignment: match text.vertical_alignment {
                    primitive::VerticalAlignment::Top => ParagraphVerticalAlignment::Top,
                    primitive::VerticalAlignment::Middle => ParagraphVerticalAlignment::Center,
                    primitive::VerticalAlignment::Bottom => ParagraphVerticalAlignment::Bottom,
                    primitive::VerticalAlignment::Baseline => ParagraphVerticalAlignment::Top,
                },
                link_list_id_reference: None,
                link_list_next_id_reference: None,
                text_width: None,
                text_height: None,
                has_text_reference: false,
                has_number_reference: false,
            },
            text_margin: Some(TextMargin {
                left: text.padding.left.value().max(0) as u32,
                right: text.padding.right.value().max(0) as u32,
                top: text.padding.top.value().max(0) as u32,
                bottom: text.padding.bottom.value().max(0) as u32,
            }),
            last_width: None,
            name: None,
            editable: text.editable,
        }
    });

    match &shape.shape_type {
        IrShapeType::Line(line) => {
            // Line의 경우 화살표 정보 포함한 선 스타일 사용
            let line_shape_with_arrows = convert_line_style_with_arrows_to_hwpx(
                &shape.line,
                &line.start_arrow,
                &line.end_arrow,
            );
            let hwpx_line = Line {
                size,
                position,
                outside_margin: None,
                caption: caption.clone(),
                shape_comment: None,
                meta_tag: None,
                offset,
                original_size,
                current_size,
                flip,
                rotation_info,
                rendering_info,
                line_shape: line_shape_with_arrows,
                fill_brush: fill_brush.clone(),
                draw_text: draw_text.clone(),
                shadow: None,
                start_point: Point {
                    x: Some(line.start.x.value()),
                    y: Some(line.start.y.value()),
                },
                end_point: Point {
                    x: Some(line.end.x.value()),
                    y: Some(line.end.y.value()),
                },
                id: shape.common.id,
                z_order: shape.common.z_order,
                numbering_type: Default::default(),
                text_wrap,
                text_flow,
                lock: false,
                href: None,
                group_level: 0,
                instance_id: None,
                is_reverse_horizontal_vertical: false,
            };
            Ok(Some(HwpxRunContent::Line(Box::new(hwpx_line))))
        }
        IrShapeType::Rectangle(rect) => {
            let hwpx_rect = Rectangle {
                size,
                position,
                outside_margin: None,
                caption: caption.clone(),
                shape_comment: None,
                meta_tag: None,
                offset,
                original_size,
                current_size,
                flip,
                rotation_info,
                rendering_info,
                line_shape,
                fill_brush: fill_brush.clone(),
                draw_text: draw_text.clone(),
                shadow: None,
                point0: Point {
                    x: Some(0),
                    y: Some(0),
                },
                point1: Point {
                    x: Some(width as i32),
                    y: Some(0),
                },
                point2: Point {
                    x: Some(width as i32),
                    y: Some(height as i32),
                },
                point3: Point {
                    x: Some(0),
                    y: Some(height as i32),
                },
                id: shape.common.id,
                z_order: shape.common.z_order,
                numbering_type: Default::default(),
                text_wrap,
                text_flow,
                lock: false,
                href: None,
                group_level: 0,
                instance_id: None,
                ratio: if rect.corner_radius.value() > 0 {
                    Some(rect.corner_radius.value() as u32)
                } else {
                    None
                },
            };
            Ok(Some(HwpxRunContent::Rectangle(Box::new(hwpx_rect))))
        }
        IrShapeType::Ellipse(ellipse) => {
            let half_width = (width / 2) as i32;
            let half_height = (height / 2) as i32;
            let hwpx_ellipse = Ellipse {
                size,
                position,
                outside_margin: None,
                caption: caption.clone(),
                shape_comment: None,
                meta_tag: None,
                offset,
                original_size,
                current_size,
                flip,
                rotation_info,
                rendering_info,
                line_shape,
                fill_brush: fill_brush.clone(),
                draw_text: draw_text.clone(),
                shadow: None,
                center: Point {
                    x: Some(half_width),
                    y: Some(half_height),
                },
                axis1: Point {
                    x: Some(half_width),
                    y: Some(0),
                },
                axis2: Point {
                    x: Some(0),
                    y: Some(half_height),
                },
                start1: Point {
                    x: Some(half_width),
                    y: Some(0),
                },
                end1: Point {
                    x: Some(half_width),
                    y: Some(0),
                },
                start2: Point {
                    x: Some(half_width),
                    y: Some(0),
                },
                end2: Point {
                    x: Some(half_width),
                    y: Some(0),
                },
                id: shape.common.id,
                z_order: shape.common.z_order,
                numbering_type: Default::default(),
                text_wrap,
                text_flow,
                lock: false,
                href: None,
                group_level: 0,
                instance_id: None,
                interval_dirty: false,
                has_arc_properties: ellipse.arc_type != IrArcType::Full,
                arc_type: convert_arc_type_to_hwpx(&ellipse.arc_type),
            };
            Ok(Some(HwpxRunContent::Ellipse(Box::new(hwpx_ellipse))))
        }
        IrShapeType::Arc(arc) => {
            let half_width = (width / 2) as i32;
            let half_height = (height / 2) as i32;
            let hwpx_arc = Arc {
                size,
                position,
                outside_margin: None,
                caption: caption.clone(),
                shape_comment: None,
                meta_tag: None,
                offset,
                original_size,
                current_size,
                flip,
                rotation_info,
                rendering_info,
                line_shape,
                fill_brush: fill_brush.clone(),
                draw_text: draw_text.clone(),
                shadow: None,
                center: Point {
                    x: Some(half_width),
                    y: Some(half_height),
                },
                axis1: Point {
                    x: Some(half_width),
                    y: Some(0),
                },
                axis2: Point {
                    x: Some(0),
                    y: Some(half_height),
                },
                id: shape.common.id,
                z_order: shape.common.z_order,
                numbering_type: Default::default(),
                text_wrap,
                text_flow,
                lock: false,
                href: None,
                group_level: 0,
                instance_id: None,
                arc_type: convert_arc_type_to_hwpx(&arc.arc_type),
            };
            Ok(Some(HwpxRunContent::Arc(Box::new(hwpx_arc))))
        }
        IrShapeType::Polygon(poly) => {
            let points: Vec<Point> = poly
                .points
                .iter()
                .map(|p| Point {
                    x: Some(p.x.value()),
                    y: Some(p.y.value()),
                })
                .collect();
            let hwpx_polygon = Polygon {
                size,
                position,
                outside_margin: None,
                caption: caption.clone(),
                shape_comment: None,
                meta_tag: None,
                offset,
                original_size,
                current_size,
                flip,
                rotation_info,
                rendering_info,
                line_shape,
                fill_brush: fill_brush.clone(),
                draw_text: draw_text.clone(),
                shadow: None,
                points,
                id: shape.common.id,
                z_order: shape.common.z_order,
                numbering_type: Default::default(),
                text_wrap,
                text_flow,
                lock: false,
                href: None,
                group_level: 0,
                instance_id: None,
            };
            Ok(Some(HwpxRunContent::Polygon(Box::new(hwpx_polygon))))
        }
        IrShapeType::Curve(curve) => {
            use crate::paragraph::shape_common::{CurveSegment, CurveSegmentType};

            // 곡선 세그먼트 변환 - HWPX의 CurveSegment는 (x1, y1, x2, y2) 형식
            let segments: Vec<CurveSegment> = curve
                .points
                .windows(2)
                .map(|pair| CurveSegment {
                    segment_type: CurveSegmentType::Curve, // 기본값으로 곡선 사용
                    x1: Some(pair[0].point.x.value()),
                    y1: Some(pair[0].point.y.value()),
                    x2: Some(pair[1].point.x.value()),
                    y2: Some(pair[1].point.y.value()),
                })
                .collect();

            let hwpx_curve = Curve {
                size,
                position,
                outside_margin: None,
                caption,
                shape_comment: None,
                meta_tag: None,
                offset,
                original_size,
                current_size,
                flip,
                rotation_info,
                rendering_info,
                line_shape,
                fill_brush,
                draw_text,
                shadow: None,
                segments,
                id: shape.common.id,
                z_order: shape.common.z_order,
                numbering_type: Default::default(),
                text_wrap,
                text_flow,
                lock: false,
                href: None,
                group_level: 0,
                instance_id: None,
            };
            Ok(Some(HwpxRunContent::Curve(Box::new(hwpx_curve))))
        }
        IrShapeType::Connector(connector) => {
            let hwpx_connect_line = convert_connector_to_hwpx(shape, connector)?;
            Ok(Some(HwpxRunContent::ConnectLine(Box::new(
                hwpx_connect_line,
            ))))
        }
        IrShapeType::Group(child_shapes) => {
            let hwpx_container = convert_group_to_container(shape, child_shapes)?;
            Ok(Some(HwpxRunContent::Container(Box::new(hwpx_container))))
        }
    }
}

/// IR Connector → HWPX ConnectLine 변환
fn convert_connector_to_hwpx(
    shape: &IrShape,
    connector: &ir::shape::ConnectorShape,
) -> Result<crate::paragraph::drawing::ConnectLine, ConversionError> {
    use crate::core::types::Matrix;
    use crate::paragraph::drawing::ConnectLine;
    use crate::paragraph::shape_common::{
        ConnectLineStyle, ConnectPoint, CurrentSize, Flip, OriginalSize, RenderingInfo,
        RotationInfo, ShapeComponentOffset,
    };

    // 연결선 타입 결정 (화살표 유무에 따라)
    let has_start_arrow = !matches!(connector.start_arrow.arrow_type, primitive::ArrowType::None);
    let has_end_arrow = !matches!(connector.end_arrow.arrow_type, primitive::ArrowType::None);

    let line_type = match connector.connector_type {
        ir::shape::ConnectorType::Straight => match (has_start_arrow, has_end_arrow) {
            (false, false) => ConnectLineStyle::StraightNoArrow,
            (true, true) => ConnectLineStyle::StraightBoth,
            _ => ConnectLineStyle::StraightOneWay,
        },
        ir::shape::ConnectorType::Elbow => match (has_start_arrow, has_end_arrow) {
            (false, false) => ConnectLineStyle::StrokeNoArrow,
            (true, true) => ConnectLineStyle::StrokeBoth,
            _ => ConnectLineStyle::StrokeOneWay,
        },
        ir::shape::ConnectorType::Curved => match (has_start_arrow, has_end_arrow) {
            (false, false) => ConnectLineStyle::ArcNoArrow,
            (true, true) => ConnectLineStyle::ArcBoth,
            _ => ConnectLineStyle::ArcOneWay,
        },
    };

    // 시작점/끝점
    let start_point = ConnectPoint {
        x: Some(connector.start.point.x.value()),
        y: Some(connector.start.point.y.value()),
        subject_id_reference: connector
            .start
            .subject_id_ref
            .map(crate::core::types::SubjectIdRef),
        subject_index: connector.start.subject_index,
    };
    let end_point = ConnectPoint {
        x: Some(connector.end.point.x.value()),
        y: Some(connector.end.point.y.value()),
        subject_id_reference: connector
            .end
            .subject_id_ref
            .map(crate::core::types::SubjectIdRef),
        subject_index: connector.end.subject_index,
    };

    // 선 스타일 (화살표 포함)
    let line_shape = convert_line_style_with_arrows_to_hwpx(
        &shape.line,
        &connector.start_arrow,
        &connector.end_arrow,
    );

    // ObjectCommon → HWPX 속성 변환
    let size = convert_object_common_to_size(&shape.common);
    let position = convert_object_common_to_position(&shape.common);
    let text_wrap = Some(convert_text_wrap_to_mode(&shape.common.text_wrap));
    let text_flow = convert_text_wrap_to_flow(&shape.common.text_wrap);

    // 캡션 변환
    let caption = if let Some(ref ir_caption) = shape.common.caption {
        Some(convert_ir_caption_to_hwpx(ir_caption)?)
    } else {
        None
    };

    Ok(ConnectLine {
        size,
        position,
        outside_margin: None,
        caption,
        shape_comment: None,
        meta_tag: None,
        offset: ShapeComponentOffset {
            x: shape.common.position.x.value().max(0) as u32,
            y: shape.common.position.y.value().max(0) as u32,
        },
        original_size: OriginalSize {
            width: Some(shape.common.size.width.value() as u32),
            height: Some(shape.common.size.height.value() as u32),
        },
        current_size: CurrentSize {
            width: Some(shape.common.size.width.value() as u32),
            height: Some(shape.common.size.height.value() as u32),
        },
        flip: Flip::default(),
        rotation_info: RotationInfo {
            angle: (shape.rotation * 10.0) as i32, // HWPX는 0.1도 단위
            center_x: None,
            center_y: None,
            rotate_image: None,
        },
        rendering_info: RenderingInfo {
            transform_matrix: Matrix::default(),
            matrix_pairs: vec![],
        },
        line_shape,
        fill_brush: convert_fill_to_hwpx(&shape.fill),
        draw_text: shape.text.as_ref().map(|text| {
            use crate::paragraph::drawing::{DrawText, TextMargin};
            use crate::paragraph::enums::{
                ParagraphLineWrap, ParagraphVerticalAlignment, TextDirection,
            };
            use crate::paragraph::para_list::ParagraphList;

            let paragraphs = text
                .paragraphs
                .iter()
                .enumerate()
                .filter_map(|(i, p)| convert_paragraph(p, i as u32).ok())
                .collect::<Vec<_>>();

            DrawText {
                paragraph_list: ParagraphList {
                    paragraphs,
                    id: "0".to_string(),
                    text_direction: TextDirection::Horizontal,
                    line_wrap: ParagraphLineWrap::Break,
                    vertical_alignment: ParagraphVerticalAlignment::Top,
                    link_list_id_reference: None,
                    link_list_next_id_reference: None,
                    text_width: None,
                    text_height: None,
                    has_text_reference: false,
                    has_number_reference: false,
                },
                text_margin: Some(TextMargin {
                    left: text.padding.left.value().max(0) as u32,
                    right: text.padding.right.value().max(0) as u32,
                    top: text.padding.top.value().max(0) as u32,
                    bottom: text.padding.bottom.value().max(0) as u32,
                }),
                last_width: None,
                name: None,
                editable: text.editable,
            }
        }),
        shadow: None,
        start_point,
        end_point,
        control_points: if connector.control_points.is_empty() {
            None
        } else {
            use crate::paragraph::drawing::ConnectControlPoints;
            use crate::paragraph::shape_common::ConnectControlPoint;

            Some(ConnectControlPoints {
                points: connector
                    .control_points
                    .iter()
                    .map(|cp| {
                        // CurvePointType → u32 변환
                        let point_type = match cp.point_type {
                            ir::shape::CurvePointType::Normal => 0,
                            ir::shape::CurvePointType::Control1 => 1,
                            ir::shape::CurvePointType::Control2 => 2,
                        };

                        ConnectControlPoint {
                            x: Some(cp.point.x.value()),
                            y: Some(cp.point.y.value()),
                            point_type: Some(point_type),
                        }
                    })
                    .collect(),
            })
        },
        id: shape.common.id,
        z_order: shape.common.z_order,
        numbering_type: Default::default(),
        text_wrap,
        text_flow,
        lock: false,
        href: None,
        group_level: 0,
        instance_id: None,
        line_type: Some(line_type),
    })
}

/// IR 선 스타일 (화살표 포함) → HWPX LineShape 변환
fn convert_line_style_with_arrows_to_hwpx(
    line: &ir::shape::LineStyle,
    start_arrow: &ir::shape::Arrow,
    end_arrow: &ir::shape::Arrow,
) -> crate::paragraph::line_shape::LineShape {
    use crate::paragraph::line_shape::{LineEndCapStyle, LineShape, OutlineStyle};

    LineShape {
        color: Some(RgbColor {
            r: line.color.red,
            g: line.color.green,
            b: line.color.blue,
            a: 255,
        }),
        width: Some(line.width.value() as u32),
        style: convert_line_type_to_hwpx2(&line.line_type),
        end_cap: match line.cap {
            primitive::LineCap::Flat => LineEndCapStyle::Flat,
            primitive::LineCap::Round => LineEndCapStyle::Round,
            primitive::LineCap::Square => LineEndCapStyle::Flat,
        },
        head_style: convert_arrow_type_to_hwpx(&start_arrow.arrow_type, start_arrow.filled),
        tail_style: convert_arrow_type_to_hwpx(&end_arrow.arrow_type, end_arrow.filled),
        head_fill: start_arrow.filled,
        tail_fill: end_arrow.filled,
        head_size: convert_arrow_size_to_hwpx(&start_arrow.size),
        tail_size: convert_arrow_size_to_hwpx(&end_arrow.size),
        outline_style: match line.outline_style {
            primitive::LineOutlineStyle::Normal => OutlineStyle::Normal,
            primitive::LineOutlineStyle::Outer => OutlineStyle::Outer,
            primitive::LineOutlineStyle::Inner => OutlineStyle::Inner,
        },
        alpha: line.alpha,
    }
}

/// IR ArrowSize → HWPX ArrowSize 변환
fn convert_arrow_size_to_hwpx(size: &primitive::ArrowSize) -> crate::core::enums::ArrowSize {
    use crate::core::enums::ArrowSize as HwpxArrowSize;
    use primitive::ArrowSize;

    // IR은 단일 크기, HWPX는 WIDTH_HEIGHT 형식
    // 단일 크기를 같은 값의 조합으로 변환 (예: Medium → MediumMedium)
    match size {
        ArrowSize::Small => HwpxArrowSize::SmallSmall,
        ArrowSize::Medium => HwpxArrowSize::MediumMedium,
        ArrowSize::Large => HwpxArrowSize::LargeLarge,
    }
}

/// IR 화살표 타입 → HWPX ArrowStyle 변환
fn convert_arrow_type_to_hwpx(
    arrow_type: &primitive::ArrowType,
    filled: bool,
) -> crate::core::enums::ArrowStyle {
    use crate::core::enums::ArrowStyle;

    match arrow_type {
        primitive::ArrowType::None => ArrowStyle::Normal, // Normal means no arrow in HWPX
        primitive::ArrowType::Arrow => ArrowStyle::Arrow,
        primitive::ArrowType::ArrowOpen => ArrowStyle::Arrow, // Open arrow uses same style
        primitive::ArrowType::Stealth => ArrowStyle::Spear,
        primitive::ArrowType::Diamond => {
            if filled {
                ArrowStyle::FilledDiamond
            } else {
                ArrowStyle::EmptyDiamond
            }
        }
        primitive::ArrowType::Circle => {
            if filled {
                ArrowStyle::FilledCircle
            } else {
                ArrowStyle::EmptyCircle
            }
        }
        primitive::ArrowType::Square => {
            if filled {
                ArrowStyle::FilledBox
            } else {
                ArrowStyle::EmptyBox
            }
        }
    }
}

/// IR 선 스타일 → HWPX LineShape 변환
fn convert_line_style_to_hwpx(
    line: &ir::shape::LineStyle,
) -> crate::paragraph::line_shape::LineShape {
    use crate::core::enums::ArrowStyle;
    use crate::paragraph::line_shape::{LineEndCapStyle, LineShape, OutlineStyle};

    LineShape {
        color: Some(RgbColor {
            r: line.color.red,
            g: line.color.green,
            b: line.color.blue,
            a: 255,
        }),
        width: Some(line.width.value() as u32),
        style: convert_line_type_to_hwpx2(&line.line_type),
        end_cap: match line.cap {
            primitive::LineCap::Flat => LineEndCapStyle::Flat,
            primitive::LineCap::Round => LineEndCapStyle::Round,
            primitive::LineCap::Square => LineEndCapStyle::Flat,
        },
        head_style: ArrowStyle::Normal,
        tail_style: ArrowStyle::Normal,
        head_fill: false,
        tail_fill: false,
        head_size: Default::default(),
        tail_size: Default::default(),
        outline_style: match line.outline_style {
            primitive::LineOutlineStyle::Normal => OutlineStyle::Normal,
            primitive::LineOutlineStyle::Outer => OutlineStyle::Outer,
            primitive::LineOutlineStyle::Inner => OutlineStyle::Inner,
        },
        alpha: line.alpha,
    }
}

/// IR 선 종류 → HWPX LineStyleType2 변환 (도형 선용)
fn convert_line_type_to_hwpx2(line_type: &primitive::LineType) -> LineStyleType2 {
    match line_type {
        primitive::LineType::None => LineStyleType2::None,
        primitive::LineType::Solid => LineStyleType2::Solid,
        primitive::LineType::Dash => LineStyleType2::Dash,
        primitive::LineType::Dot => LineStyleType2::Dot,
        primitive::LineType::DashDot => LineStyleType2::DashDot,
        primitive::LineType::DashDotDot => LineStyleType2::DashDotDot,
        primitive::LineType::LongDash => LineStyleType2::LongDash,
        primitive::LineType::Double => LineStyleType2::DoubleSlim,
        primitive::LineType::Triple => LineStyleType2::SlimThickSlim,
        primitive::LineType::Wave => LineStyleType2::Solid, // LineStyleType2 doesn't have Wave
        primitive::LineType::DoubleWave => LineStyleType2::Solid,
        primitive::LineType::ThickThinLarge => LineStyleType2::ThickSlim,
        primitive::LineType::ThinThickLarge => LineStyleType2::SlimThick,
        primitive::LineType::Circle => LineStyleType2::Circle,
    }
}

/// IR 호 종류 → HWPX ArcStyle 변환
fn convert_arc_type_to_hwpx(arc_type: &IrArcType) -> crate::paragraph::shape_common::ArcStyle {
    use crate::paragraph::shape_common::ArcStyle;
    match arc_type {
        IrArcType::Full => ArcStyle::Pie,
        IrArcType::Arc => ArcStyle::Normal,
        IrArcType::Pie => ArcStyle::Pie,
        IrArcType::Chord => ArcStyle::Chord,
    }
}

/// IR Video → HWPX Video 변환
fn convert_video_to_hwpx(video: &IrVideo) -> Result<crate::paragraph::Video, ConversionError> {
    use crate::core::types::{FileIdRef, ImageIdRef, Matrix};
    use crate::paragraph::shape_common::{
        CurrentSize, Flip, OriginalSize, RenderingInfo, RotationInfo, ShapeComponentOffset,
        ShapeNumberingType,
    };
    use crate::paragraph::video_chart::VideoType;

    // 비디오 종류 변환
    let video_type = match video.video_type {
        IrVideoType::Embedded => VideoType::Local,
        IrVideoType::Linked | IrVideoType::YouTube => VideoType::Web,
    };

    // 비디오 ID (로컬인 경우)
    let file_id_ref = video
        .video_id
        .as_ref()
        .filter(|id| !id.value().is_empty() && id.value() != "0")
        .map(|id| FileIdRef(id.value().to_string()));

    // poster_binary_id 우선, 없으면 preview_image_id 사용
    let image_id_ref = video
        .poster_binary_id
        .as_ref()
        .or(video.preview_image_id.as_ref())
        .filter(|id| !id.value().is_empty() && id.value() != "0")
        .map(|id| ImageIdRef(id.value().to_string()));

    // ObjectCommon → HWPX 속성 변환
    let size = convert_object_common_to_size(&video.common);
    let position = convert_object_common_to_position(&video.common);
    let text_wrap = Some(convert_text_wrap_to_mode(&video.common.text_wrap));
    let text_flow = convert_text_wrap_to_flow(&video.common.text_wrap);

    // 캡션 변환
    let caption = if let Some(ref ir_caption) = video.common.caption {
        Some(convert_ir_caption_to_hwpx(ir_caption)?)
    } else {
        None
    };

    Ok(crate::paragraph::Video {
        // AbstractShapeObjectType 요소들
        size,
        position,
        outside_margin: None,
        caption,
        shape_comment: None,
        meta_tag: None,
        // AbstractShapeComponentType 요소들
        offset: ShapeComponentOffset {
            x: video.common.position.x.value().max(0) as u32,
            y: video.common.position.y.value().max(0) as u32,
        },
        original_size: OriginalSize {
            // HWP width/height 필드 우선 사용, 없으면 common.size 사용
            width: Some(
                video
                    .width
                    .map(|w| w.value() as u32)
                    .unwrap_or(video.common.size.width.value() as u32),
            ),
            height: Some(
                video
                    .height
                    .map(|h| h.value() as u32)
                    .unwrap_or(video.common.size.height.value() as u32),
            ),
        },
        current_size: CurrentSize {
            width: Some(video.common.size.width.value() as u32),
            height: Some(video.common.size.height.value() as u32),
        },
        flip: Flip::default(),
        rotation_info: RotationInfo::default(),
        rendering_info: RenderingInfo {
            transform_matrix: Matrix::default(),
            matrix_pairs: Vec::new(),
        },
        // 속성들
        id: video.common.id,
        z_order: video.common.z_order,
        numbering_type: ShapeNumberingType::default(),
        text_wrap,
        text_flow,
        lock: false,
        href: None,
        group_level: 0,
        instance_id: None,
        // Video 전용 속성들
        video_type,
        file_id_ref,
        image_id_ref,
        tag: None,
    })
}

/// IR OleObject → HWPX Ole 변환
fn convert_ole_to_hwpx(ole: &IrOleObject) -> Result<crate::paragraph::Ole, ConversionError> {
    use crate::core::types::{BinaryItemIdRef, Matrix, Point};
    use crate::paragraph::line_shape::LineShape;
    use crate::paragraph::shape_common::{
        CurrentSize, Flip, OriginalSize, RenderingInfo, RotationInfo, ShapeComponentOffset,
        ShapeNumberingType,
    };

    let binary_id = ole.binary_id.value();
    let binary_item_id_ref = if !binary_id.is_empty() && binary_id != "0" {
        Some(BinaryItemIdRef(binary_id.to_string()))
    } else {
        None
    };

    // ObjectCommon → HWPX 속성 변환
    let size = convert_object_common_to_size(&ole.common);
    let position = convert_object_common_to_position(&ole.common);
    let text_wrap = Some(convert_text_wrap_to_mode(&ole.common.text_wrap));
    let text_flow = convert_text_wrap_to_flow(&ole.common.text_wrap);

    // 캡션 변환
    let caption = if let Some(ref ir_caption) = ole.common.caption {
        Some(convert_ir_caption_to_hwpx(ir_caption)?)
    } else {
        None
    };

    Ok(crate::paragraph::Ole {
        // AbstractShapeObjectType 요소들
        size,
        position,
        outside_margin: None,
        caption,
        shape_comment: None,
        meta_tag: None,
        // AbstractShapeComponentType 요소들
        offset: ShapeComponentOffset {
            x: ole.common.position.x.value().max(0) as u32,
            y: ole.common.position.y.value().max(0) as u32,
        },
        original_size: OriginalSize {
            width: Some(ole.common.size.width.value() as u32),
            height: Some(ole.common.size.height.value() as u32),
        },
        current_size: CurrentSize {
            width: Some(ole.common.size.width.value() as u32),
            height: Some(ole.common.size.height.value() as u32),
        },
        flip: Flip::default(),
        rotation_info: RotationInfo::default(),
        rendering_info: RenderingInfo {
            transform_matrix: Matrix::default(),
            matrix_pairs: Vec::new(),
        },
        // OLEType 전용 요소들
        extent: Point::default(),
        line_shape: LineShape::default(),
        // 속성들
        id: ole.common.id,
        z_order: ole.common.z_order,
        numbering_type: ShapeNumberingType::default(),
        text_wrap,
        text_flow,
        lock: false,
        href: None,
        group_level: 0,
        instance_id: None,
        // OLEType 전용 속성들
        object_type: None,
        binary_item_id_ref,
        has_moniker: false,
        draw_aspect: None,
        equation_baseline: 85,
    })
}

/// IR Chart → HWPX Chart 변환
fn convert_chart_to_hwpx(chart: &IrChart) -> Result<crate::paragraph::Chart, ConversionError> {
    use crate::core::types::ChartIdRef;
    use crate::paragraph::shape_common::ShapeNumberingType;

    let chart_id_ref = if !chart.chart_id.is_empty() {
        Some(ChartIdRef(chart.chart_id.clone()))
    } else {
        None
    };

    // ObjectCommon → HWPX 속성 변환
    let size = convert_object_common_to_size(&chart.common);
    let position = convert_object_common_to_position(&chart.common);
    let text_wrap = Some(convert_text_wrap_to_mode(&chart.common.text_wrap));
    let text_flow = convert_text_wrap_to_flow(&chart.common.text_wrap);

    // 캡션 변환
    let caption = if let Some(ref ir_caption) = chart.common.caption {
        Some(convert_ir_caption_to_hwpx(ir_caption)?)
    } else {
        None
    };

    Ok(crate::paragraph::Chart {
        // AbstractShapeObjectType 요소들
        size,
        position,
        outside_margin: None,
        caption,
        shape_comment: None,
        meta_tag: None,
        // 속성들
        id: chart.common.id,
        z_order: chart.common.z_order,
        numbering_type: ShapeNumberingType::default(),
        text_wrap,
        text_flow,
        lock: false,
        // Chart 전용 속성들
        version: None,
        chart_id_ref,
    })
}

/// IR FormCharProperty → HWPX FormCharacterProperty 변환
fn convert_form_char_property_to_hwpx(
    prop: &ir::control::FormCharProperty,
) -> crate::paragraph::form_control::FormCharacterProperty {
    use crate::core::types::CharShapeIdRef;
    use crate::paragraph::form_control::FormCharacterProperty;

    FormCharacterProperty {
        char_property_id_ref: prop.char_shape_id.map(CharShapeIdRef),
        follow_context: prop.follow_context,
        auto_size: prop.auto_size,
        word_wrap: prop.word_wrap,
    }
}

/// IR FormListItem → HWPX ListItem 변환
fn convert_form_list_items_to_hwpx(
    items: &[ir::control::FormListItem],
) -> Vec<crate::paragraph::form_control::ListItem> {
    use crate::paragraph::form_control::ListItem;

    items
        .iter()
        .map(|item| ListItem {
            display_text: item.display_text.clone(),
            value: item.value.clone(),
        })
        .collect()
}

/// IR FormObject → HWPX RunContent 변환
fn convert_form_object_to_hwpx(
    form: &IrFormObject,
) -> Result<Option<HwpxRunContent>, ConversionError> {
    use crate::core::types::BorderTypeIdRef;
    use crate::paragraph::form_control::{
        Button, ButtonValue, ComboBox, Edit, ListBox, ScrollBar, ScrollBarType as HwpxScrollBarType,
    };
    use crate::paragraph::shape_common::ShapeNumberingType;

    // FormCharProperty 변환
    let form_char_property = convert_form_char_property_to_hwpx(&form.char_property);

    // ListItems 변환
    let list_items = convert_form_list_items_to_hwpx(&form.items);

    // ObjectCommon → HWPX 속성 변환
    let size = convert_object_common_to_size(&form.common);
    let position = convert_object_common_to_position(&form.common);
    let text_wrap = Some(convert_text_wrap_to_mode(&form.common.text_wrap));
    let text_flow = convert_text_wrap_to_flow(&form.common.text_wrap);
    let id = form.common.id;
    let z_order = form.common.z_order;

    // 캡션 변환
    let caption = if let Some(ref ir_caption) = form.common.caption {
        Some(convert_ir_caption_to_hwpx(ir_caption)?)
    } else {
        None
    };

    // 공통 속성 변환 (모든 양식 객체에서 사용)
    let fore_color = form.fore_color.as_ref().map(|c| RgbColor {
        r: c.red,
        g: c.green,
        b: c.blue,
        a: c.alpha,
    });
    let back_color = form.back_color.as_ref().map(|c| RgbColor {
        r: c.red,
        g: c.green,
        b: c.blue,
        a: c.alpha,
    });
    let group_name = form.group_name.clone();
    let tab_stop = form.tab_stop;
    let enabled = form.enabled;
    let border_type_id_ref = form.border_type_id_ref.map(BorderTypeIdRef);
    let draw_frame = form.draw_frame;
    let printable = form.printable;

    // ButtonValue 변환 (Button, RadioButton, CheckButton용)
    // IR에서는 문자열로 저장되므로 파싱 필요
    let button_value = form.value.as_ref().and_then(|v| match v.as_str() {
        "Checked" => Some(ButtonValue::Checked),
        "Unchecked" => Some(ButtonValue::Unchecked),
        "Indeterminate" => Some(ButtonValue::Indeterminate),
        _ => None,
    });

    // 양식 타입에 따라 적절한 HWPX 요소 생성
    match form.form_type {
        IrFormObjectType::Button => {
            let btn = Button {
                // AbstractShapeObjectType 요소들
                size,
                position,
                outside_margin: None,
                caption_element: None,
                shape_comment: None,
                meta_tag: None,
                // AbstractFormObjectType 요소
                form_char_property,
                // 속성들
                id,
                z_order,
                numbering_type: ShapeNumberingType::default(),
                text_wrap,
                text_flow,
                lock: false,
                // AbstractFormObjectType 속성들 (IR에서 가져옴)
                name: form.name.clone(),
                fore_color,
                back_color,
                group_name: group_name.clone(),
                tab_stop,
                editable: true,
                tab_order: None,
                enabled,
                border_type_id_ref,
                draw_frame,
                printable,
                // AbstractButtonObjectType 속성들
                caption_text: None,
                value: button_value,
                radio_group_name: None,
                tri_state: false,
                back_style: None,
            };
            Ok(Some(HwpxRunContent::Button(Box::new(btn))))
        }
        IrFormObjectType::RadioButton => {
            let btn = Button {
                size,
                position,
                outside_margin: None,
                caption_element: None,
                shape_comment: None,
                meta_tag: None,
                form_char_property,
                id,
                z_order,
                numbering_type: ShapeNumberingType::default(),
                text_wrap,
                text_flow,
                lock: false,
                name: form.name.clone(),
                fore_color,
                back_color,
                group_name: group_name.clone(),
                tab_stop,
                editable: true,
                tab_order: None,
                enabled,
                border_type_id_ref,
                draw_frame,
                printable,
                caption_text: None,
                value: button_value,
                radio_group_name: form.group_name.clone(), // RadioButton은 group_name을 radio_group_name으로도 사용
                tri_state: false,
                back_style: None,
            };
            Ok(Some(HwpxRunContent::RadioButton(Box::new(btn))))
        }
        IrFormObjectType::CheckBox => {
            let btn = Button {
                size,
                position,
                outside_margin: None,
                caption_element: None,
                shape_comment: None,
                meta_tag: None,
                form_char_property,
                id,
                z_order,
                numbering_type: ShapeNumberingType::default(),
                text_wrap,
                text_flow,
                lock: false,
                name: form.name.clone(),
                fore_color,
                back_color,
                group_name: group_name.clone(),
                tab_stop,
                editable: true,
                tab_order: None,
                enabled,
                border_type_id_ref,
                draw_frame,
                printable,
                caption_text: None,
                value: button_value,
                radio_group_name: None,
                tri_state: false,
                back_style: None,
            };
            Ok(Some(HwpxRunContent::CheckButton(Box::new(btn))))
        }
        IrFormObjectType::ComboBox => {
            let combo = ComboBox {
                size,
                position,
                outside_margin: None,
                caption: caption.clone(),
                shape_comment: None,
                meta_tag: None,
                form_char_property,
                list_items: list_items.clone(),
                id,
                z_order,
                numbering_type: ShapeNumberingType::default(),
                text_wrap,
                text_flow,
                lock: false,
                name: form.name.clone(),
                fore_color,
                back_color,
                group_name: group_name.clone(),
                tab_stop,
                form_editable: true,
                tab_order: None,
                enabled,
                border_type_id_ref,
                draw_frame,
                printable,
                list_box_rows: None,
                list_box_width: None,
                edit_enable: false,
                selected_value: None,
            };
            Ok(Some(HwpxRunContent::ComboBox(Box::new(combo))))
        }
        IrFormObjectType::ListBox => {
            let list = ListBox {
                size,
                position,
                outside_margin: None,
                caption: caption.clone(),
                shape_comment: None,
                meta_tag: None,
                form_char_property,
                list_items: list_items.clone(),
                id,
                z_order,
                numbering_type: ShapeNumberingType::default(),
                text_wrap,
                text_flow,
                lock: false,
                name: form.name.clone(),
                fore_color,
                back_color,
                group_name: group_name.clone(),
                tab_stop,
                editable: true,
                tab_order: None,
                enabled,
                border_type_id_ref,
                draw_frame,
                printable,
                item_height: None,
                top_index: None,
                selected_value: None,
            };
            Ok(Some(HwpxRunContent::ListBox(Box::new(list))))
        }
        IrFormObjectType::Edit => {
            let edit = Edit {
                size,
                position,
                outside_margin: None,
                caption: caption.clone(),
                shape_comment: None,
                meta_tag: None,
                form_char_property,
                text: form.value.clone().unwrap_or_default(),
                id,
                z_order,
                numbering_type: ShapeNumberingType::default(),
                text_wrap,
                text_flow,
                lock: false,
                name: form.name.clone(),
                fore_color,
                back_color,
                group_name: group_name.clone(),
                tab_stop,
                form_editable: true,
                tab_order: None,
                enabled,
                border_type_id_ref,
                draw_frame,
                printable,
                multi_line: false,
                password_char: "*".to_string(),
                max_length: None,
                scroll_bars: Default::default(),
                tab_key_behavior: None,
                number_only: false,
                read_only: false,
                align_text: Default::default(),
            };
            Ok(Some(HwpxRunContent::Edit(Box::new(edit))))
        }
        IrFormObjectType::ScrollBar => {
            // IR ScrollBarType → HWPX ScrollBarType 변환
            let bar_type = form.bar_type.map(|bt| match bt {
                ir::control::ScrollBarType::Horizontal => HwpxScrollBarType::Horizontal,
                ir::control::ScrollBarType::Vertical => HwpxScrollBarType::Vertical,
            });

            let scroll = ScrollBar {
                size,
                position,
                outside_margin: None,
                caption,
                shape_comment: None,
                meta_tag: None,
                form_char_property,
                id,
                z_order,
                numbering_type: ShapeNumberingType::default(),
                text_wrap,
                text_flow,
                lock: false,
                name: form.name.clone(),
                fore_color,
                back_color,
                group_name,
                tab_stop,
                editable: true,
                tab_order: None,
                enabled,
                border_type_id_ref,
                draw_frame,
                printable,
                delay: None,
                large_change: None,
                small_change: None,
                min: None,
                max: None,
                page: None,
                value: form.value.as_ref().and_then(|v| v.parse().ok()),
                bar_type,
            };
            Ok(Some(HwpxRunContent::ScrollBar(Box::new(scroll))))
        }
    }
}

/// IR TextArt → HWPX TextArt 변환
fn convert_text_art_to_hwpx(
    text_art: &ir::control::TextArt,
) -> Result<crate::paragraph::TextArt, ConversionError> {
    use crate::core::types::{Matrix, Point as HwpxPoint};
    use crate::paragraph::shadow::ShapeShadow;
    use crate::paragraph::shape_common::{
        CurrentSize, Flip, OriginalSize, RenderingInfo, RotationInfo, ShapeComponentOffset,
        ShapeNumberingType,
    };
    use crate::paragraph::text_art::{
        TextArt as HwpxTextArt, TextArtAlignment as HwpxTextArtAlignment, TextArtFontType,
        TextArtProperties, TextArtShape as HwpxTextArtShape,
    };
    use ir::control::{TextArtAlignment, TextArtFontStyle, TextArtShapeType};

    let offset = ShapeComponentOffset {
        x: text_art.common.position.x.value().max(0) as u32,
        y: text_art.common.position.y.value().max(0) as u32,
    };

    let original_size = OriginalSize {
        width: Some(text_art.common.size.width.value() as u32),
        height: Some(text_art.common.size.height.value() as u32),
    };

    let current_size = CurrentSize {
        width: Some(text_art.common.size.width.value() as u32),
        height: Some(text_art.common.size.height.value() as u32),
    };

    let flip = Flip {
        horizontal: false,
        vertical: false,
    };

    let rotation_info = RotationInfo {
        angle: 0,
        center_x: None,
        center_y: None,
        rotate_image: None,
    };

    let rendering_info = RenderingInfo {
        transform_matrix: Matrix::default(),
        matrix_pairs: Vec::new(),
    };

    // ObjectCommon → HWPX 속성 변환
    let size = convert_object_common_to_size(&text_art.common);
    let position = convert_object_common_to_position(&text_art.common);
    let text_wrap = Some(convert_text_wrap_to_mode(&text_art.common.text_wrap));
    let text_flow = convert_text_wrap_to_flow(&text_art.common.text_wrap);

    // 캡션 변환
    let caption = if let Some(ref ir_caption) = text_art.common.caption {
        Some(convert_ir_caption_to_hwpx(ir_caption)?)
    } else {
        None
    };

    // 선 스타일 변환
    let line_shape = convert_line_style_to_hwpx(&text_art.line);

    // 글맵시 모양 변환
    let text_shape = match text_art.shape {
        TextArtShapeType::Rectangle => Some(HwpxTextArtShape::Rectangle),
        TextArtShapeType::Circle => Some(HwpxTextArtShape::Circle),
        TextArtShapeType::ArchUp => Some(HwpxTextArtShape::ArchUp),
        TextArtShapeType::ArchDown => Some(HwpxTextArtShape::ArchDown),
        TextArtShapeType::Wave => Some(HwpxTextArtShape::Wave1),
        TextArtShapeType::Cylinder => Some(HwpxTextArtShape::Cylinder),
        TextArtShapeType::Inflate => Some(HwpxTextArtShape::Inflate),
        TextArtShapeType::Deflate => Some(HwpxTextArtShape::Deflate),
        TextArtShapeType::Other(_) => Some(HwpxTextArtShape::Rectangle),
    };

    // 정렬 변환
    let alignment = match text_art.alignment {
        TextArtAlignment::Left => HwpxTextArtAlignment::Left,
        TextArtAlignment::Center => HwpxTextArtAlignment::Center,
        TextArtAlignment::Right => HwpxTextArtAlignment::Right,
        TextArtAlignment::Full => HwpxTextArtAlignment::Full,
    };

    // 글꼴 스타일 변환
    let font_style = match text_art.font_style {
        TextArtFontStyle::Regular => "REGULAR".to_string(),
        TextArtFontStyle::Bold => "BOLD".to_string(),
        TextArtFontStyle::Italic => "ITALIC".to_string(),
        TextArtFontStyle::BoldItalic => "BOLD_ITALIC".to_string(),
    };

    // 그림자 속성
    let shadow = text_art
        .shadow
        .as_ref()
        .map(|s| ShapeShadow {
            shadow_type: crate::paragraph::shadow::ShadowEffectType::ParallelRightBottom,
            color: RgbColor {
                r: s.color.red,
                g: s.color.green,
                b: s.color.blue,
                a: 255,
            },
            offset_x: Some(s.offset_x.value()),
            offset_y: Some(s.offset_y.value()),
            alpha: Some(s.alpha as f32),
        })
        .unwrap_or_else(|| ShapeShadow {
            shadow_type: crate::paragraph::shadow::ShadowEffectType::None,
            color: RgbColor::black(),
            offset_x: None,
            offset_y: None,
            alpha: None,
        });

    let properties = TextArtProperties {
        shadow: shadow.clone(),
        font_name: text_art.font_name.clone(),
        font_style,
        font_type: TextArtFontType::Ttf,
        text_shape,
        line_spacing: text_art.line_spacing,
        char_spacing: text_art.char_spacing,
        alignment,
    };

    // 꼭짓점 기본값 생성
    let width = text_art.common.size.width.value();
    let height = text_art.common.size.height.value();
    let point0 = HwpxPoint {
        x: Some(0),
        y: Some(0),
    };
    let point1 = HwpxPoint {
        x: Some(width),
        y: Some(0),
    };
    let point2 = HwpxPoint {
        x: Some(width),
        y: Some(height),
    };
    let point3 = HwpxPoint {
        x: Some(0),
        y: Some(height),
    };

    Ok(HwpxTextArt {
        size,
        position,
        outside_margin: None,
        caption,
        shape_comment: None,
        meta_tag: None,
        offset,
        original_size,
        current_size,
        flip,
        rotation_info,
        rendering_info,
        line_shape,
        fill_brush: convert_fill_to_hwpx(&text_art.fill),
        draw_text: None, // TextArt는 텍스트를 별도 필드로 가짐
        shape_shadow: text_art.shadow.as_ref().map(|s| ShapeShadow {
            shadow_type: crate::paragraph::shadow::ShadowEffectType::ParallelRightBottom,
            color: RgbColor {
                r: s.color.red,
                g: s.color.green,
                b: s.color.blue,
                a: 255,
            },
            offset_x: Some(s.offset_x.value()),
            offset_y: Some(s.offset_y.value()),
            alpha: Some(s.alpha as f32),
        }),
        point0,
        point1,
        point2,
        point3,
        properties,
        id: text_art.common.id,
        z_order: text_art.common.z_order,
        numbering_type: ShapeNumberingType::default(),
        text_wrap,
        text_flow,
        lock: false,
        href: None,
        group_level: 0,
        instance_id: None,
        text: Some(text_art.text.clone()),
    })
}

/// IR Group → HWPX Container 변환
fn convert_group_to_container(
    shape: &IrShape,
    child_shapes: &[IrShape],
) -> Result<crate::paragraph::ole_equation::Container, ConversionError> {
    use crate::core::types::Matrix;
    use crate::paragraph::ole_equation::Container;
    use crate::paragraph::shape_common::{
        CurrentSize, Flip, OriginalSize, RenderingInfo, RotationInfo, ShapeComponentOffset,
    };

    // 자식 도형들을 재귀적으로 변환
    let mut children = Vec::new();
    for child_shape in child_shapes {
        if let Some(child) = convert_ir_shape_to_container_child(child_shape)? {
            children.push(child);
        }
    }

    // ObjectCommon → HWPX 속성 변환
    let size = convert_object_common_to_size(&shape.common);
    let position = convert_object_common_to_position(&shape.common);
    let text_wrap = Some(convert_text_wrap_to_mode(&shape.common.text_wrap));
    let text_flow = convert_text_wrap_to_flow(&shape.common.text_wrap);

    // 캡션 변환
    let caption = if let Some(ref ir_caption) = shape.common.caption {
        Some(convert_ir_caption_to_hwpx(ir_caption)?)
    } else {
        None
    };

    Ok(Container {
        size,
        position,
        outside_margin: None,
        caption,
        shape_comment: None,
        meta_tag: None,
        offset: ShapeComponentOffset {
            x: shape.common.position.x.value().max(0) as u32,
            y: shape.common.position.y.value().max(0) as u32,
        },
        original_size: OriginalSize {
            width: Some(shape.common.size.width.value() as u32),
            height: Some(shape.common.size.height.value() as u32),
        },
        current_size: CurrentSize {
            width: Some(shape.common.size.width.value() as u32),
            height: Some(shape.common.size.height.value() as u32),
        },
        flip: Flip::default(),
        rotation_info: RotationInfo {
            angle: (shape.rotation * 10.0) as i32, // HWPX는 0.1도 단위
            center_x: None,
            center_y: None,
            rotate_image: None,
        },
        rendering_info: RenderingInfo {
            transform_matrix: Matrix::default(),
            matrix_pairs: vec![],
        },
        children,
        id: shape.common.id,
        z_order: shape.common.z_order,
        numbering_type: Default::default(),
        text_wrap,
        text_flow,
        lock: false,
        href: None,
        group_level: 0,
        instance_id: None,
    })
}

/// IR FieldStart → HWPX FieldBegin 변환
fn convert_field_start_to_hwpx(field_start: &IrFieldStart) -> HwpxFieldBegin {
    let field_type = match field_start.field_type {
        IrFieldType::Date | IrFieldType::Time => HwpxFieldType::Date,
        IrFieldType::FileName | IrFieldType::FilePath => HwpxFieldType::Path,
        IrFieldType::PageNumber | IrFieldType::PageCount => HwpxFieldType::ClickHere,
        IrFieldType::Title | IrFieldType::Summary => HwpxFieldType::Summary,
        IrFieldType::Author => HwpxFieldType::UserInfo,
        IrFieldType::CrossReference => HwpxFieldType::CrossReference,
        IrFieldType::MailMerge => HwpxFieldType::MailMerge,
        IrFieldType::TableOfContents => HwpxFieldType::Formula,
        IrFieldType::Bookmark => HwpxFieldType::Bookmark,
        IrFieldType::Hyperlink => HwpxFieldType::Hyperlink,
        IrFieldType::ClickHere => HwpxFieldType::ClickHere,
        IrFieldType::UserInfo => HwpxFieldType::UserInfo,
        IrFieldType::Formula => HwpxFieldType::Formula,
        IrFieldType::Memo => HwpxFieldType::Memo,
        IrFieldType::PrivateInfo => HwpxFieldType::PrivateInfo,
        IrFieldType::MetaTag => HwpxFieldType::MetaTag,
        IrFieldType::Unknown => HwpxFieldType::ClickHere,
    };

    // IR FieldParameters → HWPX ParameterList 변환
    let parameters = field_start
        .parameters
        .as_ref()
        .and_then(convert_ir_parameters_to_hwpx);

    // IR sub_paragraphs → HWPX ParagraphList 변환
    let sub_list = field_start.sub_paragraphs.as_ref().and_then(|sub_paras| {
        let mut hwpx_paragraphs = Vec::new();
        for (idx, ir_para) in sub_paras.iter().enumerate() {
            if let Ok(hwpx_para) = convert_paragraph(ir_para, idx as u32) {
                hwpx_paragraphs.push(hwpx_para);
            }
        }
        if hwpx_paragraphs.is_empty() {
            None
        } else {
            // ParagraphList needs proper initialization
            // For now we'll skip sub_list support as it requires more context
            None
        }
    });

    HwpxFieldBegin {
        parameters,
        sub_list,
        meta_tag: None,
        id: field_start.id,
        field_type,
        name: field_start.instruction.clone(),
        editable: field_start.editable,
        dirty: field_start.dirty,
        z_order: field_start.z_order,
        field_id: field_start.field_id,
    }
}

/// IR FieldParameters → HWPX ParameterList 변환
fn convert_ir_parameters_to_hwpx(
    params: &IrFieldParameters,
) -> Option<crate::paragraph::ParameterList> {
    let mut hwpx_items = Vec::new();

    for item in &params.items {
        if let Some(hwpx_param) = convert_ir_parameter_to_hwpx(item) {
            hwpx_items.push(hwpx_param);
        }
    }

    if hwpx_items.is_empty() {
        return None;
    }

    let count = hwpx_items.len() as u32;
    Some(crate::paragraph::ParameterList {
        items: hwpx_items,
        count,
        name: params.name.clone(),
    })
}

/// IR FieldParameter → HWPX ParameterItem 변환
fn convert_ir_parameter_to_hwpx(
    param: &IrFieldParameter,
) -> Option<crate::paragraph::ParameterItem> {
    use crate::paragraph::{
        BooleanParameter, FloatParameter, IntegerParameter, ParameterItem, StringParameter,
    };

    match param {
        IrFieldParameter::Boolean { name, value } => {
            Some(ParameterItem::Boolean(BooleanParameter {
                value: *value,
                name: name.clone(),
            }))
        }
        IrFieldParameter::Integer { name, value } => {
            Some(ParameterItem::Integer(IntegerParameter {
                value: *value,
                name: name.clone(),
            }))
        }
        IrFieldParameter::Float { name, value } => Some(ParameterItem::Float(FloatParameter {
            value: *value,
            name: name.clone(),
        })),
        IrFieldParameter::String { name, value } => Some(ParameterItem::String(StringParameter {
            value: value.clone(),
            name: name.clone(),
        })),
        IrFieldParameter::List(nested) => {
            // 재귀적으로 중첩된 리스트 변환
            convert_ir_parameters_to_hwpx(nested).map(|list| ParameterItem::List(Box::new(list)))
        }
    }
}

// ============================================
// ObjectCommon → HWPX 변환 헬퍼 함수들
// ============================================

/// IR ObjectCommon → HWPX ShapeObjectSize 변환
fn convert_object_common_to_size(common: &IrObjectCommon) -> Option<ShapeObjectSize> {
    Some(ShapeObjectSize {
        width: Some(common.size.width.value() as u32),
        width_relative_to: Default::default(),
        height: Some(common.size.height.value() as u32),
        height_relative_to: Default::default(),
        protect: false,
    })
}

/// IR ObjectCommon → HWPX ShapeObjectPosition 변환
fn convert_object_common_to_position(common: &IrObjectCommon) -> Option<ShapeObjectPosition> {
    let text_wrap = &common.text_wrap;

    let vertical_relative_to = match text_wrap.vertical_rel {
        IrVerticalRelativeTo::Paper => Some(HwpxVertRelTo::Paper),
        IrVerticalRelativeTo::Page => Some(HwpxVertRelTo::Page),
        IrVerticalRelativeTo::Paragraph => Some(HwpxVertRelTo::Paragraph),
    };

    let horizontal_relative_to = match text_wrap.horizontal_rel {
        IrHorizontalRelativeTo::Paper => Some(HwpxHorzRelTo::Paper),
        IrHorizontalRelativeTo::Page => Some(HwpxHorzRelTo::Page),
        IrHorizontalRelativeTo::Column => Some(HwpxHorzRelTo::Column),
        IrHorizontalRelativeTo::Paragraph => Some(HwpxHorzRelTo::Paragraph),
    };

    Some(ShapeObjectPosition {
        treat_as_character: text_wrap.treat_as_char,
        affect_line_spacing: false,
        flow_with_text: text_wrap.flow_with_text,
        allow_overlap: text_wrap.allow_overlap,
        hold_anchor_and_shape_object: false,
        vertical_relative_to,
        horizontal_relative_to,
        vertical_alignment: None,
        horizontal_alignment: None,
        vertical_offset: common.position.y.value().max(0) as u32,
        horizontal_offset: common.position.x.value().max(0) as u32,
    })
}

/// IR TextWrap → HWPX TextWrapMode 변환
fn convert_text_wrap_to_mode(text_wrap: &IrTextWrap) -> TextWrapMode {
    match text_wrap.wrap_type {
        IrTextWrapType::Inline => TextWrapMode::Square,
        IrTextWrapType::Square => TextWrapMode::Square,
        IrTextWrapType::Tight => TextWrapMode::Tight,
        IrTextWrapType::Behind => TextWrapMode::BehindText,
        IrTextWrapType::InFront => TextWrapMode::InFrontOfText,
    }
}

/// IR TextWrap → HWPX TextFlowMode 변환
fn convert_text_wrap_to_flow(text_wrap: &IrTextWrap) -> TextFlowMode {
    match text_wrap.wrap_side {
        IrTextWrapSide::Both => TextFlowMode::BothSides,
        IrTextWrapSide::Left => TextFlowMode::LeftOnly,
        IrTextWrapSide::Right => TextFlowMode::RightOnly,
        IrTextWrapSide::Largest => TextFlowMode::LargestOnly,
    }
}

/// IR Caption → HWPX Caption 변환
fn convert_ir_caption_to_hwpx(
    caption: &ir::control::Caption,
) -> Result<crate::paragraph::table::Caption, ConversionError> {
    use crate::paragraph::enums::{ParagraphLineWrap, ParagraphVerticalAlignment, TextDirection};
    use crate::paragraph::para_list::ParagraphList;
    use crate::paragraph::shape_common::CaptionSide;
    use ir::control::CaptionPosition as IrCaptionPosition;

    // 캡션 위치 변환
    let side = match caption.position {
        IrCaptionPosition::Left => CaptionSide::Left,
        IrCaptionPosition::Right => CaptionSide::Right,
        IrCaptionPosition::Top => CaptionSide::Top,
        IrCaptionPosition::Bottom => CaptionSide::Bottom,
    };

    // 캡션 내용 (문단들) 변환
    let mut paragraphs = Vec::new();
    for (i, para) in caption.paragraphs.iter().enumerate() {
        paragraphs.push(convert_paragraph(para, i as u32)?);
    }

    let paragraph_list = ParagraphList {
        paragraphs,
        id: "0".to_string(),
        text_direction: TextDirection::default(),
        line_wrap: ParagraphLineWrap::default(),
        vertical_alignment: ParagraphVerticalAlignment::default(),
        link_list_id_reference: None,
        link_list_next_id_reference: None,
        text_width: None,
        text_height: None,
        has_text_reference: false,
        has_number_reference: false,
    };

    Ok(crate::paragraph::table::Caption {
        paragraph_list,
        side,
        full_size: false,
        width: if caption.width.value() > 0 {
            Some(caption.width.value())
        } else {
            None
        },
        gap: caption.gap.value(),
        last_width: None,
    })
}

/// IR Shape → ContainerChild 변환
fn convert_ir_shape_to_container_child(
    shape: &IrShape,
) -> Result<Option<crate::paragraph::ole_equation::ContainerChild>, ConversionError> {
    use crate::paragraph::ole_equation::ContainerChild;
    use ir::shape::ShapeType;

    // convert_shape_to_hwpx를 활용하여 RunContent를 얻은 후 ContainerChild로 변환
    if let Some(run_content) = convert_shape_to_hwpx(shape)? {
        match run_content {
            HwpxRunContent::Line(line) => Ok(Some(ContainerChild::Line(*line))),
            HwpxRunContent::Rectangle(rect) => Ok(Some(ContainerChild::Rectangle(*rect))),
            HwpxRunContent::Ellipse(ellipse) => Ok(Some(ContainerChild::Ellipse(*ellipse))),
            HwpxRunContent::Arc(arc) => Ok(Some(ContainerChild::Arc(*arc))),
            HwpxRunContent::Polygon(polygon) => Ok(Some(ContainerChild::Polygon(*polygon))),
            HwpxRunContent::Curve(curve) => Ok(Some(ContainerChild::Curve(*curve))),
            HwpxRunContent::Container(container) => Ok(Some(ContainerChild::Container(container))),
            // 기타 RunContent는 ContainerChild로 변환 불가
            _ => Ok(None),
        }
    } else {
        // Group인 경우 직접 처리
        if let ShapeType::Group(children) = &shape.shape_type {
            let hwpx_container = convert_group_to_container(shape, children)?;
            Ok(Some(ContainerChild::Container(Box::new(hwpx_container))))
        } else {
            Ok(None)
        }
    }
}

/// IR 마스터 페이지 → HWPX 마스터 페이지 변환
fn convert_master_page_to_hwpx(
    mp_info: &ir::extensions::MasterPageInfo,
) -> Result<crate::master_page::MasterPage, ConversionError> {
    use crate::master_page::{MasterPage, MasterPageApplicationType};
    use crate::paragraph::enums::{ParagraphLineWrap, ParagraphVerticalAlignment, TextDirection};
    use crate::paragraph::para_list::ParagraphList;

    // 적용 유형 변환
    let application_type = match mp_info.application_type {
        ir::extensions::MasterPageApplicationType::Both => MasterPageApplicationType::Both,
        ir::extensions::MasterPageApplicationType::Even => MasterPageApplicationType::Even,
        ir::extensions::MasterPageApplicationType::Odd => MasterPageApplicationType::Odd,
        ir::extensions::MasterPageApplicationType::Last => MasterPageApplicationType::LastPage,
        ir::extensions::MasterPageApplicationType::Optional => {
            MasterPageApplicationType::OptionalPage
        }
    };

    // 문단들 변환
    let mut paragraphs = Vec::new();
    for (i, para) in mp_info.paragraphs.iter().enumerate() {
        paragraphs.push(convert_paragraph(para, i as u32)?);
    }

    let paragraph_list = ParagraphList {
        paragraphs,
        id: "0".to_string(),
        text_direction: TextDirection::default(),
        line_wrap: ParagraphLineWrap::default(),
        vertical_alignment: ParagraphVerticalAlignment::default(),
        link_list_id_reference: None,
        link_list_next_id_reference: None,
        text_width: None,
        text_height: None,
        has_text_reference: false,
        has_number_reference: false,
    };

    Ok(MasterPage {
        paragraph_list,
        id: mp_info.id.clone(),
        application_type,
        page_number: mp_info.page_number,
        page_duplicate: mp_info.page_duplicate,
        page_front: mp_info.page_front,
    })
}
