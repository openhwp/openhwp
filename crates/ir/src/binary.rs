//! 바이너리 데이터
//!
//! 문서에 포함된 이미지, OLE 객체 등의 바이너리 데이터를 관리합니다.

use primitive::BinaryDataId;
use std::collections::HashMap;

/// 바이너리 데이터 저장소
#[derive(Debug, Clone, Default)]
pub struct BinaryDataStore {
    /// 데이터 맵 (ID → 데이터)
    data: HashMap<BinaryDataId, BinaryData>,
}

impl BinaryDataStore {
    /// 빈 저장소 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 바이너리 데이터 추가
    pub fn add(&mut self, id: BinaryDataId, data: BinaryData) {
        self.data.insert(id, data);
    }

    /// 바이너리 데이터 가져오기
    pub fn get(&self, id: &BinaryDataId) -> Option<&BinaryData> {
        self.data.get(id)
    }

    /// 바이너리 데이터 가져오기 (mutable)
    pub fn get_mut(&mut self, id: &BinaryDataId) -> Option<&mut BinaryData> {
        self.data.get_mut(id)
    }

    /// 바이너리 데이터 삭제
    pub fn remove(&mut self, id: &BinaryDataId) -> Option<BinaryData> {
        self.data.remove(id)
    }

    /// 모든 ID 반환
    pub fn ids(&self) -> impl Iterator<Item = &BinaryDataId> {
        self.data.keys()
    }

    /// 모든 데이터 반환
    pub fn iter(&self) -> impl Iterator<Item = (&BinaryDataId, &BinaryData)> {
        self.data.iter()
    }

    /// 데이터 수 반환
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 비어있는지 확인
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 새 ID로 데이터 추가 후 ID 반환
    pub fn add_with_auto_id(&mut self, data: BinaryData) -> BinaryDataId {
        let id = BinaryDataId::new(format!("BIN{:04}", self.data.len()));
        self.data.insert(id.clone(), data);
        id
    }
}

/// 바이너리 데이터
#[derive(Debug, Clone)]
pub struct BinaryData {
    /// 데이터 형식
    pub format: BinaryFormat,
    /// 원본 파일명 (있는 경우)
    pub filename: Option<String>,
    /// 압축 여부
    pub compressed: bool,
    /// 데이터 내용
    pub data: Vec<u8>,
}

impl BinaryData {
    /// 바이너리 데이터 생성
    pub const fn new(format: BinaryFormat, data: Vec<u8>) -> Self {
        Self {
            format,
            filename: None,
            compressed: false,
            data,
        }
    }

    /// 파일명과 함께 생성
    pub fn with_filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
        self
    }

    /// 데이터 크기 반환
    pub const fn len(&self) -> usize {
        self.data.len()
    }

    /// 비어있는지 확인
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 확장자에서 형식 추측
    pub fn format_from_extension(ext: &str) -> BinaryFormat {
        match ext.to_lowercase().as_str() {
            "bmp" => BinaryFormat::Bmp,
            "jpg" | "jpeg" => BinaryFormat::Jpg,
            "png" => BinaryFormat::Png,
            "gif" => BinaryFormat::Gif,
            "tif" | "tiff" => BinaryFormat::Tiff,
            "wmf" => BinaryFormat::Wmf,
            "emf" => BinaryFormat::Emf,
            "ole" => BinaryFormat::Ole,
            "mp4" | "avi" | "wmv" | "mov" => BinaryFormat::Video,
            _ => BinaryFormat::Unknown,
        }
    }
}

/// 바이너리 데이터 형식
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BinaryFormat {
    /// 알 수 없음
    #[default]
    Unknown,
    /// BMP 이미지
    Bmp,
    /// JPEG 이미지
    Jpg,
    /// PNG 이미지
    Png,
    /// GIF 이미지
    Gif,
    /// TIFF 이미지
    Tiff,
    /// WMF (Windows Metafile)
    Wmf,
    /// EMF (Enhanced Metafile)
    Emf,
    /// OLE 객체
    Ole,
    /// 비디오
    Video,
}

impl BinaryFormat {
    /// MIME 타입 반환
    pub const fn mime_type(&self) -> &'static str {
        match self {
            BinaryFormat::Bmp => "image/bmp",
            BinaryFormat::Jpg => "image/jpeg",
            BinaryFormat::Png => "image/png",
            BinaryFormat::Gif => "image/gif",
            BinaryFormat::Tiff => "image/tiff",
            BinaryFormat::Wmf => "image/x-wmf",
            BinaryFormat::Emf => "image/x-emf",
            BinaryFormat::Ole => "application/x-ole-object",
            BinaryFormat::Video => "video/*",
            BinaryFormat::Unknown => "application/octet-stream",
        }
    }

    /// 확장자 반환
    pub const fn extension(&self) -> &'static str {
        match self {
            BinaryFormat::Bmp => "bmp",
            BinaryFormat::Jpg => "jpg",
            BinaryFormat::Png => "png",
            BinaryFormat::Gif => "gif",
            BinaryFormat::Tiff => "tiff",
            BinaryFormat::Wmf => "wmf",
            BinaryFormat::Emf => "emf",
            BinaryFormat::Ole => "ole",
            BinaryFormat::Video => "mp4",
            BinaryFormat::Unknown => "bin",
        }
    }

    /// 이미지 형식인지 확인
    pub const fn is_image(&self) -> bool {
        matches!(
            self,
            BinaryFormat::Bmp
                | BinaryFormat::Jpg
                | BinaryFormat::Png
                | BinaryFormat::Gif
                | BinaryFormat::Tiff
                | BinaryFormat::Wmf
                | BinaryFormat::Emf
        )
    }
}
