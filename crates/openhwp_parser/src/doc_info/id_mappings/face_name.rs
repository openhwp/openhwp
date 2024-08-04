use super::IdMappingCount;
use crate::{u16, DocInfoError, DocInfoTag, RecordIter};

#[derive(Debug)]
pub struct FaceName {
    pub language: FontLanguage,
    pub name: String,
    pub alternative: Option<AlternativeFaceName>,
    pub panose: Option<Panose>,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum FontLanguage {
    Hangul,
    English,
    Chinese,
    Japanese,
    Etc,
    Symbol,
    User,
}

#[derive(Debug)]
pub struct AlternativeFaceName {
    pub kind: AlternativeFaceNameKind,
    pub name: String,
}

#[derive(Debug)]
pub enum AlternativeFaceNameKind {
    /// 원래 종류를 알 수 없을 때
    Unknown,
    /// 트루타입 글꼴(TTF)
    Ttf,
    /// 한/글 전용 글꼴(HFT)
    Htf,
    Unexpected(u8),
}

/// https://en.wikipedia.org/wiki/PANOSE
#[derive(Debug)]
pub struct Panose {
    /// 글꼴 계열
    pub kind: u8,
    /// 세리프 유형
    pub serif_style: u8,
    /// 굵기
    pub weight: u8,
    /// 비례
    pub proportion: u8,
    /// 대조
    pub contrast: u8,
    /// 스트로크 편차
    pub stroke_variation: u8,
    /// 자획 유형
    pub arm_style: u8,
    /// 글자형
    pub letterform: u8,
    /// 중간선
    pub midline: u8,
    /// X-높이
    pub x_height: u8,
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn face_names(
        &mut self,
        id_mappings: &IdMappingCount,
    ) -> Result<Vec<FaceName>, DocInfoError> {
        macro_rules! face_name_count {
            ($( $tag:ident -> $language:ident .take ($count:expr) )+) => {
                let face_name_count = id_mappings.hangul_font
                    + id_mappings.english_font
                    + id_mappings.chinese_font
                    + id_mappings.japanese_font
                    + id_mappings.etc_font
                    + id_mappings.symbol_font
                    + id_mappings.user_font;
                let mut face_names = Vec::with_capacity(face_name_count as usize);

                $(
                    for record in self
                        .take($count as usize)
                        .take_while(|record| record.tag_id == DocInfoTag::HWPTAG_FACE_NAME as u16)
                    {
                        face_names.push(FaceName::from_buf(record.payload, FontLanguage::$language)?);
                    }
                )+

                Ok(face_names)
            };
        }

        face_name_count! {
            hangul_font -> Hangul.take(id_mappings.hangul_font)
            english_font -> English.take(id_mappings.english_font)
            chinese_font -> Chinese.take(id_mappings.chinese_font)
            japanese_font -> Japanese.take(id_mappings.japanese_font)
            etc_font -> Etc.take(id_mappings.etc_font)
            symbol_font -> Symbol.take(id_mappings.symbol_font)
            user_font -> User.take(id_mappings.user_font)
        }
    }
}

impl FaceName {
    pub fn from_buf(buf: &[u8], language: FontLanguage) -> Result<Self, DocInfoError> {
        let (attribute, buf) = buf.split_at(1);
        // 대체 글꼴 존재 여부
        let has_alternative = attribute[0] & 0x80 != 0;
        // 글꼴 유형 정보 존재 여부
        let has_panose = attribute[0] & 0x40 != 0;
        // 기본 글꼴 존재 여부
        let has_default = attribute[0] & 0x20 != 0;

        let (name, buf) = {
            let (name_size, buf) = buf.split_at(2);
            let name_size = 2 * u16(name_size, 0) as usize;
            let (name, buf) = buf.split_at(name_size);
            let name: Vec<_> = name
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();

            (String::from_utf16_lossy(&name).to_string(), buf)
        };
        let (alternative, buf) = if has_alternative {
            let (kind, buf) = {
                let (kind, buf) = buf.split_at(1);
                let kind = match kind[0] {
                    0 => AlternativeFaceNameKind::Unknown,
                    1 => AlternativeFaceNameKind::Ttf,
                    2 => AlternativeFaceNameKind::Htf,
                    kind => AlternativeFaceNameKind::Unexpected(kind),
                };

                (kind, buf)
            };
            let (name, buf) = {
                let (name_size, buf) = buf.split_at(2);
                let name_size = 2 * u16(name_size, 0) as usize;
                let (name, buf) = buf.split_at(name_size);
                let name: Vec<_> = name
                    .chunks_exact(2)
                    .map(|c| u16::from_le_bytes([c[0], c[1]]))
                    .collect();

                (String::from_utf16_lossy(&name).to_string(), buf)
            };
            let alternative = AlternativeFaceName { kind, name };

            (Some(alternative), buf)
        } else {
            (None, buf)
        };
        let (panose, buf) = if has_panose {
            let (panose, buf) = buf.split_at(10);
            let panose = Panose {
                kind: panose[0],
                serif_style: panose[1],
                weight: panose[2],
                proportion: panose[3],
                contrast: panose[4],
                stroke_variation: panose[5],
                arm_style: panose[6],
                letterform: panose[7],
                midline: panose[8],
                x_height: panose[9],
            };

            (Some(panose), buf)
        } else {
            (None, buf)
        };
        let default = if has_default {
            let (default_size, buf) = buf.split_at(2);
            let default_size = 2 * u16(default_size, 0) as usize;
            let (default, _) = buf.split_at(default_size);
            let default: Vec<_> = default
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            let default = String::from_utf16_lossy(&default).to_string();

            Some(default)
        } else {
            None
        };

        Ok(Self {
            language,
            name,
            alternative,
            panose,
            default,
        })
    }
}
