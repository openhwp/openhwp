use crate::{Body, DocInfo, FileHeader, HwpRead, HwpReader};
use std::path::Path;

#[derive(Debug)]
pub struct HwpDocument {
    pub file_header: FileHeader,
    pub doc_info: DocInfo,
    pub body: Body,
}

#[derive(Debug, Error)]
pub enum HwpDocumentError {
    #[error("Decompression error: {0}")]
    Decompression(#[from] std::io::Error),
    #[error("Invalid tag id: {0:?}")]
    InvalidTagId(u16),
    #[error("Cannot open file: {0}")]
    CannotOpenFile(anyhow::Error),
    #[error("Cannot find file header: {0}")]
    CannotFindFileHeader(anyhow::Error),
    #[error("Cannot find doc info: {0}")]
    CannotFindDocInfo(anyhow::Error),
    #[error("Cannot find section: {0}")]
    CannotFindSection(anyhow::Error),
    #[error("Invalid file header: {0}")]
    FileHeader(#[from] crate::FileHeaderError),
    #[error("Invalid section: {0}")]
    Section(#[from] crate::SectionError),
}

impl HwpDocument {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, HwpDocumentError> {
        let mut reader = HwpReader::from_path(path.as_ref())?;

        Self::from_reader(&mut reader)
    }

    pub fn from_reader<R: HwpRead>(reader: &mut R) -> Result<Self, HwpDocumentError> {
        let file_header = FileHeader::from_reader(reader)?;
        let doc_info = DocInfo::from_reader(reader, &file_header)?;
        let body = Body::from_reader(reader, &file_header)?;

        Ok(Self {
            file_header,
            doc_info,
            body,
        })
    }
}
