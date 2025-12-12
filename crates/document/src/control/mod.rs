//! 컨트롤 (Control)
//!
//! 문서 내 다양한 개체(표, 그림, 도형 등)를 정의합니다.

mod common;

pub use common::*;

use crate::id::ParagraphId;
use crate::table::Table;
use primitive::{Alignment, NumberFormat, VerticalAlignment};

// Re-export from primitive (only types with matching variants)
pub use primitive::{ArcType, ImageEffect, ImageFlip};

/// 컨트롤
#[derive(Debug, Clone)]
pub enum Control {
    /// 표
    Table(Table),
    /// 그림
    Picture(Picture),
    /// 도형
    Shape(Shape),
    /// 수식
    Equation(Equation),
    /// OLE 객체
    Ole(Ole),
    /// 텍스트 박스
    TextBox(TextBox),
    /// 각주
    Footnote(Note),
    /// 미주
    Endnote(Note),
    /// 숨은 설명
    HiddenComment(HiddenComment),
    /// 하이퍼링크
    Hyperlink(Hyperlink),
    /// 책갈피
    Bookmark(Bookmark),
    /// 자동 번호
    AutoNumber(AutoNumber),
    /// 새 번호
    NewNumber(NewNumber),
    /// 양식 객체
    FormObject(FormObject),
    /// 비디오
    Video(Video),
    /// 차트
    Chart(Chart),
    /// 글맵시
    TextArt(TextArt),
    /// 글자 겹침
    Compose(Compose),
    /// 덧말
    Dutmal(Dutmal),
    /// 색인 표시
    IndexMark(IndexMark),
    /// 연결선
    ConnectLine(ConnectLine),
    /// 알 수 없는 컨트롤
    Unknown(Vec<u8>),
}

/// 그림
#[derive(Debug, Clone)]
pub struct Picture {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 바이너리 데이터 ID
    pub binary_id: ir::BinaryDataId,
    /// 원본 크기
    pub original_size: ir::Size,
    /// 자르기
    pub crop: ir::Insets,
    /// 뒤집기
    pub flip: ImageFlip,
    /// 회전 (도)
    pub rotation: f64,
    /// 효과
    pub effect: ImageEffect,
    /// 밝기 (-100 ~ 100)
    pub brightness: i8,
    /// 대비 (-100 ~ 100)
    pub contrast: i8,
    /// 투명도 (0 ~ 100)
    pub alpha: u8,
    /// 테두리
    pub border: Option<LineStyle>,
    /// 그림자
    pub shadow: Option<Shadow>,
}


/// 도형
#[derive(Debug, Clone)]
pub struct Shape {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 도형 종류
    pub shape_type: ShapeType,
    /// 선 스타일
    pub line: Option<LineStyle>,
    /// 채우기
    pub fill: Option<Fill>,
    /// 그림자
    pub shadow: Option<Shadow>,
    /// 회전 (도)
    pub rotation: f64,
    /// 내부 텍스트
    pub text: Option<ShapeText>,
}

/// 도형 종류
#[derive(Debug, Clone)]
pub enum ShapeType {
    /// 직선
    Line {
        start: ir::Point,
        end: ir::Point,
        start_arrow: Option<Arrow>,
        end_arrow: Option<Arrow>,
    },
    /// 사각형
    Rectangle {
        corner_radius: ir::HwpUnit,
    },
    /// 타원
    Ellipse {
        arc_type: ArcType,
        start_angle: f64,
        end_angle: f64,
    },
    /// 호
    Arc {
        arc_type: ArcType,
        start_angle: f64,
        end_angle: f64,
    },
    /// 다각형
    Polygon {
        points: Vec<ir::Point>,
    },
    /// 곡선
    Curve {
        points: Vec<CurvePoint>,
        closed: bool,
    },
    /// 연결선
    Connector {
        connector_type: ConnectorType,
        points: Vec<ir::Point>,
        start_arrow: Option<Arrow>,
        end_arrow: Option<Arrow>,
    },
    /// 그룹
    Group {
        children: Vec<Shape>,
    },
}


/// 곡선 점
#[derive(Debug, Clone)]
pub struct CurvePoint {
    pub point: ir::Point,
    pub point_type: CurvePointType,
}

/// 곡선 점 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurvePointType {
    #[default]
    Normal,
    Control1,
    Control2,
}

