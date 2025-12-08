//! [AI 생성] 글맵시/덧말/겹침 텍스트 도형
//!
//! 워드아트에 해당하는 장식 텍스트를 정의합니다. 텍스트 자체의 도형(곡률·리본·실린더), 그림자/선/채우기, 글꼴 유형까지 한 번에 묶어 문단과 별도로 배치합니다. KS X 6101:2024 `paralist.xsd` 기반.

use serde::{Deserialize, Serialize};

use super::drawing::DrawText;
use super::line_shape::LineShape;
use super::shadow::ShapeShadow;
use super::shape_common::{
    CurrentSize, Flip, OriginalSize, OutsideMargin, RenderingInfo, RotationInfo,
    ShapeComponentOffset, ShapeNumberingType, TextFlowMode, TextWrapMode,
};
use super::shape_common::{ShapeObjectPosition, ShapeObjectSize};
use super::table::Caption;
use crate::core::types::{CharShapeIdRef, FillBrush, MetaTag, Point, StyleIdRef};

/// [AI 생성] 글맵시 글꼴 종류 (TTF/HTF 구분)
///
/// 원본: `textart/textartPr.fontType` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TextArtFontType {
    /// [AI 생성] TTF
    #[default]
    #[serde(rename = "TTF")]
    Ttf,
    /// [AI 생성] HTF
    #[serde(rename = "HTF")]
    Htf,
}

