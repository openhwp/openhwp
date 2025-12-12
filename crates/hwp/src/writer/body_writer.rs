//! BodyText 스트림 생성기
//!
//! BodyText 스트림은 섹션 단위로 문단들과 컨트롤들을 포함합니다.
//!
//! # 레코드 구조
//!
//! - PARAGRAPH_HEADER (0x042)
//! - PARAGRAPH_TEXT (0x043)
//! - PARAGRAPH_CHAR_SHAPE (0x044)
//! - PARAGRAPH_LINE_SEG (0x045)
//! - PARAGRAPH_RANGE_TAG (0x046)
//! - CTRL_HEADER (0x047)
//! - LIST_HEADER (0x048)
//! - PAGE_DEF (0x049)
//! - FOOTNOTE_SHAPE (0x04A)
//! - PAGE_BORDER_FILL (0x04B)
//! - ...

use super::ByteWriter;
use crate::doc_info::FillInfo;
use crate::primitive::RecordTagId;

/// BodyText (Section) 스트림 생성기
pub struct BodyWriter {
    /// 섹션 데이터들
    sections: Vec<SectionData>,
}

/// 섹션 데이터
#[derive(Debug, Clone, Default)]
pub struct SectionData {
    /// 페이지 정의
    pub page_definition: Option<PageDefinitionData>,
    /// 페이지 테두리/배경
    pub page_border_fill: Option<PageBorderFillData>,
    /// 각주 모양
    pub footnote_shape: Option<FootnoteShapeData>,
    /// 미주 모양
    pub endnote_shape: Option<EndnoteShapeData>,
    /// 구역 정의
    pub section_definition: Option<SectionDefinitionData>,
    /// 단 정의
    pub column_definition: Option<ColumnDefinitionData>,
    /// 문단들
    pub paragraphs: Vec<ParagraphData>,
}

/// 페이지 정의 데이터
#[derive(Debug, Clone)]
pub struct PageDefinitionData {
    /// 용지 너비 (HwpUnit)
    pub paper_width: u32,
    /// 용지 높이 (HwpUnit)
    pub paper_height: u32,
    /// 왼쪽 여백
    pub margin_left: u32,
    /// 오른쪽 여백
    pub margin_right: u32,
    /// 위쪽 여백
    pub margin_top: u32,
    /// 아래쪽 여백
    pub margin_bottom: u32,
    /// 머리말 여백
    pub margin_header: u32,
    /// 꼬리말 여백
    pub margin_footer: u32,
    /// 제본 여백
    pub margin_gutter: u32,
    /// 속성 (하위 1비트: 용지 방향, 2-3비트: 제본 여백 위치)
    pub properties: u32,
}

/// 페이지 테두리/배경 데이터
#[derive(Debug, Clone)]
pub struct PageBorderFillData {
    /// 테두리/채우기 ID
    pub border_fill_id: u16,
    /// 속성 (하위 1비트: 위치(0=용지, 1=본문), 2비트: 머리글 포함, 4비트: 바닥글 포함)
    pub properties: u32,
    /// 왼쪽 오프셋
    pub offset_left: i32,
    /// 오른쪽 오프셋
    pub offset_right: i32,
    /// 위쪽 오프셋
    pub offset_top: i32,
    /// 아래쪽 오프셋
    pub offset_bottom: i32,
}

/// 각주 모양 데이터
#[derive(Debug, Clone)]
pub struct FootnoteShapeData {
    /// 속성 (번호 형식, 위첨자 등)
    pub properties: u32,
    /// 접두 문자열
    pub prefix: String,
    /// 접미 문자열
    pub suffix: String,
    /// 시작 번호
    pub start_number: u16,
    /// 구분선 길이 (0-100%)
    pub separator_length: u16,
    /// 구분선 위치 (여백으로부터의 거리)
    pub separator_position: u16,
    /// 구분선 위쪽 여백
    pub space_above: u16,
    /// 구분선 아래쪽 여백
    pub space_below: u16,
    /// 각주 간 간격
    pub space_between: u16,
    /// 구분선 종류
    pub separator_line_type: u8,
    /// 구분선 두께 (0.1mm 단위)
    pub separator_line_thickness: u8,
    /// 구분선 색상 (RGB)
    pub separator_line_color: u32,
}

/// 미주 모양 데이터
#[derive(Debug, Clone)]
pub struct EndnoteShapeData {
    /// 속성
    pub properties: u32,
    /// 접두 문자열
    pub prefix: String,
    /// 접미 문자열
    pub suffix: String,
    /// 시작 번호
    pub start_number: u16,
    /// 구분선 위치 (여백으로부터의 거리)
    pub separator_position: u16,
}

/// 구역 정의 데이터 (HWP 표 129)
#[derive(Debug, Clone)]
pub struct SectionDefinitionData {
    /// 속성 (표 130)
    pub properties: u32,
    /// 단 간격
    pub column_gap: u16,
    /// 세로 격자선 간격
    pub vertical_grid: u16,
    /// 가로 격자선 간격
    pub horizontal_grid: u16,
    /// 기본 탭 간격
    pub default_tab_interval: u32,
    /// 번호 문단 모양 ID
    pub numbering_shape_id: u16,
    /// 시작 페이지 번호 (0이면 이전 섹션에서 이어감)
    pub starting_page_number: u16,
    /// 그림/표/수식 시작 번호
    pub starting_figure_number: u16,
    pub starting_table_number: u16,
    pub starting_equation_number: u16,
    /// 대표 언어
    pub language: u16,
}

impl Default for SectionDefinitionData {
    fn default() -> Self {
        Self {
            properties: 0,
            column_gap: 0,
            vertical_grid: 0,
            horizontal_grid: 0,
            default_tab_interval: 800, // 기본 탭 간격 (약 8mm)
            numbering_shape_id: 0,
            starting_page_number: 0,
            starting_figure_number: 1,
            starting_table_number: 1,
            starting_equation_number: 1,
            language: 0, // 기본값 0 (시스템 기본 언어)
        }
    }
}

/// 단 정의 데이터 (HWP 표 138)
#[derive(Debug, Clone)]
pub struct ColumnDefinitionData {
    /// 속성1 (표 139) - 단 종류, 단 수, 단 방향, 같은 너비
    pub properties1: u16,
    /// 단 간격
    pub column_gap: u16,
    /// 개별 단 너비 (같은 너비가 아닐 때만)
    pub column_widths: Vec<u16>,
    /// 속성2
    pub properties2: u16,
    /// 구분선 종류
    pub separator_style: u8,
    /// 구분선 두께
    pub separator_thickness: u8,
    /// 구분선 색상 (COLORREF)
    pub separator_color: u32,
}

impl Default for ColumnDefinitionData {
    fn default() -> Self {
        Self {
            properties1: 1 << 2 | 1 << 12, // 단 수 1, 같은 너비
            column_gap: 0,
            column_widths: Vec::new(),
            properties2: 0,
            separator_style: 0,
            separator_thickness: 0,
            separator_color: 0,
        }
    }
}

/// 문단 데이터
#[derive(Debug, Clone)]
pub struct ParagraphData {
    /// 문단 모양 ID
    pub para_shape_id: u16,
    /// 스타일 ID
    pub style_id: u8,
    /// 문단 텍스트 (UTF-16)
    pub text: Vec<u16>,
    /// 글자 모양 참조들
    pub char_shape_refs: Vec<CharShapeRef>,
    /// 범위 태그들 (변경 추적, 형광펜 등)
    pub range_tags: Vec<RangeTagData>,
    /// 컨트롤들
    pub controls: Vec<ControlData>,
}

/// 글자 모양 참조
#[derive(Debug, Clone, Copy)]
pub struct CharShapeRef {
    /// 위치
    pub position: u32,
    /// 글자 모양 ID
    pub char_shape_id: u32,
}

/// 범위 태그 데이터
#[derive(Debug, Clone, Copy)]
pub struct RangeTagData {
    /// 시작 위치
    pub start_position: u32,
    /// 끝 위치
    pub end_position: u32,
    /// 태그 (3바이트)
    pub tag: [u8; 3],
}

