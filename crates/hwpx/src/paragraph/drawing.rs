//! [AI 생성] 그리기 객체 집합 (선/도형/커넥터)
//!
//! 문단에 배치되는 대부분의 벡터 도형을 정의합니다. 공통 배치/감싸기/회전 정보를 `shape_common`에서 상속받고, 텍스트 박스(`drawText`)를 통해 내부 문단도 포함할 수 있습니다. KS X 6101:2024 `paralist.xsd` 기준.

use serde::{Deserialize, Serialize};

use super::line_shape::LineShape;
use super::para_list::ParagraphList;
use super::shadow::ShapeShadow;
use super::shape_common::{
    ArcStyle, ConnectControlPoint, ConnectLineStyle, ConnectPoint, CurrentSize,
    CurveSegment, Flip, OriginalSize, OutsideMargin, RenderingInfo, RotationInfo,
    ShapeComponentOffset, ShapeNumberingType, TextFlowMode, TextWrapMode,
};
use super::shape_common::{ShapeObjectPosition, ShapeObjectSize};
use super::table::Caption;
use crate::core::types::{FillBrush, MetaTag, Point};

/// [AI 생성] 텍스트 여백 (`textMargin`)
///
/// 원본: `drawText/textMargin` 요소의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "textMargin")]
pub struct TextMargin {
    /// 왼쪽 여백
    ///
    /// 원본: `left` 속성
    #[serde(rename = "@left", default)]
    pub left: u32,

    /// 오른쪽 여백
    ///
    /// 원본: `right` 속성
    #[serde(rename = "@right", default)]
    pub right: u32,

    /// 위쪽 여백
    ///
    /// 원본: `top` 속성
    #[serde(rename = "@top", default)]
    pub top: u32,

    /// 아래쪽 여백
    ///
    /// 원본: `bottom` 속성
    #[serde(rename = "@bottom", default)]
    pub bottom: u32,
}

/// [AI 생성] 그리기 텍스트 블록
///
/// 원본: `AbstractDrawingObjectType/drawText` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "drawText")]
pub struct DrawText {
    /// 문단 목록
    ///
    /// 원본: `subList` 요소
    #[serde(rename = "subList")]
    pub paragraph_list: ParagraphList,

    /// 텍스트 여백
    ///
    /// 원본: `textMargin` 요소
    #[serde(rename = "textMargin", skip_serializing_if = "Option::is_none")]
    pub text_margin: Option<TextMargin>,

    /// 마지막 너비
    ///
    /// 원본: `lastWidth` 속성
    #[serde(rename = "@lastWidth", skip_serializing_if = "Option::is_none")]
    pub last_width: Option<u32>,

    /// 이름
    ///
    /// 원본: `name` 속성
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 편집 가능 여부
    ///
    /// 원본: `editable` 속성
    #[serde(rename = "@editable", default)]
    pub editable: bool,
}

/// [AI 생성] 시작·끝 좌표로 정의되는 직선 도형
///
/// 원본: `LineType` (`line` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "line")]
pub struct Line {
    // AbstractShapeObjectType 요소들
    /// [AI 생성] 배치 기준 크기 (텍스트 감싸기 계산에 사용)
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,
    /// [AI 생성] 페이지/단 기준 위치
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,
    /// [AI 생성] 텍스트와의 외곽 여백
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,
    /// [AI 생성] 도형 캡션
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
    /// [AI 생성] 편집기 내부 메모
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// [AI 생성] 추가 메타데이터
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    // AbstractShapeComponentType 요소들
    /// 오프셋
    #[serde(rename = "offset")]
    pub offset: ShapeComponentOffset,
    /// 원본 크기
    #[serde(rename = "orgSz")]
    pub original_size: OriginalSize,
    /// 현재 크기
    #[serde(rename = "curSz")]
    pub current_size: CurrentSize,
    /// 뒤집기
    #[serde(rename = "flip")]
    pub flip: Flip,
    /// 회전 정보
    #[serde(rename = "rotationInfo")]
    pub rotation_info: RotationInfo,
    /// 렌더링 정보
    #[serde(rename = "renderingInfo")]
    pub rendering_info: RenderingInfo,

