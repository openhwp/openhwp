//! Equation (mathematical formula) parsing.
//!
//! Equations in HWP documents use a proprietary markup language similar to
//! LaTeX for representing mathematical formulas.

use crate::error::Result;
use crate::primitive::HwpUnit;
use crate::util::ByteReader;

/// Equation line mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EquationLineMode {
    /// Baseline alignment.
    #[default]
    Baseline,
    /// Center alignment.
    Center,
    /// Bottom alignment.
    Bottom,
    /// Top alignment.
    Top,
}

impl EquationLineMode {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Baseline,
            1 => Self::Center,
            2 => Self::Bottom,
            3 => Self::Top,
            _ => Self::Baseline,
        }
    }
}

/// Equation properties.
#[derive(Debug, Clone, Default)]
pub struct EquationProperties {
    /// Properties flags.
    pub properties: u32,
    /// Equation width.
    pub width: HwpUnit,
    /// Equation height.
    pub height: HwpUnit,
    /// Line mode.
    pub line_mode: EquationLineMode,
    /// Script (formula text).
    pub script: String,
    /// Base font size in points * 100.
    pub base_font_size: u32,
    /// Text color (ARGB).
    pub text_color: u32,
    /// Baseline offset.
    pub baseline_offset: i16,
    /// Version string.
    pub version: String,
    /// Font name.
    pub font_name: String,
}

impl EquationProperties {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u32()?;

        // Read script (equation markup)
        let script_length = reader.read_u16()? as usize;
        let script_bytes = reader.read_bytes(script_length * 2)?;
        let script = decode_utf16_le(script_bytes);

        let base_font_size = reader.read_u32()?;
        let text_color = reader.read_u32()?;
        let baseline_offset = reader.read_i16()?;

        // Version string
        let version_length = reader.read_u16()? as usize;
        let version_bytes = reader.read_bytes(version_length * 2)?;
        let version = decode_utf16_le(version_bytes);

        // Font name (version 5.0.2.3+)
        let font_name = if !reader.is_empty() {
            let font_name_length = reader.read_u16()? as usize;
            if font_name_length > 0 {
                let font_name_bytes = reader.read_bytes(font_name_length * 2)?;
                decode_utf16_le(font_name_bytes)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Read dimensions if available (the order varies by version)
        let line_mode = EquationLineMode::from_raw((properties & 0xFF) as u8);

        Ok(Self {
            properties,
            width: HwpUnit::new(0),  // Set externally from control header
            height: HwpUnit::new(0), // Set externally from control header
            line_mode,
            script,
            base_font_size,
            text_color,
            baseline_offset,
            version,
            font_name,
        })
    }
}

/// An equation (mathematical formula) in the document.
#[derive(Debug, Clone, Default)]
pub struct Equation {
    /// Equation properties.
    pub properties: EquationProperties,
}

impl Equation {
    /// Creates a new equation with properties.
    pub fn new(properties: EquationProperties) -> Self {
        Self { properties }
    }

    /// Parses equation from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = EquationProperties::from_reader(reader)?;
        Ok(Self::new(properties))
    }

    /// Returns the equation script (markup text).
    pub fn script(&self) -> &str {
        &self.properties.script
    }

    /// Returns the base font size in points.
    pub fn font_size_points(&self) -> f32 {
        self.properties.base_font_size as f32 / 100.0
    }

    /// Returns the text color as ARGB.
    pub fn text_color(&self) -> u32 {
        self.properties.text_color
    }

    /// Returns the font name.
    pub fn font_name(&self) -> &str {
        &self.properties.font_name
    }

    /// Returns the version string.
    pub fn version(&self) -> &str {
        &self.properties.version
    }
}

/// Decodes UTF-16 little-endian bytes to a String.
fn decode_utf16_le(bytes: &[u8]) -> String {
    let mut chars = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks(2) {
        if chunk.len() == 2 {
            let code_unit = u16::from_le_bytes([chunk[0], chunk[1]]);
            chars.push(code_unit);
        }
    }
    String::from_utf16_lossy(&chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation_line_mode() {
        assert_eq!(EquationLineMode::from_raw(0), EquationLineMode::Baseline);
        assert_eq!(EquationLineMode::from_raw(1), EquationLineMode::Center);
        assert_eq!(EquationLineMode::from_raw(2), EquationLineMode::Bottom);
        assert_eq!(EquationLineMode::from_raw(3), EquationLineMode::Top);
        assert_eq!(EquationLineMode::from_raw(255), EquationLineMode::Baseline);
    }
}
