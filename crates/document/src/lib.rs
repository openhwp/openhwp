//! # Document Model for Rich Text Editor
//!
//! IR 기반 리치 텍스트 에디터용 문서 모델입니다.
//!
//! ## 설계 원칙
//!
//! - **IR 완전 호환**: IR의 모든 기능을 Document로 표현 가능
//! - **편집 최적화**: 효율적인 삽입, 삭제, 수정 연산
//! - **안정적인 ID**: SlotMap 기반 ID로 삭제 후에도 유효
//! - **계층 분리**: Document, Layout, Paint, View 레이어 분리
//!
//! ## 사용 예시
//!
//! ```ignore
//! use document::Document;
//!
//! // IR에서 Document 생성
//! let ir_doc = ir::Document::default();
//! let doc = Document::from(ir_doc);
//!
//! // Document를 IR로 변환
//! let ir_doc: ir::Document = doc.into();
//! ```

#![deny(clippy::all)]

pub mod arena;
pub mod command;
pub mod control;
pub mod convert;
pub mod cursor;
pub mod document;
pub mod extensions;
pub mod history;
pub mod id;
pub mod paragraph;
pub mod run;
pub mod run_content;
pub mod section;
pub mod table;
pub mod transaction;

pub use arena::DocumentArena;
pub use command::{Command, CommandError, CommandResult, DeleteText, InsertText};
pub use control::Control;
pub use cursor::{Cursor, Position, Selection};
pub use document::Document;
pub use extensions::hwp::HwpExtension;
pub use extensions::hwpx::HwpxExtension;
pub use extensions::track_change::{TrackChange, TrackChangeManager, TrackChangeType};
pub use history::CommandHistory;
pub use id::*;
pub use paragraph::Paragraph;
pub use run::Run;
pub use run_content::RunContent;
pub use section::Section;
pub use table::{Table, TableCell, TableRow};
pub use transaction::{Transaction, TransactionBuilder};
