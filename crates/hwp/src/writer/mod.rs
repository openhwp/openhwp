//! HWP 5.0 파일 생성 모듈
//!
//! IR 문서를 HWP 5.0 바이너리 형식으로 변환하는 기능을 제공합니다.
//!
//! # HWP 5.0 파일 구조
//!
//! HWP 5.0 파일은 OLE(CFB) 컨테이너 형식을 사용하며, 다음과 같은 스트림을 포함합니다:
//!
//! - `/FileHeader`: 256바이트 고정 크기의 파일 헤더
//! - `/DocInfo`: 문서 정보 (압축됨)
//! - `/BodyText/Section{N}`: 본문 내용 (압축됨)
//! - `/BinData/BIN{XXXX}.{ext}`: 바이너리 데이터 (이미지 등)

pub mod body_writer;
pub mod byte_writer;
pub mod doc_info_writer;
pub mod file_header_writer;

pub use body_writer::BodyWriter;
pub use byte_writer::ByteWriter;
pub use doc_info_writer::DocInfoWriter;
pub use file_header_writer::FileHeaderWriter;

use std::collections::HashMap;
use std::io::{Cursor, Write};

use cfb::CompoundFile;
use miniz_oxide::deflate::compress_to_vec;

use crate::error::{Error, Result};
use primitive::Version;

/// HWP 파일 생성기
///
/// IR 문서 또는 직접 데이터를 입력받아 HWP 5.0 바이너리 파일을 생성합니다.
pub struct HwpWriter {
    /// 문서 버전
    version: Version,
    /// FileHeader 데이터
    file_header: Vec<u8>,
    /// DocInfo 데이터 (압축 전)
    doc_info: Vec<u8>,
    /// BodyText 섹션들 (압축 전)
    sections: Vec<Vec<u8>>,
    /// 바이너리 데이터 (이미지 등)
    binary_data: HashMap<String, Vec<u8>>,
    /// 압축 활성화 여부
    compress: bool,
}

impl HwpWriter {
    /// 새 HwpWriter를 생성합니다.
    pub fn new() -> Self {
        Self {
            version: Version::V5_0_3_0,
            file_header: Vec::new(),
            doc_info: Vec::new(),
            sections: Vec::new(),
            binary_data: HashMap::new(),
            compress: true,
        }
    }

    /// 문서 버전을 설정합니다.
    pub fn with_version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }

    /// 압축 활성화 여부를 설정합니다.
    pub fn with_compression(mut self, compress: bool) -> Self {
        self.compress = compress;
        self
    }

    /// FileHeader 데이터를 설정합니다.
    pub fn set_file_header(&mut self, data: Vec<u8>) {
        self.file_header = data;
    }

    /// DocInfo 데이터를 설정합니다 (압축 전).
    pub fn set_doc_info(&mut self, data: Vec<u8>) {
        self.doc_info = data;
    }

    /// 섹션 데이터를 추가합니다 (압축 전).
    pub fn add_section(&mut self, data: Vec<u8>) {
        self.sections.push(data);
    }

    /// 바이너리 데이터를 추가합니다.
    pub fn add_binary_data(&mut self, name: String, data: Vec<u8>) {
        self.binary_data.insert(name, data);
    }

    /// HWP 파일을 바이트로 생성합니다.
    pub fn write_to_bytes(&self) -> Result<Vec<u8>> {
        let cursor = Cursor::new(Vec::new());
        let mut cfb =
            CompoundFile::create(cursor).map_err(|e| Error::Io(std::io::Error::other(e)))?;

        // FileHeader 스트림 생성
        self.write_file_header(&mut cfb)?;

        // DocInfo 스트림 생성
        self.write_doc_info(&mut cfb)?;

        // BodyText 스트림들 생성
        self.write_body_text(&mut cfb)?;

        // BinData 스트림들 생성
        self.write_bin_data(&mut cfb)?;

        // CFB 파일 완성
        cfb.flush()
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;

        let cursor = cfb.into_inner();
        Ok(cursor.into_inner())
    }

    fn write_file_header<W: std::io::Read + std::io::Write + std::io::Seek>(
        &self,
        cfb: &mut CompoundFile<W>,
    ) -> Result<()> {
        let mut stream = cfb
            .create_stream("/FileHeader")
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;

        let header_data = if self.file_header.is_empty() {
            FileHeaderWriter::new(self.version, self.compress).build()
        } else {
            self.file_header.clone()
        };

        stream.write_all(&header_data)?;

        Ok(())
    }

    fn write_doc_info<W: std::io::Read + std::io::Write + std::io::Seek>(
        &self,
        cfb: &mut CompoundFile<W>,
    ) -> Result<()> {
        let mut stream = cfb
            .create_stream("/DocInfo")
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;

        let data = if self.compress {
            compress_to_vec(&self.doc_info, 6)
        } else {
            self.doc_info.clone()
        };

        stream.write_all(&data)?;

        Ok(())
    }

    fn write_body_text<W: std::io::Read + std::io::Write + std::io::Seek>(
        &self,
        cfb: &mut CompoundFile<W>,
    ) -> Result<()> {
        // BodyText 디렉토리 생성
        cfb.create_storage("/BodyText")
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;

        for (i, section_data) in self.sections.iter().enumerate() {
            let stream_name = format!("/BodyText/Section{}", i);
            let mut stream = cfb
                .create_stream(&stream_name)
                .map_err(|e| Error::Io(std::io::Error::other(e)))?;

            let data = if self.compress {
                compress_to_vec(section_data, 6)
            } else {
                section_data.clone()
            };

            stream.write_all(&data)?;
        }

        Ok(())
    }

    fn write_bin_data<W: std::io::Read + std::io::Write + std::io::Seek>(
        &self,
        cfb: &mut CompoundFile<W>,
    ) -> Result<()> {
        if self.binary_data.is_empty() {
            return Ok(());
        }

        // BinData 디렉토리 생성
        cfb.create_storage("/BinData")
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;

        for (name, data) in &self.binary_data {
            let stream_name = format!("/BinData/{}", name);
            let mut stream = cfb
                .create_stream(&stream_name)
                .map_err(|e| Error::Io(std::io::Error::other(e)))?;

            // BinData는 압축 여부에 따라 다르게 처리
            let final_data = if self.compress {
                compress_to_vec(data, 6)
            } else {
                data.clone()
            };

            stream.write_all(&final_data)?;
        }

        Ok(())
    }
}