    // AbstractDrawingObjectType 요소들
    /// 선 모양
    #[serde(rename = "lineShape")]
    pub line_shape: LineShape,
    /// 채우기
    #[serde(rename = "fillBrush", skip_serializing_if = "Option::is_none")]
    pub fill_brush: Option<FillBrush>,
    /// 그리기 텍스트
    #[serde(rename = "drawText", skip_serializing_if = "Option::is_none")]
    pub draw_text: Option<DrawText>,
    /// 그림자
    #[serde(rename = "shadow", skip_serializing_if = "Option::is_none")]
    pub shadow: Option<ShapeShadow>,

    // LineType 전용 요소들
    /// [AI 생성] 선의 시작 좌표 (`startPt` 요소)
    #[serde(rename = "startPt")]
    pub start_point: Point,
    /// [AI 생성] 선의 끝 좌표 (`endPt` 요소)
    #[serde(rename = "endPt")]
    pub end_point: Point,

    // 속성들
    /// [AI 생성] 도형 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 그림/표/수식 등 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,
    /// [AI 생성] 하이퍼링크 대상 (`href` 속성)
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// [AI 생성] 그룹 중첩 깊이 (`groupLevel` 속성)
    #[serde(rename = "@groupLevel", default)]
    pub group_level: u32,
    /// [AI 생성] 복제된 인스턴스 식별자 (`instid` 속성)
    #[serde(rename = "@instid", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<u32>,
    /// [AI 생성] 좌우/상하 반전 여부 (`isReverseHV` 속성)
    #[serde(rename = "@isReverseHV", default)]
    pub is_reverse_horizontal_vertical: bool,
}

/// [AI 생성] 네 꼭짓점으로 정의되는 사각형 도형 (둥근 모서리 가능)
///
/// 원본: `RectangleType` (`rect` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "rect")]
pub struct Rectangle {
    // AbstractShapeObjectType 요소들
    /// [AI 생성] 크기 (`sz` 요소). 도형의 가로/세로.
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,
    /// [AI 생성] 위치 (`pos` 요소). 페이지 기준 배치 위치.
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,
    /// [AI 생성] 바깥 여백 (`outMargin` 요소). 주변 여백.
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,
    /// [AI 생성] 캡션 (`caption` 요소)
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
    /// [AI 생성] 도형 주석 텍스트 (`shapeComment` 요소)
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// [AI 생성] 메타 태그 (`metaTag` 요소)
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    // AbstractShapeComponentType 요소들
    /// [AI 생성] 기준점 오프셋 (`offset` 요소). 그룹 기준 위치 조정.
    #[serde(rename = "offset")]
    pub offset: ShapeComponentOffset,
    /// [AI 생성] 원본 크기 (`orgSz` 요소). 비율 계산용.
    #[serde(rename = "orgSz")]
    pub original_size: OriginalSize,
    /// [AI 생성] 현재 크기 (`curSz` 요소). 렌더링 시 실제 크기.
    #[serde(rename = "curSz")]
    pub current_size: CurrentSize,
    /// [AI 생성] 좌우/상하 반전 (`flip` 요소)
    #[serde(rename = "flip")]
    pub flip: Flip,
    /// [AI 생성] 회전 정보 (`rotationInfo` 요소)
    #[serde(rename = "rotationInfo")]
    pub rotation_info: RotationInfo,
    /// [AI 생성] 렌더링 부가 정보 (`renderingInfo` 요소)
    #[serde(rename = "renderingInfo")]
    pub rendering_info: RenderingInfo,

