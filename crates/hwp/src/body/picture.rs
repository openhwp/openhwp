//! Picture (image) control parsing.
//!
//! Pictures in HWP documents can contain various image formats including
//! BMP, JPG, PNG, GIF, TIFF, and Windows Metafile (WMF/EMF).

use crate::error::Result;
use crate::primitive::{HwpUnit16, SignedHwpUnit};
use crate::util::ByteReader;

use super::shape::Point;

/// Picture effect type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PictureEffectType {
    /// No effect (real picture).
    #[default]
    RealPicture,
    /// Grayscale effect.
    Grayscale,
    /// Black and white effect.
    BlackWhite,
    /// Pattern 8x8 effect.
    Pattern8x8,
}

impl PictureEffectType {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::RealPicture,
            1 => Self::Grayscale,
            2 => Self::BlackWhite,
            3 => Self::Pattern8x8,
            _ => Self::RealPicture,
        }
    }
}

/// Picture brightness/contrast/effect settings.
#[derive(Debug, Clone, Default)]
pub struct PictureEffect {
    /// Brightness (-100 to 100).
    pub brightness: i8,
    /// Contrast (-100 to 100).
    pub contrast: i8,
    /// Effect type.
    pub effect_type: PictureEffectType,
    /// Binary picture reference (BinData ID) for pattern effect.
    pub binary_pattern: u16,
}

impl PictureEffect {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            brightness: reader.read_i8()?,
            contrast: reader.read_i8()?,
            effect_type: PictureEffectType::from_raw(reader.read_u8()?),
            binary_pattern: reader.read_u16()?,
        })
    }
}

/// Image flip type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageFlip {
    /// No flip.
    #[default]
    None,
    /// Horizontal flip.
    Horizontal,
    /// Vertical flip.
    Vertical,
    /// Both horizontal and vertical flip.
    Both,
}

impl ImageFlip {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Horizontal,
            2 => Self::Vertical,
            3 => Self::Both,
            _ => Self::None,
        }
    }
}

/// Picture fill information.
#[derive(Debug, Clone, Default)]
pub struct PictureFill {
    /// Fill type.
    pub fill_type: u8,
    /// Fill color (ARGB).
    pub fill_color: u32,
}

/// Image cropping information.
#[derive(Debug, Clone, Copy, Default)]
pub struct ImageCrop {
    /// Left crop amount.
    pub left: SignedHwpUnit,
    /// Top crop amount.
    pub top: SignedHwpUnit,
    /// Right crop amount.
    pub right: SignedHwpUnit,
    /// Bottom crop amount.
    pub bottom: SignedHwpUnit,
}

impl ImageCrop {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            left: reader.read_signed_hwp_unit()?,
            top: reader.read_signed_hwp_unit()?,
            right: reader.read_signed_hwp_unit()?,
            bottom: reader.read_signed_hwp_unit()?,
        })
    }
}

/// Inner margin information for picture.
#[derive(Debug, Clone, Copy, Default)]
pub struct InnerMargin {
    /// Left margin.
    pub left: HwpUnit16,
    /// Right margin.
    pub right: HwpUnit16,
    /// Top margin.
    pub top: HwpUnit16,
    /// Bottom margin.
    pub bottom: HwpUnit16,
}

impl InnerMargin {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            left: reader.read_hwp_unit16()?,
            right: reader.read_hwp_unit16()?,
            top: reader.read_hwp_unit16()?,
            bottom: reader.read_hwp_unit16()?,
        })
    }
}

/// Picture properties.
#[derive(Debug, Clone, Default)]
pub struct PictureProperties {
    /// Border color.
    pub border_color: u32,
    /// Border thickness.
    pub border_thickness: i32,
    /// Border properties.
    pub border_properties: u32,
    /// Corner points (4 corners).
    pub corners: [Point; 4],
    /// Image crop settings.
    pub crop: ImageCrop,
    /// Inner margin.
    pub inner_margin: InnerMargin,
    /// Picture effect settings.
    pub effect: PictureEffect,
    /// Binary data ID reference.
    pub binary_data_id: u16,
    /// Border transparency.
    pub border_transparency: u8,
    /// Instance ID.
    pub instance_id: u32,
    /// Image dimension in pixels.
    pub image_dimension: (u32, u32),
}

