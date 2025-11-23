use crate::{HwpDocumentError, HwpTag, u32};

#[derive(Debug, Clone, Copy)]
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
    pub const fn iter(buf: &[u8]) -> RecordIter<'_> {
        RecordIter::new(buf)
    }
}

impl<'hwp> RecordIter<'hwp> {
    #[inline]
    pub const fn new(buf: &'hwp [u8]) -> Self {
        Self { buf }
    }

    pub fn expect(&mut self, tag: HwpTag) -> Result<Record<'_>, HwpDocumentError> {
        self.next_if(|record| record.tag == tag)
            .ok_or_else(|| HwpDocumentError::InvalidTagId(None, tag))
    }

    #[inline]
    pub fn next_if<F>(&mut self, f: F) -> Option<Record<'hwp>>
    where
        F: FnOnce(&Record<'hwp>) -> bool,
    {
        match separate(self.buf) {
            Some((record, buf)) if f(&record) => {
                self.buf = buf;
                Some(record)
            }
            _ => None,
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

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_if(|_| true)
    }
}

const fn separate(buf: &[u8]) -> Option<(Record<'_>, &[u8])> {
    if buf.is_empty() {
        return None;
    }

    const OVER_SIZED: usize = 0xFFF;

    let header = u32(buf, 0);

    let tag = ((header >> 0) & 0x3FF) as u16;
    let level = ((header >> 10) & 0x3FF) as u16;
    let size = ((header >> 20) & 0xFFF) as usize;

    let (size, (payload, buf)) = match size {
        OVER_SIZED => {
            let size = u32(buf, 4) as usize;
            (size, buf.split_at(8).1.split_at(size))
        }
        size => (size, buf.split_at(4).1.split_at(size)),
    };

    let record = Record {
        tag: HwpTag::from_u16(tag),
        level,
        size,
        payload,
    };

    Some((record, buf))
}
