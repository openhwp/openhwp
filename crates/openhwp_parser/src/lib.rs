#![allow(clippy::all)]

#[macro_use]
extern crate thiserror;

#[cfg(test)]
mod tests;

mod bytes;

pub mod body;
pub mod doc_info;
pub mod document;
pub mod domain;
pub mod file_header;
pub mod reader;
pub mod record;
pub mod tag;

use bytes::*;

pub use body::*;
pub use doc_info::*;
pub use document::*;
pub use domain::*;
pub use file_header::*;
pub use reader::*;
pub use record::*;
pub use tag::*;

#[inline]
pub fn from_path(path: &str) -> Result<HwpDocument, HwpDocumentError> {
    HwpDocument::from_path(path)
}

#[inline]
pub fn from_reader<R: HwpRead>(reader: &mut R) -> Result<HwpDocument, HwpDocumentError> {
    HwpDocument::from_reader(reader)
}
