//! 컨트롤 (내장 객체)
//!
//! 문단 내에 삽입되는 표, 그림, 도형 등의 객체를 정의합니다.

use crate::paragraph::Paragraph;
use crate::picture::Picture;
use crate::shape::Shape;
use crate::table::Table;
use primitive::{
    BinaryDataId, HeaderFooterApplyTo, HeightRelativeTo, HorizontalOffsetType,
    HorizontalRelativeTo, HwpUnit, Insets, LineWrap, NoteNumberPosition, NumberFormat, Point, Size,
    TextDirection, TextWrapSide, TextWrapType, VerticalAlignment, VerticalOffsetType,
    VerticalRelativeTo, WidthRelativeTo,
};

// Re-export primitive types
pub use primitive::{
    AutoNumberFormat, AutoNumberType, ButtonBackStyle, ButtonValue, CaptionPosition, ChartType,
    EditScrollBars, EditTabKeyBehavior, EditTextAlignment, EquationFormat, EquationLineMode,
    FormCharProperty, FormListItem, FormObjectType, MemoType, ObjectMargin, ObjectNumberingType,
    ScrollBarType, TextArtAlignment, TextArtFontStyle, TextArtFontType, TextArtProperties,
    TextArtShapeType, VideoType,
};

/// 컨트롤 (내장 객체)
#[derive(Debug, Clone)]
pub enum Control {
    /// 표
    Table(Box<Table>),
    /// 그림
    Picture(Box<Picture>),
    /// 도형
    Shape(Box<Shape>),
    /// 수식
    Equation(Box<Equation>),
    /// OLE 객체
    Ole(Box<OleObject>),
    /// 텍스트 박스
    TextBox(Box<TextBox>),
    /// 머리글
    Header(Box<HeaderFooterControl>),
    /// 바닥글
    Footer(Box<HeaderFooterControl>),
    /// 각주
    Footnote(Box<Note>),
    /// 미주
    Endnote(Box<Note>),
    /// 하이퍼링크
    Hyperlink(Box<Hyperlink>),
    /// 책갈피
    Bookmark(Box<Bookmark>),
    /// 색인 표시
    IndexMark(Box<IndexMark>),
    /// 자동 번호
    AutoNumber(Box<AutoNumber>),
    /// 새 번호
    NewNumber(Box<NewNumber>),
    /// 숨은 설명
    HiddenComment(Box<HiddenComment>),
    /// 차트
    Chart(Box<Chart>),
    /// 비디오
    Video(Box<Video>),
    /// 양식 객체
    FormObject(Box<FormObject>),
    /// 글맵시 (TextArt)
    TextArt(Box<TextArt>),
    /// 메모 (Memo/Annotation)
    Memo(Box<Memo>),
    /// 알 수 없는 컨트롤
    Unknown(Box<UnknownControl>),
}

impl Control {
    /// 표인지 확인
    pub const fn is_table(&self) -> bool {
        matches!(self, Control::Table(_))
    }

    /// 그림인지 확인
    pub const fn is_picture(&self) -> bool {
        matches!(self, Control::Picture(_))
    }

    /// 도형인지 확인
    pub const fn is_shape(&self) -> bool {
        matches!(self, Control::Shape(_))
    }
}

// ObjectMargin re-exported from primitive

/// 개체 공통 속성
#[derive(Debug, Clone, Default)]
pub struct ObjectCommon {
    /// 개체 ID
    pub id: Option<u32>,
    /// 위치
    pub position: Point,
    /// 크기
    pub size: Size,
    /// Z 순서
    pub z_order: i32,
    /// 텍스트 배치
    pub text_wrap: TextWrap,
    /// 캡션
    pub caption: Option<Caption>,
    /// 번호 매기기 종류 (HWPX 전용: 그림/표/수식 번호)
    pub numbering_type: Option<ObjectNumberingType>,
    /// 도형 주석 (HWPX 전용)
    pub shape_comment: Option<String>,
    /// 메타 태그 (HWPX 전용)
    pub meta_tag: Option<String>,
    /// 변경됨 여부 (HWPX 전용)
    pub dirty: bool,
    /// 너비 기준 (HWPX/HWP 확장)
    pub width_relative_to: WidthRelativeTo,
    /// 높이 기준 (HWPX/HWP 확장)
    pub height_relative_to: HeightRelativeTo,
    /// 개체 여백 (본문과의 간격)
    pub margin: ObjectMargin,
}

