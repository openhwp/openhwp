use crate::u32;

#[derive(Debug)]
pub struct LayoutCompatibility {
    pub text_attribute: u32,
    pub paragraph_attribute: u32,
    pub section_attribute: u32,
    pub object_attribute: u32,
    pub field_attribute: u32,
}

impl LayoutCompatibility {
    pub fn from_payload(payload: &[u8]) -> Self {
        Self {
            text_attribute: u32(payload, 0),
            paragraph_attribute: u32(payload, 4),
            section_attribute: u32(payload, 8),
            object_attribute: u32(payload, 12),
            field_attribute: u32(payload, 16),
        }
    }
}
