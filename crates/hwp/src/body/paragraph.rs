//! Paragraph record structures.
//!
//! A paragraph consists of:
//! - Paragraph header (character count, shape references)
//! - Paragraph text (actual text content)
//! - Character shape references (which shapes apply to which ranges)
//! - Line segments (layout information)
//! - Range tags (bookmarks, hyperlinks, etc.)
//! - Controls (tables, shapes, etc.)

use crate::error::Result;
use crate::util::ByteReader;
use primitive::HwpUnit;

use super::control::Control;

/// Break type flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BreakType(u8);

impl BreakType {
    /// Section break.
    pub const SECTION: u8 = 0x01;
    /// Multi-column break.
    pub const MULTI_COLUMN: u8 = 0x02;
    /// Page break.
    pub const PAGE: u8 = 0x04;
    /// Column break.
    pub const COLUMN: u8 = 0x08;

    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        Self(value)
    }

    /// Returns true if section break.
    pub const fn is_section_break(&self) -> bool {
        (self.0 & Self::SECTION) != 0
    }

    /// Returns true if multi-column break.
    pub const fn is_multi_column_break(&self) -> bool {
        (self.0 & Self::MULTI_COLUMN) != 0
    }

    /// Returns true if page break.
    pub const fn is_page_break(&self) -> bool {
        (self.0 & Self::PAGE) != 0
    }

    /// Returns true if column break.
    pub const fn is_column_break(&self) -> bool {
        (self.0 & Self::COLUMN) != 0
    }
}

/// Paragraph text content.
///
/// Contains the text as UTF-16LE characters. Control characters
/// (0-31) have special meanings in HWP format.
#[derive(Debug, Clone)]
pub struct ParagraphText {
    /// Raw UTF-16 character codes.
    raw_chars: Vec<u16>,
}

impl ParagraphText {
    /// Creates from raw UTF-16 characters.
    pub fn new(raw_chars: Vec<u16>) -> Self {
        Self { raw_chars }
    }

    /// Parses paragraph text from reader.
    pub fn from_reader(reader: &mut ByteReader, char_count: u32) -> Result<Self> {
        let mut raw_chars = Vec::with_capacity(char_count as usize);

        // Read char_count UTF-16 characters
        let mut remaining = char_count;
        while remaining > 0 {
            let ch = reader.read_u16()?;
            raw_chars.push(ch);
            remaining -= 1;

            // Extended control characters take additional space
            // inline controls: 8 extra WCHARs (char codes 4, 5, 6, 7, 8, 9, 10, 11)
            // extended controls: 12 extra WCHARs (char codes 1, 2, 3, 11, 12, 14, 15, 16, 17, 18, 21, 22, 23)
            if ch <= 31 {
                let extra = match ch {
                    // inline controls (4-10)
                    4..=10 => 7, // 8 total
                    // extended controls
                    1..=3 | 11 | 12 | 14..=18 | 21..=23 => 11, // 12 total
                    _ => 0,
                };
                for _ in 0..extra {
                    if remaining > 0 {
                        raw_chars.push(reader.read_u16()?);
                        remaining -= 1;
                    }
                }
            }
        }

        Ok(Self { raw_chars })
    }

    /// Returns the raw UTF-16 characters.
    pub fn raw_chars(&self) -> &[u16] {
        &self.raw_chars
    }

    /// Converts to a plain text string, skipping control characters.
    pub fn to_plain_text(&self) -> String {
        let mut result = String::new();
        let mut i = 0;

        while i < self.raw_chars.len() {
            let ch = self.raw_chars[i];

            if ch <= 31 {
                // Skip control characters and their extra data
                let extra = match ch {
                    4..=10 => 7,
                    1..=3 | 11 | 12 | 14..=18 | 21..=23 => 11,
                    _ => 0,
                };
                i += 1 + extra;

                // Add appropriate replacements
                match ch {
                    10 => result.push('\t'), // Tab
                    13 => result.push('\n'), // Paragraph break
                    24 => result.push(' '),  // Non-breaking space
                    30 => result.push('-'),  // Non-breaking hyphen
                    31 => result.push(' '),  // Fixed-width space
                    _ => {}
                }
            } else {
                // Regular character - decode UTF-16
                if let Some(c) = char::from_u32(ch as u32) {
                    result.push(c);
                } else if (0xD800..=0xDBFF).contains(&ch) && i + 1 < self.raw_chars.len() {
                    // High surrogate - combine with low surrogate
                    let high = ch;
                    let low = self.raw_chars[i + 1];
                    if (0xDC00..=0xDFFF).contains(&low) {
                        let code_point =
                            0x10000 + ((high as u32 - 0xD800) << 10) + (low as u32 - 0xDC00);
                        if let Some(c) = char::from_u32(code_point) {
                            result.push(c);
                        }
                        i += 1;
                    }
                }
                i += 1;
            }
        }

        result
    }

