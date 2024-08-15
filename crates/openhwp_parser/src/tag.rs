macro_rules! hwp_tag {
    (
        enum $doc_info_tag_ident:ident {
            $(
                $(#[$doc_info_attr:meta])*
                $doc_info_tag:ident = $doc_info_value:expr,
            )+
        }

        enum $body_tag_ident:ident {
            $(
                $(#[$body_attr:meta])*
                $body_tag:ident = $body_value:expr,
            )+
        }

        enum BothTAG {
            $(
                $(#[$both_attr:meta])*
                $both_tag:ident = $both_value:expr,
            )+
        }
    ) => {
        mod __tag {
            const HWPTAG_BEGIN: u8 = 0x010;

            // DocInfo
            $(
                pub const $doc_info_tag: u8 = $doc_info_value;
            )+

            // Body
            $(
                pub const $body_tag: u8 = $body_value;
            )+

            // Both
            $(
                pub const $both_tag: u8 = $both_value;
            )+
        }

        #[allow(non_camel_case_types)]
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum HwpTag {
            // DocInfo
            $(
                $(#[$doc_info_attr])*
                $doc_info_tag,
            )+

            // Body
            $(
                $(#[$body_attr])*
                $body_tag,
            )+

            // Both
            $(
                $(#[$both_attr])*
                $both_tag,
            )+

            /// Unknown
            Unknown(u8),
        }

        #[allow(non_camel_case_types)]
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $doc_info_tag_ident {
            $(
                $(#[$doc_info_attr])*
                $doc_info_tag,
            )+

            // Both
            $(
                $(#[$both_attr])*
                $both_tag,
            )+

            /// Unknown
            Unknown(u8),
        }

        #[allow(non_camel_case_types)]
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $body_tag_ident {
            $(
                $(#[$body_attr])*
                $body_tag,
            )+

            // Both
            $(
                $(#[$both_attr])*
                $both_tag,
            )+

            /// Unknown
            Unknown(u8),
        }

        impl HwpTag {
            pub const fn from_u16(tag: u16) -> Self {
                match tag as u8 {
                    $(
                        __tag::$doc_info_tag => Self::$doc_info_tag,
                    )+
                    $(
                        __tag::$body_tag => Self::$body_tag,
                    )+
                    $(
                        __tag::$both_tag => Self::$both_tag,
                    )+
                    tag => Self::Unknown(tag),
                }
            }
        }

        impl $doc_info_tag_ident {
            pub const fn from_u16(tag: u16) -> Self {
                match tag as u8 {
                    $(
                        __tag::$doc_info_tag => Self::$doc_info_tag,
                    )+
                    $(
                        __tag::$both_tag => Self::$both_tag,
                    )+
                    tag => Self::Unknown(tag),
                }
            }

            pub const fn as_hwp_tag(self) -> HwpTag {
                match self {
                    $(
                        Self::$doc_info_tag => HwpTag::$doc_info_tag,
                    )+
                    $(
                        Self::$both_tag => HwpTag::$both_tag,
                    )+
                    Self::Unknown(tag) => HwpTag::Unknown(tag),
                }
            }
        }

        impl $body_tag_ident {
            pub const fn from_u16(tag: u16) -> Self {
                match tag as u8 {
                    $(
                        __tag::$body_tag => Self::$body_tag,
                    )+
                    $(
                        __tag::$both_tag => Self::$both_tag,
                    )+
                    tag => Self::Unknown(tag),
                }
            }

            pub const fn as_hwp_tag(self) -> HwpTag {
                match self {
                    $(
                        Self::$body_tag => HwpTag::$body_tag,
                    )+
                    $(
                        Self::$both_tag => HwpTag::$both_tag,
                    )+
                    Self::Unknown(tag) => HwpTag::Unknown(tag),
                }
            }
        }
    };
}

hwp_tag! {
    enum DocInfoTag {
        /// 문서 속성
        HWPTAG_DOCUMENT_PROPERTIES = HWPTAG_BEGIN,
        /// 아이디 매핑 헤더
        HWPTAG_ID_MAPPINGS = HWPTAG_BEGIN + 1,
        /// BinData
        HWPTAG_BIN_DATA = HWPTAG_BEGIN + 2,
        /// Typeface Name
        HWPTAG_FACE_NAME = HWPTAG_BEGIN + 3,
        /// 테두리/배경
        HWPTAG_BORDER_FILL = HWPTAG_BEGIN + 4,
        /// 글자 모양
        HWPTAG_CHAR_SHAPE = HWPTAG_BEGIN + 5,
        /// 탭 정의
        HWPTAG_TAB_DEF = HWPTAG_BEGIN + 6,
        /// 번호 정의
        HWPTAG_NUMBERING = HWPTAG_BEGIN + 7,
        /// 불릿 정의
        HWPTAG_BULLET = HWPTAG_BEGIN + 8,
        /// 문단 모양
        HWPTAG_PARA_SHAPE = HWPTAG_BEGIN + 9,
        /// 스타일
        HWPTAG_STYLE = HWPTAG_BEGIN + 10,
        /// 문서의 임의의 데이터
        HWPTAG_DOC_DATA = HWPTAG_BEGIN + 11,
        /// 배포용 문서 데이터
        HWPTAG_DISTRIBUTE_DOC_DATA = HWPTAG_BEGIN + 12,
        /// 호환 문서
        HWPTAG_COMPATIBLE_DOCUMENT = HWPTAG_BEGIN + 14,
        /// 레이아웃 호환성
        HWPTAG_LAYOUT_COMPATIBILITY = HWPTAG_BEGIN + 15,
        /// 변경 추적 정보
        HWPTAG_TRACKCHANGE = HWPTAG_BEGIN + 16,
        /// 금칙처리 문자
        HWPTAG_FORBIDDEN_CHAR = HWPTAG_BEGIN + 78,
        /// 변경 추적 내용 및 모양
        HWPTAG_TRACK_CHANGE = HWPTAG_BEGIN + 80,
        /// 변경 추적 작성자
        HWPTAG_TRACK_CHANGE_AUTHOR = HWPTAG_BEGIN + 81,
    }

    enum BodyTag {
        /// 문단 헤더
        HWPTAG_PARA_HEADER = HWPTAG_BEGIN + 50,
        /// 문단의 텍스트
        HWPTAG_PARA_TEXT = HWPTAG_BEGIN + 51,
        /// 문단의 글자 모양
        HWPTAG_PARA_CHAR_SHAPE = HWPTAG_BEGIN + 52,
        /// 문단의 레이아웃
        HWPTAG_PARA_LINE_SEG = HWPTAG_BEGIN + 53,
        /// 문단의 영역 태그
        HWPTAG_PARA_RANGE_TAG = HWPTAG_BEGIN + 54,
        /// 컨트롤 헤더
        HWPTAG_CTRL_HEADER = HWPTAG_BEGIN + 55,
        /// 문단 리스트 헤더
        HWPTAG_LIST_HEADER = HWPTAG_BEGIN + 56,
        /// 용지 설정
        HWPTAG_PAGE_DEF = HWPTAG_BEGIN + 57,
        /// 각주/미주 모양
        HWPTAG_FOOTNOTE_SHAPE = HWPTAG_BEGIN + 58,
        /// 쪽 테두리/배경
        HWPTAG_PAGE_BORDER_FILL = HWPTAG_BEGIN + 59,
        /// 개체
        HWPTAG_SHAPE_COMPONENT = HWPTAG_BEGIN + 60,
        /// 표 개체
        HWPTAG_TABLE = HWPTAG_BEGIN + 61,
        /// 직선 개체
        HWPTAG_SHAPE_COMPONENT_LINE = HWPTAG_BEGIN + 62,
        /// 사각형 개체
        HWPTAG_SHAPE_COMPONENT_RECTANGLE = HWPTAG_BEGIN + 63,
        /// 타원 개체
        HWPTAG_SHAPE_COMPONENT_ELLIPSE = HWPTAG_BEGIN + 64,
        /// 호 개체
        HWPTAG_SHAPE_COMPONENT_ARC = HWPTAG_BEGIN + 65,
        /// 다각형 개체
        HWPTAG_SHAPE_COMPONENT_POLYGON = HWPTAG_BEGIN + 66,
        /// 곡선 개체
        HWPTAG_SHAPE_COMPONENT_CURVE = HWPTAG_BEGIN + 67,
        /// OLE 개체
        HWPTAG_SHAPE_COMPONENT_OLE = HWPTAG_BEGIN + 68,
        /// 그림 개체
        HWPTAG_SHAPE_COMPONENT_PICTURE = HWPTAG_BEGIN + 69,
        /// 컨테이너 개체
        HWPTAG_SHAPE_COMPONENT_CONTAINER = HWPTAG_BEGIN + 70,
        /// 컨트롤 임의의 데이터
        HWPTAG_CTRL_DATA = HWPTAG_BEGIN + 71,
        /// 수식 개체
        HWPTAG_EQEDIT = HWPTAG_BEGIN + 72,
        /// 글맵시
        HWPTAG_SHAPE_COMPONENT_TEXTART = HWPTAG_BEGIN + 74,
        /// 양식 개체
        HWPTAG_FORM_OBJECT = HWPTAG_BEGIN + 75,
        /// 메모 리스트 헤더
        HWPTAG_MEMO_LIST = HWPTAG_BEGIN + 77,
        /// 차트 데이터
        HWPTAG_CHART_DATA = HWPTAG_BEGIN + 79,
        /// 비디오 데이터
        HWPTAG_VIDEO_DATA = HWPTAG_BEGIN + 82,
        /// Unknown
        HWPTAG_SHAPE_COMPONENT_UNKNOWN = HWPTAG_BEGIN + 99,
    }

    enum BothTAG {
        /// 예약
        RESERVED = HWPTAG_BEGIN + 73,
        /// 메모 모양
        HWPTAG_MEMO_SHAPE = HWPTAG_BEGIN + 76,
    }
}
