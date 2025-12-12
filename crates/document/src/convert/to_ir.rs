//! Document → IR 변환
//!
//! Document 모델을 IR 문서로 변환합니다.

use crate::control::Control;
use crate::document::Document;
use crate::paragraph::{BreakType, Paragraph, RangeTag, RangeTagType};
use crate::run::Run;
use crate::run_content::RunContent;
use crate::section::Section;
use crate::table::{Table, TableCell, TableRow};

impl From<Document> for ir::Document {
    fn from(doc: Document) -> Self {
        let mut ir_doc = ir::Document::new();

        // 섹션 변환 (doc 참조 필요하므로 먼저 수행)
        for &section_id in &doc.sections {
            if let Some(section) = doc.arena.get_section(section_id) {
                let ir_section = convert_section_to_ir(section, &doc);
                ir_doc.sections.push(ir_section);
            }
        }

        // 데이터 복사 (섹션 변환 후 이동)
        ir_doc.metadata = doc.metadata;
        ir_doc.styles = doc.styles;
        ir_doc.binary_data = doc.binary_data;
        ir_doc.extensions = doc.extensions;

        ir_doc
    }
}

/// Section을 IR로 변환
fn convert_section_to_ir(section: &Section, doc: &Document) -> ir::Section {
    let mut ir_section = ir::Section::default();

    // 문단 변환
    for &para_id in &section.paragraphs {
        if let Some(para) = doc.arena.get_paragraph(para_id) {
            let ir_para = convert_paragraph_to_ir(para, doc);
            ir_section.paragraphs.push(ir_para);
        }
    }

    ir_section
}

/// Paragraph을 IR로 변환
fn convert_paragraph_to_ir(para: &Paragraph, doc: &Document) -> ir::Paragraph {
    let mut ir_para = ir::Paragraph {
        para_shape_id: para.para_shape_id,
        style_id: para.style_id,
        runs: Vec::new(),
        break_type: match para.break_type {
            BreakType::None => primitive::BreakType::None,
            BreakType::Page => primitive::BreakType::Page,
            BreakType::Column => primitive::BreakType::Column,
            BreakType::Section => primitive::BreakType::Section,
        },
        instance_id: para.instance_id,
        line_segments: None,
        range_tags: para
            .range_tags
            .iter()
            .map(convert_range_tag_to_ir)
            .collect(),
    };

    // 런 변환
    for &run_id in &para.runs {
        if let Some(run) = doc.arena.get_run(run_id) {
            let ir_run = convert_run_to_ir(run, doc);
            ir_para.runs.push(ir_run);
        }
    }

    ir_para
}

/// RangeTag을 IR로 변환
fn convert_range_tag_to_ir(tag: &RangeTag) -> ir::paragraph::RangeTag {
    ir::paragraph::RangeTag {
        start: tag.start,
        end: tag.end,
        tag_type: match tag.tag_type {
            RangeTagType::Bookmark => ir::paragraph::RangeTagType::Bookmark,
            RangeTagType::Hyperlink => ir::paragraph::RangeTagType::Hyperlink,
            RangeTagType::TrackChangeInsert => ir::paragraph::RangeTagType::TrackChangeInsert,
            RangeTagType::TrackChangeDelete => ir::paragraph::RangeTagType::TrackChangeDelete,
            RangeTagType::Highlight => ir::paragraph::RangeTagType::Highlight,
            RangeTagType::Other(v) => ir::paragraph::RangeTagType::Other(v),
        },
        data: tag.data.clone(),
        track_change_info: None,
    }
}

/// Run을 IR로 변환
fn convert_run_to_ir(run: &Run, doc: &Document) -> ir::paragraph::Run {
    let mut ir_run = ir::paragraph::Run {
        char_shape_id: run.char_shape_id,
        contents: Vec::new(),
    };

    for content in &run.contents {
        let ir_content = convert_run_content_to_ir(content, doc);
        ir_run.contents.push(ir_content);
    }

    ir_run
}

/// RunContent을 IR로 변환
fn convert_run_content_to_ir(content: &RunContent, doc: &Document) -> ir::paragraph::RunContent {
    match content {
        RunContent::Text(s) => ir::paragraph::RunContent::Text(ir::paragraph::Text::new(s.clone())),
        RunContent::Tab => ir::paragraph::RunContent::Tab(ir::paragraph::TabChar::default()),
        RunContent::LineBreak => ir::paragraph::RunContent::LineBreak,
        RunContent::Hyphen => ir::paragraph::RunContent::Hyphen,
        RunContent::NonBreakingSpace => ir::paragraph::RunContent::NonBreakingSpace,
        RunContent::FixedWidthSpace => ir::paragraph::RunContent::FixedWidthSpace,
        RunContent::Control(ctrl_id) => {
            if let Some(ctrl) = doc.arena.get_control(*ctrl_id) {
                // Compose와 Dutmal은 IR에서 RunContent이므로 특별 처리
                match ctrl {
                    Control::Compose(c) => convert_compose_to_ir_run_content(c),
                    Control::Dutmal(d) => convert_dutmal_to_ir_run_content(d),
                    _ => {
                        let ir_ctrl = convert_control_to_ir(ctrl, doc);
                        ir::paragraph::RunContent::Control(Box::new(ir_ctrl))
                    }
                }
            } else {
                ir::paragraph::RunContent::Text(ir::paragraph::Text::new(""))
            }
        }
        RunContent::FieldStart(_) => {
            // TODO: proper conversion
            ir::paragraph::RunContent::FieldEnd(ir::paragraph::FieldEnd { id: 0 })
        }
        RunContent::FieldEnd => {
            ir::paragraph::RunContent::FieldEnd(ir::paragraph::FieldEnd { id: 0 })
        }
        RunContent::BookmarkStart(bs) => {
            ir::paragraph::RunContent::BookmarkStart(ir::paragraph::BookmarkStart {
                id: bs.id,
                name: bs.name.clone(),
            })
        }
        RunContent::BookmarkEnd => {
            ir::paragraph::RunContent::BookmarkEnd(ir::paragraph::BookmarkEnd { id: 0 })
        }
    }
}

/// 빈 UnknownControl 생성
const fn empty_unknown_control() -> ir::control::UnknownControl {
    ir::control::UnknownControl {
        ctrl_id: [0, 0, 0, 0],
        data: Vec::new(),
    }
}

/// ObjectCommon을 IR로 변환
fn convert_object_common_to_ir(common: &crate::control::ObjectCommon) -> ir::control::ObjectCommon {
    ir::control::ObjectCommon {
        id: common.id.as_ref().and_then(|s| {
            // String을 u32로 변환 시도 (파싱 실패시 None)
            s.parse::<u32>().ok()
        }),
        position: common.position,
        size: common.size,
        z_order: common.z_order,
        text_wrap: convert_text_wrap_to_ir(&common.text_wrap),
        caption: common.caption.as_ref().map(convert_caption_to_ir),
        numbering_type: None,
        shape_comment: None,
        meta_tag: None,
        dirty: false,
        width_relative_to: primitive::WidthRelativeTo::Absolute,
        height_relative_to: primitive::HeightRelativeTo::Absolute,
        margin: ir::control::ObjectMargin::default(),
    }
}

