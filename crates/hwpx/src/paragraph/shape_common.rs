//! [AI 생성] 도형 개체 공통 타입
//!
//! 모든 도형·미디어·폼 컨트롤이 공유하는 배치/정렬/크기 정의입니다. 기준(용지/쪽/단/문단)과 앵커 이동 규칙을 한 곳에서 정리해 텍스트 흐름과의 관계를 명시합니다. KS X 6101:2024 `paralist.xsd`.

use serde::{Deserialize, Serialize};

use crate::core::types::{Matrix, Point, SubjectIdRef};

/// [AI 생성] 너비 기준 (`sz.widthRelTo`)
///
/// 원본: `AbstractShapeObjectType/sz.widthRelTo` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WidthRelativeTo {
    /// 용지
    #[serde(rename = "PAPER")]
    Paper,
    /// 쪽
    #[serde(rename = "PAGE")]
    Page,
    /// 단
    #[serde(rename = "COLUMN")]
    Column,
    /// 문단
    #[serde(rename = "PARA")]
    Paragraph,
    /// 절대값
    #[default]
    #[serde(rename = "ABSOLUTE")]
    Absolute,
}

/// [AI 생성] 높이 기준 (`sz.heightRelTo`)
///
/// 원본: `AbstractShapeObjectType/sz.heightRelTo` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HeightRelativeTo {
    /// 용지
    #[serde(rename = "PAPER")]
    Paper,
    /// 쪽
    #[serde(rename = "PAGE")]
    Page,
    /// 절대값
    #[default]
    #[serde(rename = "ABSOLUTE")]
    Absolute,
}

/// [AI 생성] 개체 크기 (`sz` 요소)
///
/// 원본: `AbstractShapeObjectType/sz` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "sz")]
pub struct ShapeObjectSize {
    /// [AI 생성] 개체의 표시 너비 (`width` 속성)
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// [AI 생성] 너비 기준 (용지/쪽/단/문단/절대) (`widthRelTo` 속성)
    #[serde(rename = "@widthRelTo", default)]
    pub width_relative_to: WidthRelativeTo,

    /// [AI 생성] 개체의 표시 높이 (`height` 속성)
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    /// [AI 생성] 높이 기준 (용지/쪽/절대) (`heightRelTo` 속성)
    #[serde(rename = "@heightRelTo", default)]
    pub height_relative_to: HeightRelativeTo,

    /// [AI 생성] 크기 잠금 여부 (`protect` 속성)
    #[serde(rename = "@protect", default)]
    pub protect: bool,
}

/// 세로 기준
///
/// 원본: `AbstractShapeObjectType/pos.vertRelTo` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum VerticalRelativeTo {
    /// 용지
    #[default]
    #[serde(rename = "PAPER")]
    Paper,
    /// 쪽
    #[serde(rename = "PAGE")]
    Page,
    /// 문단
    #[serde(rename = "PARA")]
    Paragraph,
}

/// 가로 기준
///
/// 원본: `AbstractShapeObjectType/pos.horzRelTo` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HorizontalRelativeTo {
    /// 용지
    #[default]
    #[serde(rename = "PAPER")]
    Paper,
    /// 쪽
    #[serde(rename = "PAGE")]
    Page,
    /// 단
    #[serde(rename = "COLUMN")]
    Column,
    /// 문단
    #[serde(rename = "PARA")]
    Paragraph,
}

/// 세로 정렬
///
/// 원본: `AbstractShapeObjectType/pos.vertAlign` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ShapeVerticalAlignment {
    /// 위쪽
    #[default]
    #[serde(rename = "TOP")]
    Top,
    /// 가운데
    #[serde(rename = "CENTER")]
    Center,
    /// 아래쪽
    #[serde(rename = "BOTTOM")]
    Bottom,
    /// 안쪽
    #[serde(rename = "INSIDE")]
    Inside,
    /// 바깥쪽
    #[serde(rename = "OUTSIDE")]
    Outside,
}

