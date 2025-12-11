//! 도형
//!
//! 문서 내 도형(선, 사각형, 타원 등)을 정의합니다.

use crate::border_fill::Fill;
use primitive::Color;
use crate::control::ObjectCommon;
use crate::paragraph::Paragraph;
use primitive::{ArrowSize, ArrowType, HwpUnit, Insets, LineCap, LineOutlineStyle, LineType, Point, TextDirection, VerticalAlignment};

/// 2D 변환 행렬 (3x3 affine transform의 6개 값)
///
/// 행렬 형태:
/// | e1  e2  e5 |
/// | e3  e4  e6 |
/// | 0   0   1  |
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformMatrix {
    pub e1: f64,
    pub e2: f64,
    pub e3: f64,
    pub e4: f64,
    pub e5: f64,
    pub e6: f64,
}

impl Default for TransformMatrix {
    fn default() -> Self {
        // 단위 행렬 (identity matrix)
        Self {
            e1: 1.0,
            e2: 0.0,
            e3: 0.0,
            e4: 1.0,
            e5: 0.0,
            e6: 0.0,
        }
    }
}

impl TransformMatrix {
    /// 단위 행렬 생성
    pub fn identity() -> Self {
        Self::default()
    }

    /// HWP 행렬 벡터에서 변환 (6개 또는 그 이상의 값)
    pub fn from_hwp_matrix(matrix: &[f64]) -> Option<Self> {
        if matrix.len() >= 6 {
            Some(Self {
                e1: matrix[0],
                e2: matrix[1],
                e3: matrix[2],
                e4: matrix[3],
                e5: matrix[4],
                e6: matrix[5],
            })
        } else {
            None
        }
    }

    /// HWPX Matrix 타입으로 변환
    pub fn to_hwpx_values(&self) -> (f32, f32, f32, f32, f32, f32) {
        (
            self.e1 as f32,
            self.e2 as f32,
            self.e3 as f32,
            self.e4 as f32,
            self.e5 as f32,
            self.e6 as f32,
        )
    }
}

/// 도형
#[derive(Debug, Clone)]
pub struct Shape {
    /// 공통 속성
    pub common: ObjectCommon,
    /// 도형 종류
    pub shape_type: ShapeType,
    /// 선 스타일
    pub line: LineStyle,
    /// 채우기
    pub fill: Fill,
    /// 그림자
    pub shadow: Option<ShapeShadow>,
    /// 회전 각도 (도)
    pub rotation: f64,
    /// 내부 텍스트
    pub text: Option<ShapeText>,
    /// 변환 행렬 (translation matrix)
    pub translation_matrix: Option<TransformMatrix>,
    /// 크기 조정 행렬 (scale matrix)
    pub scale_matrix: Option<TransformMatrix>,
    /// 회전 행렬 (rotation matrix)
    pub rotation_matrix: Option<TransformMatrix>,
}

impl Shape {
    /// 도형 생성
    pub fn new(shape_type: ShapeType) -> Self {
        Self {
            common: ObjectCommon::default(),
            shape_type,
            line: LineStyle::default(),
            fill: Fill::None,
            shadow: None,
            rotation: 0.0,
            text: None,
            translation_matrix: None,
            scale_matrix: None,
            rotation_matrix: None,
        }
    }
}

/// 도형 종류
#[derive(Debug, Clone)]
pub enum ShapeType {
    /// 선
    Line(LineShape),
    /// 사각형
    Rectangle(RectangleShape),
    /// 타원
    Ellipse(EllipseShape),
    /// 호
    Arc(ArcShape),
    /// 다각형
    Polygon(PolygonShape),
    /// 곡선
    Curve(CurveShape),
    /// 연결선
    Connector(ConnectorShape),
    /// 그룹
    Group(Vec<Shape>),
}

/// 선
#[derive(Debug, Clone, Default)]
pub struct LineShape {
    /// 시작점
    pub start: Point,
    /// 끝점
    pub end: Point,
    /// 시작 화살표
    pub start_arrow: Arrow,
    /// 끝 화살표
    pub end_arrow: Arrow,
}

/// 화살표
#[derive(Debug, Clone, Default)]
pub struct Arrow {
    /// 화살표 종류
    pub arrow_type: ArrowType,
    /// 화살표 크기
    pub size: ArrowSize,
    /// 채움 여부
    pub filled: bool,
}

/// 사각형
#[derive(Debug, Clone, Default)]
pub struct RectangleShape {
    /// 모서리 반지름 (둥근 모서리)
    pub corner_radius: HwpUnit,
}

