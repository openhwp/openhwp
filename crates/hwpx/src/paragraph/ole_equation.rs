//! [AI 생성] OLE/수식/컨테이너 개체
//!
//! 외부 애플리케이션 개체, HWP 수식, 도형 컨테이너를 문단에 배치할 때 사용합니다. 임베드/링크 여부, 표시 방식(drawAspect), 기준선(eqBaseLine)을 명시해 렌더러가 올바른 공간을 확보하도록 합니다. KS X 6101:2024 `paralist.xsd`.

use serde::{Deserialize, Serialize};

use super::drawing::{Arc, ConnectLine, Curve, Ellipse, Line, Polygon, Rectangle};
use super::line_shape::LineShape;
use super::picture::Picture;
use super::shape_common::{
    CurrentSize, EquationLineMode, Flip, OleDrawAspect, OleObjectKind, OriginalSize, OutsideMargin,
    RenderingInfo, RotationInfo, ShapeComponentOffset, ShapeNumberingType, TextFlowMode,
    TextWrapMode,
};
use super::shape_common::{ShapeObjectPosition, ShapeObjectSize};
use super::table::Caption;
use crate::core::types::{BinaryItemIdRef, MetaTag, Point, RgbColor};

/// [AI 생성] 문서 내 임베드/링크되는 OLE 개체(차트/외부 앱 등)
///
/// 원본: `OLEType` (`ole` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ole")]
pub struct Ole {
    // AbstractShapeObjectType 요소들
    /// [AI 생성] 크기 (`sz` 요소)
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,
    /// [AI 생성] 위치 (`pos` 요소)
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,
    /// [AI 생성] 바깥 여백 (`outMargin` 요소)
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,
    /// [AI 생성] 캡션 (`caption` 요소)
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
    /// [AI 생성] 도형 주석 (`shapeComment` 요소)
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// [AI 생성] 메타 태그 (`metaTag` 요소)
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

    // OLEType 전용 요소들
    /// [AI 생성] 개체가 차지하는 영역 좌표 (`extent` 요소)
    #[serde(rename = "extent")]
    pub extent: Point,
    /// [AI 생성] 선 모양 (`lineShape`)
    #[serde(rename = "lineShape")]
    pub line_shape: LineShape,

    // 속성들
    /// [AI 생성] 객체 식별자 (`id` 속성)
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

    // OLEType 전용 속성들
    /// [AI 생성] 개체 종류 (임베드/링크/정적 등) (`objectType` 속성)
    #[serde(rename = "@objectType", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<OleObjectKind>,
    /// 바이너리 아이템 참조
    #[serde(rename = "@binaryItemIDRef", skip_serializing_if = "Option::is_none")]
    pub binary_item_id_ref: Option<BinaryItemIdRef>,
    /// [AI 생성] OLE 모니커 존재 여부 (`hasMoniker` 속성)
    #[serde(rename = "@hasMoniker", default)]
    pub has_moniker: bool,
    /// 표시 방식
    #[serde(rename = "@drawAspect", skip_serializing_if = "Option::is_none")]
    pub draw_aspect: Option<OleDrawAspect>,
    /// 수식 기준선
    #[serde(rename = "@eqBaseLine", default = "default_eq_baseline")]
    pub equation_baseline: i32,
}

fn default_eq_baseline() -> i32 {
    85
}

/// [AI 생성] HWP 수식(OLE 기반) 개체
///
/// 원본: `EquationType` (`equation` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "equation")]
pub struct Equation {
    // AbstractShapeObjectType 요소들
    /// [AI 생성] 크기 (`sz` 요소)
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,
    /// [AI 생성] 위치 (`pos` 요소)
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,
    /// [AI 생성] 바깥 여백 (`outMargin` 요소)
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,
    /// [AI 생성] 캡션 (`caption` 요소)
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
    /// [AI 생성] 도형 주석 (`shapeComment` 요소)
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// [AI 생성] 메타 태그 (`metaTag` 요소)
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<MetaTag>,

