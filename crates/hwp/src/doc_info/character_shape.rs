//! Character shape record.
//!
//! Defines character formatting including font, size, color, and decorations.

use crate::error::Result;
use crate::primitive::ColorReference;
use crate::util::ByteReader;

/// Language type for font references.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LanguageType {
    /// Korean.
    Korean = 0,
    /// English.
    English = 1,
    /// Chinese.
    Chinese = 2,
    /// Japanese.
    Japanese = 3,
    /// Other.
    Other = 4,
    /// Symbol.
    Symbol = 5,
    /// User-defined.
    User = 6,
}

impl LanguageType {
    /// All language types in order.
    pub const ALL: [Self; 7] = [
        Self::Korean,
        Self::English,
        Self::Chinese,
        Self::Japanese,
        Self::Other,
        Self::Symbol,
        Self::User,
    ];
}

/// Underline position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnderlinePosition {
    /// No underline.
    None,
    /// Below text.
    Bottom,
    /// Above text.
    Top,
}

impl UnderlinePosition {
    /// Creates from raw value.
    pub const fn from_raw(value: u32) -> Self {
        match (value >> 2) & 0x03 {
            1 => Self::Bottom,
            3 => Self::Top,
            _ => Self::None,
        }
    }
}

/// Underline line shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnderlineShape {
    /// Solid line.
    Solid,
    /// Long dashes.
    LongDash,
    /// Short dashes.
    Dash,
    /// Dash-dot pattern.
    DashDot,
    /// Dash-dot-dot pattern.
    DashDotDot,
    /// Long dash.
    LongDashLong,
    /// Large circles.
    Circle,
    /// Double line.
    Double,
    /// Thin then thick.
    ThinThick,
    /// Thick then thin.
    ThickThin,
    /// Thin-thick-thin.
    ThinThickThin,
    /// Wave.
    Wave,
    /// Double wave.
    DoubleWave,
    /// Thick 3D.
    Thick3D,
    /// Thick 3D reversed.
    Thick3DReversed,
    /// Single 3D.
    Single3D,
    /// Single 3D reversed.
    Single3DReversed,
}

impl UnderlineShape {
    /// Creates from raw value.
    pub const fn from_raw(value: u32) -> Self {
        match (value >> 4) & 0x0F {
            0 => Self::Solid,
            1 => Self::LongDash,
            2 => Self::Dash,
            3 => Self::DashDot,
            4 => Self::DashDotDot,
            5 => Self::LongDashLong,
            6 => Self::Circle,
            7 => Self::Double,
            8 => Self::ThinThick,
            9 => Self::ThickThin,
            10 => Self::ThinThickThin,
            11 => Self::Wave,
            12 => Self::DoubleWave,
            13 => Self::Thick3D,
            14 => Self::Thick3DReversed,
            15 => Self::Single3D,
            _ => Self::Solid,
        }
    }
}

/// Outline type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutlineType {
    /// No outline.
    None,
    /// Solid outline.
    Solid,
    /// Dotted outline.
    Dotted,
    /// Thick outline.
    Thick,
    /// Dashed outline.
    Dashed,
    /// Dash-dot.
    DashDot,
    /// Dash-dot-dot.
    DashDotDot,
}

impl OutlineType {
    /// Creates from raw value.
    pub const fn from_raw(value: u32) -> Self {
        match (value >> 8) & 0x07 {
            0 => Self::None,
            1 => Self::Solid,
            2 => Self::Dotted,
            3 => Self::Thick,
            4 => Self::Dashed,
            5 => Self::DashDot,
            6 => Self::DashDotDot,
            _ => Self::None,
        }
    }
}

/// Shadow type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadowType {
    /// No shadow.
    None,
    /// Discrete shadow.
    Discrete,
    /// Continuous shadow.
    Continuous,
}

impl ShadowType {
    /// Creates from raw value.
    pub const fn from_raw(value: u32) -> Self {
        match (value >> 11) & 0x03 {
            1 => Self::Discrete,
            2 => Self::Continuous,
            _ => Self::None,
        }
    }
}