/// 컨트롤 데이터
#[derive(Debug, Clone)]
pub enum ControlData {
    /// 표
    Table(TableData),
    /// 그림
    Picture(PictureData),
    /// 텍스트 박스
    TextBox(TextBoxData),
    /// 수식
    Equation(EquationData),
    /// 도형
    Shape(ShapeData),
    /// 머리글
    Header(HeaderFooterData),
    /// 바닥글
    Footer(HeaderFooterData),
    /// 각주
    Footnote(NoteData),
    /// 미주
    Endnote(NoteData),
    /// 하이퍼링크
    Hyperlink(HyperlinkData),
    /// 책갈피
    Bookmark(BookmarkData),
    /// 자동 번호
    AutoNumber(AutoNumberData),
    /// 새 번호
    NewNumber(NewNumberData),
    /// 페이지 번호
    PageNumber(PageNumberData),
    /// 숨은 설명
    HiddenComment(HiddenCommentData),
    /// 비디오
    Video(VideoData),
    /// OLE 객체
    Ole(OleData),
    /// 차트
    Chart(ChartData),
    /// 양식 객체
    FormObject(FormObjectData),
    /// 글맵시
    TextArt(TextArtData),
    /// 필드 (날짜, 시간, 페이지 번호 등)
    Field(FieldData),
}

/// 개체 공통 속성 데이터
#[derive(Debug, Clone, Default)]
pub struct ObjectCommonData {
    /// 위치 속성 (TextWrap 등)
    pub properties: u32,
    /// 세로 오프셋 (HwpUnit)
    pub vertical_offset: i32,
    /// 가로 오프셋 (HwpUnit)
    pub horizontal_offset: i32,
    /// 너비 (HwpUnit)
    pub width: i32,
    /// 높이 (HwpUnit)
    pub height: i32,
    /// Z 순서
    pub z_order: i32,
    /// 왼쪽 여백
    pub margin_left: u16,
    /// 오른쪽 여백
    pub margin_right: u16,
    /// 위쪽 여백
    pub margin_top: u16,
    /// 아래쪽 여백
    pub margin_bottom: u16,
}

/// 캡션 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CaptionDirection {
    /// 아래
    #[default]
    Below,
    /// 위
    Above,
    /// 왼쪽
    Left,
    /// 오른쪽
    Right,
}

/// 캡션 데이터
#[derive(Debug, Clone)]
pub struct CaptionData {
    /// 캡션 방향
    pub direction: CaptionDirection,
    /// 캡션 간격
    pub gap: i32,
    /// 캡션 내용 (문단들)
    pub paragraphs: Vec<ParagraphData>,
}

/// 표 데이터
#[derive(Debug, Clone)]
pub struct TableData {
    /// 공통 속성
    pub common: ObjectCommonData,
    /// 행 수
    pub rows: u16,
    /// 열 수
    pub columns: u16,
    /// 열 너비들
    pub column_widths: Vec<u16>,
    /// 셀들
    pub cells: Vec<TableCellData>,
    /// 테두리/채우기 ID
    pub border_fill_id: u16,
    /// 캡션 (있는 경우)
    pub caption: Option<CaptionData>,
    /// 표 속성 (bit 0-1: 페이지 나눔, bit 2: 제목 줄 자동 반복)
    pub properties: u32,
}

/// 표 셀 데이터
#[derive(Debug, Clone)]
pub struct TableCellData {
    /// 열 주소
    pub col: u16,
    /// 행 주소
    pub row: u16,
    /// 열 병합 수
    pub col_span: u16,
    /// 행 병합 수
    pub row_span: u16,
    /// 너비
    pub width: u32,
    /// 높이
    pub height: u32,
    /// 셀 내 문단들
    pub paragraphs: Vec<ParagraphData>,
    /// 테두리/채우기 ID
    pub border_fill_id: u16,
}

/// 그림 데이터
#[derive(Debug, Clone)]
pub struct PictureData {
    /// 공통 속성
    pub common: ObjectCommonData,
    /// 바이너리 데이터 ID
    pub binary_data_id: u16,
    /// 너비
    pub width: u32,
    /// 높이
    pub height: u32,
    /// 캡션 (있는 경우)
    pub caption: Option<CaptionData>,
}

/// 텍스트 박스 데이터
#[derive(Debug, Clone)]
pub struct TextBoxData {
    /// 공통 속성
    pub common: ObjectCommonData,
    /// 너비
    pub width: u32,
    /// 높이
    pub height: u32,
    /// 내부 문단들
    pub paragraphs: Vec<ParagraphData>,
}

/// 수식 데이터
#[derive(Debug, Clone)]
pub struct EquationData {
    /// 공통 속성
    pub common: ObjectCommonData,
    /// 수식 스크립트
    pub script: String,
    /// 크기
    pub base_size: u32,
    /// 캡션 (있는 경우)
    pub caption: Option<CaptionData>,
}

/// 머리글/바닥글 데이터
#[derive(Debug, Clone)]
pub struct HeaderFooterData {
    /// 적용 대상 (0: 양쪽, 1: 짝수, 2: 홀수)
    pub apply_to: u8,
    /// 내용 문단들
    pub paragraphs: Vec<ParagraphData>,
}

/// 각주/미주 데이터
#[derive(Debug, Clone)]
pub struct NoteData {
    /// 번호
    pub number: u16,
    /// 내용 문단들
    pub paragraphs: Vec<ParagraphData>,
}

/// 하이퍼링크 데이터
#[derive(Debug, Clone)]
pub struct HyperlinkData {
    /// URL
    pub url: String,
    /// 툴팁
    pub tooltip: Option<String>,
}

/// 책갈피 데이터
#[derive(Debug, Clone)]
pub struct BookmarkData {
    /// 이름
    pub name: String,
}

/// 자동 번호 데이터
#[derive(Debug, Clone)]
pub struct AutoNumberData {
    /// 번호 타입 (0: 페이지, 1: 각주, 2: 미주, 3: 그림, 4: 표, 5: 수식)
    pub number_type: u16,
    /// 번호 형식 (0: 숫자, 1: 원숫자, 2: 로마 대문자, 등)
    pub number_format: u16,
}

/// 새 번호 데이터
#[derive(Debug, Clone)]
pub struct NewNumberData {
    /// 번호 타입
    pub number_type: u16,
    /// 새 번호 값
    pub number: u16,
}

/// 페이지 번호 데이터
#[derive(Debug, Clone)]
pub struct PageNumberData {
    /// 위치 (0: 없음, 1: 위왼쪽, 2: 위가운데, ... 10: 안쪽 아래)
    pub position: u16,
    /// 번호 형식
    pub number_format: u16,
    /// 줄표 문자
    pub side_character: String,
}

/// 숨은 설명 데이터
#[derive(Debug, Clone)]
pub struct HiddenCommentData {
    /// 내용 문단들
    pub paragraphs: Vec<ParagraphData>,
}

/// 비디오 데이터
#[derive(Debug, Clone)]
pub struct VideoData {
    /// 공통 속성
    pub common: ObjectCommonData,
    /// 비디오 타입 (0: 임베디드, 1: 링크)
    pub video_type: u8,
    /// 바이너리 데이터 ID (임베디드용)
    pub bin_data_id: u16,
    /// 포스터 바이너리 데이터 ID
    pub poster_bin_id: u16,
    /// 소스 URL (링크용)
    pub source_url: String,
    /// 비디오 너비 (HWP 단위)
    pub width: u32,
    /// 비디오 높이 (HWP 단위)
    pub height: u32,
}

/// OLE 데이터
#[derive(Debug, Clone)]
pub struct OleData {
    /// 공통 속성
    pub common: ObjectCommonData,
    /// 속성 플래그
    pub properties: u32,
    /// 너비
    pub extent_width: i32,
    /// 높이
    pub extent_height: i32,
    /// 바이너리 데이터 ID
    pub bin_data_id: u16,
    /// 테두리 색상
    pub border_color: u32,
    /// 테두리 두께
    pub border_thickness: i32,
}

/// 차트 데이터
#[derive(Debug, Clone)]
pub struct ChartData {
    /// 공통 속성
    pub common: ObjectCommonData,
    /// 차트 타입
    pub chart_type: u8,
}

