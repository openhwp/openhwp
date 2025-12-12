//! Track change (revision tracking) parsing.
//!
//! Track change records store information about document revisions,
//! including author information and change content.

use crate::error::Result;
use crate::util::ByteReader;

/// Track change operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrackChangeType {
    /// Unknown operation.
    #[default]
    Unknown,
    /// Text insertion.
    Insert,
    /// Text deletion.
    Delete,
    /// Format change.
    Format,
}

impl TrackChangeType {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Insert,
            1 => Self::Delete,
            2 => Self::Format,
            _ => Self::Unknown,
        }
    }
}

/// Track change author information.
///
/// Stores information about who made a tracked change.
#[derive(Debug, Clone, Default)]
pub struct TrackChangeAuthor {
    /// Author name.
    name: String,
    /// Author ID (internal identifier).
    author_id: u32,
}

impl TrackChangeAuthor {
    /// Creates an author with name.
    pub const fn with_name(name: String) -> Self {
        Self { name, author_id: 0 }
    }

    /// Returns the author name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the author name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns the author ID.
    pub const fn author_id(&self) -> u32 {
        self.author_id
    }

    /// Sets the author ID.
    pub fn set_author_id(&mut self, id: u32) {
        self.author_id = id;
    }

    /// Parses track change author from reader.
    ///
    /// Format (per HWP spec - HWPTAG_TRACK_CHANGE_AUTHOR):
    /// - WCHAR[]: Author name (length-prefixed)
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        // Read author name (length-prefixed UTF-16)
        let name = if !reader.is_empty() && reader.remaining() >= 2 {
            let name_len = reader.read_u16()? as usize;
            if name_len > 0 && reader.remaining() >= name_len * 2 {
                let mut chars = Vec::with_capacity(name_len);
                for _ in 0..name_len {
                    chars.push(reader.read_u16()?);
                }
                String::from_utf16_lossy(&chars)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Read author ID if available
        let author_id = if reader.remaining() >= 4 {
            reader.read_u32()?
        } else {
            0
        };

        Ok(Self { name, author_id })
    }
}

/// Track change content and shape information.
///
/// Contains details about what was changed.
#[derive(Debug, Clone, Default)]
pub struct TrackChangeContent {
    /// Change type (insert, delete, format).
    change_type: TrackChangeType,
    /// Author index (reference to TrackChangeAuthor).
    author_index: u16,
    /// Change date/time.
    timestamp: u32,
}

impl TrackChangeContent {
    /// Returns the change type.
    pub const fn change_type(&self) -> TrackChangeType {
        self.change_type
    }

    /// Sets the change type.
    pub fn set_change_type(&mut self, change_type: TrackChangeType) {
        self.change_type = change_type;
    }

    /// Returns the author index.
    pub const fn author_index(&self) -> u16 {
        self.author_index
    }

    /// Sets the author index.
    pub fn set_author_index(&mut self, index: u16) {
        self.author_index = index;
    }

    /// Returns the timestamp.
    pub const fn timestamp(&self) -> u32 {
        self.timestamp
    }

    /// Sets the timestamp.
    pub fn set_timestamp(&mut self, timestamp: u32) {
        self.timestamp = timestamp;
    }

    /// Parses track change content from reader.
    ///
    /// Format (per HWP spec - HWPTAG_TRACK_CHANGE):
    /// - Variable length data containing change details
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let change_type = if reader.remaining() >= 1 {
            TrackChangeType::from_raw(reader.read_u8()?)
        } else {
            TrackChangeType::Unknown
        };

        let author_index = if reader.remaining() >= 2 {
            reader.read_u16()?
        } else {
            0
        };

        let timestamp = if reader.remaining() >= 4 {
            reader.read_u32()?
        } else {
            0
        };

        Ok(Self {
            change_type,
            author_index,
            timestamp,
        })
    }
}

/// Track change information (main record in DocInfo).
///
/// HWPTAG_TRACKCHANGE (0x020) - 1032 bytes fixed size per spec.
/// Contains overall track change settings and metadata.
#[derive(Debug, Clone, Default)]
pub struct TrackChangeInfo {
    /// Track change enabled flag.
    enabled: bool,
    /// Show changes flag.
    show_changes: bool,
}

impl TrackChangeInfo {
    /// Returns whether track change is enabled.
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Sets the enabled flag.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Returns whether changes are shown.
    pub const fn show_changes(&self) -> bool {
        self.show_changes
    }

    /// Sets the show changes flag.
    pub fn set_show_changes(&mut self, show: bool) {
        self.show_changes = show;
    }

    /// Parses track change info from reader.
    ///
    /// Format (per HWP spec - HWPTAG_TRACKCHANGE):
    /// - Fixed 1032 bytes per spec
    /// - Actual field layout is not fully documented
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        // Read flags from first few bytes if available
        let flags = if reader.remaining() >= 4 {
            reader.read_u32()?
        } else {
            0
        };

        let enabled = (flags & 0x01) != 0;
        let show_changes = (flags & 0x02) != 0;

        // Skip remaining data (not fully documented in public HWP spec)
        //
        // ## Spec Limitation Note
        //
        // The HWP 5.0 public specification only documents basic TrackChange fields.
        // Real-world documents may contain additional proprietary fields for:
        // - Extended change metadata
        // - Change revision history
        // - Merge conflict markers
        //
        // These undocumented fields are skipped to avoid parsing corruption.
        // The core enabled/show_changes flags are parsed correctly.
        let remaining = reader.remaining();
        if remaining > 0 {
            let _ = reader.read_bytes(remaining)?;
        }

        Ok(Self {
            enabled,
            show_changes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_change_type_from_raw() {
        assert_eq!(TrackChangeType::from_raw(0), TrackChangeType::Insert);
        assert_eq!(TrackChangeType::from_raw(1), TrackChangeType::Delete);
        assert_eq!(TrackChangeType::from_raw(2), TrackChangeType::Format);
        assert_eq!(TrackChangeType::from_raw(255), TrackChangeType::Unknown);
    }

    #[test]
    fn test_track_change_author_default() {
        let author = TrackChangeAuthor::default();
        assert_eq!(author.name(), "");
        assert_eq!(author.author_id(), 0);
    }

    #[test]
    fn test_track_change_author_with_name() {
        let author = TrackChangeAuthor::with_name("홍길동".to_string());
        assert_eq!(author.name(), "홍길동");
    }

    #[test]
    fn test_track_change_content_default() {
        let content = TrackChangeContent::default();
        assert_eq!(content.change_type(), TrackChangeType::Unknown);
        assert_eq!(content.author_index(), 0);
        assert_eq!(content.timestamp(), 0);
    }

    #[test]
    fn test_track_change_info_default() {
        let info = TrackChangeInfo::default();
        assert!(!info.is_enabled());
        assert!(!info.show_changes());
    }

    #[test]
    fn test_track_change_info_setters() {
        let mut info = TrackChangeInfo::default();
        info.set_enabled(true);
        info.set_show_changes(true);
        assert!(info.is_enabled());
        assert!(info.show_changes());
    }
}
