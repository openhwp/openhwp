//! Shape (drawing object) parsing.
//!
//! Shapes include lines, rectangles, ellipses, arcs, polygons, and curves.

use crate::doc_info::{
    FillInfo, FillType, GradientFill, GradientType, ImageFill, ImageFillType, ImageInfo,
    PatternFill, PatternType,
};
use crate::error::Result;
use crate::primitive::ColorReference;
use primitive::HwpUnit;
use crate::util::ByteReader;

/// Line end cap style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineEndCap {
    /// Round cap.
    #[default]
    Round,
    /// Flat cap.
    Flat,
}

impl LineEndCap {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Round,
            _ => Self::Flat,
        }
    }
}

/// Arrow head type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrowType {
    /// No arrow.
    #[default]
    None,
    /// Arrow head.
    Arrow,
    /// Arrow head (spear).
    Spear,
    /// Concave arrow.
    ConcaveArrow,
    /// Empty diamond.
    EmptyDiamond,
    /// Empty circle.
    EmptyCircle,
    /// Empty box.
    EmptyBox,
    /// Filled diamond.
    FilledDiamond,
    /// Filled circle.
    FilledCircle,
    /// Filled box.
    FilledBox,
}

impl ArrowType {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Arrow,
            2 => Self::Spear,
            3 => Self::ConcaveArrow,
            4 => Self::EmptyDiamond,
            5 => Self::EmptyCircle,
            6 => Self::EmptyBox,
            7 => Self::FilledDiamond,
            8 => Self::FilledCircle,
            9 => Self::FilledBox,
            _ => Self::None,
        }
    }
}

/// Arrow head size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrowSize {
    /// Smallest size.
    Smallest,
    /// Small size.
    Small,
    /// Medium size.
    #[default]
    Medium,
    /// Large size.
    Large,
    /// Largest size.
    Largest,
}

impl ArrowSize {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Smallest,
            1 => Self::Small,
            2 => Self::Medium,
            3 => Self::Large,
            4 => Self::Largest,
            _ => Self::Medium,
        }
    }
}

/// Border line information for shapes.
#[derive(Debug, Clone, Copy, Default)]
pub struct ShapeBorderLine {
    /// Line color.
    pub color: ColorReference,
    /// Line thickness.
    pub thickness: i32,
    /// Line style properties.
    pub properties: u32,
    /// Outline style.
    pub outline_style: u8,
}

impl ShapeBorderLine {
    /// Size in bytes (표 86: 4 + 2 + 4 + 1 = 11 바이트, but INT16 for thickness).
    pub const SIZE: usize = 11;

    /// Parses from reader (표 86: 테두리 선 정보).
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let color = reader.read_color()?;
        // 표 86에서는 INT16이지만 실제로는 i32로 저장되는 경우도 있음
        // 스펙: INT16 (2바이트) 선 굵기
        let thickness = reader.read_i16()? as i32;
        let properties = reader.read_u32()?;
        let outline_style = reader.read_u8()?;

        Ok(Self {
            color,
            thickness,
            properties,
            outline_style,
        })
    }

    /// Returns the line end cap style.
    pub const fn end_cap(&self) -> LineEndCap {
        LineEndCap::from_raw((self.properties & 0xFF) as u8)
    }

    /// Returns the start arrow type.
    pub const fn start_arrow(&self) -> ArrowType {
        ArrowType::from_raw(((self.properties >> 8) & 0x0F) as u8)
    }

    /// Returns the end arrow type.
    pub const fn end_arrow(&self) -> ArrowType {
        ArrowType::from_raw(((self.properties >> 12) & 0x0F) as u8)
    }

    /// Returns the start arrow size (표 87: bit 22~25).
    pub const fn start_arrow_size(&self) -> ArrowSize {
        ArrowSize::from_raw(((self.properties >> 22) & 0x0F) as u8)
    }

    /// Returns the end arrow size (표 87: bit 26~29).
    pub const fn end_arrow_size(&self) -> ArrowSize {
        ArrowSize::from_raw(((self.properties >> 26) & 0x0F) as u8)
    }
}

/// A 2D point.
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    /// X coordinate.
    pub x: HwpUnit,
    /// Y coordinate.
    pub y: HwpUnit,
}

impl Point {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            x: reader.read_signed_hwp_unit()?,
            y: reader.read_signed_hwp_unit()?,
        })
    }
}