/// 양식 객체 데이터
#[derive(Debug, Clone, Default)]
pub struct FormObjectData {
    /// 공통 속성
    pub common: ObjectCommonData,
    /// 양식 타입
    pub form_type: u8,
    /// 이름
    pub name: String,
    /// 값
    pub value: String,
    /// 속성 플래그
    pub properties: u32,
    /// 버튼 캡션
    pub caption: Option<String>,
    /// 라디오 그룹 이름
    pub radio_group_name: Option<String>,
    /// 배경 색상 (BGR format)
    pub back_color: Option<u32>,
    /// 비밀번호 문자 (UTF-16 코드)
    pub password_char: Option<u16>,
    /// 최대 길이
    pub max_length: Option<u32>,
    /// 선택된 값
    pub selected_value: Option<String>,
    /// 목록 박스 행 수
    pub list_box_rows: Option<i32>,
    /// 목록 박스 폭
    pub list_box_width: Option<i32>,
    /// 항목 높이
    pub item_height: Option<i32>,
    /// 최상단 인덱스
    pub top_index: Option<u32>,
    /// 목록 항목 (표시 텍스트)
    pub items_text: Vec<String>,
    /// 목록 항목 (값)
    pub items_value: Vec<String>,
    /// 최소값 (ScrollBar)
    pub min: Option<i32>,
    /// 최대값 (ScrollBar)
    pub max: Option<i32>,
    /// 현재 값 (ScrollBar)
    pub scroll_value: Option<i32>,
    /// 작은 증감값
    pub small_change: Option<u32>,
    /// 큰 증감값
    pub large_change: Option<u32>,
    /// 페이지 단위
    pub page: Option<i32>,
    /// 반복 지연
    pub delay: Option<u32>,
}

/// 글맵시 데이터
#[derive(Debug, Clone)]
pub struct TextArtData {
    /// 공통 속성
    pub common: ObjectCommonData,
    /// 텍스트 내용
    pub text: String,
    /// 글꼴 이름
    pub font_name: Option<String>,
    /// 글꼴 스타일 (0: Regular, 1: Bold, 2: Italic, 3: BoldItalic)
    pub font_style: u8,
    /// 모양 (0: 사각형, 1: 원형, 2: 아치 위, 3: 아치 아래, 등)
    pub shape_type: u8,
    /// 줄 간격
    pub line_spacing: u32,
    /// 자간
    pub char_spacing: u32,
    /// 정렬 (0: 왼쪽, 1: 가운데, 2: 오른쪽, 3: 양쪽)
    pub alignment: u8,
    /// 너비
    pub width: u32,
    /// 높이
    pub height: u32,
}

/// 필드 데이터
#[derive(Debug, Clone)]
pub struct FieldData {
    /// 필드 타입
    pub field_type: FieldTypeData,
    /// 명령/형식 문자열
    pub instruction: String,
}

/// 필드 타입 데이터
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldTypeData {
    /// 알 수 없음
    Unknown,
    /// 날짜
    Date,
    /// 시간
    Time,
    /// 파일 경로
    FilePath,
    /// 문서 제목
    DocTitle,
    /// 저자
    Author,
    /// 페이지 번호
    PageNumber,
    /// 총 페이지 수
    TotalPages,
    /// 요약
    Summary,
    /// 상호 참조
    CrossReference,
    /// 하이퍼링크
    Hyperlink,
    /// 메모
    Memo,
    /// 수식
    Formula,
    /// 클릭 필드
    ClickHere,
    /// 사용자 정보
    UserInfo,
    /// 개인정보
    PrivateInfo,
    /// 메타태그
    MetaTag,
    /// 메일 머지
    MailMerge,
    /// 목차
    TableOfContents,
}

impl FieldTypeData {
    /// HWP 컨트롤 ID로 변환
    pub fn to_control_id(&self) -> &'static [u8; 4] {
        match self {
            FieldTypeData::Date => b"%dat",
            FieldTypeData::Time => b"%tim",
            FieldTypeData::FilePath => b"%fil",
            FieldTypeData::DocTitle => b"%tit",
            FieldTypeData::Author => b"%aut",
            FieldTypeData::PageNumber => b"%pn ",
            FieldTypeData::TotalPages => b"%pn ", // 총 페이지는 %pn과 같은 컨트롤
            FieldTypeData::Summary => b"%smr",
            FieldTypeData::CrossReference => b"%xrf",
            FieldTypeData::Hyperlink => b"%hlk",
            FieldTypeData::Memo => b"%mem",
            FieldTypeData::Formula => b"%frm",
            FieldTypeData::ClickHere => b"%clk",
            FieldTypeData::UserInfo => b"%usr",
            FieldTypeData::PrivateInfo => b"%prv",
            FieldTypeData::MetaTag => b"%mtg",
            FieldTypeData::MailMerge => b"%mmr",
            FieldTypeData::TableOfContents => b"%toc",
            FieldTypeData::Unknown => b"%unk",
        }
    }
}

/// 도형 데이터
#[derive(Debug, Clone)]
pub struct ShapeData {
    /// 공통 속성
    pub common: ObjectCommonData,
    /// 도형 타입
    pub shape_type: ShapeTypeData,
    /// 너비
    pub width: u32,
    /// 높이
    pub height: u32,
    /// X 오프셋
    pub offset_x: i32,
    /// Y 오프셋
    pub offset_y: i32,
    /// 선 색상
    pub line_color: u32,
    /// 선 두께
    pub line_thickness: u16,
    /// 채우기 정보 (그라데이션, 패턴, 이미지 포함)
    pub fill: FillInfo,
    /// 캡션 (있는 경우)
    pub caption: Option<CaptionData>,
    /// 변환 행렬 (translation matrix)
    pub translation_matrix: Option<[f64; 6]>,
    /// 크기 조정 행렬 (scale matrix)
    pub scale_matrix: Option<[f64; 6]>,
    /// 회전 행렬 (rotation matrix)
    pub rotation_matrix: Option<[f64; 6]>,
    /// 회전 각도 (도)
    pub rotation: f64,
}

/// 도형 타입 데이터
#[derive(Debug, Clone)]
pub enum ShapeTypeData {
    /// 선
    Line(LineShapeData),
    /// 사각형
    Rectangle(RectangleShapeData),
    /// 타원
    Ellipse(EllipseShapeData),
    /// 호
    Arc(ArcShapeData),
    /// 다각형
    Polygon(PolygonShapeData),
    /// 곡선
    Curve(CurveShapeData),
    /// 그룹 (컨테이너)
    Container(ContainerShapeData),
    /// 연결선
    Connector(ConnectorShapeData),
}

/// 연결선 도형 데이터
#[derive(Debug, Clone)]
pub struct ConnectorShapeData {
    /// 연결선 스타일 (0~8)
    pub line_type: u8,
    /// 시작점 X
    pub start_x: i32,
    /// 시작점 Y
    pub start_y: i32,
    /// 시작점 연결 개체 ID
    pub start_subject_id: Option<u32>,
    /// 시작점 연결 인덱스
    pub start_subject_index: Option<u32>,
    /// 끝점 X
    pub end_x: i32,
    /// 끝점 Y
    pub end_y: i32,
    /// 끝점 연결 개체 ID
    pub end_subject_id: Option<u32>,
    /// 끝점 연결 인덱스
    pub end_subject_index: Option<u32>,
    /// 제어점 목록
    pub control_points: Vec<(i32, i32)>,
    /// 시작 화살표 타입
    pub start_arrow_type: u8,
    /// 시작 화살표 크기
    pub start_arrow_size: u8,
    /// 끝 화살표 타입
    pub end_arrow_type: u8,
    /// 끝 화살표 크기
    pub end_arrow_size: u8,
}

/// 그룹 도형 데이터 (컨테이너)
#[derive(Debug, Clone)]
pub struct ContainerShapeData {
    /// 자식 도형들
    pub children: Vec<ShapeData>,
}

/// 선 도형 데이터
#[derive(Debug, Clone)]
pub struct LineShapeData {
    /// 시작점 X
    pub start_x: i32,
    /// 시작점 Y
    pub start_y: i32,
    /// 끝점 X
    pub end_x: i32,
    /// 끝점 Y
    pub end_y: i32,
    /// 시작 화살표 타입
    pub start_arrow_type: u8,
    /// 시작 화살표 크기
    pub start_arrow_size: u8,
    /// 끝 화살표 타입
    pub end_arrow_type: u8,
    /// 끝 화살표 크기
    pub end_arrow_size: u8,
}

/// 사각형 도형 데이터
#[derive(Debug, Clone)]
pub struct RectangleShapeData {
    /// 모서리 반지름
    pub corner_radius: u32,
}

/// 타원 도형 데이터
#[derive(Debug, Clone)]
pub struct EllipseShapeData {
    /// 호 타입 (0: 전체, 1: 호, 2: 부채꼴, 3: 활꼴)
    pub arc_type: u8,
    /// 시작 각도 (degree * 100)
    pub start_angle: i32,
    /// 끝 각도 (degree * 100)
    pub end_angle: i32,
}

