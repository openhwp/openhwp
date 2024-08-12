use crate::HwpDocumentError;
use std::{fs::File, path::Path};

pub trait HwpRead {
    fn header(&mut self) -> Result<Vec<u8>, HwpDocumentError>;

    fn doc_info(&mut self) -> Result<Vec<u8>, HwpDocumentError>;

    fn sections(&mut self) -> impl Iterator<Item = Result<Vec<u8>, HwpDocumentError>>;
}

pub struct HwpReader {
    file: cfb::CompoundFile<File>,
}

impl HwpReader {
    pub fn from_path(path: &Path) -> Result<Self, HwpDocumentError> {
        Ok(Self {
            file: cfb::open(path)
                .map_err(|error| HwpDocumentError::CannotOpenFile(error.into()))?,
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
    fn header(&mut self) -> Result<Vec<u8>, HwpDocumentError> {
        Self::read(&mut self.file, "/FileHeader")
            .map_err(|error| HwpDocumentError::CannotFindFileHeader(error.into()))
    }

    fn doc_info(&mut self) -> Result<Vec<u8>, HwpDocumentError> {
        Self::read(&mut self.file, "/DocInfo")
            .map_err(|error| HwpDocumentError::CannotFindDocInfo(error.into()))
    }

    fn sections(&mut self) -> impl Iterator<Item = Result<Vec<u8>, HwpDocumentError>> {
        let size = match self.file.read_storage("/BodyText") {
            Ok(storage) => storage.count(),
            Err(_) => 0,
        };
        let mut index = 0;

        std::iter::from_fn(move || {
            if index >= size {
                return None;
            }

            match Self::read(&mut self.file, &format!("/BodyText/Section{}", index)) {
                Ok(section) => {
                    index += 1;
                    Some(Ok(section))
                }
                Err(error) => Some(Err(HwpDocumentError::CannotFindSection(error.into()))),
            }
        })
    }
}
