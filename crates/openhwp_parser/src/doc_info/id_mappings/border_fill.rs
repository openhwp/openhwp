use super::IdMappingCount;
use crate::{DocInfoIter, HwpTag, u16, u32};

#[derive(Debug)]
pub struct BorderFill {
    pub slash_diagonal: SlashDiagonal,
    pub borders: [Border; 4],
    pub diagonal: Diagonal,
    pub fill: Fill,
}

#[derive(Debug)]
pub struct SlashDiagonal {
    pub effect_3d: bool,
    pub effect_shadow: bool,
    pub shape: SlashDiagonalShape,
    pub back_shape: BackSlashDiagonalShape,
    pub broken_line: bool,
    pub broken_back_line: bool,
    pub line_rotated: bool,
    pub back_line_rotated: bool,
    pub center_line: bool,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SlashDiagonalShape {
    /// none
    None = 0b_0000,
    /// slash
    Slash = 0b_0010,
    /// LeftTop --> Bottom Edge
    LeftTop2BottomEdge = 0b_0011,
    /// LeftTop --> Right Edge
    LeftTop2RightEdge = 0b_0110,
    /// LeftTop --> Bottom & Right Edge
    LeftTop2BottomAndRightEdge = 0b_0111,
    Unknown(u8),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BackSlashDiagonalShape {
    /// none
    None = 0b_0000,
    /// back slash
    BackSlash = 0b_0010,
    /// RightTop --> Bottom Edge
    RightTop2BottomEdge = 0b_0011,
    /// RightTop --> Left Edge
    RightTop2LeftEdge = 0b_0110,
    /// RightTop --> Bottom & Left Edge
    RightTop2BottomAndLeftEdge = 0b_0111,
    Unknown(u8),
}

#[derive(Debug)]
pub struct Border {
    /// 4방향 테두리선 종류
    pub shape: BorderShape,
    /// 4방향 테두리선 굵기
    pub width: BorderWidth,
    /// 4방향 테두리선
    pub color: Color,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BorderShape {
    /// 실선
    Solid = 0,
    /// 긴 점선
    Dashed = 1,
    /// 점선
    Dotted = 2,
    /// -.-.-.-.
    DashDot = 3,
    /// -..-..-..
    DashDotDot = 4,
    /// Dash보다 긴 선분의 반복
    LongDash = 5,
    /// Dot보다 큰 동그라미의 반복
    Circle = 6,
    /// 2중선
    Double = 7,
    /// 가는선 + 굵은선 2중선
    SlimThick = 8,
    /// 굵은선 + 가는선 2중선
    ThickSlim = 9,
    /// 가는선 + 굵은선 + 가는선 3중선
    SlimThickSlim = 10,
    /// 물결
    Wave = 11,
    /// 물결 2중선
    DoubleWave = 12,
    /// 두꺼운 3D
    Thick3D = 13,
    /// 두꺼운 3D(광원 반대)
    Thick3DInset = 14,
    /// 3D 단선
    Slim3D = 15,
    /// 3D 단선(광원 반대)
    Slim3DInset = 16,
    Unknown(u8),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BorderWidth {
    /// 0.1
    W0_1 = 0,
    /// 0.12
    W0_12 = 1,
    /// 0.15
    W0_15 = 2,
    /// 0.2
    W0_2 = 3,
    /// 0.25
    W0_25 = 4,
    /// 0.3
    W0_3 = 5,
    /// 0.4
    W0_4 = 6,
    /// 0.5
    W0_5 = 7,
    /// 0.6
    W0_6 = 8,
    /// 0.7
    W0_7 = 9,
    /// 1.0
    W1_0 = 10,
    /// 1.5
    W1_5 = 11,
    /// 2.0
    W2_0 = 12,
    /// 3.0
    W3_0 = 13,
    /// 4.0
    W4_0 = 14,
    /// 5.0
    W5_0 = 15,
    Unknown(u8),
}

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug)]
pub struct Diagonal {
    pub kind: DiagonalKind,
    pub width: BorderWidth,
    pub color: Color,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DiagonalKind {
    Slash = 0,
    BackSlash = 1,
    CrookedSlash = 2,
    Unknown(u8),
}

#[derive(Debug)]
pub enum Fill {
    None,
    Solid(FillSolid),
    Gradation(FillGradation),
    Image(FillImage),
    Unknown(u8),
}

#[derive(Debug)]
pub struct FillSolid {
    /// 배경색
    pub background_color: Color,
    /// 무늬색
    pub pattern_color: Color,
    /// 무늬 종류
    pub pattern_kind: PatternKind,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PatternKind {
    /// 없음
    None,
    /// - - - -
    Horizontal,
    /// |||||
    Vertical,
    /// \\\\\
    BackSlash,
    /// `/////`
    Slash,
    /// +++++
    Cross,
    /// xxxxx
    CrossDiagonal,
    Unknown(u8),
}

#[derive(Debug)]
pub struct FillImage {
    /// 이미지 채우기 유형
    pub kind: ImageFillKind,
    /// 이미지 정보
    pub image: Image,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ImageFillKind {
    /// 바둑판식으로-모두
    Tile = 0,
    /// 바둑판식으로-가로/위
    TileHorizontalTop = 1,
    /// 바둑판식으로-가로/아래
    TileHorizontalBottom = 2,
    /// 바둑판식으로-세로/왼쪽
    TileVerticalLeft = 3,
    /// 바둑판식으로-세로/오른쪽
    TileVerticalRight = 4,
    /// 크기에 맞추어
    Total = 5,
    /// 가운데로
    Center = 6,
    /// 가운데 위로
    CenterTop = 7,
    /// 가운데 아래로
    CenterBottom = 8,
    /// 왼쪽 가운데로
    CenterLeft = 9,
    /// 왼쪽 위로
    LeftTop = 10,
    /// 왼쪽 아래로
    LeftBottom = 11,
    /// 오른쪽 가운데로
    RightCenter = 12,
    /// 오른쪽 위로
    RightTop = 13,
    /// 오른쪽 아래로
    RightBottom = 14,
    /// NONE
    None = 15,
    Unknown(u8),
}

#[derive(Debug)]
pub struct Image {
    /// 밝기
    pub bright: u8,
    /// 명암
    pub contrast: u8,
    /// 그림 효과
    pub effect: ImageEffect,
    /// BinItem의 아이디 참조값
    pub bin_item_id: u16,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ImageEffect {
    /// 원래 그림에서
    RealPic,
    /// 그레이스케일로
    GrayScale,
    /// 흑백으로
    BlackWhite,
    /// 패턴 8x8
    Pattern8x8,
    Unknown(u8),
}

#[derive(Debug)]
pub struct FillGradation {
    /// 그러데이션 유형
    pub kind: GradationKind,
    /// 그러데이션의 기울임(시작 각)
    pub angle: u32,
    /// 그러데이션의 가로 중심(중심 X 좌표)
    pub center_x: u32,
    /// 그러데이션의 세로 중심(중심 Y 좌표)
    pub center_y: u32,
    /// 그러데이션 번짐 정도
    pub blurry_degree: u32,
    /// 색상이 바뀌는 곳의 위치
    pub change_points: Vec<u32>,
    /// 색상
    pub colors: Vec<Color>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum GradationKind {
    /// 줄무늬형
    Linear = 1,
    /// 원형
    Radial = 2,
    /// 원뿔형
    Conical = 3,
    /// 사각형
    Square = 4,
    Unknown(u8),
}

impl<'hwp> DocInfoIter<'hwp> {
    pub fn border_fills(&mut self, id_mapping_counts: &IdMappingCount) -> Vec<BorderFill> {
        let mut border_fills = Vec::with_capacity(id_mapping_counts.border_fill as usize);

        for _ in 0..id_mapping_counts.border_fill {
            match self.next_if(|record| record.tag == HwpTag::HWPTAG_BORDER_FILL) {
                Some(record) => border_fills.push(BorderFill::from_buf(record.payload)),
                None => break,
            }
        }

        border_fills
    }
}

impl BorderFill {
    pub fn from_buf(buf: &[u8]) -> Self {
        let (slash_diagonal, buf) = buf.split_at(2);
        let (border0, buf) = buf.split_at(6);
        let (border1, buf) = buf.split_at(6);
        let (border2, buf) = buf.split_at(6);
        let (border3, buf) = buf.split_at(6);
        let (diagonal, fill) = buf.split_at(6);

        Self {
            slash_diagonal: SlashDiagonal::from_buf(slash_diagonal),
            borders: [
                Border::from_buf(border0),
                Border::from_buf(border1),
                Border::from_buf(border2),
                Border::from_buf(border3),
            ],
            diagonal: Diagonal::from_buf(diagonal),
            fill: Fill::from_buf(fill),
        }
    }
}

impl SlashDiagonal {
    pub const fn from_buf(buf: &[u8]) -> Self {
        Self {
            effect_3d: buf[0] & 0b_0000_0001 != 0,
            effect_shadow: buf[0] & 0b_0000_0010 != 0,
            shape: match buf[0] & 0b_0001_1100 {
                0b_0000 => SlashDiagonalShape::None,
                0b_0010 => SlashDiagonalShape::Slash,
                0b_0011 => SlashDiagonalShape::LeftTop2BottomEdge,
                0b_0110 => SlashDiagonalShape::LeftTop2RightEdge,
                0b_0111 => SlashDiagonalShape::LeftTop2BottomAndRightEdge,
                shape => SlashDiagonalShape::Unknown(shape),
            },
            back_shape: match buf[0] & 0b_1110_0000 {
                0b_0000 => BackSlashDiagonalShape::None,
                0b_0010 => BackSlashDiagonalShape::BackSlash,
                0b_0011 => BackSlashDiagonalShape::RightTop2BottomEdge,
                0b_0110 => BackSlashDiagonalShape::RightTop2LeftEdge,
                0b_0111 => BackSlashDiagonalShape::RightTop2BottomAndLeftEdge,
                shape => BackSlashDiagonalShape::Unknown(shape),
            },
            broken_line: buf[1] & 0b_0000_0011 != 0,
            broken_back_line: buf[1] & 0b_0000_0100 != 0,
            line_rotated: buf[1] & 0b_0000_1000 != 0,
            back_line_rotated: buf[1] & 0b_0001_0000 != 0,
            center_line: buf[1] & 0b_0010_0000 != 0,
        }
    }
}

impl Border {
    pub const fn from_buf(buf: &[u8]) -> Self {
        let (kind, buf) = buf.split_at(1);
        let (width, color) = buf.split_at(1);

        Self {
            shape: BorderShape::from_buf(kind),
            width: BorderWidth::from_buf(width),
            color: Color::from_buf(color),
        }
    }
}

impl BorderShape {
    pub const fn from_buf(buf: &[u8]) -> Self {
        match buf[0] & 0b_0000_1111 {
            0 => BorderShape::Solid,
            1 => BorderShape::Dashed,
            2 => BorderShape::Dotted,
            3 => BorderShape::DashDot,
            4 => BorderShape::DashDotDot,
            5 => BorderShape::LongDash,
            6 => BorderShape::Circle,
            7 => BorderShape::Double,
            8 => BorderShape::SlimThick,
            9 => BorderShape::ThickSlim,
            10 => BorderShape::SlimThickSlim,
            11 => BorderShape::Wave,
            12 => BorderShape::DoubleWave,
            13 => BorderShape::Thick3D,
            14 => BorderShape::Thick3DInset,
            15 => BorderShape::Slim3D,
            16 => BorderShape::Slim3DInset,
            kind => BorderShape::Unknown(kind),
        }
    }
}

impl BorderWidth {
    pub const fn from_buf(buf: &[u8]) -> Self {
        match buf[0] {
            0 => BorderWidth::W0_1,
            1 => BorderWidth::W0_12,
            2 => BorderWidth::W0_15,
            3 => BorderWidth::W0_2,
            4 => BorderWidth::W0_25,
            5 => BorderWidth::W0_3,
            6 => BorderWidth::W0_4,
            7 => BorderWidth::W0_5,
            8 => BorderWidth::W0_6,
            9 => BorderWidth::W0_7,
            10 => BorderWidth::W1_0,
            11 => BorderWidth::W1_5,
            12 => BorderWidth::W2_0,
            13 => BorderWidth::W3_0,
            14 => BorderWidth::W4_0,
            15 => BorderWidth::W5_0,
            width => BorderWidth::Unknown(width),
        }
    }
}

impl Color {
    pub const fn from_buf(buf: &[u8]) -> Self {
        Self {
            red: buf[0],
            green: buf[1],
            blue: buf[2],
        }
    }
}

impl Diagonal {
    pub const fn from_buf(buf: &[u8]) -> Self {
        let (kind, buf) = buf.split_at(1);
        let kind = match kind[0] {
            0 => DiagonalKind::Slash,
            1 => DiagonalKind::BackSlash,
            2 => DiagonalKind::CrookedSlash,
            kind => DiagonalKind::Unknown(kind),
        };
        let (width, buf) = buf.split_at(1);
        let width = BorderWidth::from_buf(width);
        let color = Color::from_buf(buf);

        Self { kind, width, color }
    }
}

impl Fill {
    pub fn from_buf(buf: &[u8]) -> Self {
        let (r#type, buf) = buf.split_at(4);

        match r#type[0] {
            0 => Self::None,
            1 => Self::Solid(FillSolid::from_buf(buf)),
            2 => Self::Gradation(FillGradation::from_buf(buf)),
            3 => Self::Image(FillImage::from_buf(buf)),
            kind => Self::Unknown(kind),
        }
    }
}

impl FillSolid {
    pub const fn from_buf(buf: &[u8]) -> Self {
        let (background_color, buf) = buf.split_at(4);
        let (pattern_color, pattern_kind) = buf.split_at(4);

        Self {
            background_color: Color::from_buf(background_color),
            pattern_color: Color::from_buf(pattern_color),
            pattern_kind: match pattern_kind[0] {
                0 => PatternKind::None,
                1 => PatternKind::Horizontal,
                2 => PatternKind::Vertical,
                3 => PatternKind::BackSlash,
                4 => PatternKind::Slash,
                5 => PatternKind::Cross,
                6 => PatternKind::CrossDiagonal,
                kind => PatternKind::Unknown(kind),
            },
        }
    }
}

impl Image {
    pub const fn from_buf(buf: &[u8]) -> Self {
        Self {
            bright: buf[0],
            contrast: buf[1],
            effect: match buf[2] {
                0 => ImageEffect::RealPic,
                1 => ImageEffect::GrayScale,
                2 => ImageEffect::BlackWhite,
                3 => ImageEffect::Pattern8x8,
                effect => ImageEffect::Unknown(effect),
            },
            bin_item_id: u16(buf, 3),
        }
    }
}

impl FillGradation {
    pub fn from_buf(buf: &[u8]) -> Self {
        let kind = match buf[0] {
            1 => GradationKind::Linear,
            2 => GradationKind::Radial,
            3 => GradationKind::Conical,
            4 => GradationKind::Square,
            kind => GradationKind::Unknown(kind),
        };
        let angle = u32(buf, 1);
        let center_x = u32(buf, 5);
        let center_y = u32(buf, 9);
        let blurry_degree = u32(buf, 13);
        let color_count = u32(buf, 17);
        let (change_points, buf) = if color_count > 2 {
            let (change_points, buf) = buf.split_at(17 + 4 * color_count as usize);
            let change_points = change_points[17..]
                .chunks_exact(4)
                .map(|chunk| u32(chunk, 0))
                .collect();

            (change_points, buf)
        } else {
            (vec![], buf)
        };
        let colors = buf[..color_count as usize * 4]
            .chunks_exact(4)
            .map(Color::from_buf)
            .collect();

        Self {
            kind,
            angle,
            center_x,
            center_y,
            blurry_degree,
            change_points,
            colors,
        }
    }
}

impl FillImage {
    pub const fn from_buf(buf: &[u8]) -> Self {
        let (kind, image) = buf.split_at(1);

        Self {
            kind: match kind[0] {
                0 => ImageFillKind::Tile,
                1 => ImageFillKind::TileHorizontalTop,
                2 => ImageFillKind::TileHorizontalBottom,
                3 => ImageFillKind::TileVerticalLeft,
                4 => ImageFillKind::TileVerticalRight,
                5 => ImageFillKind::Total,
                6 => ImageFillKind::Center,
                7 => ImageFillKind::CenterTop,
                8 => ImageFillKind::CenterBottom,
                9 => ImageFillKind::CenterLeft,
                10 => ImageFillKind::LeftTop,
                11 => ImageFillKind::LeftBottom,
                12 => ImageFillKind::RightCenter,
                13 => ImageFillKind::RightTop,
                14 => ImageFillKind::RightBottom,
                15 => ImageFillKind::None,
                kind => ImageFillKind::Unknown(kind),
            },
            image: Image::from_buf(image),
        }
    }
}