/// 호 도형 데이터
#[derive(Debug, Clone)]
pub struct ArcShapeData {
    /// 호 타입
    pub arc_type: u8,
    /// 시작 각도
    pub start_angle: i32,
    /// 끝 각도
    pub end_angle: i32,
}

/// 다각형 도형 데이터
#[derive(Debug, Clone)]
pub struct PolygonShapeData {
    /// 꼭짓점 좌표들 (x, y 쌍)
    pub points: Vec<(i32, i32)>,
}

/// 곡선 도형 데이터
#[derive(Debug, Clone)]
pub struct CurveShapeData {
    /// 제어점 좌표들 (x, y, type 튜플)
    pub points: Vec<(i32, i32, u8)>,
    /// 닫힌 곡선 여부
    pub closed: bool,
}

impl BodyWriter {
    /// 새 BodyWriter를 생성합니다.
    pub const fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }

    /// 섹션을 추가합니다.
    pub fn add_section(&mut self, section: SectionData) {
        self.sections.push(section);
    }

    /// 모든 섹션을 빌드합니다. 각 섹션은 별도의 Vec<u8>로 반환됩니다.
    pub fn build(&self) -> Vec<Vec<u8>> {
        self.sections
            .iter()
            .map(|section| self.build_section(section))
            .collect()
    }

    fn build_section(&self, section: &SectionData) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        // 페이지 정의 (있으면)
        if let Some(ref page_def) = section.page_definition {
            self.write_page_definition(&mut writer, page_def);
        }

        // 각주 모양 (있으면)
        if let Some(ref footnote_shape) = section.footnote_shape {
            self.write_footnote_shape(&mut writer, footnote_shape);
        }

        // 미주 모양 (있으면) - 같은 태그 사용, 다른 속성
        if let Some(ref endnote_shape) = section.endnote_shape {
            self.write_endnote_shape(&mut writer, endnote_shape);
        }

        // 페이지 테두리/배경 (있으면)
        if let Some(ref page_border_fill) = section.page_border_fill {
            self.write_page_border_fill(&mut writer, page_border_fill);
        }

        // 문단들
        for (i, para) in section.paragraphs.iter().enumerate() {
            self.write_paragraph(&mut writer, para, i as u32);
        }

        writer.into_bytes()
    }

    fn write_page_definition(&self, writer: &mut ByteWriter, page_def: &PageDefinitionData) {
        let mut data = ByteWriter::new();

        data.write_u32(page_def.paper_width);
        data.write_u32(page_def.paper_height);
        data.write_u32(page_def.margin_left);
        data.write_u32(page_def.margin_right);
        data.write_u32(page_def.margin_top);
        data.write_u32(page_def.margin_bottom);
        data.write_u32(page_def.margin_header);
        data.write_u32(page_def.margin_footer);
        data.write_u32(page_def.margin_gutter);
        data.write_u32(page_def.properties);

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::PageDefinition, 0, &data_bytes);
    }

    fn write_footnote_shape(&self, writer: &mut ByteWriter, footnote: &FootnoteShapeData) {
        let mut data = ByteWriter::new();

        data.write_u32(footnote.properties);
        data.write_hwp_string(&footnote.prefix);
        data.write_hwp_string(&footnote.suffix);
        data.write_u16(footnote.start_number);
        data.write_u16(footnote.separator_length);
        data.write_u16(footnote.separator_position);
        data.write_u16(footnote.space_above);
        data.write_u16(footnote.space_below);
        data.write_u16(footnote.space_between);
        data.write_u8(footnote.separator_line_type);
        data.write_u8(footnote.separator_line_thickness);
        data.write_u32(footnote.separator_line_color);

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::FootnoteShape, 0, &data_bytes);
    }

    fn write_endnote_shape(&self, writer: &mut ByteWriter, endnote: &EndnoteShapeData) {
        let mut data = ByteWriter::new();

        data.write_u32(endnote.properties);
        data.write_hwp_string(&endnote.prefix);
        data.write_hwp_string(&endnote.suffix);
        data.write_u16(endnote.start_number);
        // 미주는 구분선 관련 필드가 각주와 다름 - 위치만 있음
        data.write_u16(endnote.separator_position);

        let data_bytes = data.into_bytes();
        // 미주 모양도 FootnoteShape 태그 사용 (0x04A) - 같은 포맷
        writer.write_record(RecordTagId::FootnoteShape, 0, &data_bytes);
    }

    fn write_page_border_fill(&self, writer: &mut ByteWriter, border_fill: &PageBorderFillData) {
        let mut data = ByteWriter::new();

        data.write_u32(border_fill.properties);
        data.write_u16(border_fill.border_fill_id);
        data.write_i32(border_fill.offset_left);
        data.write_i32(border_fill.offset_right);
        data.write_i32(border_fill.offset_top);
        data.write_i32(border_fill.offset_bottom);

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::PageBorderFill, 0, &data_bytes);
    }

    fn write_paragraph(&self, writer: &mut ByteWriter, para: &ParagraphData, instance_id: u32) {
        // 1. PARAGRAPH_HEADER
        self.write_paragraph_header(writer, para, instance_id);

        // 2. PARAGRAPH_TEXT (있으면)
        if !para.text.is_empty() {
            self.write_paragraph_text(writer, para);
        }

        // 3. PARAGRAPH_CHAR_SHAPE (있으면)
        if !para.char_shape_refs.is_empty() {
            self.write_paragraph_char_shape(writer, para);
        }

        // 4. 컨트롤들
        for ctrl in &para.controls {
            self.write_control(writer, ctrl);
        }
    }

    fn write_paragraph_header(
        &self,
        writer: &mut ByteWriter,
        para: &ParagraphData,
        instance_id: u32,
    ) {
        let mut data = ByteWriter::new();

        // 텍스트 길이 계산
        let char_count = self.calculate_char_count(para);

        data.write_u32(char_count); // character_count
        data.write_u32(0); // control_mask
        data.write_u16(para.para_shape_id); // paragraph_shape_id
        data.write_u8(para.style_id); // style_id
        data.write_u8(0); // break_type
        data.write_u16(para.char_shape_refs.len() as u16); // char_shape_count
        data.write_u16(0); // range_tag_count
        data.write_u16(0); // line_seg_count
        data.write_u32(instance_id); // instance_id

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::ParagraphHeader, 0, &data_bytes);
    }

    fn write_paragraph_text(&self, writer: &mut ByteWriter, para: &ParagraphData) {
        let mut data = ByteWriter::new();

        // 텍스트 (UTF-16LE)
        for ch in &para.text {
            data.write_u16(*ch);
        }

        // 문단 끝 표시 (char code 13)
        data.write_u16(13);

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::ParagraphText, 1, &data_bytes);
    }

    fn write_paragraph_char_shape(&self, writer: &mut ByteWriter, para: &ParagraphData) {
        let mut data = ByteWriter::new();

        for char_ref in &para.char_shape_refs {
            data.write_u32(char_ref.position);
            data.write_u32(char_ref.char_shape_id);
        }

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::ParagraphCharacterShape, 1, &data_bytes);
    }

    fn write_control(&self, writer: &mut ByteWriter, ctrl: &ControlData) {
        match ctrl {
            ControlData::Table(table) => self.write_table(writer, table),
            ControlData::Picture(picture) => self.write_picture(writer, picture),
            ControlData::TextBox(textbox) => self.write_textbox(writer, textbox),
            ControlData::Equation(equation) => self.write_equation(writer, equation),
            ControlData::Shape(shape) => self.write_shape(writer, shape),
            ControlData::Header(header) => self.write_header_footer(writer, header, true),
            ControlData::Footer(footer) => self.write_header_footer(writer, footer, false),
            ControlData::Footnote(note) => self.write_note(writer, note, true),
            ControlData::Endnote(note) => self.write_note(writer, note, false),
            ControlData::Hyperlink(link) => self.write_hyperlink(writer, link),
            ControlData::Bookmark(bookmark) => self.write_bookmark(writer, bookmark),
            ControlData::AutoNumber(auto_num) => self.write_auto_number(writer, auto_num),
            ControlData::NewNumber(new_num) => self.write_new_number(writer, new_num),
            ControlData::PageNumber(page_num) => self.write_page_number(writer, page_num),
            ControlData::HiddenComment(comment) => self.write_hidden_comment(writer, comment),
            ControlData::Video(video) => self.write_video(writer, video),
            ControlData::Ole(ole) => self.write_ole(writer, ole),
            ControlData::Chart(chart) => self.write_chart(writer, chart),
            ControlData::FormObject(form) => self.write_form_object(writer, form),
            ControlData::TextArt(text_art) => self.write_text_art(writer, text_art),
            ControlData::Field(field) => self.write_field(writer, field),
        }
    }

    /// 공통 컨트롤 헤더 작성 헬퍼
    fn write_ctrl_header(
        &self,
        writer: &mut ByteWriter,
        ctrl_id: &[u8; 4],
        common: &ObjectCommonData,
    ) {
        let mut ctrl_header = ByteWriter::new();
        // Control ID
        ctrl_header.write_bytes(ctrl_id);
        // 공통 속성
        ctrl_header.write_u32(common.properties);
        ctrl_header.write_i32(common.vertical_offset);
        ctrl_header.write_i32(common.horizontal_offset);
        ctrl_header.write_i32(common.width);
        ctrl_header.write_i32(common.height);
        ctrl_header.write_i32(common.z_order);
        ctrl_header.write_u16(common.margin_left);
        ctrl_header.write_u16(common.margin_right);
        ctrl_header.write_u16(common.margin_top);
        ctrl_header.write_u16(common.margin_bottom);
        ctrl_header.write_u32(0); // instance_id
        ctrl_header.write_u32(0); // prevent_page_break
        ctrl_header.write_hwp_string(""); // description

        let ctrl_header_bytes = ctrl_header.into_bytes();
        writer.write_record(RecordTagId::ControlHeader, 1, &ctrl_header_bytes);
    }

    fn write_table(&self, writer: &mut ByteWriter, table: &TableData) {
        // CTRL_HEADER (tbl )
        self.write_ctrl_header(writer, b" tbl", &table.common);

        // TABLE record
        let mut table_data = ByteWriter::new();
        table_data.write_u32(table.properties); // properties (bit 0-1: page break, bit 2: auto repeat title row)
        table_data.write_u16(table.rows);
        table_data.write_u16(table.columns);
        table_data.write_u16(0); // cell_spacing
        table_data.write_u16(0); // margin_left
        table_data.write_u16(0); // margin_right

        // 열 너비들
        for width in &table.column_widths {
            table_data.write_u16(*width);
        }

        table_data.write_u16(table.border_fill_id);
        table_data.write_u16(0); // zone_info_count

        let table_bytes = table_data.into_bytes();
        writer.write_record(RecordTagId::Table, 2, &table_bytes);

        // 셀들
        for cell in &table.cells {
            self.write_table_cell(writer, cell);
        }
    }

    fn write_table_cell(&self, writer: &mut ByteWriter, cell: &TableCellData) {
        // LIST_HEADER + Cell properties
        let mut list_header = ByteWriter::new();
        list_header.write_u16(cell.paragraphs.len() as u16); // paragraph_count
        list_header.write_u32(0); // properties
        list_header.write_u16(cell.col); // col
        list_header.write_u16(cell.row); // row
        list_header.write_u16(cell.col_span); // col_span
        list_header.write_u16(cell.row_span); // row_span
        list_header.write_u32(cell.width); // width
        list_header.write_u32(cell.height); // height
        list_header.write_u16(0); // margin_left
        list_header.write_u16(0); // margin_right
        list_header.write_u16(0); // margin_top
        list_header.write_u16(0); // margin_bottom
        list_header.write_u16(cell.border_fill_id); // border_fill_id
        list_header.write_u32(0); // unknown

        let list_header_bytes = list_header.into_bytes();
        writer.write_record(RecordTagId::ListHeader, 2, &list_header_bytes);

        // 셀 내 문단들
        for (i, para) in cell.paragraphs.iter().enumerate() {
            self.write_paragraph(writer, para, (i + 1000) as u32);
        }
    }

    fn write_picture(&self, writer: &mut ByteWriter, picture: &PictureData) {
        // CTRL_HEADER (gso )
        self.write_ctrl_header(writer, b" gso", &picture.common);

        // SHAPE_COMPONENT_PICTURE
        let mut pic_data = ByteWriter::new();
        pic_data.write_u32(0); // border_color
        pic_data.write_u32(0); // border_thickness
        pic_data.write_u32(0); // border_properties
        pic_data.write_i32(0); // crop_left
        pic_data.write_i32(0); // crop_top
        pic_data.write_i32(0); // crop_right
        pic_data.write_i32(0); // crop_bottom
        pic_data.write_u16(0); // inner_margin_left
        pic_data.write_u16(0); // inner_margin_right
        pic_data.write_u16(0); // inner_margin_top
        pic_data.write_u16(0); // inner_margin_bottom
        pic_data.write_u8(100); // brightness
        pic_data.write_u8(100); // contrast
        pic_data.write_u8(0); // effect
        pic_data.write_u16(picture.binary_data_id);
        pic_data.write_u8(0); // border_transparency
        pic_data.write_u32(0); // instance_id
        pic_data.write_u8(0); // picture_effect_info count
        pic_data.write_u16(0); // image_width
        pic_data.write_u16(0); // image_height

        let pic_bytes = pic_data.into_bytes();
        writer.write_record(RecordTagId::ShapeComponentPicture, 2, &pic_bytes);
    }

    fn write_textbox(&self, writer: &mut ByteWriter, textbox: &TextBoxData) {
        // CTRL_HEADER (gso )
        self.write_ctrl_header(writer, b" gso", &textbox.common);

        // LIST_HEADER
        let mut list_header = ByteWriter::new();
        list_header.write_u16(textbox.paragraphs.len() as u16);
        list_header.write_u32(0);

        let list_header_bytes = list_header.into_bytes();
        writer.write_record(RecordTagId::ListHeader, 2, &list_header_bytes);

        // 내부 문단들
        for (i, para) in textbox.paragraphs.iter().enumerate() {
            self.write_paragraph(writer, para, (i + 2000) as u32);
        }
    }

    fn write_equation(&self, writer: &mut ByteWriter, equation: &EquationData) {
        // CTRL_HEADER (eqed)
        self.write_ctrl_header(writer, b"deqe", &equation.common);

        // EQUATION record
        let mut eq_data = ByteWriter::new();
        eq_data.write_u32(0); // properties
        eq_data.write_hwp_string(&equation.script);
        eq_data.write_u32(equation.base_size);
        eq_data.write_u32(0); // text_color
        eq_data.write_i16(100); // base_line
        eq_data.write_hwp_string(""); // version_info
        eq_data.write_hwp_string(""); // font_name

        let eq_bytes = eq_data.into_bytes();
        writer.write_record(RecordTagId::Equation, 2, &eq_bytes);
    }

    fn write_header_footer(
        &self,
        writer: &mut ByteWriter,
        data: &HeaderFooterData,
        is_header: bool,
    ) {
        // CTRL_HEADER (daeh for header, toof for footer)
        let mut ctrl_header = ByteWriter::new();
        if is_header {
            ctrl_header.write_bytes(b"head"); // "daeh" in LE
        } else {
            ctrl_header.write_bytes(b"foot"); // "toof" in LE
        }
        ctrl_header.write_u32(data.apply_to as u32); // apply_to

        let ctrl_header_bytes = ctrl_header.into_bytes();
        writer.write_record(RecordTagId::ControlHeader, 1, &ctrl_header_bytes);

        // LIST_HEADER
        let mut list_header = ByteWriter::new();
        list_header.write_u16(data.paragraphs.len() as u16); // paragraph_count
        list_header.write_u32(0); // properties

        let list_header_bytes = list_header.into_bytes();
        writer.write_record(RecordTagId::ListHeader, 2, &list_header_bytes);

        // 내부 문단들
        for (i, para) in data.paragraphs.iter().enumerate() {
            self.write_paragraph(writer, para, (i + 3000) as u32);
        }
    }

    fn write_note(&self, writer: &mut ByteWriter, data: &NoteData, is_footnote: bool) {
        // CTRL_HEADER (nf for footnote, ne for endnote)
        let mut ctrl_header = ByteWriter::new();
        if is_footnote {
            ctrl_header.write_bytes(b"  nf"); // "fn  " in LE
        } else {
            ctrl_header.write_bytes(b"  ne"); // "en  " in LE
        }
        ctrl_header.write_u16(data.number);
        ctrl_header.write_u16(0); // before_decoration
        ctrl_header.write_u16(0); // after_decoration
        ctrl_header.write_u32(0); // instance_id

        let ctrl_header_bytes = ctrl_header.into_bytes();
        writer.write_record(RecordTagId::ControlHeader, 1, &ctrl_header_bytes);

        // LIST_HEADER
        let mut list_header = ByteWriter::new();
        list_header.write_u16(data.paragraphs.len() as u16);
        list_header.write_u32(0);

        let list_header_bytes = list_header.into_bytes();
        writer.write_record(RecordTagId::ListHeader, 2, &list_header_bytes);

        // 내부 문단들
        for (i, para) in data.paragraphs.iter().enumerate() {
            self.write_paragraph(writer, para, (i + 4000) as u32);
        }
    }

    fn write_hyperlink(&self, writer: &mut ByteWriter, data: &HyperlinkData) {
        // CTRL_HEADER (klh )
        let mut ctrl_header = ByteWriter::new();
        ctrl_header.write_bytes(b" klh"); // "hlk " in LE
        ctrl_header.write_hwp_string(&data.url);
        if let Some(ref tooltip) = data.tooltip {
            ctrl_header.write_hwp_string(tooltip);
        } else {
            ctrl_header.write_hwp_string("");
        }

        let ctrl_header_bytes = ctrl_header.into_bytes();
        writer.write_record(RecordTagId::ControlHeader, 1, &ctrl_header_bytes);
    }

    fn write_bookmark(&self, writer: &mut ByteWriter, data: &BookmarkData) {
        // CTRL_HEADER (kobm)
        let mut ctrl_header = ByteWriter::new();
        ctrl_header.write_bytes(b"mbok"); // "bokm" in LE
        ctrl_header.write_u32(0); // properties
        ctrl_header.write_hwp_string(&data.name);

        let ctrl_header_bytes = ctrl_header.into_bytes();
        writer.write_record(RecordTagId::ControlHeader, 1, &ctrl_header_bytes);
    }

    fn write_auto_number(&self, writer: &mut ByteWriter, data: &AutoNumberData) {
        // CTRL_HEADER (onta)
        let mut ctrl_header = ByteWriter::new();
        ctrl_header.write_bytes(b"atno"); // "atno" in LE
        ctrl_header.write_u16(data.number_type);
        ctrl_header.write_u16(data.number_format);
        ctrl_header.write_u16(0); // superscript

        let ctrl_header_bytes = ctrl_header.into_bytes();
        writer.write_record(RecordTagId::ControlHeader, 1, &ctrl_header_bytes);
    }

    fn write_new_number(&self, writer: &mut ByteWriter, data: &NewNumberData) {
        // CTRL_HEADER (nwno)
        let mut ctrl_header = ByteWriter::new();
        ctrl_header.write_bytes(b"nwno"); // "nwno" in LE
        ctrl_header.write_u16(data.number_type);
        ctrl_header.write_u16(data.number);

        let ctrl_header_bytes = ctrl_header.into_bytes();
        writer.write_record(RecordTagId::ControlHeader, 1, &ctrl_header_bytes);
    }

    fn write_page_number(&self, writer: &mut ByteWriter, data: &PageNumberData) {
        // CTRL_HEADER (pgnp)
        let mut ctrl_header = ByteWriter::new();
        ctrl_header.write_bytes(b"pgnp"); // "pgnp" in LE

        // 속성: bit 0~3: 위치, bit 4~11: 번호 모양
        let properties = (data.position as u32) | ((data.number_format as u32) << 4);
        ctrl_header.write_u32(properties);

        // 줄표 문자 (UTF-16)
        for ch in data.side_character.encode_utf16() {
            ctrl_header.write_u16(ch);
        }
        ctrl_header.write_u16(0); // NULL terminator

        let ctrl_header_bytes = ctrl_header.into_bytes();
        writer.write_record(RecordTagId::ControlHeader, 1, &ctrl_header_bytes);
    }

    fn write_hidden_comment(&self, writer: &mut ByteWriter, data: &HiddenCommentData) {
        // CTRL_HEADER (tcmo)
        let mut ctrl_header = ByteWriter::new();
        ctrl_header.write_bytes(b"omct"); // "tcmo" in LE

        let ctrl_header_bytes = ctrl_header.into_bytes();
        writer.write_record(RecordTagId::ControlHeader, 1, &ctrl_header_bytes);

        // LIST_HEADER
        let mut list_header = ByteWriter::new();
        list_header.write_u16(data.paragraphs.len() as u16);
        list_header.write_u32(0);

        let list_header_bytes = list_header.into_bytes();
        writer.write_record(RecordTagId::ListHeader, 2, &list_header_bytes);

        // 내부 문단들
        for (i, para) in data.paragraphs.iter().enumerate() {
            self.write_paragraph(writer, para, (i + 5000) as u32);
        }
    }

    fn write_video(&self, writer: &mut ByteWriter, video: &VideoData) {
        // CTRL_HEADER (vid )
        self.write_ctrl_header(writer, b" vid", &video.common);

        // VIDEO_DATA
        let mut video_data = ByteWriter::new();
        video_data.write_i32(video.video_type as i32);

        if video.video_type == 0 {
            // 임베디드 비디오
            video_data.write_u16(video.bin_data_id);
            video_data.write_u16(video.poster_bin_id);
        } else {
            // 링크 비디오 - URL 문자열 작성
            video_data.write_hwp_string(&video.source_url);
            video_data.write_u16(video.poster_bin_id);
        }

        let video_data_bytes = video_data.into_bytes();
        writer.write_record(RecordTagId::VideoData, 1, &video_data_bytes);
    }

    fn write_ole(&self, writer: &mut ByteWriter, ole: &OleData) {
        // CTRL_HEADER (ole )
        self.write_ctrl_header(writer, b" ole", &ole.common);

        // OLE_OBJECT
        let mut ole_data = ByteWriter::new();
        ole_data.write_u32(ole.properties);
        ole_data.write_i32(ole.extent_width);
        ole_data.write_i32(ole.extent_height);
        ole_data.write_u16(ole.bin_data_id);
        ole_data.write_u32(ole.border_color);
        ole_data.write_i32(ole.border_thickness);
        ole_data.write_u32(0); // border_properties

        let ole_data_bytes = ole_data.into_bytes();
        writer.write_record(RecordTagId::ShapeComponentOle, 1, &ole_data_bytes);
    }

    fn write_chart(&self, writer: &mut ByteWriter, chart: &ChartData) {
        // CTRL_HEADER (cht )
        self.write_ctrl_header(writer, b" cht", &chart.common);

        // CHART_DATA (2바이트만)
        let mut chart_data = ByteWriter::new();
        chart_data.write_u16(chart.chart_type as u16);

        let chart_data_bytes = chart_data.into_bytes();
        writer.write_record(RecordTagId::ChartData, 1, &chart_data_bytes);
    }

    fn write_form_object(&self, writer: &mut ByteWriter, form: &FormObjectData) {
        // CTRL_HEADER (form)
        self.write_ctrl_header(writer, b"mrof", &form.common);

        // FORM_OBJECT
        let mut form_data = ByteWriter::new();
        form_data.write_u32(form.form_type as u32); // properties (하위 바이트가 form type)
        form_data.write_hwp_string(&form.name);
        form_data.write_hwp_string(&form.value);

        let form_data_bytes = form_data.into_bytes();
        writer.write_record(RecordTagId::FormObject, 1, &form_data_bytes);
    }

    fn write_text_art(&self, writer: &mut ByteWriter, text_art: &TextArtData) {
        // CTRL_HEADER (gso ) - TextArt is a type of general shape object
        self.write_ctrl_header(writer, b" osg", &text_art.common);

        // SHAPE_COMPONENT for TextArt
        let mut shape_comp = ByteWriter::new();
        shape_comp.write_bytes(b" tra"); // "art " in LE for TextArt
        shape_comp.write_i32(0); // x_pos
        shape_comp.write_i32(0); // y_pos
        shape_comp.write_u16(0); // group_level
        shape_comp.write_u16(0); // local_file_version
        shape_comp.write_u32(text_art.width); // initial_width
        shape_comp.write_u32(text_art.height); // initial_height
        shape_comp.write_u32(text_art.width); // current_width
        shape_comp.write_u32(text_art.height); // current_height
        shape_comp.write_u32(0); // properties
        shape_comp.write_u16(0); // rotation_angle
        shape_comp.write_i16(0); // center_x
        shape_comp.write_i16(0); // center_y
        // 변환 행렬 (6x6 단위 행렬 근사)
        for _ in 0..6 {
            shape_comp.write_i32(1);
            shape_comp.write_i32(0);
        }

        let shape_comp_bytes = shape_comp.into_bytes();
        writer.write_record(RecordTagId::ShapeComponent, 1, &shape_comp_bytes);

        // SHAPE_COMPONENT_TEXTART
        let mut text_art_data = ByteWriter::new();
        text_art_data.write_hwp_string(&text_art.text);
        text_art_data.write_hwp_string(text_art.font_name.as_deref().unwrap_or("함초롬바탕"));
        text_art_data.write_u8(text_art.font_style);
        text_art_data.write_u8(text_art.shape_type);
        text_art_data.write_u32(text_art.line_spacing);
        text_art_data.write_u32(text_art.char_spacing);
        text_art_data.write_u8(text_art.alignment);

        let text_art_bytes = text_art_data.into_bytes();
        writer.write_record(RecordTagId::ShapeComponentTextArt, 1, &text_art_bytes);
    }

    fn write_field(&self, writer: &mut ByteWriter, field: &FieldData) {
        // 필드 컨트롤 헤더 작성
        // 필드는 ObjectCommon 없이 컨트롤 ID만 필요
        let ctrl_id = field.field_type.to_control_id();

        let mut ctrl_header = ByteWriter::new();
        ctrl_header.write_bytes(ctrl_id);
        // 필드 속성 (properties) - 기본값
        ctrl_header.write_u32(0);

        let header_bytes = ctrl_header.into_bytes();
        writer.write_record(RecordTagId::ControlHeader, 1, &header_bytes);

        // CTRL_DATA (필드 명령 저장)
        // Parameter Set 형식으로 instruction 저장
        if !field.instruction.is_empty() {
            let mut ctrl_data = ByteWriter::new();
            // Parameter Set format: Item ID(2) + Item Type(2) + Data
            // Item ID 0x4000 = command/instruction
            ctrl_data.write_u16(0x4000); // FIELD_COMMAND item ID
            ctrl_data.write_u16(0x0001); // Item type: string (HWPSTRING)
            ctrl_data.write_hwp_string(&field.instruction);

            let data_bytes = ctrl_data.into_bytes();
            writer.write_record(RecordTagId::ControlData, 1, &data_bytes);
        }
    }

    fn write_shape(&self, writer: &mut ByteWriter, shape: &ShapeData) {
        // CTRL_HEADER (gso )
        self.write_ctrl_header(writer, b" gso", &shape.common);

        // SHAPE_COMPONENT (공통 도형 속성)
        let mut shape_comp = ByteWriter::new();
        // 도형 타입에 따라 컴포넌트 ID 결정
        let component_id: [u8; 4] = match &shape.shape_type {
            ShapeTypeData::Line(_) => [b'e', b'n', b'i', b'l'], // "line" in LE
            ShapeTypeData::Rectangle(_) => [b't', b'c', b'e', b'r'], // "rect" in LE
            ShapeTypeData::Ellipse(_) => [b'l', b'l', b'e', b' '], // " ell" in LE
            ShapeTypeData::Arc(_) => [b' ', b'c', b'r', b'a'],  // "arc " in LE
            ShapeTypeData::Polygon(_) => [b'g', b'l', b'o', b'p'], // "poly" in LE (approximation)
            ShapeTypeData::Curve(_) => [b'v', b'r', b'u', b'c'], // "curv" in LE
            ShapeTypeData::Container(_) => [b' ', b'n', b'o', b'c'], // "con " in LE (container)
            ShapeTypeData::Connector(_) => [b't', b'c', b'n', b'c'], // "cnct" in LE (connector)
        };
        shape_comp.write_bytes(&component_id);
        shape_comp.write_i32(0); // x_pos
        shape_comp.write_i32(0); // y_pos
        shape_comp.write_u16(0); // group_level
        shape_comp.write_u16(0); // local_file_version
        shape_comp.write_u32(shape.width); // initial_width
        shape_comp.write_u32(shape.height); // initial_height
        shape_comp.write_u32(shape.width); // current_width
        shape_comp.write_u32(shape.height); // current_height
        shape_comp.write_u32(0); // properties
        shape_comp.write_u16((shape.rotation * 100.0) as u16); // rotation_angle (degrees * 100)
        shape_comp.write_i32(0); // rotation_center_x
        shape_comp.write_i32(0); // rotation_center_y
        shape_comp.write_u16(1); // render_count (1 = 1개의 도형 요소)

        // 변환 행렬 (18개 element: translation 6 + scale 6 + rotation 6)
        // Translation matrix
        if let Some(trans) = shape.translation_matrix {
            for val in &trans {
                shape_comp.write_i32((*val * 65536.0) as i32); // fixed point representation
            }
        } else {
            // 단위 행렬
            shape_comp.write_i32(65536);
            shape_comp.write_i32(0);
            shape_comp.write_i32(0);
            shape_comp.write_i32(65536);
            shape_comp.write_i32(0);
            shape_comp.write_i32(0);
        }

        // Scale matrix
        if let Some(scale) = shape.scale_matrix {
            for val in &scale {
                shape_comp.write_i32((*val * 65536.0) as i32);
            }
        } else {
            // 단위 행렬
            shape_comp.write_i32(65536);
            shape_comp.write_i32(0);
            shape_comp.write_i32(0);
            shape_comp.write_i32(65536);
            shape_comp.write_i32(0);
            shape_comp.write_i32(0);
        }

        // Rotation matrix
        if let Some(rot) = shape.rotation_matrix {
            for val in &rot {
                shape_comp.write_i32((*val * 65536.0) as i32);
            }
        } else {
            // 단위 행렬
            shape_comp.write_i32(65536);
            shape_comp.write_i32(0);
            shape_comp.write_i32(0);
            shape_comp.write_i32(65536);
            shape_comp.write_i32(0);
            shape_comp.write_i32(0);
        }

        let shape_comp_bytes = shape_comp.into_bytes();
        writer.write_record(RecordTagId::ShapeComponent, 2, &shape_comp_bytes);

        // 도형 타입별 레코드
        match &shape.shape_type {
            ShapeTypeData::Line(line) => self.write_shape_line(writer, line, shape),
            ShapeTypeData::Rectangle(rect) => self.write_shape_rectangle(writer, rect, shape),
            ShapeTypeData::Ellipse(ellipse) => self.write_shape_ellipse(writer, ellipse, shape),
            ShapeTypeData::Arc(arc) => self.write_shape_arc(writer, arc, shape),
            ShapeTypeData::Polygon(poly) => self.write_shape_polygon(writer, poly, shape),
            ShapeTypeData::Curve(curve) => self.write_shape_curve(writer, curve, shape),
            ShapeTypeData::Container(container) => self.write_shape_container(writer, container),
            ShapeTypeData::Connector(connector) => {
                self.write_shape_connector(writer, connector, shape)
            }
        }
    }

    fn write_shape_container(&self, writer: &mut ByteWriter, container: &ContainerShapeData) {
        // Container (그룹 도형)의 모든 자식 도형 재귀적으로 작성
        for child in &container.children {
            self.write_shape(writer, child);
        }
    }

    fn write_shape_line(&self, writer: &mut ByteWriter, line: &LineShapeData, shape: &ShapeData) {
        let mut data = ByteWriter::new();
        data.write_i32(line.start_x);
        data.write_i32(line.start_y);
        data.write_i32(line.end_x);
        data.write_i32(line.end_y);
        // 선 스타일
        data.write_u32(shape.line_color); // color
        data.write_u16(shape.line_thickness); // thickness
        data.write_u8(0); // line_type (실선)
        data.write_u8(0); // end_cap
        data.write_u8(line.start_arrow_type);
        data.write_u8(line.start_arrow_size);
        data.write_u8(line.end_arrow_type);
        data.write_u8(line.end_arrow_size);

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::ShapeComponentLine, 2, &data_bytes);
    }

    fn write_shape_rectangle(
        &self,
        writer: &mut ByteWriter,
        rect: &RectangleShapeData,
        shape: &ShapeData,
    ) {
        let mut data = ByteWriter::new();
        data.write_u8(1); // ratio
        data.write_u32(rect.corner_radius); // corner_radius_x
        data.write_u32(rect.corner_radius); // corner_radius_y
        data.write_u32(rect.corner_radius);
        data.write_u32(rect.corner_radius);
        data.write_u32(rect.corner_radius);
        data.write_u32(rect.corner_radius);
        data.write_u32(rect.corner_radius);
        data.write_u32(rect.corner_radius);
        // 선 스타일
        data.write_u32(shape.line_color);
        data.write_u16(shape.line_thickness);
        data.write_u8(0); // line_type
        data.write_u8(0); // end_cap
        // 채우기 (구버전 호환: 단순 색상으로 작성)
        data.write_u32(Self::get_fill_color(&shape.fill));

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::ShapeComponentRectangle, 2, &data_bytes);
    }

    fn write_shape_ellipse(
        &self,
        writer: &mut ByteWriter,
        ellipse: &EllipseShapeData,
        shape: &ShapeData,
    ) {
        let mut data = ByteWriter::new();
        data.write_u32(0); // properties
        data.write_i32(0); // center_x
        data.write_i32(0); // center_y
        data.write_i32((shape.width / 2) as i32); // axis1_x
        data.write_i32(0); // axis1_y
        data.write_i32(0); // axis2_x
        data.write_i32((shape.height / 2) as i32); // axis2_y
        data.write_i32(ellipse.start_angle); // start_angle (x100)
        data.write_i32(0); // start1_x
        data.write_i32(0); // start1_y
        data.write_i32(ellipse.end_angle); // end_angle (x100)
        data.write_i32(0); // end1_x
        data.write_i32(0); // end1_y
        data.write_u8(ellipse.arc_type); // arc_type
        // 선 스타일
        data.write_u32(shape.line_color);
        data.write_u16(shape.line_thickness);
        data.write_u8(0); // line_type
        data.write_u8(0); // end_cap

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::ShapeComponentEllipse, 2, &data_bytes);
    }

    fn write_shape_arc(&self, writer: &mut ByteWriter, arc: &ArcShapeData, shape: &ShapeData) {
        let mut data = ByteWriter::new();
        data.write_u8(arc.arc_type);
        data.write_i32(0); // center_x
        data.write_i32(0); // center_y
        data.write_i32((shape.width / 2) as i32); // axis1_x
        data.write_i32(0); // axis1_y
        data.write_i32(0); // axis2_x
        data.write_i32((shape.height / 2) as i32); // axis2_y
        data.write_i32(arc.start_angle);
        data.write_i32(arc.end_angle);
        // 선 스타일
        data.write_u32(shape.line_color);
        data.write_u16(shape.line_thickness);
        data.write_u8(0); // line_type
        data.write_u8(0); // end_cap

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::ShapeComponentArc, 2, &data_bytes);
    }

    fn write_shape_polygon(
        &self,
        writer: &mut ByteWriter,
        poly: &PolygonShapeData,
        shape: &ShapeData,
    ) {
        let mut data = ByteWriter::new();
        data.write_u16(poly.points.len() as u16); // point_count
        for (x, y) in &poly.points {
            data.write_i32(*x);
            data.write_i32(*y);
        }
        // 선 스타일
        data.write_u32(shape.line_color);
        data.write_u16(shape.line_thickness);
        data.write_u8(0); // line_type
        data.write_u8(0); // end_cap
        // 채우기 (구버전 호환: 단순 색상으로 작성)
        data.write_u32(Self::get_fill_color(&shape.fill));

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::ShapeComponentPolygon, 2, &data_bytes);
    }

    fn write_shape_curve(
        &self,
        writer: &mut ByteWriter,
        curve: &CurveShapeData,
        shape: &ShapeData,
    ) {
        let mut data = ByteWriter::new();
        data.write_u16(curve.points.len() as u16); // point_count
        for (x, y, pt_type) in &curve.points {
            data.write_i32(*x);
            data.write_i32(*y);
            data.write_u8(*pt_type);
        }
        data.write_u8(if curve.closed { 1 } else { 0 }); // closed
        // 선 스타일
        data.write_u32(shape.line_color);
        data.write_u16(shape.line_thickness);
        data.write_u8(0); // line_type
        data.write_u8(0); // end_cap

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::ShapeComponentCurve, 2, &data_bytes);
    }

    fn write_shape_connector(
        &self,
        writer: &mut ByteWriter,
        connector: &ConnectorShapeData,
        shape: &ShapeData,
    ) {
        // 연결선은 특수한 선 타입으로 처리
        let mut data = ByteWriter::new();

        // 연결선 타입 (0~8)
        data.write_u8(connector.line_type);

        // 시작점
        data.write_i32(connector.start_x);
        data.write_i32(connector.start_y);
        data.write_u32(connector.start_subject_id.unwrap_or(0));
        data.write_u32(connector.start_subject_index.unwrap_or(0));

        // 끝점
        data.write_i32(connector.end_x);
        data.write_i32(connector.end_y);
        data.write_u32(connector.end_subject_id.unwrap_or(0));
        data.write_u32(connector.end_subject_index.unwrap_or(0));

        // 제어점 개수 및 데이터
        data.write_u16(connector.control_points.len() as u16);
        for (x, y) in &connector.control_points {
            data.write_i32(*x);
            data.write_i32(*y);
        }

        // 선 스타일
        data.write_u32(shape.line_color);
        data.write_u16(shape.line_thickness);
        data.write_u8(0); // line_style (실선)
        data.write_u8(0); // end_cap

        // 화살표 스타일
        data.write_u8(connector.start_arrow_type);
        data.write_u8(connector.start_arrow_size);
        data.write_u8(connector.end_arrow_type);
        data.write_u8(connector.end_arrow_size);

        let data_bytes = data.into_bytes();
        writer.write_record(RecordTagId::ShapeComponentLine, 2, &data_bytes);
    }

    fn calculate_char_count(&self, para: &ParagraphData) -> u32 {
        // 텍스트 길이 + 문단 끝 표시 (13)
        para.text.len() as u32 + 1
    }

    /// FillInfo에서 단순 색상값 추출 (구버전 호환용)
    fn get_fill_color(fill: &FillInfo) -> u32 {
        match fill {
            FillInfo::None => 0xFFFFFF,
            FillInfo::Pattern(pattern) => pattern.background_color.value(),
            FillInfo::Gradient(gradient) => gradient
                .colors
                .first()
                .map(|c| c.value())
                .unwrap_or(0xFFFFFF),
            FillInfo::Image(_) => 0xFFFFFF,
        }
    }
}

