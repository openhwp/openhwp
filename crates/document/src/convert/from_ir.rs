//! IR → Document 변환
//!
//! IR 문서를 Document 모델로 변환합니다.

use crate::control::Control;
use crate::document::Document;
use crate::paragraph::{BreakType, Paragraph, RangeTag, RangeTagType};
use crate::run::Run;
use crate::run_content::RunContent;
use crate::section::Section;
use crate::table::{Table, TableCell, TableRow};

impl From<ir::Document> for Document {
    fn from(ir_doc: ir::Document) -> Self {
        let mut doc = Document::new();

        // 메타데이터 복사
        doc.metadata = ir_doc.metadata;

        // 스타일 복사
        doc.styles = ir_doc.styles;

        // 바이너리 데이터 복사
        doc.binary_data = ir_doc.binary_data;

        // 확장 데이터 복사
        doc.extensions = ir_doc.extensions;

        // 섹션 변환
        for ir_section in ir_doc.sections {
            let section = convert_section(&ir_section, &mut doc);
            doc.add_section(section);
        }

        doc
    }
}

/// IR Section을 Document Section으로 변환
fn convert_section(ir_section: &ir::Section, doc: &mut Document) -> Section {
    let mut section = Section::default();

    // 문단 변환
    for ir_para in &ir_section.paragraphs {
        let para_id = convert_paragraph(ir_para, doc);
        section.paragraphs.push(para_id);
    }

    section
}

/// IR Paragraph 변환
fn convert_paragraph(ir_para: &ir::Paragraph, doc: &mut Document) -> crate::ParagraphId {
    let mut para = Paragraph {
        para_shape_id: ir_para.para_shape_id,
        style_id: ir_para.style_id,
        runs: Vec::new(),
        break_type: match ir_para.break_type {
            primitive::BreakType::None => BreakType::None,
            primitive::BreakType::Page => BreakType::Page,
            primitive::BreakType::Column => BreakType::Column,
            primitive::BreakType::Section => BreakType::Section,
        },
        instance_id: ir_para.instance_id,
        range_tags: ir_para
            .range_tags
            .iter()
            .map(convert_range_tag)
            .collect(),
    };

    // 런 변환
    for ir_run in &ir_para.runs {
        let run_id = convert_run(ir_run, doc);
        para.runs.push(run_id);
    }

    doc.arena.insert_paragraph(para)
}

/// IR RangeTag 변환
fn convert_range_tag(ir_tag: &ir::paragraph::RangeTag) -> RangeTag {
    RangeTag {
        start: ir_tag.start,
        end: ir_tag.end,
        tag_type: match ir_tag.tag_type {
            ir::paragraph::RangeTagType::Bookmark => RangeTagType::Bookmark,
            ir::paragraph::RangeTagType::Hyperlink => RangeTagType::Hyperlink,
            ir::paragraph::RangeTagType::TrackChangeInsert => RangeTagType::TrackChangeInsert,
            ir::paragraph::RangeTagType::TrackChangeDelete => RangeTagType::TrackChangeDelete,
            ir::paragraph::RangeTagType::Highlight => RangeTagType::Highlight,
            ir::paragraph::RangeTagType::Other(v) => RangeTagType::Other(v),
        },
        data: ir_tag.data.clone(),
        track_change_info: None,
    }
}

/// IR Run 변환
fn convert_run(ir_run: &ir::paragraph::Run, doc: &mut Document) -> crate::RunId {
    let mut run = Run {
        char_shape_id: ir_run.char_shape_id,
        contents: Vec::new(),
    };

    // 런 내용 변환
    for ir_content in &ir_run.contents {
        let content = convert_run_content(ir_content, doc);
        run.contents.push(content);
    }

    doc.arena.insert_run(run)
}

/// IR RunContent 변환
fn convert_run_content(ir_content: &ir::paragraph::RunContent, doc: &mut Document) -> RunContent {
    match ir_content {
        ir::paragraph::RunContent::Text(t) => RunContent::Text(t.text.clone()),
        ir::paragraph::RunContent::Tab(_) => RunContent::Tab,
        ir::paragraph::RunContent::LineBreak => RunContent::LineBreak,
        ir::paragraph::RunContent::Hyphen => RunContent::Hyphen,
        ir::paragraph::RunContent::NonBreakingSpace => RunContent::NonBreakingSpace,
        ir::paragraph::RunContent::FixedWidthSpace => RunContent::FixedWidthSpace,
        ir::paragraph::RunContent::Control(ir_ctrl) => {
            let ctrl = convert_control(ir_ctrl, doc);
            let ctrl_id = doc.arena.insert_control(ctrl);
            RunContent::Control(ctrl_id)
        }
        ir::paragraph::RunContent::FieldStart(_) => RunContent::FieldEnd, // TODO: proper conversion
        ir::paragraph::RunContent::FieldEnd(_) => RunContent::FieldEnd,
        ir::paragraph::RunContent::BookmarkStart(bs) => {
            RunContent::BookmarkStart(crate::run_content::BookmarkStart {
                id: bs.id,
                name: bs.name.clone(),
            })
        }
        ir::paragraph::RunContent::BookmarkEnd(_) => RunContent::BookmarkEnd,
        ir::paragraph::RunContent::Compose(c) => {
            let ctrl = Control::Compose(convert_compose(c));
            let ctrl_id = doc.arena.insert_control(ctrl);
            RunContent::Control(ctrl_id)
        }
        ir::paragraph::RunContent::Dutmal(d) => {
            let ctrl = Control::Dutmal(convert_dutmal(d));
            let ctrl_id = doc.arena.insert_control(ctrl);
            RunContent::Control(ctrl_id)
        }
    }
}

/// IR Control 변환
fn convert_control(ir_ctrl: &ir::control::Control, doc: &mut Document) -> Control {
    match ir_ctrl {
        ir::control::Control::Table(t) => Control::Table(convert_table(t, doc)),
        ir::control::Control::Picture(p) => Control::Picture(convert_picture(p, doc)),
        ir::control::Control::Shape(s) => Control::Shape(convert_shape(s, doc)),
        ir::control::Control::Equation(e) => Control::Equation(convert_equation(e, doc)),
        ir::control::Control::Ole(o) => Control::Ole(convert_ole(o, doc)),
        ir::control::Control::TextBox(tb) => Control::TextBox(convert_textbox(tb, doc)),
        ir::control::Control::Header(_) => Control::Unknown(Vec::new()),
        ir::control::Control::Footer(_) => Control::Unknown(Vec::new()),
        ir::control::Control::Footnote(n) => Control::Footnote(convert_note(n, doc)),
        ir::control::Control::Endnote(n) => Control::Endnote(convert_note(n, doc)),
        ir::control::Control::Hyperlink(h) => Control::Hyperlink(convert_hyperlink(h)),
        ir::control::Control::Bookmark(b) => Control::Bookmark(crate::control::Bookmark {
            name: b.name.clone(),
        }),
        ir::control::Control::IndexMark(im) => Control::IndexMark(convert_index_mark(im)),
        ir::control::Control::AutoNumber(an) => Control::AutoNumber(convert_auto_number(an)),
        ir::control::Control::NewNumber(nn) => Control::NewNumber(convert_new_number(nn)),
        ir::control::Control::HiddenComment(hc) => Control::HiddenComment(convert_hidden_comment(hc, doc)),
        ir::control::Control::Chart(c) => Control::Chart(convert_chart(c)),
        ir::control::Control::Video(v) => Control::Video(convert_video(v)),
        ir::control::Control::FormObject(fo) => Control::FormObject(convert_form_object(fo)),
        ir::control::Control::TextArt(ta) => Control::TextArt(convert_text_art(ta)),
        ir::control::Control::Memo(_) => Control::Unknown(Vec::new()),       // TODO
        ir::control::Control::Unknown(data) => Control::Unknown(data.data.clone()),
    }
}