    // AbstractDrawingObjectType 요소들
    /// [AI 생성] 선 모양 (`lineShape` 요소)
    #[serde(rename = "lineShape")]
    pub line_shape: LineShape,
    /// [AI 생성] 채우기 브러시 (`fillBrush` 요소)
    #[serde(rename = "fillBrush", skip_serializing_if = "Option::is_none")]
    pub fill_brush: Option<FillBrush>,
    /// [AI 생성] 도형 내 텍스트 (`drawText` 요소)
    #[serde(rename = "drawText", skip_serializing_if = "Option::is_none")]
    pub draw_text: Option<DrawText>,
    /// [AI 생성] 그림자 (`shadow` 요소)
    #[serde(rename = "shadow", skip_serializing_if = "Option::is_none")]
    pub shadow: Option<ShapeShadow>,

    // RectangleType 전용 요소들
    /// [AI 생성] 사각형 꼭짓점 0 (`pt0` 요소)
    #[serde(rename = "pt0")]
    pub point0: Point,
    /// [AI 생성] 사각형 꼭짓점 1 (`pt1` 요소)
    #[serde(rename = "pt1")]
    pub point1: Point,
    /// [AI 생성] 사각형 꼭짓점 2 (`pt2` 요소)
    #[serde(rename = "pt2")]
    pub point2: Point,
    /// [AI 생성] 사각형 꼭짓점 3 (`pt3` 요소)
    #[serde(rename = "pt3")]
    pub point3: Point,

    // 속성들
    /// [AI 생성] 도형 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 그림/표/수식 등 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,
    /// [AI 생성] 하이퍼링크 대상 (`href` 속성)
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// [AI 생성] 그룹 중첩 깊이 (`groupLevel` 속성)
    #[serde(rename = "@groupLevel", default)]
    pub group_level: u32,
    /// [AI 생성] 복제된 인스턴스 식별자 (`instid` 속성)
    #[serde(rename = "@instid", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<u32>,
    /// [AI 생성] 둥근 모서리 비율 (`ratio` 속성, 0이면 직각)
    #[serde(rename = "@ratio", skip_serializing_if = "Option::is_none")]
    pub ratio: Option<u32>,
}

/// [AI 생성] 중심과 두 축으로 정의되는 타원 또는 원호
///
/// 원본: `EllipseType` (`ellipse` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ellipse")]
pub struct Ellipse {
    // 공통 요소들
    /// [AI 생성] 크기 (`sz` 요소). 도형의 가로/세로.
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,
    /// [AI 생성] 위치 (`pos` 요소). 페이지 기준 배치 위치.
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,
    /// [AI 생성] 바깥 여백 (`outMargin` 요소). 주변 여백.
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,
    /// [AI 생성] 캡션 (`caption` 요소)
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
    /// [AI 생성] 도형 주석 텍스트 (`shapeComment` 요소)
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// [AI 생성] 메타 태그 (`metaTag` 요소)
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    /// [AI 생성] 기준점 오프셋 (`offset` 요소). 그룹 기준 위치 조정.
    #[serde(rename = "offset")]
    pub offset: ShapeComponentOffset,
    /// [AI 생성] 원본 크기 (`orgSz` 요소). 비율 계산용.
    #[serde(rename = "orgSz")]
    pub original_size: OriginalSize,
    /// [AI 생성] 현재 크기 (`curSz` 요소). 렌더링 시 실제 크기.
    #[serde(rename = "curSz")]
    pub current_size: CurrentSize,
    /// [AI 생성] 좌우/상하 반전 (`flip` 요소)
    #[serde(rename = "flip")]
    pub flip: Flip,
    /// [AI 생성] 회전 정보 (`rotationInfo` 요소)
    #[serde(rename = "rotationInfo")]
    pub rotation_info: RotationInfo,
    /// [AI 생성] 렌더링 부가 정보 (`renderingInfo` 요소)
    #[serde(rename = "renderingInfo")]
    pub rendering_info: RenderingInfo,

    /// [AI 생성] 선 모양 (`lineShape` 요소)
    #[serde(rename = "lineShape")]
    pub line_shape: LineShape,
    /// [AI 생성] 채우기 브러시 (`fillBrush` 요소)
    #[serde(rename = "fillBrush", skip_serializing_if = "Option::is_none")]
    pub fill_brush: Option<FillBrush>,
    /// [AI 생성] 도형 내 텍스트 (`drawText` 요소)
    #[serde(rename = "drawText", skip_serializing_if = "Option::is_none")]
    pub draw_text: Option<DrawText>,
    /// [AI 생성] 그림자 (`shadow` 요소)
    #[serde(rename = "shadow", skip_serializing_if = "Option::is_none")]
    pub shadow: Option<ShapeShadow>,

