use crate::DocInfoIter;

#[derive(Debug)]
pub struct LayoutCompatibility {
    //
}

impl<'hwp> DocInfoIter<'hwp> {
    pub fn layout_compatibility(&mut self) -> Option<LayoutCompatibility> {
        None
    }
}
