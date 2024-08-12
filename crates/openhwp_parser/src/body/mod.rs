pub mod section;

pub use section::*;

use crate::{HwpDocumentError, HwpRead};

#[derive(Debug)]
pub struct Body {
    pub sections: Vec<Section>,
}

#[derive(Debug, Error)]
pub enum BodyError {
    #[error("Invalid body")]
    InvalidBody,
    #[error("Invalid section: {0}")]
    Section(#[from] SectionError),
}

impl Body {
    pub fn from_reader<R: HwpRead>(reader: &mut R) -> Result<Self, HwpDocumentError> {
        let mut sections = vec![];
        for section in reader.sections() {
            sections.push(Section::from_vec(section?).map_err(BodyError::Section)?);
        }

        Ok(Self { sections })
    }
}
