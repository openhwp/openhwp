//! Error types for HWP parsing.

use std::fmt;

/// Result type alias for HWP operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when parsing HWP documents.
#[derive(Debug)]
pub enum Error {
    /// I/O error occurred while reading the file.
    Io(std::io::Error),

    /// The file does not have a valid HWP signature.
    InvalidSignature,

    /// The HWP version is not supported.
    UnsupportedVersion {
        /// Major version number.
        major: u8,
        /// Minor version number.
        minor: u8,
        /// Build version number.
        build: u8,
        /// Revision version number.
        revision: u8,
    },

    /// The file is encrypted and no password was provided.
    EncryptedDocument,

    /// The provided password is incorrect.
    InvalidPassword,

    /// The encryption version is not supported.
    UnsupportedEncryptionVersion(u32),

    /// A required stream is missing from the compound file.
    MissingStream {
        /// Name of the missing stream.
        name: String,
    },

    /// Invalid record header encountered.
    InvalidRecordHeader {
        /// Offset where the invalid header was found.
        offset: u64,
    },

    /// Unknown record tag encountered.
    UnknownRecordTag {
        /// The unknown tag ID.
        tag_id: u16,
        /// Offset where the unknown tag was found.
        offset: u64,
    },

    /// Record data is malformed or truncated.
    MalformedRecord {
        /// Description of what went wrong.
        description: String,
    },

    /// Decompression failed.
    DecompressionFailed {
        /// Description of the decompression error.
        description: String,
    },

    /// Invalid UTF-16 string data.
    InvalidUtf16String,

    /// Unexpected end of data while parsing.
    UnexpectedEndOfData {
        /// Expected number of bytes.
        expected: usize,
        /// Actual number of bytes available.
        actual: usize,
    },

    /// Invalid control ID encountered.
    InvalidControlId(u32),

    /// Reference to non-existent ID mapping.
    InvalidIdReference {
        /// Type of reference (e.g., "font", "style").
        reference_type: String,
        /// The invalid ID.
        id: u32,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "I/O error: {}", err),
            Error::InvalidSignature => write!(f, "Invalid HWP signature"),
            Error::UnsupportedVersion {
                major,
                minor,
                build,
                revision,
            } => write!(
                f,
                "Unsupported HWP version: {}.{}.{}.{}",
                major, minor, build, revision
            ),
            Error::EncryptedDocument => write!(f, "Document is encrypted"),
            Error::InvalidPassword => write!(f, "Invalid password"),
            Error::UnsupportedEncryptionVersion(version) => {
                write!(f, "Unsupported encryption version: {}", version)
            }
            Error::MissingStream { name } => {
                write!(f, "Missing stream: {}", name)
            }
            Error::InvalidRecordHeader { offset } => {
                write!(f, "Invalid record header at offset {}", offset)
            }
            Error::UnknownRecordTag { tag_id, offset } => {
                write!(f, "Unknown record tag {} at offset {}", tag_id, offset)
            }
            Error::MalformedRecord { description } => {
                write!(f, "Malformed record: {}", description)
            }
            Error::DecompressionFailed { description } => {
                write!(f, "Decompression failed: {}", description)
            }
            Error::InvalidUtf16String => write!(f, "Invalid UTF-16 string"),
            Error::UnexpectedEndOfData { expected, actual } => {
                write!(
                    f,
                    "Unexpected end of data: expected {} bytes, got {}",
                    expected, actual
                )
            }
            Error::InvalidControlId(id) => {
                write!(f, "Invalid control ID: 0x{:08X}", id)
            }
            Error::InvalidIdReference { reference_type, id } => {
                write!(f, "Invalid {} reference: {}", reference_type, id)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
