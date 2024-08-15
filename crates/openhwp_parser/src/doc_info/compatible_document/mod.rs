mod layout_compatibility;
mod track_change;

use crate::{DocInfoIter, HwpTag};

pub use layout_compatibility::*;
pub use track_change::*;

#[derive(Debug)]
pub struct CompatibleDocument {
    pub layout_compatibility: Option<LayoutCompatibility>,
    pub track_changes: Option<TrackChange>,
}

impl<'hwp> DocInfoIter<'hwp> {
    pub fn compatible_document(&mut self) -> CompatibleDocument {
        match self.expect(HwpTag::HWPTAG_COMPATIBLE_DOCUMENT) {
            Ok(_) => CompatibleDocument {
                layout_compatibility: self.layout_compatibility(),
                track_changes: self.track_changes(),
            },
            Err(_) => CompatibleDocument {
                layout_compatibility: None,
                track_changes: None,
            },
        }
    }
}
