//! Control (special object) definitions.
//!
//! Controls are special elements embedded in paragraphs like tables,
//! images, shapes, equations, etc.

use crate::error::Result;
use crate::util::ByteReader;

use super::chart::ChartData;
use super::container::ShapeContainer;
use super::equation::Equation;
use super::field::Field;
use super::footnote::{Endnote, Footnote};
use super::form_object::FormObject;
use super::header_footer::{Footer, Header};
use super::hyperlink::Hyperlink;
use super::memo::Memo;
use super::picture::{OleObject, Picture};
use super::section_definition::{ColumnDefinition, SectionDefinition};
use super::shape::Shape;
use super::table::Table;
use super::text_art::TextArt;
use super::text_box::{Caption, TextBox};
use super::video::VideoData;

/// 파싱된 컨트롤 콘텐츠.
///
/// 컨트롤이 포함하는 실제 데이터 (표, 그림, 도형 등).
#[derive(Debug, Clone)]
pub enum ControlContent {
    /// 표.
    Table(Table),
    /// 도형.
    Shape(Shape),
    /// 그림 (이미지).
    Picture(Picture),
    /// 수식.
    Equation(Equation),
    /// OLE 개체.
    OleObject(OleObject),
    /// 머리글.
    Header(Header),
    /// 꼬리글.
    Footer(Footer),
    /// 각주.
    Footnote(Footnote),
    /// 미주.
    Endnote(Endnote),
    /// 하이퍼링크.
    Hyperlink(Hyperlink),
    /// 필드 (날짜, 시간, 페이지 번호 등).
    Field(Field),
    /// 텍스트 박스.
    TextBox(TextBox),
    /// 캡션.
    Caption(Caption),
    /// 메모 (주석).
    Memo(Memo),
    /// 양식 객체.
    FormObject(FormObject),
    /// 텍스트 아트.
    TextArt(TextArt),
    /// 차트.
    Chart(ChartData),
    /// 비디오.
    Video(VideoData),
    /// 그룹화된 도형들.
    Container(ShapeContainer),
    /// 구역 정의.
    SectionDefinition(SectionDefinition),
    /// 단 정의.
    ColumnDefinition(ColumnDefinition),
}

/// Control character types in HWP documents.
///
/// These are special Unicode characters used to mark various elements
/// in the paragraph text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlCharacter {
    /// Line break (0x000A).
    LineBreak,
    /// Paragraph break (0x000D).
    ParagraphBreak,
    /// Hard hyphen (0x001E) - non-breaking hyphen.
    HardHyphen,
    /// Non-breaking space (0x00A0).
    NonBreakingSpace,
    /// Section/column definition (0x0002).
    SectionColumnDefinition,
    /// Field start (0x0003).
    FieldStart,
    /// Field end (0x0004).
    FieldEnd,
    /// Title mark (0x0005).
    TitleMark,
    /// Tab (0x0009).
    Tab,
    /// Drawing object/table (0x000B).
    DrawingTableObject,
    /// Inline control (0x000C) - equations, etc.
    InlineControl,
    /// Bookmark/hyperlink start (0x0010).
    BookmarkStart,
    /// Bookmark/hyperlink end (0x0011).
    BookmarkEnd,
    /// Hidden comment (0x0015).
    HiddenComment,
    /// Header/footer (0x0016).
    HeaderFooter,
    /// Footnote/endnote (0x0017).
    FootnoteEndnote,
    /// Auto numbering (0x0018).
    AutoNumbering,
    /// Page control (0x001C) - page hide, etc.
    PageControl,
    /// Unknown control character.
    Unknown(u16),
}

impl ControlCharacter {
    /// Creates from a character code.
    pub const fn from_char(c: u16) -> Option<Self> {
        match c {
            0x000A => Some(Self::LineBreak),
            0x000D => Some(Self::ParagraphBreak),
            0x001E => Some(Self::HardHyphen),
            0x00A0 => Some(Self::NonBreakingSpace),
            0x0002 => Some(Self::SectionColumnDefinition),
            0x0003 => Some(Self::FieldStart),
            0x0004 => Some(Self::FieldEnd),
            0x0005 => Some(Self::TitleMark),
            0x0009 => Some(Self::Tab),
            0x000B => Some(Self::DrawingTableObject),
            0x000C => Some(Self::InlineControl),
            0x0010 => Some(Self::BookmarkStart),
            0x0011 => Some(Self::BookmarkEnd),
            0x0015 => Some(Self::HiddenComment),
            0x0016 => Some(Self::HeaderFooter),
            0x0017 => Some(Self::FootnoteEndnote),
            0x0018 => Some(Self::AutoNumbering),
            0x001C => Some(Self::PageControl),
            _ => None,
        }
    }

