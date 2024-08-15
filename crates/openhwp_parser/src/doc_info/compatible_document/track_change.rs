use crate::RecordIter;

#[derive(Debug)]
pub struct TrackChange {
    //
}

impl<'hwp> RecordIter<'hwp> {
    pub fn track_changes(&mut self) -> Option<TrackChange> {
        None
    }
}
