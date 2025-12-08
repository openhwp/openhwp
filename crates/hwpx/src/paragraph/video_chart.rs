//! [AI 생성] 비디오·차트 도형
//!
//! 문단 위에 배치되는 미디어(비디오)와 데이터 시각화(차트) 개체입니다. 일반 도형과 동일한 배치/감싸기 속성을 물려받고, 리소스 참조나 데이터 버전만 추가로 가집니다. KS X 6101:2024 `paralist.xsd` 기준.

use serde::{Deserialize, Serialize};

use super::shape_common::{
    CurrentSize, Flip, OriginalSize, OutsideMargin, RenderingInfo, RotationInfo,
    ShapeComponentOffset, ShapeNumberingType, TextFlowMode, TextWrapMode,
};
use super::shape_common::{ShapeObjectPosition, ShapeObjectSize};
use super::table::Caption;
use crate::core::types::{ChartIdRef, FileIdRef, ImageIdRef, MetaTag};

/// [AI 생성] 비디오 종류
///
/// 원본: `video.videotype` 속성의 익명 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoType {
    /// [AI 생성] 동영상 (로컬 저장 또는 패키지 내부 자원)
    #[serde(rename = "Local")]
    Local,
    /// [AI 생성] 인터넷 동영상 (외부 URL 스트림)
    #[serde(rename = "Web")]
    Web,
}

/// [AI 생성] 비디오 개체 정의
///
/// 원본: `video` 요소
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "video")]
pub struct Video {
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

    // Video 전용 속성들
    /// [AI 생성] 비디오 종류 (`videotype`)
    #[serde(rename = "@videotype")]
    pub video_type: VideoType,
    /// [AI 생성] 파일 참조 (`fileIDRef`, 로컬 비디오만 해당)
    #[serde(rename = "@fileIDRef", skip_serializing_if = "Option::is_none")]
    pub file_id_ref: Option<FileIdRef>,
    /// [AI 생성] 이미지 참조 (`imageIDRef`, 포스터 이미지)
    #[serde(rename = "@imageIDRef", skip_serializing_if = "Option::is_none")]
    pub image_id_ref: Option<ImageIdRef>,
    /// [AI 생성] 태그 (`tag`, 식별/분류 용도)
    #[serde(rename = "@tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// [AI 생성] 차트 개체 정의
///
/// 원본: `chart` 요소
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "chart")]
pub struct Chart {
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

    // 속성들
    /// [AI 생성] 객체 ID (`id`)
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// [AI 생성] Z-Order (앞뒤 순서)
    #[serde(rename = "@zOrder", default)]
    pub z_order: i32,
    /// [AI 생성] 번호 매김 방식 (`numberingType`)
    #[serde(rename = "@numberingType", default)]
    pub numbering_type: ShapeNumberingType,
    /// [AI 생성] 텍스트 줄바꿈 모드 (`textWrap`)
    #[serde(rename = "@textWrap", skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<TextWrapMode>,
    /// [AI 생성] 텍스트 흐름 방향 (`textFlow`)
    #[serde(rename = "@textFlow", default)]
    pub text_flow: TextFlowMode,
    /// [AI 생성] 편집 잠금 여부 (`lock`)
    #[serde(rename = "@lock", default)]
    pub lock: bool,

    // Chart 전용 속성들
    /// [AI 생성] 차트 데이터 버전 (`version`)
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<f32>,
    /// [AI 생성] 차트 참조 (`chartIDRef`)
    #[serde(rename = "@chartIDRef", skip_serializing_if = "Option::is_none")]
    pub chart_id_ref: Option<ChartIdRef>,
}
