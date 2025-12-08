//! [AI 생성] 그림 도형 (배치/효과/자르기)
//!
//! 래스터 이미지를 도형처럼 배치할 때 쓰이는 타입입니다. 원본 크기와 표시 크기를 분리해 DPI 무관하게 레이아웃을 맞추고, 클리핑·안쪽 여백·효과까지 포함합니다. KS X 6101:2024 `paralist.xsd`.

use serde::{Deserialize, Serialize};

use super::effects::Effects;
use super::line_shape::LineShape;
use super::shape_common::{
    CurrentSize, Flip, ImageClip, ImageDimension, ImageRectangle, InsideMargin, OriginalSize,
    OutsideMargin, RenderingInfo, RotationInfo, ShapeComponentOffset, ShapeObjectPosition,
    ShapeObjectSize,
};
use super::table::Caption;
use crate::core::types::{Image, MetaTag};

/// [AI 생성] 그림
///
/// AbstractShapeComponentType을 확장한 그림 타입
///
/// 원본: `PictureType` (`pic` 요소)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "pic")]
pub struct Picture {
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
    /// [AI 생성] 오프셋 (`offset` 요소)
    #[serde(rename = "offset")]
    pub offset: ShapeComponentOffset,

    /// [AI 생성] 원본 크기 (`orgSz` 요소)
    #[serde(rename = "orgSz")]
    pub original_size: OriginalSize,

    /// [AI 생성] 현재 크기 (`curSz` 요소)
    #[serde(rename = "curSz")]
    pub current_size: CurrentSize,

    /// [AI 생성] 좌우/상하 뒤집기 (`flip` 요소)
    #[serde(rename = "flip")]
    pub flip: Flip,

    /// [AI 생성] 회전 정보 (`rotationInfo` 요소)
    #[serde(rename = "rotationInfo")]
    pub rotation_info: RotationInfo,

    /// [AI 생성] 렌더링 정보 (`renderingInfo` 요소)
    #[serde(rename = "renderingInfo")]
    pub rendering_info: RenderingInfo,

    // PictureType 전용 요소들
    /// [AI 생성] 선 모양 (`lineShape` 요소)
    #[serde(rename = "lineShape", skip_serializing_if = "Option::is_none")]
    pub line_shape: Option<LineShape>,

    /// [AI 생성] 이미지 사각형 (`imgRect` 요소)
    #[serde(rename = "imgRect", skip_serializing_if = "Option::is_none")]
    pub image_rectangle: Option<ImageRectangle>,

    /// [AI 생성] 이미지 클립 (`imgClip` 요소)
    #[serde(rename = "imgClip", skip_serializing_if = "Option::is_none")]
    pub image_clip: Option<ImageClip>,

    /// [AI 생성] 효과 (`effects` 요소)
    #[serde(rename = "effects", skip_serializing_if = "Option::is_none")]
    pub effects: Option<Effects>,

    /// [AI 생성] 안쪽 여백 (`inMargin` 요소)
    #[serde(rename = "inMargin", skip_serializing_if = "Option::is_none")]
    pub inside_margin: Option<InsideMargin>,

    /// [AI 생성] 이미지 차원 (`imgDim` 요소)
    #[serde(rename = "imgDim", skip_serializing_if = "Option::is_none")]
    pub image_dimension: Option<ImageDimension>,

    /// [AI 생성] 이미지 (`img` 요소)
    #[serde(rename = "img", skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,

    // AbstractShapeObjectType 속성들
    /// [AI 생성] 아이디 (`id` 속성)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// [AI 생성] Z 순서 (`zOrder` 속성)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,

    /// [AI 생성] 번호 매기기 종류 (`numberingType` 속성)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: super::shape_common::ShapeNumberingType,

    /// [AI 생성] 텍스트 배치 (`textWrap` 속성)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<super::shape_common::TextWrapMode>,

    /// [AI 생성] 텍스트 흐름 (`textFlow` 속성)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: super::shape_common::TextFlowMode,

    /// [AI 생성] 잠금 여부 (`lock` 속성)
    #[serde(rename = "@lock", default)]
    pub lock: bool,

    // AbstractShapeComponentType 속성들
    /// [AI 생성] 링크 (`href` 속성)
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,

    /// [AI 생성] 그룹 레벨 (`groupLevel` 속성)
    #[serde(rename = "@groupLevel", default)]
    pub group_level: u32,

    /// [AI 생성] 인스턴스 아이디 (`instid` 속성)
    #[serde(rename = "@instid", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<u32>,

    // PictureType 전용 속성들
    /// [AI 생성] 역방향 여부 (`reverse` 속성)
    #[serde(rename = "@reverse", skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
}