/// Emphasis type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmphasisType {
    /// No emphasis.
    None,
    /// Filled circle.
    FilledCircle,
    /// Open circle.
    OpenCircle,
    /// Caron.
    Caron,
    /// Tilde.
    Tilde,
    /// Dot.
    Dot,
    /// Colon.
    Colon,
}

impl EmphasisType {
    /// Creates from raw value.
    pub const fn from_raw(value: u32) -> Self {
        match (value >> 21) & 0x0F {
            0 => Self::None,
            1 => Self::FilledCircle,
            2 => Self::OpenCircle,
            3 => Self::Caron,
            4 => Self::Tilde,
            5 => Self::Dot,
            6 => Self::Colon,
            _ => Self::None,
        }
    }
}

/// Strikethrough shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrikethroughShape {
    /// No strikethrough.
    None,
    /// Single line.
    Single,
    /// Double line.
    Double,
}

impl StrikethroughShape {
    /// Creates from raw value.
    pub const fn from_raw(value: u32) -> Self {
        match (value >> 18) & 0x07 {
            0 => Self::None,
            1 => Self::Single,
            _ => Self::Single,
        }
    }
}

/// Character shape.
///
/// Defines character formatting like font, size, color, and decorations.
#[derive(Debug, Clone)]
pub struct CharacterShape {
    /// Font ID references for each language.
    font_ids: [u16; 7],
    /// Width ratios for each language (50-200%).
    width_ratios: [u8; 7],
    /// Character spacing for each language (-50 to 50%).
    spacings: [i8; 7],
    /// Relative sizes for each language (10-250%).
    relative_sizes: [u8; 7],
    /// Character positions for each language (-100 to 100%).
    positions: [i8; 7],
    /// Base size in points (0-4096).
    base_size: i32,
    /// Properties bit field.
    properties: u32,
    /// Shadow X offset.
    shadow_offset_x: i8,
    /// Shadow Y offset.
    shadow_offset_y: i8,
    /// Text color.
    text_color: ColorReference,
    /// Underline color.
    underline_color: ColorReference,
    /// Shade color.
    shade_color: ColorReference,
    /// Shadow color.
    shadow_color: ColorReference,
    /// Border fill ID (optional, version 5.0.2.1+).
    border_fill_id: Option<u16>,
    /// Strikethrough color (optional, version 5.0.3.0+).
    strikethrough_color: Option<ColorReference>,
}

impl CharacterShape {
    /// Parses CharacterShape from a reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let mut font_ids = [0u16; 7];
        for id in &mut font_ids {
            *id = reader.read_u16()?;
        }

        let mut width_ratios = [0u8; 7];
        for ratio in &mut width_ratios {
            *ratio = reader.read_u8()?;
        }

        let mut spacings = [0i8; 7];
        for spacing in &mut spacings {
            *spacing = reader.read_i8()?;
        }

        let mut relative_sizes = [0u8; 7];
        for size in &mut relative_sizes {
            *size = reader.read_u8()?;
        }

        let mut positions = [0i8; 7];
        for pos in &mut positions {
            *pos = reader.read_i8()?;
        }

        let base_size = reader.read_i32()?;
        let properties = reader.read_u32()?;
        let shadow_offset_x = reader.read_i8()?;
        let shadow_offset_y = reader.read_i8()?;
        let text_color = reader.read_color()?;
        let underline_color = reader.read_color()?;
        let shade_color = reader.read_color()?;
        let shadow_color = reader.read_color()?;

        let border_fill_id = if reader.remaining() >= 2 {
            Some(reader.read_u16()?)
        } else {
            None
        };

        let strikethrough_color = if reader.remaining() >= 4 {
            Some(reader.read_color()?)
        } else {
            None
        };

