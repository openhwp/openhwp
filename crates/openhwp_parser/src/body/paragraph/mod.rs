pub mod char_shape;
pub mod control;
pub mod line_segment;
pub mod paragraph_header;
pub mod paragraph_range_tag;
pub mod paragraph_text;

pub use char_shape::*;
pub use control::*;
pub use line_segment::*;
pub use paragraph_header::*;
pub use paragraph_range_tag::*;
pub use paragraph_text::*;

use crate::{BodyIter, HwpDocumentError, Version};

#[derive(Debug)]
pub struct Paragraph {
    pub header: ParagraphHeader,
    pub text: ParagraphText,
    pub char_shapes: Vec<CharShape>,
    pub line_segments: Vec<LineSegment>,
    pub range_tags: Vec<ParagraphRangeTag>,
    pub controls: Vec<Control>,
}

impl<'hwp> BodyIter<'hwp> {
    pub fn paragraphs(&mut self, version: &Version) -> Result<Vec<Paragraph>, HwpDocumentError> {
        let mut paragraphs = vec![];

        while !self.is_empty() {
            paragraphs.push(self.paragraph(version)?);
        }

        Ok(paragraphs)
    }

    pub fn paragraph(&mut self, version: &Version) -> Result<Paragraph, HwpDocumentError> {
        let header = self.paragraph_header(version)?;
        let text = self.paragraph_text(header.text_size);
        let char_shapes = self.char_shapes(header.char_shape_count);
        let line_segments = self.line_segments(header.line_segment_count);
        let range_tags = self.paragraph_range_tags(header.range_tag_count);
        let controls = self.controls(text.control_count())?;

        Ok(Paragraph {
            header,
            text,
            char_shapes,
            line_segments,
            range_tags,
            controls,
        })
    }
}
