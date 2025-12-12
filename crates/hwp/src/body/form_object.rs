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
    /// Raw properties (bit flags from HWP format).
    properties: u32,
    /// Additional type-specific properties parsed from the binary data.
    extended_properties: ExtendedProperties,
}

/// Extended properties for different form object types.
#[derive(Debug, Clone, Default)]
pub struct ExtendedProperties {
    // Button properties
    pub caption: Option<String>,
    pub radio_group_name: Option<String>,
    pub back_style: Option<u8>,
    pub back_color: Option<u32>,
    pub tri_state: bool,
    pub gradient_fill: bool,
    pub image_fill: bool,

    // Edit properties
    pub multiline: bool,
    pub password_char: Option<u16>,
    pub max_length: Option<u32>,
    pub scroll_bars: Option<u8>,
    pub tab_key_behavior: Option<u8>,
    pub num_only: bool,
    pub read_only: bool,
    pub alignment: Option<u8>,

    // ComboBox/ListBox properties
    pub edit_enable: bool,
    pub items_text: Vec<String>,
    pub items_value: Vec<String>,
    pub selected_value: Option<String>,
    pub list_box_rows: Option<i32>,
    pub list_box_width: Option<i32>,
    pub item_height: Option<i32>,
    pub top_index: Option<u32>,

    // ScrollBar properties
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub value: Option<i32>,
    pub small_change: Option<u32>,
    pub large_change: Option<u32>,
    pub page: Option<i32>,
    pub delay: Option<u32>,
}

