mod layout_compatibility;
mod track_change;

use crate::{DocInfoIter, HwpTag, u32};

pub use layout_compatibility::*;
pub use track_change::*;

#[derive(Debug)]
pub struct CompatibleDocument {
    pub target_program: TargetProgram,
    pub layout_compatibility: Option<LayoutCompatibility>,
    pub track_changes: Option<TrackChange>,
}

impl<'hwp> DocInfoIter<'hwp> {
    pub fn compatible_document(&mut self) -> Option<CompatibleDocument> {
        let record = self.next_if(|record| record.tag == HwpTag::HWPTAG_COMPATIBLE_DOCUMENT)?;

        let target_program = TargetProgram::from_u32(u32(record.payload, 0));
        let layout_compatibility = self
            .next_if(|record| record.tag == HwpTag::HWPTAG_LAYOUT_COMPATIBILITY)
            .map(|record| LayoutCompatibility::from_payload(record.payload));
        let track_changes = self
            .next_if(|record| record.tag == HwpTag::HWPTAG_TRACKCHANGE)
            .map(|record| TrackChange {
                raw: record.payload.to_vec(),
            });

        Some(CompatibleDocument {
            target_program,
            layout_compatibility,
            track_changes,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetProgram {
    Current,
    Hwp2007,
    MsWord,
    Unknown(u32),
}

impl TargetProgram {
    const fn from_u32(value: u32) -> Self {
        match value {
            0 => TargetProgram::Current,
            1 => TargetProgram::Hwp2007,
            2 => TargetProgram::MsWord,
            other => TargetProgram::Unknown(other),
        }
    }
}