/// 가로 정렬
///
/// 원본: `AbstractShapeObjectType/pos.horzAlign` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ShapeHorizontalAlignment {
    /// 왼쪽
    #[default]
    #[serde(rename = "LEFT")]
    Left,
    /// 가운데
    #[serde(rename = "CENTER")]
    Center,
    /// 오른쪽
    #[serde(rename = "RIGHT")]
    Right,
    /// 안쪽
    #[serde(rename = "INSIDE")]
    Inside,
    /// 바깥쪽
    #[serde(rename = "OUTSIDE")]
    Outside,
}

/// 개체 위치
///
/// 원본: `AbstractShapeObjectType/pos` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "pos")]
pub struct ShapeObjectPosition {
    /// [AI 생성] 줄 내에 글자처럼 배치할지 여부 (`treatAsChar` 속성)
    #[serde(rename = "@treatAsChar", default)]
    pub treat_as_character: bool,

    /// [AI 생성] 줄 간격을 재계산할지 여부 (`affectLSpacing` 속성)
    #[serde(rename = "@affectLSpacing", default)]
    pub affect_line_spacing: bool,

    /// [AI 생성] 본문 흐름과 함께 이동할지 (`flowWithText` 속성)
    #[serde(rename = "@flowWithText", default)]
    pub flow_with_text: bool,

    /// [AI 생성] 다른 개체와 겹침 허용 여부 (`allowOverlap` 속성)
    #[serde(rename = "@allowOverlap", default)]
    pub allow_overlap: bool,

    /// [AI 생성] 앵커 이동 시 개체를 고정할지 여부 (`holdAnchorAndSO` 속성)
    #[serde(rename = "@holdAnchorAndSO", default)]
    pub hold_anchor_and_shape_object: bool,

    /// 세로 기준
    ///
    /// 원본: `vertRelTo` 속성
    #[serde(rename = "@vertRelTo", skip_serializing_if = "Option::is_none")]
    pub vertical_relative_to: Option<VerticalRelativeTo>,

    /// 가로 기준
    ///
    /// 원본: `horzRelTo` 속성
    #[serde(rename = "@horzRelTo", skip_serializing_if = "Option::is_none")]
    pub horizontal_relative_to: Option<HorizontalRelativeTo>,

    /// 세로 정렬
    ///
    /// 원본: `vertAlign` 속성
    #[serde(rename = "@vertAlign", skip_serializing_if = "Option::is_none")]
    pub vertical_alignment: Option<ShapeVerticalAlignment>,

    /// 가로 정렬
    ///
    /// 원본: `horzAlign` 속성
    #[serde(rename = "@horzAlign", skip_serializing_if = "Option::is_none")]
    pub horizontal_alignment: Option<ShapeHorizontalAlignment>,

    /// 세로 오프셋
    ///
    /// 원본: `vertOffset` 속성
    #[serde(rename = "@vertOffset", default)]
    pub vertical_offset: u32,

    /// 가로 오프셋
    ///
    /// 원본: `horzOffset` 속성
    #[serde(rename = "@horzOffset", default)]
    pub horizontal_offset: u32,
}

/// 바깥 여백
///
/// 원본: `AbstractShapeObjectType/outMargin` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "outMargin")]
pub struct OutsideMargin {
    /// 왼쪽
    #[serde(rename = "@left", default)]
    pub left: u32,
    /// 오른쪽
    #[serde(rename = "@right", default)]
    pub right: u32,
    /// 위쪽
    #[serde(rename = "@top", default)]
    pub top: u32,
    /// 아래쪽
    #[serde(rename = "@bottom", default)]
    pub bottom: u32,
}

/// 안쪽 여백
///
/// 원본: `InsideMarginType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "inMargin")]
pub struct InsideMargin {
    /// 왼쪽
    #[serde(rename = "@left", default)]
    pub left: u32,
    /// 오른쪽
    #[serde(rename = "@right", default)]
    pub right: u32,
    /// 위쪽
    #[serde(rename = "@top", default)]
    pub top: u32,
    /// 아래쪽
    #[serde(rename = "@bottom", default)]
    pub bottom: u32,
}