/// IR Video 변환
fn convert_video(ir_video: &ir::control::Video) -> crate::control::Video {
    crate::control::Video {
        common: convert_object_common(&ir_video.common),
        video_type: convert_video_type(ir_video.video_type),
        video_id: ir_video.video_id.clone(),
        source_url: ir_video.source_url.clone(),
        preview_image_id: ir_video.preview_image_id.clone(),
    }
}

/// VideoType 변환
fn convert_video_type(ir_type: ir::control::VideoType) -> crate::control::VideoType {
    match ir_type {
        ir::control::VideoType::Embedded => crate::control::VideoType::Embedded,
        ir::control::VideoType::Linked => crate::control::VideoType::Linked,
        // IR의 YouTube와 Web을 Document의 Web으로 매핑
        ir::control::VideoType::YouTube | ir::control::VideoType::Web => {
            crate::control::VideoType::Web
        }
    }
}

/// IR Chart 변환
fn convert_chart(ir_chart: &ir::control::Chart) -> crate::control::Chart {
    crate::control::Chart {
        common: convert_object_common(&ir_chart.common),
        // IR의 chart_id는 String이므로 BinaryDataId로 변환
        chart_id: ir::BinaryDataId::new(ir_chart.chart_id.clone()),
        chart_type: convert_chart_type(ir_chart.chart_type),
    }
}

/// ChartType 변환
fn convert_chart_type(ir_type: ir::control::ChartType) -> crate::control::ChartType {
    match ir_type {
        ir::control::ChartType::Bar => crate::control::ChartType::Bar,
        ir::control::ChartType::Column => crate::control::ChartType::Bar, // 열 차트를 막대로 매핑
        ir::control::ChartType::Line => crate::control::ChartType::Line,
        ir::control::ChartType::Pie => crate::control::ChartType::Pie,
        ir::control::ChartType::Area => crate::control::ChartType::Area,
        ir::control::ChartType::Scatter => crate::control::ChartType::Scatter,
        ir::control::ChartType::Bubble => crate::control::ChartType::Scatter, // 버블을 분산형으로 매핑
        ir::control::ChartType::Radar => crate::control::ChartType::Radar,
        ir::control::ChartType::Stock => crate::control::ChartType::Line, // 주식형을 선형으로 매핑
        ir::control::ChartType::Surface => crate::control::ChartType::Area, // 표면형을 영역형으로 매핑
        ir::control::ChartType::Doughnut => crate::control::ChartType::Pie, // 도넛형을 원형으로 매핑
    }
}

/// IR FormObject 변환
fn convert_form_object(ir_form: &ir::control::FormObject) -> crate::control::FormObject {
    crate::control::FormObject {
        common: convert_object_common(&ir_form.common),
        form_type: convert_form_object_type(ir_form),
        name: ir_form.name.clone().unwrap_or_default(),
        value: ir_form.value.clone(),
    }
}

/// FormObjectType 변환 (IR의 여러 필드를 기반으로 Document의 복잡한 enum으로 변환)
fn convert_form_object_type(ir_form: &ir::control::FormObject) -> crate::control::FormObjectType {
    match ir_form.form_type {
        ir::control::FormObjectType::Button => crate::control::FormObjectType::Button,
        ir::control::FormObjectType::CheckBox => {
            let checked = matches!(
                ir_form.button_value,
                Some(ir::control::ButtonValue::Checked)
            );
            crate::control::FormObjectType::CheckBox { checked }
        }
        ir::control::FormObjectType::RadioButton => {
            let group_name = ir_form.radio_group_name.clone().unwrap_or_default();
            let checked = matches!(
                ir_form.button_value,
                Some(ir::control::ButtonValue::Checked)
            );
            crate::control::FormObjectType::RadioButton { group_name, checked }
        }
        ir::control::FormObjectType::ComboBox => {
            let items: Vec<String> = ir_form
                .items
                .iter()
                .map(|item| item.display_text.clone().unwrap_or_default())
                .collect();
            // selected_value와 매칭되는 항목의 인덱스를 찾음
            let selected = if let Some(ref selected_value) = ir_form.selected_value {
                ir_form
                    .items
                    .iter()
                    .position(|item| {
                        item.value.as_ref() == Some(selected_value)
                            || item.display_text.as_ref() == Some(selected_value)
                    })
            } else {
                None
            };
            crate::control::FormObjectType::ComboBox { items, selected }
        }
        ir::control::FormObjectType::ListBox => {
            let items: Vec<String> = ir_form
                .items
                .iter()
                .map(|item| item.display_text.clone().unwrap_or_default())
                .collect();
            // selected_value와 매칭되는 항목의 인덱스를 찾음
            let selected = if let Some(ref selected_value) = ir_form.selected_value {
                ir_form
                    .items
                    .iter()
                    .position(|item| {
                        item.value.as_ref() == Some(selected_value)
                            || item.display_text.as_ref() == Some(selected_value)
                    })
            } else {
                None
            };
            crate::control::FormObjectType::ListBox { items, selected }
        }
        ir::control::FormObjectType::Edit => {
            let multiline = ir_form.multiline;
            let password = ir_form.password_char.is_some();
            crate::control::FormObjectType::Edit { multiline, password }
        }
        ir::control::FormObjectType::ScrollBar => {
            let min = ir_form.min.unwrap_or(0);
            let max = ir_form.max.unwrap_or(100);
            let value = ir_form.scroll_value.unwrap_or(min);
            crate::control::FormObjectType::ScrollBar { min, max, value }
        }
        // Signature는 Button으로 대체
        ir::control::FormObjectType::Signature => crate::control::FormObjectType::Button,
    }
}

/// IR TextArt 변환
fn convert_text_art(ir_text_art: &ir::control::TextArt) -> crate::control::TextArt {
    crate::control::TextArt {
        common: convert_object_common(&ir_text_art.common),
        text: ir_text_art.text.clone(),
        font_name: ir_text_art.font_name.clone().unwrap_or_else(|| "굴림".to_string()),
        font_style: convert_font_style(ir_text_art.font_style),
        shape: convert_text_art_shape(ir_text_art.shape),
        // IR은 u32 (50-500), Document는 Percent로 변환
        line_spacing: ir::Percent::new(ir_text_art.line_spacing as f64),
        char_spacing: ir::Percent::new(ir_text_art.char_spacing as f64),
        alignment: convert_text_art_alignment(ir_text_art.alignment),
        line: Some(convert_line_style(&ir_text_art.line)),
        fill: Some(convert_fill(&ir_text_art.fill)),
        shadow: ir_text_art.shadow.as_ref().map(convert_shadow_from_shape),
    }
}

