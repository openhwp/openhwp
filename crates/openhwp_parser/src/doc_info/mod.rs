mod record;

use record::*;

#[derive(Debug, Default)]
pub struct DocInfo {
    pub section_size: u16,
    pub starting_index: StartingIndex,
    pub carat_location: CaratLocation,
    pub id_mapping_count: IdMappingCount,
    pub bin_data: Vec<BinData>,
}

#[derive(Debug, Default)]
pub struct StartingIndex {
    pub page: u16,
    pub footnote: u16,
    pub endnote: u16,
    pub picture: u16,
    pub table: u16,
    pub equation: u16,
}

#[derive(Debug, Default)]
pub struct CaratLocation {
    pub list_id: u32,
    pub paragraph_id: u32,
    pub char_index: u32,
}

#[derive(Debug, Default)]
pub struct IdMappingCount {
    /// 바이너리 데이터
    binary_data: u32,
    /// 한글 글꼴
    hangul_font: u32,
    /// 영어 글꼴
    english_font: u32,
    /// 한자 글꼴
    chinese_font: u32,
    /// 일어 글꼴
    japanese_font: u32,
    /// 기타 글꼴
    etc_font: u32,
    /// 기호 글꼴
    symbol_font: u32,
    /// 사용자 글꼴
    user_font: u32,
    /// 테두리/배경
    border_fill: u32,
    /// 글자 모양
    char_shape: u32,
    /// 탭 정의
    tab_def: u32,
    /// 문단 번호
    numbering: u32,
    /// 글머리표
    bullet: u32,
    /// 문단 모양
    paragraph_shape: u32,
    /// 스타일
    style: u32,
    /// 메모 모양 (5.0.2.1 이상)
    memo_shape: u32,
    /// 변경추적 (5.0.3.2 이상)
    track_change: u32,
    /// 변경추적 사용자 (5.0.3.2 이상)
    track_change_author: u32,
}

