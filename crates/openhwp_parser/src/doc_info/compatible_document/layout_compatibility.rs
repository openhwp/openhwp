use crate::RecordIter;

#[derive(Debug)]
pub struct LayoutCompatibility {
    //
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn layout_compatibility(&mut self) -> Option<LayoutCompatibility> {
        None
    }
}