/// FontStyle 변환
fn convert_font_style(ir_style: ir::control::TextArtFontStyle) -> crate::control::FontStyle {
    match ir_style {
        ir::control::TextArtFontStyle::Regular => crate::control::FontStyle::Regular,
        ir::control::TextArtFontStyle::Bold => crate::control::FontStyle::Bold,
        ir::control::TextArtFontStyle::Italic => crate::control::FontStyle::Italic,
        ir::control::TextArtFontStyle::BoldItalic => crate::control::FontStyle::BoldItalic,
    }
}

/// TextArtShape 변환
fn convert_text_art_shape(ir_shape: ir::control::TextArtShapeType) -> crate::control::TextArtShape {
    match ir_shape {
        ir::control::TextArtShapeType::Rectangle => crate::control::TextArtShape::Rectangle,
        ir::control::TextArtShapeType::Circle => crate::control::TextArtShape::Circle,
        // IR의 ArchUp/ArchDown을 Document의 Arch로 매핑
        ir::control::TextArtShapeType::ArchUp => crate::control::TextArtShape::Arch,
        ir::control::TextArtShapeType::ArchDown => crate::control::TextArtShape::Arch,
        ir::control::TextArtShapeType::Wave => crate::control::TextArtShape::Wave,
        // Document에 없는 나머지 타입은 Rectangle로 매핑
        _ => crate::control::TextArtShape::Rectangle,
    }
}

/// TextArtAlignment 변환
fn convert_text_art_alignment(ir_alignment: ir::control::TextArtAlignment) -> primitive::Alignment {
    match ir_alignment {
        ir::control::TextArtAlignment::Left => primitive::Alignment::Left,
        ir::control::TextArtAlignment::Center => primitive::Alignment::Center,
        ir::control::TextArtAlignment::Right => primitive::Alignment::Right,
        ir::control::TextArtAlignment::Full => primitive::Alignment::Justify,
    }
}

/// IR Hyperlink 변환
fn convert_hyperlink(ir_hyperlink: &ir::control::Hyperlink) -> crate::control::Hyperlink {
    crate::control::Hyperlink {
        target: convert_hyperlink_target(&ir_hyperlink.target),
        tooltip: ir_hyperlink.tooltip.clone(),
        display_text: ir_hyperlink.display_text.clone(),
    }
}

/// HyperlinkTarget 변환
fn convert_hyperlink_target(ir_target: &ir::control::HyperlinkTarget) -> crate::control::HyperlinkTarget {
    match ir_target {
        ir::control::HyperlinkTarget::Url(url) => crate::control::HyperlinkTarget::Url(url.clone()),
        ir::control::HyperlinkTarget::Email(email) => crate::control::HyperlinkTarget::Email(email.clone()),
        ir::control::HyperlinkTarget::File(file) => crate::control::HyperlinkTarget::File(file.clone()),
        ir::control::HyperlinkTarget::Bookmark(bookmark) => crate::control::HyperlinkTarget::Bookmark(bookmark.clone()),
    }
}

/// IR AutoNumber 변환
fn convert_auto_number(ir_auto_number: &ir::control::AutoNumber) -> crate::control::AutoNumber {
    crate::control::AutoNumber {
        number_type: convert_auto_number_type(ir_auto_number.number_type),
        number_format: ir_auto_number.number_format,
    }
}

/// AutoNumberType 변환
fn convert_auto_number_type(ir_type: ir::control::AutoNumberType) -> crate::control::AutoNumberType {
    match ir_type {
        ir::control::AutoNumberType::Page => crate::control::AutoNumberType::Page,
        ir::control::AutoNumberType::Footnote => crate::control::AutoNumberType::Footnote,
        ir::control::AutoNumberType::Endnote => crate::control::AutoNumberType::Endnote,
        ir::control::AutoNumberType::Picture => crate::control::AutoNumberType::Picture,
        ir::control::AutoNumberType::Table => crate::control::AutoNumberType::Table,
        ir::control::AutoNumberType::Equation => crate::control::AutoNumberType::Equation,
        // IR의 TotalPages를 Document의 Page로 매핑 (Document에는 TotalPages가 없음)
        ir::control::AutoNumberType::TotalPages => crate::control::AutoNumberType::Page,
    }
}

/// IR NewNumber 변환
fn convert_new_number(ir_new_number: &ir::control::NewNumber) -> crate::control::NewNumber {
    crate::control::NewNumber {
        number_type: convert_auto_number_type(ir_new_number.number_type),
        number: ir_new_number.number,
    }
}

/// IR IndexMark 변환
fn convert_index_mark(ir_index_mark: &ir::control::IndexMark) -> crate::control::IndexMark {
    crate::control::IndexMark {
        first_key: ir_index_mark.first_key.clone(),
        // IR의 second_key는 String이지만, Document는 Option<String>
        // 빈 문자열인 경우 None으로 변환
        second_key: if ir_index_mark.second_key.is_empty() {
            None
        } else {
            Some(ir_index_mark.second_key.clone())
        },
    }
}

/// IR ObjectCommon 변환
fn convert_object_common(ir_common: &ir::control::ObjectCommon) -> crate::control::ObjectCommon {
    crate::control::ObjectCommon {
        id: ir_common.id.as_ref().map(|id| id.to_string()),
        position: ir_common.position,
        size: ir_common.size,
        z_order: ir_common.z_order,
        text_wrap: convert_text_wrap(&ir_common.text_wrap),
        caption: ir_common.caption.as_ref().map(convert_caption),
    }
}

/// IR TextWrap 변환
fn convert_text_wrap(ir_wrap: &ir::control::TextWrap) -> crate::control::TextWrap {
    crate::control::TextWrap {
        wrap_type: convert_text_wrap_type(ir_wrap.wrap_type),
        wrap_side: convert_text_wrap_side(ir_wrap.wrap_side),
        margin: ir::Insets::all(ir_wrap.margin),
        vertical_rel: convert_vertical_rel(ir_wrap.vertical_rel),
        horizontal_rel: convert_horizontal_rel(ir_wrap.horizontal_rel),
        treat_as_char: ir_wrap.treat_as_char,
        flow_with_text: ir_wrap.flow_with_text,
        allow_overlap: ir_wrap.allow_overlap,
    }
}

/// TextWrapType 변환 (now same type via re-export)
fn convert_text_wrap_type(ir_type: primitive::TextWrapType) -> crate::control::TextWrapType {
    ir_type
}

/// TextWrapSide 변환 (now same type via re-export)
fn convert_text_wrap_side(ir_side: primitive::TextWrapSide) -> crate::control::TextWrapSide {
    ir_side
}

/// VerticalRelativeTo 변환 (now same type via re-export)
fn convert_vertical_rel(ir_rel: primitive::VerticalRelativeTo) -> crate::control::VerticalRelativeTo {
    ir_rel
}

