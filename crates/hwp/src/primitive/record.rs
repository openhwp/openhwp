//! HWP record structure definitions.
//!
//! HWP documents use a tag-based record structure for storing data.
//! Each record has a 32-bit header containing tag ID, level, and size.

use std::fmt;

/// Record tag IDs used in HWP documents.
///
/// Tag IDs identify the type of data stored in a record.
/// The range 0x010-0x1FF is reserved for HWP internal use.
/// The range 0x200-0x3FF is available for external applications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum RecordTagId {
    // ============================================================
    // DocInfo records (0x010 - 0x05F)
    // ============================================================
    /// Document properties (section count, numbering, caret position).
    DocumentProperties = 0x010,

    /// ID mappings header (counts for fonts, styles, etc.).
    IdMappings = 0x011,

    /// Binary data item information.
    BinaryData = 0x012,

    /// Font face name and properties.
    FaceName = 0x013,

    /// Border and fill properties.
    BorderFill = 0x014,

    /// Character shape (font, size, color, etc.).
    CharacterShape = 0x015,

    /// Tab definition.
    TabDefinition = 0x016,

    /// Paragraph numbering definition.
    Numbering = 0x017,

    /// Bullet definition.
    Bullet = 0x018,

    /// Paragraph shape (margins, alignment, spacing).
    ParagraphShape = 0x019,

    /// Style definition.
    Style = 0x01A,

    /// Document-level arbitrary data.
    DocumentData = 0x01B,

    /// Distribution document data.
    DistributeDocumentData = 0x01C,

    // 0x01D is reserved

    /// Compatible document settings.
    CompatibleDocument = 0x01E,

    /// Layout compatibility settings.
    LayoutCompatibility = 0x01F,

    /// Track change information.
    TrackChange = 0x020,

    // 0x021 - 0x05B are reserved or version-specific

    /// Memo shape.
    MemoShape = 0x05C,

    // 0x05D is reserved

    /// Forbidden characters.
    ForbiddenCharacter = 0x05E,

    /// Track change content and shape.
    TrackChangeContent = 0x060,

    /// Track change author.
    TrackChangeAuthor = 0x061,

    // ============================================================
    // BodyText records (0x042 - 0x073)
    // ============================================================
    /// Paragraph header.
    ParagraphHeader = 0x042,

    /// Paragraph text content.
    ParagraphText = 0x043,

    /// Paragraph character shape references.
    ParagraphCharacterShape = 0x044,

    /// Paragraph line segment (layout cache).
    ParagraphLineSegment = 0x045,

    /// Paragraph range tag.
    ParagraphRangeTag = 0x046,

    /// Control header.
    ControlHeader = 0x047,

    /// List header (for controls containing paragraphs).
    ListHeader = 0x048,

    /// Page definition.
    PageDefinition = 0x049,

    /// Footnote/endnote shape.
    FootnoteShape = 0x04A,

    /// Page border and fill.
    PageBorderFill = 0x04B,

    /// Shape component (drawing object base).
    ShapeComponent = 0x04C,

    /// Table object.
    Table = 0x04D,

    /// Line shape.
    ShapeComponentLine = 0x04E,

    /// Rectangle shape.
    ShapeComponentRectangle = 0x04F,

    /// Ellipse shape.
    ShapeComponentEllipse = 0x050,

    /// Arc shape.
    ShapeComponentArc = 0x051,

    /// Polygon shape.
    ShapeComponentPolygon = 0x052,

    /// Curve shape.
    ShapeComponentCurve = 0x053,

    /// OLE object shape.
    ShapeComponentOle = 0x054,

    /// Picture shape.
    ShapeComponentPicture = 0x055,

    /// Container (grouped objects).
    ShapeComponentContainer = 0x056,

    /// Control arbitrary data.
    ControlData = 0x057,

    /// Equation editor.
    Equation = 0x058,

    // 0x059 is reserved

    /// Text art (WordArt-like).
    ShapeComponentTextArt = 0x05A,

    /// Form object.
    FormObject = 0x05B,

    // MemoShape = 0x05C (already defined above)

    /// Memo list header.
    MemoList = 0x05D,

    // ForbiddenCharacter = 0x05E (already defined above)

    /// Chart data.
    ChartData = 0x05F,

    // TrackChangeContent = 0x060 (already defined above)
    // TrackChangeAuthor = 0x061 (already defined above)

    /// Video data.
    VideoData = 0x062,

    /// Unknown shape component.
    ShapeComponentUnknown = 0x073,
}

