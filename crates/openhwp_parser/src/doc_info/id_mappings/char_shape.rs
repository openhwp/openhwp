use super::{BorderShape, Color, IdMappingCount};
use crate::{DocInfoIter, HwpTag, Version, u16, u32};

#[derive(Debug)]
pub struct CharShape {
    pub font_by_language: FontByLanguage,
    /// 기준 크기, 0pt～4096pt
    pub base_font_size: i32,
    pub shape: FontShape,
    /// 그림자 간격, -100%～100%
    pub shadow_x: i8,
    /// 그림자 간격, -100%～100%
    pub shadow_y: i8,
    /// 글자 색
    pub font_color: Color,
    /// 밑줄 색
    pub underline_color: Color,
    /// 음영 색
    pub shade_color: Color,
    /// 그림자 색
    pub shadow_color: Color,
    /// 글자 테두리/배경 ID(CharShapeBorderFill ID) 참조 값 (5.0.2.1 이상)
    pub border_fill_id: Option<u16>,
    /// 취소선 색 (5.0.3.0 이상
    pub strike_color: Option<Color>,
}

#[derive(Debug)]
pub struct FontByLanguage {
    /// 한글
    pub korean: Font,
    /// 영어
    pub english: Font,
    /// 한자
    pub chinese: Font,
    /// 일어
    pub japanese: Font,
    /// 기타
    pub etc: Font,
    /// 기호
    pub symbol: Font,
    /// 사용자
    pub user: Font,
}

#[derive(Debug)]
pub struct Font {
    /// 언어별 글꼴 ID(FaceID) 참조 값
    pub id: u16,
    /// 언어별 장평, 50%～200%
    pub scale: u8,
    /// 언어별 자간, -50%～50%
    pub spacing: i8,
    /// 언어별 상대 크기, 10%～250%
    pub size: u8,
    /// 언어별 글자 위치, -100%～100%
    pub position: i8,
}

