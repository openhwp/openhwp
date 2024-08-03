use super::{IdMappingCounts, IdMappingsError};
use crate::{u16, u32, DocInfoError, DocInfoTag, RecordIter};

#[derive(Debug)]
pub struct BinData {
    pub kind: BinDataKind,
    pub compression: Compression,
    pub state: State,
}

#[derive(Debug)]
pub enum BinDataKind {
    Link {
        /// Type이 "LINK"일 때, 연결 파일의 절대 경로 길이 (len1)
        absolute_path_size: u16,
        /// Type이 "LINK"일 때, 연결 파일의 절대 경로
        absolute_path: String,
        /// Type이 "LINK"일 때, 연결 파일의 상대 경로 길이 (len2)
        relative_path_size: u16,
        /// Type이 "LINK"일 때, 연결 파일의 상대 경로
        relative_path: String,
    },
    Embedding {
        /// Type이 "EMBEDDING"이거나 "STORAGE"일 때, BINDATASTORAGE에 저장된 바이너리 데이터의 아이디
        id: u16,
        /// Type이 "EMBEDDING"일 때, 바이너리 데이터의 형식 이름의 길이 (len3)
        extension_size: u16,
        /// Type이 "EMBEDDING"일 때 extension("." 제외)
        extension: Extension,
    },
    Storage {
        id: u32,
    },
}

#[derive(Debug)]
pub enum Extension {
    Jpg,
    Bmp,
    Gif,
    Png,
    Wmf,
    Ole,
    Unknown(String),
}

#[derive(Debug)]
pub enum Compression {
    Default,
    Yes,
    No,
}

#[derive(Debug)]
pub enum State {
    NoAccessed,
    Accessed,
    Failed,
    Ignored,
}

#[derive(Debug, Error)]
pub enum BinDataError {
    #[error("Unknown compression: {0}")]
    UnknownCompression(u16),
    #[error("Unknown state: {0}")]
    UnknownState(u16),
    #[error("Unknown bin data type: {0}")]
    UnknownDataType(u16),
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn bin_data(
        &mut self,
        id_mappings: &IdMappingCounts,
    ) -> Result<Vec<BinData>, DocInfoError> {
        let mut bin_data = Vec::with_capacity(id_mappings.binary_data as usize);
        for record in self
            .take(id_mappings.binary_data as usize)
            .take_while(|record| record.tag_id == DocInfoTag::HWPTAG_BIN_DATA as u16)
        {
            bin_data.push(BinData::from_buf(record.payload)?);
        }

        Ok(bin_data)
    }
}

impl BinData {
    pub fn from_buf(buf: &[u8]) -> Result<Self, DocInfoError> {
        let attribute = u16(buf, 0);

        Ok(Self {
            kind: match attribute & 0x000f {
                0x0000 => {
                    let absolute_path_size = u16(buf, 2);
                    let absolute_path_start = 4;
                    let absolute_path_end = absolute_path_start + (absolute_path_size * 2) as usize;
                    let path = unsafe {
                        std::mem::transmute(&buf[absolute_path_start..absolute_path_end])
                    };
                    let absolute_path = String::from_utf16_lossy(path).to_string();

                    let relative_path_start = absolute_path_end;
                    let relative_path_size = u16(buf, relative_path_start as usize);
                    let relative_path_end = relative_path_start + (relative_path_size * 2) as usize;
                    let path = unsafe {
                        std::mem::transmute(&buf[relative_path_start..relative_path_end])
                    };
                    let relative_path = String::from_utf16_lossy(path).to_string();

                    BinDataKind::Link {
                        absolute_path_size,
                        absolute_path,
                        relative_path_size,
                        relative_path,
                    }
                }
                0x00001 => {
                    let id = u16(buf, 2);
                    let extension_size = u16(buf, 4);
                    let extension: Vec<_> = buf[6..(6 + (extension_size * 2) as usize)]
                        .chunks_exact(2)
                        .map(|c| u16::from_le_bytes([c[0], c[1]]))
                        .collect();
                    let extension = String::from_utf16_lossy(&extension)
                        .to_string()
                        .to_lowercase();

                    BinDataKind::Embedding {
                        id,
                        extension_size,
                        extension: match extension.as_str() {
                            "jpg" => Extension::Jpg,
                            "bmp" => Extension::Bmp,
                            "gif" => Extension::Gif,
                            "png" => Extension::Png,
                            "wmf" => Extension::Wmf,
                            "ole" => Extension::Ole,
                            _ => Extension::Unknown(extension),
                        },
                    }
                }
                0x00002 => BinDataKind::Storage { id: u32(buf, 4) },
                r#type => Err(IdMappingsError::BinaryData(BinDataError::UnknownDataType(
                    r#type,
                )))?,
            },
            compression: match attribute & 0x00f0 {
                0x0000 => Compression::Default,
                0x0010 => Compression::Yes,
                0x0020 => Compression::No,
                compression => Err(IdMappingsError::BinaryData(
                    BinDataError::UnknownCompression(compression),
                ))?,
            },
            state: match attribute & 0x0f00 {
                0x0000 => State::NoAccessed,
                0x0100 => State::Accessed,
                0x0200 => State::Failed,
                0x0300 => State::Ignored,
                state => Err(IdMappingsError::BinaryData(BinDataError::UnknownState(
                    state,
                )))?,
            },
        })
    }
}
