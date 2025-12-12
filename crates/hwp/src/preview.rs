//! Preview image parsing.
//!
//! The PrvImage stream contains a preview image of the document,
//! typically in PNG or GIF format.

use crate::error::Result;

/// Preview image format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreviewFormat {
    /// Unknown format.
    #[default]
    Unknown,
    /// PNG format.
    Png,
    /// GIF format.
    Gif,
    /// BMP format.
    Bmp,
}

impl PreviewFormat {
    /// Detects format from magic bytes.
    pub const fn detect(data: &[u8]) -> Self {
        match data {
            // PNG magic: 89 50 4E 47 0D 0A 1A 0A
            [0x89, 0x50, 0x4E, 0x47, ..] => Self::Png,
            // GIF magic: GIF87a or GIF89a
            [b'G', b'I', b'F', ..] => Self::Gif,
            // BMP magic: BM
            [b'B', b'M', ..] => Self::Bmp,
            // Unknown or insufficient data
            _ => Self::Unknown,
        }
    }
}

/// Preview image of the document.
#[derive(Debug, Clone, Default)]
pub struct PreviewImage {
    /// Raw image data.
    pub data: Vec<u8>,
}

impl PreviewImage {
    /// Creates a preview image from raw bytes.
    pub const fn from_bytes(data: Vec<u8>) -> Self {
        Self { data }
    }

    #[inline]
    pub fn format(&self) -> PreviewFormat {
        PreviewFormat::detect(&self.data)
    }
}

/// Preview text of the document.
///
/// This is a plain text preview stored in PrvText stream.
#[derive(Debug, Clone, Default)]
pub struct PreviewText {
    /// Plain text content.
    pub text: String,
}

impl PreviewText {
    /// Creates preview text from UTF-16 bytes.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let text: Vec<_> = data
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .take_while(|&code_unit| code_unit != 0)
            .collect();
        let text = String::from_utf16_lossy(&text);

        Ok(Self { text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_detection() {
        assert_eq!(
            PreviewFormat::detect(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]),
            PreviewFormat::Png
        );
        assert_eq!(
            PreviewFormat::detect(&[b'G', b'I', b'F', b'8', b'9', b'a']),
            PreviewFormat::Gif
        );
        assert_eq!(
            PreviewFormat::detect(&[b'B', b'M', 0x00, 0x00]),
            PreviewFormat::Bmp
        );
        assert_eq!(
            PreviewFormat::detect(&[0x00, 0x00, 0x00]),
            PreviewFormat::Unknown
        );
    }

    #[test]
    fn test_preview_image() {
        let png_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let preview = PreviewImage::from_bytes(png_data);
        assert_eq!(preview.format(), PreviewFormat::Png);
    }

    #[test]
    fn test_preview_text() {
        // "Hello" in UTF-16LE
        let data = [0x48, 0x00, 0x65, 0x00, 0x6C, 0x00, 0x6C, 0x00, 0x6F, 0x00];
        let preview = PreviewText::from_bytes(&data).unwrap();
        assert_eq!(preview.text, "Hello");
    }
}
