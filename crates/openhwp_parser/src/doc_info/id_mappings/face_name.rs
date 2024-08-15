use super::IdMappingCount;
use crate::{to_string, u16, HwpTag, RecordIter};

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
    Korean,
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

/// <https://en.wikipedia.org/wiki/PANOSE>
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
    pub fn face_names(&mut self, id_mappings: &IdMappingCount) -> Vec<FaceName> {
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
                        .clone()
                        .take($count as usize)
                        .take_while(|record| record.tag == HwpTag::HWPTAG_FACE_NAME)
                    {
                        face_names.push(FaceName::from_buf(record.payload, FontLanguage::$language));
                        self.next();
                    }
                )+

                face_names
            };
        }

        face_name_count! {
            hangul_font -> Korean.take(id_mappings.hangul_font)
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
    pub fn from_buf(buf: &[u8], language: FontLanguage) -> Self {
        let (attribute, buf) = buf.split_at(1);
        // 대체 글꼴 존재 여부
        let has_alternative = attribute[0] & 0x80 != 0;
        // 글꼴 유형 정보 존재 여부
        let has_panose = attribute[0] & 0x40 != 0;
        // 기본 글꼴 존재 여부
        let has_default = attribute[0] & 0x20 != 0;

        let (name, buf) = {
            let (size, buf) = buf.split_at(2);
            let size = 2 * u16(size, 0) as usize;
            let (name, buf) = buf.split_at(size);
            let name = to_string(name);

            (name, buf)
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
                let (size, buf) = buf.split_at(2);
                let size = 2 * u16(size, 0) as usize;
                let (name, buf) = buf.split_at(size);
                let name = to_string(name);

                (name, buf)
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
            let (size, buf) = buf.split_at(2);
            let size = 2 * u16(size, 0) as usize;
            let (default, _) = buf.split_at(size);
            let default = to_string(default);

            Some(default)
        } else {
            None
        };

        Self {
            language,
            name,
            alternative,
            panose,
            default,
        }
    }
}
