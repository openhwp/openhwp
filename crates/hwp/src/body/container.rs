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

use crate::error::Result;
use crate::util::ByteReader;

use super::control::Control;

/// A container for grouped shapes.
#[derive(Debug, Clone, Default)]
pub struct ShapeContainer {
    /// Expected number of children (from container record).
    expected_child_count: u16,
    /// Child controls in this container.
    children: Vec<Control>,
}

impl ShapeContainer {
    /// Returns the child controls.
    pub fn children(&self) -> &[Control] {
        &self.children
    }

    /// Returns mutable reference to child controls.
    pub fn children_mut(&mut self) -> &mut Vec<Control> {
        &mut self.children
    }

    /// Adds a child control.
    pub fn add_child(&mut self, child: Control) {
        self.children.push(child);
    }

    /// Returns the number of children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Returns true if this container is empty.
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns the expected child count (from the container record).
    pub const fn expected_child_count(&self) -> u16 {
        self.expected_child_count
    }

    /// Checks if the actual child count matches the expected count.
    ///
    /// Returns `true` if matches or if expected count is 0 (unknown).
    pub fn validate_child_count(&self) -> bool {
        self.expected_child_count == 0 || self.children.len() == self.expected_child_count as usize
    }

    /// Parses container from reader.
    ///
    /// Format (per HWP spec - HWPTAG_SHAPE_COMPONENT_CONTAINER):
    /// - UINT16: Child count
    /// - Child shape IDs follow (parsed separately)
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let expected_child_count = reader.read_u16()?;

        // Child count is informational - actual children are parsed as
        // subsequent ShapeComponent records and added via add_child().
        Ok(Self {
            expected_child_count,
            children: Vec::new(),
        })
    }

    /// Extracts plain text from all children.
    pub fn plain_text(&self) -> String {
        self.children
            .iter()
            .filter_map(|c| {
                let text = c.plain_text();
                if text.is_empty() {
                    None
                } else {
                    Some(text)
                }
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
        assert!(container.is_empty());
        assert_eq!(container.child_count(), 0);
        assert_eq!(container.expected_child_count(), 0);
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
        assert!(container.validate_child_count());

        // With expected_child_count > 0, validation checks actual count
        let data = [0x02, 0x00]; // expected_child_count = 2
        let mut reader = crate::util::ByteReader::new(&data);
        let container = ShapeContainer::from_reader(&mut reader).unwrap();
        assert_eq!(container.expected_child_count(), 2);
        assert!(!container.validate_child_count()); // 0 children, expected 2
    }
}
