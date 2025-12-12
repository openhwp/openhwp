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
//! - `fill` - 채우기 (FillType, Fill, SolidFill, GradientFill 등)
//! - `border` - 테두리 (Border, DiagonalType)
//! - `shape` - 도형 (ShapeType, ArcType, FormObjectType 등)
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
//! - `form` - 양식 객체
//! - `paragraph` - 문단 관련
//! - `equation` - 수식 관련
//! - `text_art` - 글맵시 관련
//! - `memo` - 메모 관련
//! - `geometry` - 기하학 관련
//! - `compatibility` - 호환성 관련
//! - `char_style` - 글자 스타일 관련
//! - `section` - 섹션/구역 관련
//! - `note` - 각주/미주 관련
//! - `object` - 개체 공통 관련
//! - `misc` - 기타

#![deny(clippy::all)]

pub mod alignment;
pub mod arrow;
pub mod border;
pub mod char_style;
pub mod color;
pub mod compatibility;
pub mod direction;
pub mod equation;
pub mod field;
pub mod fill;
pub mod font;
pub mod form;
pub mod geometry;
pub mod heading;
pub mod id;
pub mod image;
pub mod line;
pub mod line_break;
pub mod line_style;
pub mod memo;
pub mod misc;
pub mod note;
pub mod numbering;
pub mod object;
pub mod page;
pub mod panose;
pub mod paragraph;
pub mod positioning;
pub mod section;
pub mod shape;
pub mod spacing;
pub mod style;
pub mod tab;
pub mod table;
pub mod text_art;
pub mod text_decoration;
pub mod unit;
pub mod version;
pub mod wrap;

pub use alignment::{Alignment, HorizontalOffsetType, VerticalAlignment, VerticalOffsetType};
pub use arrow::{ArrowSize, ArrowType};
pub use border::{Border, DiagonalType};
pub use char_style::{
    CharShadowStyle, EmphasisStyle, FontFamily, FontType, SubstituteFont, UnderlineStyle,
};
pub use color::Color;
pub use compatibility::{
    ChangeHistoryEntry, DistributeDocData, HwpxDocumentOption, HwpxLayoutCompatibility,
    HwpxTargetProgram, LayoutCompatibility, PresentationSettings, TrackChangeConfig,
};
pub use direction::{PageOrientation, TextDirection};
pub use equation::EquationLineMode;
pub use field::{BinaryDataState, BinaryDataType, FieldType};
pub use fill::{
    Fill, FillAreaType, FillType, GradientFill, GradientStop, GradientType, ImageFill,
    ImageFillMode, PatternFill, PatternType, SolidFill,
};
pub use font::{FontFamilyType, FontLanguage, LanguageType};
pub use form::{
    ButtonBackStyle, ButtonValue, EditScrollBars, EditTabKeyBehavior, FormCharProperty,
    FormListItem, ScrollBarType,
};
pub use geometry::{
    Arrow, ConnectorPoint, CurvePoint, CurvePointKind, ImageCrop, RectangleCorner, TransformMatrix,
};
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
pub use memo::MemoType;
pub use misc::{
    AutoNumberType, CaptionPosition, CurveSegmentType, FontStyle, ObjectNumberingType,
    ParameterType, ShapeShadowType, TextOffsetType, TrackChangeType,
};
pub use note::{NoteNumberPositionType, NoteSeparatorLine, NoteShapeBase};
pub use numbering::{
    EndnotePlacement, FootnotePlacement, NoteNumberPosition, NoteNumbering, NumberFormat,
};
pub use object::{EditTextAlignment, ObjectMargin, TextArtFontStyle, TextArtShapeType};
pub use page::{
    BreakType, GutterPosition, HeaderFooterApplyTo, LineNumberRestartType, PageMargins,
    PageNumberPosition, PageStartsOn,
};
pub use panose::{
    ArmStyle, Contrast, FamilyType, Letterform, Midline, Panose, Proportion, SerifStyle,
    StrokeVariation, Weight, XHeight,
};
pub use paragraph::{
    AutoNumberFormat, LineSpacingType, ParagraphBorder, ParagraphNumbering, Tab, TabDef,
    TablePageBreak,
};
pub use positioning::{
    HeightRelativeTo, HorizontalRelativeTo, VerticalRelativeTo, WidthRelativeTo,
};
pub use section::{
    ColumnDirection, ColumnSeparator, ColumnSeparatorType, ColumnType, LineNumberShape,
    PageBorderFillArea, PageBorderPageType, PageBorderPosition, SectionGrid, SectionStartNumber,
    SectionVisibility, VisibilityOption,
};
pub use shape::{
    ArcType, ChartType, CircleType, ComposeType, ConnectorType, CurvePointType, DutmalPosition,
    EquationFormat, FormObjectType, HyperlinkTarget, ShapeType, TextArtShape, TextBoxDirection,
    VideoType,
};
pub use spacing::Spacing;
pub use style::StyleType;
pub use tab::{TabLeader, TabType};
pub use table::{CenterLineType, SlashDiagonalType};
pub use text_art::{TextArtAlignment, TextArtFontType, TextArtProperties};
pub use text_decoration::{
    EmphasisType, OutlineType, ShadowType, StrikethroughType, UnderlinePosition, UnderlineType,
};
pub use unit::{HwpUnit, Insets, Percent, Point, Rect, Size};
pub use version::Version;
pub use wrap::{TextWrapSide, TextWrapType};