/// HorizontalRelativeTo 변환 (now same type via re-export)
fn convert_horizontal_rel(ir_rel: primitive::HorizontalRelativeTo) -> crate::control::HorizontalRelativeTo {
    ir_rel
}

/// Caption 변환
fn convert_caption(ir_caption: &ir::control::Caption) -> crate::control::Caption {
    crate::control::Caption {
        position: convert_caption_position(ir_caption.position),
        width: ir_caption.width,
        gap: ir_caption.gap,
        paragraphs: Vec::new(), // TODO: 캡션 문단 변환 필요
    }
}

/// CaptionPosition 변환 (ir still has its own CaptionPosition, so we need mapping)
fn convert_caption_position(ir_pos: ir::control::CaptionPosition) -> crate::control::CaptionPosition {
    match ir_pos {
        ir::control::CaptionPosition::Left => crate::control::CaptionPosition::Left,
        ir::control::CaptionPosition::Right => crate::control::CaptionPosition::Right,
        ir::control::CaptionPosition::Top => crate::control::CaptionPosition::Top,
        ir::control::CaptionPosition::Bottom => crate::control::CaptionPosition::Bottom,
    }
}

/// IR Picture 변환
fn convert_picture(ir_picture: &ir::picture::Picture, _doc: &mut Document) -> crate::control::Picture {
    crate::control::Picture {
        common: convert_object_common(&ir_picture.common),
        binary_id: ir_picture.binary_id.clone(),
        original_size: ir_picture.original_size,
        crop: ir::Insets {
            left: ir_picture.crop.left,
            right: ir_picture.crop.right,
            top: ir_picture.crop.top,
            bottom: ir_picture.crop.bottom,
        },
        flip: convert_image_flip(ir_picture.flip),
        rotation: ir_picture.rotation,
        effect: convert_image_effect(ir_picture.effect),
        brightness: ir_picture.brightness,
        contrast: ir_picture.contrast,
        alpha: ((ir_picture.alpha * 100.0) as u8).min(100),
        border: ir_picture.border.as_ref().map(convert_line_style_from_picture_border),
        shadow: ir_picture.shadow.as_ref().map(convert_shadow_from_picture),
    }
}

/// ImageFlip 변환
fn convert_image_flip(ir_flip: primitive::ImageFlip) -> crate::control::ImageFlip {
    match ir_flip {
        primitive::ImageFlip::None => crate::control::ImageFlip::None,
        primitive::ImageFlip::Horizontal => crate::control::ImageFlip::Horizontal,
        primitive::ImageFlip::Vertical => crate::control::ImageFlip::Vertical,
        primitive::ImageFlip::Both => crate::control::ImageFlip::Both,
    }
}

/// ImageEffect 변환
fn convert_image_effect(ir_effect: primitive::ImageEffect) -> crate::control::ImageEffect {
    match ir_effect {
        primitive::ImageEffect::Original => crate::control::ImageEffect::Original,
        primitive::ImageEffect::Grayscale => crate::control::ImageEffect::Grayscale,
        primitive::ImageEffect::BlackWhite => crate::control::ImageEffect::BlackWhite,
        primitive::ImageEffect::Pattern => crate::control::ImageEffect::Pattern,
    }
}

/// PictureBorder를 LineStyle로 변환
fn convert_line_style_from_picture_border(ir_border: &ir::picture::PictureBorder) -> crate::control::LineStyle {
    crate::control::LineStyle {
        line_type: ir_border.line_type,
        width: ir_border.width,
        color: ir_border.color,
        cap: crate::control::LineCap::Flat,
        outline_style: None,
        alpha: None,
    }
}

/// PictureShadow를 Shadow로 변환
fn convert_shadow_from_picture(ir_shadow: &ir::picture::PictureShadow) -> crate::control::Shadow {
    crate::control::Shadow {
        shadow_type: crate::control::ShadowType::Drop,
        offset_x: ir_shadow.offset_x,
        offset_y: ir_shadow.offset_y,
        color: ir_shadow.color,
        alpha: ((ir_shadow.alpha * 100.0) as u8).min(100),
        blur: None,
        direction: None,
        distance: None,
    }
}

/// IR Shape 변환
fn convert_shape(ir_shape: &ir::shape::Shape, doc: &mut Document) -> crate::control::Shape {
    crate::control::Shape {
        common: convert_object_common(&ir_shape.common),
        shape_type: convert_shape_type(&ir_shape.shape_type),
        line: Some(convert_line_style(&ir_shape.line)),
        fill: Some(convert_fill(&ir_shape.fill)),
        shadow: ir_shape.shadow.as_ref().map(convert_shadow_from_shape),
        rotation: ir_shape.rotation,
        text: ir_shape.text.as_ref().map(|t| convert_shape_text(t, doc)),
    }
}

/// ShapeType 변환
fn convert_shape_type(ir_type: &ir::shape::ShapeType) -> crate::control::ShapeType {
    match ir_type {
        ir::shape::ShapeType::Line(line) => crate::control::ShapeType::Line {
            start: line.start,
            end: line.end,
            start_arrow: if line.start_arrow.arrow_type != primitive::ArrowType::None {
                Some(convert_arrow(&line.start_arrow))
            } else {
                None
            },
            end_arrow: if line.end_arrow.arrow_type != primitive::ArrowType::None {
                Some(convert_arrow(&line.end_arrow))
            } else {
                None
            },
        },
        ir::shape::ShapeType::Rectangle(rect) => crate::control::ShapeType::Rectangle {
            corner_radius: rect.corner_radius,
        },
        ir::shape::ShapeType::Ellipse(ellipse) => crate::control::ShapeType::Ellipse {
            arc_type: convert_arc_type(ellipse.arc_type),
            start_angle: ellipse.start_angle,
            end_angle: ellipse.end_angle,
        },
        ir::shape::ShapeType::Arc(arc) => crate::control::ShapeType::Arc {
            arc_type: convert_arc_type(arc.arc_type),
            start_angle: arc.start_angle,
            end_angle: arc.end_angle,
        },
        ir::shape::ShapeType::Polygon(polygon) => crate::control::ShapeType::Polygon {
            points: polygon.points.clone(),
        },
        ir::shape::ShapeType::Curve(curve) => crate::control::ShapeType::Curve {
            points: curve.points.iter().map(convert_curve_point).collect(),
            closed: curve.closed,
        },
        ir::shape::ShapeType::Connector(connector) => crate::control::ShapeType::Connector {
            connector_type: convert_connector_type(connector.connector_type),
            points: connector.control_points.iter().map(|cp| cp.point).collect(),
            start_arrow: Some(convert_arrow(&connector.start_arrow)),
            end_arrow: Some(convert_arrow(&connector.end_arrow)),
        },
        ir::shape::ShapeType::Group(shapes) => crate::control::ShapeType::Group {
            children: shapes.iter().map(|s| convert_shape(s, &mut Document::new())).collect(),
        },
    }
}

