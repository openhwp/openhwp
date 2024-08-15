use crate::RecordIter;

#[derive(Debug)]
pub struct LayoutCompatibility {
    //
}

impl<'hwp> RecordIter<'hwp> {
    pub fn layout_compatibility(&mut self) -> Option<LayoutCompatibility> {
        None
    }
}