/// 연결선 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectorType {
    #[default]
    Straight,
    Elbow,
    Curved,
}

/// 화살표
#[derive(Debug, Clone)]
pub struct Arrow {
    pub arrow_type: ArrowType,
    pub size: ArrowSize,
    pub filled: bool,
}

/// 화살표 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrowType {
    #[default]
    None,
    Normal,
    Stealth,
    Diamond,
    Circle,
    Open,
}

/// 화살표 크기
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrowSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// 도형 내부 텍스트
#[derive(Debug, Clone)]
pub struct ShapeText {
    /// 문단 목록
    pub paragraphs: Vec<ParagraphId>,
    /// 안쪽 여백
    pub padding: ir::Insets,
    /// 세로 정렬
    pub vertical_alignment: VerticalAlignment,
    /// 텍스트 방향
    pub text_direction: TextDirection,
    /// 편집 가능 여부
    pub editable: bool,
}

/// 텍스트 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextDirection {
    #[default]
    Horizontal,
    Vertical,
    VerticalAll,
}

/// 수식
#[derive(Debug, Clone)]
pub struct Equation {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 수식 스크립트
    pub script: String,
    /// 수식 형식
    pub format: EquationFormat,
    /// 기준선 오프셋
    pub baseline_offset: ir::HwpUnit,
    /// 글꼴 크기
    pub font_size: ir::HwpUnit,
    /// 색상
    pub color: ir::Color,
}

/// 수식 형식
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EquationFormat {
    #[default]
    HwpScript,
    MathML,
    LaTeX,
}

/// OLE 객체
#[derive(Debug, Clone)]
pub struct Ole {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 바이너리 데이터 ID
    pub binary_id: ir::BinaryDataId,
    /// 클래스 ID
    pub class_id: Option<String>,
    /// 미리보기 이미지 ID
    pub preview_image_id: Option<ir::BinaryDataId>,
}

/// 텍스트 박스
#[derive(Debug, Clone)]
pub struct TextBox {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 문단 목록
    pub paragraphs: Vec<ParagraphId>,
    /// 텍스트 방향
    pub text_direction: TextDirection,
    /// 세로 정렬
    pub vertical_alignment: VerticalAlignment,
    /// 안쪽 여백
    pub padding: ir::Insets,
    /// 편집 가능 여부
    pub editable: bool,
}

/// 각주/미주
#[derive(Debug, Clone)]
pub struct Note {
    /// 번호
    pub number: u32,
    /// 번호 형식
    pub number_format: NumberFormat,
    /// 번호 위치
    pub number_position: crate::section::NoteNumberPosition,
    /// 문단 목록
    pub paragraphs: Vec<ParagraphId>,
    /// 인스턴스 ID
    pub instance_id: Option<u32>,
}

/// 숨은 설명
#[derive(Debug, Clone)]
pub struct HiddenComment {
    /// 문단 목록
    pub paragraphs: Vec<ParagraphId>,
}

/// 하이퍼링크
#[derive(Debug, Clone)]
pub struct Hyperlink {
    /// 대상
    pub target: HyperlinkTarget,
    /// 툴팁
    pub tooltip: Option<String>,
    /// 표시 텍스트
    pub display_text: Option<String>,
}

/// 하이퍼링크 대상
#[derive(Debug, Clone)]
pub enum HyperlinkTarget {
    Url(String),
    Email(String),
    File(String),
    Bookmark(String),
}

/// 책갈피
#[derive(Debug, Clone)]
pub struct Bookmark {
    /// 이름
    pub name: String,
}

/// 자동 번호
#[derive(Debug, Clone)]
pub struct AutoNumber {
    /// 번호 종류
    pub number_type: AutoNumberType,
    /// 번호 형식
    pub number_format: NumberFormat,
}

/// 자동 번호 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoNumberType {
    Page,
    Footnote,
    Endnote,
    Picture,
    Table,
    Equation,
}

/// 새 번호
#[derive(Debug, Clone)]
pub struct NewNumber {
    /// 번호 종류
    pub number_type: AutoNumberType,
    /// 새 번호 값
    pub number: u32,
}

/// 양식 객체
#[derive(Debug, Clone)]
pub struct FormObject {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 양식 종류
    pub form_type: FormObjectType,
    /// 이름
    pub name: String,
    /// 값
    pub value: Option<String>,
}

