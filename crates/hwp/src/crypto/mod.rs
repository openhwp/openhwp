//! Cryptographic utilities for HWP documents.
//!
//! This module handles decryption for:
//! - Password-encrypted documents
//! - Distribution documents

mod distribution;
mod password;

pub(crate) use distribution::decrypt_distribution_stream;
pub(crate) use password::decrypt_password_stream;
