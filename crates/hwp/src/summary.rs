//! Summary information parsing.
//!
//! The Summary information is stored in OLE Document Summary Information
//! property sets as defined by Microsoft's Compound File format.

use crate::error::Result;

/// Document summary information.
///
/// Contains metadata about the document such as title, subject, author, etc.
#[derive(Debug, Clone, Default)]
pub struct SummaryInfo {
    /// Document title.
    pub title: Option<String>,
    /// Document subject.
    pub subject: Option<String>,
    /// Document author.
    pub author: Option<String>,
    /// Keywords.
    pub keywords: Option<String>,
    /// Comments.
    pub comments: Option<String>,
    /// Last author (who last modified).
    pub last_author: Option<String>,
    /// Application name.
    pub application_name: Option<String>,
    /// Creation date.
    pub creation_date: Option<String>,
    /// Last saved date.
    pub last_saved_date: Option<String>,
}

impl SummaryInfo {
    /// OLE property IDs for DocumentSummaryInformation.
    const PIDSI_TITLE: u32 = 0x02;
    const PIDSI_SUBJECT: u32 = 0x03;
    const PIDSI_AUTHOR: u32 = 0x04;
    const PIDSI_KEYWORDS: u32 = 0x05;
    const PIDSI_COMMENTS: u32 = 0x06;
    const PIDSI_LAST_AUTHOR: u32 = 0x08;
    const PIDSI_APPNAME: u32 = 0x12;

    /// Parses summary information from bytes.
    ///
    /// The format follows Microsoft's Property Set format:
    /// - Byte order mark (0xFFFE)
    /// - Format version
    /// - OS version
    /// - Class ID (16 bytes)
    /// - Number of property sections
    /// - Section FMTID and offset pairs
    /// - Property sections
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < 28 {
            return Ok(Self::default());
        }

        // Check byte order mark
        let byte_order = u16::from_le_bytes([data[0], data[1]]);
        if byte_order != 0xFFFE {
            return Ok(Self::default());
        }

        // Skip header and read section count
        let section_count = u32::from_le_bytes([data[24], data[25], data[26], data[27]]) as usize;
        if section_count == 0 {
            return Ok(Self::default());
        }

        // Each section header is 20 bytes (16 byte FMTID + 4 byte offset)
        let first_section_offset_pos = 28 + 16; // After first FMTID
        if data.len() < first_section_offset_pos + 4 {
            return Ok(Self::default());
        }

        let section_offset = u32::from_le_bytes([
            data[first_section_offset_pos],
            data[first_section_offset_pos + 1],
            data[first_section_offset_pos + 2],
            data[first_section_offset_pos + 3],
        ]) as usize;

        if section_offset >= data.len() {
            return Ok(Self::default());
        }

        Self::parse_property_section(&data[section_offset..])
    }

    fn parse_property_section(data: &[u8]) -> Result<Self> {
        if data.len() < 8 {
            return Ok(Self::default());
        }

        let _section_size = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let property_count = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;

        let mut info = SummaryInfo::default();

        // Property ID/offset pairs start at byte 8
        for i in 0..property_count {
            let pair_offset = 8 + i * 8;
            if data.len() < pair_offset + 8 {
                break;
            }

            let property_id = u32::from_le_bytes([
                data[pair_offset],
                data[pair_offset + 1],
                data[pair_offset + 2],
                data[pair_offset + 3],
            ]);

            let value_offset = u32::from_le_bytes([
                data[pair_offset + 4],
                data[pair_offset + 5],
                data[pair_offset + 6],
                data[pair_offset + 7],
            ]) as usize;

            if value_offset >= data.len() {
                continue;
            }

            if let Some(value) = Self::read_property_value(&data[value_offset..]) {
                match property_id {
                    Self::PIDSI_TITLE => info.title = Some(value),
                    Self::PIDSI_SUBJECT => info.subject = Some(value),
                    Self::PIDSI_AUTHOR => info.author = Some(value),
                    Self::PIDSI_KEYWORDS => info.keywords = Some(value),
                    Self::PIDSI_COMMENTS => info.comments = Some(value),
                    Self::PIDSI_LAST_AUTHOR => info.last_author = Some(value),
                    Self::PIDSI_APPNAME => info.application_name = Some(value),
                    _ => {}
                }
            }
        }

        Ok(info)
    }

    fn read_property_value(data: &[u8]) -> Option<String> {
        if data.len() < 8 {
            return None;
        }

        let property_type = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

        match property_type {
            // VT_LPSTR (30)
            30 => {
                let string_len =
                    u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
                if data.len() >= 8 + string_len && string_len > 0 {
                    // Try to decode as UTF-8, falling back to latin1
                    let bytes = &data[8..8 + string_len - 1]; // Exclude null terminator
                    Some(String::from_utf8_lossy(bytes).into_owned())
                } else {
                    None
                }
            }
            // VT_LPWSTR (31)
            31 => {
                let char_count =
                    u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
                let byte_len = char_count * 2;
                if data.len() >= 8 + byte_len && char_count > 0 {
                    let mut chars = Vec::with_capacity(char_count);
                    for i in 0..(char_count - 1) {
                        // Exclude null terminator
                        let offset = 8 + i * 2;
                        let code_unit = u16::from_le_bytes([data[offset], data[offset + 1]]);
                        chars.push(code_unit);
                    }
                    Some(String::from_utf16_lossy(&chars))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns true if the summary info has any data.
    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.subject.is_none()
            && self.author.is_none()
            && self.keywords.is_none()
            && self.comments.is_none()
            && self.last_author.is_none()
            && self.application_name.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_summary() {
        let data = [];
        let info = SummaryInfo::from_bytes(&data).unwrap();
        assert!(info.is_empty());
    }
}
