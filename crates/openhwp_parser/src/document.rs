use crate::{DocInfo, FileHeader, Section};
use cfb::CompoundFile;
use std::{fs::File, path::Path};

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

        let mut index = 0;
        let sections =
            std::iter::from_fn(
                || match read(&mut file, format!("BodyText/Section{:04}", index)) {
                    Ok(section) => {
                        index += 1;
                        match Section::from_vec(section) {
                            Ok(section) => Some(Ok(section)),
                            Err(error) => Some(Err(HwpError::Section(error))),
                        }
                    }
                    Err(error) => Some(Err(HwpError::CannotFindSection(error))),
                },
            )
            .take_while(|result| result.is_ok())
            .filter_map(Result::ok)
            .collect();

        Ok(Self {
            file_header,
            doc_info,
            sections,
        })
    }
}

fn read<P: AsRef<Path>>(file: &mut CompoundFile<File>, path: P) -> Result<Vec<u8>, std::io::Error> {
    #[inline(never)]
    fn read_to_end(file: &mut CompoundFile<File>, path: &Path) -> Result<Vec<u8>, std::io::Error> {
        use std::io::Read;

        let mut stream = file.open_stream(path)?;
        let mut buf = vec![];
        stream.read_to_end(&mut buf)?;

        Ok(buf)
    }

    read_to_end(file, path.as_ref())
}