/// TextWrap을 IR로 변환
const fn convert_text_wrap_to_ir(wrap: &crate::control::TextWrap) -> ir::control::TextWrap {
    ir::control::TextWrap {
        wrap_type: wrap.wrap_type,
        wrap_side: wrap.wrap_side,
        margin: wrap.margin.top,
        vertical_rel: wrap.vertical_rel,
        horizontal_rel: wrap.horizontal_rel,
        vertical_offset_type: primitive::VerticalOffsetType::Top,
        horizontal_offset_type: primitive::HorizontalOffsetType::Left,
        treat_as_char: wrap.treat_as_char,
        flow_with_text: wrap.flow_with_text,
        allow_overlap: wrap.allow_overlap,
    }
}

/// Caption을 IR로 변환
const fn convert_caption_to_ir(caption: &crate::control::Caption) -> ir::control::Caption {
    ir::control::Caption {
        position: match caption.position {
            crate::control::CaptionPosition::Left => ir::control::CaptionPosition::Left,
            crate::control::CaptionPosition::Right => ir::control::CaptionPosition::Right,
            crate::control::CaptionPosition::Top => ir::control::CaptionPosition::Top,
            crate::control::CaptionPosition::Bottom => ir::control::CaptionPosition::Bottom,
        },
        width: caption.width,
        gap: caption.gap,
        paragraphs: Vec::new(), // 캡션의 문단은 arena에서 참조되므로 여기서는 빈 벡터
    }
}

/// Ole을 IR로 변환
fn convert_ole_to_ir(ole: &crate::control::Ole) -> ir::control::OleObject {
    ir::control::OleObject {
        common: convert_object_common_to_ir(&ole.common),
        binary_id: ole.binary_id.clone(),
        class_id: ole.class_id.clone(),
        preview_image_id: ole.preview_image_id.clone(),
    }
}

/// TextBox을 IR로 변환
fn convert_textbox_to_ir(
    textbox: &crate::control::TextBox,
    doc: &Document,
) -> ir::control::TextBox {
    let mut paragraphs = Vec::new();
    for &para_id in &textbox.paragraphs {
        if let Some(para) = doc.arena.get_paragraph(para_id) {
            paragraphs.push(convert_paragraph_to_ir(para, doc));
        }
    }

    ir::control::TextBox {
        common: convert_object_common_to_ir(&textbox.common),
        paragraphs,
        text_direction: convert_text_direction_to_ir(textbox.text_direction),
        vertical_alignment: textbox.vertical_alignment,
        padding: textbox.padding,
        editable: textbox.editable,
        name: None,
        last_width: None,
        line_wrap: primitive::LineWrap::Break,
        link_list_id_reference: None,
        link_list_next_id_reference: None,
        text_width: None,
        text_height: None,
        has_text_reference: false,
        has_number_reference: false,
    }
}

/// Note을 IR로 변환
fn convert_note_to_ir(note: &crate::control::Note, doc: &Document) -> ir::control::Note {
    let mut paragraphs = Vec::new();
    for &para_id in &note.paragraphs {
        if let Some(para) = doc.arena.get_paragraph(para_id) {
            paragraphs.push(convert_paragraph_to_ir(para, doc));
        }
    }

    ir::control::Note {
        number: note.number,
        number_format: note.number_format,
        number_position: convert_note_number_position_to_ir(note.number_position),
        paragraphs,
        instance_id: note.instance_id,
    }
}

/// HiddenComment을 IR로 변환
fn convert_hidden_comment_to_ir(
    hidden_comment: &crate::control::HiddenComment,
    doc: &Document,
) -> ir::control::HiddenComment {
    let mut paragraphs = Vec::new();
    for &para_id in &hidden_comment.paragraphs {
        if let Some(para) = doc.arena.get_paragraph(para_id) {
            paragraphs.push(convert_paragraph_to_ir(para, doc));
        }
    }

    ir::control::HiddenComment { paragraphs }
}

/// TextDirection을 IR로 변환
const fn convert_text_direction_to_ir(
    text_direction: crate::control::TextDirection,
) -> primitive::TextDirection {
    match text_direction {
        crate::control::TextDirection::Horizontal => primitive::TextDirection::Horizontal,
        crate::control::TextDirection::Vertical => primitive::TextDirection::Vertical,
        crate::control::TextDirection::VerticalAll => primitive::TextDirection::VerticalRightToLeft,
    }
}

/// NoteNumberPosition을 IR로 변환
const fn convert_note_number_position_to_ir(
    position: crate::section::NoteNumberPosition,
) -> primitive::NoteNumberPosition {
    match position {
        crate::section::NoteNumberPosition::Superscript => {
            primitive::NoteNumberPosition::Superscript
        }
        crate::section::NoteNumberPosition::Subscript => primitive::NoteNumberPosition::Subscript,
    }
}

/// Control을 IR로 변환
fn convert_control_to_ir(ctrl: &Control, doc: &Document) -> ir::control::Control {
    match ctrl {
        Control::Table(t) => ir::control::Control::Table(Box::new(convert_table_to_ir(t, doc))),
        Control::Picture(p) => ir::control::Control::Picture(Box::new(convert_picture_to_ir(p))),
        Control::Shape(s) => ir::control::Control::Shape(Box::new(convert_shape_to_ir(s, doc))),
        Control::Equation(e) => ir::control::Control::Equation(Box::new(convert_equation_to_ir(e))),
        Control::Ole(o) => ir::control::Control::Ole(Box::new(convert_ole_to_ir(o))),
        Control::TextBox(tb) => {
            ir::control::Control::TextBox(Box::new(convert_textbox_to_ir(tb, doc)))
        }
        Control::Footnote(n) => {
            ir::control::Control::Footnote(Box::new(convert_note_to_ir(n, doc)))
        }
        Control::Endnote(n) => ir::control::Control::Endnote(Box::new(convert_note_to_ir(n, doc))),
        Control::HiddenComment(hc) => {
            ir::control::Control::HiddenComment(Box::new(convert_hidden_comment_to_ir(hc, doc)))
        }
        Control::Hyperlink(h) => {
            ir::control::Control::Hyperlink(Box::new(convert_hyperlink_to_ir(h)))
        }
        Control::Bookmark(b) => ir::control::Control::Bookmark(Box::new(ir::control::Bookmark {
            name: b.name.clone(),
        })),
        Control::AutoNumber(an) => {
            ir::control::Control::AutoNumber(Box::new(convert_auto_number_to_ir(an)))
        }
        Control::NewNumber(nn) => {
            ir::control::Control::NewNumber(Box::new(convert_new_number_to_ir(nn)))
        }
        Control::FormObject(fo) => {
            ir::control::Control::FormObject(Box::new(convert_form_object_to_ir(fo)))
        }
        Control::Video(v) => ir::control::Control::Video(Box::new(convert_video_to_ir(v))),
        Control::Chart(c) => ir::control::Control::Chart(Box::new(convert_chart_to_ir(c))),
        Control::TextArt(ta) => ir::control::Control::TextArt(Box::new(convert_text_art_to_ir(ta))),
        // Compose와 Dutmal은 IR에서 RunContent이므로 여기 도달하면 안됨 (convert_run_content_to_ir에서 처리)
        // 만약 여기 도달하면 Unknown으로 처리
        Control::Compose(_) => ir::control::Control::Unknown(Box::new(empty_unknown_control())),
        Control::Dutmal(_) => ir::control::Control::Unknown(Box::new(empty_unknown_control())),
        Control::IndexMark(im) => {
            ir::control::Control::IndexMark(Box::new(convert_index_mark_to_ir(im)))
        }
        // ConnectLine은 추후 구현 (일단 Unknown으로)
        Control::ConnectLine(_) => ir::control::Control::Unknown(Box::new(empty_unknown_control())),
        Control::Unknown(data) => {
            ir::control::Control::Unknown(Box::new(ir::control::UnknownControl {
                ctrl_id: [0, 0, 0, 0],
                data: data.clone(),
            }))
        }
    }
}

