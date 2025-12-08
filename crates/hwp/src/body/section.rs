//! Section (stream) parsing.
//!
//! Each Section stream contains a list of paragraphs.

#![allow(clippy::collapsible_if)]

use crate::error::Result;
use crate::primitive::RecordTagId;
use crate::util::ByteReader;

use super::chart::ChartData;
use super::container::ShapeContainer;
use super::control::{Control, ControlContent, ControlId, ControlType};
use super::control_data::ControlData;
use super::equation::Equation;
use super::field::{Field, FieldType};
use super::footnote::{Endnote, Footnote, FootnoteShape};
use super::form_object::FormObject;
use super::hyperlink::Hyperlink;
use super::page::{PageBorderFill, PageDefinition};
use super::header_footer::{Footer, Header, HeaderFooterTarget};
use super::list_header::ListHeader;
use super::memo::{Memo, MemoShape};
use super::paragraph::{
    CharacterShapeReference, LineSegment, Paragraph, ParagraphText, RangeTag,
};
use super::parse_record_header;
use super::picture::{OleObject, Picture};
use super::shape::{
    ArcShape, CurveShape, EllipseShape, LineShape, PolygonShape, RectangleShape, Shape,
    ShapeElementProperties, ShapeType,
};
use super::table::{Table, TableCell};
use super::text_art::TextArt;
use super::video::VideoData;

/// Parsing context for nested content.
///
/// HWP documents have nested structures where controls (tables, text boxes, etc.)
/// can contain paragraphs, which in turn can contain more controls.
#[derive(Debug, Clone)]
enum ParsingContext {
    /// Top-level section parsing.
    Section,
    /// Inside a table cell.
    TableCell {
        /// Expected number of paragraphs in this cell.
        paragraph_count: u16,
        /// Cell properties parsed from ListHeader.
        cell: TableCell,
    },
    /// Inside a header or footer.
    HeaderFooter {
        /// Expected number of paragraphs.
        paragraph_count: u16,
    },
    /// Inside a footnote or endnote.
    FootnoteEndnote {
        /// Expected number of paragraphs.
        paragraph_count: u16,
    },
    /// Inside a text box or caption.
    TextBox {
        /// Expected number of paragraphs.
        paragraph_count: u16,
    },
}

impl ParsingContext {
    /// Returns the expected paragraph count for this context.
    fn paragraph_count(&self) -> Option<u16> {
        match self {
            Self::Section => None,
            Self::TableCell { paragraph_count, .. } => Some(*paragraph_count),
            Self::HeaderFooter { paragraph_count } => Some(*paragraph_count),
            Self::FootnoteEndnote { paragraph_count } => Some(*paragraph_count),
            Self::TextBox { paragraph_count } => Some(*paragraph_count),
        }
    }
}

/// A section in the document.
///
/// Sections are the top-level containers in the body text.
/// Each section can have different page settings.
#[derive(Debug, Clone, Default)]
pub struct Section {
    /// Paragraphs in this section.
    paragraphs: Vec<Paragraph>,
    /// Page definition (size, margins, orientation).
    page_definition: Option<PageDefinition>,
    /// Footnote shape settings.
    footnote_shape: Option<FootnoteShape>,
    /// Page border and fill settings.
    page_border_fill: Option<PageBorderFill>,
    /// Memos (annotations) in this section.
    memos: Vec<Memo>,
}

impl Section {
    /// Creates a new empty section.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses a section from bytes.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let mut reader = ByteReader::new(data);
        let mut section = Section::new();

        // Parsing state
        let mut current_paragraph: Option<Paragraph> = None;
        let mut current_control: Option<Control> = None;
        let mut current_memo_shape: Option<MemoShape> = None;

        // Stack for nested content parsing
        let mut context_stack: Vec<ParsingContext> = vec![ParsingContext::Section];
        let mut nested_paragraphs: Vec<Vec<Paragraph>> = vec![Vec::new()];
        let mut cell_paragraph_counts: Vec<u16> = vec![0];