/// 양식 객체 종류
#[derive(Debug, Clone)]
pub enum FormObjectType {
    Button,
    CheckBox { checked: bool },
    RadioButton { group_name: String, checked: bool },
    ComboBox { items: Vec<String>, selected: Option<usize> },
    ListBox { items: Vec<String>, selected: Option<usize> },
    Edit { multiline: bool, password: bool },
    ScrollBar { min: i32, max: i32, value: i32 },
}

/// 비디오
#[derive(Debug, Clone)]
pub struct Video {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 비디오 종류
    pub video_type: VideoType,
    /// 비디오 ID
    pub video_id: Option<ir::BinaryDataId>,
    /// 소스 URL
    pub source_url: Option<String>,
    /// 미리보기 이미지 ID
    pub preview_image_id: Option<ir::BinaryDataId>,
}

/// 비디오 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VideoType {
    #[default]
    Embedded,
    Linked,
    Web,
}

/// 차트
#[derive(Debug, Clone)]
pub struct Chart {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 차트 ID
    pub chart_id: ir::BinaryDataId,
    /// 차트 종류
    pub chart_type: ChartType,
}

/// 차트 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChartType {
    #[default]
    Bar,
    Line,
    Pie,
    Area,
    Scatter,
    Radar,
}

/// 글맵시
#[derive(Debug, Clone)]
pub struct TextArt {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 텍스트
    pub text: String,
    /// 폰트 이름
    pub font_name: String,
    /// 폰트 스타일
    pub font_style: FontStyle,
    /// 글맵시 모양
    pub shape: TextArtShape,
    /// 줄 간격
    pub line_spacing: ir::Percent,
    /// 글자 간격
    pub char_spacing: ir::Percent,
    /// 정렬
    pub alignment: Alignment,
    /// 선 스타일
    pub line: Option<LineStyle>,
    /// 채우기
    pub fill: Option<Fill>,
    /// 그림자
    pub shadow: Option<Shadow>,
}

/// 폰트 스타일
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontStyle {
    #[default]
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

/// 글맵시 모양
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextArtShape {
    #[default]
    Rectangle,
    Circle,
    Arch,
    Wave,
    // TODO: 나머지 40+ 종류 추가
}

/// 글자 겹침
#[derive(Debug, Clone)]
pub struct Compose {
    /// 겹침 종류
    pub compose_type: ComposeType,
    /// 테두리 종류
    pub circle_type: CircleType,
    /// 글자 크기 비율
    pub char_size: ir::Percent,
    /// 겹침 텍스트
    pub compose_text: String,
}

/// 겹침 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComposeType {
    #[default]
    Spread,
    Overlap,
}

/// 테두리 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CircleType {
    #[default]
    None,
    Circle,
    // TODO: 나머지 12종 추가
}

/// 덧말
#[derive(Debug, Clone)]
pub struct Dutmal {
    /// 위치
    pub position: DutmalPosition,
    /// 정렬
    pub alignment: Alignment,
    /// 주 텍스트
    pub main_text: String,
    /// 덧말 텍스트
    pub sub_text: String,
    /// 크기 비율
    pub size_ratio: ir::Percent,
}

/// 덧말 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DutmalPosition {
    #[default]
    Top,
    Bottom,
}

/// 색인 표시
#[derive(Debug, Clone)]
pub struct IndexMark {
    /// 첫 번째 키
    pub first_key: String,
    /// 두 번째 키
    pub second_key: Option<String>,
}

/// 연결선
#[derive(Debug, Clone)]
pub struct ConnectLine {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 연결선 종류
    pub line_type: ConnectLineType,
    /// 시작점
    pub start_point: ConnectionPoint,
    /// 끝점
    pub end_point: ConnectionPoint,
    /// 제어점들
    pub control_points: Vec<ir::Point>,
}

/// 연결선 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectLineType {
    #[default]
    Straight,
    // TODO: 나머지 8종 추가
}

/// 연결점
#[derive(Debug, Clone)]
pub struct ConnectionPoint {
    /// 좌표
    pub point: ir::Point,
    /// 연결 대상 ID
    pub subject_id: Option<String>,
    /// 연결 인덱스
    pub subject_index: Option<u32>,
}
