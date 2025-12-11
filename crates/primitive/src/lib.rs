//! 한글 문서 형식의 기본 타입들
//!
//! HWP, HWPX, IR 크레이트 간 공유되는 primitive 타입들입니다.
//!
//! ## 모듈 구성
//!
//! - `color` - ARGB 색상
//! - `unit` - HwpUnit, Size, Point, Rect, Insets, Percent
//! - `panose` - PANOSE 1.0 글꼴 분류 시스템
//! - `id` - ID 참조 타입 (CharShapeId, ParaShapeId 등)
//! - `version` - HWP 문서 버전
//! - `spacing` - 여백/패딩
//! - `line_style` - 테두리/밑줄 선 스타일
//! - `tab` - 탭 정렬/채움선
//! - `style` - 스타일 종류
//! - `heading` - 문단 머리 종류
//! - `alignment` - 텍스트 정렬
//! - `line` - 선 스타일
//! - `arrow` - 화살표
//! - `fill` - 채우기
//! - `text_decoration` - 밑줄, 취소선, 강조점 등
//! - `numbering` - 번호 매기기
//! - `direction` - 방향
//! - `wrap` - 텍스트 감싸기
//! - `positioning` - 개체 위치
//! - `page` - 페이지 설정
//! - `image` - 이미지 효과
//! - `font` - 글꼴 속성
//! - `table` - 표 관련
//! - `field` - 필드 종류
//! - `line_break` - 줄 나눔
//! - `misc` - 기타

#![deny(clippy::all)]

pub mod alignment;
pub mod arrow;
pub mod color;
pub mod direction;
pub mod field;
pub mod fill;
pub mod font;
pub mod heading;
pub mod id;
pub mod image;
pub mod line;
pub mod line_break;
pub mod line_style;
pub mod misc;
pub mod numbering;
pub mod page;
pub mod panose;
pub mod positioning;
pub mod spacing;
pub mod style;
pub mod tab;
pub mod table;
pub mod text_decoration;
pub mod unit;
pub mod version;
pub mod wrap;

pub use alignment::{Alignment, HorizontalOffsetType, VerticalAlignment, VerticalOffsetType};
pub use arrow::{ArrowSize, ArrowType};
pub use color::Color;
pub use direction::{PageOrientation, TextDirection};
pub use field::{BinaryDataState, BinaryDataType, FieldType};
pub use fill::{FillAreaType, FillType, GradientType, ImageFillMode, PatternType};
pub use font::{FontFamilyType, FontLanguage, LanguageType};
pub use heading::HeadingType;
pub use id::{
    // Numeric IDs - Document Style
    BeginId,
    // String IDs
    BinaryDataId,
    BorderFillId,
    BorderTypeId,
    CharShapeId,
    FileId,
    FontId,
    ImageId,
    LinkListId,
    MasterPageId,
    MemoShapeId,
    NumberingId,
    OutlineShapeId,
    ParaShapeId,
    PropertyId,
    StyleId,
    SubjectId,
    TabDefId,
};
pub use image::{ImageEffect, ImageFlip};
pub use line::{LineCap, LineOutlineStyle, LineType, LineWrap};
pub use line_break::{LineBreakKorean, LineBreakLatin};
pub use line_style::{BorderLineStyle, LineStyle, UnderlineShape};
pub use misc::{CurveSegmentType, ParameterType, TextOffsetType, TrackChangeType};
pub use numbering::{
    EndnotePlacement, FootnotePlacement, NoteNumberPosition, NoteNumbering, NumberFormat,
};
pub use page::{
    BreakType, GutterPosition, HeaderFooterApplyTo, LineNumberRestartType, PageMargins,
    PageNumberPosition, PageStartsOn,
};
pub use panose::{
    ArmStyle, Contrast, FamilyType, Letterform, Midline, Panose, Proportion, SerifStyle,
    StrokeVariation, Weight, XHeight,
};
pub use positioning::{
    HeightRelativeTo, HorizontalRelativeTo, VerticalRelativeTo, WidthRelativeTo,
};
pub use spacing::Spacing;
pub use style::StyleType;
pub use tab::{TabLeader, TabType};
pub use table::{CenterLineType, SlashDiagonalType};
pub use text_decoration::{
    EmphasisType, OutlineType, ShadowType, StrikethroughType, UnderlinePosition, UnderlineType,
};
pub use unit::{HwpUnit, Insets, Percent, Point, Rect, Size};
pub use version::Version;
pub use wrap::{TextWrapSide, TextWrapType};