    /// Returns true if this paragraph text is empty or contains only a paragraph break.
    pub fn is_empty(&self) -> bool {
        self.raw_chars.is_empty()
            || (self.raw_chars.len() == 1 && self.raw_chars[0] == 13)
    }
}

/// Character shape reference in a paragraph.
///
/// Maps a position in the paragraph text to a character shape ID.
#[derive(Debug, Clone, Copy)]
pub struct CharacterShapeReference {
    /// Position in text where this shape starts.
    pub position: u32,
    /// Character shape ID (reference to DocInfo).
    pub character_shape_id: u32,
}

impl CharacterShapeReference {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            position: reader.read_u32()?,
            character_shape_id: reader.read_u32()?,
        })
    }
}

/// Line segment (layout cache information).
#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    /// Text start position.
    pub text_start_position: u32,
    /// Vertical position of the line.
    pub vertical_position: HwpUnit,
    /// Line height.
    pub line_height: HwpUnit,
    /// Text part height.
    pub text_height: HwpUnit,
    /// Distance from vertical position to baseline.
    pub baseline_distance: HwpUnit,
    /// Line spacing.
    pub line_spacing: HwpUnit,
    /// Start position in column.
    pub column_start_position: HwpUnit,
    /// Segment width.
    pub segment_width: HwpUnit,
    /// Tag value.
    pub tag: u32,
}

impl LineSegment {
    /// Size of a line segment record in bytes.
    pub const SIZE: usize = 36;

    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            text_start_position: reader.read_u32()?,
            vertical_position: reader.read_signed_hwp_unit()?,
            line_height: reader.read_signed_hwp_unit()?,
            text_height: reader.read_signed_hwp_unit()?,
            baseline_distance: reader.read_signed_hwp_unit()?,
            line_spacing: reader.read_signed_hwp_unit()?,
            column_start_position: reader.read_signed_hwp_unit()?,
            segment_width: reader.read_signed_hwp_unit()?,
            tag: reader.read_u32()?,
        })
    }
}

/// Range tag for bookmarks, hyperlinks, etc.
#[derive(Debug, Clone, Copy)]
pub struct RangeTag {
    /// Start position.
    pub start_position: u32,
    /// End position.
    pub end_position: u32,
    /// Tag data (interpretation depends on type).
    pub tag: [u8; 3],
}

impl RangeTag {
    /// Size of a range tag record in bytes.
    pub const SIZE: usize = 12;

    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let start_position = reader.read_u32()?;
        let end_position = reader.read_u32()?;
        let tag_word = reader.read_u32()?;
        let tag = [
            tag_word as u8,
            (tag_word >> 8) as u8,
            (tag_word >> 16) as u8,
        ];
        Ok(Self {
            start_position,
            end_position,
            tag,
        })
    }
}

/// A paragraph in the document.
#[derive(Debug, Clone)]
pub struct Paragraph {
    /// Number of characters in the paragraph.
    character_count: u32,
    /// Control mask flags.
    control_mask: u32,
    /// Paragraph shape ID reference.
    paragraph_shape_id: u16,
    /// Style ID reference.
    style_id: u8,
    /// Break type flags.
    break_type: BreakType,
    /// Instance ID (unique).
    instance_id: u32,
    /// Track change merge flag (version 5.0.3.2+).
    track_change_merge: Option<u16>,
    /// Paragraph text content.
    text: Option<ParagraphText>,
    /// Character shape references.
    character_shape_references: Vec<CharacterShapeReference>,
    /// Line segments (layout cache).
    line_segments: Vec<LineSegment>,
    /// Range tags.
    range_tags: Vec<RangeTag>,
    /// Controls in this paragraph.
    controls: Vec<Control>,
}

