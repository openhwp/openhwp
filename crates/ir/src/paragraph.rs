//! 문단
//!
//! 문서의 문단과 그 내용을 정의합니다.

use crate::control::Control;
use primitive::{BreakType, FieldType, HwpUnit};
use primitive::{CharShapeId, ParaShapeId, StyleId};

/// 문단
#[derive(Debug, Clone, Default)]
pub struct Paragraph {
    /// 문단 모양 ID
    pub para_shape_id: Option<ParaShapeId>,

    /// 스타일 ID
    pub style_id: Option<StyleId>,

    /// 런(텍스트 조각) 목록
    pub runs: Vec<Run>,

    /// 문단 앞 나누기
    pub break_type: BreakType,

    /// 문단 인스턴스 ID (문서 내 고유)
    pub instance_id: Option<u32>,

    /// 줄 세그먼트 정보 (레이아웃 캐시)
    ///
    /// 레이아웃 엔진이 계산한 줄 배치 정보입니다.
    /// 변환 시 손실될 수 있으며, 대상 형식에서 다시 계산됩니다.
    pub line_segments: Option<Vec<LineSegment>>,

    /// 범위 태그 (책갈피, 하이퍼링크 등)
    pub range_tags: Vec<RangeTag>,
}

impl Paragraph {
    /// 빈 문단 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 텍스트로 문단 생성
    pub fn with_text(text: impl Into<String>) -> Self {
        let mut para = Self::new();
        para.runs.push(Run::text(text));
        para
    }

    /// 런 추가
    pub fn add_run(&mut self, run: Run) {
        self.runs.push(run);
    }

    /// 전체 텍스트 추출
    pub fn to_plain_text(&self) -> String {
        let mut text = String::new();
        for run in &self.runs {
            for content in &run.contents {
                if let RunContent::Text(t) = content {
                    text.push_str(&t.text);
                }
            }
        }
        text
    }

    /// 문단이 비어있는지 확인
    pub fn is_empty(&self) -> bool {
        self.runs.is_empty() || self.runs.iter().all(|r| r.contents.is_empty())
    }
}

/// 런 (동일한 글자 모양을 가진 텍스트 조각)
#[derive(Debug, Clone, Default)]
pub struct Run {
    /// 글자 모양 ID
    pub char_shape_id: Option<CharShapeId>,

    /// 런 내용 목록
    pub contents: Vec<RunContent>,
}

impl Run {
    /// 빈 런 생성
    pub fn new() -> Self {
        Self::default()
    }

    /// 텍스트 런 생성
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            char_shape_id: None,
            contents: vec![RunContent::Text(Text { text: text.into() })],
        }
    }

    /// 글자 모양과 함께 텍스트 런 생성
    pub fn text_with_shape(text: impl Into<String>, char_shape_id: CharShapeId) -> Self {
        Self {
            char_shape_id: Some(char_shape_id),
            contents: vec![RunContent::Text(Text { text: text.into() })],
        }
    }

    /// 컨트롤 런 생성
    pub fn control(control: Control) -> Self {
        Self {
            char_shape_id: None,
            contents: vec![RunContent::Control(Box::new(control))],
        }
    }
}

/// 런 내용
#[derive(Debug, Clone)]
pub enum RunContent {
    /// 텍스트
    Text(Text),
    /// 탭
    Tab(TabChar),
    /// 줄 바꿈 (Shift+Enter)
    LineBreak,
    /// 하이픈
    Hyphen,
    /// 줄 바꿈 없는 공백
    NonBreakingSpace,
    /// 고정 너비 공백
    FixedWidthSpace,
    /// 컨트롤 (표, 그림 등)
    Control(Box<Control>),
    /// 필드 시작
    FieldStart(FieldStart),
    /// 필드 끝
    FieldEnd(FieldEnd),
    /// 책갈피 시작
    BookmarkStart(BookmarkStart),
    /// 책갈피 끝
    BookmarkEnd(BookmarkEnd),
    /// 글자 겹침 (Compose)
    Compose(Compose),
    /// 덧말 (Dutmal/Ruby)
    Dutmal(Dutmal),
}

/// 텍스트
#[derive(Debug, Clone)]
pub struct Text {
    /// 텍스트 내용
    pub text: String,
}

