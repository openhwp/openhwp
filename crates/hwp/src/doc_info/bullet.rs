//! Bullet definition record.

use crate::error::Result;
use crate::util::ByteReader;

use super::numbering::ParagraphHeadInfo;

/// Bullet definition.
#[derive(Debug, Clone)]
pub struct Bullet {
    head_info: ParagraphHeadInfo,
    bullet_char: char,
    use_image: bool,
    image_id: Option<u32>,
    check_bullet_char: char,
}

impl Bullet {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let head_info = ParagraphHeadInfo::from_reader(reader)?;
        let bullet_char_code = reader.read_u16()?;
        let bullet_char = char::from_u32(bullet_char_code as u32).unwrap_or('\u{25CF}');

        let image_bullet_flag = reader.read_i32()?;
        let use_image = image_bullet_flag != 0;
        let image_id = if use_image {
            Some(image_bullet_flag as u32)
        } else {
            None
        };

        // Skip image bullet info (brightness, contrast, effect, id) if present
        if use_image {
            reader.skip(4)?;
        }

        let check_bullet_char_code = reader.read_u16()?;
        let check_bullet_char = char::from_u32(check_bullet_char_code as u32).unwrap_or('\u{2611}');

        Ok(Self {
            head_info,
            bullet_char,
            use_image,
            image_id,
            check_bullet_char,
        })
    }

    /// Returns the paragraph head info.
    pub const fn head_info(&self) -> &ParagraphHeadInfo {
        &self.head_info
    }

    /// Returns the bullet character.
    pub const fn bullet_char(&self) -> char {
        self.bullet_char
    }

    /// Whether this uses an image bullet.
    pub const fn use_image(&self) -> bool {
        self.use_image
    }

    /// Returns the image ID if using image bullet.
    pub const fn image_id(&self) -> Option<u32> {
        self.image_id
    }

    /// Returns the check bullet character.
    pub const fn check_bullet_char(&self) -> char {
        self.check_bullet_char
    }
}
