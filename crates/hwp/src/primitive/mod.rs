//! Primitive types used throughout the HWP format.
//!
//! This module defines the fundamental data types that correspond to
//! the HWP file format specification.

mod color;
mod record;
mod unit;
mod version;

pub use color::ColorReference;
pub use record::{RecordHeader, RecordTagId};
pub use unit::{HwpUnit, HwpUnit16, SignedHwpUnit};
pub use version::Version;