#[derive(Debug)]
pub struct BinData {
    pub compression: Compression,
    pub state: State,
    pub kind: BinDataKind,
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

#[derive(Debug)]
pub enum BinDataKind {
    Link {
        absolute_path_size: u16,
        relative_path_size: u16,
        absolute_path: String,
        relative_path: String,
    },
    Embedding {
        id: u32,
    },
    Storage {
        id: u32,
    },
}

#[derive(Debug, Error)]
pub enum DocInfoError {
    #[error("{0}")]
    Record(#[from] RecordError),
}

impl DocInfo {
    pub fn from_vec(bytes: Vec<u8>, compressed: bool) -> Result<Self, DocInfoError> {
        let bytes = match compressed {
            true => decompress(&bytes)?,
            false => bytes,
        };
        let mut doc_info = DocInfo::default();
        for record in record::inflate(&bytes)? {
            visit(&mut doc_info, &record);
        }

        Ok(doc_info)
    }
}

fn decompress(bytes: &[u8]) -> Result<Vec<u8>, RecordError> {
    use flate2::bufread::DeflateDecoder;
    use std::io::Read;

    let mut buf = vec![];
    DeflateDecoder::new(bytes).read_to_end(&mut buf)?;

    Ok(buf)
}

fn visit(doc_info: &mut DocInfo, record: &Record) {
    const HWPTAG_BEGIN: u16 = 0x010;

    const HWPTAG_DOCUMENT_PROPERTIES: u16 = HWPTAG_BEGIN;
    const HWPTAG_ID_MAPPINGS: u16 = HWPTAG_BEGIN + 1;
    const HWPTAG_BIN_DATA: u16 = HWPTAG_BEGIN + 2;
    const HWPTAG_FACE_NAME: u16 = HWPTAG_BEGIN + 3;
    const HWPTAG_BORDER_FILL: u16 = HWPTAG_BEGIN + 4;
    const HWPTAG_CHAR_SHAPE: u16 = HWPTAG_BEGIN + 5;
    const HWPTAG_TAB_DEF: u16 = HWPTAG_BEGIN + 6;
    const HWPTAG_NUMBERING: u16 = HWPTAG_BEGIN + 7;
    const HWPTAG_BULLET: u16 = HWPTAG_BEGIN + 8;
    const HWPTAG_PARA_SHAPE: u16 = HWPTAG_BEGIN + 9;
    const HWPTAG_STYLE: u16 = HWPTAG_BEGIN + 10;
    const HWPTAG_DOC_DATA: u16 = HWPTAG_BEGIN + 11;
    const HWPTAG_DISTRIBUTE_DOC_DATA: u16 = HWPTAG_BEGIN + 12;
    const RESERVED: u16 = HWPTAG_BEGIN + 13;
    const HWPTAG_COMPATIBLE_DOCUMENT: u16 = HWPTAG_BEGIN + 14;
    const HWPTAG_LAYOUT_COMPATIBILITY: u16 = HWPTAG_BEGIN + 15;
    const HWPTAG_TRACKCHANGE: u16 = HWPTAG_BEGIN + 16;
    const HWPTAG_MEMO_SHAPE: u16 = HWPTAG_BEGIN + 76;
    const HWPTAG_FORBIDDEN_CHAR: u16 = HWPTAG_BEGIN + 78;
    const HWPTAG_TRACK_CHANGE: u16 = HWPTAG_BEGIN + 80;
    const HWPTAG_TRACK_CHANGE_AUTHOR: u16 = HWPTAG_BEGIN + 81;

    match record.tag_id {
        HWPTAG_DOCUMENT_PROPERTIES => visit_document_properties(doc_info, record),
        HWPTAG_ID_MAPPINGS => visit_id_mappings(doc_info, record),
        HWPTAG_BIN_DATA => visit_bin_data(doc_info, record),
        _ => (),
    }

    for record in &record.children {
        visit(doc_info, record);
    }
}

fn visit_document_properties(doc_info: &mut DocInfo, record: &Record) {
    let buf = &record.payload;

    doc_info.section_size = u16(buf);

    doc_info.starting_index.page = u16(&buf[2..]);
    doc_info.starting_index.footnote = u16(&buf[4..]);
    doc_info.starting_index.endnote = u16(&buf[6..]);
    doc_info.starting_index.picture = u16(&buf[8..]);
    doc_info.starting_index.table = u16(&buf[10..]);
    doc_info.starting_index.equation = u16(&buf[12..]);

    doc_info.carat_location.list_id = u32(&buf[14..]);
    doc_info.carat_location.paragraph_id = u32(&buf[18..]);
    doc_info.carat_location.char_index = u32(&buf[22..]);
}

fn visit_id_mappings(doc_info: &mut DocInfo, record: &Record) {
    let buf = &record.payload;

    doc_info.id_mapping_count.binary_data = u32(buf);
    doc_info.id_mapping_count.hangul_font = u32(&buf[4..]);
    doc_info.id_mapping_count.english_font = u32(&buf[8..]);
    doc_info.id_mapping_count.chinese_font = u32(&buf[12..]);
    doc_info.id_mapping_count.japanese_font = u32(&buf[16..]);
    doc_info.id_mapping_count.etc_font = u32(&buf[20..]);
    doc_info.id_mapping_count.symbol_font = u32(&buf[24..]);
    doc_info.id_mapping_count.user_font = u32(&buf[28..]);
    doc_info.id_mapping_count.border_fill = u32(&buf[32..]);
    doc_info.id_mapping_count.char_shape = u32(&buf[36..]);
    doc_info.id_mapping_count.tab_def = u32(&buf[40..]);
    doc_info.id_mapping_count.numbering = u32(&buf[44..]);
    doc_info.id_mapping_count.bullet = u32(&buf[48..]);
    doc_info.id_mapping_count.paragraph_shape = u32(&buf[52..]);
    doc_info.id_mapping_count.style = u32(&buf[56..]);
    doc_info.id_mapping_count.memo_shape = u32(&buf[60..]);
    doc_info.id_mapping_count.track_change = u32(&buf[64..]);
    doc_info.id_mapping_count.track_change_author = u32(&buf[68..]);
}

fn visit_bin_data(doc_info: &mut DocInfo, record: &Record) {
    let buf = &record.payload;

    let attribute = u32(buf);

    let r#type = attribute & 0x000f;
    let compression = attribute & 0x00f0;
    let state = attribute & 0x0f00;
}

const fn u16(buf: &[u8]) -> u16 {
    <u16>::from_le_bytes([buf[0], buf[1]])
}

const fn u32(buf: &[u8]) -> u32 {
    <u32>::from_le_bytes([buf[0], buf[1], buf[2], buf[3]])
}