    // EllipseType 전용 요소들
    /// [AI 생성] 타원의 중심점 (`center` 요소)
    #[serde(rename = "center")]
    pub center: Point,
    /// [AI 생성] 첫 번째 축 벡터 끝점 (`ax1` 요소)
    #[serde(rename = "ax1")]
    pub axis1: Point,
    /// [AI 생성] 두 번째 축 벡터 끝점 (`ax2` 요소)
    #[serde(rename = "ax2")]
    pub axis2: Point,
    /// [AI 생성] 첫 번째 호 시작점 (`start1` 요소)
    #[serde(rename = "start1")]
    pub start1: Point,
    /// [AI 생성] 첫 번째 호 끝점 (`end1` 요소)
    #[serde(rename = "end1")]
    pub end1: Point,
    /// [AI 생성] 두 번째 호 시작점 (`start2` 요소)
    #[serde(rename = "start2")]
    pub start2: Point,
    /// [AI 생성] 두 번째 호 끝점 (`end2` 요소)
    #[serde(rename = "end2")]
    pub end2: Point,

    // 속성들
    /// [AI 생성] 도형 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 그림/표/수식 등 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,
    /// [AI 생성] 하이퍼링크 대상 (`href` 속성)
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// [AI 생성] 그룹 중첩 깊이 (`groupLevel` 속성)
    #[serde(rename = "@groupLevel", default)]
    pub group_level: u32,
    /// [AI 생성] 복제된 인스턴스 식별자 (`instid` 속성)
    #[serde(rename = "@instid", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<u32>,
    /// [AI 생성] 호 구간이 수정되었는지 여부 (`intervalDirty` 속성)
    #[serde(rename = "@intervalDirty", default)]
    pub interval_dirty: bool,
    /// [AI 생성] 추가 호 속성 존재 여부 (`hasArcPr` 속성)
    #[serde(rename = "@hasArcPr", default)]
    pub has_arc_properties: bool,
    /// [AI 생성] 호 표현 방식 (`arcType` 속성)
    #[serde(rename = "@arcType", default)]
    pub arc_type: ArcStyle,
}

/// [AI 생성] 원호(Arc) 도형
///
/// 원본: `ArcType` (`arc` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "arc")]
pub struct Arc {
    // 공통 요소들
    /// [AI 생성] 크기 (`sz` 요소). 도형의 가로/세로.
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,
    /// [AI 생성] 위치 (`pos` 요소). 페이지 기준 배치 위치.
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,
    /// [AI 생성] 바깥 여백 (`outMargin` 요소). 주변 여백.
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,
    /// [AI 생성] 캡션 (`caption` 요소)
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
    /// [AI 생성] 도형 주석 텍스트 (`shapeComment` 요소)
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// [AI 생성] 메타 태그 (`metaTag` 요소)
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    /// [AI 생성] 기준점 오프셋 (`offset` 요소). 그룹 기준 위치 조정.
    #[serde(rename = "offset")]
    pub offset: ShapeComponentOffset,
    /// [AI 생성] 원본 크기 (`orgSz` 요소). 비율 계산용.
    #[serde(rename = "orgSz")]
    pub original_size: OriginalSize,
    /// [AI 생성] 현재 크기 (`curSz` 요소). 렌더링 시 실제 크기.
    #[serde(rename = "curSz")]
    pub current_size: CurrentSize,
    /// [AI 생성] 좌우/상하 반전 (`flip` 요소)
    #[serde(rename = "flip")]
    pub flip: Flip,
    /// [AI 생성] 회전 정보 (`rotationInfo` 요소)
    #[serde(rename = "rotationInfo")]
    pub rotation_info: RotationInfo,
    /// [AI 생성] 렌더링 부가 정보 (`renderingInfo` 요소)
    #[serde(rename = "renderingInfo")]
    pub rendering_info: RenderingInfo,

