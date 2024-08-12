use crate::HwpDocumentError;
use std::{fs::File, path::Path};

pub trait HwpRead {
    fn header(&mut self) -> Result<Vec<u8>, HwpDocumentError>;

    fn doc_info(&mut self) -> Result<Vec<u8>, HwpDocumentError>;

    fn body_text(&mut self) -> impl Iterator<Item = Result<Vec<u8>, HwpDocumentError>>;

    fn view_text(&mut self) -> impl Iterator<Item = Result<Vec<u8>, HwpDocumentError>>;
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

    fn read_iter(
        &mut self,
        path: &'static str,
    ) -> impl Iterator<Item = Result<Vec<u8>, HwpDocumentError>> + '_ {
        struct Iter<'hwp> {
            size: usize,
            index: usize,
            file: &'hwp mut cfb::CompoundFile<File>,
            path: &'hwp str,
        }

        impl<'hwp> Iterator for Iter<'hwp> {
            type Item = Result<Vec<u8>, HwpDocumentError>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index >= self.size {
                    return None;
                }

                let path = format!("{}/Section{}", self.path, self.index);

                match HwpReader::read(&mut self.file, &path) {
                    Ok(section) => {
                        self.index += 1;
                        Some(Ok(section))
                    }
                    Err(error) => Some(Err(HwpDocumentError::CannotFindSection(error.into()))),
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (0, Some(self.size))
            }
        }

        Iter {
            size: match self.file.read_storage(path) {
                Ok(storage) => storage.count(),
                Err(_) => 0,
            },
            index: 0,
            file: &mut self.file,
            path,
        }
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

    #[inline]
    fn body_text(&mut self) -> impl Iterator<Item = Result<Vec<u8>, HwpDocumentError>> {
        self.read_iter("/BodyText")
    }

    #[inline]
    fn view_text(&mut self) -> impl Iterator<Item = Result<Vec<u8>, HwpDocumentError>> {
        self.read_iter("/ViewText")
    }
}
