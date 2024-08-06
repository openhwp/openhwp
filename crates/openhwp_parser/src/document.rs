use crate::{section, DocInfo, FileHeader, Section};
use cfb::CompoundFile;
use std::fs::File;

#[derive(Debug)]
pub struct HwpDocument {
    pub file_header: FileHeader,
    pub doc_info: DocInfo,
    pub sections: Vec<Section>,
}

#[derive(Debug, Error)]
pub enum HwpError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Cannot find file header: {0}")]
    CannotFindFileHeader(std::io::Error),
    #[error("Invalid file header: {0}")]
    FileHeader(#[from] crate::FileHeaderError),
    #[error("Cannot find doc info: {0}")]
    CannotFindDocInfo(std::io::Error),
    #[error("Invalid doc info: {0}")]
    DocInfo(#[from] crate::DocInfoError),
    #[error("Cannot find sections: {0}")]
    CannotFindSections(std::io::Error),
    #[error("Cannot find section: {0}")]
    CannotFindSection(std::io::Error),
    #[error("Malformed section: {0}")]
    MalformedSection(std::io::Error),
    #[error("Invalid section: {0}")]
    Section(#[from] crate::SectionError),
}

impl HwpDocument {
    pub fn from_path(path: &str) -> Result<Self, HwpError> {
        let mut file = cfb::open(path)?;

        let file_header = read(&mut file, "FileHeader").map_err(HwpError::CannotFindFileHeader)?;
        let file_header = FileHeader::from_vec(file_header)?;

        let doc_info = read(&mut file, "DocInfo").map_err(HwpError::CannotFindDocInfo)?;
        let doc_info = DocInfo::from_vec(
            doc_info,
            file_header.properties.compressed,
            &file_header.version,
        )?;

        let storages: Vec<_> = file
            .walk_storage("BodyText")
            .map_err(HwpError::CannotFindSections)?
            .collect();
        let mut sections = vec![];
        for section in storages {
            let mut stream = file
                .open_stream(section.path())
                .map_err(HwpError::CannotFindSections)?;
            let section = read_to_end(&mut stream).map_err(HwpError::MalformedSection)?;
            sections.push(Section::from_buf(&section).map_err(HwpError::Section)?);
        }

        Ok(Self {
            file_header,
            doc_info,
            sections,
        })
    }
}

fn read(file: &mut CompoundFile<File>, name: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut stream = file.open_stream(name)?;

    read_to_end(&mut stream)
}

fn read_to_end(stream: &mut cfb::Stream<File>) -> Result<Vec<u8>, std::io::Error> {
    use std::io::Read;

    let mut buf = vec![];
    stream.read_to_end(&mut buf)?;

    Ok(buf)
}
