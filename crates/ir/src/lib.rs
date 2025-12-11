//! # IR (Intermediate Representation)
//!
//! HWP와 HWPX 문서 형식 간의 변환을 위한 중간 표현입니다.
//!
//! ## 개요
//!
//! IR은 확장자에 독립적인 문서 모델로, 다양한 한글 문서 형식(HWP, HWPX)을
//! 통합된 방식으로 다룰 수 있게 합니다.
//!
//! ## 설계 원칙
//!
//! - **형식 독립성**: HWP, HWPX 등 특정 형식에 의존하지 않음
//! - **완전한 표현**: 양쪽 형식의 모든 개념을 표현 가능
//! - **확장 가능**: 형식별 고유 기능을 위한 확장 필드 제공
//! - **단위 통합**: HwpUnit 기반 단일 단위 시스템
//!
//! ## 사용 예시
//!
//! ```ignore
//! // hwp 크레이트에서
//! let hwp_doc = hwp::HwpDocument::from_bytes(&bytes)?;
//! let ir_doc = ir::Document::try_from(hwp_doc)?;
//!
//! // hwpx 크레이트로 변환
//! let hwpx_doc = hwpx::Document::try_from(ir_doc)?;
//! ```

#![deny(clippy::all)]

pub mod binary;
pub mod border_fill;
pub mod char_shape;
pub mod control;
pub mod document;
pub mod error;
pub mod extensions;
pub mod metadata;
pub mod para_shape;
pub mod paragraph;
pub mod picture;
pub mod section;
pub mod shape;
pub mod style;
pub mod table;

// Re-exports from internal modules
pub use binary::{BinaryData, BinaryDataStore, BinaryFormat};
pub use char_shape::{
    CharShape, EmphasisStyle, Font, FontFamily, FontRef, FontSet, FontType, ShadowStyle,
    SubstituteFont, UnderlineStyle,
};
pub use document::{CaretPosition, CompatibleDocument, Document, DocumentSettings};
pub use error::{
    ConversionError, ConversionErrorKind, ConversionResult, ConversionWarning,
    ConversionWarningKind, WarningCollector,
};
pub use extensions::{Extensions, HwpExtensions, HwpxExtensions};
pub use metadata::{DocumentVersion, Metadata};
pub use paragraph::{Paragraph, Run, RunContent};
pub use section::Section;
pub use style::StyleStore;

// Re-exports from primitive crate
pub use primitive::{
    // Color
    Color,
    // Unit types
    HwpUnit, Insets, Percent, Point, Rect, Size,
    // ID types
    BinaryDataId, BorderFillId, CharShapeId, FontId, NumberingId, ParaShapeId, StyleId, TabDefId,
    // Font
    Panose,
    // Shared enums (for convenience)
    HeadingType, StyleType, TabLeader, TabType,
};