    /// [AI 생성] 선 모양 (`lineShape` 요소)
    #[serde(rename = "lineShape")]
    pub line_shape: LineShape,
    /// [AI 생성] 채우기 브러시 (`fillBrush` 요소)
    #[serde(rename = "fillBrush", skip_serializing_if = "Option::is_none")]
    pub fill_brush: Option<FillBrush>,
    /// [AI 생성] 도형 내 텍스트 (`drawText` 요소)
    #[serde(rename = "drawText", skip_serializing_if = "Option::is_none")]
    pub draw_text: Option<DrawText>,
    /// [AI 생성] 그림자 (`shadow` 요소)
    #[serde(rename = "shadow", skip_serializing_if = "Option::is_none")]
    pub shadow: Option<ShapeShadow>,

    // ArcType 전용 요소들
    /// [AI 생성] 호의 중심점 (`center` 요소)
    #[serde(rename = "center")]
    pub center: Point,
    /// [AI 생성] 첫 번째 축 벡터 (`ax1` 요소)
    #[serde(rename = "ax1")]
    pub axis1: Point,
    /// [AI 생성] 두 번째 축 벡터 (`ax2` 요소)
    #[serde(rename = "ax2")]
    pub axis2: Point,

    // 속성들
    /// [AI 생성] 도형 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 그림/표/수식 등 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,
    /// [AI 생성] 하이퍼링크 대상 (`href` 속성)
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// [AI 생성] 그룹 중첩 깊이 (`groupLevel` 속성)
    #[serde(rename = "@groupLevel", default)]
    pub group_level: u32,
    /// [AI 생성] 복제된 인스턴스 식별자 (`instid` 속성)
    #[serde(rename = "@instid", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<u32>,
    /// [AI 생성] 호 표현 방식 (`type` 속성)
    #[serde(rename = "@type", default)]
    pub arc_type: ArcStyle,
}

/// [AI 생성] 여러 꼭짓점을 연결한 다각형 도형
///
/// 원본: `PolygonType` (`polygon` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "polygon")]
pub struct Polygon {
    // 공통 요소들
    /// [AI 생성] 크기 (`sz` 요소). 도형의 가로/세로.
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,
    /// [AI 생성] 위치 (`pos` 요소). 페이지 기준 배치 위치.
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,
    /// [AI 생성] 바깥 여백 (`outMargin` 요소). 주변 여백.
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,
    /// [AI 생성] 캡션 (`caption` 요소)
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
    /// [AI 생성] 도형 주석 텍스트 (`shapeComment` 요소)
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// [AI 생성] 메타 태그 (`metaTag` 요소)
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    /// [AI 생성] 기준점 오프셋 (`offset` 요소). 그룹 기준 위치 조정.
    #[serde(rename = "offset")]
    pub offset: ShapeComponentOffset,
    /// [AI 생성] 원본 크기 (`orgSz` 요소). 비율 계산용.
    #[serde(rename = "orgSz")]
    pub original_size: OriginalSize,
    /// [AI 생성] 현재 크기 (`curSz` 요소). 렌더링 시 실제 크기.
    #[serde(rename = "curSz")]
    pub current_size: CurrentSize,
    /// [AI 생성] 좌우/상하 반전 (`flip` 요소)
    #[serde(rename = "flip")]
    pub flip: Flip,
    /// [AI 생성] 회전 정보 (`rotationInfo` 요소)
    #[serde(rename = "rotationInfo")]
    pub rotation_info: RotationInfo,
    /// [AI 생성] 렌더링 부가 정보 (`renderingInfo` 요소)
    #[serde(rename = "renderingInfo")]
    pub rendering_info: RenderingInfo,

