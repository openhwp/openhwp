//! Forbidden characters (HWPTAG_FORBIDDEN_CHAR) parsing.
//!
//! Stores characters that should not appear at line start or end
//! (Korean typography rules for line breaking).

use crate::error::Result;
use crate::util::ByteReader;

/// Forbidden characters configuration.
///
/// Controls which characters cannot appear at certain positions
/// for proper Korean/CJK line breaking.
#[derive(Debug, Clone, Default)]
pub struct ForbiddenChar {
    /// Characters forbidden at line start (e.g., closing punctuation).
    line_start_chars: String,
    /// Characters forbidden at line end (e.g., opening brackets).
    line_end_chars: String,
}

impl ForbiddenChar {
    /// Returns characters forbidden at line start.
    pub fn line_start_chars(&self) -> &str {
        &self.line_start_chars
    }

    /// Sets characters forbidden at line start.
    pub fn set_line_start_chars(&mut self, chars: String) {
        self.line_start_chars = chars;
    }

    /// Returns characters forbidden at line end.
    pub fn line_end_chars(&self) -> &str {
        &self.line_end_chars
    }

    /// Sets characters forbidden at line end.
    pub fn set_line_end_chars(&mut self, chars: String) {
        self.line_end_chars = chars;
    }

    /// Checks if a character is forbidden at line start.
    pub fn is_forbidden_at_line_start(&self, ch: char) -> bool {
        self.line_start_chars.contains(ch)
    }

    /// Checks if a character is forbidden at line end.
    pub fn is_forbidden_at_line_end(&self, ch: char) -> bool {
        self.line_end_chars.contains(ch)
    }

    /// Parses forbidden characters from reader.
    ///
    /// Format (per HWP spec - HWPTAG_FORBIDDEN_CHAR):
    /// - WCHAR[]: Line start forbidden characters (length-prefixed)
    /// - WCHAR[]: Line end forbidden characters (length-prefixed)
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        // Read line start forbidden characters
        let line_start_chars = if !reader.is_empty() && reader.remaining() >= 2 {
            let len = reader.read_u16()? as usize;
            if len > 0 && reader.remaining() >= len * 2 {
                let mut chars = Vec::with_capacity(len);
                for _ in 0..len {
                    chars.push(reader.read_u16()?);
                }
                String::from_utf16_lossy(&chars)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Read line end forbidden characters
        let line_end_chars = if !reader.is_empty() && reader.remaining() >= 2 {
            let len = reader.read_u16()? as usize;
            if len > 0 && reader.remaining() >= len * 2 {
                let mut chars = Vec::with_capacity(len);
                for _ in 0..len {
                    chars.push(reader.read_u16()?);
                }
                String::from_utf16_lossy(&chars)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        Ok(Self {
            line_start_chars,
            line_end_chars,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forbidden_char_new() {
        let fc = ForbiddenChar::default();
        assert_eq!(fc.line_start_chars(), "");
        assert_eq!(fc.line_end_chars(), "");
    }

    #[test]
    fn test_forbidden_char_setters() {
        let mut fc = ForbiddenChar::default();
        fc.set_line_start_chars(")]}」』】".to_string());
        fc.set_line_end_chars("([{「『【".to_string());

        assert!(fc.is_forbidden_at_line_start(')'));
        assert!(fc.is_forbidden_at_line_start(']'));
        assert!(!fc.is_forbidden_at_line_start('('));

        assert!(fc.is_forbidden_at_line_end('('));
        assert!(fc.is_forbidden_at_line_end('['));
        assert!(!fc.is_forbidden_at_line_end(')'));
    }

    #[test]
    fn test_forbidden_char_korean() {
        let mut fc = ForbiddenChar::default();
        // Korean closing punctuation
        fc.set_line_start_chars("。、」』】".to_string());
        // Korean opening punctuation
        fc.set_line_end_chars("「『【".to_string());

        assert!(fc.is_forbidden_at_line_start('。'));
        assert!(fc.is_forbidden_at_line_end('「'));
    }
}
