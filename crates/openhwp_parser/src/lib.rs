#![allow(clippy::all)]
#![allow(dead_code)]

#[macro_use]
extern crate thiserror;

#[cfg(test)]
mod tests;

mod bytes;

pub mod doc_info;
pub mod document;
pub mod file_header;
pub mod section;

use bytes::*;

pub use doc_info::*;
pub use document::*;
pub use file_header::*;
pub use section::*;