    /// [AI 생성] 선 모양 (`lineShape` 요소)
    #[serde(rename = "lineShape")]
    pub line_shape: LineShape,
    /// [AI 생성] 채우기 브러시 (`fillBrush` 요소)
    #[serde(rename = "fillBrush", skip_serializing_if = "Option::is_none")]
    pub fill_brush: Option<FillBrush>,
    /// [AI 생성] 도형 내 텍스트 (`drawText` 요소)
    #[serde(rename = "drawText", skip_serializing_if = "Option::is_none")]
    pub draw_text: Option<DrawText>,
    /// [AI 생성] 그림자 (`shadow` 요소)
    #[serde(rename = "shadow", skip_serializing_if = "Option::is_none")]
    pub shadow: Option<ShapeShadow>,

    // PolygonType 전용 요소들
    /// [AI 생성] 꼭짓점 좌표 목록 (`pt` 요소)
    #[serde(rename = "pt")]
    pub points: Vec<Point>,

    // 속성들
    /// [AI 생성] 도형 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 그림/표/수식 등 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,
    /// [AI 생성] 하이퍼링크 대상 (`href` 속성)
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// [AI 생성] 그룹 중첩 깊이 (`groupLevel` 속성)
    #[serde(rename = "@groupLevel", default)]
    pub group_level: u32,
    /// [AI 생성] 복제된 인스턴스 식별자 (`instid` 속성)
    #[serde(rename = "@instid", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<u32>,
}

/// [AI 생성] 직선·베지어 세그먼트로 구성된 곡선 도형
///
/// 원본: `CurveType` (`curve` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "curve")]
pub struct Curve {
    // 공통 요소들
    /// [AI 생성] 크기 (`sz` 요소). 도형의 가로/세로.
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,
    /// [AI 생성] 위치 (`pos` 요소). 페이지 기준 배치 위치.
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,
    /// [AI 생성] 바깥 여백 (`outMargin` 요소). 주변 여백.
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,
    /// [AI 생성] 캡션 (`caption` 요소)
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
    /// [AI 생성] 도형 주석 텍스트 (`shapeComment` 요소)
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// [AI 생성] 메타 태그 (`metaTag` 요소)
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    /// [AI 생성] 기준점 오프셋 (`offset` 요소). 그룹 기준 위치 조정.
    #[serde(rename = "offset")]
    pub offset: ShapeComponentOffset,
    /// [AI 생성] 원본 크기 (`orgSz` 요소). 비율 계산용.
    #[serde(rename = "orgSz")]
    pub original_size: OriginalSize,
    /// [AI 생성] 현재 크기 (`curSz` 요소). 렌더링 시 실제 크기.
    #[serde(rename = "curSz")]
    pub current_size: CurrentSize,
    /// [AI 생성] 좌우/상하 반전 (`flip` 요소)
    #[serde(rename = "flip")]
    pub flip: Flip,
    /// [AI 생성] 회전 정보 (`rotationInfo` 요소)
    #[serde(rename = "rotationInfo")]
    pub rotation_info: RotationInfo,
    /// [AI 생성] 렌더링 부가 정보 (`renderingInfo` 요소)
    #[serde(rename = "renderingInfo")]
    pub rendering_info: RenderingInfo,

    /// [AI 생성] 선 모양 (`lineShape` 요소)
    #[serde(rename = "lineShape")]
    pub line_shape: LineShape,
    /// [AI 생성] 채우기 브러시 (`fillBrush` 요소)
    #[serde(rename = "fillBrush", skip_serializing_if = "Option::is_none")]
    pub fill_brush: Option<FillBrush>,
    /// [AI 생성] 도형 내 텍스트 (`drawText` 요소)
    #[serde(rename = "drawText", skip_serializing_if = "Option::is_none")]
    pub draw_text: Option<DrawText>,
    /// [AI 생성] 그림자 (`shadow` 요소)
    #[serde(rename = "shadow", skip_serializing_if = "Option::is_none")]
    pub shadow: Option<ShapeShadow>,

    // CurveType 전용 요소들
    /// [AI 생성] 곡선 세그먼트 정의 (`seg` 요소)
    #[serde(rename = "seg")]
    pub segments: Vec<CurveSegment>,

