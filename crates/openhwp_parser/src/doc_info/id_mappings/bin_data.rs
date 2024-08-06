use super::IdMappingCount;
use crate::{to_string, u16, u32, DocInfoTag, RecordIter};

#[derive(Debug)]
pub struct BinData {
    pub kind: BinDataKind,
    pub compression: Compression,
    pub state: State,
}

#[derive(Debug)]
pub enum BinDataKind {
    Link {
        /// Type이 "LINK"일 때, 연결 파일의 절대 경로
        absolute_path: String,
        /// Type이 "LINK"일 때, 연결 파일의 상대 경로
        relative_path: String,
    },
    Embedding {
        /// Type이 "EMBEDDING"이거나 "STORAGE"일 때, BINDATASTORAGE에 저장된 바이너리 데이터의 아이디
        id: u16,
        /// Type이 "EMBEDDING"일 때 extension("." 제외)
        extension: Extension,
    },
    Storage {
        id: u32,
    },
    Unknown(u16),
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
    Unknown(u16),
}

#[derive(Debug)]
pub enum State {
    NoAccessed,
    Accessed,
    Failed,
    Ignored,
    Unknown(u16),
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn bin_data(&mut self, id_mappings: &IdMappingCount) -> Vec<BinData> {
        let mut bin_data = Vec::with_capacity(id_mappings.binary_data as usize);

        for record in self
            .take(id_mappings.binary_data as usize)
            .take_while(|record| record.tag_id == DocInfoTag::HWPTAG_BIN_DATA as u16)
        {
            bin_data.push(BinData::from_buf(record.payload));
        }

        bin_data
    }
}

impl BinData {
    pub fn from_buf(buf: &[u8]) -> Self {
        let (attribute, buf) = buf.split_at(2);
        let attribute = u16(attribute, 0);

        Self {
            kind: match attribute & 0x000f {
                0x0000 => {
                    let (size, buf) = buf.split_at(2);
                    let size = u16(size, 0);
                    let (absolute_path, buf) = buf.split_at(2 * size as usize);
                    let absolute_path = to_string(absolute_path);

                    let (size, buf) = buf.split_at(2);
                    let size = u16(size, 0);
                    let (relative_path, _) = buf.split_at(2 * size as usize);
                    let relative_path = to_string(relative_path);

                    BinDataKind::Link {
                        absolute_path,
                        relative_path,
                    }
                }
                0x00001 => {
                    let (id, buf) = buf.split_at(2);
                    let id = u16(id, 0);
                    let (extension_size, buf) = buf.split_at(2);
                    let extension_size = u16(extension_size, 0);
                    let (extension, _) = buf.split_at(2 * extension_size as usize);
                    let extension = to_string(extension).to_lowercase();

                    BinDataKind::Embedding {
                        id,
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
                r#type => BinDataKind::Unknown(r#type),
            },
            compression: match attribute & 0x00f0 {
                0x0000 => Compression::Default,
                0x0010 => Compression::Yes,
                0x0020 => Compression::No,
                compression => Compression::Unknown(compression),
            },
            state: match attribute & 0x0f00 {
                0x0000 => State::NoAccessed,
                0x0100 => State::Accessed,
                0x0200 => State::Failed,
                0x0300 => State::Ignored,
                state => State::Unknown(state),
            },
        }
    }
}
