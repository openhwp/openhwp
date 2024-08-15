use super::{IdMappingCount, Image, ParagraphHeader};
use crate::{u16, u32, HwpTag, RecordIter};

#[derive(Debug)]
pub struct Bullet {
    pub paragraph_header: ParagraphHeader,
    pub char: char,
    pub use_image: bool,
    pub image: Image,
    pub checked_char: char,
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn bullets(&mut self, id_mappings: &IdMappingCount) -> Vec<Bullet> {
        let mut bullets = vec![];

        for record in self
            .clone()
            .take(id_mappings.bullet as usize)
            .filter(|record| record.tag == HwpTag::HWPTAG_BULLET)
        {
            bullets.push(Bullet::from_buf(record.payload));
            self.next();
        }

        bullets
    }
}

impl Bullet {
    pub fn from_buf(buf: &[u8]) -> Bullet {
        Bullet {
            paragraph_header: ParagraphHeader::from_buf(buf),
            char: unsafe { char::from_u32_unchecked(u16(buf, 12) as u32) },
            use_image: u32(buf, 14) != 0,
            image: Image::from_buf(&buf[18..23]),
            checked_char: unsafe { char::from_u32_unchecked(u16(buf, 23) as u32) },
        }
    }
}
