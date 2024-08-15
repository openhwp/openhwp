use super::IdMappingCount;
use crate::{u16, u32, HwpTag, RecordIter, Version};

#[derive(Debug)]
pub struct ParagraphShape {
    pub line_space_kind: LineSpacingKind,
    pub alignment: ParagraphShapeAlignment,
    pub break_word_alphabet: BreakWordAlphabet,
    pub break_word_hangeul: BreakWordHangeul,
    pub snap_to_grid: bool,
    pub condense: u8,
    pub widow_orphan: bool,
    pub keep_with_next: bool,
    pub keep_lines: bool,
    pub page_break_before: bool,
    pub vertical_align: VerticalAlign,
    pub auto_line_height: bool,
    pub heading_kind: HeadingKind,
    pub heading_level: HeadingLevel,
    pub border_connect: bool,
    pub border_ignore_margin: bool,
    pub tailing: u8,
    pub padding_left: i32,
    pub padding_right: i32,
    pub indent: i32,
    pub margin_top: i32,
    pub margin_bottom: i32,
    pub line_space: i32,
    pub tab_definition_id: u16,
    pub numbering_bullet_id: u16,
    pub border_fill_id: u16,
    pub border_offset_left: i16,
    pub border_offset_right: i16,
    pub border_offset_top: i16,
    pub border_offset_bottom: i16,
    pub attribute_5_0_1_7: Option<Attribute5_0_1_7>,
    pub attribute_5_0_2_5: Option<Attribute5_0_2_5>,
}

#[derive(Debug, Clone, Copy)]
pub enum LineSpacingKind {
    /// 글자에 따라 (%)
    Percent,
    /// 고정값
    Fixed,
    /// 여백만 지정
    BetweenLine,
    /// 최소 (5.0.2.5 버전 이상)
    AtLeast,
}