#[allow(dead_code)]
impl RecordTagId {
    /// Base value for HWP internal tags.
    pub const HWPTAG_BEGIN: u16 = 0x010;

    /// Tries to convert a u16 value to a RecordTagId.
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0x010 => Some(Self::DocumentProperties),
            0x011 => Some(Self::IdMappings),
            0x012 => Some(Self::BinaryData),
            0x013 => Some(Self::FaceName),
            0x014 => Some(Self::BorderFill),
            0x015 => Some(Self::CharacterShape),
            0x016 => Some(Self::TabDefinition),
            0x017 => Some(Self::Numbering),
            0x018 => Some(Self::Bullet),
            0x019 => Some(Self::ParagraphShape),
            0x01A => Some(Self::Style),
            0x01B => Some(Self::DocumentData),
            0x01C => Some(Self::DistributeDocumentData),
            0x01E => Some(Self::CompatibleDocument),
            0x01F => Some(Self::LayoutCompatibility),
            0x020 => Some(Self::TrackChange),
            0x042 => Some(Self::ParagraphHeader),
            0x043 => Some(Self::ParagraphText),
            0x044 => Some(Self::ParagraphCharacterShape),
            0x045 => Some(Self::ParagraphLineSegment),
            0x046 => Some(Self::ParagraphRangeTag),
            0x047 => Some(Self::ControlHeader),
            0x048 => Some(Self::ListHeader),
            0x049 => Some(Self::PageDefinition),
            0x04A => Some(Self::FootnoteShape),
            0x04B => Some(Self::PageBorderFill),
            0x04C => Some(Self::ShapeComponent),
            0x04D => Some(Self::Table),
            0x04E => Some(Self::ShapeComponentLine),
            0x04F => Some(Self::ShapeComponentRectangle),
            0x050 => Some(Self::ShapeComponentEllipse),
            0x051 => Some(Self::ShapeComponentArc),
            0x052 => Some(Self::ShapeComponentPolygon),
            0x053 => Some(Self::ShapeComponentCurve),
            0x054 => Some(Self::ShapeComponentOle),
            0x055 => Some(Self::ShapeComponentPicture),
            0x056 => Some(Self::ShapeComponentContainer),
            0x057 => Some(Self::ControlData),
            0x058 => Some(Self::Equation),
            0x05A => Some(Self::ShapeComponentTextArt),
            0x05B => Some(Self::FormObject),
            0x05C => Some(Self::MemoShape),
            0x05D => Some(Self::MemoList),
            0x05E => Some(Self::ForbiddenCharacter),
            0x05F => Some(Self::ChartData),
            0x060 => Some(Self::TrackChangeContent),
            0x061 => Some(Self::TrackChangeAuthor),
            0x062 => Some(Self::VideoData),
            0x073 => Some(Self::ShapeComponentUnknown),
            _ => None,
        }
    }

    /// Returns the tag ID as u16.
    #[inline]
    pub const fn as_u16(self) -> u16 {
        self as u16
    }

    /// Returns true if this is a DocInfo record.
    #[inline]
    pub const fn is_doc_info(self) -> bool {
        let v = self as u16;
        v >= 0x010 && v <= 0x061
    }

    /// Returns true if this is a BodyText record.
    #[inline]
    pub const fn is_body_text(self) -> bool {
        let v = self as u16;
        v >= 0x042 && v <= 0x073
    }
}

