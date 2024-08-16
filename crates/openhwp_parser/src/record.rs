use crate::{u32, HwpDocumentError, HwpTag};

#[derive(Debug)]
pub struct Record<'hwp> {
    pub tag: HwpTag,
    pub level: u16,
    pub size: usize,
    pub payload: &'hwp [u8],
}

#[derive(Debug, Clone, Copy)]
pub struct RecordIter<'hwp> {
    buf: &'hwp [u8],
}

impl<'hwp> Record<'hwp> {
    #[inline]
    pub const fn iter(buf: &[u8]) -> RecordIter {
        RecordIter::new(buf)
    }
}

impl<'hwp> RecordIter<'hwp> {
    #[inline]
    pub const fn new(buf: &'hwp [u8]) -> Self {
        Self { buf }
    }

    pub fn expect(&mut self, tag: HwpTag) -> Result<Record, HwpDocumentError> {
        match self.clone().next() {
            Some(record) if record.tag == tag => {
                self.next();
                Ok(record)
            }
            Some(record) => Err(HwpDocumentError::InvalidTagId(Some(record.tag), tag)),
            None => Err(HwpDocumentError::InvalidTagId(None, tag)),
        }
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    #[inline]
    pub const fn remaining(&self) -> &[u8] {
        self.buf
    }
}

impl<'hwp> Iterator for RecordIter<'hwp> {
    type Item = Record<'hwp>;

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

    let tag = (header & 0b0000_0000_0000_0000_0000_0011_1111_1111) as u16;
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
        tag: HwpTag::from_u16(tag),
        level,
        size,
        payload,
    };

    (record, buf)
}