impl Default for BodyWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PageDefinitionData {
    fn default() -> Self {
        Self {
            // A4 용지 기본값 (HwpUnit: 1/7200 inch)
            // A4: 210mm × 297mm
            // 210mm = 8.27 inch = 59544 HwpUnit
            // 297mm = 11.69 inch = 84168 HwpUnit
            paper_width: 59544,
            paper_height: 84168,
            margin_left: 8503,   // 30mm
            margin_right: 8503,  // 30mm
            margin_top: 5669,    // 20mm
            margin_bottom: 4252, // 15mm
            margin_header: 4252, // 15mm
            margin_footer: 4252, // 15mm
            margin_gutter: 0,
            properties: 0,
        }
    }
}

impl Default for ParagraphData {
    fn default() -> Self {
        Self {
            para_shape_id: 0,
            style_id: 0,
            text: Vec::new(),
            char_shape_refs: vec![CharShapeRef {
                position: 0,
                char_shape_id: 0,
            }],
            range_tags: Vec::new(),
            controls: Vec::new(),
        }
    }
}

impl ParagraphData {
    /// 텍스트로 문단을 생성합니다.
    pub fn with_text(text: &str) -> Self {
        let utf16: Vec<u16> = text.encode_utf16().collect();
        Self {
            text: utf16,
            ..Default::default()
        }
    }
}

