#[cfg(test)]
mod tests;

pub mod paragraph;
pub mod section;

mod stream;

pub use paragraph::*;
pub use section::*;
pub use stream::*;

use crate::{Compressed, FileHeader, HwpDocumentError, HwpRead, Version};

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
        let version = &file_header.version;
        let non_distributed = Self::from_reader_for_non_distributed(reader, compressed, version)?;
        let distributed = Self::from_reader_for_distributed(reader, compressed, version)?;

        Ok(Self {
            non_distributed,
            distributed,
        })
    }

    pub fn from_reader_for_non_distributed<R: HwpRead>(
        reader: &mut R,
        compressed: Compressed,
        version: &Version,
    ) -> Result<Vec<Section>, HwpDocumentError> {
        let iter = reader.body_text();
        let mut sections = match iter.size_hint() {
            (_, Some(size)) => Vec::with_capacity(size),
            _ => vec![],
        };
        for section in iter {
            let section = Section::from_non_distributed(section?, compressed, version)?;
            sections.push(section);
        }

        Ok(sections)
    }

    pub fn from_reader_for_distributed<R: HwpRead>(
        reader: &mut R,
        compressed: Compressed,
        version: &Version,
    ) -> Result<Vec<Section>, HwpDocumentError> {
        let iter = reader.view_text();
        let mut sections = match iter.size_hint() {
            (_, Some(size)) => Vec::with_capacity(size),
            _ => vec![],
        };
        for section in iter {
            let section = Section::from_distributed(section?, compressed, version)?;
            sections.push(section);
        }

        Ok(sections)
    }
}