    /// Returns true if this is a control character.
    pub const fn is_control_char(c: u16) -> bool {
        matches!(
            c,
            0x0002
                | 0x0003
                | 0x0004
                | 0x0005
                | 0x0009
                | 0x000A
                | 0x000B
                | 0x000C
                | 0x000D
                | 0x0010
                | 0x0011
                | 0x0015
                | 0x0016
                | 0x0017
                | 0x0018
                | 0x001C
                | 0x001E
                | 0x00A0
        )
    }
}

/// Control type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlType {
    /// Section definition.
    SectionDefinition,
    /// Column definition.
    ColumnDefinition,
    /// Table.
    Table,
    /// Equation.
    Equation,
    /// Drawing object (shapes, lines, etc.).
    DrawingObject,
    /// Picture.
    Picture,
    /// OLE object.
    Ole,
    /// Header.
    Header,
    /// Footer.
    Footer,
    /// Footnote.
    Footnote,
    /// Endnote.
    Endnote,
    /// Auto number.
    AutoNumber,
    /// Page number position.
    PageNumberPosition,
    /// New number.
    NewNumber,
    /// Page hide.
    PageHide,
    /// Page odd/even.
    PageOddEven,
    /// Bookmark.
    Bookmark,
    /// Index mark.
    IndexMark,
    /// Hidden comment.
    HiddenComment,
    /// Text art.
    TextArt,
    /// Form object.
    FormObject,
    /// Field (date, time, etc.).
    Field,
    /// Unknown control.
    Unknown,
}

/// Control ID (4-byte character code).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ControlId(u32);

impl ControlId {
    // Common control IDs
    /// Section definition.
    pub const SECTION_DEFINITION: Self = Self::from_chars(b"secd");
    /// Column definition.
    pub const COLUMN_DEFINITION: Self = Self::from_chars(b"cold");
    /// Table.
    pub const TABLE: Self = Self::from_chars(b"tbl ");
    /// Equation.
    pub const EQUATION: Self = Self::from_chars(b"eqed");
    /// Generic shape container.
    pub const SHAPE_CONTAINER: Self = Self::from_chars(b"gso ");
    /// Header.
    pub const HEADER: Self = Self::from_chars(b"head");
    /// Footer.
    pub const FOOTER: Self = Self::from_chars(b"foot");
    /// Footnote.
    pub const FOOTNOTE: Self = Self::from_chars(b"fn  ");
    /// Endnote.
    pub const ENDNOTE: Self = Self::from_chars(b"en  ");
    /// Auto number.
    pub const AUTO_NUMBER: Self = Self::from_chars(b"atno");
    /// New number.
    pub const NEW_NUMBER: Self = Self::from_chars(b"nwno");
    /// Page number position.
    pub const PAGE_NUMBER_POSITION: Self = Self::from_chars(b"pgnp");
    /// Page hide.
    pub const PAGE_HIDE: Self = Self::from_chars(b"pgct");
    /// Page odd/even adjust.
    pub const PAGE_ODD_EVEN: Self = Self::from_chars(b"pght");
    /// Bookmark.
    pub const BOOKMARK: Self = Self::from_chars(b"bokm");
    /// Index mark.
    pub const INDEX_MARK: Self = Self::from_chars(b"idxm");
    /// Hidden comment.
    pub const HIDDEN_COMMENT: Self = Self::from_chars(b"tcmt");
    /// Text art.
    pub const TEXT_ART: Self = Self::from_chars(b"dso ");
    /// Form object.
    pub const FORM_OBJECT: Self = Self::from_chars(b"form");

