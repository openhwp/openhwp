pub mod section;

pub use section::*;

use crate::{decompress, FileHeader, HwpDocumentError, HwpRead};

#[derive(Debug)]
pub struct Body {
    pub sections: Vec<Section>,
}

impl Body {
    pub fn from_reader<R: HwpRead>(
        reader: &mut R,
        file_header: &FileHeader,
    ) -> Result<Self, HwpDocumentError> {
        Ok(Self::from_iter(
            reader.sections(),
            file_header.properties.compressed,
        )?)
    }

    pub fn from_iter<I: Iterator<Item = Result<Vec<u8>, HwpDocumentError>>>(
        iter: I,
        compressed: bool,
    ) -> Result<Self, HwpDocumentError> {
        let mut sections = vec![];
        for section in iter {
            let buf = section?;
            let buf = match compressed {
                true => decompress(&buf)?,
                false => buf,
            };
            sections.push(Section::from_vec(buf)?);
        }

        Ok(Self { sections })
    }
}
