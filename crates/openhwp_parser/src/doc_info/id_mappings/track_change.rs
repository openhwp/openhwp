use super::IdMappingCount;
use crate::{DocInfoIter, HwpTag};

#[derive(Debug, Clone)]
pub struct TrackChangeEntry {
    pub raw: Vec<u8>,
}

impl TrackChangeEntry {
    #[inline]
    fn from_buf(buf: &[u8]) -> Self {
        Self { raw: buf.to_vec() }
    }
}

impl<'hwp> DocInfoIter<'hwp> {
    pub fn track_change_entries(&mut self, counts: &IdMappingCount) -> Vec<TrackChangeEntry> {
        let mut entries = Vec::with_capacity(counts.track_change as usize);

        for _ in 0..counts.track_change {
            match self.next_if(|record| record.tag == HwpTag::HWPTAG_TRACK_CHANGE) {
                Some(record) => entries.push(TrackChangeEntry::from_buf(record.payload)),
                _ => break,
            }
        }

        entries
    }
}