/// Table을 IR로 변환
fn convert_table_to_ir(table: &Table, doc: &Document) -> ir::table::Table {
    let mut ir_table = ir::table::Table::new(table.row_count, table.column_count);
    ir_table.cell_spacing = table.cell_spacing;
    ir_table.border_fill_id = table.border_fill_id;

    // 행/셀 변환
    ir_table.rows.clear();
    for &row_id in &table.rows {
        if let Some(row) = doc.arena.get_row(row_id) {
            let ir_row = convert_table_row_to_ir(row, doc);
            ir_table.rows.push(ir_row);
        }
    }

    ir_table
}

/// TableRow을 IR로 변환
fn convert_table_row_to_ir(row: &TableRow, doc: &Document) -> ir::table::TableRow {
    let mut ir_row = ir::table::TableRow {
        height: row.height,
        cells: Vec::new(),
    };

    for &cell_id in &row.cells {
        if let Some(cell) = doc.arena.get_cell(cell_id) {
            let ir_cell = convert_table_cell_to_ir(cell, doc);
            ir_row.cells.push(ir_cell);
        }
    }

    ir_row
}

/// TableCell을 IR로 변환
fn convert_table_cell_to_ir(cell: &TableCell, doc: &Document) -> ir::table::TableCell {
    let mut ir_cell = ir::table::TableCell::new(cell.row, cell.column);
    ir_cell.column_span = cell.column_span;
    ir_cell.row_span = cell.row_span;
    ir_cell.width = cell.width;
    ir_cell.height = cell.height;
    ir_cell.padding = cell.padding;
    ir_cell.border_fill_id = cell.border_fill_id;

    // 셀 내용 문단 변환
    for &para_id in &cell.paragraphs {
        if let Some(para) = doc.arena.get_paragraph(para_id) {
            let ir_para = convert_paragraph_to_ir(para, doc);
            ir_cell.paragraphs.push(ir_para);
        }
    }

    ir_cell
}

/// Picture를 IR로 변환
fn convert_picture_to_ir(picture: &crate::control::Picture) -> ir::picture::Picture {
    ir::picture::Picture {
        common: convert_object_common_to_ir(&picture.common),
        binary_id: picture.binary_id.clone(),
        original_size: picture.original_size,
        crop: ir::picture::ImageCrop {
            left: picture.crop.left,
            right: picture.crop.right,
            top: picture.crop.top,
            bottom: picture.crop.bottom,
        },
        flip: match picture.flip {
            crate::control::ImageFlip::None => primitive::ImageFlip::None,
            crate::control::ImageFlip::Horizontal => primitive::ImageFlip::Horizontal,
            crate::control::ImageFlip::Vertical => primitive::ImageFlip::Vertical,
            crate::control::ImageFlip::Both => primitive::ImageFlip::Both,
        },
        rotation: picture.rotation,
        effect: match picture.effect {
            crate::control::ImageEffect::Original => primitive::ImageEffect::Original,
            crate::control::ImageEffect::Grayscale => primitive::ImageEffect::Grayscale,
            crate::control::ImageEffect::BlackWhite => primitive::ImageEffect::BlackWhite,
            crate::control::ImageEffect::Pattern => primitive::ImageEffect::Pattern,
        },
        brightness: picture.brightness,
        contrast: picture.contrast,
        alpha: (picture.alpha as f64) / 100.0, // 0-100 -> 0.0-1.0
        transparent_color: None,
        border: picture.border.as_ref().map(|b| ir::picture::PictureBorder {
            line_type: b.line_type,
            width: b.width,
            color: b.color,
        }),
        shadow: picture.shadow.as_ref().map(|s| ir::picture::PictureShadow {
            shadow_type: match s.shadow_type {
                crate::control::ShadowType::None => ir::picture::PictureShadowType::None,
                crate::control::ShadowType::Drop => ir::picture::PictureShadowType::BottomRight,
                crate::control::ShadowType::Inner => ir::picture::PictureShadowType::None,
            },
            color: s.color,
            offset_x: s.offset_x,
            offset_y: s.offset_y,
            alpha: (s.alpha as f64) / 100.0,
        }),
        inside_margin: ir::Insets::ZERO,
    }
}

/// Shape를 IR로 변환
fn convert_shape_to_ir(shape: &crate::control::Shape, doc: &Document) -> ir::shape::Shape {
    ir::shape::Shape {
        common: convert_object_common_to_ir(&shape.common),
        shape_type: convert_shape_type_to_ir(&shape.shape_type, doc),
        line: convert_line_style_to_ir(&shape.line),
        fill: convert_fill_to_ir(&shape.fill),
        shadow: shape.shadow.as_ref().map(|s| ir::shape::ShapeShadow {
            color: s.color,
            offset_x: s.offset_x,
            offset_y: s.offset_y,
            alpha: (s.alpha as f64) / 100.0,
            blur: s.blur.map(|b| (b.value() as f64 / 100.0) as f32), // HwpUnit to f32
            direction: s.direction.map(|d| d as f32),
            distance: s.distance,
        }),
        rotation: shape.rotation,
        text: shape
            .text
            .as_ref()
            .map(|t| convert_shape_text_to_ir(t, doc)),
        translation_matrix: None,
        scale_matrix: None,
        rotation_matrix: None,
    }
}

