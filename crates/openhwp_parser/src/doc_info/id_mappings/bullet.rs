use super::{IdMappingCount, Image, NumberingParagraphHeader};
use crate::{DocInfoIter, HwpTag, u16, u32};

#[derive(Debug)]
pub struct Bullet {
    pub paragraph_header: NumberingParagraphHeader,
    pub char: char,
    pub use_image: bool,
    pub image: Image,
    pub checked_char: char,
}

impl<'hwp> DocInfoIter<'hwp> {
    pub fn bullets(&mut self, id_mappings: &IdMappingCount) -> Vec<Bullet> {
        let mut bullets = vec![];

        for _ in 0..id_mappings.bullet {
            match self.next_if(|record| record.tag == HwpTag::HWPTAG_BULLET) {
                Some(record) => bullets.push(Bullet::from_buf(record.payload)),
                None => break,
            }
        }

        bullets
    }
}

impl Bullet {
    pub fn from_buf(buf: &[u8]) -> Bullet {
        Bullet {
            paragraph_header: NumberingParagraphHeader::from_buf(buf),
            char: unsafe { char::from_u32_unchecked(u16(buf, 12) as u32) },
            use_image: u32(buf, 14) != 0,
            image: Image::from_buf(&buf[18..23]),
            checked_char: unsafe { char::from_u32_unchecked(u16(buf, 23) as u32) },
        }
    }
}