#[derive(Debug, Clone, Copy)]
pub enum ParagraphShapeAlignment {
    /// 양쪽 정렬
    Justify,
    /// 왼쪽 정렬
    Left,
    /// 오른쪽 정렬
    Right,
    /// 가운데 정렬
    Center,
    /// 배분 정렬
    Distributive,
    /// 나눔 정렬
    DistributiveSpace,
    Unknown(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum BreakWordAlphabet {
    /// 단어
    Word,
    /// 하이픈
    Hyphen,
    // 글자
    Character,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum BreakWordHangeul {
    /// 어절
    Word,
    /// 글자
    Character,
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAlign {
    /// 글꼴기준
    Baseline,
    /// 위쪽
    Top,
    /// 가운데
    Center,
    /// 아래
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub enum HeadingKind {
    /// 없음
    None,
    /// 개요
    Outline,
    /// 번호
    Number,
    /// 글머리표
    Bullet,
}

#[derive(Debug, Clone, Copy)]
pub enum HeadingLevel {
    /// 1수준
    Level1,
    /// 2수준
    Level2,
    /// 3수준
    Level3,
    /// 4수준
    Level4,
    /// 5수준
    Level5,
    /// 6수준
    Level6,
    /// 7수준
    Level7,
    Unknown,
}

/// 5.0.1.7 버전 이상
#[derive(Debug, Clone, Copy)]
pub struct Attribute5_0_1_7 {
    /// 한 줄로 입력 여부
    pub single_line: bool,
    /// 한글과 영어 간격을 자동 조절 여부
    pub auto_spacing_hangeul_alphabet: bool,
    /// 한글과 숫자 간격을 자동 조절 여부
    pub auto_spacing_hangeul_number: bool,
}

/// 5.0.2.5 버전 이상
#[derive(Debug, Clone, Copy)]
pub struct Attribute5_0_2_5 {
    /// 줄 간격 종류
    pub line_spacing_kind: LineSpacingKind,
    /// 줄 간격
    pub line_spacing: i32,
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn paragraph_shapes(
        &mut self,
        id_mappings: &IdMappingCount,
        version: &Version,
    ) -> Vec<ParagraphShape> {
        let mut paragraph_shapes = Vec::with_capacity(id_mappings.paragraph_shape as usize);

        for record in self
            .clone()
            .take(id_mappings.paragraph_shape as usize)
            .take_while(|record| record.tag == HwpTag::HWPTAG_PARA_SHAPE)
        {
            paragraph_shapes.push(ParagraphShape::from_buf(record.payload, version));
            self.next();
        }

        paragraph_shapes
    }
}

impl ParagraphShape {
    fn from_buf(buf: &[u8], version: &Version) -> Self {
        let line_space_kind = match buf[0] & 0b0000_0011 {
            0b0000_0000 => LineSpacingKind::Percent,
            0b0000_0001 => LineSpacingKind::Fixed,
            0b0000_0010 => LineSpacingKind::BetweenLine,
            0b0000_0011 => LineSpacingKind::AtLeast,
            _ => std::unreachable!(),
        };
        let alignment = match buf[0] & 0b0001_1100 {
            0b0000_0000 => ParagraphShapeAlignment::Justify,
            0b0000_0100 => ParagraphShapeAlignment::Left,
            0b0000_1000 => ParagraphShapeAlignment::Right,
            0b0000_1100 => ParagraphShapeAlignment::Center,
            0b0001_0000 => ParagraphShapeAlignment::Distributive,
            0b0001_0100 => ParagraphShapeAlignment::DistributiveSpace,
            alignment => ParagraphShapeAlignment::Unknown(alignment as u8),
        };
        let break_word_alphabet = match buf[0] & 0b0110_0000 {
            0b0000_0000 => BreakWordAlphabet::Word,
            0b0010_0000 => BreakWordAlphabet::Hyphen,
            0b0100_0000 => BreakWordAlphabet::Character,
            0b0110_0000 => BreakWordAlphabet::Unknown,
            _ => std::unreachable!(),
        };
        let break_word_hangeul = match buf[0] & 0b1000_0000 {
            0b0000_0000 => BreakWordHangeul::Word,
            0b1000_0000 => BreakWordHangeul::Character,
            _ => std::unreachable!(),
        };
        let snap_to_grid = buf[1] & 0b0000_0001 != 0;
        let condense = (buf[1] >> 1) & 0b0111_1111;
        let widow_orphan = buf[2] & 0b0000_0001 != 0;
        let keep_with_next = buf[2] & 0b0000_0010 != 0;
        let keep_lines = buf[2] & 0b0000_0100 != 0;
        let page_break_before = buf[2] & 0b0000_1000 != 0;
        let vertical_align = match (buf[2] >> 4) & 0b0000_0011 {
            0b0000_0000 => VerticalAlign::Baseline,
            0b0001_0000 => VerticalAlign::Top,
            0b0010_0000 => VerticalAlign::Center,
            0b0011_0000 => VerticalAlign::Bottom,
            _ => std::unreachable!(),
        };
        let auto_line_height = buf[2] & 0b0100_0000 != 0;
        let heading_kind = match u16(buf, 2) & 0b0000_0001_1000_0000 {
            0b0000_0000_0000_0000 => HeadingKind::None,
            0b0000_0000_1000_0000 => HeadingKind::Outline,
            0b0000_0001_0000_0000 => HeadingKind::Number,
            0b0000_0001_1000_0000 => HeadingKind::Bullet,
            _ => std::unreachable!(),
        };
        let heading_level = match buf[3] & 0b0000_0110 {
            0b0000_0000 => HeadingLevel::Level1,
            0b0000_0010 => HeadingLevel::Level2,
            0b0000_0100 => HeadingLevel::Level3,
            0b0000_0110 => HeadingLevel::Level4,
            0b0000_1000 => HeadingLevel::Level5,
            0b0000_1010 => HeadingLevel::Level6,
            0b0000_1100 => HeadingLevel::Level7,
            0b0000_1110 => HeadingLevel::Unknown,
            _ => std::unreachable!(),
        };
        let border_connect = buf[3] & 0b0000_1000 != 0;
        let border_ignore_margin = buf[3] & 0b0001_0000 != 0;
        let tailing = (buf[3] >> 5) & 0b0000_0111;
        let padding_left = u32(buf, 4) as i32;
        let padding_right = u32(buf, 8) as i32;
        let indent = u32(buf, 12) as i32;
        let margin_top = u32(buf, 16) as i32;
        let margin_bottom = u32(buf, 20) as i32;
        let line_space = u32(buf, 24) as i32;
        let tab_definition_id = u16(buf, 28);
        let numbering_bullet_id = u16(buf, 30);
        let border_fill_id = u16(buf, 32);
        let border_offset_left = u16(buf, 34) as i16;
        let border_offset_right = u16(buf, 36) as i16;
        let border_offset_top = u16(buf, 38) as i16;
        let border_offset_bottom = u16(buf, 40) as i16;
        let (attribute_5_0_1_7, buf) = if version >= &Version::V5_0_1_7 {
            let single_line = buf[42] & 0b0000_0001 != 0;
            let auto_spacing_hangeul_alphabet = buf[42] & 0b0000_0010 != 0;
            let auto_spacing_hangeul_number = buf[42] & 0b0000_0100 != 0;
            let attribute_5_0_1_7 = Attribute5_0_1_7 {
                single_line,
                auto_spacing_hangeul_alphabet,
                auto_spacing_hangeul_number,
            };
            (Some(attribute_5_0_1_7), &buf[46..])
        } else {
            (None, &buf[42..])
        };
        let attribute_5_0_2_5 = if version >= &Version::V5_0_2_5 {
            let line_spacing_kind = match buf[0] & 0b0000_0011 {
                0b0000_0000 => LineSpacingKind::Percent,
                0b0000_0001 => LineSpacingKind::Fixed,
                0b0000_0010 => LineSpacingKind::BetweenLine,
                0b0000_0011 => LineSpacingKind::AtLeast,
                _ => std::unreachable!(),
            };
            let line_spacing = u32(buf, 4) as i32;
            Some(Attribute5_0_2_5 {
                line_spacing_kind,
                line_spacing,
            })
        } else {
            None
        };

        Self {
            line_space_kind,
            alignment,
            break_word_alphabet,
            break_word_hangeul,
            snap_to_grid,
            condense,
            widow_orphan,
            keep_with_next,
            keep_lines,
            page_break_before,
            vertical_align,
            auto_line_height,
            heading_kind,
            heading_level,
            border_connect,
            border_ignore_margin,
            tailing,
            padding_left,
            padding_right,
            indent,
            margin_top,
            margin_bottom,
            line_space,
            tab_definition_id,
            numbering_bullet_id,
            border_fill_id,
            border_offset_left,
            border_offset_right,
            border_offset_top,
            border_offset_bottom,
            attribute_5_0_1_7,
            attribute_5_0_2_5,
        }
    }
}