/// ShapeType을 IR로 변환
fn convert_shape_type_to_ir(
    shape_type: &crate::control::ShapeType,
    doc: &Document,
) -> ir::shape::ShapeType {
    match shape_type {
        crate::control::ShapeType::Line {
            start,
            end,
            start_arrow,
            end_arrow,
        } => ir::shape::ShapeType::Line(ir::shape::LineShape {
            start: *start,
            end: *end,
            start_arrow: start_arrow
                .as_ref()
                .map_or(ir::shape::Arrow::default(), convert_arrow_to_ir),
            end_arrow: end_arrow
                .as_ref()
                .map_or(ir::shape::Arrow::default(), convert_arrow_to_ir),
        }),
        crate::control::ShapeType::Rectangle { corner_radius } => {
            ir::shape::ShapeType::Rectangle(ir::shape::RectangleShape {
                corner_radius: *corner_radius,
            })
        }
        crate::control::ShapeType::Ellipse {
            arc_type,
            start_angle,
            end_angle,
        } => ir::shape::ShapeType::Ellipse(ir::shape::EllipseShape {
            arc_type: convert_arc_type_to_ir(*arc_type),
            start_angle: *start_angle,
            end_angle: *end_angle,
        }),
        crate::control::ShapeType::Arc {
            arc_type,
            start_angle,
            end_angle,
        } => ir::shape::ShapeType::Arc(ir::shape::ArcShape {
            arc_type: convert_arc_type_to_ir(*arc_type),
            start_angle: *start_angle,
            end_angle: *end_angle,
        }),
        crate::control::ShapeType::Polygon { points } => {
            ir::shape::ShapeType::Polygon(ir::shape::PolygonShape {
                points: points.clone(),
            })
        }
        crate::control::ShapeType::Curve { points, closed } => {
            ir::shape::ShapeType::Curve(ir::shape::CurveShape {
                points: points
                    .iter()
                    .map(|p| ir::shape::CurvePoint {
                        point: p.point,
                        point_type: match p.point_type {
                            crate::control::CurvePointType::Normal => {
                                ir::shape::CurvePointType::Normal
                            }
                            crate::control::CurvePointType::Control1 => {
                                ir::shape::CurvePointType::Control1
                            }
                            crate::control::CurvePointType::Control2 => {
                                ir::shape::CurvePointType::Control2
                            }
                        },
                    })
                    .collect(),
                closed: *closed,
            })
        }
        crate::control::ShapeType::Connector {
            connector_type,
            points,
            start_arrow,
            end_arrow,
        } => ir::shape::ShapeType::Connector(ir::shape::ConnectorShape {
            connector_type: match connector_type {
                crate::control::ConnectorType::Straight => ir::shape::ConnectorType::Straight,
                crate::control::ConnectorType::Elbow => ir::shape::ConnectorType::Elbow,
                crate::control::ConnectorType::Curved => ir::shape::ConnectorType::Curved,
            },
            start: ir::shape::ConnectorPoint {
                point: points.first().copied().unwrap_or_default(),
                subject_id_ref: None,
                subject_index: None,
            },
            end: ir::shape::ConnectorPoint {
                point: points.last().copied().unwrap_or_default(),
                subject_id_ref: None,
                subject_index: None,
            },
            start_arrow: start_arrow
                .as_ref()
                .map_or(ir::shape::Arrow::default(), convert_arrow_to_ir),
            end_arrow: end_arrow
                .as_ref()
                .map_or(ir::shape::Arrow::default(), convert_arrow_to_ir),
            control_points: points[1..points.len().saturating_sub(1)]
                .iter()
                .map(|&point| ir::shape::CurvePoint {
                    point,
                    point_type: ir::shape::CurvePointType::Normal,
                })
                .collect(),
        }),
        crate::control::ShapeType::Group { children } => ir::shape::ShapeType::Group(
            children
                .iter()
                .map(|s| convert_shape_to_ir(s, doc))
                .collect(),
        ),
    }
}

/// Arrow을 IR로 변환
const fn convert_arrow_to_ir(arrow: &crate::control::Arrow) -> ir::shape::Arrow {
    ir::shape::Arrow {
        arrow_type: match arrow.arrow_type {
            crate::control::ArrowType::None => primitive::ArrowType::None,
            crate::control::ArrowType::Normal => primitive::ArrowType::Arrow,
            crate::control::ArrowType::Stealth => primitive::ArrowType::Stealth,
            crate::control::ArrowType::Diamond => primitive::ArrowType::Diamond,
            crate::control::ArrowType::Circle => primitive::ArrowType::Circle,
            crate::control::ArrowType::Open => primitive::ArrowType::ArrowOpen,
        },
        size: match arrow.size {
            crate::control::ArrowSize::Small => primitive::ArrowSize::Small,
            crate::control::ArrowSize::Medium => primitive::ArrowSize::Medium,
            crate::control::ArrowSize::Large => primitive::ArrowSize::Large,
        },
        filled: arrow.filled,
    }
}

/// ArcType을 IR로 변환
const fn convert_arc_type_to_ir(arc_type: crate::control::ArcType) -> ir::shape::ArcType {
    match arc_type {
        crate::control::ArcType::Full => ir::shape::ArcType::Full,
        crate::control::ArcType::Arc => ir::shape::ArcType::Arc,
        crate::control::ArcType::Pie => ir::shape::ArcType::Pie,
        crate::control::ArcType::Chord => ir::shape::ArcType::Chord,
    }
}

/// LineStyle을 IR로 변환
fn convert_line_style_to_ir(line: &Option<crate::control::LineStyle>) -> ir::shape::LineStyle {
    match line {
        Some(l) => ir::shape::LineStyle {
            line_type: l.line_type,
            width: l.width,
            color: l.color,
            cap: match l.cap {
                crate::control::LineCap::Flat => primitive::LineCap::Flat,
                crate::control::LineCap::Round => primitive::LineCap::Round,
                crate::control::LineCap::Square => primitive::LineCap::Square,
            },
            outline_style: match l.outline_style {
                Some(crate::control::OutlineStyle::Normal) => primitive::LineOutlineStyle::Normal,
                Some(crate::control::OutlineStyle::Outer) => primitive::LineOutlineStyle::Outer,
                Some(crate::control::OutlineStyle::Inner) => primitive::LineOutlineStyle::Inner,
                None => primitive::LineOutlineStyle::Normal,
            },
            alpha: l.alpha.map(|a| a as f32),
        },
        None => ir::shape::LineStyle::default(),
    }
}

/// Fill을 IR로 변환
fn convert_fill_to_ir(fill: &Option<crate::control::Fill>) -> ir::border_fill::Fill {
    match fill {
        Some(crate::control::Fill::Solid(s)) => {
            ir::border_fill::Fill::Solid(ir::border_fill::SolidFill {
                color: s.color,
                alpha: ((s.alpha as f32 / 100.0) * 255.0) as u8,
            })
        }
        Some(crate::control::Fill::Gradient(g)) => {
            ir::border_fill::Fill::Gradient(ir::border_fill::GradientFill {
                gradient_type: match g.gradient_type {
                    crate::control::GradientType::Linear => primitive::GradientType::Linear,
                    crate::control::GradientType::Radial => primitive::GradientType::Radial,
                    crate::control::GradientType::Conical => primitive::GradientType::Conical,
                    crate::control::GradientType::Square => primitive::GradientType::Square,
                },
                angle: (g.angle as u16).clamp(0, 360),
                center_x: (g.center_x.clamp(0, 100)) as u8,
                center_y: (g.center_y.clamp(0, 100)) as u8,
                stops: g
                    .stops
                    .iter()
                    .map(|s| ir::border_fill::GradientStop {
                        position: s.position,
                        color: s.color,
                    })
                    .collect(),
                blur: 0, // 기본값
                step_center: (g.step_center.clamp(0, 100)) as u8,
            })
        }
        Some(crate::control::Fill::Image(i)) => {
            ir::border_fill::Fill::Image(ir::border_fill::ImageFill {
                binary_id: i.binary_id.clone(),
                mode: i.fill_type,
                brightness: i.brightness,
                contrast: i.contrast,
                effect: i.effect,
                offset_x: primitive::HwpUnit::ZERO,
                offset_y: primitive::HwpUnit::ZERO,
                size: None,
            })
        }
        Some(crate::control::Fill::Pattern(p)) => {
            ir::border_fill::Fill::Pattern(ir::border_fill::PatternFill {
                pattern_type: p.pattern_type,
                foreground: p.foreground,
                background: p.background,
            })
        }
        None => ir::border_fill::Fill::None,
    }
}