impl Default for TableData {
    fn default() -> Self {
        Self {
            common: ObjectCommonData::default(),
            rows: 1,
            columns: 1,
            column_widths: vec![4252], // 15mm
            cells: Vec::new(),
            border_fill_id: 0,
            caption: None,
            properties: 0,
        }
    }
}

impl Default for TableCellData {
    fn default() -> Self {
        Self {
            col: 0,
            row: 0,
            col_span: 1,
            row_span: 1,
            width: 4252,
            height: 1417, // 5mm
            paragraphs: Vec::new(),
            border_fill_id: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_section() {
        let mut body_writer = BodyWriter::new();
        body_writer.add_section(SectionData::default());

        let sections = body_writer.build();
        assert_eq!(sections.len(), 1);
    }

    #[test]
    fn test_section_with_paragraph() {
        let mut body_writer = BodyWriter::new();
        let mut section = SectionData::default();
        section
            .paragraphs
            .push(ParagraphData::with_text("Hello, World!"));

        body_writer.add_section(section);

        let sections = body_writer.build();
        assert_eq!(sections.len(), 1);
        assert!(!sections[0].is_empty());
    }

    #[test]
    fn test_paragraph_with_text() {
        let para = ParagraphData::with_text("테스트");
        assert!(!para.text.is_empty());
    }
}