/// [AI 생성] 글맵시 모양
///
/// 원본: `textart/textartPr.textShape` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextArtShape {
    /// [AI 생성] 평행사변형
    #[serde(rename = "PARALLELOGRAM")]
    Parallelogram,
    /// [AI 생성] 역평행사변형
    #[serde(rename = "INVERTED_PARALLELOGRAM")]
    InvertedParallelogram,
    /// [AI 생성] 역상향 계단형
    #[serde(rename = "INVERTED_UPWARD_CASCADE")]
    InvertedUpwardCascade,
    /// [AI 생성] 역하향 계단형
    #[serde(rename = "INVERTED_DOWNWARD_CASCADE")]
    InvertedDownwardCascade,
    /// [AI 생성] 상향 계단형
    #[serde(rename = "UPWARD_CASCADE")]
    UpwardCascade,
    /// [AI 생성] 하향 계단형
    #[serde(rename = "DOWNWARD_CASCADE")]
    DownwardCascade,
    /// [AI 생성] 우측으로 좁아짐
    #[serde(rename = "REDUCE_RIGHT")]
    ReduceRight,
    /// [AI 생성] 좌측으로 좁아짐
    #[serde(rename = "REDUCE_LEFT")]
    ReduceLeft,
    /// [AI 생성] 이등변 사다리꼴
    #[serde(rename = "ISOSCELES_TRAPEZOID")]
    IsoscelesTrapezoid,
    /// [AI 생성] 역 이등변 사다리꼴
    #[serde(rename = "INVERTED_ISOSCELES_TRAPEZOID")]
    InvertedIsoscelesTrapezoid,
    /// [AI 생성] 상단 리본 사각형
    #[serde(rename = "TOP_RIBBON_RECTANGLE")]
    TopRibbonRectangle,
    /// [AI 생성] 하단 리본 사각형
    #[serde(rename = "BOTTOM_RIBBON_RECTANGLE")]
    BottomRibbonRectangle,
    /// [AI 생성] V자(체브론) 아래 방향
    #[serde(rename = "CHEVRON_DOWN")]
    ChevronDown,
    /// [AI 생성] V자(체브론)
    #[serde(rename = "CHEVRON")]
    Chevron,
    /// [AI 생성] 나비넥타이 모양
    #[serde(rename = "BOW_TIE")]
    BowTie,
    /// [AI 생성] 육각형
    #[serde(rename = "HEXAGON")]
    Hexagon,
    /// [AI 생성] 물결 1
    #[serde(rename = "WAVE1")]
    Wave1,
    /// [AI 생성] 물결 2
    #[serde(rename = "WAVE2")]
    Wave2,
    /// [AI 생성] 물결 3
    #[serde(rename = "WAVE3")]
    Wave3,
    /// [AI 생성] 물결 4
    #[serde(rename = "WAVE4")]
    Wave4,
    /// [AI 생성] 좌측 기울어진 실린더
    #[serde(rename = "LEFT_TILT_CYLINDER")]
    LeftTiltCylinder,
    /// [AI 생성] 우측 기울어진 실린더
    #[serde(rename = "RIGHT_TILT_CYLINDER")]
    RightTiltCylinder,
    /// [AI 생성] 하단이 넓은 실린더
    #[serde(rename = "BOTTOM_WIDE_CYLINDER")]
    BottomWideCylinder,
    /// [AI 생성] 상단이 넓은 실린더
    #[serde(rename = "TOP_WIDE_CYLINDER")]
    TopWideCylinder,
    /// [AI 생성] 가는 곡선 위 1
    #[serde(rename = "THIN_CURVE_UP1")]
    ThinCurveUp1,
    /// [AI 생성] 가는 곡선 위 2
    #[serde(rename = "THIN_CURVE_UP2")]
    ThinCurveUp2,
    /// [AI 생성] 가는 곡선 아래 1
    #[serde(rename = "THIN_CURVE_DOWN1")]
    ThinCurveDown1,
    /// [AI 생성] 가는 곡선 아래 2
    #[serde(rename = "THIN_CURVE_DOWN2")]
    ThinCurveDown2,
    /// [AI 생성] 역 모서리형(손톱)
    #[serde(rename = "INVERSED_FINGERNAIL")]
    InversedFingernail,
    /// [AI 생성] 모서리형(손톱)
    #[serde(rename = "FINGERNAIL")]
    Fingernail,
    /// [AI 생성] 은행잎 1
    #[serde(rename = "GINKO_LEAF1")]
    GinkoLeaf1,
    /// [AI 생성] 은행잎 2
    #[serde(rename = "GINKO_LEAF2")]
    GinkoLeaf2,
    /// [AI 생성] 우측 볼록
    #[serde(rename = "INFLATE_RIGHT")]
    InflateRight,
    /// [AI 생성] 좌측 볼록
    #[serde(rename = "INFLATE_LEFT")]
    InflateLeft,
    /// [AI 생성] 상단 볼록
    #[serde(rename = "INFLATE_UP_CONVEX")]
    InflateUpConvex,
    /// [AI 생성] 하단 볼록
    #[serde(rename = "INFLATE_BOTTOM_CONVEX")]
    InflateBottomConvex,
    /// [AI 생성] 상단 오목
    #[serde(rename = "DEFLATE_TOP")]
    DeflateTop,
    /// [AI 생성] 하단 오목
    #[serde(rename = "DEFLATE_BOTTOM")]
    DeflateBottom,
    /// [AI 생성] 전체 오목
    #[serde(rename = "DEFLATE")]
    Deflate,
    /// [AI 생성] 전체 볼록
    #[serde(rename = "INFLATE")]
    Inflate,
    /// [AI 생성] 상단 볼록(대)
    #[serde(rename = "INFLATE_TOP")]
    InflateTop,
    /// [AI 생성] 하단 볼록(대)
    #[serde(rename = "INFLATE_BOTTOM")]
    InflateBottom,
    /// [AI 생성] 사각형
    #[serde(rename = "RECTANGLE")]
    Rectangle,
    /// [AI 생성] 좌측 실린더
    #[serde(rename = "LEFT_CYLINDER")]
    LeftCylinder,
    /// [AI 생성] 실린더
    #[serde(rename = "CYLINDER")]
    Cylinder,
    /// [AI 생성] 우측 실린더
    #[serde(rename = "RIGHT_CYLINDER")]
    RightCylinder,
    /// [AI 생성] 원형
    #[serde(rename = "CIRCLE")]
    Circle,
    /// [AI 생성] 아래로 굽은 곡선
    #[serde(rename = "CURVE_DOWN")]
    CurveDown,
    /// [AI 생성] 위로 아치형
    #[serde(rename = "ARCH_UP")]
    ArchUp,
    /// [AI 생성] 아래로 아치형
    #[serde(rename = "ARCH_DOWN")]
    ArchDown,
    /// [AI 생성] 단선 원형 1
    #[serde(rename = "SINGLE_LINE_CIRCLE1")]
    SingleLineCircle1,
    /// [AI 생성] 단선 원형 2
    #[serde(rename = "SINGLE_LINE_CIRCLE2")]
    SingleLineCircle2,
    /// [AI 생성] 삼선 원형 1
    #[serde(rename = "TRIPLE_LINE_CIRCLE1")]
    TripleLineCircle1,
    /// [AI 생성] 삼선 원형 2
    #[serde(rename = "TRIPLE_LINE_CIRCLE2")]
    TripleLineCircle2,
    /// [AI 생성] 복선 원형
    #[serde(rename = "DOUBLE_LINE_CIRCLE")]
    DoubleLineCircle,
}