/// 타원
#[derive(Debug, Clone, Default)]
pub struct EllipseShape {
    /// 호 종류 (전체, 호, 부채꼴, 활꼴)
    pub arc_type: ArcType,
    /// 시작 각도 (도)
    pub start_angle: f64,
    /// 끝 각도 (도)
    pub end_angle: f64,
}

/// 호 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArcType {
    /// 전체 (타원)
    #[default]
    Full,
    /// 호
    Arc,
    /// 부채꼴
    Pie,
    /// 활꼴
    Chord,
}

/// 호
#[derive(Debug, Clone, Default)]
pub struct ArcShape {
    /// 호 종류
    pub arc_type: ArcType,
    /// 시작 각도 (도)
    pub start_angle: f64,
    /// 끝 각도 (도)
    pub end_angle: f64,
}

/// 다각형
#[derive(Debug, Clone, Default)]
pub struct PolygonShape {
    /// 꼭짓점 목록
    pub points: Vec<Point>,
}

/// 곡선
#[derive(Debug, Clone, Default)]
pub struct CurveShape {
    /// 제어점 목록 (베지어 곡선)
    pub points: Vec<CurvePoint>,
    /// 닫힌 곡선 여부
    pub closed: bool,
}

/// 곡선 제어점
#[derive(Debug, Clone)]
pub struct CurvePoint {
    /// 점 좌표
    pub point: Point,
    /// 점 종류
    pub point_type: CurvePointType,
}

/// 곡선 점 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurvePointType {
    /// 일반 점
    #[default]
    Normal,
    /// 제어점 1 (이전 점의 나가는 핸들)
    Control1,
    /// 제어점 2 (현재 점의 들어오는 핸들)
    Control2,
}

/// 연결선
#[derive(Debug, Clone, Default)]
pub struct ConnectorShape {
    /// 연결선 종류
    pub connector_type: ConnectorType,
    /// 시작점
    pub start: ConnectorPoint,
    /// 끝점
    pub end: ConnectorPoint,
    /// 시작 화살표
    pub start_arrow: Arrow,
    /// 끝 화살표
    pub end_arrow: Arrow,
    /// 제어점 목록 (HWPX 전용: 연결선 꺾임/곡률 제어)
    pub control_points: Vec<CurvePoint>,
}

/// 연결선 연결점 (HWPX 전용 subject 참조 포함)
#[derive(Debug, Clone, Default)]
pub struct ConnectorPoint {
    /// 좌표
    pub point: Point,
    /// 대상 개체 ID 참조 (HWPX 전용)
    pub subject_id_ref: Option<u32>,
    /// 대상 개체 연결 인덱스 (HWPX 전용)
    pub subject_index: Option<u32>,
}

/// 연결선 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectorType {
    /// 직선
    #[default]
    Straight,
    /// 꺾인선
    Elbow,
    /// 곡선
    Curved,
}

/// 선 스타일
#[derive(Debug, Clone)]
pub struct LineStyle {
    /// 선 종류
    pub line_type: LineType,
    /// 선 두께
    pub width: HwpUnit,
    /// 선 색상
    pub color: Color,
    /// 선 끝 모양
    pub cap: LineCap,
    /// 외곽선 스타일 (HWPX 전용)
    pub outline_style: LineOutlineStyle,
    /// 투명도 (HWPX 전용, 0.0 ~ 1.0)
    pub alpha: Option<f32>,
}

impl Default for LineStyle {
    fn default() -> Self {
        Self {
            line_type: LineType::Solid,
            width: HwpUnit::new(10), // 0.1pt
            color: Color::BLACK,
            cap: LineCap::Flat,
            outline_style: LineOutlineStyle::Normal,
            alpha: None,
        }
    }
}

/// 도형 그림자
#[derive(Debug, Clone)]
pub struct ShapeShadow {
    /// 그림자 색상
    pub color: Color,
    /// 가로 오프셋
    pub offset_x: HwpUnit,
    /// 세로 오프셋
    pub offset_y: HwpUnit,
    /// 투명도 (0.0 ~ 1.0)
    pub alpha: f64,
    /// 흐림 효과 (blur radius)
    pub blur: Option<f32>,
    /// 방향 (0~360도)
    pub direction: Option<f32>,
    /// 거리
    pub distance: Option<HwpUnit>,
}

/// 도형 내부 텍스트
#[derive(Debug, Clone)]
pub struct ShapeText {
    /// 문단 목록
    pub paragraphs: Vec<Paragraph>,
    /// 안쪽 여백
    pub padding: Insets,
    /// 세로 정렬
    pub vertical_alignment: VerticalAlignment,
    /// 텍스트 방향
    pub text_direction: TextDirection,
    /// 편집 가능 여부
    pub editable: bool,
}