impl Text {
    /// 텍스트 생성
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

/// 탭 문자
#[derive(Debug, Clone, Default)]
pub struct TabChar {
    /// 탭 너비 (레이아웃 계산 결과)
    pub width: Option<HwpUnit>,
    /// 채움선 문자
    pub leader: Option<char>,
    /// 탭 종류 (Left/Right/Center/Decimal)
    pub tab_type: Option<primitive::TabType>,
}

/// 필드 시작
#[derive(Debug, Clone)]
pub struct FieldStart {
    /// 필드 ID
    pub id: u32,
    /// 필드 타입
    pub field_type: FieldType,
    /// 필드 명령/매개변수
    pub instruction: Option<String>,
    /// 필드 매개변수 목록 (HWPX 전용)
    pub parameters: Option<FieldParameters>,
    /// 서브 문단 목록 (HWPX 전용 - 각주/미주 등)
    pub sub_paragraphs: Option<Vec<Paragraph>>,
    /// 편집 가능 여부 (HWPX 전용)
    pub editable: bool,
    /// 변경됨 여부 (HWPX 전용)
    pub dirty: bool,
    /// Z 순서 (HWPX 전용)
    pub z_order: Option<i32>,
    /// 필드 아이디 (HWPX 전용 - instruction과 별개)
    pub field_id: Option<u32>,
}

impl Default for FieldStart {
    fn default() -> Self {
        Self {
            id: 0,
            field_type: FieldType::Unknown,
            instruction: None,
            parameters: None,
            sub_paragraphs: None,
            editable: true,
            dirty: false,
            z_order: None,
            field_id: None,
        }
    }
}

/// 필드 매개변수 목록
#[derive(Debug, Clone)]
pub struct FieldParameters {
    /// 매개변수 항목들
    pub items: Vec<FieldParameter>,
    /// 이름
    pub name: Option<String>,
}

/// 필드 매개변수 항목
#[derive(Debug, Clone)]
pub enum FieldParameter {
    /// 불리언 매개변수
    Boolean { name: Option<String>, value: bool },
    /// 정수 매개변수
    Integer { name: Option<String>, value: i64 },
    /// 실수 매개변수
    Float { name: Option<String>, value: f32 },
    /// 문자열 매개변수
    String { name: Option<String>, value: String },
    /// 리스트 매개변수 (재귀적)
    List(FieldParameters),
}

/// 필드 끝
#[derive(Debug, Clone)]
pub struct FieldEnd {
    /// 필드 ID (FieldStart와 매칭)
    pub id: u32,
}

/// 책갈피 시작
#[derive(Debug, Clone)]
pub struct BookmarkStart {
    /// 책갈피 ID
    pub id: u32,
    /// 책갈피 이름
    pub name: String,
}

/// 책갈피 끝
#[derive(Debug, Clone)]
pub struct BookmarkEnd {
    /// 책갈피 ID (BookmarkStart와 매칭)
    pub id: u32,
}

/// 줄 세그먼트 (레이아웃 캐시)
///
/// 문단 내 각 줄의 레이아웃 정보입니다.
/// 이 정보는 레이아웃 엔진이 계산한 결과이며,
/// 다른 형식으로 변환 시 재계산이 필요할 수 있습니다.
#[derive(Debug, Clone)]
pub struct LineSegment {
    /// 텍스트 시작 위치 (문자 인덱스)
    pub text_start: u32,
    /// 세로 위치
    pub vertical_position: HwpUnit,
    /// 줄 높이
    pub line_height: HwpUnit,
    /// 텍스트 높이
    pub text_height: HwpUnit,
    /// 기준선 거리
    pub baseline_distance: HwpUnit,
    /// 줄 간격
    pub line_spacing: HwpUnit,
    /// 단 시작 위치
    pub column_start: HwpUnit,
    /// 세그먼트 너비
    pub segment_width: HwpUnit,
}

/// 범위 태그
///
/// 문단 내 특정 범위를 표시하는 태그입니다.
/// 책갈피, 하이퍼링크, 변경 추적 등에 사용됩니다.
#[derive(Debug, Clone)]
pub struct RangeTag {
    /// 시작 위치 (문자 인덱스)
    pub start: u32,
    /// 끝 위치 (문자 인덱스)
    pub end: u32,
    /// 태그 종류
    pub tag_type: RangeTagType,
    /// 태그 데이터
    pub data: Option<String>,
    /// 변경 추적 정보 (TrackChange 타입인 경우)
    pub track_change_info: Option<TrackChangeInfo>,
}

/// 범위 태그 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeTagType {
    /// 책갈피
    Bookmark,
    /// 하이퍼링크
    Hyperlink,
    /// 변경 추적 - 삽입
    TrackChangeInsert,
    /// 변경 추적 - 삭제
    TrackChangeDelete,
    /// 형광펜
    Highlight,
    /// 기타
    Other(u8),
}