    // 속성들
    /// [AI 생성] 도형 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 그림/표/수식 등 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,
    /// [AI 생성] 하이퍼링크 대상 (`href` 속성)
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// [AI 생성] 그룹 중첩 깊이 (`groupLevel` 속성)
    #[serde(rename = "@groupLevel", default)]
    pub group_level: u32,
    /// [AI 생성] 복제된 인스턴스 식별자 (`instid` 속성)
    #[serde(rename = "@instid", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<u32>,
}

/// [AI 생성] 연결선 꺾임/곡률을 결정하는 제어점 목록
///
/// 원본: `ConnectLineType/controlPoints` 요소의 익명 타입
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename = "controlPoints")]
pub struct ConnectControlPoints {
    /// [AI 생성] 제어점 좌표들
    #[serde(rename = "point")]
    pub points: Vec<ConnectControlPoint>,
}

/// [AI 생성] 개체 간 연결선(커넥터)
///
/// 원본: `ConnectLineType` (`connectLine` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "connectLine")]
pub struct ConnectLine {
    // 공통 요소들
    /// [AI 생성] 크기 (`sz` 요소). 도형의 가로/세로.
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,
    /// [AI 생성] 위치 (`pos` 요소). 페이지 기준 배치 위치.
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,
    /// [AI 생성] 바깥 여백 (`outMargin` 요소)
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,
    /// [AI 생성] 캡션 (`caption` 요소)
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
    /// [AI 생성] 도형 주석 텍스트 (`shapeComment` 요소)
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// [AI 생성] 메타 태그 (`metaTag` 요소)
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    /// [AI 생성] 기준점 오프셋 (`offset` 요소)
    #[serde(rename = "offset")]
    pub offset: ShapeComponentOffset,
    /// [AI 생성] 원본 크기 (`orgSz` 요소)
    #[serde(rename = "orgSz")]
    pub original_size: OriginalSize,
    /// [AI 생성] 현재 크기 (`curSz` 요소)
    #[serde(rename = "curSz")]
    pub current_size: CurrentSize,
    /// [AI 생성] 좌우/상하 반전 (`flip` 요소)
    #[serde(rename = "flip")]
    pub flip: Flip,
    /// [AI 생성] 회전 정보 (`rotationInfo` 요소)
    #[serde(rename = "rotationInfo")]
    pub rotation_info: RotationInfo,
    /// [AI 생성] 렌더링 부가 정보 (`renderingInfo` 요소)
    #[serde(rename = "renderingInfo")]
    pub rendering_info: RenderingInfo,

    /// [AI 생성] 선 모양 (`lineShape` 요소)
    #[serde(rename = "lineShape")]
    pub line_shape: LineShape,
    /// [AI 생성] 채우기 브러시 (`fillBrush` 요소)
    #[serde(rename = "fillBrush", skip_serializing_if = "Option::is_none")]
    pub fill_brush: Option<FillBrush>,
    /// [AI 생성] 도형 내 텍스트 (`drawText` 요소)
    #[serde(rename = "drawText", skip_serializing_if = "Option::is_none")]
    pub draw_text: Option<DrawText>,
    /// [AI 생성] 그림자 (`shadow` 요소)
    #[serde(rename = "shadow", skip_serializing_if = "Option::is_none")]
    pub shadow: Option<ShapeShadow>,

    // ConnectLineType 전용 요소들
    /// [AI 생성] 연결 시작점 (개체 연결 정보를 포함) (`startPt` 요소)
    #[serde(rename = "startPt")]
    pub start_point: ConnectPoint,
    /// [AI 생성] 연결 끝점 (`endPt` 요소)
    #[serde(rename = "endPt")]
    pub end_point: ConnectPoint,
    /// [AI 생성] 꺾임/곡률 제어점 (`controlPoints` 요소)
    #[serde(rename = "controlPoints", skip_serializing_if = "Option::is_none")]
    pub control_points: Option<ConnectControlPoints>,