impl fmt::Display for RecordTagId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Record header (32-bit).
///
/// The record header contains:
/// - Tag ID (10 bits): Type of the record
/// - Level (10 bits): Hierarchy depth
/// - Size (12 bits): Data size in bytes
///
/// If size == 0xFFF, an additional DWORD follows containing the actual size.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RecordHeader {
    /// The 32-bit header value.
    raw: u32,
    /// Extended size (if size field == 0xFFF).
    extended_size: Option<u32>,
}

impl RecordHeader {
    /// Size field value that indicates extended size follows.
    pub const EXTENDED_SIZE_MARKER: u32 = 0xFFF;

    /// Creates a new RecordHeader from raw 32-bit value.
    #[inline]
    pub const fn new(raw: u32) -> Self {
        Self {
            raw,
            extended_size: None,
        }
    }

    /// Creates a RecordHeader with extended size.
    #[inline]
    pub const fn with_extended_size(raw: u32, extended_size: u32) -> Self {
        Self {
            raw,
            extended_size: Some(extended_size),
        }
    }

    /// Returns the tag ID (10 bits, 0-1023).
    #[inline]
    pub const fn tag_id_raw(&self) -> u16 {
        (self.raw & 0x3FF) as u16
    }

    /// Returns the tag ID as enum, if recognized.
    #[inline]
    pub fn tag_id(&self) -> Option<RecordTagId> {
        RecordTagId::from_u16(self.tag_id_raw())
    }

    /// Returns the hierarchy level (10 bits, 0-1023).
    #[inline]
    pub const fn level(&self) -> u16 {
        ((self.raw >> 10) & 0x3FF) as u16
    }

    /// Returns the size field value (12 bits, 0-4095).
    #[inline]
    const fn size_field(&self) -> u32 {
        (self.raw >> 20) & 0xFFF
    }

    /// Returns true if this record uses extended size.
    #[inline]
    pub const fn has_extended_size(&self) -> bool {
        self.size_field() == Self::EXTENDED_SIZE_MARKER
    }

    /// Returns the actual data size in bytes.
    #[inline]
    pub const fn data_size(&self) -> u32 {
        match self.extended_size {
            Some(size) => size,
            None => self.size_field(),
        }
    }

    /// Returns the total header size in bytes (4 or 8).
    #[inline]
    #[allow(dead_code)]
    pub const fn header_size(&self) -> usize {
        if self.extended_size.is_some() {
            8
        } else {
            4
        }
    }

    /// Creates from little-endian bytes.
    #[inline]
    #[allow(dead_code)]
    pub fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self::new(u32::from_le_bytes(bytes))
    }
}

impl fmt::Debug for RecordHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecordHeader")
            .field("tag_id", &format_args!("0x{:03X}", self.tag_id_raw()))
            .field("tag", &self.tag_id())
            .field("level", &self.level())
            .field("data_size", &self.data_size())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_header_parsing() {
        // Tag ID = 0x010 (16), Level = 0, Size = 30
        // binary: 0000_0000_0001_1110 | 00_0000_0000 | 00_0001_0000
        //         size (12 bits)      level (10)     tag (10)
        // = 0x0001E010
        let header = RecordHeader::new(0x01E00010);
        assert_eq!(header.tag_id_raw(), 0x010);
        assert_eq!(header.tag_id(), Some(RecordTagId::DocumentProperties));
        assert_eq!(header.level(), 0);
        assert_eq!(header.data_size(), 30);
    }

    #[test]
    fn test_record_header_extended_size() {
        // Size field = 0xFFF indicates extended size
        let header = RecordHeader::with_extended_size(0xFFF00010, 10000);
        assert!(header.has_extended_size());
        assert_eq!(header.data_size(), 10000);
        assert_eq!(header.header_size(), 8);
    }

    #[test]
    fn test_record_tag_id_conversion() {
        assert_eq!(
            RecordTagId::from_u16(0x010),
            Some(RecordTagId::DocumentProperties)
        );
        assert_eq!(RecordTagId::from_u16(0x042), Some(RecordTagId::ParagraphHeader));
        assert_eq!(RecordTagId::from_u16(0xFFF), None);
    }
}
