//! # HWP 5.0 파서 라이브러리
//!
//! HWP (한글 워드프로세서) 5.0 문서 파일을 파싱하기 위한 Rust 라이브러리.
//!
//! ## 개요
//!
//! 이 라이브러리는 한글 2002 이후 버전에서 사용하는 HWP 5.0 바이너리 형식의
//! 문서를 읽고 파싱하는 기능을 제공합니다.
//!
//! ## 기능
//!
//! - HWP 5.0 형식 완전 지원
//! - 암호화된 문서 지원
//! - 배포용 문서 지원
//! - 텍스트 및 이미지 추출 API
//!
//! ## 사용 예시
//!
//! ```ignore
//! use hwp::HwpDocument;
//!
//! let bytes = std::fs::read("document.hwp")?;
//! let document = HwpDocument::from_bytes(&bytes)?;
//!
//! // 문서 속성 접근
//! println!("버전: {}", document.header().version());
//!
//! // 텍스트 추출
//! let text = document.extract_text();
//! println!("{}", text);
//! ```
//!
//! ## 주요 타입
//!
//! - [`HwpDocument`]: 문서 전체를 나타내는 최상위 타입
//! - [`FileHeader`]: 파일 헤더 정보 (버전, 암호화 여부 등)
//! - [`DocInfo`]: 문서 정보 (폰트, 스타일, 문단 모양 등)
//! - [`Section`] - 섹션 (본문 내용)
//! - [`Paragraph`] - 문단
//!
//! ## HWP 5.0 형식 구조
//!
//! HWP 5.0 파일은 OLE(CFB) 컨테이너 형식을 사용하며, 다음과 같은 스트림을 포함합니다:
//!
//! - `FileHeader`: 문서 식별 및 속성 정보
//! - `DocInfo`: 문서 수준 정보 (폰트, 스타일, 번호 매김 등)
//! - `BodyText/SectionN`: 본문 내용 (섹션별로 분리)
//! - `BinData`: 바이너리 데이터 (이미지 등)
//! - `PrvText`, `PrvImage`: 미리보기 텍스트 및 이미지

pub mod error;

mod body;
mod crypto;
mod doc_info;
mod doc_options;
mod document;
mod header;
mod preview;
mod primitive;
mod script;
mod summary;
mod util;

// 핵심 API 타입 재export
pub use document::HwpDocument;
pub use error::{Error, Result};

// 주요 타입 재export
pub use body::Section;
pub use doc_info::DocInfo;
pub use header::FileHeader;
pub use primitive::Version;

// 본문(Body) 타입 - HWP 스펙에 정의된 타입들
pub use body::{
    // 컨트롤 타입 (표, 그림 등 특수 개체)
    Control, ControlCharacter, ControlContent, ControlId, ControlType,
    // 수식 타입
    Equation, EquationLineMode, EquationProperties,
    // 필드 컨트롤
    Field, FieldType,
    // 각주/미주
    Endnote, EndnoteShape, Footnote, FootnoteShape, NotePlacement, NoteNumberingType,
    // 머리글/꼬리글
    Footer, Header, HeaderFooterTarget,
    // 하이퍼링크
    Hyperlink, HyperlinkType,
    // 리스트 헤더 (중첩 콘텐츠 컨테이너)
    ListHeader, TextDirection,
    // 페이지 설정
    GutterPosition, PageBorderFill, PageBorderFillPosition, PageDefinition, PageMargins, PageOrientation,
    // 문단 타입
    BreakType, CharacterShapeReference, LineSegment, Paragraph, RangeTag,
    // 그림 타입
    ImageCrop, ImageFlip, InnerMargin, OleObject, Picture, PictureEffect, PictureFill,
    PictureProperties,
    // 도형 타입
    ArcShape, ArcType, ArrowSize, ArrowType, CurveSegmentType, CurveShape, EllipseShape,
    LineEndCap, LineShape, Point, PolygonShape, RectangleShape, Shape, ShapeBorderLine,
    ShapeElementProperties, ShapeType,
    // 표 타입
    Table, TableCell, TableProperties,
    // 텍스트 박스 및 캡션
    Caption, CaptionDirection, TextBox, VerticalAlignment,
    // 추가 컨트롤 타입
    ChartData, ChartSeries, ChartType, FormObject, FormObjectType, Memo, MemoShape,
    ShapeContainer, TextArt, TextArtAlignment, TextArtShape, VideoData, VideoType,
};

// 문서 정보(DocInfo) 타입 - HWP 스펙에 정의된 타입들
pub use doc_info::{
    // 바이너리 데이터 (이미지 등)
    BinaryData,
    // 테두리/채우기 타입
    BorderFill, BorderLineStyle, BorderLineThickness, DiagonalType, FillInfo, FillType,
    GradientFill, GradientType, ImageFill, ImageFillType, ImageInfo, PatternFill, PatternType,
    // 글머리표
    Bullet,
    // 글자 모양 타입
    CharacterShape, EmphasisType, LanguageType, OutlineType, ShadowType, StrikethroughShape,
    UnderlinePosition, UnderlineShape,
    // 문서 속성
    CompatibleDocument, DocumentProperties, LayoutCompatibility,
    // 글꼴 이름
    FaceName,
    // ID 매핑
    IdMappings,
    // 번호 매김 타입
    Numbering, NumberingLevel, ParagraphHeadAlignment, ParagraphHeadInfo,
    // 문단 모양
    ParagraphShape,
    // 스타일
    Style,
    // 탭 정의
    TabDefinition,
};

// 미리보기 및 요약 정보 타입
pub use preview::{PreviewFormat, PreviewImage, PreviewText};
pub use script::{ScriptHeader, ScriptSource, ScriptVersion, Scripts};
pub use summary::SummaryInfo;

// 문서 옵션 타입
pub use doc_options::{DocOptions, DrmLicense, LinkDoc};

// 추가 DocInfo 타입 (변경 추적, 배포용 문서 등)
pub use doc_info::{
    DistributeDocData, DocumentData, ForbiddenChar, TrackChangeAuthor, TrackChangeContent,
    TrackChangeInfo,
};