/// Arrow 변환
fn convert_arrow(ir_arrow: &ir::shape::Arrow) -> crate::control::Arrow {
    crate::control::Arrow {
        arrow_type: convert_arrow_type(ir_arrow.arrow_type),
        size: convert_arrow_size(ir_arrow.size),
        filled: ir_arrow.filled,
    }
}

/// ArrowType 변환
fn convert_arrow_type(ir_type: primitive::ArrowType) -> crate::control::ArrowType {
    match ir_type {
        primitive::ArrowType::None => crate::control::ArrowType::None,
        primitive::ArrowType::Arrow => crate::control::ArrowType::Normal,
        primitive::ArrowType::ArrowOpen => crate::control::ArrowType::Open,
        primitive::ArrowType::Stealth => crate::control::ArrowType::Stealth,
        primitive::ArrowType::Diamond => crate::control::ArrowType::Diamond,
        primitive::ArrowType::Circle => crate::control::ArrowType::Circle,
        _ => crate::control::ArrowType::Normal,
    }
}

/// ArrowSize 변환
fn convert_arrow_size(ir_size: primitive::ArrowSize) -> crate::control::ArrowSize {
    match ir_size {
        primitive::ArrowSize::Small => crate::control::ArrowSize::Small,
        primitive::ArrowSize::Medium => crate::control::ArrowSize::Medium,
        primitive::ArrowSize::Large => crate::control::ArrowSize::Large,
    }
}

/// ArcType 변환
fn convert_arc_type(ir_type: ir::shape::ArcType) -> crate::control::ArcType {
    match ir_type {
        ir::shape::ArcType::Full => crate::control::ArcType::Arc,
        ir::shape::ArcType::Arc => crate::control::ArcType::Arc,
        ir::shape::ArcType::Pie => crate::control::ArcType::Pie,
        ir::shape::ArcType::Chord => crate::control::ArcType::Chord,
    }
}

/// CurvePoint 변환
fn convert_curve_point(ir_point: &ir::shape::CurvePoint) -> crate::control::CurvePoint {
    crate::control::CurvePoint {
        point: ir_point.point,
        point_type: convert_curve_point_type(ir_point.point_type),
    }
}

/// CurvePointType 변환
fn convert_curve_point_type(ir_type: ir::shape::CurvePointType) -> crate::control::CurvePointType {
    match ir_type {
        ir::shape::CurvePointType::Normal => crate::control::CurvePointType::Normal,
        ir::shape::CurvePointType::Control1 => crate::control::CurvePointType::Control1,
        ir::shape::CurvePointType::Control2 => crate::control::CurvePointType::Control2,
    }
}

/// ConnectorType 변환
fn convert_connector_type(ir_type: ir::shape::ConnectorType) -> crate::control::ConnectorType {
    match ir_type {
        ir::shape::ConnectorType::Straight => crate::control::ConnectorType::Straight,
        ir::shape::ConnectorType::Elbow => crate::control::ConnectorType::Elbow,
        ir::shape::ConnectorType::Curved => crate::control::ConnectorType::Curved,
        // HWPX 전용 타입들은 Elbow로 매핑
        ir::shape::ConnectorType::VerticalHorizontal
        | ir::shape::ConnectorType::HorizontalVertical => crate::control::ConnectorType::Elbow,
    }
}

/// LineStyle 변환
fn convert_line_style(ir_line: &ir::shape::LineStyle) -> crate::control::LineStyle {
    crate::control::LineStyle {
        line_type: ir_line.line_type,
        width: ir_line.width,
        color: ir_line.color,
        cap: convert_line_cap(ir_line.cap),
        outline_style: Some(convert_outline_style(ir_line.outline_style)),
        alpha: ir_line.alpha.map(|a| a as f64),
    }
}

/// LineCap 변환
fn convert_line_cap(ir_cap: primitive::LineCap) -> crate::control::LineCap {
    match ir_cap {
        primitive::LineCap::Flat => crate::control::LineCap::Flat,
        primitive::LineCap::Round => crate::control::LineCap::Round,
        primitive::LineCap::Square => crate::control::LineCap::Square,
    }
}

/// OutlineStyle 변환
fn convert_outline_style(ir_style: primitive::LineOutlineStyle) -> crate::control::OutlineStyle {
    match ir_style {
        primitive::LineOutlineStyle::Normal => crate::control::OutlineStyle::Normal,
        primitive::LineOutlineStyle::Outer => crate::control::OutlineStyle::Outer,
        primitive::LineOutlineStyle::Inner => crate::control::OutlineStyle::Inner,
    }
}

/// Fill 변환
fn convert_fill(ir_fill: &ir::border_fill::Fill) -> crate::control::Fill {
    match ir_fill {
        ir::border_fill::Fill::None => crate::control::Fill::Solid(crate::control::SolidFill {
            color: ir::Color::WHITE,
            alpha: 0,
        }),
        ir::border_fill::Fill::Solid(solid) => crate::control::Fill::Solid(crate::control::SolidFill {
            color: solid.color,
            alpha: solid.color.alpha,
        }),
        ir::border_fill::Fill::Gradient(gradient) => {
            crate::control::Fill::Gradient(crate::control::GradientFill {
                gradient_type: convert_gradient_type(gradient.gradient_type),
                angle: gradient.angle as f64,
                center_x: gradient.center_x as i32,
                center_y: gradient.center_y as i32,
                stops: gradient.stops.iter().map(convert_gradient_stop).collect(),
                step_center: gradient.step_center as i32,
            })
        }
        ir::border_fill::Fill::Image(image) => crate::control::Fill::Image(crate::control::ImageFill {
            fill_type: convert_image_fill_mode(image.mode),
            binary_id: image.binary_id.clone(),
            effect: convert_image_effect(image.effect),
            brightness: image.brightness,
            contrast: image.contrast,
        }),
        ir::border_fill::Fill::Pattern(pattern) => {
            crate::control::Fill::Pattern(crate::control::PatternFill {
                pattern_type: convert_pattern_type(pattern.pattern_type),
                foreground: pattern.foreground,
                background: pattern.background,
            })
        }
    }
}

/// GradientType 변환
fn convert_gradient_type(ir_type: primitive::GradientType) -> crate::control::GradientType {
    match ir_type {
        primitive::GradientType::Linear => crate::control::GradientType::Linear,
        primitive::GradientType::Radial => crate::control::GradientType::Radial,
        primitive::GradientType::Conical => crate::control::GradientType::Conical,
        primitive::GradientType::Square => crate::control::GradientType::Square,
    }
}

/// GradientStop 변환
fn convert_gradient_stop(ir_stop: &ir::border_fill::GradientStop) -> crate::control::GradientStop {
    crate::control::GradientStop {
        position: ir_stop.position,
        color: ir_stop.color,
    }
}

/// ImageFillMode 변환 (now same type via re-export)
fn convert_image_fill_mode(ir_mode: primitive::ImageFillMode) -> crate::control::ImageFillType {
    ir_mode
}

/// PatternType 변환 (now same type via re-export)
fn convert_pattern_type(ir_type: ir::border_fill::PatternType) -> crate::control::PatternType {
    ir_type
}