/// ShapeText을 IR로 변환
fn convert_shape_text_to_ir(
    text: &crate::control::ShapeText,
    doc: &Document,
) -> ir::shape::ShapeText {
    let mut paragraphs = Vec::new();
    for &para_id in &text.paragraphs {
        if let Some(para) = doc.arena.get_paragraph(para_id) {
            paragraphs.push(convert_paragraph_to_ir(para, doc));
        }
    }

    ir::shape::ShapeText {
        paragraphs,
        padding: text.padding,
        vertical_alignment: text.vertical_alignment,
        text_direction: match text.text_direction {
            crate::control::TextDirection::Horizontal => primitive::TextDirection::Horizontal,
            crate::control::TextDirection::Vertical => primitive::TextDirection::Vertical,
            crate::control::TextDirection::VerticalAll => {
                primitive::TextDirection::VerticalRightToLeft
            }
        },
        editable: text.editable,
    }
}

/// Equation을 IR로 변환
fn convert_equation_to_ir(equation: &crate::control::Equation) -> ir::control::Equation {
    ir::control::Equation {
        common: convert_object_common_to_ir(&equation.common),
        script: equation.script.clone(),
        format: match equation.format {
            crate::control::EquationFormat::HwpScript => ir::control::EquationFormat::HwpScript,
            crate::control::EquationFormat::MathML => ir::control::EquationFormat::MathML,
            crate::control::EquationFormat::LaTeX => ir::control::EquationFormat::LaTeX,
        },
        baseline_offset: equation.baseline_offset,
        font_size: equation.font_size,
        color: Some(equation.color),
        line_mode: None,
        version: None,
        font_name: None,
        properties: None,
    }
}

/// Hyperlink을 IR로 변환
fn convert_hyperlink_to_ir(hyperlink: &crate::control::Hyperlink) -> ir::control::Hyperlink {
    ir::control::Hyperlink {
        target: convert_hyperlink_target_to_ir(&hyperlink.target),
        tooltip: hyperlink.tooltip.clone(),
        display_text: hyperlink.display_text.clone(),
    }
}

/// HyperlinkTarget을 IR로 변환
fn convert_hyperlink_target_to_ir(
    target: &crate::control::HyperlinkTarget,
) -> ir::control::HyperlinkTarget {
    match target {
        crate::control::HyperlinkTarget::Url(url) => ir::control::HyperlinkTarget::Url(url.clone()),
        crate::control::HyperlinkTarget::Email(email) => {
            ir::control::HyperlinkTarget::Email(email.clone())
        }
        crate::control::HyperlinkTarget::File(file) => {
            ir::control::HyperlinkTarget::File(file.clone())
        }
        crate::control::HyperlinkTarget::Bookmark(bookmark) => {
            ir::control::HyperlinkTarget::Bookmark(bookmark.clone())
        }
    }
}

/// AutoNumber을 IR로 변환
fn convert_auto_number_to_ir(auto_number: &crate::control::AutoNumber) -> ir::control::AutoNumber {
    ir::control::AutoNumber {
        number_type: convert_auto_number_type_to_ir(auto_number.number_type),
        number_format: auto_number.number_format,
        auto_number_format: None,
    }
}

/// AutoNumberType을 IR로 변환
const fn convert_auto_number_type_to_ir(
    number_type: crate::control::AutoNumberType,
) -> ir::control::AutoNumberType {
    match number_type {
        crate::control::AutoNumberType::Page => ir::control::AutoNumberType::Page,
        crate::control::AutoNumberType::Footnote => ir::control::AutoNumberType::Footnote,
        crate::control::AutoNumberType::Endnote => ir::control::AutoNumberType::Endnote,
        crate::control::AutoNumberType::Picture => ir::control::AutoNumberType::Picture,
        crate::control::AutoNumberType::Table => ir::control::AutoNumberType::Table,
        crate::control::AutoNumberType::Equation => ir::control::AutoNumberType::Equation,
    }
}

/// NewNumber을 IR로 변환
fn convert_new_number_to_ir(new_number: &crate::control::NewNumber) -> ir::control::NewNumber {
    ir::control::NewNumber {
        number_type: convert_auto_number_type_to_ir(new_number.number_type),
        number: new_number.number,
    }
}

/// IndexMark을 IR로 변환
fn convert_index_mark_to_ir(index_mark: &crate::control::IndexMark) -> ir::control::IndexMark {
    ir::control::IndexMark {
        first_key: index_mark.first_key.clone(),
        second_key: index_mark.second_key.clone().unwrap_or_default(),
    }
}

/// Video를 IR로 변환
fn convert_video_to_ir(video: &crate::control::Video) -> ir::control::Video {
    ir::control::Video {
        common: convert_object_common_to_ir(&video.common),
        video_type: match video.video_type {
            crate::control::VideoType::Embedded => ir::control::VideoType::Embedded,
            crate::control::VideoType::Linked => ir::control::VideoType::Linked,
            crate::control::VideoType::Web => ir::control::VideoType::YouTube,
        },
        video_id: video.video_id.clone(),
        source_url: video.source_url.clone(),
        preview_image_id: video.preview_image_id.clone(),
        poster_binary_id: None,
        width: None,
        height: None,
    }
}

/// Chart를 IR로 변환
fn convert_chart_to_ir(chart: &crate::control::Chart) -> ir::control::Chart {
    ir::control::Chart {
        common: convert_object_common_to_ir(&chart.common),
        chart_id: chart.chart_id.to_string(),
        chart_type: match chart.chart_type {
            crate::control::ChartType::Bar => ir::control::ChartType::Bar,
            crate::control::ChartType::Line => ir::control::ChartType::Line,
            crate::control::ChartType::Pie => ir::control::ChartType::Pie,
            crate::control::ChartType::Area => ir::control::ChartType::Area,
            crate::control::ChartType::Scatter => ir::control::ChartType::Scatter,
            crate::control::ChartType::Radar => ir::control::ChartType::Radar,
        },
    }
}

