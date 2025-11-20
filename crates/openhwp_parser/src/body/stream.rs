use crate::{HwpDocumentError, HwpTag, Record, RecordIter, Version};

#[derive(Debug, Clone)]
pub struct BodyIter<'hwp> {
    stream: RecordIter<'hwp>,
    version: Version,
    pending: Option<Record<'hwp>>,
}

impl<'hwp> BodyIter<'hwp> {
    pub fn new(buf: &'hwp [u8], version: &Version) -> Self {
        let stream = Record::iter(buf);
        let version = version.clone();

        Self {
            stream,
            version,
            pending: None,
        }
    }

    #[inline]
    pub const fn version(&self) -> &Version {
        &self.version
    }

    pub fn is_empty(&mut self) -> bool {
        self.peek().is_none()
    }

    #[inline]
    pub fn expect(&mut self, tag: HwpTag) -> Result<Record<'hwp>, HwpDocumentError> {
        match self.peek() {
            Some(record) if record.tag == tag => {
                self.next();
                Ok(record)
            }
            Some(record) => Err(HwpDocumentError::InvalidTagId(Some(record.tag), tag)),
            None => Err(HwpDocumentError::InvalidTagId(None, tag)),
        }
    }

    #[inline]
    pub fn next(&mut self) -> Option<Record<'hwp>> {
        match self.pending.take() {
            Some(record) => Some(record),
            None => self.stream.next(),
        }
    }

    #[inline]
    pub fn peek(&mut self) -> Option<Record<'hwp>> {
        if self.pending.is_none() {
            self.pending = self.stream.next();
        }

        self.pending
    }

    pub fn take_children(&mut self, parent_level: u16) -> Vec<Record<'hwp>> {
        let mut children = vec![];

        while let Some(record) = self.peek() {
            if record.level <= parent_level {
                break;
            }

            children.push(self.next().expect("peek ensured record exists"));
        }

        children
    }
}
