use crate::DocInfoIter;

#[derive(Debug)]
pub struct TrackChange {
    //
}

impl<'hwp> DocInfoIter<'hwp> {
    pub fn track_changes(&mut self) -> Option<TrackChange> {
        None
    }
}