/// Shape element common properties.
#[derive(Debug, Clone, Default)]
pub struct ShapeElementProperties {
    /// Element type.
    pub element_type: u32,
    /// Properties flags.
    pub properties: u32,
    /// Rotation angle in degrees.
    pub rotation: i16,
    /// Center X coordinate.
    pub center_x: HwpUnit,
    /// Center Y coordinate.
    pub center_y: HwpUnit,
    /// Number of matrix elements.
    pub matrix_count: u16,
    /// Transformation matrix.
    pub matrix: Vec<f64>,
}

impl ShapeElementProperties {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let element_type = reader.read_u32()?;
        let properties = reader.read_u32()?;
        let rotation = reader.read_i16()?;
        let center_x = reader.read_signed_hwp_unit()?;
        let center_y = reader.read_signed_hwp_unit()?;
        let matrix_count = reader.read_u16()?;

        let mut matrix = Vec::with_capacity(matrix_count as usize);
        for _ in 0..matrix_count {
            // Matrix elements are stored as HWPUNIT (i32) but represent fixed-point values
            let value = reader.read_i32()? as f64 / 65536.0;
            matrix.push(value);
        }

        Ok(Self {
            element_type,
            properties,
            rotation,
            center_x,
            center_y,
            matrix_count,
            matrix,
        })
    }

    /// Returns true if shape flipped horizontally.
    pub const fn is_flipped_horizontal(&self) -> bool {
        (self.properties & (1 << 0)) != 0
    }

    /// Returns true if shape flipped vertically.
    pub const fn is_flipped_vertical(&self) -> bool {
        (self.properties & (1 << 1)) != 0
    }
}

/// Line shape data.
#[derive(Debug, Clone, Default)]
pub struct LineShape {
    /// Start point.
    pub start: Point,
    /// End point.
    pub end: Point,
    /// Whether arrow at start.
    pub is_reversed: bool,
}

impl LineShape {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            start: Point::from_reader(reader)?,
            end: Point::from_reader(reader)?,
            is_reversed: reader.read_u32()? != 0,
        })
    }
}

/// Rectangle shape data.
#[derive(Debug, Clone, Default)]
pub struct RectangleShape {
    /// Corner rounding ratio (0-100).
    pub round_ratio: u8,
    /// Corner points (4 corners, x1, y1, x2, y2, etc.).
    pub corners: [Point; 4],
}

impl RectangleShape {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let round_ratio = reader.read_u8()?;
        let mut corners = [Point::default(); 4];
        for corner in &mut corners {
            *corner = Point::from_reader(reader)?;
        }
        Ok(Self {
            round_ratio,
            corners,
        })
    }
}

/// Arc type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArcType {
    /// Normal arc.
    #[default]
    Arc,
    /// Pie (wedge).
    Pie,
    /// Chord.
    Chord,
}

impl ArcType {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Arc,
            1 => Self::Pie,
            _ => Self::Chord,
        }
    }
}

/// Ellipse shape data.
#[derive(Debug, Clone, Default)]
pub struct EllipseShape {
    /// Properties flags.
    pub properties: u32,
    /// Center point.
    pub center: Point,
    /// First axis point.
    pub axis1: Point,
    /// Second axis point.
    pub axis2: Point,
    /// Start point (for arc).
    pub start: Point,
    /// End point (for arc).
    pub end: Point,
    /// Start point 2 (for arc).
    pub start2: Point,
    /// End point 2 (for arc).
    pub end2: Point,
}

impl EllipseShape {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            properties: reader.read_u32()?,
            center: Point::from_reader(reader)?,
            axis1: Point::from_reader(reader)?,
            axis2: Point::from_reader(reader)?,
            start: Point::from_reader(reader)?,
            end: Point::from_reader(reader)?,
            start2: Point::from_reader(reader)?,
            end2: Point::from_reader(reader)?,
        })
    }

    /// Returns the arc type.
    pub const fn arc_type(&self) -> ArcType {
        ArcType::from_raw((self.properties & 0xFF) as u8)
    }
}

/// Arc shape data.
#[derive(Debug, Clone, Default)]
pub struct ArcShape {
    /// Arc type.
    pub arc_type: ArcType,
    /// Center point.
    pub center: Point,
    /// First axis point.
    pub axis1: Point,
    /// Second axis point.
    pub axis2: Point,
}

impl ArcShape {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            arc_type: ArcType::from_raw(reader.read_u8()?),
            center: Point::from_reader(reader)?,
            axis1: Point::from_reader(reader)?,
            axis2: Point::from_reader(reader)?,
        })
    }
}

