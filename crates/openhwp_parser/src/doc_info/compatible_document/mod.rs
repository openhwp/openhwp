mod layout_compatibility;
mod track_change;

use super::{DocInfoTag, RecordIter};

pub use layout_compatibility::*;
pub use track_change::*;

#[derive(Debug)]
pub struct CompatibleDocument {
    pub layout_compatibility: Option<LayoutCompatibility>,
    pub track_changes: Option<TrackChange>,
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn compatible_document(&mut self) -> CompatibleDocument {
        match self.expect(DocInfoTag::HWPTAG_COMPATIBLE_DOCUMENT as u16) {
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
