//! 도형 관련 타입
//!
//! 그리기 개체의 종류와 속성을 정의합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 도형 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ShapeType {
    /// 선
    #[default]
    Line,
    /// 사각형
    Rectangle,
    /// 타원
    Ellipse,
    /// 호
    Arc,
    /// 다각형
    Polygon,
    /// 곡선
    Curve,
    /// 연결선
    Connector,
    /// 글맵시
    TextArt,
    /// 그룹
    Group,
    /// OLE
    Ole,
    /// 컨테이너
    Container,
}

/// 호 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

/// 연결선 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConnectorType {
    /// 직선
    #[default]
    Straight,
    /// 꺾인선
    Elbow,
    /// 곡선
    Curved,
    /// 수직/수평선 (HWPX 전용)
    VerticalHorizontal,
    /// 수평/수직선 (HWPX 전용)
    HorizontalVertical,
}

/// 곡선 제어점 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CurvePointType {
    /// 직선
    #[default]
    Line,
    /// 곡선
    Curve,
}

/// 폼 객체 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FormObjectType {
    /// 버튼
    #[default]
    Button,
    /// 체크 박스
    CheckBox,
    /// 라디오 버튼
    RadioButton,
    /// 콤보 박스
    ComboBox,
    /// 목록 상자
    ListBox,
    /// 텍스트 입력
    Edit,
    /// 스크롤 바
    ScrollBar,
    /// 서명 (HWPX 전용)
    Signature,
}

/// 텍스트 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextBoxDirection {
    /// 가로쓰기
    #[default]
    Horizontal,
    /// 세로쓰기 (왼쪽으로)
    VerticalLeft,
    /// 세로쓰기 (오른쪽으로)
    VerticalRight,
}

/// 글맵시 도형 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextArtShape {
    /// 기본
    #[default]
    Plain,
    /// 위로 기울어진 삼각형
    TopTriangle,
    /// 아래로 기울어진 삼각형
    BottomTriangle,
    /// 역삼각형
    InverseTriangle,
    /// 역삼각형 (아래)
    InverseTriangleBottom,
    /// 마름모
    Diamond,
    /// 볼록
    Convex,
    /// 오목
    Concave,
    /// 위 아치
    ArchUp,
    /// 아래 아치
    ArchDown,
    /// 원
    Circle,
    /// 버튼
    Button,
    /// 물결
    Wave,
    /// 계단
    Stairs,
    /// 평행사변형
    Parallelogram,
    /// 평행사변형 (반전)
    ParallelogramInverse,
    /// 기타
    Custom,
}

/// 비디오 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VideoType {
    /// 임베디드 (문서에 포함)
    #[default]
    Embedded,
    /// 링크 (외부 URL)
    Linked,
    /// 유튜브
    YouTube,
    /// 웹 URL (HWPX 전용)
    Web,
}

/// 차트 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ChartType {
    /// 막대형
    #[default]
    Bar,
    /// 세로 막대형
    Column,
    /// 꺾은선형
    Line,
    /// 원형
    Pie,
    /// 영역형
    Area,
    /// 분산형
    Scatter,
    /// 버블형
    Bubble,
    /// 방사형
    Radar,
    /// 주식형
    Stock,
    /// 표면형
    Surface,
    /// 도넛형
    Doughnut,
}

/// 수식 형식
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EquationFormat {
    /// 한글 수식 스크립트
    #[default]
    HwpScript,
    /// MathML
    MathML,
    /// LaTeX
    LaTeX,
}

/// 조판 부호 (덧말/각주 등)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ComposeType {
    /// 글자 겹침
    #[default]
    CharOverlap,
    /// 덧말
    Ruby,
    /// 할주
    Divisor,
}

/// 원 문자 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CircleType {
    /// 글자 원
    #[default]
    Character,
    /// 글자 타원
    CharacterEllipse,
    /// 글자 사각형
    CharacterRectangle,
    /// 글자 삼각형
    CharacterTriangle,
}

/// 덧말 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DutmalPosition {
    /// 위
    #[default]
    Top,
    /// 아래
    Bottom,
    /// 가운데
    Center,
}

/// 하이퍼링크 대상 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HyperlinkTarget {
    /// URL
    #[default]
    Url,
    /// 책갈피
    Bookmark,
    /// 파일
    File,
    /// 메일
    Email,
    /// 현재 문서
    CurrentDocument,
}
