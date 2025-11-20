use crate::{Record, RecordIter, Version};

#[derive(Debug, Clone)]
pub struct DocInfoIter<'hwp> {
    stream: RecordIter<'hwp>,
    version: Version,
}

impl<'hwp> DocInfoIter<'hwp> {
    pub fn new(buf: &'hwp [u8], version: &Version) -> Self {
        let stream = Record::iter(buf);
        let version = version.clone();

        Self { stream, version }
    }

    #[inline]
    pub const fn version(&self) -> &Version {
        &self.version
    }
}

impl<'hwp> Iterator for DocInfoIter<'hwp> {
    type Item = Record<'hwp>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.stream.next()
    }
}

impl<'hwp> std::ops::Deref for DocInfoIter<'hwp> {
    type Target = RecordIter<'hwp>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}

impl<'hwp> std::ops::DerefMut for DocInfoIter<'hwp> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stream
    }
}