/// 캡션 측면
///
/// 원본: `AbstractShapeObjectType/caption.side` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CaptionSide {
    /// 왼쪽
    #[default]
    #[serde(rename = "LEFT")]
    Left,
    /// 오른쪽
    #[serde(rename = "RIGHT")]
    Right,
    /// 위쪽
    #[serde(rename = "TOP")]
    Top,
    /// 아래쪽
    #[serde(rename = "BOTTOM")]
    Bottom,
}

/// 개체 번호 매기기 유형
///
/// 원본: `AbstractShapeObjectType.numberingType` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ShapeNumberingType {
    /// 없음
    #[default]
    #[serde(rename = "NONE")]
    None,
    /// 그림
    #[serde(rename = "PICTURE")]
    Picture,
    /// 표
    #[serde(rename = "TABLE")]
    Table,
    /// 수식
    #[serde(rename = "EQUATION")]
    Equation,
}

/// 텍스트 감싸기
///
/// 원본: `AbstractShapeObjectType.textWrap` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TextWrapMode {
    /// 사각형
    #[serde(rename = "SQUARE")]
    Square,
    /// 빽빽하게
    #[serde(rename = "TIGHT")]
    Tight,
    /// 투과
    #[serde(rename = "THROUGH")]
    Through,
    /// 위 아래
    #[default]
    #[serde(rename = "TOP_AND_BOTTOM")]
    TopAndBottom,
    /// 글 뒤로
    #[serde(rename = "BEHIND_TEXT")]
    BehindText,
    /// 글 앞으로
    #[serde(rename = "IN_FRONT_OF_TEXT")]
    InFrontOfText,
}

/// 텍스트 흐름
///
/// 원본: `AbstractShapeObjectType.textFlow` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TextFlowMode {
    /// 양쪽
    #[default]
    #[serde(rename = "BOTH_SIDES")]
    BothSides,
    /// 왼쪽만
    #[serde(rename = "LEFT_ONLY")]
    LeftOnly,
    /// 오른쪽만
    #[serde(rename = "RIGHT_ONLY")]
    RightOnly,
    /// 가장 큰 쪽만
    #[serde(rename = "LARGEST_ONLY")]
    LargestOnly,
}

/// 개체 오프셋
///
/// 원본: `AbstractShapeComponentType/offset` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "offset")]
pub struct ShapeComponentOffset {
    /// X 오프셋
    ///
    /// 원본: `x` 속성
    #[serde(rename = "@x", default)]
    pub x: u32,

    /// Y 오프셋
    ///
    /// 원본: `y` 속성
    #[serde(rename = "@y", default)]
    pub y: u32,
}

/// 원본 크기
///
/// 원본: `AbstractShapeComponentType/orgSz` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "orgSz")]
pub struct OriginalSize {
    /// 너비
    ///
    /// 원본: `width` 속성
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// 높이
    ///
    /// 원본: `height` 속성
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

/// 현재 크기
///
/// 원본: `AbstractShapeComponentType/curSz` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "curSz")]
pub struct CurrentSize {
    /// 너비
    ///
    /// 원본: `width` 속성
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// 높이
    ///
    /// 원본: `height` 속성
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

/// 뒤집기
///
/// 원본: `AbstractShapeComponentType/flip` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "flip")]
pub struct Flip {
    /// 수평 뒤집기
    ///
    /// 원본: `horizontal` 속성
    #[serde(rename = "@horizontal", default)]
    pub horizontal: bool,

    /// 수직 뒤집기
    ///
    /// 원본: `vertical` 속성
    #[serde(rename = "@vertical", default)]
    pub vertical: bool,
}

/// 회전 정보
///
/// 원본: `AbstractShapeComponentType/rotationInfo` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "rotationInfo")]
pub struct RotationInfo {
    /// 회전 각도
    ///
    /// 원본: `angle` 속성
    #[serde(rename = "@angle", default)]
    pub angle: i32,

    /// 중심 X
    ///
    /// 원본: `centerX` 속성
    #[serde(rename = "@centerX", skip_serializing_if = "Option::is_none")]
    pub center_x: Option<u32>,

