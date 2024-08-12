pub mod section;

pub use section::*;

use crate::{decompress, Compressed, FileHeader, HwpDocumentError, HwpRead};

#[derive(Debug)]
pub struct Body {
    pub non_distributed: Vec<Section>,
    pub distributed: Vec<Section>,
}

impl Body {
    pub fn from_reader<R: HwpRead>(
        reader: &mut R,
        file_header: &FileHeader,
    ) -> Result<Self, HwpDocumentError> {
        let compressed = file_header.properties.compressed;
        let non_distributed = Self::from_iter(reader.body_text(), compressed)?;
        let distributed = Self::from_iter(reader.view_text(), Compressed::Yes)?;

        Ok(Self {
            non_distributed,
            distributed,
        })
    }

    pub fn from_iter<I: Iterator<Item = Result<Vec<u8>, HwpDocumentError>>>(
        iter: I,
        compressed: Compressed,
    ) -> Result<Vec<Section>, HwpDocumentError> {
        let mut sections = match iter.size_hint() {
            (_, Some(size)) => Vec::with_capacity(size),
            _ => vec![],
        };
        for section in iter {
            let buf = section?;
            let buf = match compressed {
                Compressed::Yes => decompress(&buf)?,
                Compressed::No => buf,
            };
            sections.push(Section::from_vec(buf)?);
        }

        Ok(sections)
    }
}