        while !reader.is_empty() {
            // Try to parse record header - if not enough data, stop parsing
            let header = match parse_record_header(&mut reader) {
                Ok(h) => h,
                Err(_) => break, // Stop if we can't read a valid record header
            };

            // Check if we have enough data for the record - if not, stop gracefully
            let data_size = header.data_size() as usize;
            if reader.remaining() < data_size {
                // Not enough data remaining - stop parsing this section gracefully
                // This can happen with truncated streams or very large documents
                break;
            }

            let record_data = reader.read_bytes(data_size)?;
            let mut record_reader = ByteReader::new(record_data);

            match header.tag_id() {
                Some(RecordTagId::ParagraphHeader) => {
                    // Save previous paragraph
                    if let Some(para) = current_paragraph.take() {
                        Self::save_paragraph(&mut section, &mut context_stack, &mut nested_paragraphs, &mut cell_paragraph_counts, &mut current_control, para);
                    }

                    current_paragraph =
                        Some(Paragraph::from_reader(&mut record_reader, header.data_size())?);
                }

                Some(RecordTagId::ParagraphText) => {
                    if let Some(ref mut para) = current_paragraph {
                        let text =
                            ParagraphText::from_reader(&mut record_reader, para.character_count())?;
                        para.set_text(text);
                    }
                }

                Some(RecordTagId::ParagraphCharacterShape) => {
                    if let Some(ref mut para) = current_paragraph {
                        let count = header.data_size() / 8;
                        for _ in 0..count {
                            let reference = CharacterShapeReference::from_reader(&mut record_reader)?;
                            para.add_character_shape_reference(reference);
                        }
                    }
                }

                Some(RecordTagId::ParagraphLineSegment) => {
                    if let Some(ref mut para) = current_paragraph {
                        let count = header.data_size() as usize / LineSegment::SIZE;
                        for _ in 0..count {
                            let segment = LineSegment::from_reader(&mut record_reader)?;
                            para.add_line_segment(segment);
                        }
                    }
                }

                Some(RecordTagId::ParagraphRangeTag) => {
                    if let Some(ref mut para) = current_paragraph {
                        let count = header.data_size() as usize / RangeTag::SIZE;
                        for _ in 0..count {
                            let tag = RangeTag::from_reader(&mut record_reader)?;
                            para.add_range_tag(tag);
                        }
                    }
                }

                Some(RecordTagId::ControlHeader) => {
                    // Save previous control if exists
                    if let Some(ctrl) = current_control.take() {
                        if let Some(ref mut para) = current_paragraph {
                            para.add_control(ctrl);
                        }
                    }

                    let control_id = ControlId::from_reader(&mut record_reader)?;
                    let remaining = record_data.len().saturating_sub(4);
                    let control_data = if remaining > 0 {
                        record_reader.read_bytes(remaining)?.to_vec()
                    } else {
                        Vec::new()
                    };

                    let mut ctrl = Control::with_data(control_id, control_data.clone());

                    // Initialize content for Header/Footer controls
                    match ctrl.control_type() {
                        ControlType::Header => {
                            // Parse target from control data (first byte)
                            let target = if !control_data.is_empty() {
                                HeaderFooterTarget::from_raw(control_data[0])
                            } else {
                                HeaderFooterTarget::BothPages
                            };
                            ctrl.set_content(ControlContent::Header(Header::new(target)));
                        }
                        ControlType::Footer => {
                            let target = if !control_data.is_empty() {
                                HeaderFooterTarget::from_raw(control_data[0])
                            } else {
                                HeaderFooterTarget::BothPages
                            };
                            ctrl.set_content(ControlContent::Footer(Footer::new(target)));
                        }
                        ControlType::Footnote => {
                            // Parse note number from control data (bytes 4-5, u16)
                            let number = if control_data.len() >= 6 {
                                u16::from_le_bytes([control_data[4], control_data[5]])
                            } else {
                                0
                            };
                            ctrl.set_content(ControlContent::Footnote(Footnote::new(number)));
                        }
                        ControlType::Endnote => {
                            let number = if control_data.len() >= 6 {
                                u16::from_le_bytes([control_data[4], control_data[5]])
                            } else {
                                0
                            };
                            ctrl.set_content(ControlContent::Endnote(Endnote::new(number)));
                        }
                        _ => {}
                    }

                    current_control = Some(ctrl);
                }

                Some(RecordTagId::ListHeader) => {
                    // ListHeader marks the beginning of nested content
                    if let Ok(list_header) = ListHeader::from_reader(&mut record_reader) {
                        let para_count = list_header.paragraph_count();

                        // Determine context type based on current control
                        if let Some(ref ctrl) = current_control {
                            match ctrl.control_type() {
                                ControlType::Table => {
                                    // This is a table cell - read cell properties after ListHeader
                                    // Cell properties follow ListHeader in the record
                                    let cell = if record_reader.remaining() >= TableCell::SIZE {
                                        TableCell::from_reader(&mut record_reader).unwrap_or_default()
                                    } else {
                                        TableCell::default()
                                    };

                                    context_stack.push(ParsingContext::TableCell {
                                        paragraph_count: para_count,
                                        cell,
                                    });
                                    nested_paragraphs.push(Vec::new());
                                    cell_paragraph_counts.push(0);
                                }
                                ControlType::Header | ControlType::Footer => {
                                    context_stack.push(ParsingContext::HeaderFooter {
                                        paragraph_count: para_count,
                                    });
                                    nested_paragraphs.push(Vec::new());
                                    cell_paragraph_counts.push(0);
                                }
                                ControlType::Footnote | ControlType::Endnote => {
                                    context_stack.push(ParsingContext::FootnoteEndnote {
                                        paragraph_count: para_count,
                                    });
                                    nested_paragraphs.push(Vec::new());
                                    cell_paragraph_counts.push(0);
                                }
                                _ => {
                                    // Generic nested content (text box, caption, etc.)
                                    context_stack.push(ParsingContext::TextBox {
                                        paragraph_count: para_count,
                                    });
                                    nested_paragraphs.push(Vec::new());
                                    cell_paragraph_counts.push(0);
                                }
                            }
                        }
                    }
                }

                Some(RecordTagId::PageDefinition) => {
                    if let Ok(page_def) = PageDefinition::from_reader(&mut record_reader) {
                        section.page_definition = Some(page_def);
                    }
                }

                Some(RecordTagId::FootnoteShape) => {
                    if let Ok(footnote_shape) = FootnoteShape::from_reader(&mut record_reader) {
                        section.footnote_shape = Some(footnote_shape);
                    }
                }

                Some(RecordTagId::PageBorderFill) => {
                    if let Ok(border_fill) = PageBorderFill::from_reader(&mut record_reader) {
                        section.page_border_fill = Some(border_fill);
                    }
                }

                Some(RecordTagId::Table) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(table) = Table::from_reader(&mut record_reader) {
                            ctrl.set_content(ControlContent::Table(table));
                        }
                    }
                }

