//! File header parsing for HWP documents.
//!
//! The FileHeader stream contains document identification and properties.

mod file_header;
mod properties;

pub use file_header::FileHeader;
pub(crate) use properties::EncryptionVersion;