/// 변경 추적 정보
#[derive(Debug, Clone)]
pub struct TrackChangeInfo {
    /// 변경 추적 ID
    pub track_change_id: u32,
    /// 태그 ID (InsertBegin/DeleteBegin 등의 고유 ID)
    pub tag_id: Option<u32>,
    /// 문단 끝 여부
    pub paragraph_end: bool,
}

/// 글자 겹침 (Compose)
///
/// 여러 글자를 겹쳐서 표현하는 기능입니다.
#[derive(Debug, Clone)]
pub struct Compose {
    /// 겹침 텍스트
    pub compose_text: String,
    /// 겹침 종류 (Spread/Overlap)
    pub compose_type: Option<ComposeType>,
    /// 테두리 종류
    pub circle_type: ComposeCircleType,
    /// 테두리 내부 글자 크기 비율 (%)
    pub char_size: Option<i32>,
    /// 글자 속성 참조 목록
    pub char_shape_ids: Vec<Option<primitive::CharShapeId>>,
}

/// 글자 겹침 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposeType {
    /// 글자를 벌려 배치
    Spread,
    /// 글자를 겹쳐 배치
    Overlap,
}

/// 글자 겹침 테두리 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComposeCircleType {
    /// 문자를 그대로 사용
    Char,
    /// 원형 테두리
    #[default]
    ShapeCircle,
    /// 역원형 테두리
    ShapeReversalCircle,
    /// 사각형 테두리
    ShapeRectangle,
    /// 역사각형 테두리
    ShapeReversalRectangle,
    /// 삼각형 테두리
    ShapeTriangle,
    /// 역삼각형 테두리
    ShapeReversalTriangle,
    /// 전구/전광판형 테두리
    ShapeLight,
    /// 마름모 테두리
    ShapeRhombus,
    /// 역마름모 테두리
    ShapeReversalRhombus,
    /// 둥근 사각형 테두리
    ShapeRoundedRectangle,
    /// 빈 순환 삼각형 테두리
    ShapeEmptyCirculateTriangle,
    /// 가는 순환 삼각형 테두리
    ShapeThinCirculateTriangle,
    /// 두꺼운 순환 삼각형 테두리
    ShapeThickCirculateTriangle,
}

/// 덧말 (Dutmal/Ruby)
///
/// 주 텍스트 위/아래에 작은 텍스트를 추가하는 기능입니다.
#[derive(Debug, Clone)]
pub struct Dutmal {
    /// 주 텍스트
    pub main_text: String,
    /// 덧말 텍스트
    pub sub_text: String,
    /// 위치 (Top/Bottom)
    pub position_type: DutmalPosition,
    /// 크기 비율
    pub size_ratio: Option<u32>,
    /// 옵션
    pub option: Option<u32>,
    /// 스타일 ID 참조
    pub style_id_ref: Option<primitive::StyleId>,
    /// 정렬
    pub alignment: DutmalAlignment,
}

/// 덧말 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DutmalPosition {
    /// 위쪽 덧말
    #[default]
    Top,
    /// 아래쪽 덧말
    Bottom,
}

/// 덧말 정렬
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DutmalAlignment {
    /// 양쪽 맞춤
    Justify,
    /// 왼쪽 정렬
    Left,
    /// 오른쪽 정렬
    Right,
    /// 가운데 정렬
    #[default]
    Center,
    /// 균등 배치
    Distribute,
    /// 공백 포함 균등 배치
    DistributeSpace,
}