/// [AI 생성] 글맵시 정렬
///
/// 원본: `textart/textartPr.align` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TextArtAlignment {
    #[default]
    /// [AI 생성] 왼쪽 정렬
    #[serde(rename = "LEFT")]
    Left,
    /// [AI 생성] 오른쪽 정렬
    #[serde(rename = "RIGHT")]
    Right,
    /// [AI 생성] 가운데 정렬
    #[serde(rename = "CENTER")]
    Center,
    /// [AI 생성] 양쪽 맞춤
    #[serde(rename = "FULL")]
    Full,
    /// [AI 생성] 표 스타일 정렬
    #[serde(rename = "TABLE")]
    Table,
}

/// [AI 생성] 글맵시 속성
///
/// 원본: `textart/textartPr` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "textartPr")]
pub struct TextArtProperties {
    /// [AI 생성] 그림자 (텍스트 그림자 스타일)
    #[serde(rename = "shadow")]
    pub shadow: ShapeShadow,

    /// [AI 생성] 글꼴 이름 (`fontName`)
    #[serde(rename = "@fontName", skip_serializing_if = "Option::is_none")]
    pub font_name: Option<String>,

    /// [AI 생성] 글꼴 스타일 (`fontStyle`, REGULAR/BOLD 등)
    #[serde(rename = "@fontStyle", default = "default_font_style")]
    pub font_style: String,

    /// [AI 생성] 글꼴 종류 (`fontType`, TTF/HTF)
    #[serde(rename = "@fontType", default)]
    pub font_type: TextArtFontType,

    /// [AI 생성] 텍스트 모양 (`textShape`, 배치 곡률/도형)
    #[serde(rename = "@textShape", skip_serializing_if = "Option::is_none")]
    pub text_shape: Option<TextArtShape>,

    /// [AI 생성] 줄 간격 (50-500, 기본 120)
    #[serde(rename = "@lineSpacing", default = "default_line_spacing")]
    pub line_spacing: u32,

    /// [AI 생성] 자간 (50-500, 기본 100)
    #[serde(rename = "@charSpacing", default = "default_char_spacing")]
    pub char_spacing: u32,

    /// [AI 생성] 정렬 (`align`, 글꼴 배치 기준)
    #[serde(rename = "@align", default)]
    pub alignment: TextArtAlignment,
}

fn default_font_style() -> String {
    "REGULAR".to_string()
}

fn default_line_spacing() -> u32 {
    120
}

fn default_char_spacing() -> u32 {
    100
}

/// [AI 생성] 글맵시 (워드아트 형태 도형)
///
/// 원본: `textart` 요소
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "textart")]
pub struct TextArt {
    // AbstractShapeObjectType 요소들
    /// [AI 생성] 크기 (`sz`)
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,
    /// [AI 생성] 위치 (`pos`)
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,
    /// [AI 생성] 바깥 여백 (`outMargin`)
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,
    /// [AI 생성] 캡션 (`caption`)
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
    /// [AI 생성] 도형 설명 (`shapeComment`)
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// [AI 생성] 메타 태그 (`metaTag`)
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    // AbstractShapeComponentType 요소들
    /// [AI 생성] 기준 오프셋 (`offset`)
    #[serde(rename = "offset")]
    pub offset: ShapeComponentOffset,
    /// [AI 생성] 원본 크기 (`orgSz`)
    #[serde(rename = "orgSz")]
    pub original_size: OriginalSize,
    /// [AI 생성] 현재 크기 (`curSz`)
    #[serde(rename = "curSz")]
    pub current_size: CurrentSize,
    /// [AI 생성] 대칭/뒤집기 (`flip`)
    #[serde(rename = "flip")]
    pub flip: Flip,
    /// [AI 생성] 회전 정보 (`rotationInfo`)
    #[serde(rename = "rotationInfo")]
    pub rotation_info: RotationInfo,
    /// [AI 생성] 렌더링 정보 (`renderingInfo`)
    #[serde(rename = "renderingInfo")]
    pub rendering_info: RenderingInfo,

