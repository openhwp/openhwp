#[derive(Debug, Default)]
pub struct Record<'doc_info> {
    pub tag_id: u16,
    pub level: u16,
    pub size: usize,
    pub payload: &'doc_info [u8],
}

pub struct RecordIter<'doc_info> {
    bytes: &'doc_info [u8],
}

impl<'doc_info> Record<'doc_info> {
    #[inline]
    pub const fn iter(bytes: &[u8]) -> RecordIter {
        RecordIter::new(bytes)
    }
}

impl<'doc_info> RecordIter<'doc_info> {
    #[inline]
    pub const fn new(bytes: &'doc_info [u8]) -> Self {
        Self { bytes }
    }
}

impl<'doc_info> Iterator for RecordIter<'doc_info> {
    type Item = Record<'doc_info>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }

        let (record, rest) = consume(self.bytes);
        self.bytes = rest;

        Some(record)
    }
}

const fn consume(bytes: &[u8]) -> (Record, &[u8]) {
    const OVER_SIZED: usize = 0xFFF;

    let header = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

    let tag_id = (header & 0x3FF) as u16;
    let level = ((header >> 10) & 0x3FF) as u16;
    let size = ((header >> 20) & 0xFFF) as usize;

    let (size, payload, bytes) = match size {
        OVER_SIZED => {
            let size = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
            let (_, bytes) = bytes.split_at(8);
            let (payload, bytes) = bytes.split_at(size);

            (size, payload, bytes)
        }
        size => {
            let (_, bytes) = bytes.split_at(4);
            let (payload, bytes) = bytes.split_at(size);

            (size, payload, bytes)
        }
    };
    let record = Record {
        tag_id,
        level,
        size,
        payload,
    };

    (record, bytes)
}