/// ShapeShadow를 Shadow로 변환
fn convert_shadow_from_shape(ir_shadow: &ir::shape::ShapeShadow) -> crate::control::Shadow {
    crate::control::Shadow {
        shadow_type: crate::control::ShadowType::Drop,
        offset_x: ir_shadow.offset_x,
        offset_y: ir_shadow.offset_y,
        color: ir_shadow.color,
        alpha: ((ir_shadow.alpha * 100.0) as u8).min(100),
        blur: ir_shadow.blur.map(|b| ir::HwpUnit::new((b * 100.0) as i32)),
        direction: ir_shadow.direction.map(|d| d as f64),
        distance: ir_shadow.distance,
    }
}

/// ShapeText 변환
fn convert_shape_text(ir_text: &ir::shape::ShapeText, doc: &mut Document) -> crate::control::ShapeText {
    let mut paragraph_ids = Vec::new();
    for ir_para in &ir_text.paragraphs {
        let para_id = convert_paragraph(ir_para, doc);
        paragraph_ids.push(para_id);
    }

    crate::control::ShapeText {
        paragraphs: paragraph_ids,
        padding: ir_text.padding,
        vertical_alignment: ir_text.vertical_alignment,
        text_direction: convert_text_direction(ir_text.text_direction),
        editable: ir_text.editable,
    }
}

/// TextDirection 변환
fn convert_text_direction(ir_dir: primitive::TextDirection) -> crate::control::TextDirection {
    match ir_dir {
        primitive::TextDirection::Horizontal => crate::control::TextDirection::Horizontal,
        primitive::TextDirection::Vertical => crate::control::TextDirection::Vertical,
        primitive::TextDirection::VerticalRightToLeft => crate::control::TextDirection::VerticalAll,
        primitive::TextDirection::RightToLeft => crate::control::TextDirection::Horizontal,
    }
}

/// IR Equation 변환
fn convert_equation(ir_equation: &ir::control::Equation, _doc: &mut Document) -> crate::control::Equation {
    crate::control::Equation {
        common: convert_object_common(&ir_equation.common),
        script: ir_equation.script.clone(),
        format: convert_equation_format(ir_equation.format),
        baseline_offset: ir_equation.baseline_offset,
        font_size: ir_equation.font_size,
        color: ir_equation.color.unwrap_or(ir::Color::BLACK),
    }
}

/// EquationFormat 변환
fn convert_equation_format(ir_format: ir::control::EquationFormat) -> crate::control::EquationFormat {
    match ir_format {
        ir::control::EquationFormat::HwpScript => crate::control::EquationFormat::HwpScript,
        ir::control::EquationFormat::MathML => crate::control::EquationFormat::MathML,
        ir::control::EquationFormat::LaTeX => crate::control::EquationFormat::LaTeX,
    }
}

/// IR Ole 변환
fn convert_ole(ir_ole: &ir::control::OleObject, _doc: &mut Document) -> crate::control::Ole {
    crate::control::Ole {
        common: convert_object_common(&ir_ole.common),
        binary_id: ir_ole.binary_id.clone(),
        class_id: ir_ole.class_id.clone(),
        preview_image_id: ir_ole.preview_image_id.clone(),
    }
}

/// IR TextBox 변환
fn convert_textbox(ir_textbox: &ir::control::TextBox, doc: &mut Document) -> crate::control::TextBox {
    let mut paragraph_ids = Vec::new();
    for ir_para in &ir_textbox.paragraphs {
        let para_id = convert_paragraph(ir_para, doc);
        paragraph_ids.push(para_id);
    }

    crate::control::TextBox {
        common: convert_object_common(&ir_textbox.common),
        paragraphs: paragraph_ids,
        text_direction: convert_text_direction(ir_textbox.text_direction),
        vertical_alignment: ir_textbox.vertical_alignment,
        padding: ir_textbox.padding,
        editable: ir_textbox.editable,
    }
}

/// IR Note 변환
fn convert_note(ir_note: &ir::control::Note, doc: &mut Document) -> crate::control::Note {
    let mut paragraph_ids = Vec::new();
    for ir_para in &ir_note.paragraphs {
        let para_id = convert_paragraph(ir_para, doc);
        paragraph_ids.push(para_id);
    }

    crate::control::Note {
        number: ir_note.number,
        number_format: ir_note.number_format,
        number_position: convert_note_number_position(ir_note.number_position),
        paragraphs: paragraph_ids,
        instance_id: ir_note.instance_id,
    }
}

/// NoteNumberPosition 변환
fn convert_note_number_position(ir_pos: primitive::NoteNumberPosition) -> crate::section::NoteNumberPosition {
    match ir_pos {
        primitive::NoteNumberPosition::Superscript => crate::section::NoteNumberPosition::Superscript,
        primitive::NoteNumberPosition::Subscript => crate::section::NoteNumberPosition::Subscript,
    }
}

/// IR HiddenComment 변환
fn convert_hidden_comment(ir_comment: &ir::control::HiddenComment, doc: &mut Document) -> crate::control::HiddenComment {
    let mut paragraph_ids = Vec::new();
    for ir_para in &ir_comment.paragraphs {
        let para_id = convert_paragraph(ir_para, doc);
        paragraph_ids.push(para_id);
    }

    crate::control::HiddenComment {
        paragraphs: paragraph_ids,
    }
}

/// IR Table 변환
fn convert_table(ir_table: &ir::table::Table, doc: &mut Document) -> Table {
    let mut table = Table {
        row_count: ir_table.row_count,
        column_count: ir_table.column_count,
        cell_spacing: ir_table.cell_spacing,
        border_fill_id: ir_table.border_fill_id,
        ..Default::default()
    };

    // 행/셀 변환
    for ir_row in &ir_table.rows {
        let row = convert_table_row(ir_row, doc);
        let row_id = doc.arena.insert_row(row);
        table.rows.push(row_id);
    }

    table
}

/// IR TableRow 변환
fn convert_table_row(ir_row: &ir::table::TableRow, doc: &mut Document) -> TableRow {
    let mut row = TableRow {
        height: ir_row.height,
        cells: Vec::new(),
    };

    for ir_cell in &ir_row.cells {
        let cell = convert_table_cell(ir_cell, doc);
        let cell_id = doc.arena.insert_cell(cell);
        row.cells.push(cell_id);
    }

    row
}

/// IR TableCell 변환
fn convert_table_cell(ir_cell: &ir::table::TableCell, doc: &mut Document) -> TableCell {
    let mut cell = TableCell {
        column: ir_cell.column,
        row: ir_cell.row,
        column_span: ir_cell.column_span,
        row_span: ir_cell.row_span,
        width: ir_cell.width,
        height: ir_cell.height,
        padding: ir_cell.padding,
        border_fill_id: ir_cell.border_fill_id,
        ..Default::default()
    };

    // 셀 내용 문단 변환
    for ir_para in &ir_cell.paragraphs {
        let para_id = convert_paragraph(ir_para, doc);
        cell.paragraphs.push(para_id);
    }

    cell
}

