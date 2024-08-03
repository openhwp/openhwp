const HWPTAG_BEGIN: u16 = 0x010;

#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum DocInfoTag {
    HWPTAG_DOCUMENT_PROPERTIES = HWPTAG_BEGIN,

    HWPTAG_ID_MAPPINGS = HWPTAG_BEGIN + 1,
    HWPTAG_BIN_DATA = HWPTAG_BEGIN + 2,
    HWPTAG_FACE_NAME = HWPTAG_BEGIN + 3,
    HWPTAG_BORDER_FILL = HWPTAG_BEGIN + 4,
    HWPTAG_CHAR_SHAPE = HWPTAG_BEGIN + 5,
    HWPTAG_TAB_DEF = HWPTAG_BEGIN + 6,
    HWPTAG_NUMBERING = HWPTAG_BEGIN + 7,
    HWPTAG_BULLET = HWPTAG_BEGIN + 8,
    HWPTAG_PARA_SHAPE = HWPTAG_BEGIN + 9,
    HWPTAG_STYLE = HWPTAG_BEGIN + 10,
    HWPTAG_DOC_DATA = HWPTAG_BEGIN + 11,
    HWPTAG_DISTRIBUTE_DOC_DATA = HWPTAG_BEGIN + 12,
    RESERVED = HWPTAG_BEGIN + 13,
    HWPTAG_COMPATIBLE_DOCUMENT = HWPTAG_BEGIN + 14,
    HWPTAG_LAYOUT_COMPATIBILITY = HWPTAG_BEGIN + 15,
    HWPTAG_TRACKCHANGE = HWPTAG_BEGIN + 16,
    HWPTAG_MEMO_SHAPE = HWPTAG_BEGIN + 76,
    HWPTAG_FORBIDDEN_CHAR = HWPTAG_BEGIN + 78,
    HWPTAG_TRACK_CHANGE = HWPTAG_BEGIN + 80,
    HWPTAG_TRACK_CHANGE_AUTHOR = HWPTAG_BEGIN + 81,
}