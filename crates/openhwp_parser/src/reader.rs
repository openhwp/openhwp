use crate::HwpDocumentError;
use std::{fs::File, path::Path};

#[derive(Debug, Clone)]
pub struct EmbeddedStream {
    pub name: String,
    pub data: Vec<u8>,
}

pub trait HwpRead {
    fn header(&mut self) -> Result<Vec<u8>, HwpDocumentError>;

    fn doc_info(&mut self) -> Result<Vec<u8>, HwpDocumentError>;

    fn body_text(&mut self) -> impl Iterator<Item = Result<Vec<u8>, HwpDocumentError>>;

    fn view_text(&mut self) -> impl Iterator<Item = Result<Vec<u8>, HwpDocumentError>>;

    fn summary_information(&mut self) -> Option<Vec<u8>>;

    fn preview_text(&mut self) -> Option<Vec<u8>>;

    fn preview_image(&mut self) -> Option<Vec<u8>>;

    fn bin_data(&mut self) -> Vec<EmbeddedStream>;

    fn doc_options(&mut self) -> Vec<EmbeddedStream>;

    fn scripts(&mut self) -> Vec<EmbeddedStream>;

    fn doc_history(&mut self) -> Vec<EmbeddedStream>;
}

pub struct HwpReader {
    file: cfb::CompoundFile<File>,
}

impl HwpReader {
    pub fn from_path(path: &Path) -> Result<Self, HwpDocumentError> {
        match cfb::open(path) {
            Ok(file) => Ok(Self { file }),
            Err(error) => Err(HwpDocumentError::CannotOpenFile(error.into())),
        }
    }

    fn read(file: &mut cfb::CompoundFile<File>, path: &str) -> Result<Vec<u8>, std::io::Error> {
        use std::io::Read;

        let mut stream = file.open_stream(path)?;
        let mut buf = vec![];
        stream.read_to_end(&mut buf)?;

        Ok(buf)
    }

    fn read_optional(file: &mut cfb::CompoundFile<File>, path: &str) -> Option<Vec<u8>> {
        Self::read(file, path).ok()
    }

    fn read_storage_streams(
        file: &mut cfb::CompoundFile<File>,
        storage: &str,
    ) -> Vec<EmbeddedStream> {
        let names: Vec<String> = match file.read_storage(storage) {
            Ok(entries) => entries
                .filter(|entry| entry.is_stream())
                .map(|entry| entry.name().to_owned())
                .collect(),
            Err(_) => vec![],
        };

        let mut streams = Vec::with_capacity(names.len());
        for name in names {
            let path = format!("{}/{}", storage, name);
            if let Ok(data) = Self::read(file, &path) {
                streams.push(EmbeddedStream {
                    name: format!("{}/{}", storage.trim_start_matches('/'), name),
                    data,
                });
            }
        }

        streams
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

    #[inline]
    fn summary_information(&mut self) -> Option<Vec<u8>> {
        Self::read_optional(&mut self.file, "\u{0005}HwpSummaryInformation")
    }

    #[inline]
    fn preview_text(&mut self) -> Option<Vec<u8>> {
        Self::read_optional(&mut self.file, "PrvText")
    }

    #[inline]
    fn preview_image(&mut self) -> Option<Vec<u8>> {
        Self::read_optional(&mut self.file, "PrvImage")
    }

    #[inline]
    fn bin_data(&mut self) -> Vec<EmbeddedStream> {
        Self::read_storage_streams(&mut self.file, "/BinData")
    }

    #[inline]
    fn doc_options(&mut self) -> Vec<EmbeddedStream> {
        Self::read_storage_streams(&mut self.file, "/DocOptions")
    }

    #[inline]
    fn scripts(&mut self) -> Vec<EmbeddedStream> {
        Self::read_storage_streams(&mut self.file, "/Scripts")
    }

    #[inline]
    fn doc_history(&mut self) -> Vec<EmbeddedStream> {
        Self::read_storage_streams(&mut self.file, "/DocHistory")
    }
}
