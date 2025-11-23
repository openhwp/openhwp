use crate::{BodyIter, HwpDocumentError, HwpTag, Record, u32};

#[derive(Debug)]
pub enum Control {
    Unknown {
        id: [u8; 4],
        payload: Vec<u8>,
        records: Vec<ControlRecord>,
    },
}

impl<'hwp> BodyIter<'hwp> {
    pub fn controls(&mut self, count: usize) -> Result<Vec<Control>, HwpDocumentError> {
        let mut controls = Vec::with_capacity(count);

        for _ in 0..count {
            controls.push(self.control()?);
        }

        Ok(controls)
    }

    pub fn control(&mut self) -> Result<Control, HwpDocumentError> {
        let record = self.expect(HwpTag::HWPTAG_CTRL_HEADER)?;
        let children = self.take_children(record.level);
        let control = Control::from_buf(record.payload, children);

        Ok(control)
    }
}

impl Control {
    pub fn from_buf(buf: &[u8], children: Vec<Record<'_>>) -> Self {
        let (id, payload) = buf.split_at(4);
        let mut reversed = [0u8; 4];
        reversed.copy_from_slice(&[id[3], id[2], id[1], id[0]]);

        Self::Unknown {
            id: reversed,
            payload: payload.to_vec(),
            records: children.into_iter().map(ControlRecord::from).collect(),
        }
    }
}

#[derive(Debug)]
pub enum ControlRecord {
    ListHeader {
        level: u16,
        header: ListHeader,
    },
    PageDefinition {
        level: u16,
        page: PageDef,
    },
    PageBorderFill {
        level: u16,
        fill: PageBorderFill,
    },
    FootnoteShape {
        level: u16,
        shape: FootnoteShape,
    },
    Raw {
        tag: HwpTag,
        level: u16,
        payload: Vec<u8>,
    },
}

impl From<Record<'_>> for ControlRecord {
    fn from(record: Record<'_>) -> Self {
        match record.tag {
            HwpTag::HWPTAG_LIST_HEADER => ControlRecord::ListHeader {
                level: record.level,
                header: ListHeader::from_buf(record.payload),
            },
            HwpTag::HWPTAG_PAGE_DEF => ControlRecord::PageDefinition {
                level: record.level,
                page: PageDef::from_buf(record.payload),
            },
            HwpTag::HWPTAG_PAGE_BORDER_FILL => ControlRecord::PageBorderFill {
                level: record.level,
                fill: PageBorderFill::from_buf(record.payload),
            },
            HwpTag::HWPTAG_FOOTNOTE_SHAPE => ControlRecord::FootnoteShape {
                level: record.level,
                shape: FootnoteShape::from_buf(record.payload),
            },
            _ => ControlRecord::Raw {
                tag: record.tag,
                level: record.level,
                payload: record.payload.to_vec(),
            },
        }
    }
}

#[derive(Debug)]
pub struct ListHeader {
    pub paragraph_count: i16,
    pub property: u32,
}

impl ListHeader {
    pub fn from_buf(buf: &[u8]) -> Self {
        let paragraph_count = i16::from_le_bytes([buf[0], buf[1]]);
        let property = u32(buf, 2);

        Self {
            paragraph_count,
            property,
        }
    }
}

#[derive(Debug)]
pub struct PageDef {
    pub width: i32,
    pub height: i32,
    pub margin_left: i32,
    pub margin_right: i32,
    pub margin_top: i32,
    pub margin_bottom: i32,
    pub margin_header: i32,
    pub margin_footer: i32,
    pub gutter: i32,
    pub property: PageDefProperty,
}

impl PageDef {
    pub fn from_buf(buf: &[u8]) -> Self {
        Self {
            width: hwpunit(buf, 0),
            height: hwpunit(buf, 4),
            margin_left: hwpunit(buf, 8),
            margin_right: hwpunit(buf, 12),
            margin_top: hwpunit(buf, 16),
            margin_bottom: hwpunit(buf, 20),
            margin_header: hwpunit(buf, 24),
            margin_footer: hwpunit(buf, 28),
            gutter: hwpunit(buf, 32),
            property: PageDefProperty::from_raw(u32(buf, 36)),
        }
    }
}