                Some(RecordTagId::ShapeComponent) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(props) = ShapeElementProperties::from_reader(&mut record_reader) {
                            let shape = Shape::new(props);
                            ctrl.set_content(ControlContent::Shape(shape));
                        }
                    }
                }

                Some(RecordTagId::ShapeComponentLine) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(line) = LineShape::from_reader(&mut record_reader) {
                            if let Some(shape) = ctrl.as_shape_mut() {
                                shape.shape_type = ShapeType::Line(line);
                            }
                        }
                    }
                }

                Some(RecordTagId::ShapeComponentRectangle) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(rect) = RectangleShape::from_reader(&mut record_reader) {
                            if let Some(shape) = ctrl.as_shape_mut() {
                                shape.shape_type = ShapeType::Rectangle(rect);
                            }
                        }
                    }
                }

                Some(RecordTagId::ShapeComponentEllipse) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(ellipse) = EllipseShape::from_reader(&mut record_reader) {
                            if let Some(shape) = ctrl.as_shape_mut() {
                                shape.shape_type = ShapeType::Ellipse(ellipse);
                            }
                        }
                    }
                }

                Some(RecordTagId::ShapeComponentArc) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(arc) = ArcShape::from_reader(&mut record_reader) {
                            if let Some(shape) = ctrl.as_shape_mut() {
                                shape.shape_type = ShapeType::Arc(arc);
                            }
                        }
                    }
                }

                Some(RecordTagId::ShapeComponentPolygon) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(polygon) = PolygonShape::from_reader(&mut record_reader) {
                            if let Some(shape) = ctrl.as_shape_mut() {
                                shape.shape_type = ShapeType::Polygon(polygon);
                            }
                        }
                    }
                }

                Some(RecordTagId::ShapeComponentCurve) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(curve) = CurveShape::from_reader(&mut record_reader) {
                            if let Some(shape) = ctrl.as_shape_mut() {
                                shape.shape_type = ShapeType::Curve(curve);
                            }
                        }
                    }
                }

                Some(RecordTagId::ShapeComponentPicture) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(picture) = Picture::from_reader(&mut record_reader) {
                            ctrl.set_content(ControlContent::Picture(picture));
                        }
                    }
                }

                Some(RecordTagId::ShapeComponentOle) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(ole) = OleObject::from_reader(&mut record_reader) {
                            ctrl.set_content(ControlContent::OleObject(ole));
                        }
                    }
                }

                Some(RecordTagId::Equation) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(equation) = Equation::from_reader(&mut record_reader) {
                            ctrl.set_content(ControlContent::Equation(equation));
                        }
                    }
                }

                Some(RecordTagId::ChartData) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(chart) = ChartData::from_reader(&mut record_reader) {
                            ctrl.set_content(ControlContent::Chart(chart));
                        }
                    }
                }

                Some(RecordTagId::VideoData) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(video) = VideoData::from_reader(&mut record_reader) {
                            ctrl.set_content(ControlContent::Video(video));
                        }
                    }
                }

                Some(RecordTagId::ShapeComponentTextArt) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(text_art) = TextArt::from_reader(&mut record_reader) {
                            ctrl.set_content(ControlContent::TextArt(text_art));
                        }
                    }
                }

                Some(RecordTagId::FormObject) => {
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(form) = FormObject::from_reader(&mut record_reader) {
                            ctrl.set_content(ControlContent::FormObject(form));
                        }
                    }
                }

                Some(RecordTagId::MemoShape) => {
                    // MemoShape defines the visual properties of a memo
                    // Store it for association with the following MemoList
                    if let Ok(memo_shape) = MemoShape::from_reader(&mut record_reader) {
                        current_memo_shape = Some(memo_shape);
                    }
                }

                Some(RecordTagId::MemoList) => {
                    // MemoList is a 4-byte record header for memo content
                    // Format: UINT32 (unknown content, possibly flags)
                    // The memo shape was already stored via MemoShape record
                    // For now, just acknowledge the record - memo content parsing
                    // requires further investigation of HWP spec
                    if record_reader.remaining() >= 4 {
                        let _flags = record_reader.read_u32().unwrap_or(0);
                        // Store the memo with its shape if available
                        if let Some(shape) = current_memo_shape.take() {
                            section.memos.push(Memo::with_shape(shape));
                        }
                    }
                }

                Some(RecordTagId::ShapeComponentContainer) => {
                    // Container for grouped shapes - parse container info
                    // Children are parsed as subsequent ShapeComponent records
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(container) = ShapeContainer::from_reader(&mut record_reader) {
                            if let Some(shape) = ctrl.as_shape_mut() {
                                // Initialize as Container type with empty children
                                // Children will be populated from subsequent records
                                shape.shape_type = ShapeType::Container(Vec::new());
                            }
                            // Store container metadata if needed in the future
                            let _ = container;
                        }
                    }
                }

                Some(RecordTagId::ControlData) => {
                    // Control arbitrary data (field names, hyperlink info)
                    // Parse ControlData and update the current control
                    if let Some(ref mut ctrl) = current_control {
                        if let Ok(ctrl_data) = ControlData::from_reader(&mut record_reader) {
                            // Update control based on type
                            let control_id_bytes = ctrl.id().raw().to_le_bytes();
                            match ctrl.control_type() {
                                ControlType::Field => {
                                    // Parse field control with control data
                                    let field = Field::from_control_id_and_data(
                                        &control_id_bytes,
                                        Some(&ctrl_data),
                                    );
                                    // Special case: hyperlink field (%hlk)
                                    if field.field_type() == FieldType::Hyperlink {
                                        let hyperlink = Hyperlink::from_control_data(&ctrl_data);
                                        ctrl.set_content(ControlContent::Hyperlink(hyperlink));
                                    } else {
                                        ctrl.set_content(ControlContent::Field(field));
                                    }
                                }
                                _ => {
                                    // Other control types may use control data differently
                                }
                            }
                        }
                    }
                }

                Some(RecordTagId::ShapeComponentUnknown) => {
                    // Unknown shape component - skip
                }

                _ => {
                    // Skip unknown or unhandled records
                }
            }
        }

        // Save final paragraph
        if let Some(ctrl) = current_control.take() {
            if let Some(ref mut para) = current_paragraph {
                para.add_control(ctrl);
            }
        }
        if let Some(para) = current_paragraph.take() {
            Self::save_paragraph(&mut section, &mut context_stack, &mut nested_paragraphs, &mut cell_paragraph_counts, &mut None, para);
        }

        Ok(section)
    }

    /// Saves a paragraph to the appropriate container based on the current context.
    fn save_paragraph(
        section: &mut Section,
        context_stack: &mut Vec<ParsingContext>,
        nested_paragraphs: &mut Vec<Vec<Paragraph>>,
        cell_paragraph_counts: &mut Vec<u16>,
        current_control: &mut Option<Control>,
        paragraph: Paragraph,
    ) {
        // Get the current context
        let context_idx = context_stack.len().saturating_sub(1);

        match context_stack.get(context_idx) {
            Some(ParsingContext::Section) | None => {
                // Top-level paragraph
                section.paragraphs.push(paragraph);
            }
            Some(ParsingContext::TableCell { .. })
            | Some(ParsingContext::HeaderFooter { .. })
            | Some(ParsingContext::FootnoteEndnote { .. })
            | Some(ParsingContext::TextBox { .. }) => {
                // Nested paragraph - store in the nested container
                if let Some(paragraphs) = nested_paragraphs.get_mut(context_idx) {
                    paragraphs.push(paragraph);
                }

                // Increment the paragraph count for this context
                if let Some(count) = cell_paragraph_counts.get_mut(context_idx) {
                    *count += 1;
                }

                // Check if we've collected all paragraphs for this context
                Self::try_complete_context(
                    context_stack,
                    nested_paragraphs,
                    cell_paragraph_counts,
                    current_control,
                );
            }
        }
    }

    /// Tries to complete the current context if all paragraphs have been collected.
    fn try_complete_context(
        context_stack: &mut Vec<ParsingContext>,
        nested_paragraphs: &mut Vec<Vec<Paragraph>>,
        cell_paragraph_counts: &mut Vec<u16>,
        current_control: &mut Option<Control>,
    ) {
        let context_idx = context_stack.len().saturating_sub(1);

        // Don't pop Section context
        if context_idx == 0 {
            return;
        }

        let should_pop = if let Some(ctx) = context_stack.get(context_idx) {
            if let Some(expected) = ctx.paragraph_count() {
                if let Some(&current) = cell_paragraph_counts.get(context_idx) {
                    current >= expected
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if should_pop {
            // Pop the context
            if let Some(ctx) = context_stack.pop() {
                let paragraphs = nested_paragraphs.pop().unwrap_or_default();
                cell_paragraph_counts.pop();

                // Add paragraphs to the appropriate target
                match ctx {
                    ParsingContext::TableCell { mut cell, .. } => {
                        // Store paragraphs in the cell
                        cell.paragraphs = paragraphs;
                        // Add cell to the table in current control
                        if let Some(ctrl) = current_control {
                            if let Some(table) = ctrl.as_table_mut() {
                                table.add_cell(cell);
                            }
                        }
                    }
                    ParsingContext::HeaderFooter { .. } => {
                        // Store paragraphs in the Header/Footer content
                        if let Some(ctrl) = current_control {
                            if let Some(header) = ctrl.as_header_mut() {
                                header.set_paragraphs(paragraphs);
                            } else if let Some(footer) = ctrl.as_footer_mut() {
                                footer.set_paragraphs(paragraphs);
                            } else {
                                // Fallback: add as children
                                for para in paragraphs {
                                    ctrl.add_child(para);
                                }
                            }
                        }
                    }
                    ParsingContext::FootnoteEndnote { .. } => {
                        // Store paragraphs in the Footnote/Endnote content
                        if let Some(ctrl) = current_control {
                            if let Some(footnote) = ctrl.as_footnote_mut() {
                                footnote.set_paragraphs(paragraphs);
                            } else if let Some(endnote) = ctrl.as_endnote_mut() {
                                endnote.set_paragraphs(paragraphs);
                            } else {
                                // Fallback: add as children
                                for para in paragraphs {
                                    ctrl.add_child(para);
                                }
                            }
                        }
                    }
                    ParsingContext::TextBox { .. } => {
                        // Add as children to the current control
                        if let Some(ctrl) = current_control {
                            for para in paragraphs {
                                ctrl.add_child(para);
                            }
                        }
                    }
                    ParsingContext::Section => {
                        // Should not happen
                    }
                }
            }
        }
    }

    /// Returns the paragraphs in this section.
    pub fn paragraphs(&self) -> &[Paragraph] {
        &self.paragraphs
    }

    /// Returns a mutable reference to the paragraphs.
    pub fn paragraphs_mut(&mut self) -> &mut Vec<Paragraph> {
        &mut self.paragraphs
    }

    /// Extracts all plain text from this section.
    pub fn plain_text(&self) -> String {
        self.paragraphs
            .iter()
            .map(|p| p.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Returns the number of paragraphs.
    pub fn paragraph_count(&self) -> usize {
        self.paragraphs.len()
    }

    /// Returns the page definition for this section.
    pub fn page_definition(&self) -> Option<&PageDefinition> {
        self.page_definition.as_ref()
    }

    /// Returns the footnote shape settings for this section.
    pub fn footnote_shape(&self) -> Option<&FootnoteShape> {
        self.footnote_shape.as_ref()
    }

    /// Returns the page border fill settings for this section.
    pub fn page_border_fill(&self) -> Option<&PageBorderFill> {
        self.page_border_fill.as_ref()
    }

    /// Returns the memos (annotations) in this section.
    pub fn memos(&self) -> &[Memo] {
        &self.memos
    }

    /// Returns the number of memos.
    pub fn memo_count(&self) -> usize {
        self.memos.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BreakType;

    #[test]
    fn test_section_new() {
        let section = Section::new();
        assert!(section.paragraphs().is_empty());
        assert_eq!(section.paragraph_count(), 0);
    }

    #[test]
    fn test_parsing_context_paragraph_count() {
        let section_ctx = ParsingContext::Section;
        assert_eq!(section_ctx.paragraph_count(), None);

        let table_cell_ctx = ParsingContext::TableCell {
            paragraph_count: 3,
            cell: TableCell::default(),
        };
        assert_eq!(table_cell_ctx.paragraph_count(), Some(3));

        let header_ctx = ParsingContext::HeaderFooter { paragraph_count: 2 };
        assert_eq!(header_ctx.paragraph_count(), Some(2));

        let footnote_ctx = ParsingContext::FootnoteEndnote { paragraph_count: 1 };
        assert_eq!(footnote_ctx.paragraph_count(), Some(1));

        let textbox_ctx = ParsingContext::TextBox { paragraph_count: 5 };
        assert_eq!(textbox_ctx.paragraph_count(), Some(5));
    }

    #[test]
    fn test_section_plain_text_empty() {
        let section = Section::new();
        assert_eq!(section.plain_text(), "");
    }

    #[test]
    fn test_section_paragraphs_mut() {
        let mut section = Section::new();
        assert!(section.paragraphs_mut().is_empty());

        // Can add paragraphs via mutable reference
        let para = Paragraph::new(0, 0, 0, 0, BreakType::from_raw(0), 0, None);
        section.paragraphs_mut().push(para);
        assert_eq!(section.paragraph_count(), 1);
    }
}