    /// 중심 Y
    ///
    /// 원본: `centerY` 속성
    #[serde(rename = "@centerY", skip_serializing_if = "Option::is_none")]
    pub center_y: Option<u32>,

    /// 이미지 회전
    ///
    /// 원본: `rotateimage` 속성
    #[serde(rename = "@rotateimage", skip_serializing_if = "Option::is_none")]
    pub rotate_image: Option<bool>,
}

/// 이미지 클리핑
///
/// 원본: `PictureType/imgClip` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "imgClip")]
pub struct ImageClip {
    /// 왼쪽
    ///
    /// 원본: `left` 속성
    #[serde(rename = "@left", skip_serializing_if = "Option::is_none")]
    pub left: Option<i32>,

    /// 오른쪽
    ///
    /// 원본: `right` 속성
    #[serde(rename = "@right", skip_serializing_if = "Option::is_none")]
    pub right: Option<i32>,

    /// 위쪽
    ///
    /// 원본: `top` 속성
    #[serde(rename = "@top", skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,

    /// 아래쪽
    ///
    /// 원본: `bottom` 속성
    #[serde(rename = "@bottom", skip_serializing_if = "Option::is_none")]
    pub bottom: Option<i32>,
}

/// 이미지 크기
///
/// 원본: `PictureType/imgDim` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "imgDim")]
pub struct ImageDimension {
    /// 너비
    ///
    /// 원본: `dimwidth` 속성
    #[serde(rename = "@dimwidth", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// 높이
    ///
    /// 원본: `dimheight` 속성
    #[serde(rename = "@dimheight", skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

/// 이미지 사각형
///
/// 원본: `PictureType/imgRect` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "imgRect")]
pub struct ImageRectangle {
    /// 점 0
    #[serde(rename = "pt0")]
    pub point0: Point,
    /// 점 1
    #[serde(rename = "pt1")]
    pub point1: Point,
    /// 점 2
    #[serde(rename = "pt2")]
    pub point2: Point,
    /// 점 3
    #[serde(rename = "pt3")]
    pub point3: Point,
}

/// 호 유형
///
/// 원본: `EllipseType.arcType` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ArcStyle {
    /// 일반
    #[default]
    #[serde(rename = "NORMAL")]
    Normal,
    /// 파이
    #[serde(rename = "PIE")]
    Pie,
    /// 현
    #[serde(rename = "CHORD")]
    Chord,
}

/// 곡선 세그먼트 유형
///
/// 원본: `CurveType/seg.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CurveSegmentType {
    /// 직선
    #[serde(rename = "LINE")]
    Line,
    /// 곡선
    #[default]
    #[serde(rename = "CURVE")]
    Curve,
}

/// 곡선 세그먼트
///
/// 원본: `CurveType/seg` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "seg")]
pub struct CurveSegment {
    /// 유형
    ///
    /// 원본: `type` 속성
    #[serde(rename = "@type", default)]
    pub segment_type: CurveSegmentType,

    /// X1
    #[serde(rename = "@x1", skip_serializing_if = "Option::is_none")]
    pub x1: Option<i32>,

    /// Y1
    #[serde(rename = "@y1", skip_serializing_if = "Option::is_none")]
    pub y1: Option<i32>,

    /// X2
    #[serde(rename = "@x2", skip_serializing_if = "Option::is_none")]
    pub x2: Option<i32>,