/// 텍스트 배치 설정
#[derive(Debug, Clone, Default)]
pub struct TextWrap {
    /// 배치 종류
    pub wrap_type: TextWrapType,
    /// 배치 방향
    pub wrap_side: TextWrapSide,
    /// 본문과의 간격 (구 버전 호환용, ObjectMargin 사용 권장)
    pub margin: HwpUnit,
    /// 수직 기준
    pub vertical_rel: VerticalRelativeTo,
    /// 수평 기준
    pub horizontal_rel: HorizontalRelativeTo,
    /// 수직 오프셋 타입 (정렬 방식)
    pub vertical_offset_type: VerticalOffsetType,
    /// 수평 오프셋 타입 (정렬 방식)
    pub horizontal_offset_type: HorizontalOffsetType,
    /// 글자처럼 취급
    pub treat_as_char: bool,
    /// 페이지와 함께 이동
    pub flow_with_text: bool,
    /// 다른 개체 겹침 허용
    pub allow_overlap: bool,
}

/// 캡션
#[derive(Debug, Clone)]
pub struct Caption {
    /// 캡션 위치
    pub position: CaptionPosition,
    /// 캡션 너비
    pub width: HwpUnit,
    /// 캡션 간격
    pub gap: HwpUnit,
    /// 캡션 내용
    pub paragraphs: Vec<Paragraph>,
}

/// 수식
#[derive(Debug, Clone)]
pub struct Equation {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 수식 스크립트 (LaTeX 또는 MathML 등)
    pub script: String,
    /// 수식 형식
    pub format: EquationFormat,
    /// 기준선 오프셋
    pub baseline_offset: HwpUnit,
    /// 글자 크기
    pub font_size: HwpUnit,
    /// 텍스트 색상
    pub color: Option<primitive::Color>,
    /// 라인 모드 (HWP 전용: Baseline/Center/Bottom/Top)
    pub line_mode: Option<EquationLineMode>,
    /// 수식 버전 문자열 (HWP 전용)
    pub version: Option<String>,
    /// 수식 폰트 이름 (HWP 전용)
    pub font_name: Option<String>,
    /// 속성 플래그 (HWP 전용)
    pub properties: Option<u32>,
}

/// OLE 객체
#[derive(Debug, Clone)]
pub struct OleObject {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 바이너리 데이터 ID
    pub binary_id: BinaryDataId,
    /// OLE 클래스 ID
    pub class_id: Option<String>,
    /// 미리보기 이미지 ID
    pub preview_image_id: Option<BinaryDataId>,
}

/// 텍스트 박스
#[derive(Debug, Clone)]
pub struct TextBox {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 내용
    pub paragraphs: Vec<Paragraph>,
    /// 텍스트 방향
    pub text_direction: TextDirection,
    /// 세로 정렬
    pub vertical_alignment: VerticalAlignment,
    /// 안쪽 여백
    pub padding: Insets,
    /// 편집 가능 여부
    pub editable: bool,
    /// 이름 (HWPX)
    pub name: Option<String>,
    /// 마지막 너비 (HWPX)
    pub last_width: Option<HwpUnit>,
    /// 줄 나눔 방식 (HWPX DrawText)
    pub line_wrap: LineWrap,
    /// 연결 리스트 ID 참조 (HWPX)
    pub link_list_id_reference: Option<u32>,
    /// 연결 리스트 다음 ID 참조 (HWPX)
    pub link_list_next_id_reference: Option<u32>,
    /// 텍스트 영역 폭 (HWPX)
    pub text_width: Option<HwpUnit>,
    /// 텍스트 영역 높이 (HWPX)
    pub text_height: Option<HwpUnit>,
    /// 텍스트 참조 여부 (HWPX)
    pub has_text_reference: bool,
    /// 번호 참조 여부 (HWPX)
    pub has_number_reference: bool,
}

/// 머리글/바닥글 컨트롤
#[derive(Debug, Clone)]
pub struct HeaderFooterControl {
    /// 적용 대상 페이지
    pub apply_to: HeaderFooterApplyTo,
    /// 내용
    pub paragraphs: Vec<Paragraph>,
}

/// 각주/미주
#[derive(Debug, Clone)]
pub struct Note {
    /// 번호
    pub number: u32,
    /// 번호 형식
    pub number_format: NumberFormat,
    /// 번호 위치
    pub number_position: NoteNumberPosition,
    /// 내용
    pub paragraphs: Vec<Paragraph>,
    /// 인스턴스 ID
    pub instance_id: Option<u32>,
}