impl PictureProperties {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let border_color = reader.read_u32()?;
        let border_thickness = reader.read_i32()?;
        let border_properties = reader.read_u32()?;

        let mut corners = [Point::default(); 4];
        for corner in &mut corners {
            *corner = Point::from_reader(reader)?;
        }

        let crop = ImageCrop::from_reader(reader)?;
        let inner_margin = InnerMargin::from_reader(reader)?;
        let effect = PictureEffect::from_reader(reader)?;
        let binary_data_id = reader.read_u16()?;
        let border_transparency = reader.read_u8()?;

        // Version 5.0.2.4+ fields
        let instance_id = if reader.remaining() >= 4 {
            reader.read_u32()?
        } else {
            0
        };

        // Version 5.0.2.7+ fields
        let image_dimension = if reader.remaining() >= 8 {
            let width = reader.read_u32()?;
            let height = reader.read_u32()?;
            (width, height)
        } else {
            (0, 0)
        };

        Ok(Self {
            border_color,
            border_thickness,
            border_properties,
            corners,
            crop,
            inner_margin,
            effect,
            binary_data_id,
            border_transparency,
            instance_id,
            image_dimension,
        })
    }

    /// Returns true if the picture has a border.
    pub const fn has_border(&self) -> bool {
        self.border_thickness > 0
    }
}

/// Picture (image) in the document.
#[derive(Debug, Clone, Default)]
pub struct Picture {
    /// Picture properties.
    pub properties: PictureProperties,
    /// Original file name (if known).
    pub original_filename: Option<String>,
    /// Caption text (if any).
    pub caption: Option<String>,
}

impl Picture {
    /// Creates a new picture with properties.
    pub fn new(properties: PictureProperties) -> Self {
        Self {
            properties,
            original_filename: None,
            caption: None,
        }
    }

    /// Parses picture from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let properties = PictureProperties::from_reader(reader)?;
        Ok(Self::new(properties))
    }

    /// Returns the binary data ID for the image content.
    pub fn binary_data_id(&self) -> u16 {
        self.properties.binary_data_id
    }

    /// Returns the image dimensions in pixels.
    pub fn dimensions(&self) -> (u32, u32) {
        self.properties.image_dimension
    }

    /// Returns the image width in pixels.
    pub fn width(&self) -> u32 {
        self.properties.image_dimension.0
    }

    /// Returns the image height in pixels.
    pub fn height(&self) -> u32 {
        self.properties.image_dimension.1
    }
}

/// OLE (Object Linking and Embedding) object.
#[derive(Debug, Clone, Default)]
pub struct OleObject {
    /// Object properties.
    pub properties: u32,
    /// Object extent width.
    pub extent_width: i32,
    /// Object extent height.
    pub extent_height: i32,
    /// Binary data ID reference.
    pub binary_data_id: u16,
    /// Border color.
    pub border_color: u32,
    /// Border thickness.
    pub border_thickness: i32,
    /// Border properties.
    pub border_properties: u32,
}

impl OleObject {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            properties: reader.read_u32()?,
            extent_width: reader.read_i32()?,
            extent_height: reader.read_i32()?,
            binary_data_id: reader.read_u16()?,
            border_color: reader.read_u32()?,
            border_thickness: reader.read_i32()?,
            border_properties: reader.read_u32()?,
        })
    }

    /// Returns true if object is linked (vs embedded).
    pub const fn is_linked(&self) -> bool {
        (self.properties & (1 << 0)) != 0
    }

    /// Returns true if object has been converted.
    pub const fn is_converted(&self) -> bool {
        (self.properties & (1 << 1)) != 0
    }

    /// Returns true if object is an icon.
    pub const fn is_icon(&self) -> bool {
        (self.properties & (1 << 2)) != 0
    }
}
