//! 글꼴 속성 관련 열거형

/// 언어 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LanguageType {
    /// 한글
    #[default]
    Korean,
    /// 영어
    English,
    /// 한자
    Hanja,
    /// 일본어
    Japanese,
    /// 기타
    Other,
    /// 기호
    Symbol,
    /// 사용자 정의
    User,
}

/// 글꼴 언어
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontLanguage {
    /// 한글
    #[default]
    Hangul,
    /// 영어 (라틴)
    Latin,
    /// 한자
    Hanja,
    /// 일본어
    Japanese,
    /// 기타
    Other,
    /// 심볼
    Symbol,
    /// 사용자
    User,
}

/// 글꼴 패밀리 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontFamilyType {
    /// 알 수 없음
    #[default]
    Unknown,
    /// 명조 (serif)
    Myungjo,
    /// 고딕 (sans-serif)
    Gothic,
    /// 세리프
    SansSerif,
    /// 필기체
    BrushScript,
    /// 장식체
    Decorative,
    /// 비정형 명조
    NonRectMyungjo,
    /// 비정형 고딕
    NonRectGothic,
}