#[derive(Debug)]
pub struct PageDefProperty {
    pub raw: u32,
    pub landscape: bool,
    pub binding: BindingMethod,
}

impl PageDefProperty {
    pub const fn from_raw(raw: u32) -> Self {
        Self {
            raw,
            landscape: raw & 0b_1 != 0,
            binding: BindingMethod::from_bits(((raw >> 1) & 0b_11) as u8),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingMethod {
    OneSided,
    FacingPages,
    TopBinding,
    Unknown(u8),
}

impl BindingMethod {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => BindingMethod::OneSided,
            1 => BindingMethod::FacingPages,
            2 => BindingMethod::TopBinding,
            other => BindingMethod::Unknown(other),
        }
    }
}

const fn hwpunit(buf: &[u8], start_index: usize) -> i32 {
    i32::from_le_bytes([
        buf[start_index],
        buf[start_index + 1],
        buf[start_index + 2],
        buf[start_index + 3],
    ])
}

#[derive(Debug)]
pub struct PageBorderFill {
    pub property: PageBorderFillProperty,
    pub gap_left: i16,
    pub gap_right: i16,
    pub gap_top: i16,
    pub gap_bottom: i16,
    pub border_fill_id: u16,
}

impl PageBorderFill {
    pub fn from_buf(buf: &[u8]) -> Self {
        let property = match buf.len() >= 4 {
            true => u32(buf, 0),
            false => 0,
        };
        Self {
            property: PageBorderFillProperty::from_raw(property),
            gap_left: read_i16(buf, 4),
            gap_right: read_i16(buf, 6),
            gap_top: read_i16(buf, 8),
            gap_bottom: read_i16(buf, 10),
            border_fill_id: if buf.len() >= 14 {
                u16::from_le_bytes([buf[12], buf[13]])
            } else {
                0
            },
        }
    }
}

#[derive(Debug)]
pub struct PageBorderFillProperty {
    pub raw: u32,
    pub relative_to_paper: bool,
    pub include_header: bool,
    pub include_footer: bool,
    pub area: BorderFillArea,
}

impl PageBorderFillProperty {
    pub const fn from_raw(raw: u32) -> Self {
        Self {
            raw,
            relative_to_paper: raw & 0b_1 != 0,
            include_header: raw & 0b_10 != 0,
            include_footer: raw & 0b_100 != 0,
            area: BorderFillArea::from_bits(((raw >> 3) & 0b_11) as u8),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderFillArea {
    Paper,
    Page,
    Border,
    Unknown(u8),
}

impl BorderFillArea {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => BorderFillArea::Paper,
            1 => BorderFillArea::Page,
            2 => BorderFillArea::Border,
            other => BorderFillArea::Unknown(other),
        }
    }
}

#[derive(Debug)]
pub struct FootnoteShape {
    pub property: FootnoteShapeProperty,
    pub user_symbol: u16,
    pub prefix_char: u16,
    pub suffix_char: u16,
    pub start_number: u16,
    pub line_length: i16,
    pub line_margin_top: i16,
    pub line_margin_bottom: i16,
    pub between_margin: i16,
    pub line_type: u8,
    pub line_thickness: u8,
    pub line_color: u32,
}

impl FootnoteShape {
    pub fn from_buf(buf: &[u8]) -> Self {
        Self {
            property: FootnoteShapeProperty::from_raw(u32(buf, 0)),
            user_symbol: read_u16(buf, 4),
            prefix_char: read_u16(buf, 6),
            suffix_char: read_u16(buf, 8),
            start_number: read_u16(buf, 10),
            line_length: read_i16(buf, 12),
            line_margin_top: read_i16(buf, 14),
            line_margin_bottom: read_i16(buf, 16),
            between_margin: read_i16(buf, 18),
            line_type: buf.get(20).copied().unwrap_or_default(),
            line_thickness: buf.get(21).copied().unwrap_or_default(),
            line_color: if buf.len() >= 26 { u32(buf, 22) } else { 0 },
        }
    }
}

#[derive(Debug)]
pub struct FootnoteShapeProperty {
    pub raw: u32,
    pub number_shape: u8,
    pub arrangement: u8,
    pub numbering: u8,
    pub superscript: bool,
    pub continue_with_text: bool,
}

impl FootnoteShapeProperty {
    pub const fn from_raw(raw: u32) -> Self {
        Self {
            raw,
            number_shape: (raw & 0xff) as u8,
            arrangement: ((raw >> 8) & 0b_11) as u8,
            numbering: ((raw >> 10) & 0b_11) as u8,
            superscript: raw & (1 << 12) != 0,
            continue_with_text: raw & (1 << 13) != 0,
        }
    }
}

const fn read_u16(buf: &[u8], index: usize) -> u16 {
    if buf.len() >= index + 2 {
        u16::from_le_bytes([buf[index], buf[index + 1]])
    } else {
        0
    }
}

const fn read_i16(buf: &[u8], index: usize) -> i16 {
    if buf.len() >= index + 2 {
        i16::from_le_bytes([buf[index], buf[index + 1]])
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BindingMethod, BorderFillArea, FootnoteShape, ListHeader, PageBorderFill, PageDef,
    };

    #[test]
    fn parses_list_header_payload() {
        let buf = [0x02, 0x00, 0x34, 0x12, 0x78, 0x56];
        let header = ListHeader::from_buf(&buf);

        assert_eq!(header.paragraph_count, 2);
        assert_eq!(header.property, 0x5678_1234);
    }

    #[test]
    fn parses_page_definition_payload() {
        let mut buf = vec![];
        for value in 1i32..=9 {
            buf.extend_from_slice(&(value * 10).to_le_bytes());
        }
        buf.extend_from_slice(&0b_10u32.to_le_bytes());

        let page_def = PageDef::from_buf(&buf);
        assert_eq!(page_def.width, 10);
        assert_eq!(page_def.gutter, 90);
        assert!(!page_def.property.landscape);
        assert_eq!(page_def.property.binding, BindingMethod::FacingPages);
    }

    #[test]
    fn parses_page_border_fill_payload() {
        let mut buf = vec![];
        buf.extend_from_slice(&0b_1111u32.to_le_bytes());
        buf.extend_from_slice(&5i16.to_le_bytes());
        buf.extend_from_slice(&6i16.to_le_bytes());
        buf.extend_from_slice(&7i16.to_le_bytes());
        buf.extend_from_slice(&8i16.to_le_bytes());
        buf.extend_from_slice(&9u16.to_le_bytes());

        let fill = PageBorderFill::from_buf(&buf);
        assert!(fill.property.relative_to_paper);
        assert!(fill.property.include_header);
        assert!(fill.property.include_footer);
        assert_eq!(fill.property.area, BorderFillArea::Page);
        assert_eq!(fill.border_fill_id, 9);
    }

    #[test]
    fn parses_footnote_shape_payload() {
        let mut buf = vec![];
        buf.extend_from_slice(&0x0000_1D02u32.to_le_bytes());
        buf.extend_from_slice(&1u16.to_le_bytes());
        buf.extend_from_slice(&2u16.to_le_bytes());
        buf.extend_from_slice(&3u16.to_le_bytes());
        buf.extend_from_slice(&4u16.to_le_bytes());
        buf.extend_from_slice(&5i16.to_le_bytes());
        buf.extend_from_slice(&6i16.to_le_bytes());
        buf.extend_from_slice(&7i16.to_le_bytes());
        buf.extend_from_slice(&8i16.to_le_bytes());
        buf.push(9);
        buf.push(10);
        buf.extend_from_slice(&0x1122_3344u32.to_le_bytes());

        let shape = FootnoteShape::from_buf(&buf);
        assert_eq!(shape.property.number_shape, 0x02);
        assert_eq!(shape.property.arrangement, 1);
        assert_eq!(shape.property.numbering, 3);
        assert!(shape.property.superscript);
        assert!(!shape.property.continue_with_text);
        assert_eq!(shape.user_symbol, 1);
        assert_eq!(shape.prefix_char, 2);
        assert_eq!(shape.line_color, 0x1122_3344);
    }
}
