#[cfg(test)]
mod tests;

pub mod paragraph;
pub mod section;

mod stream;

pub use paragraph::*;
pub use section::*;
pub use stream::*;

use crate::doc_info::memo_shape::MemoShape as DocMemoShape;
use crate::{Compressed, DocInfo, FileHeader, HwpDocumentError, HwpRead, Version};

#[derive(Debug)]
pub struct Body {
    pub non_distributed: Vec<Section>,
    pub distributed: Vec<Section>,
    pub memos: MemoStore,
}

impl Body {
    pub fn from_reader<R: HwpRead>(
        reader: &mut R,
        file_header: &FileHeader,
        doc_info: &DocInfo,
    ) -> Result<Self, HwpDocumentError> {
        let compressed = file_header.properties.compressed;
        let version = &file_header.version;
        let expected_sections = doc_info.document_properties.section_size as usize;
        let non_distributed = Self::from_reader_for_non_distributed(
            reader,
            compressed,
            version,
            (!file_header.properties.distribution).then_some(expected_sections),
        )?;
        let distributed = if file_header.properties.distribution {
            Self::from_reader_for_distributed(reader, compressed, version, Some(expected_sections))?
        } else {
            vec![]
        };

        let memos = MemoStore::from_sections(
            &doc_info.id_mappings.memo_shapes,
            &non_distributed,
            &distributed,
        );

        Ok(Self {
            non_distributed,
            distributed,
            memos,
        })
    }

    pub fn from_reader_for_non_distributed<R: HwpRead>(
        reader: &mut R,
        compressed: Compressed,
        version: &Version,
        expected_sections: Option<usize>,
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

        Self::validate_sections(sections, expected_sections, "BodyText")
    }

    pub fn from_reader_for_distributed<R: HwpRead>(
        reader: &mut R,
        compressed: Compressed,
        version: &Version,
        expected_sections: Option<usize>,
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

        Self::validate_sections(sections, expected_sections, "ViewText")
    }

    fn validate_sections(
        sections: Vec<Section>,
        expected_sections: Option<usize>,
        stream: &'static str,
    ) -> Result<Vec<Section>, HwpDocumentError> {
        if let Some(expected) = expected_sections {
            if sections.len() != expected {
                return Err(HwpDocumentError::SectionCountMismatch {
                    stream,
                    expected,
                    actual: sections.len(),
                });
            }
        }

        Ok(sections)
    }
}

#[derive(Debug, Default)]
pub struct MemoStore {
    pub doc_shapes: Vec<DocMemoShape>,
    pub section_shapes: Vec<SectionMemoShape>,
    pub lists: Vec<MemoListHeader>,
}

impl MemoStore {
    fn from_sections(
        doc_shapes: &[DocMemoShape],
        non_distributed: &[Section],
        distributed: &[Section],
    ) -> Self {
        let mut store = Self {
            doc_shapes: doc_shapes.to_vec(),
            ..Self::default()
        };
        for memo in distributed
            .iter()
            .chain(non_distributed.iter())
            .flat_map(|section| &section.memos)
        {
            match memo {
                MemoRecord::Shape(shape) => store.section_shapes.push(shape.clone()),
                MemoRecord::List(list) => store.lists.push(list.clone()),
            }
        }

        store
    }
}