/// IR Compose 변환
fn convert_compose(ir_compose: &ir::paragraph::Compose) -> crate::control::Compose {
    crate::control::Compose {
        compose_type: ir_compose
            .compose_type
            .map(convert_compose_type)
            .unwrap_or(crate::control::ComposeType::Spread),
        circle_type: convert_compose_circle_type(ir_compose.circle_type),
        char_size: ir::Percent::new(ir_compose.char_size.unwrap_or(100) as f64),
        compose_text: ir_compose.compose_text.clone(),
    }
}

/// ComposeType 변환
fn convert_compose_type(ir_type: ir::paragraph::ComposeType) -> crate::control::ComposeType {
    match ir_type {
        ir::paragraph::ComposeType::Spread => crate::control::ComposeType::Spread,
        ir::paragraph::ComposeType::Overlap => crate::control::ComposeType::Overlap,
    }
}

/// ComposeCircleType 변환
fn convert_compose_circle_type(ir_type: ir::paragraph::ComposeCircleType) -> crate::control::CircleType {
    match ir_type {
        ir::paragraph::ComposeCircleType::Char => crate::control::CircleType::None,
        ir::paragraph::ComposeCircleType::ShapeCircle => crate::control::CircleType::Circle,
        // Document에는 Circle과 None만 있으므로, 나머지는 Circle로 매핑
        _ => crate::control::CircleType::Circle,
    }
}

/// IR Dutmal 변환
fn convert_dutmal(ir_dutmal: &ir::paragraph::Dutmal) -> crate::control::Dutmal {
    crate::control::Dutmal {
        position: convert_dutmal_position(ir_dutmal.position_type),
        alignment: convert_dutmal_alignment(ir_dutmal.alignment),
        main_text: ir_dutmal.main_text.clone(),
        sub_text: ir_dutmal.sub_text.clone(),
        size_ratio: ir::Percent::new(ir_dutmal.size_ratio.unwrap_or(50) as f64),
    }
}

/// DutmalPosition 변환
fn convert_dutmal_position(ir_pos: ir::paragraph::DutmalPosition) -> crate::control::DutmalPosition {
    match ir_pos {
        ir::paragraph::DutmalPosition::Top => crate::control::DutmalPosition::Top,
        ir::paragraph::DutmalPosition::Bottom => crate::control::DutmalPosition::Bottom,
    }
}

