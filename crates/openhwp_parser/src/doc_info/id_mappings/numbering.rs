use super::IdMappingCount;
use crate::{u16, u32, DocInfoTag, RecordIter, Version};

#[derive(Debug)]
pub struct Numbering {
    pub start_number: u16,
    pub paragraphs: ParagraphVec,
}

#[derive(Debug)]
pub enum ParagraphVec {
    NonExtends([Paragraph; 7]),
    Extends([Paragraph; 10]),
}

#[derive(Debug)]
pub struct Paragraph {
    pub start_number: Option<u32>,
    pub format: String,
    pub header: ParagraphHeader,
}

#[derive(Debug)]
pub struct ParagraphHeader {
    pub alignment: Alignment,
    pub use_instance_width: bool,
    pub auth_indent: bool,
    pub text_offset_kind: TextOffsetKind,
    pub correction_width: i16,
    pub distance_from_body: i16,
    pub char_shape_id: u32,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Unknown(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum TextOffsetKind {
    Relative,
    Value,
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn numberings(
        &mut self,
        id_mappings: &IdMappingCount,
        version: &Version,
    ) -> Vec<Numbering> {
        let mut numberings = Vec::with_capacity(id_mappings.numbering as usize);

        for record in self
            .take(id_mappings.numbering as usize)
            .take_while(|record| record.tag_id == DocInfoTag::HWPTAG_NUMBERING as u16)
        {
            numberings.push(Numbering::from_buf(record.payload, version));
        }

        numberings
    }
}

impl Numbering {
    pub fn from_buf(buf: &[u8], version: &Version) -> Numbering {
        let (mut paragraph0, buf) = Paragraph::from_buf(buf);
        let (mut paragraph1, buf) = Paragraph::from_buf(buf);
        let (mut paragraph2, buf) = Paragraph::from_buf(buf);
        let (mut paragraph3, buf) = Paragraph::from_buf(buf);
        let (mut paragraph4, buf) = Paragraph::from_buf(buf);
        let (mut paragraph5, buf) = Paragraph::from_buf(buf);
        let (mut paragraph6, buf) = Paragraph::from_buf(buf);
        let (start_number, buf) = buf.split_at(2);
        let start_number = u16(start_number, 0);
        let buf = if version >= &Version::new(5, 0, 2, 5) {
            paragraph0.start_number = Some(u32(buf, 0));
            paragraph1.start_number = Some(u32(buf, 4));
            paragraph2.start_number = Some(u32(buf, 8));
            paragraph3.start_number = Some(u32(buf, 12));
            paragraph4.start_number = Some(u32(buf, 16));
            paragraph5.start_number = Some(u32(buf, 20));
            paragraph6.start_number = Some(u32(buf, 24));

            &buf[28..]
        } else {
            buf
        };
        let paragraphs = if !buf.is_empty() {
            let (mut paragraph7, buf) = Paragraph::from_buf(buf);
            let (mut paragraph8, buf) = Paragraph::from_buf(buf);
            let (mut paragraph9, buf) = Paragraph::from_buf(buf);
            if version >= &Version::new(5, 1, 0, 0) {
                paragraph7.start_number = Some(u32(buf, 0));
                paragraph8.start_number = Some(u32(buf, 4));
                paragraph9.start_number = Some(u32(buf, 8));
            }

            ParagraphVec::Extends([
                paragraph0, paragraph1, paragraph2, paragraph3, paragraph4, paragraph5, paragraph6,
                paragraph7, paragraph8, paragraph9,
            ])
        } else {
            ParagraphVec::NonExtends([
                paragraph0, paragraph1, paragraph2, paragraph3, paragraph4, paragraph5, paragraph6,
            ])
        };

        Numbering {
            start_number,
            paragraphs,
        }
    }
}

impl Paragraph {
    pub fn from_buf(buf: &[u8]) -> (Self, &[u8]) {
        let header = ParagraphHeader::from_buf(buf);
        let format_size = u16(buf, 12);
        let format: Vec<_> = buf[14..14 + 2 * format_size as usize]
            .chunks_exact(2)
            .map(|c| <u16>::from_le_bytes([c[0], c[1]]))
            .collect();
        let format = String::from_utf16_lossy(&format);
        let paragraph = Self {
            start_number: None,
            header,
            format,
        };

        (paragraph, &buf[14 + 2 * format_size as usize..])
    }
}

impl ParagraphHeader {
    pub fn from_buf(buf: &[u8]) -> Self {
        let attribute = u32(buf, 0);
        let alignment = match attribute & 0b0000_0011 {
            0 => Alignment::Left,
            1 => Alignment::Center,
            2 => Alignment::Right,
            alignment => Alignment::Unknown(alignment as u8),
        };
        let use_instance_width = attribute & 0b0000_0100 != 0;
        let auth_indent = attribute & 0b0000_1000 != 0;
        let text_offset_kind = if attribute & 0b0001_0000 != 0 {
            TextOffsetKind::Value
        } else {
            TextOffsetKind::Relative
        };
        let correction_width = u16(buf, 4) as i16;
        let distance_from_body = u16(buf, 6) as i16;
        let char_shape_id = u32(buf, 8);

        let paragraph = Self {
            alignment,
            use_instance_width,
            auth_indent,
            text_offset_kind,
            correction_width,
            distance_from_body,
            char_shape_id,
        };

        paragraph
    }
}
