//! Compatible document settings record.

use crate::error::Result;
use crate::util::ByteReader;

/// Target program for compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TargetProgram {
    #[default]
    HwpCurrent,
    Hwp2007,
    MsWord,
}

impl TargetProgram {
    pub const fn from_raw(value: u32) -> Self {
        match value {
            0 => Self::HwpCurrent,
            1 => Self::Hwp2007,
            2 => Self::MsWord,
            _ => Self::HwpCurrent,
        }
    }
}

/// Compatible document settings.
#[derive(Debug, Clone, Copy)]
pub struct CompatibleDocument {
    target_program: TargetProgram,
}

impl CompatibleDocument {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let target = reader.read_u32()?;
        Ok(Self {
            target_program: TargetProgram::from_raw(target),
        })
    }

    /// Returns the target program for compatibility.
    pub const fn target_program(&self) -> TargetProgram {
        self.target_program
    }
}