/// FormObject를 IR로 변환
fn convert_form_object_to_ir(form_object: &crate::control::FormObject) -> ir::control::FormObject {
    let mut ir_form = ir::control::FormObject {
        common: convert_object_common_to_ir(&form_object.common),
        form_type: ir::control::FormObjectType::Button, // 기본값, 아래서 재설정
        name: Some(form_object.name.clone()),
        value: form_object.value.clone(),
        char_property: ir::control::FormCharProperty::default(),
        items: Vec::new(),
        fore_color: None,
        back_color: None,
        group_name: None,
        tab_stop: true,
        enabled: true,
        editable: true,
        border_type_id_ref: None,
        draw_frame: true,
        printable: true,
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
    };

    // 양식 타입에 따라 필드 설정
    match &form_object.form_type {
        crate::control::FormObjectType::Button => {
            ir_form.form_type = ir::control::FormObjectType::Button;
        }
        crate::control::FormObjectType::CheckBox { checked } => {
            ir_form.form_type = ir::control::FormObjectType::CheckBox;
            ir_form.button_value = Some(if *checked {
                ir::control::ButtonValue::Checked
            } else {
                ir::control::ButtonValue::Unchecked
            });
        }
        crate::control::FormObjectType::RadioButton {
            group_name,
            checked,
        } => {
            ir_form.form_type = ir::control::FormObjectType::RadioButton;
            ir_form.radio_group_name = Some(group_name.clone());
            ir_form.button_value = Some(if *checked {
                ir::control::ButtonValue::Checked
            } else {
                ir::control::ButtonValue::Unchecked
            });
        }
        crate::control::FormObjectType::ComboBox { items, selected } => {
            ir_form.form_type = ir::control::FormObjectType::ComboBox;
            ir_form.items = items
                .iter()
                .map(|item| ir::control::FormListItem {
                    display_text: Some(item.clone()),
                    value: Some(item.clone()),
                })
                .collect();
            if let Some(item) = selected.and_then(|idx| items.get(idx)) {
                ir_form.selected_value = Some(item.clone());
            }
        }
        crate::control::FormObjectType::ListBox { items, selected } => {
            ir_form.form_type = ir::control::FormObjectType::ListBox;
            ir_form.items = items
                .iter()
                .map(|item| ir::control::FormListItem {
                    display_text: Some(item.clone()),
                    value: Some(item.clone()),
                })
                .collect();
            if let Some(item) = selected.and_then(|idx| items.get(idx)) {
                ir_form.selected_value = Some(item.clone());
            }
        }
        crate::control::FormObjectType::Edit {
            multiline,
            password,
        } => {
            ir_form.form_type = ir::control::FormObjectType::Edit;
            ir_form.multiline = *multiline;
            if *password {
                ir_form.password_char = Some("*".to_string());
            }
        }
        crate::control::FormObjectType::ScrollBar { min, max, value } => {
            ir_form.form_type = ir::control::FormObjectType::ScrollBar;
            ir_form.min = Some(*min);
            ir_form.max = Some(*max);
            ir_form.scroll_value = Some(*value);
        }
    }

    ir_form
}

/// TextArt를 IR로 변환
fn convert_text_art_to_ir(text_art: &crate::control::TextArt) -> ir::control::TextArt {
    ir::control::TextArt {
        common: convert_object_common_to_ir(&text_art.common),
        text: text_art.text.clone(),
        font_name: Some(text_art.font_name.clone()),
        font_style: match text_art.font_style {
            crate::control::FontStyle::Regular => ir::control::TextArtFontStyle::Regular,
            crate::control::FontStyle::Bold => ir::control::TextArtFontStyle::Bold,
            crate::control::FontStyle::Italic => ir::control::TextArtFontStyle::Italic,
            crate::control::FontStyle::BoldItalic => ir::control::TextArtFontStyle::BoldItalic,
        },
        shape: match text_art.shape {
            crate::control::TextArtShape::Rectangle => ir::control::TextArtShapeType::Rectangle,
            crate::control::TextArtShape::Circle => ir::control::TextArtShapeType::Circle,
            crate::control::TextArtShape::Arch => ir::control::TextArtShapeType::ArchUp,
            crate::control::TextArtShape::Wave => ir::control::TextArtShapeType::Wave,
        },
        line_spacing: text_art.line_spacing.0 as u32,
        char_spacing: text_art.char_spacing.0 as u32,
        alignment: match text_art.alignment {
            primitive::Alignment::Left => ir::control::TextArtAlignment::Left,
            primitive::Alignment::Center => ir::control::TextArtAlignment::Center,
            primitive::Alignment::Right => ir::control::TextArtAlignment::Right,
            primitive::Alignment::Justify
            | primitive::Alignment::Distribute
            | primitive::Alignment::Divide => ir::control::TextArtAlignment::Full,
        },
        line: convert_line_style_to_ir(&text_art.line),
        fill: convert_fill_to_ir(&text_art.fill),
        shadow: text_art.shadow.as_ref().map(|s| ir::shape::ShapeShadow {
            color: s.color,
            offset_x: s.offset_x,
            offset_y: s.offset_y,
            alpha: (s.alpha as f64) / 100.0,
            blur: s.blur.map(|b| (b.value() as f64 / 100.0) as f32),
            direction: s.direction.map(|d| d as f32),
            distance: s.distance,
        }),
        font_type: None,
        text_art_pr: None,
    }
}

/// Compose를 IR RunContent로 변환
///
/// IR에서 Compose는 RunContent이므로 직접 변환합니다.
fn convert_compose_to_ir_run_content(
    compose: &crate::control::Compose,
) -> ir::paragraph::RunContent {
    let ir_compose = ir::paragraph::Compose {
        compose_text: compose.compose_text.clone(),
        compose_type: Some(match compose.compose_type {
            crate::control::ComposeType::Spread => ir::paragraph::ComposeType::Spread,
            crate::control::ComposeType::Overlap => ir::paragraph::ComposeType::Overlap,
        }),
        circle_type: match compose.circle_type {
            crate::control::CircleType::None => ir::paragraph::ComposeCircleType::Char,
            crate::control::CircleType::Circle => ir::paragraph::ComposeCircleType::ShapeCircle,
        },
        char_size: Some(compose.char_size.0 as i32),
        char_shape_ids: Vec::new(), // Document 모델에는 개별 글자 속성이 없음
    };
    ir::paragraph::RunContent::Compose(ir_compose)
}

