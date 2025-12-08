//! Document information (DocInfo) stream parsing.
//!
//! The DocInfo stream contains document-level information including:
//! - Document properties (section count, numbering, etc.)
//! - ID mappings (fonts, styles, borders, etc.)
//! - Font definitions
//! - Character and paragraph shapes
//! - Styles
//! - And more

mod binary_data;
mod border_fill;
mod bullet;
mod character_shape;
mod compatible_document;
mod distribute_doc_data;
mod document_data;
mod document_properties;
mod face_name;
mod forbidden_char;
mod id_mappings;
mod layout_compatibility;
mod numbering;
mod paragraph_shape;
mod style;
mod tab_definition;
mod track_change;

// 공개 API - HWP 스펙에 정의된 타입들
pub use binary_data::BinaryData;
pub use border_fill::{
    BorderFill, BorderLineStyle, BorderLineThickness, DiagonalType, FillInfo, FillType,
    GradientFill, GradientType, ImageFill, ImageFillType, ImageInfo, PatternFill, PatternType,
};
pub use bullet::Bullet;
pub use character_shape::{
    CharacterShape, EmphasisType, LanguageType, OutlineType, ShadowType, StrikethroughShape,
    UnderlinePosition, UnderlineShape,
};
pub use compatible_document::CompatibleDocument;
pub use document_properties::DocumentProperties;
pub use face_name::FaceName;
pub use id_mappings::IdMappings;
pub use layout_compatibility::LayoutCompatibility;
pub use numbering::{Numbering, NumberingLevel, ParagraphHeadAlignment, ParagraphHeadInfo};
pub use paragraph_shape::ParagraphShape;
pub use style::Style;
pub use tab_definition::TabDefinition;
pub use track_change::{TrackChangeAuthor, TrackChangeContent, TrackChangeInfo};
pub use distribute_doc_data::DistributeDocData;
pub use document_data::DocumentData;
pub use forbidden_char::ForbiddenChar;

use crate::error::Result;
use crate::primitive::{RecordHeader, RecordTagId};
use crate::util::ByteReader;

/// 레코드 헤더를 파싱한다.
///
/// 크기 필드가 0xFFF인 경우 확장 크기를 처리한다.
fn parse_record_header(reader: &mut ByteReader) -> Result<RecordHeader> {
    let raw = reader.read_u32()?;
    let header = RecordHeader::new(raw);

    if header.has_extended_size() {
        let extended_size = reader.read_u32()?;
        Ok(RecordHeader::with_extended_size(raw, extended_size))
    } else {
        Ok(header)
    }
}

/// Parsed DocInfo containing all document-level information.
#[derive(Debug, Clone, Default)]
pub struct DocInfo {
    /// Document properties.
    pub document_properties: Option<DocumentProperties>,
    /// ID mappings.
    pub id_mappings: Option<IdMappings>,
    /// Binary data items.
    pub binary_data: Vec<BinaryData>,
    /// Font face names.
    pub face_names: Vec<FaceName>,
    /// Border/fill definitions.
    pub border_fills: Vec<BorderFill>,
    /// Character shapes.
    pub character_shapes: Vec<CharacterShape>,
    /// Tab definitions.
    pub tab_definitions: Vec<TabDefinition>,
    /// Numbering definitions.
    pub numberings: Vec<Numbering>,
    /// Bullet definitions.
    pub bullets: Vec<Bullet>,
    /// Paragraph shapes.
    pub paragraph_shapes: Vec<ParagraphShape>,
    /// Style definitions.
    pub styles: Vec<Style>,
    /// Compatible document settings.
    pub compatible_document: Option<CompatibleDocument>,
    /// Layout compatibility.
    pub layout_compatibility: Option<LayoutCompatibility>,
    /// Track change info (HWPTAG_TRACKCHANGE 0x020).
    pub track_change_info: Option<TrackChangeInfo>,
    /// Track change authors (HWPTAG_TRACK_CHANGE_AUTHOR 0x061).
    pub track_change_authors: Vec<TrackChangeAuthor>,
    /// Track change content (HWPTAG_TRACK_CHANGE 0x060).
    pub track_change_contents: Vec<TrackChangeContent>,
    /// Document arbitrary data (HWPTAG_DOC_DATA 0x01B).
    pub document_data: Option<DocumentData>,
    /// Forbidden characters (HWPTAG_FORBIDDEN_CHAR 0x05E).
    pub forbidden_char: Option<ForbiddenChar>,
    /// Distribution document data (HWPTAG_DISTRIBUTE_DOC_DATA 0x01C).
    pub distribute_doc_data: Option<DistributeDocData>,
}

impl DocInfo {
    /// Parses DocInfo from bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - The decompressed DocInfo stream data
    ///
    /// # Returns
    ///
    /// The parsed DocInfo.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let mut reader = ByteReader::new(data);
        let mut doc_info = DocInfo::default();

        while !reader.is_empty() {
            // Try to parse record header - if not enough data, stop parsing
            let header = match parse_record_header(&mut reader) {
                Ok(h) => h,
                Err(_) => break, // Stop if we can't read a valid record header
            };

            // Check if we have enough data for the record - if not, stop gracefully
            let data_size = header.data_size() as usize;
            if reader.remaining() < data_size {
                // Not enough data remaining - stop parsing gracefully
                break;
            }

            let record_data = reader.read_bytes(data_size)?;
            let mut record_reader = ByteReader::new(record_data);

            match header.tag_id() {
                Some(RecordTagId::DocumentProperties) => {
                    doc_info.document_properties =
                        Some(DocumentProperties::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::IdMappings) => {
                    doc_info.id_mappings = Some(IdMappings::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::BinaryData) => {
                    doc_info
                        .binary_data
                        .push(BinaryData::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::FaceName) => {
                    doc_info
                        .face_names
                        .push(FaceName::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::BorderFill) => {
                    doc_info
                        .border_fills
                        .push(BorderFill::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::CharacterShape) => {
                    doc_info
                        .character_shapes
                        .push(CharacterShape::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::TabDefinition) => {
                    doc_info
                        .tab_definitions
                        .push(TabDefinition::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::Numbering) => {
                    doc_info
                        .numberings
                        .push(Numbering::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::Bullet) => {
                    doc_info
                        .bullets
                        .push(Bullet::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::ParagraphShape) => {
                    doc_info
                        .paragraph_shapes
                        .push(ParagraphShape::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::Style) => {
                    doc_info
                        .styles
                        .push(Style::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::CompatibleDocument) => {
                    doc_info.compatible_document =
                        Some(CompatibleDocument::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::LayoutCompatibility) => {
                    doc_info.layout_compatibility =
                        Some(LayoutCompatibility::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::TrackChange) => {
                    doc_info.track_change_info =
                        Some(TrackChangeInfo::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::TrackChangeAuthor) => {
                    doc_info
                        .track_change_authors
                        .push(TrackChangeAuthor::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::TrackChangeContent) => {
                    doc_info
                        .track_change_contents
                        .push(TrackChangeContent::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::DocumentData) => {
                    doc_info.document_data =
                        Some(DocumentData::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::ForbiddenCharacter) => {
                    doc_info.forbidden_char =
                        Some(ForbiddenChar::from_reader(&mut record_reader)?);
                }
                Some(RecordTagId::DistributeDocumentData) => {
                    doc_info.distribute_doc_data =
                        Some(DistributeDocData::from_reader(&mut record_reader)?);
                }
                _ => {
                    // Skip unknown records
                }
            }
        }

        Ok(doc_info)
    }
}