    // EquationType 전용 요소
    /// [AI 생성] MathML과 유사한 수식 스크립트 본문 (`script` 요소)
    #[serde(rename = "script")]
    pub script: String,

    // 속성들
    /// [AI 생성] 객체 식별자 (`id` 속성)
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

    // EquationType 전용 속성들
    /// [AI 생성] 수식 엔진 버전 (`version` 속성)
    #[serde(rename = "@version", default = "default_equation_version")]
    pub version: String,
    /// [AI 생성] 기준선 위치 (`baseLine` 속성)
    #[serde(rename = "@baseLine", default = "default_baseline")]
    pub baseline: u32,
    /// [AI 생성] 기본 글자 색 (`textColor` 속성)
    #[serde(rename = "@textColor", default = "RgbColor::black")]
    pub text_color: RgbColor,
    /// [AI 생성] 레이아웃 기본 단위 (1/1000 em) (`baseUnit` 속성)
    #[serde(rename = "@baseUnit", default = "default_base_unit")]
    pub base_unit: u32,
    /// [AI 생성] 줄 모드: 줄 단위/글자 단위 (`lineMode` 속성)
    #[serde(rename = "@lineMode", default)]
    pub line_mode: EquationLineMode,
    /// [AI 생성] 수식 폰트 이름 (`font` 속성)
    #[serde(rename = "@font", default = "default_font")]
    pub font: String,
}

fn default_equation_version() -> String {
    "Equation Version 60".to_string()
}

fn default_baseline() -> u32 {
    85
}

fn default_base_unit() -> u32 {
    1000
}

fn default_font() -> String {
    "HYhwpEQ".to_string()
}

/// 컨테이너 내 개체
///
/// 원본: `ContainerType` 내부 choice 항목
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContainerChild {
    /// 컨테이너
    #[serde(rename = "container")]
    Container(Box<Container>),
    /// 선
    #[serde(rename = "line")]
    Line(Line),
    /// 사각형
    #[serde(rename = "rect")]
    Rectangle(Rectangle),
    /// 타원
    #[serde(rename = "ellipse")]
    Ellipse(Ellipse),
    /// 호
    #[serde(rename = "arc")]
    Arc(Arc),
    /// 다각형
    #[serde(rename = "polygon")]
    Polygon(Polygon),
    /// 곡선
    #[serde(rename = "curve")]
    Curve(Curve),
    /// 연결선
    #[serde(rename = "connectLine")]
    ConnectLine(ConnectLine),
    /// 그림
    #[serde(rename = "pic")]
    Picture(Picture),
    /// OLE
    #[serde(rename = "ole")]
    Ole(Ole),
}

/// [AI 생성] 도형/그림/OLE 등을 그룹화하는 컨테이너
///
/// 원본: `ContainerType` (`container` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "container")]
pub struct Container {
    // AbstractShapeObjectType 요소들
    /// [AI 생성] 크기 (`sz` 요소)
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub size: Option<ShapeObjectSize>,
    /// [AI 생성] 위치 (`pos` 요소)
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub position: Option<ShapeObjectPosition>,
    /// [AI 생성] 바깥 여백 (`outMargin` 요소)
    #[serde(rename = "outMargin", skip_serializing_if = "Option::is_none")]
    pub outside_margin: Option<OutsideMargin>,
    /// [AI 생성] 캡션 (`caption` 요소)
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
    /// [AI 생성] 도형 주석 (`shapeComment` 요소)
    #[serde(rename = "shapeComment", skip_serializing_if = "Option::is_none")]
    pub shape_comment: Option<String>,
    /// [AI 생성] 메타 태그 (`metaTag` 요소)
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

    // ContainerType 전용 - 자식 개체들
    /// 자식 개체들
    #[serde(rename = "$value", default)]
    pub children: Vec<ContainerChild>,

    // 속성들
    /// [AI 생성] 객체 식별자 (`id` 속성)
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
}
