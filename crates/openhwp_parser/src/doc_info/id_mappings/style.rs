use super::IdMappingCount;
use crate::{to_string, u16, DocInfoIter, HwpTag};

#[derive(Debug)]
pub struct Style {
    pub korean_name: String,
    pub english_name: String,
    pub kind: StyleKind,
    pub next_style_id: u8,
    pub language_id: LanguageId,
    pub paragraph_id: u16,
    pub character_shape_id: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum StyleKind {
    Paragraph,
    Character,
}

// TODO: Change to enum refer to https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oe376/6c085406-a698-4e12-9d4d-c3b0ee3dbc4a
#[derive(Debug, Clone, Copy)]
pub struct LanguageId(pub u16);

impl<'hwp> DocInfoIter<'hwp> {
    pub fn styles(&mut self, id_mappings: &IdMappingCount) -> Vec<Style> {
        let mut styles = Vec::with_capacity(id_mappings.style as usize);

        for record in self
            .clone()
            .take(id_mappings.style as usize)
            .take_while(|record| record.tag == HwpTag::HWPTAG_STYLE)
        {
            styles.push(Style::from_buf(record.payload));
            self.next();
        }

        styles
    }
}

impl Style {
    pub fn from_buf(buf: &[u8]) -> Style {
        let (size, buf) = buf.split_at(2);
        let size = u16(size, 0);
        let (korean_name, buf) = buf.split_at(2 * size as usize);
        let korean_name = to_string(korean_name);

        let (size, buf) = buf.split_at(2);
        let size = u16(size, 0);
        let (english_name, buf) = buf.split_at(2 * size as usize);
        let english_name = to_string(english_name);

        let kind = match buf[0] & 0b0000_0111 {
            0 => StyleKind::Paragraph,
            1 => StyleKind::Character,
            _ => unreachable!(),
        };
        let next_style_id = buf[1];
        let language_id = LanguageId(u16(buf, 2));
        let paragraph_id = u16(buf, 4);
        let character_shape_id = u16(buf, 6);

        Style {
            korean_name,
            english_name,
            kind,
            next_style_id,
            language_id,
            paragraph_id,
            character_shape_id,
        }
    }
}
