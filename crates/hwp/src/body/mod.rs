//! Body text (section stream) parsing.
//!
//! The BodyText storage contains Section streams (Section0, Section1, etc.)
//! with the document's actual content including paragraphs, tables, and shapes.

mod chart;
mod container;
mod control;
mod control_data;
mod equation;
mod field;
mod footnote;
mod form_object;
mod header_footer;
mod hyperlink;
mod list_header;
mod memo;
mod page;
mod paragraph;
mod picture;
mod section;
mod shape;
mod table;
mod text_art;
mod text_box;
mod video;

// 공개 API - HWP 스펙에 정의된 타입들
pub use control::{Control, ControlCharacter, ControlContent, ControlId, ControlType};
pub use equation::{Equation, EquationLineMode, EquationProperties};
pub use field::{Field, FieldType};
pub use footnote::{Endnote, EndnoteShape, Footnote, FootnoteShape, NotePlacement, NoteNumberingType};
pub use header_footer::{Footer, Header, HeaderFooterTarget};
pub use hyperlink::{Hyperlink, HyperlinkType};
pub use list_header::{ListHeader, TextDirection};
pub use page::{GutterPosition, PageBorderFill, PageBorderFillPosition, PageDefinition, PageMargins, PageOrientation};
pub use paragraph::{BreakType, CharacterShapeReference, LineSegment, Paragraph, RangeTag};
pub use picture::{
    ImageCrop, ImageFlip, InnerMargin, OleObject, Picture, PictureEffect, PictureFill,
    PictureProperties,
};
pub use section::Section;
pub use shape::{
    ArcShape, ArcType, ArrowSize, ArrowType, CurveSegmentType, CurveShape, EllipseShape,
    LineEndCap, LineShape, Point, PolygonShape, RectangleShape, Shape, ShapeBorderLine,
    ShapeElementProperties, ShapeType,
};
pub use table::{Table, TableCell, TableProperties};
pub use text_box::{Caption, CaptionDirection, TextBox, VerticalAlignment};

// 추가 컨트롤 타입
pub use chart::{ChartData, ChartSeries, ChartType};
pub use container::ShapeContainer;
pub use form_object::{FormObject, FormObjectType};
pub use memo::{Memo, MemoShape};
pub use text_art::{TextArt, TextArtAlignment, TextArtShape};
pub use video::{VideoData, VideoType};

use crate::error::Result;
use crate::primitive::RecordHeader;
use crate::util::ByteReader;

/// Parses a record header from the reader.
pub fn parse_record_header(reader: &mut ByteReader) -> Result<RecordHeader> {
    let raw = reader.read_u32()?;
    let header = RecordHeader::new(raw);

    if header.has_extended_size() {
        let extended_size = reader.read_u32()?;
        Ok(RecordHeader::with_extended_size(raw, extended_size))
    } else {
        Ok(header)
    }
}