    /// Y2
    #[serde(rename = "@y2", skip_serializing_if = "Option::is_none")]
    pub y2: Option<i32>,
}

/// 연결선 유형
///
/// 원본: `ConnectLineType.type` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ConnectLineStyle {
    /// 직선 화살표 없음
    #[default]
    #[serde(rename = "STRAIGHT_NOARROW")]
    StraightNoArrow,
    /// 직선 단방향
    #[serde(rename = "STRAIGHT_ONEWAY")]
    StraightOneWay,
    /// 직선 양방향
    #[serde(rename = "STRAIGHT_BOTH")]
    StraightBoth,
    /// 꺾은선 화살표 없음
    #[serde(rename = "STROKE_NOARROW")]
    StrokeNoArrow,
    /// 꺾은선 단방향
    #[serde(rename = "STROKE_ONEWAY")]
    StrokeOneWay,
    /// 꺾은선 양방향
    #[serde(rename = "STROKE_BOTH")]
    StrokeBoth,
    /// 호 화살표 없음
    #[serde(rename = "ARC_NOARROW")]
    ArcNoArrow,
    /// 호 단방향
    #[serde(rename = "ARC_ONEWAY")]
    ArcOneWay,
    /// 호 양방향
    #[serde(rename = "ARC_BOTH")]
    ArcBoth,
}

/// 연결점
///
/// 원본: `ConnectPointType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ConnectPoint {
    /// X 좌표
    #[serde(rename = "@x", skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,

    /// Y 좌표
    #[serde(rename = "@y", skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,

    /// 대상 아이디 참조
    #[serde(rename = "@subjectIDRef", skip_serializing_if = "Option::is_none")]
    pub subject_id_reference: Option<SubjectIdRef>,

    /// 대상 인덱스
    #[serde(rename = "@subjectIdx", skip_serializing_if = "Option::is_none")]
    pub subject_index: Option<u32>,
}

/// 연결선 제어점
///
/// 원본: `ConnectControlPointType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "point")]
pub struct ConnectControlPoint {
    /// X 좌표
    #[serde(rename = "@x", skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,

    /// Y 좌표
    #[serde(rename = "@y", skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,

    /// 유형
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub point_type: Option<u32>,
}

/// OLE 개체 유형
///
/// 원본: `OLEType.objectType` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OleObjectKind {
    /// 알 수 없음
    #[default]
    #[serde(rename = "UNKNOWN")]
    Unknown,
    /// 포함
    #[serde(rename = "EMBEDDED")]
    Embedded,
    /// 연결
    #[serde(rename = "LINK")]
    Link,
    /// 정적
    #[serde(rename = "STATIC")]
    Static,
    /// 수식
    #[serde(rename = "EQUATION")]
    Equation,
}

/// OLE 표시 형식
///
/// 원본: `OLEType.drawAspect` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OleDrawAspect {
    /// 내용
    #[default]
    #[serde(rename = "CONTENT")]
    Content,
    /// 썸네일
    #[serde(rename = "THUMB_NAIL")]
    Thumbnail,
    /// 아이콘
    #[serde(rename = "ICON")]
    Icon,
    /// 문서 인쇄
    #[serde(rename = "DOC_PRINT")]
    DocumentPrint,
}

/// 수식 줄 모드
///
/// 원본: `EquationType.lineMode` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EquationLineMode {
    /// 줄 단위
    #[serde(rename = "LINE")]
    Line,
    /// 글자 단위
    #[default]
    #[serde(rename = "CHAR")]
    Character,
}

/// 렌더링 변환 행렬 쌍 (스케일, 회전)
///
/// 원본: `AbstractShapeComponentType/renderingInfo` 요소의 익명 타입 내 시퀀스
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RenderingMatrixPair {
    /// 스케일 행렬
    ///
    /// 원본: `scaMatrix` 요소
    #[serde(rename = "scaMatrix")]
    pub scale_matrix: Matrix,

    /// 회전 행렬
    ///
    /// 원본: `rotMatrix` 요소
    #[serde(rename = "rotMatrix")]
    pub rotation_matrix: Matrix,
}

/// 렌더링 정보
///
/// 원본: `AbstractShapeComponentType/renderingInfo` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "renderingInfo")]
pub struct RenderingInfo {
    /// 변환 행렬
    ///
    /// 원본: `transMatrix` 요소
    #[serde(rename = "transMatrix")]
    pub transform_matrix: Matrix,

    /// 스케일/회전 행렬 쌍 목록
    ///
    /// 원본: `scaMatrix`, `rotMatrix` 요소 시퀀스
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub matrix_pairs: Vec<RenderingMatrixPair>,
}