/// Polygon shape data.
#[derive(Debug, Clone, Default)]
pub struct PolygonShape {
    /// Points.
    pub points: Vec<Point>,
}

impl PolygonShape {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let count = reader.read_u32()? as usize;
        let mut points = Vec::with_capacity(count);
        for _ in 0..count {
            points.push(Point::from_reader(reader)?);
        }
        Ok(Self { points })
    }
}

/// Curve segment type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurveSegmentType {
    /// Line segment.
    #[default]
    Line,
    /// Curve segment.
    Curve,
}

impl CurveSegmentType {
    /// Creates from raw value.
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Line,
            _ => Self::Curve,
        }
    }
}

/// Curve shape data.
#[derive(Debug, Clone, Default)]
pub struct CurveShape {
    /// Points.
    pub points: Vec<Point>,
    /// Segment types.
    pub segment_types: Vec<CurveSegmentType>,
}

impl CurveShape {
    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let count = reader.read_u32()? as usize;
        let mut points = Vec::with_capacity(count);
        for _ in 0..count {
            points.push(Point::from_reader(reader)?);
        }

        let mut segment_types = Vec::with_capacity(count);
        for _ in 0..count {
            segment_types.push(CurveSegmentType::from_raw(reader.read_u8()?));
        }

        Ok(Self {
            points,
            segment_types,
        })
    }
}

/// Shape type enumeration.
#[derive(Debug, Clone)]
pub enum ShapeType {
    /// Line shape.
    Line(LineShape),
    /// Rectangle shape.
    Rectangle(RectangleShape),
    /// Ellipse shape.
    Ellipse(EllipseShape),
    /// Arc shape.
    Arc(ArcShape),
    /// Polygon shape.
    Polygon(PolygonShape),
    /// Curve shape.
    Curve(CurveShape),
    /// Container (grouped shapes).
    Container(Vec<Shape>),
    /// Unknown shape type.
    Unknown,
}

impl Default for ShapeType {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Shape text box properties (표 90: 그리기 개체 글상자용 텍스트 속성).
#[derive(Debug, Clone, Default)]
pub struct ShapeTextBox {
    /// Left margin.
    pub margin_left: u16,
    /// Right margin.
    pub margin_right: u16,
    /// Top margin.
    pub margin_top: u16,
    /// Bottom margin.
    pub margin_bottom: u16,
    /// Maximum text width.
    pub max_width: u32,
}

impl ShapeTextBox {
    /// Size in bytes.
    pub const SIZE: usize = 12;

    /// Parses from reader.
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            margin_left: reader.read_u16()?,
            margin_right: reader.read_u16()?,
            margin_top: reader.read_u16()?,
            margin_bottom: reader.read_u16()?,
            max_width: reader.read_u32()?,
        })
    }
}

/// A shape (drawing object) in the document.
#[derive(Debug, Clone, Default)]
pub struct Shape {
    /// Shape element properties.
    pub element_properties: ShapeElementProperties,
    /// Border line.
    pub border_line: ShapeBorderLine,
    /// Fill information.
    pub fill: FillInfo,
    /// Text box properties (if shape contains text).
    pub text_box: Option<ShapeTextBox>,
    /// Shape type and data.
    pub shape_type: ShapeType,
    /// Paragraphs inside the shape (for shapes with text).
    pub paragraphs: Vec<super::Paragraph>,
    /// Expected paragraph count from ListHeader (internal use).
    pub(crate) expected_paragraph_count: u16,
}

impl Shape {
    /// Creates a new shape.
    pub fn new(element_properties: ShapeElementProperties) -> Self {
        Self {
            element_properties,
            border_line: ShapeBorderLine::default(),
            fill: FillInfo::None,
            text_box: None,
            shape_type: ShapeType::Unknown,
            paragraphs: Vec::new(),
            expected_paragraph_count: 0,
        }
    }

    /// Adds a paragraph to the shape.
    pub fn add_paragraph(&mut self, para: super::Paragraph) {
        self.paragraphs.push(para);
    }

    /// Returns the paragraphs inside the shape.
    pub fn paragraphs(&self) -> &[super::Paragraph] {
        &self.paragraphs
    }

    /// Returns true if the shape needs more paragraphs.
    pub fn needs_more_paragraphs(&self) -> bool {
        self.expected_paragraph_count > 0 && self.paragraphs.len() < self.expected_paragraph_count as usize
    }

