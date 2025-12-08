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
    pub fn detect(data: &[u8]) -> Self {
        if data.len() < 4 {
            return Self::Unknown;
        }

        // PNG magic: 89 50 4E 47 0D 0A 1A 0A
        if data.len() >= 8
            && data[0] == 0x89
            && data[1] == 0x50
            && data[2] == 0x4E
            && data[3] == 0x47
        {
            return Self::Png;
        }

        // GIF magic: GIF87a or GIF89a
        if data[0] == b'G' && data[1] == b'I' && data[2] == b'F' {
            return Self::Gif;
        }

        // BMP magic: BM
        if data[0] == b'B' && data[1] == b'M' {
            return Self::Bmp;
        }

        Self::Unknown
    }

    /// Returns the MIME type for this format.
    pub const fn mime_type(&self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Gif => "image/gif",
            Self::Bmp => "image/bmp",
            Self::Unknown => "application/octet-stream",
        }
    }

    /// Returns the file extension for this format.
    pub const fn extension(&self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Gif => "gif",
            Self::Bmp => "bmp",
            Self::Unknown => "bin",
        }
    }
}

/// Preview image of the document.
#[derive(Debug, Clone, Default)]
pub struct PreviewImage {
    /// Image format.
    pub format: PreviewFormat,
    /// Raw image data.
    pub data: Vec<u8>,
}

impl PreviewImage {
    /// Creates a preview image from raw bytes.
    pub fn from_bytes(data: Vec<u8>) -> Self {
        let format = PreviewFormat::detect(&data);
        Self { format, data }
    }

    /// Returns true if the preview image is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the image data length.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the raw image bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
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
        let mut chars = Vec::with_capacity(data.len() / 2);
        for chunk in data.chunks(2) {
            if chunk.len() == 2 {
                let code_unit = u16::from_le_bytes([chunk[0], chunk[1]]);
                if code_unit == 0 {
                    break;
                }
                chars.push(code_unit);
            }
        }

        Ok(Self {
            text: String::from_utf16_lossy(&chars),
        })
    }

    /// Returns true if the preview text is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the text content.
    pub fn as_str(&self) -> &str {
        &self.text
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
        assert_eq!(PreviewFormat::detect(&[0x00, 0x00, 0x00]), PreviewFormat::Unknown);
    }

    #[test]
    fn test_preview_image() {
        let png_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let preview = PreviewImage::from_bytes(png_data.clone());
        assert_eq!(preview.format, PreviewFormat::Png);
        assert_eq!(preview.len(), 8);
        assert_eq!(preview.as_bytes(), &png_data);
    }

    #[test]
    fn test_preview_text() {
        // "Hello" in UTF-16LE
        let data = [0x48, 0x00, 0x65, 0x00, 0x6C, 0x00, 0x6C, 0x00, 0x6F, 0x00];
        let preview = PreviewText::from_bytes(&data).unwrap();
        assert_eq!(preview.as_str(), "Hello");
    }
}
