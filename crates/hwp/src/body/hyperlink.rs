//! Hyperlink parsing.
//!
//! Hyperlinks are field controls (%hlk) that link to URLs, files, or
//! locations within the document.
//!
//! ## Hyperlink Data Format
//!
//! Hyperlink data is stored in ControlData (HWPTAG_CTRL_DATA) using the Parameter Set format.
//! The target URL is stored as a string parameter.
//!
//! ## Instantiation Pattern
//!
//! This type uses **dynamic instantiation** rather than `from_reader()` because:
//! - Hyperlinks are a special case of Field control (`%hlk`)
//! - Target URL is extracted from ControlData's Parameter Set (item ID 0x4000)
//! - Link type is auto-detected from the URL scheme
//!
//! Use [`Hyperlink::from_control_data()`] for instantiation during parsing.

use super::control_data::{hyperlink_item_ids, ControlData};

/// Hyperlink target type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HyperlinkType {
    /// Link to a URL.
    #[default]
    Url,
    /// Link to a local file.
    File,
    /// Link to a location within the document (bookmark).
    Bookmark,
    /// Link to an email address.
    Email,
}

/// A hyperlink in the document.
#[derive(Debug, Clone, Default)]
pub struct Hyperlink {
    /// Target URL, file path, or bookmark name.
    target: String,
    /// Display text (if different from target).
    display_text: Option<String>,
    /// Link type.
    link_type: HyperlinkType,
}

impl Hyperlink {
    /// Creates a new hyperlink.
    pub fn new(target: String) -> Self {
        let link_type = Self::detect_type(&target);
        Self {
            target,
            display_text: None,
            link_type,
        }
    }

    /// Creates a hyperlink with display text.
    pub fn with_display_text(target: String, display_text: String) -> Self {
        let link_type = Self::detect_type(&target);
        Self {
            target,
            display_text: Some(display_text),
            link_type,
        }
    }

    /// Detects the hyperlink type from the target string.
    fn detect_type(target: &str) -> HyperlinkType {
        let lower = target.to_lowercase();
        if lower.starts_with("http://") || lower.starts_with("https://") {
            HyperlinkType::Url
        } else if lower.starts_with("mailto:") {
            HyperlinkType::Email
        } else if lower.starts_with("file://") || lower.contains('\\') || lower.contains('/') {
            HyperlinkType::File
        } else if lower.starts_with('#') {
            HyperlinkType::Bookmark
        } else {
            HyperlinkType::Url
        }
    }

    /// Returns the target URL or path.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Returns the display text.
    pub fn display_text(&self) -> Option<&str> {
        self.display_text.as_deref()
    }

    /// Returns the link type.
    pub const fn link_type(&self) -> HyperlinkType {
        self.link_type
    }

    /// Sets the target.
    pub fn set_target(&mut self, target: String) {
        self.link_type = Self::detect_type(&target);
        self.target = target;
    }

    /// Sets the display text.
    pub fn set_display_text(&mut self, text: Option<String>) {
        self.display_text = text;
    }

    /// Returns the effective display text (display_text if set, otherwise target).
    pub fn effective_text(&self) -> &str {
        self.display_text.as_deref().unwrap_or(&self.target)
    }

    /// Returns true if this is a URL link.
    pub const fn is_url(&self) -> bool {
        matches!(self.link_type, HyperlinkType::Url)
    }

    /// Returns true if this is an email link.
    pub const fn is_email(&self) -> bool {
        matches!(self.link_type, HyperlinkType::Email)
    }

    /// Returns true if this is a file link.
    pub const fn is_file(&self) -> bool {
        matches!(self.link_type, HyperlinkType::File)
    }

    /// Returns true if this is a bookmark link.
    pub const fn is_bookmark(&self) -> bool {
        matches!(self.link_type, HyperlinkType::Bookmark)
    }

    /// Creates a hyperlink from ControlData.
    ///
    /// This extracts the target URL from the parameter set.
    pub fn from_control_data(data: &ControlData) -> Self {
        let target = data
            .get_string(hyperlink_item_ids::TARGET)
            .map(|s| s.to_string())
            .unwrap_or_default();

        Self::new(target)
    }

    /// Updates hyperlink data from ControlData.
    pub fn update_from_control_data(&mut self, data: &ControlData) {
        if let Some(target) = data.get_string(hyperlink_item_ids::TARGET) {
            self.set_target(target.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyperlink_url() {
        let link = Hyperlink::new("https://example.com".to_string());
        assert_eq!(link.target(), "https://example.com");
        assert_eq!(link.link_type(), HyperlinkType::Url);
        assert!(link.is_url());
        assert!(!link.is_email());
    }

    #[test]
    fn test_hyperlink_email() {
        let link = Hyperlink::new("mailto:test@example.com".to_string());
        assert_eq!(link.link_type(), HyperlinkType::Email);
        assert!(link.is_email());
    }

    #[test]
    fn test_hyperlink_file() {
        let link = Hyperlink::new("C:\\Documents\\file.pdf".to_string());
        assert_eq!(link.link_type(), HyperlinkType::File);
        assert!(link.is_file());
    }

    #[test]
    fn test_hyperlink_bookmark() {
        let link = Hyperlink::new("#section1".to_string());
        assert_eq!(link.link_type(), HyperlinkType::Bookmark);
        assert!(link.is_bookmark());
    }

    #[test]
    fn test_hyperlink_with_display_text() {
        let link = Hyperlink::with_display_text(
            "https://example.com".to_string(),
            "Click here".to_string(),
        );
        assert_eq!(link.display_text(), Some("Click here"));
        assert_eq!(link.effective_text(), "Click here");
    }

    #[test]
    fn test_hyperlink_effective_text() {
        let link = Hyperlink::new("https://example.com".to_string());
        assert_eq!(link.effective_text(), "https://example.com");
    }
}
