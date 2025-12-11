//! Paragraph numbering record.
//!
//! The actual binary structure differs from some HWP specification documents.
//! Based on binary analysis, the structure is:
//! - 7 levels, each containing:
//!   - ParagraphHeadInfo (12 bytes)
//!   - format string (length-prefixed UTF-16)
//! - Extended start numbers at the end (optional)

use crate::error::Result;
use crate::primitive::HwpUnit16;
use crate::util::ByteReader;

/// Paragraph head alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParagraphHeadAlignment {
    #[default]
    Left,
    Center,
    Right,
}

impl ParagraphHeadAlignment {
    pub const fn from_raw(value: u32) -> Self {
        match value & 0x03 {
            0 => Self::Left,
            1 => Self::Center,
            2 => Self::Right,
            _ => Self::Left,
        }
    }
}

/// Paragraph head information.
#[derive(Debug, Clone)]
pub struct ParagraphHeadInfo {
    properties: u32,
    width_correction: HwpUnit16,
    text_distance: HwpUnit16,
    character_shape_id: u32,
}

impl ParagraphHeadInfo {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            properties: reader.read_u32()?,
            width_correction: reader.read_hwp_unit16()?,
            text_distance: reader.read_hwp_unit16()?,
            character_shape_id: reader.read_u32()?,
        })
    }

    pub const fn alignment(&self) -> ParagraphHeadAlignment {
        ParagraphHeadAlignment::from_raw(self.properties)
    }

    pub const fn use_instance_width(&self) -> bool {
        (self.properties & (1 << 2)) != 0
    }

    pub const fn auto_indent(&self) -> bool {
        (self.properties & (1 << 3)) != 0
    }

    /// Returns the number format type.
    /// The format is stored in bits 12-15 of the properties field.
    pub const fn number_format(&self) -> u8 {
        ((self.properties >> 12) & 0x0F) as u8
    }

    pub const fn character_shape_id(&self) -> u32 {
        self.character_shape_id
    }

    /// Returns the width correction value.
    pub const fn width_correction(&self) -> HwpUnit16 {
        self.width_correction
    }

    /// Returns the text distance value.
    pub const fn text_distance(&self) -> HwpUnit16 {
        self.text_distance
    }
}

/// Numbering level information.
#[derive(Debug, Clone)]
pub struct NumberingLevel {
    pub head_info: ParagraphHeadInfo,
    pub format: String,
    pub start_number: u32,
}

impl NumberingLevel {
    /// Formats a number according to this level's format string.
    ///
    /// The format string can contain placeholders like:
    /// - ^1, ^2, ... ^7: Numbers at corresponding levels
    /// - ^n: Current level number
    ///
    /// Returns the formatted string.
    pub fn format_number(&self, numbers: &[u32]) -> String {
        let mut result = self.format.clone();

        // Replace ^1 through ^7 with corresponding level numbers
        for (i, &num) in numbers.iter().enumerate().take(7) {
            let placeholder = format!("^{}", i + 1);
            let replacement = num.to_string();
            result = result.replace(&placeholder, &replacement);
        }

        // Replace ^n with the first number (fallback)
        if result.contains("^n") {
            let num = numbers.first().copied().unwrap_or(1).to_string();
            result = result.replace("^n", &num);
        }

        result
    }
}

/// Paragraph numbering definition.
#[derive(Debug, Clone)]
pub struct Numbering {
    levels: Vec<NumberingLevel>,
}

impl Numbering {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let mut levels = Vec::with_capacity(7);

        // Read 7 levels - each level has ParagraphHeadInfo + format string
        // Note: The start_number is NOT inline with each level in the actual format
        for _ in 0..7 {
            let head_info = ParagraphHeadInfo::from_reader(reader)?;
            let format = reader.read_utf16_string()?;

            levels.push(NumberingLevel {
                head_info,
                format,
                start_number: 1, // Default, may be overwritten below
            });
        }

