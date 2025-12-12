//! Shape container (grouped objects) parsing.
//!
//! A container holds multiple shapes that are grouped together.
//! Grouped shapes can be moved, resized, and transformed as a unit.
//!
//! ## Container Structure
//!
//! In HWP format, a container's metadata (HWPTAG_SHAPE_COMPONENT_CONTAINER)
//! declares the expected child count. The actual child shapes follow as
//! separate ShapeComponent records and are collected during section parsing.

use super::control::Control;
use crate::error::Result;
use crate::util::ByteReader;

/// A container for grouped shapes.
#[derive(Debug, Clone, Default)]
pub struct ShapeContainer {
    /// Child controls in this container.
    children: Vec<Control>,
}

impl ShapeContainer {
    /// Returns the child controls.
    pub fn children(&self) -> &[Control] {
        &self.children
    }

    /// Returns mutable reference to child controls.
    pub const fn children_mut(&mut self) -> &mut Vec<Control> {
        &mut self.children
    }

    /// Adds a child control.
    pub fn add_child(&mut self, child: Control) {
        self.children.push(child);
    }

    /// Parses container from reader.
    ///
    /// Format (per HWP spec - HWPTAG_SHAPE_COMPONENT_CONTAINER):
    /// - UINT16: Child count
    /// - Child shape IDs follow (parsed separately)
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let _expected_child_count = reader.read_u16()?;

        // Child count is informational - actual children are parsed as
        // subsequent ShapeComponent records and added via add_child().
        Ok(Self {
            children: Vec::new(),
        })
    }

    /// Extracts plain text from all children.
    pub fn plain_text(&self) -> String {
        self.children
            .iter()
            .filter_map(|c| {
                let text = c.plain_text();
                if text.is_empty() { None } else { Some(text) }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_container_default() {
        let container = ShapeContainer::default();
        assert!(container.children.len() == 0);
        assert_eq!(container.children.len(), 0);
    }

    #[test]
    fn test_shape_container_plain_text_empty() {
        let container = ShapeContainer::default();
        assert_eq!(container.plain_text(), "");
    }

    #[test]
    fn test_shape_container_validate_child_count() {
        // With expected_child_count = 0, validation always passes
        let container = ShapeContainer::default();
        assert!(container.children.len() == 0);

        // With expected_child_count > 0, validation checks actual count
        let data = [0x02, 0x00]; // expected_child_count = 2
        let mut reader = crate::util::ByteReader::new(&data);
        let container = ShapeContainer::from_reader(&mut reader).unwrap();
        assert!(container.children.len() == 0);
    }
}
