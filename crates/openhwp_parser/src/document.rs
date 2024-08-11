use crate::{DocInfo, FileHeader, Section};
use std::{fs::File, path::Path};

pub trait HwpRead {
    fn header(&mut self) -> Result<Vec<u8>, HwpError>;

    fn doc_info(&mut self) -> Result<Vec<u8>, HwpError>;

    fn sections(&mut self) -> impl Iterator<Item = Result<Vec<u8>, HwpError>>;
}

#[derive(Debug)]
pub struct HwpDocument {
    pub file_header: FileHeader,
    pub doc_info: DocInfo,
    pub sections: Vec<Section>,
}

pub struct HwpReader {
    file: cfb::CompoundFile<File>,
}

#[derive(Debug, Error)]
pub enum HwpError {
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
    #[error("Invalid doc info: {0}")]
    DocInfo(#[from] crate::DocInfoError),
    #[error("Invalid section: {0}")]
    Section(#[from] crate::SectionError),
}

impl HwpDocument {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, HwpError> {
        Self::from_reader(HwpReader::from_path(path.as_ref())?)
    }

    pub fn from_reader<R: HwpRead>(mut reader: R) -> Result<Self, HwpError> {
        let file_header = FileHeader::from_vec(reader.header()?)?;
        let doc_info = DocInfo::from_vec(
            reader.doc_info()?,
            file_header.properties.compressed,
            &file_header.version,
        )?;

        let mut sections = vec![];
        for section in reader.sections() {
            sections.push(Section::from_vec(section?)?);
        }

        Ok(Self {
            file_header,
            doc_info,
            sections,
        })
    }
}

impl HwpReader {
    pub fn from_path(path: &Path) -> Result<Self, HwpError> {
        Ok(Self {
            file: cfb::open(path).map_err(|error| HwpError::CannotOpenFile(error.into()))?,
        })
    }

    fn read(file: &mut cfb::CompoundFile<File>, path: &str) -> Result<Vec<u8>, std::io::Error> {
        use std::io::Read;

        let mut stream = file.open_stream(path)?;
        let mut buf = vec![];
        stream.read_to_end(&mut buf)?;

        Ok(buf)
    }
}

impl HwpRead for HwpReader {
    fn header(&mut self) -> Result<Vec<u8>, HwpError> {
        Self::read(&mut self.file, "/FileHeader")
            .map_err(|error| HwpError::CannotFindFileHeader(error.into()))
    }

    fn doc_info(&mut self) -> Result<Vec<u8>, HwpError> {
        Self::read(&mut self.file, "/DocInfo")
            .map_err(|error| HwpError::CannotFindDocInfo(error.into()))
    }

    fn sections(&mut self) -> impl Iterator<Item = Result<Vec<u8>, HwpError>> {
        let mut index = 0;

        std::iter::from_fn(move || {
            let path = format!("/BodyText/Section{:04}", index);
            if !self.file.exists(&path) {
                return None;
            }

            match Self::read(&mut self.file, &path) {
                Ok(section) => {
                    index += 1;
                    Some(Ok(section))
                }
                Err(error) => Some(Err(HwpError::CannotFindSection(error.into()))),
            }
        })
    }
}
