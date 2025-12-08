//! Video data parsing.
//!
//! Video objects embed video content in the document.

use crate::error::Result;
use crate::util::ByteReader;

/// Video type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VideoType {
    /// Unknown video type.
    #[default]
    Unknown,
    /// Embedded video file.
    Embedded,
    /// Linked video (external URL).
    Linked,
    /// YouTube video.
    YouTube,
}

impl VideoType {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Embedded,
            1 => Self::Linked,
            2 => Self::YouTube,
            _ => Self::Unknown,
        }
    }
}

/// Video data in the document.
#[derive(Debug, Clone, Default)]
pub struct VideoData {
    /// Video type.
    pub video_type: VideoType,
    /// Video source (file path or URL).
    pub source: String,
    /// Binary data ID (for embedded videos).
    pub bin_data_id: Option<u32>,
    /// Poster image binary data ID.
    pub poster_bin_id: Option<u32>,
    /// Video width in HWP units.
    pub width: u32,
    /// Video height in HWP units.
    pub height: u32,
}

impl VideoData {
    /// Returns true if this is an embedded video.
    pub const fn is_embedded(&self) -> bool {
        matches!(self.video_type, VideoType::Embedded)
    }

    /// Returns true if this is a linked video.
    pub const fn is_linked(&self) -> bool {
        matches!(self.video_type, VideoType::Linked | VideoType::YouTube)
    }

    /// Parses video data from reader.
    ///
    /// Format (per HWP spec):
    /// - INT32: video type (0=local, 1=web)
    /// - For local: UINT16 video BinData ID, UINT16 thumbnail BinData ID
    /// - For web: WCHAR[] web tag, UINT16 thumbnail BinData ID
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let video_type_raw = reader.read_i32()?;

        let (video_type, bin_data_id, poster_bin_id, source) = if video_type_raw == 0 {
            // Local video
            let video_bin_id = reader.read_u16()? as u32;
            let thumbnail_bin_id = reader.read_u16()? as u32;
            (
                VideoType::Embedded,
                Some(video_bin_id),
                Some(thumbnail_bin_id),
                String::new(),
            )
        } else {
            // Web video - read web tag string
            let web_tag = if !reader.is_empty() && reader.remaining() > 2 {
                // Try to read as length-prefixed or null-terminated string
                let mut chars = Vec::new();
                while reader.remaining() >= 2 {
                    let ch = reader.read_u16()?;
                    if ch == 0 {
                        break;
                    }
                    chars.push(ch);
                }
                String::from_utf16_lossy(&chars)
            } else {
                String::new()
            };

            let thumbnail_bin_id = if reader.remaining() >= 2 {
                Some(reader.read_u16()? as u32)
            } else {
                None
            };

            (VideoType::Linked, None, thumbnail_bin_id, web_tag)
        };

        Ok(Self {
            video_type,
            source,
            bin_data_id,
            poster_bin_id,
            width: 0,
            height: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_type_from_raw() {
        assert_eq!(VideoType::from_raw(0), VideoType::Embedded);
        assert_eq!(VideoType::from_raw(1), VideoType::Linked);
        assert_eq!(VideoType::from_raw(2), VideoType::YouTube);
        assert_eq!(VideoType::from_raw(255), VideoType::Unknown);
    }

    #[test]
    fn test_video_data_default() {
        let video = VideoData::default();
        assert_eq!(video.video_type, VideoType::Unknown);
        assert_eq!(video.source, "");
        assert!(video.bin_data_id.is_none());
    }

    #[test]
    fn test_video_data_linked() {
        let video = VideoData {
            video_type: VideoType::Linked,
            source: "https://example.com/video.mp4".to_string(),
            ..Default::default()
        };
        assert_eq!(video.video_type, VideoType::Linked);
        assert_eq!(video.source, "https://example.com/video.mp4");
        assert!(video.is_linked());
        assert!(!video.is_embedded());
    }

    #[test]
    fn test_video_data_embedded() {
        let video = VideoData {
            video_type: VideoType::Embedded,
            bin_data_id: Some(42),
            ..Default::default()
        };
        assert_eq!(video.video_type, VideoType::Embedded);
        assert_eq!(video.bin_data_id, Some(42));
        assert!(video.is_embedded());
        assert!(!video.is_linked());
    }

    #[test]
    fn test_video_data_dimensions() {
        let mut video = VideoData::default();
        video.width = 14400;
        video.height = 8100;
        assert_eq!(video.width, 14400);
        assert_eq!(video.height, 8100);
    }

    #[test]
    fn test_video_data_poster() {
        let mut video = VideoData::default();
        assert!(video.poster_bin_id.is_none());
        video.poster_bin_id = Some(10);
        assert_eq!(video.poster_bin_id, Some(10));
    }
}
