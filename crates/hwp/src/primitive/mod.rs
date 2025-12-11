//! Primitive types used throughout the HWP format.
//!
//! This module defines the fundamental data types that correspond to
//! the HWP file format specification.

mod color;
mod record;
mod spacing;
mod unit;

pub use color::ColorReference;
pub use record::{RecordHeader, RecordTagId};
pub use spacing::{CellPadding, TablePadding};
pub use unit::HwpUnit16;
