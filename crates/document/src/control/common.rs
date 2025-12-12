//! 컨트롤 공통 속성
//!
//! 모든 컨트롤에서 공유하는 속성들입니다.

use ir::{Color, HwpUnit, Insets, Point, Size};
use primitive::LineType;

use crate::id::ParagraphId;

// Re-export from primitive
pub use primitive::{
    CaptionPosition, GradientType, HorizontalRelativeTo, ImageFillMode, ImageFillMode as ImageFillType, LineCap,
    LineOutlineStyle as OutlineStyle, PatternType, ShapeShadowType as ShadowType, TextWrapSide,
    TextWrapType, VerticalRelativeTo,
};

/// 개체 공통 속성
#[derive(Debug, Clone, Default)]
pub struct ObjectCommon {
    /// 개체 ID
    pub id: Option<String>,
    /// 위치
    pub position: Point,
    /// 크기
    pub size: Size,
    /// Z 순서
    pub z_order: i32,
    /// 텍스트 배치
    pub text_wrap: TextWrap,
    /// 캡션
    pub caption: Option<Caption>,
}

/// 텍스트 배치
#[derive(Debug, Clone, Default)]
pub struct TextWrap {
    /// 배치 종류
    pub wrap_type: TextWrapType,
    /// 배치 방향
    pub wrap_side: TextWrapSide,
    /// 여백
    pub margin: Insets,
    /// 세로 기준
    pub vertical_rel: VerticalRelativeTo,
    /// 가로 기준
    pub horizontal_rel: HorizontalRelativeTo,
    /// 글자처럼 취급
    pub treat_as_char: bool,
    /// 본문과 함께 이동
    pub flow_with_text: bool,
    /// 겹침 허용
    pub allow_overlap: bool,
}

/// 캡션
#[derive(Debug, Clone)]
pub struct Caption {
    /// 위치
    pub position: CaptionPosition,
    /// 너비
    pub width: HwpUnit,
    /// 간격
    pub gap: HwpUnit,
    /// 문단 목록
    pub paragraphs: Vec<ParagraphId>,
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
    /// 외곽선 스타일 (HWPX)
    pub outline_style: Option<OutlineStyle>,
    /// 투명도 (HWPX)
    pub alpha: Option<f64>,
}


impl Default for LineStyle {
    fn default() -> Self {
        Self {
            line_type: LineType::Solid,
            width: HwpUnit::from_pt(0.5),
            color: Color::BLACK,
            cap: LineCap::Flat,
            outline_style: None,
            alpha: None,
        }
    }
}

/// 채우기
#[derive(Debug, Clone)]
pub enum Fill {
    /// 단색 채우기
    Solid(SolidFill),
    /// 그라데이션 채우기
    Gradient(GradientFill),
    /// 이미지 채우기
    Image(ImageFill),
    /// 패턴 채우기
    Pattern(PatternFill),
}

/// 단색 채우기
#[derive(Debug, Clone)]
pub struct SolidFill {
    /// 배경색
    pub color: Color,
    /// 투명도 (0 ~ 100)
    pub alpha: u8,
}

/// 그라데이션 채우기
#[derive(Debug, Clone)]
pub struct GradientFill {
    /// 그라데이션 종류
    pub gradient_type: GradientType,
    /// 각도 (도)
    pub angle: f64,
    /// 중심 X
    pub center_x: i32,
    /// 중심 Y
    pub center_y: i32,
    /// 색상 정지점
    pub stops: Vec<GradientStop>,
    /// 번짐 중심
    pub step_center: i32,
}


/// 그라데이션 정지점
#[derive(Debug, Clone)]
pub struct GradientStop {
    /// 위치 (0 ~ 100)
    pub position: u8,
    /// 색상
    pub color: Color,
}

/// 이미지 채우기
#[derive(Debug, Clone)]
pub struct ImageFill {
    /// 채우기 종류
    pub fill_type: ImageFillMode,
    /// 바이너리 데이터 ID
    pub binary_id: ir::BinaryDataId,
    /// 효과
    pub effect: crate::control::ImageEffect,
    /// 밝기
    pub brightness: i8,
    /// 대비
    pub contrast: i8,
}

/// 패턴 채우기
#[derive(Debug, Clone)]
pub struct PatternFill {
    /// 패턴 종류
    pub pattern_type: PatternType,
    /// 전경색
    pub foreground: Color,
    /// 배경색
    pub background: Color,
}


/// 그림자
#[derive(Debug, Clone)]
pub struct Shadow {
    /// 그림자 종류
    pub shadow_type: ShadowType,
    /// X 오프셋
    pub offset_x: HwpUnit,
    /// Y 오프셋
    pub offset_y: HwpUnit,
    /// 색상
    pub color: Color,
    /// 투명도
    pub alpha: u8,
    /// 흐림 (HWPX)
    pub blur: Option<HwpUnit>,
    /// 방향 (HWPX)
    pub direction: Option<f64>,
    /// 거리 (HWPX)
    pub distance: Option<HwpUnit>,
}

