use crate::u32;

#[derive(Debug, Default)]
pub struct Record<'doc_info> {
    pub tag_id: u16,
    pub level: u16,
    pub size: usize,
    pub payload: &'doc_info [u8],
}

pub struct RecordIter<'doc_info> {
    buf: &'doc_info [u8],
}

impl<'doc_info> Record<'doc_info> {
    #[inline]
    pub const fn iter(buf: &[u8]) -> RecordIter {
        RecordIter::new(buf)
    }
}

impl<'doc_info> RecordIter<'doc_info> {
    #[inline]
    pub const fn new(buf: &'doc_info [u8]) -> Self {
        Self { buf }
    }
}

impl<'doc_info> Iterator for RecordIter<'doc_info> {
    type Item = Record<'doc_info>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            return None;
        }

        let (record, buf) = consume(self.buf);
        self.buf = buf;

        Some(record)
    }
}

const fn consume(buf: &[u8]) -> (Record, &[u8]) {
    const OVER_SIZED: usize = 0xFFF;

    let header = u32(buf, 0);

    let tag_id = (header & 0b0000_0000_0000_0000_0000_0011_1111_1111) as u16;
    let level = ((header & 0b0000_0000_0000_1111_1111_1100_0000_0000) >> 10) as u16;
    let size = ((header & 0b1111_1111_1111_0000_0000_0000_0000_0000) >> 20) as usize;

    let (size, payload, buf) = match size {
        OVER_SIZED => {
            let size = u32(buf, 4) as usize;
            let (_, buf) = buf.split_at(8);
            let (payload, buf) = buf.split_at(size);

            (size, payload, buf)
        }
        size => {
            let (_, buf) = buf.split_at(4);
            let (payload, buf) = buf.split_at(size);

            (size, payload, buf)
        }
    };
    let record = Record {
        tag_id,
        level,
        size,
        payload,
    };

    (record, buf)
}