    // Field control IDs
    /// Date field.
    pub const FIELD_DATE: Self = Self::from_chars(b"%dat");
    /// Time field.
    pub const FIELD_TIME: Self = Self::from_chars(b"%tim");
    /// File path field.
    pub const FIELD_FILE_PATH: Self = Self::from_chars(b"%fil");
    /// Document title field.
    pub const FIELD_DOC_TITLE: Self = Self::from_chars(b"%tit");
    /// Author field.
    pub const FIELD_AUTHOR: Self = Self::from_chars(b"%aut");
    /// Page number field.
    pub const FIELD_PAGE_NUMBER: Self = Self::from_chars(b"%pn ");
    /// Click here field.
    pub const FIELD_CLICK_HERE: Self = Self::from_chars(b"%clk");
    /// Summary field.
    pub const FIELD_SUMMARY: Self = Self::from_chars(b"%smr");
    /// User info field.
    pub const FIELD_USER_INFO: Self = Self::from_chars(b"%usr");
    /// Hyperlink field.
    pub const FIELD_HYPERLINK: Self = Self::from_chars(b"%hlk");
    /// Cross reference field.
    pub const FIELD_CROSS_REFERENCE: Self = Self::from_chars(b"%xrf");
    /// Private info field.
    pub const FIELD_PRIVATE_INFO: Self = Self::from_chars(b"%prv");
    /// Meta tag field.
    pub const FIELD_META_TAG: Self = Self::from_chars(b"%mtg");

    /// Creates from 4-character code.
    pub const fn from_chars(chars: &[u8; 4]) -> Self {
        Self(u32::from_le_bytes(*chars))
    }

    /// Creates from raw u32 value.
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }

    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self(reader.read_u32()?))
    }

    /// Returns the raw u32 value.
    pub const fn raw(&self) -> u32 {
        self.0
    }

    /// Returns as 4-character string.
    pub fn as_str(&self) -> String {
        let bytes = self.0.to_le_bytes();
        String::from_utf8_lossy(&bytes).to_string()
    }

    /// Returns the control type.
    pub fn control_type(&self) -> ControlType {
        match *self {
            Self::SECTION_DEFINITION => ControlType::SectionDefinition,
            Self::COLUMN_DEFINITION => ControlType::ColumnDefinition,
            Self::TABLE => ControlType::Table,
            Self::EQUATION => ControlType::Equation,
            Self::SHAPE_CONTAINER => ControlType::DrawingObject,
            Self::HEADER => ControlType::Header,
            Self::FOOTER => ControlType::Footer,
            Self::FOOTNOTE => ControlType::Footnote,
            Self::ENDNOTE => ControlType::Endnote,
            Self::AUTO_NUMBER => ControlType::AutoNumber,
            Self::NEW_NUMBER => ControlType::NewNumber,
            Self::PAGE_NUMBER_POSITION => ControlType::PageNumberPosition,
            Self::PAGE_HIDE => ControlType::PageHide,
            Self::PAGE_ODD_EVEN => ControlType::PageOddEven,
            Self::BOOKMARK => ControlType::Bookmark,
            Self::INDEX_MARK => ControlType::IndexMark,
            Self::HIDDEN_COMMENT => ControlType::HiddenComment,
            Self::TEXT_ART => ControlType::TextArt,
            Self::FORM_OBJECT => ControlType::FormObject,
            _ => {
                // Check if it's a field
                let bytes = self.0.to_le_bytes();
                if bytes[0] == b'%' {
                    ControlType::Field
                } else {
                    ControlType::Unknown
                }
            }
        }
    }

    /// Returns true if this is a field control.
    pub fn is_field(&self) -> bool {
        let bytes = self.0.to_le_bytes();
        bytes[0] == b'%'
    }
}

impl std::fmt::Debug for ControlId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ControlId(\"{}\" = 0x{:08X})", self.as_str(), self.0)
    }
}

/// A control element in the document.
#[derive(Debug, Clone)]
pub struct Control {
    /// Control ID.
    id: ControlId,
    /// Raw control data.
    data: Vec<u8>,
    /// Parsed control content (table, shape, picture, etc.).
    content: Option<ControlContent>,
    /// Child paragraphs (for controls that contain text).
    children: Vec<super::paragraph::Paragraph>,
}

impl Control {
    /// Creates a new control.
    pub fn new(id: ControlId) -> Self {
        Self {
            id,
            data: Vec::new(),
            content: None,
            children: Vec::new(),
        }
    }

    /// Creates a control with data.
    pub fn with_data(id: ControlId, data: Vec<u8>) -> Self {
        Self {
            id,
            data,
            content: None,
            children: Vec::new(),
        }
    }

    /// Returns the control ID.
    pub const fn id(&self) -> ControlId {
        self.id
    }

    /// Returns the control type.
    pub fn control_type(&self) -> ControlType {
        self.id.control_type()
    }

    /// Returns the raw data.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the parsed content if available.
    pub fn content(&self) -> Option<&ControlContent> {
        self.content.as_ref()
    }

    /// Returns a mutable reference to the parsed content.
    pub fn content_mut(&mut self) -> Option<&mut ControlContent> {
        self.content.as_mut()
    }