        // Read extended start numbers if present (7 x u32 = 28 bytes)
        // The format appears to be: u16 padding + 7 x u32 start numbers
        // But this can vary by version, so we handle it gracefully
        //
        // ## Version Compatibility Note
        //
        // HWP versions have varying extended field layouts:
        // - Older versions may have shorter records (no extended start numbers)
        // - Newer versions (5.0.2.x+) include a 2-byte field before start numbers
        // - The 2-byte field purpose is undocumented in the public spec
        // - We skip it gracefully to maintain forward/backward compatibility
        if reader.remaining() >= 2 {
            // Skip 2-byte padding/unknown field (version-dependent, undocumented)
            let _ = reader.read_u16();
        }

        // Read start numbers for each level
        for level in &mut levels {
            if reader.remaining() >= 4 {
                level.start_number = reader.read_u32()?;
            }
        }

        // Skip any remaining data (version-dependent extended fields)

        Ok(Self { levels })
    }

    /// Returns the numbering levels.
    pub fn levels(&self) -> &[NumberingLevel] {
        &self.levels
    }

    /// Returns a specific level (1-7).
    pub fn level(&self, level: usize) -> Option<&NumberingLevel> {
        if (1..=7).contains(&level) {
            self.levels.get(level - 1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 실제 HWP 파일에서 발췌한 NUMBERING 레코드 바이너리
    /// 텍스트는 임의의 숫자 형식으로 대체됨 (저작권 문제 회피)
    fn create_test_numbering_data() -> Vec<u8> {
        let mut data = Vec::new();

        // 7개 레벨의 ParagraphHeadInfo + 포맷 문자열
        for i in 0..7 {
            // ParagraphHeadInfo (12 bytes)
            // - properties: u32
            // - width_correction: u16 (HwpUnit16)
            // - text_distance: u16 (HwpUnit16)
            // - character_shape_id: u32
            data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // properties
            data.extend_from_slice(&[0x00, 0x00]); // width_correction
            data.extend_from_slice(&[0x00, 0x00]); // text_distance
            data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // character_shape_id

            // 포맷 문자열 (length-prefixed UTF-16)
            // "^n." 형식 (레벨에 따라 다름)
            let format = match i {
                0 => "^1.",
                1 => "^2.",
                2 => "^3.",
                3 => "^4.",
                4 => "^5.",
                5 => "^6.",
                6 => "^7.",
                _ => "",
            };

            // 길이 (u16)
            data.extend_from_slice(&(format.len() as u16).to_le_bytes());

            // UTF-16LE 인코딩
            for ch in format.encode_utf16() {
                data.extend_from_slice(&ch.to_le_bytes());
            }
        }

        // 패딩 (2 bytes)
        data.extend_from_slice(&[0x00, 0x00]);

        // 시작 번호 (7 x u32)
        for i in 1..=7u32 {
            data.extend_from_slice(&i.to_le_bytes());
        }

        data
    }

    #[test]
    fn test_numbering_parsing() {
        let data = create_test_numbering_data();
        let mut reader = ByteReader::new(&data);

        let numbering = Numbering::from_reader(&mut reader).unwrap();

        assert_eq!(numbering.levels().len(), 7);

        // 첫 번째 레벨 확인
        let level1 = numbering.level(1).unwrap();
        assert_eq!(level1.format, "^1.");
        assert_eq!(level1.start_number, 1);

        // 세 번째 레벨 확인
        let level3 = numbering.level(3).unwrap();
        assert_eq!(level3.format, "^3.");
        assert_eq!(level3.start_number, 3);
    }

    #[test]
    fn test_numbering_snapshot() {
        let data = create_test_numbering_data();
        let mut reader = ByteReader::new(&data);

        let numbering = Numbering::from_reader(&mut reader).unwrap();

        insta::assert_debug_snapshot!(numbering);
    }

    #[test]
    fn test_format_number() {
        let data = create_test_numbering_data();
        let mut reader = ByteReader::new(&data);
        let numbering = Numbering::from_reader(&mut reader).unwrap();

        // Level 1: "^1." -> "1."
        let level1 = numbering.level(1).unwrap();
        assert_eq!(level1.format_number(&[1]), "1.");
        assert_eq!(level1.format_number(&[5]), "5.");

        // Level 3: "^3." -> "3."
        let level3 = numbering.level(3).unwrap();
        assert_eq!(level3.format_number(&[1, 2, 3]), "3.");
    }
}