/// Dutmal을 IR RunContent로 변환
///
/// IR에서 Dutmal은 RunContent이므로 직접 변환합니다.
fn convert_dutmal_to_ir_run_content(dutmal: &crate::control::Dutmal) -> ir::paragraph::RunContent {
    let ir_dutmal = ir::paragraph::Dutmal {
        main_text: dutmal.main_text.clone(),
        sub_text: dutmal.sub_text.clone(),
        position_type: match dutmal.position {
            crate::control::DutmalPosition::Top => ir::paragraph::DutmalPosition::Top,
            crate::control::DutmalPosition::Bottom => ir::paragraph::DutmalPosition::Bottom,
        },
        size_ratio: Some(dutmal.size_ratio.0 as u32),
        option: None,
        style_id_ref: None,
        alignment: match dutmal.alignment {
            primitive::Alignment::Left => ir::paragraph::DutmalAlignment::Left,
            primitive::Alignment::Right => ir::paragraph::DutmalAlignment::Right,
            primitive::Alignment::Center => ir::paragraph::DutmalAlignment::Center,
            primitive::Alignment::Justify => ir::paragraph::DutmalAlignment::Justify,
            primitive::Alignment::Distribute | primitive::Alignment::Divide => {
                ir::paragraph::DutmalAlignment::Distribute
            }
        },
    };
    ir::paragraph::RunContent::Dutmal(ir_dutmal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_empty_document() {
        let ir_doc1 = ir::Document::default();
        let doc = Document::from(ir_doc1);
        let ir_doc2: ir::Document = doc.into();

        assert_eq!(ir_doc2.sections.len(), 0);
    }

    #[test]
    fn test_roundtrip_text_document() {
        let mut ir_doc1 = ir::Document::default();

        // IR에 섹션/문단/런 추가
        let mut section = ir::Section::default();
        let mut para = ir::Paragraph::default();
        let mut run = ir::paragraph::Run::default();
        run.contents
            .push(ir::paragraph::RunContent::Text(ir::paragraph::Text::new(
                "Hello, World!",
            )));
        para.runs.push(run);
        section.paragraphs.push(para);
        ir_doc1.sections.push(section);

        // 왕복 변환
        let doc = Document::from(ir_doc1);
        let ir_doc2: ir::Document = doc.into();

        // 검증
        assert_eq!(ir_doc2.sections.len(), 1);
        assert_eq!(ir_doc2.sections[0].paragraphs.len(), 1);
        assert_eq!(ir_doc2.sections[0].paragraphs[0].runs.len(), 1);

        if let ir::paragraph::RunContent::Text(text) =
            &ir_doc2.sections[0].paragraphs[0].runs[0].contents[0]
        {
            assert_eq!(text.text, "Hello, World!");
        } else {
            panic!("Expected Text content");
        }
    }

    #[test]
    fn test_roundtrip_table() {
        let mut ir_doc1 = ir::Document::default();

        // 표 생성 - rows와 cells를 수동으로 생성
        let mut table = ir::table::Table::new(2, 2);

        // 첫 번째 행
        let mut row1 = ir::table::TableRow::new();
        let mut cell1 = ir::table::TableCell::new(0, 0);
        cell1.paragraphs.push(ir::Paragraph::with_text("Cell 1"));
        let mut cell2 = ir::table::TableCell::new(0, 1);
        cell2.paragraphs.push(ir::Paragraph::with_text("Cell 2"));
        row1.cells.push(cell1);
        row1.cells.push(cell2);
        table.rows.push(row1);

        // 두 번째 행
        let mut row2 = ir::table::TableRow::new();
        let mut cell3 = ir::table::TableCell::new(1, 0);
        cell3.paragraphs.push(ir::Paragraph::with_text("Cell 3"));
        let mut cell4 = ir::table::TableCell::new(1, 1);
        cell4.paragraphs.push(ir::Paragraph::with_text("Cell 4"));
        row2.cells.push(cell3);
        row2.cells.push(cell4);
        table.rows.push(row2);

        let mut section = ir::Section::default();
        let mut para = ir::Paragraph::default();
        let mut run = ir::paragraph::Run::default();
        run.contents
            .push(ir::paragraph::RunContent::Control(Box::new(
                ir::control::Control::Table(Box::new(table)),
            )));
        para.runs.push(run);
        section.paragraphs.push(para);
        ir_doc1.sections.push(section);

        // 왕복 변환
        let doc = Document::from(ir_doc1);
        let ir_doc2: ir::Document = doc.into();

        // 검증
        if let ir::paragraph::RunContent::Control(ctrl) =
            &ir_doc2.sections[0].paragraphs[0].runs[0].contents[0]
        {
            if let ir::control::Control::Table(t) = ctrl.as_ref() {
                assert_eq!(t.row_count, 2);
                assert_eq!(t.column_count, 2);
                assert_eq!(t.rows[0].cells[0].paragraphs[0].to_plain_text(), "Cell 1");
            } else {
                panic!("Expected Table control");
            }
        } else {
            panic!("Expected Control content");
        }
    }

    #[test]
    fn test_roundtrip_picture() {
        let mut ir_doc1 = ir::Document::default();

        let picture = ir::picture::Picture {
            common: ir::control::ObjectCommon::default(),
            binary_id: ir::BinaryDataId::new("image1"),
            original_size: ir::Size::new(ir::HwpUnit::from_mm(100.0), ir::HwpUnit::from_mm(100.0)),
            crop: ir::picture::ImageCrop::default(),
            flip: primitive::ImageFlip::Horizontal,
            rotation: 45.0,
            effect: primitive::ImageEffect::Grayscale,
            brightness: 10,
            contrast: -5,
            alpha: 0.8,
            transparent_color: None,
            border: None,
            shadow: None,
            inside_margin: ir::Insets::ZERO,
        };

        let mut section = ir::Section::default();
        let mut para = ir::Paragraph::default();
        let mut run = ir::paragraph::Run::default();
        run.contents
            .push(ir::paragraph::RunContent::Control(Box::new(
                ir::control::Control::Picture(Box::new(picture)),
            )));
        para.runs.push(run);
        section.paragraphs.push(para);
        ir_doc1.sections.push(section);

        // 왕복 변환
        let doc = Document::from(ir_doc1);
        let ir_doc2: ir::Document = doc.into();

        // 검증
        if let ir::paragraph::RunContent::Control(ctrl) =
            &ir_doc2.sections[0].paragraphs[0].runs[0].contents[0]
        {
            if let ir::control::Control::Picture(p) = ctrl.as_ref() {
                assert_eq!(p.binary_id.value(), "image1");
                assert_eq!(p.flip, primitive::ImageFlip::Horizontal);
                assert_eq!(p.effect, primitive::ImageEffect::Grayscale);
                assert_eq!(p.brightness, 10);
                assert_eq!(p.contrast, -5);
            } else {
                panic!("Expected Picture control");
            }
        } else {
            panic!("Expected Control content");
        }
    }

    #[test]
    fn test_roundtrip_equation() {
        let mut ir_doc1 = ir::Document::default();

        let equation = ir::control::Equation {
            common: ir::control::ObjectCommon::default(),
            script: "x^2 + y^2 = z^2".to_string(),
            format: ir::control::EquationFormat::LaTeX,
            baseline_offset: ir::HwpUnit::from_pt(2.0),
            font_size: ir::HwpUnit::from_pt(12.0),
            color: Some(ir::Color::BLACK),
            line_mode: None,
            version: None,
            font_name: None,
            properties: None,
        };

        let mut section = ir::Section::default();
        let mut para = ir::Paragraph::default();
        let mut run = ir::paragraph::Run::default();
        run.contents
            .push(ir::paragraph::RunContent::Control(Box::new(
                ir::control::Control::Equation(Box::new(equation)),
            )));
        para.runs.push(run);
        section.paragraphs.push(para);
        ir_doc1.sections.push(section);

        // 왕복 변환
        let doc = Document::from(ir_doc1);
        let ir_doc2: ir::Document = doc.into();

        // 검증
        if let ir::paragraph::RunContent::Control(ctrl) =
            &ir_doc2.sections[0].paragraphs[0].runs[0].contents[0]
        {
            if let ir::control::Control::Equation(e) = ctrl.as_ref() {
                assert_eq!(e.script, "x^2 + y^2 = z^2");
                assert_eq!(e.format, ir::control::EquationFormat::LaTeX);
            } else {
                panic!("Expected Equation control");
            }
        } else {
            panic!("Expected Control content");
        }
    }

    #[test]
    fn test_roundtrip_hyperlink() {
        let mut ir_doc1 = ir::Document::default();

        let hyperlink = ir::control::Hyperlink {
            target: ir::control::HyperlinkTarget::Url("https://example.com".to_string()),
            tooltip: Some("Example".to_string()),
            display_text: Some("Click here".to_string()),
        };

        let mut section = ir::Section::default();
        let mut para = ir::Paragraph::default();
        let mut run = ir::paragraph::Run::default();
        run.contents
            .push(ir::paragraph::RunContent::Control(Box::new(
                ir::control::Control::Hyperlink(Box::new(hyperlink)),
            )));
        para.runs.push(run);
        section.paragraphs.push(para);
        ir_doc1.sections.push(section);

        // 왕복 변환
        let doc = Document::from(ir_doc1);
        let ir_doc2: ir::Document = doc.into();

        // 검증
        if let ir::paragraph::RunContent::Control(ctrl) =
            &ir_doc2.sections[0].paragraphs[0].runs[0].contents[0]
        {
            if let ir::control::Control::Hyperlink(h) = ctrl.as_ref() {
                if let ir::control::HyperlinkTarget::Url(url) = &h.target {
                    assert_eq!(url, "https://example.com");
                } else {
                    panic!("Expected URL target");
                }
                assert_eq!(h.tooltip.as_deref(), Some("Example"));
                assert_eq!(h.display_text.as_deref(), Some("Click here"));
            } else {
                panic!("Expected Hyperlink control");
            }
        } else {
            panic!("Expected Control content");
        }
    }

    #[test]
    fn test_roundtrip_footnote() {
        let mut ir_doc1 = ir::Document::default();

        let note = ir::control::Note {
            number: 1,
            number_format: primitive::NumberFormat::Digit,
            number_position: primitive::NoteNumberPosition::Superscript,
            paragraphs: vec![ir::Paragraph::with_text("Footnote content")],
            instance_id: Some(100),
        };

        let mut section = ir::Section::default();
        let mut para = ir::Paragraph::default();
        let mut run = ir::paragraph::Run::default();
        run.contents
            .push(ir::paragraph::RunContent::Control(Box::new(
                ir::control::Control::Footnote(Box::new(note)),
            )));
        para.runs.push(run);
        section.paragraphs.push(para);
        ir_doc1.sections.push(section);

        // 왕복 변환
        let doc = Document::from(ir_doc1);
        let ir_doc2: ir::Document = doc.into();

        // 검증
        if let ir::paragraph::RunContent::Control(ctrl) =
            &ir_doc2.sections[0].paragraphs[0].runs[0].contents[0]
        {
            if let ir::control::Control::Footnote(n) = ctrl.as_ref() {
                assert_eq!(n.number, 1);
                assert_eq!(n.paragraphs[0].to_plain_text(), "Footnote content");
            } else {
                panic!("Expected Footnote control");
            }
        } else {
            panic!("Expected Control content");
        }
    }

    #[test]
    fn test_roundtrip_auto_number() {
        let mut ir_doc1 = ir::Document::default();

        let auto_number = ir::control::AutoNumber {
            number_type: ir::control::AutoNumberType::Picture,
            number_format: primitive::NumberFormat::CircledDigit,
            auto_number_format: None,
        };

        let mut section = ir::Section::default();
        let mut para = ir::Paragraph::default();
        let mut run = ir::paragraph::Run::default();
        run.contents
            .push(ir::paragraph::RunContent::Control(Box::new(
                ir::control::Control::AutoNumber(Box::new(auto_number)),
            )));
        para.runs.push(run);
        section.paragraphs.push(para);
        ir_doc1.sections.push(section);

        // 왕복 변환
        let doc = Document::from(ir_doc1);
        let ir_doc2: ir::Document = doc.into();

        // 검증
        if let ir::paragraph::RunContent::Control(ctrl) =
            &ir_doc2.sections[0].paragraphs[0].runs[0].contents[0]
        {
            if let ir::control::Control::AutoNumber(an) = ctrl.as_ref() {
                assert_eq!(an.number_type, ir::control::AutoNumberType::Picture);
                assert_eq!(an.number_format, primitive::NumberFormat::CircledDigit);
            } else {
                panic!("Expected AutoNumber control");
            }
        } else {
            panic!("Expected Control content");
        }
    }

    #[test]
    fn test_roundtrip_textbox() {
        let mut ir_doc1 = ir::Document::default();

        let textbox = ir::control::TextBox {
            common: ir::control::ObjectCommon::default(),
            paragraphs: vec![ir::Paragraph::with_text("TextBox content")],
            text_direction: primitive::TextDirection::Horizontal,
            vertical_alignment: primitive::VerticalAlignment::Middle,
            padding: ir::Insets::all(ir::HwpUnit::from_mm(5.0)),
            editable: true,
            name: Some("mybox".to_string()),
            last_width: None,
            line_wrap: primitive::LineWrap::Break,
            link_list_id_reference: None,
            link_list_next_id_reference: None,
            text_width: None,
            text_height: None,
            has_text_reference: false,
            has_number_reference: false,
        };

        let mut section = ir::Section::default();
        let mut para = ir::Paragraph::default();
        let mut run = ir::paragraph::Run::default();
        run.contents
            .push(ir::paragraph::RunContent::Control(Box::new(
                ir::control::Control::TextBox(Box::new(textbox)),
            )));
        para.runs.push(run);
        section.paragraphs.push(para);
        ir_doc1.sections.push(section);

        // 왕복 변환
        let doc = Document::from(ir_doc1);
        let ir_doc2: ir::Document = doc.into();

        // 검증
        if let ir::paragraph::RunContent::Control(ctrl) =
            &ir_doc2.sections[0].paragraphs[0].runs[0].contents[0]
        {
            if let ir::control::Control::TextBox(tb) = ctrl.as_ref() {
                assert_eq!(tb.paragraphs[0].to_plain_text(), "TextBox content");
                assert_eq!(tb.editable, true);
            } else {
                panic!("Expected TextBox control");
            }
        } else {
            panic!("Expected Control content");
        }
    }
}