#[derive(Debug)]
pub struct FontShape {
    /// 기울임 여부
    pub italic: bool,
    /// 진하게 여부
    pub bold: bool,
    /// 밑줄 종류
    pub underline_kind: UnderlineKind,
    /// 밑줄 모양
    pub underline_border: BorderShape,
    /// 외곽선 종류
    pub outline_kind: OutlineKind,
    /// 그림자 종류
    pub shadow_kind: ShadowKind,
    /// 양각 여부
    pub emboss: bool,
    /// 음각 여부
    pub engrave: bool,
    /// 위 첨자 여부
    pub superscript: bool,
    /// 아래 첨자 여부
    pub subscript: bool,
    /// 취소선 여부
    pub strike: bool,
    /// 강조점 종류
    pub symbol: SymbolKind,
    /// 글꼴에 어울리는 빈칸 사용 여부
    pub use_font_space: bool,
    /// 취소선 모양
    pub strike_shape: BorderShape,
    /// Kerning 여부
    pub use_kerning: bool,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum UnderlineKind {
    /// 없음
    None = 0,
    /// 글자 아래
    Under = 1,
    /// 글자 위
    Over = 3,
    Unknown(u8),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OutlineKind {
    /// 없음
    None = 0,
    /// 실선
    Solid = 1,
    /// 점선
    Dot = 2,
    /// 굵은 실선(두꺼운 선)
    Tick = 3,
    /// 파선(긴 점선)
    Dash = 4,
    /// 일점쇄선 (-.-.-.-.)
    DashDot = 5,
    /// 이점쇄선 (-..-..-..)
    DashDotDot = 6,
    Unknown(u8),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ShadowKind {
    /// 없음
    None = 0,
    /// 비연속
    Discontinuous = 1,
    /// 연속
    Continuous = 2,
    Unknown(u8),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SymbolKind {
    /// 없음
    None = 0,
    /// 검정 동그라미 강조점
    DotAbove = 1,
    /// 속 빈 동그라미 강조점
    RingAbove = 2,
    /// ˇ
    Caron = 3,
    ///  ̃
    Tilde = 4,
    /// ･
    DotMiddle = 5,
    /// :
    Colon = 6,
    Unknown(u8),
}

impl<'hwp> DocInfoIter<'hwp> {
    pub fn char_shapes(&mut self, id_mappings: &IdMappingCount) -> Vec<CharShape> {
        let mut char_shapes = Vec::with_capacity(id_mappings.char_shape as usize);

        for _ in 0..id_mappings.char_shape {
            match self.next_if(|record| record.tag == HwpTag::HWPTAG_CHAR_SHAPE) {
                Some(record) => {
                    char_shapes.push(CharShape::from_buf(record.payload, self.version()));
                }
                None => break,
            }
        }

        char_shapes
    }
}

impl CharShape {
    pub fn from_buf(buf: &[u8], version: &Version) -> Self {
        let (font_by_language, buf) = buf.split_at(42);
        let (base_font_size, buf) = buf.split_at(4);
        let (font_shape, buf) = buf.split_at(4);
        let (shadow_x, buf) = buf.split_at(1);
        let (shadow_y, buf) = buf.split_at(1);
        let (font_color, buf) = buf.split_at(4);
        let (underline_color, buf) = buf.split_at(4);
        let (shade_color, buf) = buf.split_at(4);
        let (shadow_color, buf) = buf.split_at(4);
        let (border_fill_id, buf) = if version >= &Version::V5_0_2_1 {
            let (border_fill_id, buf) = buf.split_at(2);

            (Some(border_fill_id), buf)
        } else {
            (None, buf)
        };
        let strike_color = if version >= &Version::V5_0_3_2 {
            let (strike_color, _) = buf.split_at(4);

            Some(strike_color)
        } else {
            None
        };

        Self {
            font_by_language: FontByLanguage::from_buf(font_by_language),
            base_font_size: u32(base_font_size, 0) as i32,
            shape: FontShape::from_buf(font_shape),
            shadow_x: shadow_x[0] as i8,
            shadow_y: shadow_y[0] as i8,
            font_color: Color::from_buf(font_color),
            underline_color: Color::from_buf(underline_color),
            shade_color: Color::from_buf(shade_color),
            shadow_color: Color::from_buf(shadow_color),
            border_fill_id: border_fill_id.map(|buf| u16(buf, 0)),
            strike_color: strike_color.map(Color::from_buf),
        }
    }
}

impl FontByLanguage {
    pub const fn from_buf(buf: &[u8]) -> Self {
        Self {
            korean: Font {
                id: u16(buf, 0),
                scale: buf[2],
                spacing: buf[3] as i8,
                size: buf[4],
                position: buf[5] as i8,
            },
            english: Font {
                id: u16(buf, 6),
                scale: buf[8],
                spacing: buf[9] as i8,
                size: buf[10],
                position: buf[11] as i8,
            },
            chinese: Font {
                id: u16(buf, 12),
                scale: buf[14],
                spacing: buf[15] as i8,
                size: buf[16],
                position: buf[17] as i8,
            },
            japanese: Font {
                id: u16(buf, 18),
                scale: buf[20],
                spacing: buf[21] as i8,
                size: buf[22],
                position: buf[23] as i8,
            },
            etc: Font {
                id: u16(buf, 24),
                scale: buf[26],
                spacing: buf[27] as i8,
                size: buf[28],
                position: buf[29] as i8,
            },
            symbol: Font {
                id: u16(buf, 30),
                scale: buf[32],
                spacing: buf[33] as i8,
                size: buf[34],
                position: buf[35] as i8,
            },
            user: Font {
                id: u16(buf, 36),
                scale: buf[38],
                spacing: buf[39] as i8,
                size: buf[40],
                position: buf[41] as i8,
            },
        }
    }
}

impl FontShape {
    pub const fn from_buf(buf: &[u8]) -> Self {
        Self {
            italic: buf[0] & 0b_0000_0001 != 0,
            bold: buf[0] & 0b_0000_0010 != 0,
            underline_kind: match buf[0] & 0b_0000_1100 {
                0b_0000_0000 => UnderlineKind::None,
                0b_0000_0100 => UnderlineKind::Under,
                0b_0000_1100 => UnderlineKind::Over,
                kind => UnderlineKind::Unknown(kind),
            },
            underline_border: BorderShape::from_buf(&[buf[0] >> 4]),
            outline_kind: match buf[1] & 0b_0000_0111 {
                0 => OutlineKind::None,
                1 => OutlineKind::Solid,
                2 => OutlineKind::Dot,
                3 => OutlineKind::Tick,
                4 => OutlineKind::Dash,
                5 => OutlineKind::DashDot,
                6 => OutlineKind::DashDotDot,
                kind => OutlineKind::Unknown(kind),
            },
            shadow_kind: match buf[1] & 0b_0001_1000 {
                0 => ShadowKind::None,
                1 => ShadowKind::Discontinuous,
                2 => ShadowKind::Continuous,
                kind => ShadowKind::Unknown(kind),
            },
            emboss: buf[1] & 0b_0010_0000 != 0,
            engrave: buf[1] & 0b_0100_0000 != 0,
            superscript: buf[1] & 0b_1000_0000 != 0,
            subscript: buf[2] & 0b_0000_0001 != 0,
            strike: buf[2] & 0b_0001_1000 != 0,
            symbol: match buf[2] & 0b_1110_0000 {
                0b_0000_0000 => SymbolKind::None,
                0b_0010_0000 => SymbolKind::DotAbove,
                0b_0100_0000 => SymbolKind::RingAbove,
                0b_0110_0000 => SymbolKind::Caron,
                0b_1000_0000 => SymbolKind::Tilde,
                0b_1010_0000 => SymbolKind::DotMiddle,
                0b_1100_0000 => SymbolKind::Colon,
                kind => SymbolKind::Unknown(kind),
            },
            use_font_space: buf[3] & 0b_0000_0010 != 0,
            strike_shape: BorderShape::from_buf(&[(buf[3] >> 2) & 0b_0000_11111]),
            use_kerning: buf[3] & 0b_0100_0000 != 0,
        }
    }
}
