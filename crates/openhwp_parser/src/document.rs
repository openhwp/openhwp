use crate::{Body, DocInfo, EmbeddedStream, FileHeader, HwpRead, HwpReader, HwpTag};
use std::path::Path;

#[derive(Debug)]
pub struct HwpDocument {
    pub file_header: FileHeader,
    pub doc_info: DocInfo,
    pub body: Body,
    pub preview: Preview,
    pub storages: Storages,
}

#[derive(Debug, Error)]
pub enum HwpDocumentError {
    #[error("Decompression error: {0}")]
    Decompression(#[from] std::io::Error),
    #[error("End of record")]
    EndOfRecord,
    #[error("Invalid tag id: {0:?}, expected: {1:?}")]
    InvalidTagId(Option<HwpTag>, HwpTag),
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
    #[error("Section count mismatch for {stream} (expected {expected}, actual {actual})")]
    SectionCountMismatch {
        stream: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("Other error: {0}")]
    Any(#[from] anyhow::Error),
}

impl HwpDocument {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, HwpDocumentError> {
        let mut reader = HwpReader::from_path(path.as_ref())?;

        Self::from_reader(&mut reader)
    }

    pub fn from_reader<R: HwpRead>(reader: &mut R) -> Result<Self, HwpDocumentError> {
        let file_header = FileHeader::from_reader(reader)?;
        let doc_info = DocInfo::from_reader(reader, &file_header)?;
        let body = Body::from_reader(reader, &file_header, &doc_info)?;
        let preview = Preview::from_reader(reader);
        let storages = Storages::from_reader(reader);

        Ok(Self {
            file_header,
            doc_info,
            body,
            preview,
            storages,
        })
    }
}

#[derive(Debug, Default)]
pub struct Preview {
    pub summary_information: Option<Vec<u8>>,
    pub text: Option<Vec<u8>>,
    pub image: Option<Vec<u8>>,
}

impl Preview {
    pub fn from_reader<R: HwpRead>(reader: &mut R) -> Self {
        Self {
            summary_information: reader.summary_information(),
            text: reader.preview_text(),
            image: reader.preview_image(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Storages {
    pub bin_data: Vec<EmbeddedStream>,
    pub doc_options: Vec<EmbeddedStream>,
    pub scripts: Vec<EmbeddedStream>,
    pub doc_history: Vec<EmbeddedStream>,
}

impl Storages {
    pub fn from_reader<R: HwpRead>(reader: &mut R) -> Self {
        Self {
            bin_data: reader.bin_data(),
            doc_options: reader.doc_options(),
            scripts: reader.scripts(),
            doc_history: reader.doc_history(),
        }
    }
}
