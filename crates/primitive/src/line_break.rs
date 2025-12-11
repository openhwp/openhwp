//! 줄 나눔 관련 열거형

/// 줄 나눔 기준 (한글)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineBreakKorean {
    /// 어절 단위
    #[default]
    Word,
    /// 글자 단위
    Character,
}

/// 줄 나눔 기준 (영어)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineBreakLatin {
    /// 단어 단위
    #[default]
    Word,
    /// 하이픈 허용
    Hyphenation,
    /// 글자 단위
    Character,
}
