use crate::RecordIter;

#[derive(Debug)]
pub struct TrackChange {
    //
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn track_changes(&mut self) -> Option<TrackChange> {
        None
    }
}
