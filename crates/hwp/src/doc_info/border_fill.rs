//! Border and fill record.
//!
//! Defines border styles and fill patterns.

use crate::error::Result;
use crate::primitive::ColorReference;
use crate::util::ByteReader;

/// Border line style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum BorderLineStyle {
    /// Solid line.
    #[default]
    Solid = 0,
    /// Long dashes.
    LongDash = 1,
    /// Short dashes.
    Dash = 2,
    /// Dash-dot.
    DashDot = 3,
    /// Dash-dot-dot.
    DashDotDot = 4,
    /// Long dash (alternate).
    LongDashAlt = 5,
    /// Circle pattern.
    Circle = 6,
    /// Double line.
    Double = 7,
    /// Thin then thick.
    ThinThick = 8,
    /// Thick then thin.
    ThickThin = 9,
    /// Thin-thick-thin.
    ThinThickThin = 10,
    /// Wave.
    Wave = 11,
    /// Double wave.
    DoubleWave = 12,
    /// Thick 3D.
    Thick3D = 13,
    /// Thick 3D reversed.
    Thick3DReversed = 14,
    /// Single 3D.
    Single3D = 15,
    /// Single 3D reversed.
    Single3DReversed = 16,
}

impl BorderLineStyle {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Solid,
            1 => Self::LongDash,
            2 => Self::Dash,
            3 => Self::DashDot,
            4 => Self::DashDotDot,
            5 => Self::LongDashAlt,
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
            16 => Self::Single3DReversed,
            _ => Self::Solid,
        }
    }
}

/// Border line thickness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BorderLineThickness(u8);

impl BorderLineThickness {
    /// Thickness values in 0.01mm.
    const VALUES: [u16; 16] = [
        10, 12, 15, 20, 25, 30, 40, 50, 60, 70, 100, 150, 200, 300, 400, 500,
    ];

    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        Self(value)
    }

    /// Returns the thickness in 0.01mm units.
    pub const fn value_hundredths_mm(&self) -> u16 {
        if (self.0 as usize) < Self::VALUES.len() {
            Self::VALUES[self.0 as usize]
        } else {
            10
        }
    }

    /// Returns the thickness in millimeters.
    pub fn value_mm(&self) -> f64 {
        self.value_hundredths_mm() as f64 / 100.0
    }
}

/// Diagonal line type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiagonalType {
    #[default]
    Slash,
    BackSlash,
    CrookedSlash,
}

impl DiagonalType {
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Slash,
            1 => Self::BackSlash,
            2 => Self::CrookedSlash,
            _ => Self::Slash,
        }
    }
}

/// Fill type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FillType {
    #[default]
    None,
    Solid,
    Image,
    Gradient,
}

impl FillType {
    pub const fn from_raw(value: u32) -> Self {
        if value & 0x01 != 0 {
            Self::Solid
        } else if value & 0x02 != 0 {
            Self::Image
        } else if value & 0x04 != 0 {
            Self::Gradient
        } else {
            Self::None
        }
    }
}

/// Pattern type for solid fills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PatternType {
    #[default]
    None,
    Horizontal,
    Vertical,
    BackSlash,
    Slash,
    Cross,
    CrossDiagonal,
}

impl PatternType {
    pub const fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Horizontal,
            2 => Self::Vertical,
            3 => Self::BackSlash,
            4 => Self::Slash,
            5 => Self::Cross,
            6 => Self::CrossDiagonal,
            _ => Self::None,
        }
    }
}

/// Gradient type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GradientType {
    #[default]
    Linear,
    Radial,
    Conical,
    Rectangular,
}

impl GradientType {
    pub const fn from_raw(value: i16) -> Self {
        match value {
            1 => Self::Linear,
            2 => Self::Radial,
            3 => Self::Conical,
            4 => Self::Rectangular,
            _ => Self::Linear,
        }
    }
}

/// Image fill type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageFillType {
    #[default]
    TileAll,
    TileHorizontalTop,
    TileHorizontalBottom,
    TileVerticalLeft,
    TileVerticalRight,
    FitToSize,
    Center,
    CenterTop,
    CenterBottom,
    CenterLeft,
    TopLeft,
    BottomLeft,
    CenterRight,
    TopRight,
    BottomRight,
    None,
}

impl ImageFillType {
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::TileAll,
            1 => Self::TileHorizontalTop,
            2 => Self::TileHorizontalBottom,
            3 => Self::TileVerticalLeft,
            4 => Self::TileVerticalRight,
            5 => Self::FitToSize,
            6 => Self::Center,
            7 => Self::CenterTop,
            8 => Self::CenterBottom,
            9 => Self::CenterLeft,
            10 => Self::TopLeft,
            11 => Self::BottomLeft,
            12 => Self::CenterRight,
            13 => Self::TopRight,
            14 => Self::BottomRight,
            15 => Self::None,
            _ => Self::TileAll,
        }
    }
}

/// Pattern fill info.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PatternFill {
    pub background_color: ColorReference,
    pub pattern_color: ColorReference,
    pub pattern_type: PatternType,
}

/// Gradient fill info.
#[derive(Debug, Clone, PartialEq)]
pub struct GradientFill {
    pub gradient_type: GradientType,
    pub angle: i16,
    pub center_x: i16,
    pub center_y: i16,
    pub blur: i16,
    pub colors: Vec<ColorReference>,
}

/// Image info.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageInfo {
    pub brightness: i8,
    pub contrast: i8,
    pub effect: u8,
    pub binary_data_id: u16,
}