/// 하이퍼링크
#[derive(Debug, Clone)]
pub struct Hyperlink {
    /// 링크 대상
    pub target: HyperlinkTarget,
    /// 툴팁
    pub tooltip: Option<String>,
    /// 표시 텍스트 (target과 다를 때)
    pub display_text: Option<String>,
}

/// 하이퍼링크 대상
#[derive(Debug, Clone)]
pub enum HyperlinkTarget {
    /// URL
    Url(String),
    /// 이메일
    Email(String),
    /// 파일 경로
    File(String),
    /// 문서 내 책갈피
    Bookmark(String),
}

/// 책갈피
#[derive(Debug, Clone)]
pub struct Bookmark {
    /// 책갈피 이름
    pub name: String,
}

/// 색인 표시 (IndexMark)
#[derive(Debug, Clone, Default)]
pub struct IndexMark {
    /// 첫 번째 키
    pub first_key: String,
    /// 두 번째 키
    pub second_key: String,
}

/// 자동 번호
#[derive(Debug, Clone)]
pub struct AutoNumber {
    /// 번호 종류
    pub number_type: AutoNumberType,
    /// 번호 형식
    pub number_format: NumberFormat,
    /// 자동 번호 형식 상세 (HWPX용)
    pub auto_number_format: Option<AutoNumberFormat>,
}

// AutoNumberFormat re-exported from primitive

/// 새 번호
#[derive(Debug, Clone)]
pub struct NewNumber {
    /// 번호 종류
    pub number_type: AutoNumberType,
    /// 새 번호 값
    pub number: u32,
}

/// 숨은 설명
#[derive(Debug, Clone)]
pub struct HiddenComment {
    /// 내용
    pub paragraphs: Vec<Paragraph>,
}

/// 차트
#[derive(Debug, Clone)]
pub struct Chart {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 차트 데이터 ID
    pub chart_id: String,
    /// 차트 종류
    pub chart_type: ChartType,
}

/// 비디오
#[derive(Debug, Clone)]
pub struct Video {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 비디오 종류
    pub video_type: VideoType,
    /// 비디오 파일 ID (임베디드인 경우)
    pub video_id: Option<BinaryDataId>,
    /// 소스 URL (링크인 경우)
    pub source_url: Option<String>,
    /// 미리보기 이미지 ID
    pub preview_image_id: Option<BinaryDataId>,
    /// 포스터 바이너리 ID (HWP 전용 - poster_bin_id)
    pub poster_binary_id: Option<BinaryDataId>,
    /// 비디오 너비 (HWP 전용, HWP 단위)
    pub width: Option<HwpUnit>,
    /// 비디오 높이 (HWP 전용, HWP 단위)
    pub height: Option<HwpUnit>,
}

/// 양식 객체
#[derive(Debug, Clone, Default)]
pub struct FormObject {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 양식 종류
    pub form_type: FormObjectType,
    /// 이름
    pub name: Option<String>,
    /// 값 (Edit의 텍스트 내용)
    pub value: Option<String>,
    /// 양식 글자 속성
    pub char_property: FormCharProperty,
    /// 목록 항목들 (ComboBox, ListBox용)
    pub items: Vec<FormListItem>,

    // 공통 속성
    /// 전경색
    pub fore_color: Option<primitive::Color>,
    /// 배경색
    pub back_color: Option<primitive::Color>,
    /// 그룹 이름
    pub group_name: Option<String>,
    /// 탭 이동 허용 여부
    pub tab_stop: bool,
    /// 사용 가능 여부
    pub enabled: bool,
    /// 편집 가능 여부
    pub editable: bool,
    /// 테두리 타입 참조
    pub border_type_id_ref: Option<u32>,
    /// 프레임 출력 여부
    pub draw_frame: bool,
    /// 인쇄 여부
    pub printable: bool,
    /// 탭 순서
    pub tab_order: Option<i32>,

    // Button 공통 속성 (Button, CheckBox, RadioButton)
    /// 버튼 캡션 텍스트
    pub caption: Option<String>,
    /// 버튼 상태 값 (CheckBox, RadioButton용)
    pub button_value: Option<ButtonValue>,
    /// 라디오 그룹 이름 (RadioButton용)
    pub radio_group_name: Option<String>,
    /// 배경 스타일 (Button용)
    pub back_style: Option<ButtonBackStyle>,
    /// 삼중 상태 사용 여부 (CheckBox용)
    pub tri_state: bool,
    /// 그라디언트 채우기 사용 여부 (Button용)
    pub gradient_fill: bool,
    /// 이미지 채우기 사용 여부 (Button용)
    pub image_fill: bool,

