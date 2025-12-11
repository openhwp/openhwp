//! Field control parsing.
//!
//! Fields are special controls that display dynamic content like
//! dates, page numbers, file paths, etc.
//!
//! ## Field Data Format
//!
//! Field data is stored in ControlData (HWPTAG_CTRL_DATA) using the Parameter Set format.
//! The field type is determined by the control ID in ControlHeader.
//!
//! ## Instantiation Pattern
//!
//! This type uses **dynamic instantiation** rather than `from_reader()` because:
//! - Field type is determined by ControlHeader's control ID (e.g., `%dat`, `%pn`)
//! - Field instruction/value comes from ControlData's Parameter Set
//! - Both pieces of context are needed to construct a complete Field
//!
//! Use [`Field::from_control_id_and_data()`] for instantiation during parsing.

use super::control_data::{field_item_ids, ControlData};

/// Field type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FieldType {
    /// Unknown field type.
    #[default]
    Unknown,
    /// Date field (%dat).
    Date,
    /// Time field (%tim).
    Time,
    /// File path field (%fil).
    FilePath,
    /// Document title field (%tit).
    DocTitle,
    /// Author field (%aut).
    Author,
    /// Page number field (%pn).
    PageNumber,
    /// Total pages field.
    TotalPages,
    /// Click here field (%clk).
    ClickHere,
    /// Summary field (%smr).
    Summary,
    /// User info field (%usr).
    UserInfo,
    /// Hyperlink field (%hlk).
    Hyperlink,
    /// Cross reference field.
    CrossReference,
    /// Formula field.
    Formula,
    /// Memo field.
    Memo,
    /// Private info field.
    PrivateInfo,
    /// Meta tag field.
    MetaTag,
    /// Mail merge field.
    MailMerge,
    /// Table of contents field.
    TableOfContents,
}

impl FieldType {
    /// Creates from a control ID string.
    pub fn from_control_id(id: &[u8; 4]) -> Self {
        match id {
            b"%dat" => Self::Date,
            b"%tim" => Self::Time,
            b"%fil" => Self::FilePath,
            b"%tit" => Self::DocTitle,
            b"%aut" => Self::Author,
            b"%pn " => Self::PageNumber,
            b"%clk" => Self::ClickHere,
            b"%smr" => Self::Summary,
            b"%usr" => Self::UserInfo,
            b"%hlk" => Self::Hyperlink,
            b"%xrf" => Self::CrossReference,
            b"%frm" => Self::Formula,
            b"%mem" => Self::Memo,
            b"%prv" => Self::PrivateInfo,
            b"%mtg" => Self::MetaTag,
            b"%mmr" => Self::MailMerge,
            b"%toc" => Self::TableOfContents,
            _ => Self::Unknown,
        }
    }

    /// Returns true if this is a dynamic field (changes during document rendering).
    pub const fn is_dynamic(&self) -> bool {
        matches!(
            self,
            Self::Date | Self::Time | Self::PageNumber | Self::TotalPages
        )
    }
}

/// A field control in the document.
#[derive(Debug, Clone, Default)]
pub struct Field {
    /// Field type.
    field_type: FieldType,
    /// Field instruction (format string or command).
    instruction: String,
    /// Current value (resolved text).
    value: String,
}

impl Field {
    /// Creates a new field.
    pub fn new(field_type: FieldType) -> Self {
        Self {
            field_type,
            instruction: String::new(),
            value: String::new(),
        }
    }

    /// Creates a field with instruction.
    pub fn with_instruction(field_type: FieldType, instruction: String) -> Self {
        Self {
            field_type,
            instruction,
            value: String::new(),
        }
    }

    /// Returns the field type.
    pub const fn field_type(&self) -> FieldType {
        self.field_type
    }

    /// Returns the instruction string.
    pub fn instruction(&self) -> &str {
        &self.instruction
    }

    /// Returns the current value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Sets the instruction.
    pub fn set_instruction(&mut self, instruction: String) {
        self.instruction = instruction;
    }

    /// Sets the value.
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    /// Returns the display text (value if set, otherwise instruction).
    pub fn display_text(&self) -> &str {
        if self.value.is_empty() {
            &self.instruction
        } else {
            &self.value
        }
    }

    /// Updates field data from ControlData.
    ///
    /// This extracts the instruction and name from the parameter set.
    pub fn update_from_control_data(&mut self, data: &ControlData) {
        // Extract instruction/command from item ID 0x4000
        if let Some(instruction) = data.get_string(field_item_ids::COMMAND) {
            self.instruction = instruction.to_string();
        }

        // Extract field name from item ID 0x4001 (if present)
        if let Some(name) = data.get_string(field_item_ids::NAME) {
            // Field name can be used as value for display
            if self.value.is_empty() {
                self.value = name.to_string();
            }
        }
    }

    /// Creates a field from a control ID and optional control data.
    pub fn from_control_id_and_data(control_id: &[u8; 4], data: Option<&ControlData>) -> Self {
        let mut field = Self::new(FieldType::from_control_id(control_id));

        if let Some(ctrl_data) = data {
            field.update_from_control_data(ctrl_data);
        }

        field
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_type_from_control_id() {
        assert_eq!(FieldType::from_control_id(b"%dat"), FieldType::Date);
        assert_eq!(FieldType::from_control_id(b"%tim"), FieldType::Time);
        assert_eq!(FieldType::from_control_id(b"%pn "), FieldType::PageNumber);
        assert_eq!(FieldType::from_control_id(b"%hlk"), FieldType::Hyperlink);
        assert_eq!(FieldType::from_control_id(b"abcd"), FieldType::Unknown);
    }

    #[test]
    fn test_field_type_is_dynamic() {
        assert!(FieldType::Date.is_dynamic());
        assert!(FieldType::PageNumber.is_dynamic());
        assert!(!FieldType::DocTitle.is_dynamic());
        assert!(!FieldType::Author.is_dynamic());
    }

    #[test]
    fn test_field_new() {
        let field = Field::new(FieldType::Date);
        assert_eq!(field.field_type(), FieldType::Date);
        assert!(field.instruction().is_empty());
        assert!(field.value().is_empty());
    }

    #[test]
    fn test_field_with_instruction() {
        let field = Field::with_instruction(FieldType::Date, "yyyy-MM-dd".to_string());
        assert_eq!(field.instruction(), "yyyy-MM-dd");
    }

    #[test]
    fn test_field_display_text() {
        let mut field = Field::new(FieldType::Date);
        field.set_instruction("yyyy-MM-dd".to_string());
        assert_eq!(field.display_text(), "yyyy-MM-dd");

        field.set_value("2024-01-15".to_string());
        assert_eq!(field.display_text(), "2024-01-15");
    }
}
