use crate::{u16, u32, HwpDocumentError, HwpTag, RecordIter};

#[derive(Debug)]
pub struct DocumentProperties {
    pub section_size: u16,
    pub starting_index: StartingIndex,
    pub carat_location: CaratLocation,
}

#[derive(Debug)]
pub struct StartingIndex {
    pub page: u16,
    pub footnote: u16,
    pub endnote: u16,
    pub picture: u16,
    pub table: u16,
    pub equation: u16,
}

#[derive(Debug)]
pub struct CaratLocation {
    pub list_id: u32,
    pub paragraph_id: u32,
    pub char_index: u32,
}

impl<'hwp> RecordIter<'hwp> {
    pub fn document_properties(&mut self) -> Result<DocumentProperties, HwpDocumentError> {
        let record = self.expect(HwpTag::HWPTAG_DOCUMENT_PROPERTIES)?;
        let document_properties = DocumentProperties::from_buf(record.payload);

        Ok(document_properties)
    }
}

impl DocumentProperties {
    pub const fn from_buf(buf: &[u8]) -> Self {
        Self {
            section_size: u16(buf, 0),
            starting_index: StartingIndex {
                page: u16(buf, 2),
                footnote: u16(buf, 4),
                endnote: u16(buf, 6),
                picture: u16(buf, 8),
                table: u16(buf, 10),
                equation: u16(buf, 12),
            },
            carat_location: CaratLocation {
                list_id: u32(buf, 14),
                paragraph_id: u32(buf, 18),
                char_index: u32(buf, 22),
            },
        }
    }
}