impl FormObject {
    /// Creates a form object with type and name.
    pub fn with_type_and_name(object_type: FormObjectType, name: String) -> Self {
        Self {
            object_type,
            name,
            default_value: String::new(),
            tab_order: 0,
            properties: 0,
            extended_properties: ExtendedProperties::default(),
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

    /// Returns the raw properties.
    pub const fn properties(&self) -> u32 {
        self.properties
    }

    /// Returns the extended properties.
    pub fn extended_properties(&self) -> &ExtendedProperties {
        &self.extended_properties
    }

    /// Returns mutable extended properties.
    pub fn extended_properties_mut(&mut self) -> &mut ExtendedProperties {
        &mut self.extended_properties
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

        // Parse extended properties based on form type
        let extended_properties = Self::parse_extended_properties(object_type, properties, reader)?;

        Ok(Self {
            object_type,
            name,
            default_value,
            tab_order: 0,
            properties,
            extended_properties,
        })
    }

    /// Parses extended properties based on form object type.
    fn parse_extended_properties(
        object_type: FormObjectType,
        properties: u32,
        reader: &mut ByteReader,
    ) -> Result<ExtendedProperties> {
        let mut ext = ExtendedProperties::default();

        match object_type {
            FormObjectType::Button | FormObjectType::CheckBox | FormObjectType::RadioButton => {
                // Button-specific properties
                // Parse caption if available
                if !reader.is_empty() {
                    let caption_len = reader.read_u16()? as usize;
                    if caption_len > 0 && reader.remaining() >= caption_len * 2 {
                        let mut chars = Vec::with_capacity(caption_len);
                        for _ in 0..caption_len {
                            chars.push(reader.read_u16()?);
                        }
                        ext.caption = Some(String::from_utf16_lossy(&chars));
                    }
                }

                // Parse button properties from flags
                ext.tri_state = (properties & 0x0100) != 0;
                ext.gradient_fill = (properties & 0x0200) != 0;
                ext.image_fill = (properties & 0x0400) != 0;

                // Parse back_style (bits 8-9)
                let back_style_val = ((properties >> 10) & 0x03) as u8;
                if back_style_val > 0 {
                    ext.back_style = Some(back_style_val);
                }

                // Parse back_color if available
                if !reader.is_empty() && reader.remaining() >= 4 {
                    ext.back_color = Some(reader.read_u32()?);
                }

                // Parse radio group name for RadioButton
                if object_type == FormObjectType::RadioButton && !reader.is_empty() {
                    let group_len = reader.read_u16()? as usize;
                    if group_len > 0 && reader.remaining() >= group_len * 2 {
                        let mut chars = Vec::with_capacity(group_len);
                        for _ in 0..group_len {
                            chars.push(reader.read_u16()?);
                        }
                        ext.radio_group_name = Some(String::from_utf16_lossy(&chars));
                    }
                }
            }
            FormObjectType::TextField => {
                // Edit-specific properties
                ext.multiline = (properties & 0x0100) != 0;
                ext.num_only = (properties & 0x0200) != 0;
                ext.read_only = (properties & 0x0400) != 0;

                // Parse password char
                if !reader.is_empty() && reader.remaining() >= 2 {
                    let pwd_char = reader.read_u16()?;
                    if pwd_char > 0 {
                        ext.password_char = Some(pwd_char);
                    }
                }

                // Parse max length
                if !reader.is_empty() && reader.remaining() >= 4 {
                    let max_len = reader.read_u32()?;
                    if max_len > 0 {
                        ext.max_length = Some(max_len);
                    }
                }

                // Parse scroll bars (bits 12-13)
                let scroll_bars_val = ((properties >> 12) & 0x03) as u8;
                if scroll_bars_val > 0 {
                    ext.scroll_bars = Some(scroll_bars_val);
                }

                // Parse tab key behavior (bit 14)
                let tab_key_val = ((properties >> 14) & 0x01) as u8;
                ext.tab_key_behavior = Some(tab_key_val);

                // Parse alignment (bits 16-17)
                let alignment_val = ((properties >> 16) & 0x03) as u8;
                ext.alignment = Some(alignment_val);
            }
            FormObjectType::ComboBox | FormObjectType::ListBox => {
                // ComboBox/ListBox properties
                ext.edit_enable = (properties & 0x0100) != 0;

                // Parse selected value
                if !reader.is_empty() {
                    let sel_len = reader.read_u16()? as usize;
                    if sel_len > 0 && reader.remaining() >= sel_len * 2 {
                        let mut chars = Vec::with_capacity(sel_len);
                        for _ in 0..sel_len {
                            chars.push(reader.read_u16()?);
                        }
                        ext.selected_value = Some(String::from_utf16_lossy(&chars));
                    }
                }

                // Parse list box rows
                if !reader.is_empty() && reader.remaining() >= 4 {
                    ext.list_box_rows = Some(reader.read_i32()?);
                }

                // Parse list box width
                if !reader.is_empty() && reader.remaining() >= 4 {
                    ext.list_box_width = Some(reader.read_i32()?);
                }

                // Parse item height
                if !reader.is_empty() && reader.remaining() >= 4 {
                    ext.item_height = Some(reader.read_i32()?);
                }

                // Parse top index
                if !reader.is_empty() && reader.remaining() >= 4 {
                    ext.top_index = Some(reader.read_u32()?);
                }

                // Parse items (text and value pairs)
                if !reader.is_empty() && reader.remaining() >= 2 {
                    let item_count = reader.read_u16()? as usize;
                    for _ in 0..item_count {
                        // Parse item text
                        if !reader.is_empty() {
                            let text_len = reader.read_u16()? as usize;
                            if text_len > 0 && reader.remaining() >= text_len * 2 {
                                let mut chars = Vec::with_capacity(text_len);
                                for _ in 0..text_len {
                                    chars.push(reader.read_u16()?);
                                }
                                ext.items_text.push(String::from_utf16_lossy(&chars));
                            } else {
                                ext.items_text.push(String::new());
                            }
                        }

                        // Parse item value
                        if !reader.is_empty() {
                            let value_len = reader.read_u16()? as usize;
                            if value_len > 0 && reader.remaining() >= value_len * 2 {
                                let mut chars = Vec::with_capacity(value_len);
                                for _ in 0..value_len {
                                    chars.push(reader.read_u16()?);
                                }
                                ext.items_value.push(String::from_utf16_lossy(&chars));
                            } else {
                                ext.items_value.push(String::new());
                            }
                        }
                    }
                }
            }
            FormObjectType::Unknown => {
                // ScrollBar properties (assuming type 6 based on ScrollBar enum)
                // Parse min
                if !reader.is_empty() && reader.remaining() >= 4 {
                    ext.min = Some(reader.read_i32()?);
                }

                // Parse max
                if !reader.is_empty() && reader.remaining() >= 4 {
                    ext.max = Some(reader.read_i32()?);
                }

                // Parse value
                if !reader.is_empty() && reader.remaining() >= 4 {
                    ext.value = Some(reader.read_i32()?);
                }

                // Parse small_change
                if !reader.is_empty() && reader.remaining() >= 4 {
                    ext.small_change = Some(reader.read_u32()?);
                }

                // Parse large_change
                if !reader.is_empty() && reader.remaining() >= 4 {
                    ext.large_change = Some(reader.read_u32()?);
                }

                // Parse page
                if !reader.is_empty() && reader.remaining() >= 4 {
                    ext.page = Some(reader.read_i32()?);
                }

                // Parse delay
                if !reader.is_empty() && reader.remaining() >= 4 {
                    ext.delay = Some(reader.read_u32()?);
                }
            }
        }

        Ok(ext)
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
        let form = FormObject::default();
        assert_eq!(form.object_type(), FormObjectType::Unknown);
        assert_eq!(form.name(), "");
        assert_eq!(form.default_value(), "");
        assert_eq!(form.tab_order(), 0);
    }

    #[test]
    fn test_form_object_with_type_and_name() {
        let form =
            FormObject::with_type_and_name(FormObjectType::TextField, "username".to_string());
        assert_eq!(form.object_type(), FormObjectType::TextField);
        assert_eq!(form.name(), "username");
    }

    #[test]
    fn test_form_object_setters() {
        let mut form = FormObject::default();
        form.set_default_value("default".to_string());
        form.set_tab_order(5);
        assert_eq!(form.default_value(), "default");
        assert_eq!(form.tab_order(), 5);
    }
}
