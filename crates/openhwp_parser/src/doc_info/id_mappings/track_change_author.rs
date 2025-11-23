use super::IdMappingCount;
use crate::{DocInfoIter, HwpTag};

#[derive(Debug, Clone)]
pub struct TrackChangeAuthor {
    pub raw: Vec<u8>,
}

impl TrackChangeAuthor {
    #[inline]
    fn from_buf(buf: &[u8]) -> Self {
        Self { raw: buf.to_vec() }
    }
}

impl<'hwp> DocInfoIter<'hwp> {
    pub fn track_change_authors(&mut self, counts: &IdMappingCount) -> Vec<TrackChangeAuthor> {
        let mut authors = Vec::with_capacity(counts.track_change_author as usize);

        for _ in 0..counts.track_change_author {
            match self.next_if(|record| record.tag == HwpTag::HWPTAG_TRACK_CHANGE_AUTHOR) {
                Some(record) => authors.push(TrackChangeAuthor::from_buf(record.payload)),
                None => break,
            }
        }

        authors
    }
}