    // AbstractDrawingObjectType 요소들
    /// [AI 생성] 선 모양 (`lineShape`)
    #[serde(rename = "lineShape")]
    pub line_shape: LineShape,
    /// [AI 생성] 채우기 브러시 (`fillBrush`)
    #[serde(rename = "fillBrush", skip_serializing_if = "Option::is_none")]
    pub fill_brush: Option<FillBrush>,
    /// [AI 생성] 텍스트 그리기 설정 (`drawText`)
    #[serde(rename = "drawText", skip_serializing_if = "Option::is_none")]
    pub draw_text: Option<DrawText>,
    /// [AI 생성] 도형 그림자 (`shadow`)
    #[serde(rename = "shadow", skip_serializing_if = "Option::is_none")]
    pub shape_shadow: Option<ShapeShadow>,

    // TextArt 전용 요소들
    /// [AI 생성] 꼭지점 0
    #[serde(rename = "pt0")]
    pub point0: Point,
    /// [AI 생성] 꼭지점 1
    #[serde(rename = "pt1")]
    pub point1: Point,
    /// [AI 생성] 꼭지점 2
    #[serde(rename = "pt2")]
    pub point2: Point,
    /// [AI 생성] 꼭지점 3
    #[serde(rename = "pt3")]
    pub point3: Point,
    /// [AI 생성] 글맵시 속성 (`textartPr`)
    #[serde(rename = "textartPr")]
    pub properties: TextArtProperties,

    // 속성들
    /// [AI 생성] 도형 ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] Z-Order (앞뒤 순서)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 번호 매김 방식
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 줄바꿈 모드
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 편집 잠금 여부
    #[serde(rename = "@lock", default)]
    pub lock: bool,
    /// [AI 생성] 하이퍼링크 대상
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// [AI 생성] 그룹 계층 깊이
    #[serde(rename = "@groupLevel", default)]
    pub group_level: u32,
    /// [AI 생성] 인스턴스 ID (복제 구분)
    #[serde(rename = "@instid", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<u32>,
    /// [AI 생성] 텍스트 내용 (속성 `text`)
    #[serde(rename = "@text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// [AI 생성] 글자 겹침 테두리 종류 (원/사각형 등 배경 형태)
///
/// 원본: `compose.circleType` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ComposeCircleType {
    /// [AI 생성] 문자를 그대로 사용
    #[serde(rename = "CHAR")]
    Char,
    #[default]
    /// [AI 생성] 원형 테두리
    #[serde(rename = "SHAPE_CIRCLE")]
    ShapeCircle,
    /// [AI 생성] 역원형 테두리
    #[serde(rename = "SHAPE_REVERSAL_CIRCLE")]
    ShapeReversalCircle,
    /// [AI 생성] 사각형 테두리
    #[serde(rename = "SHAPE_RECTANGLE")]
    ShapeRectangle,
    /// [AI 생성] 역사각형 테두리
    #[serde(rename = "SHAPE_REVERSAL_RECTANGLE")]
    ShapeReversalRectangle,
    /// [AI 생성] 삼각형 테두리
    #[serde(rename = "SHAPE_TRIANGLE")]
    ShapeTriangle,
    /// [AI 생성] 역삼각형 테두리
    #[serde(rename = "SHAPE_REVERSAL_TIRANGLE")]
    ShapeReversalTriangle,
    /// [AI 생성] 전구/전광판형 테두리
    #[serde(rename = "SHAPE_LIGHT")]
    ShapeLight,
    /// [AI 생성] 마름모 테두리
    #[serde(rename = "SHAPE_RHOMBUS")]
    ShapeRhombus,
    /// [AI 생성] 역마름모 테두리
    #[serde(rename = "SHAPE_REVERSAL_RHOMBUS")]
    ShapeReversalRhombus,
    /// [AI 생성] 둥근 사각형 테두리
    #[serde(rename = "SHAPE_ROUNDED_RECTANGLE")]
    ShapeRoundedRectangle,
    /// [AI 생성] 빈 순환 삼각형 테두리
    #[serde(rename = "SHAPE_EMPTY_CIRCULATE_TRIANGLE")]
    ShapeEmptyCirculateTriangle,
    /// [AI 생성] 가는 순환 삼각형 테두리
    #[serde(rename = "SHAPE_THIN_CIRCULATE_TRIANGLE")]
    ShapeThinCirculateTriangle,
    /// [AI 생성] 두꺼운 순환 삼각형 테두리
    #[serde(rename = "SHAPE_THICK_CIRCULATE_TRIANGLE")]
    ShapeThickCirculateTriangle,
}