    // 속성들
    /// [AI 생성] 도형 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,
    /// [AI 생성] 하이퍼링크 대상 (`href` 속성)
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// [AI 생성] 그룹 중첩 깊이 (`groupLevel` 속성)
    #[serde(rename = "@groupLevel", default)]
    pub group_level: u32,
    /// [AI 생성] 복제 인스턴스 아이디 (`instid` 속성)
    #[serde(rename = "@instid", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<u32>,
    /// [AI 생성] 커넥터 스타일 (`type` 속성)
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub line_type: Option<ConnectLineStyle>,
}

/// [AI 생성] 스펙 외 개체를 보존하기 위한 UnknownObject
///
/// 원본: `UnknownObjectType`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "unknownObject")]
pub struct UnknownObject {
    // AbstractShapeObjectType 요소들
    /// 크기
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,
    /// 위치
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,
    /// 바깥 여백
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,
    /// 캡션
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
    /// 도형 주석
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// 메타 태그
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    // AbstractShapeComponentType 요소들
    /// [AI 생성] 기준점 오프셋 (`offset` 요소)
    #[serde(rename = "offset")]
    pub offset: ShapeComponentOffset,
    /// [AI 생성] 원본 크기 (`orgSz` 요소)
    #[serde(rename = "orgSz")]
    pub original_size: OriginalSize,
    /// [AI 생성] 현재 크기 (`curSz` 요소)
    #[serde(rename = "curSz")]
    pub current_size: CurrentSize,
    /// [AI 생성] 좌우/상하 반전 (`flip` 요소)
    #[serde(rename = "flip")]
    pub flip: Flip,
    /// [AI 생성] 회전 정보 (`rotationInfo` 요소)
    #[serde(rename = "rotationInfo")]
    pub rotation_info: RotationInfo,
    /// [AI 생성] 렌더링 부가 정보 (`renderingInfo` 요소)
    #[serde(rename = "renderingInfo")]
    pub rendering_info: RenderingInfo,

    // AbstractDrawingObjectType 요소들
    /// [AI 생성] 선 모양 (`lineShape` 요소)
    #[serde(rename = "lineShape")]
    pub line_shape: LineShape,
    /// [AI 생성] 채우기 브러시 (`fillBrush` 요소)
    #[serde(rename = "fillBrush", skip_serializing_if = "Option::is_none")]
    pub fill_brush: Option<FillBrush>,
    /// [AI 생성] 도형 내 텍스트 (`drawText` 요소)
    #[serde(rename = "drawText", skip_serializing_if = "Option::is_none")]
    pub draw_text: Option<DrawText>,
    /// [AI 생성] 그림자 (`shadow` 요소)
    #[serde(rename = "shadow", skip_serializing_if = "Option::is_none")]
    pub shadow: Option<ShapeShadow>,

    // UnknownObjectType 전용 요소들
    /// [AI 생성] 사각 클리핑 영역 점0 (`Point0` 요소)
    #[serde(rename = "Point0")]
    pub point0: Point,
    /// [AI 생성] 사각 클리핑 영역 점1 (`Point1` 요소)
    #[serde(rename = "Point1")]
    pub point1: Point,
    /// [AI 생성] 사각 클리핑 영역 점2 (`Point2` 요소)
    #[serde(rename = "Point2")]
    pub point2: Point,
    /// [AI 생성] 사각 클리핑 영역 점3 (`Point3` 요소)
    #[serde(rename = "Point3")]
    pub point3: Point,

    // 속성들
    /// [AI 생성] 도형 식별자 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] 겹침 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 번호 매기기 유형 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 감싸기 방식 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 이동/편집 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,
    /// [AI 생성] 하이퍼링크 대상 (`href` 속성)
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// [AI 생성] 그룹 중첩 깊이 (`groupLevel` 속성)
    #[serde(rename = "@groupLevel", default)]
    pub group_level: u32,
    /// [AI 생성] 복제 인스턴스 아이디 (`instid` 속성)
    #[serde(rename = "@instid", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<u32>,
    /// [AI 생성] 원본 컨트롤 아이디 (`ctrlID` 속성)
    #[serde(rename = "@ctrlID", skip_serializing_if = "Option::is_none")]
    pub ctrl_id: Option<String>,
}