    // Edit 전용 속성
    /// 다중 줄 입력 여부
    pub multiline: bool,
    /// 비밀번호 마스킹 문자
    pub password_char: Option<String>,
    /// 최대 길이
    pub max_length: Option<u32>,
    /// 스크롤바 표시 설정
    pub scroll_bars: Option<EditScrollBars>,
    /// 탭키 동작
    pub tab_key_behavior: Option<EditTabKeyBehavior>,
    /// 숫자 입력만 허용
    pub num_only: bool,
    /// 읽기 전용 여부
    pub read_only: bool,
    /// 텍스트 정렬
    pub alignment: Option<EditTextAlignment>,

    // ComboBox/ListBox 속성
    /// 편집 가능 여부 (ComboBox용)
    pub edit_enable: bool,
    /// 선택된 값 (ComboBox, ListBox용)
    pub selected_value: Option<String>,
    /// 표시 행 수 (ComboBox용)
    pub list_box_rows: Option<i32>,
    /// 목록 폭 (ComboBox용)
    pub list_box_width: Option<i32>,
    /// 항목 높이 (ListBox용)
    pub item_height: Option<i32>,
    /// 최상단 표시 인덱스 (ListBox용)
    pub top_index: Option<u32>,

    // ScrollBar 전용 속성
    /// 스크롤바 타입
    pub bar_type: Option<ScrollBarType>,
    /// 최소값
    pub min: Option<i32>,
    /// 최대값
    pub max: Option<i32>,
    /// 현재 값
    pub scroll_value: Option<i32>,
    /// 작은 증감값
    pub small_change: Option<u32>,
    /// 큰 증감값
    pub large_change: Option<u32>,
    /// 페이지 단위
    pub page: Option<i32>,
    /// 반복 지연 시간
    pub delay: Option<u32>,
}

// EditTextAlignment re-exported from primitive

/// 알 수 없는 컨트롤
#[derive(Debug, Clone)]
pub struct UnknownControl {
    /// 컨트롤 ID (4바이트 문자열)
    pub ctrl_id: [u8; 4],
    /// 원시 데이터
    pub data: Vec<u8>,
}

/// 글맵시 (TextArt/WordArt)
#[derive(Debug, Clone)]
pub struct TextArt {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 텍스트 내용
    pub text: String,
    /// 글꼴 이름
    pub font_name: Option<String>,
    /// 글꼴 스타일 (REGULAR, BOLD, ITALIC 등)
    pub font_style: TextArtFontStyle,
    /// 글맵시 모양
    pub shape: TextArtShapeType,
    /// 줄 간격 (50-500, 기본 120)
    pub line_spacing: u32,
    /// 자간 (50-500, 기본 100)
    pub char_spacing: u32,
    /// 정렬
    pub alignment: TextArtAlignment,
    /// 선 스타일
    pub line: crate::shape::LineStyle,
    /// 채우기
    pub fill: crate::border_fill::Fill,
    /// 그림자
    pub shadow: Option<crate::shape::ShapeShadow>,
    /// 글꼴 타입 (HWPX 전용 - TTF/HTF)
    pub font_type: Option<TextArtFontType>,
    /// HWPX 추가 속성
    pub text_art_pr: Option<TextArtProperties>,
}

// TextArtFontStyle, TextArtShapeType re-exported from primitive

/// 메모 (Memo/Annotation)
///
/// 문서에 첨부되는 주석/메모를 나타냅니다.
#[derive(Debug, Clone)]
pub struct Memo {
    /// 메모 내용 (문단 목록)
    pub paragraphs: Vec<Paragraph>,
    /// 작성자
    pub author: Option<String>,
    /// 작성일시
    pub date: Option<String>,
    /// 메모 너비 (HWP units)
    pub width: Option<HwpUnit>,
    /// 테두리 선 두께
    pub line_width: Option<HwpUnit>,
    /// 테두리 선 색상
    pub line_color: Option<primitive::Color>,
    /// 배경 색상
    pub fill_color: Option<primitive::Color>,
    /// 활성 색상 (포커스 시)
    pub active_color: Option<primitive::Color>,
    /// 메모 종류 (HWPX 전용)
    pub memo_type: Option<MemoType>,
}
