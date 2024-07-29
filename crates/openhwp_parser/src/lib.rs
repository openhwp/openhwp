#![allow(clippy::all)]

#[macro_use]
extern crate thiserror;

#[cfg(test)]
mod tests;

pub mod doc_info;
pub mod document;
pub mod file_header;
pub mod section;

pub use doc_info::*;
pub use document::*;
pub use file_header::*;
pub use section::*;