impl Paragraph {
    /// Creates a new paragraph from header data.
    pub fn new(
        character_count: u32,
        control_mask: u32,
        paragraph_shape_id: u16,
        style_id: u8,
        break_type: BreakType,
        instance_id: u32,
        track_change_merge: Option<u16>,
    ) -> Self {
        Self {
            character_count,
            control_mask,
            paragraph_shape_id,
            style_id,
            break_type,
            instance_id,
            track_change_merge,
            text: None,
            character_shape_references: Vec::new(),
            line_segments: Vec::new(),
            range_tags: Vec::new(),
            controls: Vec::new(),
        }
    }

    /// Parses paragraph header from reader.
    pub fn from_reader(reader: &mut ByteReader, data_size: u32) -> Result<Self> {
        // Character count (with possible flag in high bit)
        let raw_char_count = reader.read_u32()?;
        let character_count = raw_char_count & 0x7FFFFFFF;

        let control_mask = reader.read_u32()?;
        let paragraph_shape_id = reader.read_u16()?;
        let style_id = reader.read_u8()?;
        let break_type = BreakType::from_raw(reader.read_u8()?);

        let _char_shape_count = reader.read_u16()?;
        let _range_tag_count = reader.read_u16()?;
        let _line_seg_count = reader.read_u16()?;

        let instance_id = reader.read_u32()?;

        // Track change merge (version 5.0.3.2+, appears when size >= 24)
        let track_change_merge = if data_size >= 24 {
            Some(reader.read_u16()?)
        } else {
            None
        };

        Ok(Self::new(
            character_count,
            control_mask,
            paragraph_shape_id,
            style_id,
            break_type,
            instance_id,
            track_change_merge,
        ))
    }

    /// Returns the character count.
    pub const fn character_count(&self) -> u32 {
        self.character_count
    }

    /// Returns the control mask.
    pub const fn control_mask(&self) -> u32 {
        self.control_mask
    }

    /// Returns the paragraph shape ID.
    pub const fn paragraph_shape_id(&self) -> u16 {
        self.paragraph_shape_id
    }

    /// Returns the style ID.
    pub const fn style_id(&self) -> u8 {
        self.style_id
    }

    /// Returns the break type.
    pub const fn break_type(&self) -> BreakType {
        self.break_type
    }

    /// Returns the instance ID.
    pub const fn instance_id(&self) -> u32 {
        self.instance_id
    }

    /// Returns the track change merge flag (version 5.0.3.2+).
    pub const fn track_change_merge(&self) -> Option<u16> {
        self.track_change_merge
    }

    /// Returns the text content.
    pub fn text(&self) -> Option<&ParagraphText> {
        self.text.as_ref()
    }

    /// Sets the text content.
    pub fn set_text(&mut self, text: ParagraphText) {
        self.text = Some(text);
    }

    /// Returns character shape references.
    pub fn character_shape_references(&self) -> &[CharacterShapeReference] {
        &self.character_shape_references
    }

    /// Adds a character shape reference.
    pub fn add_character_shape_reference(&mut self, reference: CharacterShapeReference) {
        self.character_shape_references.push(reference);
    }

    /// Returns line segments.
    pub fn line_segments(&self) -> &[LineSegment] {
        &self.line_segments
    }

    /// Adds a line segment.
    pub fn add_line_segment(&mut self, segment: LineSegment) {
        self.line_segments.push(segment);
    }

    /// Returns range tags.
    pub fn range_tags(&self) -> &[RangeTag] {
        &self.range_tags
    }

    /// Adds a range tag.
    pub fn add_range_tag(&mut self, tag: RangeTag) {
        self.range_tags.push(tag);
    }

    /// Returns controls.
    pub fn controls(&self) -> &[Control] {
        &self.controls
    }

    /// Adds a control.
    pub fn add_control(&mut self, control: Control) {
        self.controls.push(control);
    }

    /// Extracts plain text from this paragraph.
    pub fn plain_text(&self) -> String {
        self.text
            .as_ref()
            .map(|t| t.to_plain_text())
            .unwrap_or_default()
    }
}
