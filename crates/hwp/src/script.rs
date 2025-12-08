//! Script parsing.
//!
//! HWP documents can contain JavaScript code for forms and automation.
//! Scripts are stored in the Scripts storage within the compound file.

use crate::error::Result;

/// Script version information.
#[derive(Debug, Clone, Default)]
pub struct ScriptVersion {
    /// Default JavaScript version.
    pub default_js_version: String,
    /// JavaScript version for each file.
    pub js_versions: Vec<String>,
}

impl ScriptVersion {
    /// Parses script version from bytes.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let text = decode_utf16_le(data);
        let mut lines: Vec<&str> = text.lines().collect();

        let default_js_version = if !lines.is_empty() {
            lines.remove(0).to_string()
        } else {
            String::new()
        };

        let js_versions = lines.iter().map(|s| s.to_string()).collect();

        Ok(Self {
            default_js_version,
            js_versions,
        })
    }
}

/// Script header information.
#[derive(Debug, Clone, Default)]
pub struct ScriptHeader {
    /// Number of script files.
    pub script_count: u32,
}

impl ScriptHeader {
    /// Parses script header from bytes.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < 4 {
            return Ok(Self::default());
        }

        let script_count = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

        Ok(Self { script_count })
    }
}

/// A JavaScript source file.
#[derive(Debug, Clone, Default)]
pub struct ScriptSource {
    /// Script file name.
    pub name: String,
    /// Script source code.
    pub code: String,
}

impl ScriptSource {
    /// Creates a new script source.
    pub fn new(name: String, code: String) -> Self {
        Self { name, code }
    }

    /// Parses script source from UTF-16 bytes.
    pub fn from_bytes(name: &str, data: &[u8]) -> Result<Self> {
        let code = decode_utf16_le(data);
        Ok(Self {
            name: name.to_string(),
            code,
        })
    }

    /// Returns true if the script is empty.
    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }
}

/// Collection of scripts in the document.
#[derive(Debug, Clone, Default)]
pub struct Scripts {
    /// Script version information.
    pub version: ScriptVersion,
    /// Script header.
    pub header: ScriptHeader,
    /// Script source files.
    pub sources: Vec<ScriptSource>,
}

impl Scripts {
    /// Returns true if there are no scripts.
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    /// Returns the number of scripts.
    pub fn len(&self) -> usize {
        self.sources.len()
    }

    /// Returns an iterator over script sources.
    pub fn iter(&self) -> impl Iterator<Item = &ScriptSource> {
        self.sources.iter()
    }
}

/// Decodes UTF-16 little-endian bytes to a String.
fn decode_utf16_le(bytes: &[u8]) -> String {
    let mut chars = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks(2) {
        if chunk.len() == 2 {
            let code_unit = u16::from_le_bytes([chunk[0], chunk[1]]);
            if code_unit == 0 {
                break;
            }
            chars.push(code_unit);
        }
    }
    String::from_utf16_lossy(&chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_header() {
        let data = [0x02, 0x00, 0x00, 0x00]; // 2 scripts
        let header = ScriptHeader::from_bytes(&data).unwrap();
        assert_eq!(header.script_count, 2);
    }

    #[test]
    fn test_script_source() {
        // "alert(1)" in UTF-16LE
        let data = [
            0x61, 0x00, 0x6C, 0x00, 0x65, 0x00, 0x72, 0x00, 0x74, 0x00, 0x28, 0x00, 0x31, 0x00,
            0x29, 0x00,
        ];
        let source = ScriptSource::from_bytes("test.js", &data).unwrap();
        assert_eq!(source.name, "test.js");
        assert_eq!(source.code, "alert(1)");
    }
}