/// DutmalAlignment 변환
fn convert_dutmal_alignment(ir_align: ir::paragraph::DutmalAlignment) -> primitive::Alignment {
    match ir_align {
        ir::paragraph::DutmalAlignment::Justify => primitive::Alignment::Justify,
        ir::paragraph::DutmalAlignment::Left => primitive::Alignment::Left,
        ir::paragraph::DutmalAlignment::Right => primitive::Alignment::Right,
        ir::paragraph::DutmalAlignment::Center => primitive::Alignment::Center,
        ir::paragraph::DutmalAlignment::Distribute => primitive::Alignment::Distribute,
        // IR의 DistributeSpace는 Document의 Distribute로 매핑
        ir::paragraph::DutmalAlignment::DistributeSpace => primitive::Alignment::Distribute,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_document_conversion() {
        let ir_doc = ir::Document::default();
        let doc = Document::from(ir_doc);

        assert_eq!(doc.section_count(), 0);
    }

    #[test]
    fn test_document_with_text() {
        let mut ir_doc = ir::Document::default();

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
        ir_doc.sections.push(section);

        // 변환
        let doc = Document::from(ir_doc);

        assert_eq!(doc.section_count(), 1);
        assert_eq!(doc.to_plain_text(), "Hello, World!");
    }

    #[test]
    fn test_hyperlink_conversion() {
        let ir_hyperlink = ir::control::Hyperlink {
            target: ir::control::HyperlinkTarget::Url("https://example.com".to_string()),
            tooltip: Some("Example Site".to_string()),
            display_text: Some("Click here".to_string()),
        };

        let doc_hyperlink = convert_hyperlink(&ir_hyperlink);

        match doc_hyperlink.target {
            crate::control::HyperlinkTarget::Url(url) => assert_eq!(url, "https://example.com"),
            _ => panic!("Expected Url target"),
        }
        assert_eq!(doc_hyperlink.tooltip, Some("Example Site".to_string()));
        assert_eq!(doc_hyperlink.display_text, Some("Click here".to_string()));
    }

    #[test]
    fn test_auto_number_conversion() {
        let ir_auto_number = ir::control::AutoNumber {
            number_type: ir::control::AutoNumberType::Page,
            number_format: primitive::NumberFormat::Digit,
            auto_number_format: None,
        };

        let doc_auto_number = convert_auto_number(&ir_auto_number);

        assert!(matches!(doc_auto_number.number_type, crate::control::AutoNumberType::Page));
        assert!(matches!(doc_auto_number.number_format, primitive::NumberFormat::Digit));
    }

    #[test]
    fn test_auto_number_type_total_pages_conversion() {
        // IR의 TotalPages는 Document의 Page로 매핑
        let doc_type = convert_auto_number_type(ir::control::AutoNumberType::TotalPages);
        assert!(matches!(doc_type, crate::control::AutoNumberType::Page));
    }

    #[test]
    fn test_new_number_conversion() {
        let ir_new_number = ir::control::NewNumber {
            number_type: ir::control::AutoNumberType::Picture,
            number: 42,
        };

        let doc_new_number = convert_new_number(&ir_new_number);

        assert!(matches!(doc_new_number.number_type, crate::control::AutoNumberType::Picture));
        assert_eq!(doc_new_number.number, 42);
    }

    #[test]
    fn test_index_mark_conversion() {
        // 빈 second_key는 None으로 변환
        let ir_index_mark1 = ir::control::IndexMark {
            first_key: "Index Entry".to_string(),
            second_key: String::new(),
        };

        let doc_index_mark1 = convert_index_mark(&ir_index_mark1);
        assert_eq!(doc_index_mark1.first_key, "Index Entry");
        assert_eq!(doc_index_mark1.second_key, None);

        // 값이 있는 second_key는 Some으로 변환
        let ir_index_mark2 = ir::control::IndexMark {
            first_key: "Main Entry".to_string(),
            second_key: "Sub Entry".to_string(),
        };

        let doc_index_mark2 = convert_index_mark(&ir_index_mark2);
        assert_eq!(doc_index_mark2.first_key, "Main Entry");
        assert_eq!(doc_index_mark2.second_key, Some("Sub Entry".to_string()));
    }

    #[test]
    fn test_video_conversion() {
        let ir_video = ir::control::Video {
            common: ir::control::ObjectCommon::default(),
            video_type: ir::control::VideoType::YouTube,
            video_id: Some(ir::BinaryDataId::new("video1")),
            source_url: Some("https://youtube.com/watch?v=test".to_string()),
            preview_image_id: Some(ir::BinaryDataId::new("preview1")),
            poster_binary_id: None,
            width: None,
            height: None,
        };

        let doc_video = convert_video(&ir_video);

        // IR의 YouTube는 Document의 Web으로 변환
        assert!(matches!(doc_video.video_type, crate::control::VideoType::Web));
        assert_eq!(doc_video.video_id.as_ref().unwrap().value(), "video1");
        assert_eq!(
            doc_video.source_url.as_ref().unwrap(),
            "https://youtube.com/watch?v=test"
        );
        assert_eq!(doc_video.preview_image_id.as_ref().unwrap().value(), "preview1");
    }

    #[test]
    fn test_chart_conversion() {
        let ir_chart = ir::control::Chart {
            common: ir::control::ObjectCommon::default(),
            chart_id: "chart_data_123".to_string(),
            chart_type: ir::control::ChartType::Pie,
        };

        let doc_chart = convert_chart(&ir_chart);

        // IR의 String chart_id가 BinaryDataId로 변환
        assert_eq!(doc_chart.chart_id.value(), "chart_data_123");
        assert!(matches!(doc_chart.chart_type, crate::control::ChartType::Pie));
    }

    #[test]
    fn test_form_object_checkbox_conversion() {
        let ir_form = ir::control::FormObject {
            common: ir::control::ObjectCommon::default(),
            form_type: ir::control::FormObjectType::CheckBox,
            name: Some("check1".to_string()),
            value: None,
            button_value: Some(ir::control::ButtonValue::Checked),
            ..Default::default()
        };

        let doc_form = convert_form_object(&ir_form);

        assert_eq!(doc_form.name, "check1");
        match doc_form.form_type {
            crate::control::FormObjectType::CheckBox { checked } => {
                assert!(checked);
            }
            _ => panic!("Expected CheckBox"),
        }
    }

    #[test]
    fn test_form_object_combobox_conversion() {
        let ir_form = ir::control::FormObject {
            common: ir::control::ObjectCommon::default(),
            form_type: ir::control::FormObjectType::ComboBox,
            name: Some("combo1".to_string()),
            value: None,
            items: vec![
                ir::control::FormListItem {
                    display_text: Some("Option 1".to_string()),
                    value: Some("opt1".to_string()),
                },
                ir::control::FormListItem {
                    display_text: Some("Option 2".to_string()),
                    value: Some("opt2".to_string()),
                },
            ],
            selected_value: Some("opt2".to_string()),
            ..Default::default()
        };

        let doc_form = convert_form_object(&ir_form);

        assert_eq!(doc_form.name, "combo1");
        match doc_form.form_type {
            crate::control::FormObjectType::ComboBox { items, selected } => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0], "Option 1");
                assert_eq!(items[1], "Option 2");
                assert_eq!(selected, Some(1));
            }
            _ => panic!("Expected ComboBox"),
        }
    }

    #[test]
    fn test_form_object_scrollbar_conversion() {
        let ir_form = ir::control::FormObject {
            common: ir::control::ObjectCommon::default(),
            form_type: ir::control::FormObjectType::ScrollBar,
            name: Some("scroll1".to_string()),
            value: None,
            min: Some(0),
            max: Some(100),
            scroll_value: Some(50),
            ..Default::default()
        };

        let doc_form = convert_form_object(&ir_form);

        assert_eq!(doc_form.name, "scroll1");
        match doc_form.form_type {
            crate::control::FormObjectType::ScrollBar { min, max, value } => {
                assert_eq!(min, 0);
                assert_eq!(max, 100);
                assert_eq!(value, 50);
            }
            _ => panic!("Expected ScrollBar"),
        }
    }

    #[test]
    fn test_text_art_conversion() {
        let ir_text_art = ir::control::TextArt {
            common: ir::control::ObjectCommon::default(),
            text: "Hello World".to_string(),
            font_name: Some("Arial".to_string()),
            font_style: ir::control::TextArtFontStyle::Bold,
            shape: ir::control::TextArtShapeType::Wave,
            line_spacing: 120,
            char_spacing: 110,
            alignment: ir::control::TextArtAlignment::Center,
            line: ir::shape::LineStyle::default(),
            fill: ir::border_fill::Fill::None,
            shadow: None,
            font_type: None,
            text_art_pr: None,
        };

        let doc_text_art = convert_text_art(&ir_text_art);

        assert_eq!(doc_text_art.text, "Hello World");
        assert_eq!(doc_text_art.font_name, "Arial");
        assert!(matches!(
            doc_text_art.font_style,
            crate::control::FontStyle::Bold
        ));
        assert!(matches!(
            doc_text_art.shape,
            crate::control::TextArtShape::Wave
        ));
        assert_eq!(doc_text_art.line_spacing.0, 120.0);
        assert_eq!(doc_text_art.char_spacing.0, 110.0);
        assert!(matches!(
            doc_text_art.alignment,
            primitive::Alignment::Center
        ));
    }

    #[test]
    fn test_text_art_shape_conversion() {
        // ArchUp과 ArchDown은 모두 Arch로 변환
        assert!(matches!(
            convert_text_art_shape(ir::control::TextArtShapeType::ArchUp),
            crate::control::TextArtShape::Arch
        ));
        assert!(matches!(
            convert_text_art_shape(ir::control::TextArtShapeType::ArchDown),
            crate::control::TextArtShape::Arch
        ));

        // 기본 타입들
        assert!(matches!(
            convert_text_art_shape(ir::control::TextArtShapeType::Rectangle),
            crate::control::TextArtShape::Rectangle
        ));
        assert!(matches!(
            convert_text_art_shape(ir::control::TextArtShapeType::Circle),
            crate::control::TextArtShape::Circle
        ));
        assert!(matches!(
            convert_text_art_shape(ir::control::TextArtShapeType::Wave),
            crate::control::TextArtShape::Wave
        ));

        // Document에 없는 타입은 Rectangle로 변환
        assert!(matches!(
            convert_text_art_shape(ir::control::TextArtShapeType::Cylinder),
            crate::control::TextArtShape::Rectangle
        ));
    }

    #[test]
    fn test_video_type_conversion() {
        assert!(matches!(
            convert_video_type(ir::control::VideoType::Embedded),
            crate::control::VideoType::Embedded
        ));
        assert!(matches!(
            convert_video_type(ir::control::VideoType::Linked),
            crate::control::VideoType::Linked
        ));
        // YouTube -> Web 변환
        assert!(matches!(
            convert_video_type(ir::control::VideoType::YouTube),
            crate::control::VideoType::Web
        ));
    }

    #[test]
    fn test_chart_type_conversion() {
        assert!(matches!(
            convert_chart_type(ir::control::ChartType::Bar),
            crate::control::ChartType::Bar
        ));
        assert!(matches!(
            convert_chart_type(ir::control::ChartType::Line),
            crate::control::ChartType::Line
        ));
        assert!(matches!(
            convert_chart_type(ir::control::ChartType::Pie),
            crate::control::ChartType::Pie
        ));
        assert!(matches!(
            convert_chart_type(ir::control::ChartType::Area),
            crate::control::ChartType::Area
        ));
        assert!(matches!(
            convert_chart_type(ir::control::ChartType::Scatter),
            crate::control::ChartType::Scatter
        ));
        assert!(matches!(
            convert_chart_type(ir::control::ChartType::Radar),
            crate::control::ChartType::Radar
        ));
    }
}