    /// Sets the expected paragraph count from ListHeader.
    pub fn set_expected_paragraph_count(&mut self, count: u16) {
        self.expected_paragraph_count = count;
    }

    /// Returns the rotation angle in degrees.
    pub fn rotation(&self) -> i16 {
        self.element_properties.rotation
    }

    /// Returns the center point.
    pub fn center(&self) -> Point {
        Point {
            x: self.element_properties.center_x,
            y: self.element_properties.center_y,
        }
    }
}

/// Drawing object common properties (표 81: 그리기 개체 공통 속성).
#[derive(Debug, Clone, Default)]
pub struct DrawingObjectCommon {
    /// Shape element properties.
    pub element_properties: ShapeElementProperties,
    /// Border line.
    pub border_line: ShapeBorderLine,
    /// Fill information.
    pub fill: FillInfo,
    /// Text box properties (if shape contains text).
    pub text_box: Option<ShapeTextBox>,
}

impl DrawingObjectCommon {
    /// Parses drawing object common properties from reader.
    /// This includes: element properties + border line (11 bytes) + fill info (variable) + text box (12 bytes).
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        let element_properties = ShapeElementProperties::from_reader(reader)?;

        // 테두리 선 정보 (표 86: 11 바이트)
        let border_line = if reader.remaining() >= ShapeBorderLine::SIZE {
            ShapeBorderLine::from_reader(reader)?
        } else {
            ShapeBorderLine::default()
        };

        // 채우기 정보 (표 28: 가변)
        let fill = parse_fill_info(reader)?;

        // 추가 속성 (표 28 끝부분): DWORD + 가변
        if reader.remaining() >= 4 {
            let size = reader.read_u32()? as usize;
            if size > 0 && reader.remaining() >= size {
                reader.skip(size)?;
            }
        }

        // 추가 채우기 속성 길이
        if reader.remaining() >= 4 {
            let size = reader.read_u32()? as usize;
            if size > 0 && reader.remaining() >= size {
                reader.skip(size)?;
            }
        }

        // 글상자 텍스트 정보 (표 89-90: 12바이트 속성 + 문단 리스트)
        let text_box = if reader.remaining() >= ShapeTextBox::SIZE {
            Some(ShapeTextBox::from_reader(reader)?)
        } else {
            None
        };

        Ok(Self {
            element_properties,
            border_line,
            fill,
            text_box,
        })
    }
}

/// Parses fill information from a reader (표 28 채우기 정보).
pub fn parse_fill_info(reader: &mut ByteReader) -> Result<FillInfo> {
    if reader.remaining() < 4 {
        return Ok(FillInfo::None);
    }

    let fill_type_raw = reader.read_u32()?;
    let fill_type = FillType::from_raw(fill_type_raw);

    match fill_type {
        FillType::Solid => {
            if reader.remaining() < 12 {
                return Ok(FillInfo::None);
            }
            let background_color = reader.read_color()?;
            let pattern_color = reader.read_color()?;
            let pattern_type = PatternType::from_raw(reader.read_i32()?);
            Ok(FillInfo::Pattern(PatternFill {
                background_color,
                pattern_color,
                pattern_type,
            }))
        }
        FillType::Gradient => {
            if reader.remaining() < 12 {
                return Ok(FillInfo::None);
            }
            let gradient_type = GradientType::from_raw(reader.read_i16()?);
            let angle = reader.read_i16()?;
            let center_x = reader.read_i16()?;
            let center_y = reader.read_i16()?;
            let blur = reader.read_i16()?;
            let color_count = reader.read_i16()? as usize;

            // Skip position data if more than 2 colors
            if color_count > 2 && reader.remaining() >= 4 * color_count {
                reader.skip(4 * color_count)?;
            }

            let mut colors = Vec::with_capacity(color_count);
            for _ in 0..color_count {
                if reader.remaining() >= 4 {
                    colors.push(reader.read_color()?);
                }
            }

            Ok(FillInfo::Gradient(GradientFill {
                gradient_type,
                angle,
                center_x,
                center_y,
                blur,
                colors,
            }))
        }
        FillType::Image => {
            if reader.remaining() < 6 {
                return Ok(FillInfo::None);
            }
            let fill_type = ImageFillType::from_raw(reader.read_u8()?);
            let image_info = ImageInfo::from_reader(reader)?;
            Ok(FillInfo::Image(ImageFill {
                fill_type,
                image_info,
            }))
        }
        FillType::None => Ok(FillInfo::None),
    }
}
