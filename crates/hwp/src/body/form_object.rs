//! Form object parsing.
//!
//! Form objects are interactive elements like text fields, checkboxes,
//! and other form controls that can be embedded in the document.

use crate::error::Result;
use crate::util::ByteReader;

/// Form object type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormObjectType {
    /// Unknown or unsupported form type.
    #[default]
    Unknown,
    /// Text input field.
    TextField,
    /// Checkbox.
    CheckBox,
    /// Radio button.
    RadioButton,
    /// Combo box (dropdown).
    ComboBox,
    /// List box.
    ListBox,
    /// Button.
    Button,
}

impl FormObjectType {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::TextField,
            1 => Self::CheckBox,
            2 => Self::RadioButton,
            3 => Self::ComboBox,
            4 => Self::ListBox,
            5 => Self::Button,
            _ => Self::Unknown,
        }
    }
}

/// A form object in the document.
#[derive(Debug, Clone, Default)]
pub struct FormObject {
    /// Form object type.
    object_type: FormObjectType,
    /// Name of the form field.
    name: String,
    /// Default value.
    default_value: String,
    /// Tab order index.
    tab_order: u16,
}

impl FormObject {
    /// Creates a new form object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a form object with type and name.
    pub fn with_type_and_name(object_type: FormObjectType, name: String) -> Self {
        Self {
            object_type,
            name,
            default_value: String::new(),
            tab_order: 0,
        }
    }

    /// Returns the form object type.
    pub const fn object_type(&self) -> FormObjectType {
        self.object_type
    }

    /// Returns the name of the form field.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the default value.
    pub fn default_value(&self) -> &str {
        &self.default_value
    }

    /// Sets the default value.
    pub fn set_default_value(&mut self, value: String) {
        self.default_value = value;
    }

    /// Returns the tab order index.
    pub const fn tab_order(&self) -> u16 {
        self.tab_order
    }

    /// Sets the tab order index.
    pub fn set_tab_order(&mut self, order: u16) {
        self.tab_order = order;
    }

    /// Parses form object from reader.
    ///
    /// Format (variable length per HWP spec):
    /// - UINT32: properties
    /// - WCHAR[]: name (length-prefixed)
    /// - Various type-specific data
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u32()?;
        let object_type = FormObjectType::from_raw((properties & 0xFF) as u8);

        // Read name string (length-prefixed UTF-16)
        let name = if !reader.is_empty() {
            let name_len = reader.read_u16()? as usize;
            if name_len > 0 && reader.remaining() >= name_len * 2 {
                let mut chars = Vec::with_capacity(name_len);
                for _ in 0..name_len {
                    chars.push(reader.read_u16()?);
                }
                String::from_utf16_lossy(&chars)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Read default value if available
        let default_value = if !reader.is_empty() {
            let value_len = reader.read_u16()? as usize;
            if value_len > 0 && reader.remaining() >= value_len * 2 {
                let mut chars = Vec::with_capacity(value_len);
                for _ in 0..value_len {
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
            object_type,
            name,
            default_value,
            tab_order: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_object_type_from_raw() {
        assert_eq!(FormObjectType::from_raw(0), FormObjectType::TextField);
        assert_eq!(FormObjectType::from_raw(1), FormObjectType::CheckBox);
        assert_eq!(FormObjectType::from_raw(2), FormObjectType::RadioButton);
        assert_eq!(FormObjectType::from_raw(3), FormObjectType::ComboBox);
        assert_eq!(FormObjectType::from_raw(4), FormObjectType::ListBox);
        assert_eq!(FormObjectType::from_raw(5), FormObjectType::Button);
        assert_eq!(FormObjectType::from_raw(255), FormObjectType::Unknown);
    }

    #[test]
    fn test_form_object_new() {
        let form = FormObject::new();
        assert_eq!(form.object_type(), FormObjectType::Unknown);
        assert_eq!(form.name(), "");
        assert_eq!(form.default_value(), "");
        assert_eq!(form.tab_order(), 0);
    }

    #[test]
    fn test_form_object_with_type_and_name() {
        let form = FormObject::with_type_and_name(FormObjectType::TextField, "username".to_string());
        assert_eq!(form.object_type(), FormObjectType::TextField);
        assert_eq!(form.name(), "username");
    }

    #[test]
    fn test_form_object_setters() {
        let mut form = FormObject::new();
        form.set_default_value("default".to_string());
        form.set_tab_order(5);
        assert_eq!(form.default_value(), "default");
        assert_eq!(form.tab_order(), 5);
    }
}