/// [AI 생성] 글자 겹침 종류 (퍼짐/겹침 방식)
///
/// 원본: `compose.composeType` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComposeType {
    /// [AI 생성] 글자를 벌려 배치
    #[serde(rename = "SPREAD")]
    Spread,
    /// [AI 생성] 글자를 겹쳐 배치
    #[serde(rename = "OVERLAP")]
    Overlap,
}

/// [AI 생성] 글자 속성 참조
///
/// 원본: `compose/charPr` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "charPr")]
pub struct ComposeCharProperty {
    /// [AI 생성] 글자 속성 참조 값
    #[serde(rename = "@prIDRef", skip_serializing_if = "Option::is_none")]
    pub property_id_ref: Option<CharShapeIdRef>,
}

/// [AI 생성] 글자 겹침 (Compose) 설정
///
/// 원본: `compose` 요소
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "compose")]
pub struct Compose {
    /// [AI 생성] 글자 속성들
    #[serde(rename = "charPr")]
    pub char_properties: Vec<ComposeCharProperty>,

    /// [AI 생성] 테두리 종류 (`circleType`)
    #[serde(rename = "@circleType", default)]
    pub circle_type: ComposeCircleType,

    /// [AI 생성] 테두리 내부 글자 크기 비율 (%)
    #[serde(rename = "@charSz", skip_serializing_if = "Option::is_none")]
    pub char_size: Option<i32>,

    /// [AI 생성] 겹침 종류 (`composeType`)
    #[serde(rename = "@composeType", skip_serializing_if = "Option::is_none")]
    pub compose_type: Option<ComposeType>,

    /// [AI 생성] 글자 속성 개수 (`charPrCnt`)
    #[serde(rename = "@charPrCnt", skip_serializing_if = "Option::is_none")]
    pub char_property_count: Option<u32>,

    /// [AI 생성] 겹침 텍스트 (`composeText`)
    #[serde(rename = "@composeText", skip_serializing_if = "Option::is_none")]
    pub compose_text: Option<String>,
}

/// [AI 생성] 덧말 위치 (`posType` 기준)
///
/// 원본: `dutmal.posType` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DutmalPosition {
    #[default]
    /// [AI 생성] 위쪽 덧말
    #[serde(rename = "TOP")]
    Top,
    /// [AI 생성] 아래쪽 덧말
    #[serde(rename = "BOTTOM")]
    Bottom,
}

/// [AI 생성] 덧말 정렬 옵션
///
/// 원본: `dutmal.align` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DutmalAlignment {
    /// [AI 생성] 양쪽 맞춤
    #[serde(rename = "JUSTIFY")]
    Justify,
    /// [AI 생성] 왼쪽 정렬
    #[serde(rename = "LEFT")]
    Left,
    /// [AI 생성] 오른쪽 정렬
    #[serde(rename = "RIGHT")]
    Right,
    #[default]
    /// [AI 생성] 가운데 정렬
    #[serde(rename = "CENTER")]
    Center,
    /// [AI 생성] 균등 배치
    #[serde(rename = "DISTRIBUTE")]
    Distribute,
    /// [AI 생성] 공백 포함 균등 배치
    #[serde(rename = "DISTRIBUTE_SPACE")]
    DistributeSpace,
}

/// [AI 생성] 덧말 (주 텍스트 위/아래 보조 텍스트)
///
/// 원본: `dutmal` 요소
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "dutmal")]
pub struct Dutmal {
    /// [AI 생성] 주 텍스트
    #[serde(rename = "mainText")]
    pub main_text: String,

    /// [AI 생성] 덧말 텍스트
    #[serde(rename = "subText")]
    pub sub_text: String,

    /// [AI 생성] 위치 (`posType`, 위/아래)
    #[serde(rename = "@posType", default)]
    pub position_type: DutmalPosition,

    /// [AI 생성] 크기 비율 (`szRatio`)
    #[serde(rename = "@szRatio", skip_serializing_if = "Option::is_none")]
    pub size_ratio: Option<u32>,

    /// [AI 생성] 옵션 (고정값 4, 사양 유지)
    #[serde(rename = "@option", skip_serializing_if = "Option::is_none")]
    pub option: Option<u32>,

    /// [AI 생성] 스타일 참조
    #[serde(rename = "@styleDRef", skip_serializing_if = "Option::is_none")]
    pub style_id_ref: Option<StyleIdRef>,

    /// [AI 생성] 정렬
    #[serde(rename = "@align", default)]
    pub alignment: DutmalAlignment,
}