        Ok(Self {
            font_ids,
            width_ratios,
            spacings,
            relative_sizes,
            positions,
            base_size,
            properties,
            shadow_offset_x,
            shadow_offset_y,
            text_color,
            underline_color,
            shade_color,
            shadow_color,
            border_fill_id,
            strikethrough_color,
        })
    }

    /// Returns the font ID for the given language.
    #[inline]
    pub const fn font_id(&self, lang: LanguageType) -> u16 {
        self.font_ids[lang as usize]
    }

    /// Returns all font IDs.
    #[inline]
    pub const fn font_ids(&self) -> &[u16; 7] {
        &self.font_ids
    }

    /// Returns the width ratio for the given language (50-200%).
    #[inline]
    pub const fn width_ratio(&self, lang: LanguageType) -> u8 {
        self.width_ratios[lang as usize]
    }

    /// Returns all width ratios.
    #[inline]
    pub const fn width_ratios(&self) -> &[u8; 7] {
        &self.width_ratios
    }

    /// Returns the character spacing for the given language (-50 to 50%).
    #[inline]
    pub const fn spacing(&self, lang: LanguageType) -> i8 {
        self.spacings[lang as usize]
    }

    /// Returns all character spacings.
    #[inline]
    pub const fn spacings(&self) -> &[i8; 7] {
        &self.spacings
    }

    /// Returns the relative size for the given language (10-250%).
    #[inline]
    pub const fn relative_size(&self, lang: LanguageType) -> u8 {
        self.relative_sizes[lang as usize]
    }

    /// Returns all relative sizes.
    #[inline]
    pub const fn relative_sizes(&self) -> &[u8; 7] {
        &self.relative_sizes
    }

    /// Returns the character position for the given language (-100 to 100%).
    #[inline]
    pub const fn position(&self, lang: LanguageType) -> i8 {
        self.positions[lang as usize]
    }

    /// Returns all character positions.
    #[inline]
    pub const fn positions(&self) -> &[i8; 7] {
        &self.positions
    }

    /// Returns the base size in HWP units (1/7200 inch).
    ///
    /// To get size in points, divide by 100.
    #[inline]
    pub const fn base_size(&self) -> i32 {
        self.base_size
    }

    /// Returns the base size in points.
    #[inline]
    pub fn base_size_points(&self) -> f64 {
        self.base_size as f64 / 100.0
    }

    /// Returns whether italic is enabled.
    #[inline]
    pub const fn is_italic(&self) -> bool {
        (self.properties & (1 << 0)) != 0
    }

    /// Returns whether bold is enabled.
    #[inline]
    pub const fn is_bold(&self) -> bool {
        (self.properties & (1 << 1)) != 0
    }

    /// Returns the underline position.
    #[inline]
    pub const fn underline_position(&self) -> UnderlinePosition {
        UnderlinePosition::from_raw(self.properties)
    }

    /// Returns the underline shape.
    #[inline]
    pub const fn underline_shape(&self) -> UnderlineShape {
        UnderlineShape::from_raw(self.properties)
    }

    /// Returns the outline type.
    #[inline]
    pub const fn outline_type(&self) -> OutlineType {
        OutlineType::from_raw(self.properties)
    }

    /// Returns the shadow type.
    #[inline]
    pub const fn shadow_type(&self) -> ShadowType {
        ShadowType::from_raw(self.properties)
    }

    /// Returns whether emboss is enabled.
    #[inline]
    pub const fn is_emboss(&self) -> bool {
        (self.properties & (1 << 13)) != 0
    }

    /// Returns whether engrave is enabled.
    #[inline]
    pub const fn is_engrave(&self) -> bool {
        (self.properties & (1 << 14)) != 0
    }

    /// Returns whether superscript is enabled.
    #[inline]
    pub const fn is_superscript(&self) -> bool {
        (self.properties & (1 << 15)) != 0
    }

    /// Returns whether subscript is enabled.
    #[inline]
    pub const fn is_subscript(&self) -> bool {
        (self.properties & (1 << 16)) != 0
    }

    /// Returns the strikethrough shape.
    #[inline]
    pub const fn strikethrough_shape(&self) -> StrikethroughShape {
        StrikethroughShape::from_raw(self.properties)
    }

    /// Returns the emphasis type.
    #[inline]
    pub const fn emphasis_type(&self) -> EmphasisType {
        EmphasisType::from_raw(self.properties)
    }

    /// Returns whether kerning is enabled.
    #[inline]
    pub const fn is_kerning(&self) -> bool {
        (self.properties & (1 << 30)) != 0
    }

    /// Returns the text color.
    #[inline]
    pub const fn text_color(&self) -> ColorReference {
        self.text_color
    }

    /// Returns the underline color.
    #[inline]
    pub const fn underline_color(&self) -> ColorReference {
        self.underline_color
    }

    /// Returns the shade color.
    #[inline]
    pub const fn shade_color(&self) -> ColorReference {
        self.shade_color
    }

    /// Returns the shadow color.
    #[inline]
    pub const fn shadow_color(&self) -> ColorReference {
        self.shadow_color
    }

    /// Returns the shadow offsets.
    #[inline]
    pub const fn shadow_offset(&self) -> (i8, i8) {
        (self.shadow_offset_x, self.shadow_offset_y)
    }

    /// Returns the border fill ID.
    #[inline]
    pub const fn border_fill_id(&self) -> Option<u16> {
        self.border_fill_id
    }

    /// Returns the strikethrough color.
    #[inline]
    pub const fn strikethrough_color(&self) -> Option<ColorReference> {
        self.strikethrough_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_underline_position() {
        assert_eq!(UnderlinePosition::from_raw(0b0000), UnderlinePosition::None);
        assert_eq!(
            UnderlinePosition::from_raw(0b0100),
            UnderlinePosition::Bottom
        );
        assert_eq!(UnderlinePosition::from_raw(0b1100), UnderlinePosition::Top);
    }

    #[test]
    fn test_properties_flags() {
        // italic = bit 0, bold = bit 1
        let props = 0b0000_0011u32;
        assert!((props & (1 << 0)) != 0); // italic
        assert!((props & (1 << 1)) != 0); // bold
    }

    /// 테스트용 CharacterShape 바이너리 데이터 생성
    fn create_test_character_shape_data() -> Vec<u8> {
        let mut data = Vec::new();

        // font_ids: 7 x u16 (각 언어별 폰트 ID)
        for i in 0..7u16 {
            data.extend_from_slice(&i.to_le_bytes());
        }

        // width_ratios: 7 x u8 (100% = 기본값)
        for _ in 0..7 {
            data.push(100);
        }

        // spacings: 7 x i8 (0 = 기본값)
        for _ in 0..7 {
            data.push(0);
        }

        // relative_sizes: 7 x u8 (100% = 기본값)
        for _ in 0..7 {
            data.push(100);
        }

        // positions: 7 x i8 (0 = 기본값)
        for _ in 0..7 {
            data.push(0);
        }

        // base_size: i32 (1000 = 10pt)
        data.extend_from_slice(&1000i32.to_le_bytes());

        // properties: u32 (italic + bold)
        let props = 0b0000_0011u32;
        data.extend_from_slice(&props.to_le_bytes());

        // shadow_offset_x: i8
        data.push(1);

        // shadow_offset_y: i8
        data.push(1);

        // text_color: ColorReference (ARGB, 검정색)
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

        // underline_color: ColorReference (검정색)
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

        // shade_color: ColorReference (흰색)
        data.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0x00]);

        // shadow_color: ColorReference (회색)
        data.extend_from_slice(&[0x80, 0x80, 0x80, 0x00]);

        data
    }

    #[test]
    fn test_character_shape_parsing() {
        let data = create_test_character_shape_data();
        let mut reader = crate::util::ByteReader::new(&data);

        let shape = CharacterShape::from_reader(&mut reader).unwrap();

        assert_eq!(shape.font_id(LanguageType::Korean), 0);
        assert_eq!(shape.font_id(LanguageType::English), 1);
        assert_eq!(shape.base_size(), 1000);
        assert!(shape.is_italic());
        assert!(shape.is_bold());
    }

    #[test]
    fn test_character_shape_snapshot() {
        let data = create_test_character_shape_data();
        let mut reader = crate::util::ByteReader::new(&data);

        let shape = CharacterShape::from_reader(&mut reader).unwrap();

        insta::assert_debug_snapshot!(shape);
    }
}