impl ImageInfo {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            brightness: reader.read_i8()?,
            contrast: reader.read_i8()?,
            effect: reader.read_u8()?,
            binary_data_id: reader.read_u16()?,
        })
    }
}

/// Image fill info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageFill {
    pub fill_type: ImageFillType,
    pub image_info: ImageInfo,
}

/// Fill information.
#[derive(Debug, Clone, PartialEq)]
pub enum FillInfo {
    None,
    Pattern(PatternFill),
    Gradient(GradientFill),
    Image(ImageFill),
}

/// Border and fill definition.
#[derive(Debug, Clone)]
pub struct BorderFill {
    properties: u16,
    border_styles: [BorderLineStyle; 4],
    border_thicknesses: [BorderLineThickness; 4],
    border_colors: [ColorReference; 4],
    diagonal_style: BorderLineStyle,
    diagonal_thickness: BorderLineThickness,
    diagonal_color: ColorReference,
    fill_info: FillInfo,
}

impl BorderFill {
    /// Parses BorderFill from a reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = reader.read_u16()?;

        // Read 4 border styles
        let mut border_styles = [BorderLineStyle::default(); 4];
        for style in &mut border_styles {
            *style = BorderLineStyle::from_raw(reader.read_u8()?);
        }

        // Read 4 border thicknesses
        let mut border_thicknesses = [BorderLineThickness::default(); 4];
        for thickness in &mut border_thicknesses {
            *thickness = BorderLineThickness::from_raw(reader.read_u8()?);
        }

        // Read 4 border colors
        let mut border_colors = [ColorReference::default(); 4];
        for color in &mut border_colors {
            *color = reader.read_color()?;
        }

        // Read diagonal
        let diagonal_style = BorderLineStyle::from_raw(reader.read_u8()?);
        let diagonal_thickness = BorderLineThickness::from_raw(reader.read_u8()?);
        let diagonal_color = reader.read_color()?;

        // Read fill info
        let fill_type_raw = reader.read_u32()?;
        let fill_type = FillType::from_raw(fill_type_raw);

        let fill_info = match fill_type {
            FillType::Solid => {
                let background_color = reader.read_color()?;
                let pattern_color = reader.read_color()?;
                let pattern_type = PatternType::from_raw(reader.read_i32()?);
                FillInfo::Pattern(PatternFill {
                    background_color,
                    pattern_color,
                    pattern_type,
                })
            }
            FillType::Gradient => {
                let gradient_type = GradientType::from_raw(reader.read_i16()?);
                let angle = reader.read_i16()?;
                let center_x = reader.read_i16()?;
                let center_y = reader.read_i16()?;
                let blur = reader.read_i16()?;
                let color_count = reader.read_i16()? as usize;

                // Skip position data if more than 2 colors
                if color_count > 2 {
                    reader.skip(4 * color_count)?;
                }

                let mut colors = Vec::with_capacity(color_count);
                for _ in 0..color_count {
                    colors.push(reader.read_color()?);
                }

                FillInfo::Gradient(GradientFill {
                    gradient_type,
                    angle,
                    center_x,
                    center_y,
                    blur,
                    colors,
                })
            }
            FillType::Image => {
                let fill_type = ImageFillType::from_raw(reader.read_u8()?);
                let image_info = ImageInfo::from_reader(reader)?;
                FillInfo::Image(ImageFill {
                    fill_type,
                    image_info,
                })
            }
            FillType::None => FillInfo::None,
        };

        Ok(Self {
            properties,
            border_styles,
            border_thicknesses,
            border_colors,
            diagonal_style,
            diagonal_thickness,
            diagonal_color,
            fill_info,
        })
    }

    /// Returns whether 3D effect is enabled.
    pub const fn has_3d_effect(&self) -> bool {
        (self.properties & (1 << 0)) != 0
    }

    /// Returns whether shadow effect is enabled.
    pub const fn has_shadow(&self) -> bool {
        (self.properties & (1 << 1)) != 0
    }

    /// Returns border styles as [left, right, top, bottom].
    pub const fn border_styles(&self) -> &[BorderLineStyle; 4] {
        &self.border_styles
    }

    /// Returns border thicknesses as [left, right, top, bottom].
    pub const fn border_thicknesses(&self) -> &[BorderLineThickness; 4] {
        &self.border_thicknesses
    }

    /// Returns border colors as [left, right, top, bottom].
    pub const fn border_colors(&self) -> &[ColorReference; 4] {
        &self.border_colors
    }

    /// Returns the fill info.
    pub const fn fill_info(&self) -> &FillInfo {
        &self.fill_info
    }

    /// Returns the diagonal line style.
    pub const fn diagonal_style(&self) -> BorderLineStyle {
        self.diagonal_style
    }

    /// Returns the diagonal line thickness.
    pub const fn diagonal_thickness(&self) -> BorderLineThickness {
        self.diagonal_thickness
    }

    /// Returns the diagonal line color.
    pub const fn diagonal_color(&self) -> ColorReference {
        self.diagonal_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_line_thickness() {
        let t = BorderLineThickness::from_raw(0);
        assert_eq!(t.value_hundredths_mm(), 10);
        assert!((t.value_mm() - 0.1).abs() < f64::EPSILON);

        let t = BorderLineThickness::from_raw(10);
        assert_eq!(t.value_hundredths_mm(), 100);
        assert!((t.value_mm() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_fill_type() {
        assert_eq!(FillType::from_raw(0), FillType::None);
        assert_eq!(FillType::from_raw(1), FillType::Solid);
        assert_eq!(FillType::from_raw(2), FillType::Image);
        assert_eq!(FillType::from_raw(4), FillType::Gradient);
    }
}