    /// Sets the parsed content.
    pub fn set_content(&mut self, content: ControlContent) {
        self.content = Some(content);
    }

    /// Returns child paragraphs.
    pub fn children(&self) -> &[super::paragraph::Paragraph] {
        &self.children
    }

    /// Adds a child paragraph.
    pub fn add_child(&mut self, paragraph: super::paragraph::Paragraph) {
        self.children.push(paragraph);
    }

    // === Convenience accessors for specific content types ===

    /// Returns the table content if this control contains a table.
    pub fn as_table(&self) -> Option<&Table> {
        match &self.content {
            Some(ControlContent::Table(table)) => Some(table),
            _ => None,
        }
    }

    /// Returns a mutable reference to the table content.
    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        match &mut self.content {
            Some(ControlContent::Table(table)) => Some(table),
            _ => None,
        }
    }

    /// Returns the shape content if this control contains a shape.
    pub fn as_shape(&self) -> Option<&Shape> {
        match &self.content {
            Some(ControlContent::Shape(shape)) => Some(shape),
            _ => None,
        }
    }

    /// Returns a mutable reference to the shape content.
    pub fn as_shape_mut(&mut self) -> Option<&mut Shape> {
        match &mut self.content {
            Some(ControlContent::Shape(shape)) => Some(shape),
            _ => None,
        }
    }

    /// Returns the picture content if this control contains a picture.
    pub fn as_picture(&self) -> Option<&Picture> {
        match &self.content {
            Some(ControlContent::Picture(picture)) => Some(picture),
            _ => None,
        }
    }

    /// Returns the equation content if this control contains an equation.
    pub fn as_equation(&self) -> Option<&Equation> {
        match &self.content {
            Some(ControlContent::Equation(equation)) => Some(equation),
            _ => None,
        }
    }

    /// Returns the OLE object content if this control contains an OLE object.
    pub fn as_ole_object(&self) -> Option<&OleObject> {
        match &self.content {
            Some(ControlContent::OleObject(ole)) => Some(ole),
            _ => None,
        }
    }

    /// Returns the header content if this control contains a header.
    pub fn as_header(&self) -> Option<&Header> {
        match &self.content {
            Some(ControlContent::Header(header)) => Some(header),
            _ => None,
        }
    }

    /// Returns a mutable reference to the header content.
    pub fn as_header_mut(&mut self) -> Option<&mut Header> {
        match &mut self.content {
            Some(ControlContent::Header(header)) => Some(header),
            _ => None,
        }
    }

    /// Returns the footer content if this control contains a footer.
    pub fn as_footer(&self) -> Option<&Footer> {
        match &self.content {
            Some(ControlContent::Footer(footer)) => Some(footer),
            _ => None,
        }
    }

    /// Returns a mutable reference to the footer content.
    pub fn as_footer_mut(&mut self) -> Option<&mut Footer> {
        match &mut self.content {
            Some(ControlContent::Footer(footer)) => Some(footer),
            _ => None,
        }
    }

    /// Returns the footnote content if this control contains a footnote.
    pub fn as_footnote(&self) -> Option<&Footnote> {
        match &self.content {
            Some(ControlContent::Footnote(footnote)) => Some(footnote),
            _ => None,
        }
    }

    /// Returns a mutable reference to the footnote content.
    pub fn as_footnote_mut(&mut self) -> Option<&mut Footnote> {
        match &mut self.content {
            Some(ControlContent::Footnote(footnote)) => Some(footnote),
            _ => None,
        }
    }

    /// Returns the endnote content if this control contains an endnote.
    pub fn as_endnote(&self) -> Option<&Endnote> {
        match &self.content {
            Some(ControlContent::Endnote(endnote)) => Some(endnote),
            _ => None,
        }
    }

    /// Returns a mutable reference to the endnote content.
    pub fn as_endnote_mut(&mut self) -> Option<&mut Endnote> {
        match &mut self.content {
            Some(ControlContent::Endnote(endnote)) => Some(endnote),
            _ => None,
        }
    }

    /// Returns the hyperlink content if this control contains a hyperlink.
    pub fn as_hyperlink(&self) -> Option<&Hyperlink> {
        match &self.content {
            Some(ControlContent::Hyperlink(hyperlink)) => Some(hyperlink),
            _ => None,
        }
    }

    /// Returns a mutable reference to the hyperlink content.
    pub fn as_hyperlink_mut(&mut self) -> Option<&mut Hyperlink> {
        match &mut self.content {
            Some(ControlContent::Hyperlink(hyperlink)) => Some(hyperlink),
            _ => None,
        }
    }

    /// Returns the field content if this control contains a field.
    pub fn as_field(&self) -> Option<&Field> {
        match &self.content {
            Some(ControlContent::Field(field)) => Some(field),
            _ => None,
        }
    }

    /// Returns a mutable reference to the field content.
    pub fn as_field_mut(&mut self) -> Option<&mut Field> {
        match &mut self.content {
            Some(ControlContent::Field(field)) => Some(field),
            _ => None,
        }
    }

    /// Returns the text box content if this control contains a text box.
    pub fn as_text_box(&self) -> Option<&TextBox> {
        match &self.content {
            Some(ControlContent::TextBox(text_box)) => Some(text_box),
            _ => None,
        }
    }

    /// Returns a mutable reference to the text box content.
    pub fn as_text_box_mut(&mut self) -> Option<&mut TextBox> {
        match &mut self.content {
            Some(ControlContent::TextBox(text_box)) => Some(text_box),
            _ => None,
        }
    }

    /// Returns the caption content if this control contains a caption.
    pub fn as_caption(&self) -> Option<&Caption> {
        match &self.content {
            Some(ControlContent::Caption(caption)) => Some(caption),
            _ => None,
        }
    }

    /// Returns a mutable reference to the caption content.
    pub fn as_caption_mut(&mut self) -> Option<&mut Caption> {
        match &mut self.content {
            Some(ControlContent::Caption(caption)) => Some(caption),
            _ => None,
        }
    }

    /// Returns the memo content if this control contains a memo.
    pub fn as_memo(&self) -> Option<&Memo> {
        match &self.content {
            Some(ControlContent::Memo(memo)) => Some(memo),
            _ => None,
        }
    }

    /// Returns a mutable reference to the memo content.
    pub fn as_memo_mut(&mut self) -> Option<&mut Memo> {
        match &mut self.content {
            Some(ControlContent::Memo(memo)) => Some(memo),
            _ => None,
        }
    }

    /// Returns the form object content if this control contains a form object.
    pub fn as_form_object(&self) -> Option<&FormObject> {
        match &self.content {
            Some(ControlContent::FormObject(form)) => Some(form),
            _ => None,
        }
    }

    /// Returns a mutable reference to the form object content.
    pub fn as_form_object_mut(&mut self) -> Option<&mut FormObject> {
        match &mut self.content {
            Some(ControlContent::FormObject(form)) => Some(form),
            _ => None,
        }
    }

    /// Returns the text art content if this control contains text art.
    pub fn as_text_art(&self) -> Option<&TextArt> {
        match &self.content {
            Some(ControlContent::TextArt(art)) => Some(art),
            _ => None,
        }
    }

    /// Returns a mutable reference to the text art content.
    pub fn as_text_art_mut(&mut self) -> Option<&mut TextArt> {
        match &mut self.content {
            Some(ControlContent::TextArt(art)) => Some(art),
            _ => None,
        }
    }

    /// Returns the chart content if this control contains a chart.
    pub fn as_chart(&self) -> Option<&ChartData> {
        match &self.content {
            Some(ControlContent::Chart(chart)) => Some(chart),
            _ => None,
        }
    }

    /// Returns a mutable reference to the chart content.
    pub fn as_chart_mut(&mut self) -> Option<&mut ChartData> {
        match &mut self.content {
            Some(ControlContent::Chart(chart)) => Some(chart),
            _ => None,
        }
    }

    /// Returns the video content if this control contains a video.
    pub fn as_video(&self) -> Option<&VideoData> {
        match &self.content {
            Some(ControlContent::Video(video)) => Some(video),
            _ => None,
        }
    }

    /// Returns a mutable reference to the video content.
    pub fn as_video_mut(&mut self) -> Option<&mut VideoData> {
        match &mut self.content {
            Some(ControlContent::Video(video)) => Some(video),
            _ => None,
        }
    }

    /// Returns the container content if this control contains grouped shapes.
    pub fn as_container(&self) -> Option<&ShapeContainer> {
        match &self.content {
            Some(ControlContent::Container(container)) => Some(container),
            _ => None,
        }
    }

    /// Returns a mutable reference to the container content.
    pub fn as_container_mut(&mut self) -> Option<&mut ShapeContainer> {
        match &mut self.content {
            Some(ControlContent::Container(container)) => Some(container),
            _ => None,
        }
    }

    /// Returns the section definition content if this control contains a section definition.
    pub fn as_section_definition(&self) -> Option<&SectionDefinition> {
        match &self.content {
            Some(ControlContent::SectionDefinition(def)) => Some(def),
            _ => None,
        }
    }

    /// Returns a mutable reference to the section definition content.
    pub fn as_section_definition_mut(&mut self) -> Option<&mut SectionDefinition> {
        match &mut self.content {
            Some(ControlContent::SectionDefinition(def)) => Some(def),
            _ => None,
        }
    }

    /// Returns the column definition content if this control contains a column definition.
    pub fn as_column_definition(&self) -> Option<&ColumnDefinition> {
        match &self.content {
            Some(ControlContent::ColumnDefinition(def)) => Some(def),
            _ => None,
        }
    }

    /// Returns a mutable reference to the column definition content.
    pub fn as_column_definition_mut(&mut self) -> Option<&mut ColumnDefinition> {
        match &mut self.content {
            Some(ControlContent::ColumnDefinition(def)) => Some(def),
            _ => None,
        }
    }

    /// Extracts plain text from the control content.
    pub fn plain_text(&self) -> String {
        match &self.content {
            Some(ControlContent::TextBox(tb)) => tb.plain_text(),
            Some(ControlContent::Caption(c)) => c.plain_text(),
            Some(ControlContent::Memo(m)) => m.plain_text(),
            Some(ControlContent::Header(h)) => h.plain_text(),
            Some(ControlContent::Footer(f)) => f.plain_text(),
            Some(ControlContent::Footnote(fn_)) => fn_.plain_text(),
            Some(ControlContent::Endnote(en)) => en.plain_text(),
            Some(ControlContent::TextArt(art)) => art.text().to_string(),
            Some(ControlContent::Table(table)) => table.plain_text(),
            Some(ControlContent::Container(c)) => c.plain_text(),
            _ => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_id_from_chars() {
        let id = ControlId::from_chars(b"tbl ");
        assert_eq!(id, ControlId::TABLE);
        assert_eq!(id.as_str(), "tbl ");
    }

    #[test]
    fn test_control_type_classification() {
        assert_eq!(ControlId::TABLE.control_type(), ControlType::Table);
        assert_eq!(ControlId::EQUATION.control_type(), ControlType::Equation);
        assert_eq!(
            ControlId::SHAPE_CONTAINER.control_type(),
            ControlType::DrawingObject
        );
        assert_eq!(ControlId::HEADER.control_type(), ControlType::Header);
        assert_eq!(ControlId::FOOTNOTE.control_type(), ControlType::Footnote);
    }

    #[test]
    fn test_field_control_detection() {
        assert!(ControlId::FIELD_DATE.is_field());
        assert!(ControlId::FIELD_HYPERLINK.is_field());
        assert!(!ControlId::TABLE.is_field());
    }

    #[test]
    fn test_control_content_accessors() {
        let mut control = Control::new(ControlId::TABLE);

        // Initially no content
        assert!(control.as_table().is_none());
        assert!(control.content().is_none());

        // Set table content
        let table = Table::default();
        control.set_content(ControlContent::Table(table));

        // Now has table content
        assert!(control.as_table().is_some());
        assert!(control.as_shape().is_none());
        assert!(control.as_picture().is_none());
        assert!(control.as_equation().is_none());

        // Can get mutable reference
        if let Some(table) = control.as_table_mut() {
            table.properties.row_count = 5;
        }
        assert_eq!(control.as_table().unwrap().row_count(), 5);
    }

    #[test]
    fn test_control_character_types() {
        assert_eq!(
            ControlCharacter::from_char(0x000A),
            Some(ControlCharacter::LineBreak)
        );
        assert_eq!(
            ControlCharacter::from_char(0x000D),
            Some(ControlCharacter::ParagraphBreak)
        );
        assert_eq!(
            ControlCharacter::from_char(0x0009),
            Some(ControlCharacter::Tab)
        );
        assert_eq!(
            ControlCharacter::from_char(0x000B),
            Some(ControlCharacter::DrawingTableObject)
        );
        assert_eq!(ControlCharacter::from_char(0x1234), None);
    }

    #[test]
    fn test_control_is_control_char() {
        assert!(ControlCharacter::is_control_char(0x000A)); // Line break
        assert!(ControlCharacter::is_control_char(0x000D)); // Paragraph break
        assert!(ControlCharacter::is_control_char(0x0009)); // Tab
        assert!(!ControlCharacter::is_control_char(0x0041)); // 'A' - not a control char
    }
}
