#[cfg(test)]
mod tests;

use crate::u16;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct HwpText {
    pub chars: Vec<HwpChar>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum HwpChar {
    Utf16(char),
    Control(Control),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Control {
    Char(CharControl),
    Inline(InlineControl),
    Extend(ExtendControl),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum CharControl {
    /// 10: 한 줄 끝(line break)
    LineBreak,
    /// 13: 문단 끝(para break)
    ParagraphBreak,
    /// 24: 하이픈
    Hyphen,
    /// 25, 26, 27, 28, 29: 예약
    Reserved,
    /// 30: 묶음 빈칸
    GroupSpace,
    /// 31: 고정폭 빈칸
    FixedWidthSpace,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct InlineControl {
    pub kind: InlineControlKind,
    pub info: [u8; 12],
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum InlineControlKind {
    /// 4: 필드 끝
    FieldEnd,
    /// 5, 6, 7, 19, 20: 예약
    Reserved,
    /// 8: title mark
    TitleMark,
    /// 9: 탭
    Tab,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct ExtendControl {
    pub kind: ExtendControlKind,
    pub pointer: [u8; 12],
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum ExtendControlKind {
    /// 1, 12, 14: 예약
    Reserved,
    /// 2: 구역 정의/단 정의
    PageOrColumn,
    /// 3: 필드 시작(누름틀, 하이퍼링크, 블록 책갈피, 표 계산식, 문서 요약, 사용자 정보, 현재 날짜/시간, 문서 날짜/시간, 파일 경로, 상호 참조, 메일 머지, 메모, 교정부호, 개인정보)
    FieldStart,
    /// 11: 그리기 개체/표
    DrawingOrTable,
    /// 15: 숨은 설명
    HiddenComment,
    /// 16: 머리말/꼬리말
    HeaderOrFooter,
    /// 17: 각주/미주
    FootnoteOrEndnote,
    /// 18: 자동번호(각주, 표 등)
    AutoNumber,
    /// 21: 페이지 컨트롤(감추기, 새 번호로 시작 등)
    PageControl,
    /// 22: 책갈피/찾아보기 표식
    BookmarkOrIndexMark,
    /// 23: 덧말/글자 겹침
    RubyOrOverlapping,
}

impl HwpText {
    pub fn from_buf(mut buf: &[u8], size: usize) -> Self {
        let mut chars = vec![];
        let mut current = 0;
        while current < size {
            let char = HwpChar::from_buf(buf);
            (_, buf) = buf.split_at(char.len() * 2);
            current += char.len();
            chars.push(char);
        }

        Self { chars }
    }

    #[inline]
    pub fn to_string(&self) -> String {
        self.chars().collect()
    }

    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars.iter().filter_map(|char| {
            if let HwpChar::Utf16(char) = char {
                Some(*char)
            } else {
                None
            }
        })
    }

    pub fn controls(&self) -> impl Iterator<Item = &Control> + '_ {
        self.chars.iter().filter_map(|char| {
            if let HwpChar::Control(control) = char {
                Some(control)
            } else {
                None
            }
        })
    }
}

impl Default for HwpText {
    fn default() -> Self {
        let chars = vec![HwpChar::Control(Control::Char(CharControl::ParagraphBreak))];

        Self { chars }
    }
}

impl HwpChar {
    pub const fn from_buf(buf: &[u8]) -> Self {
        macro_rules! char {
            ($variant:ident) => {
                Self::Control(Control::Char(CharControl::$variant))
            };
        }

        macro_rules! inline {
            ($variant:ident) => {{
                let info = [
                    buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10],
                    buf[11], buf[12], buf[13],
                ];

                Self::Control(Control::Inline(InlineControl {
                    kind: InlineControlKind::$variant,
                    info,
                }))
            }};
        }

        macro_rules! extend {
            ($variant:ident) => {{
                let pointer = [
                    buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10],
                    buf[11], buf[12], buf[13],
                ];

                Self::Control(Control::Extend(ExtendControl {
                    kind: ExtendControlKind::$variant,
                    pointer,
                }))
            }};
        }

        match u16(buf, 0) {
            // CharControl
            10 => char!(LineBreak),
            13 => char!(ParagraphBreak),
            24 => char!(Hyphen),
            25 | 26 | 27 | 28 | 29 => char!(Reserved),
            30 => char!(GroupSpace),
            31 => char!(FixedWidthSpace),

            // InlineControl
            4 => inline!(FieldEnd),
            5 | 6 | 7 | 19 | 20 => inline!(Reserved),
            8 => inline!(TitleMark),
            9 => inline!(Tab),

            // ExtendControl
            1 | 12 | 14 => extend!(Reserved),
            2 => extend!(PageOrColumn),
            3 => extend!(FieldStart),
            11 => extend!(DrawingOrTable),
            15 => extend!(HiddenComment),
            16 => extend!(HeaderOrFooter),
            17 => extend!(FootnoteOrEndnote),
            18 => extend!(AutoNumber),
            21 => extend!(PageControl),
            22 => extend!(BookmarkOrIndexMark),
            23 => extend!(RubyOrOverlapping),

            // UTF-16
            char => match char::from_u32(char as u32) {
                Some(char) => Self::Utf16(char),
                None => std::unreachable!(),
            },
        }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        match self {
            Self::Utf16(_) => 1,
            Self::Control(Control::Char(_)) => 1,
            Self::Control(Control::Inline(_)) => 8,
            Self::Control(Control::Extend(_)) => 8,
        }
    }
}
