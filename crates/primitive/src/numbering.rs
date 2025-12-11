//! 번호 매기기 관련 열거형

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 번호 형식
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NumberFormat {
    /// 아라비아 숫자 (1, 2, 3)
    #[default]
    Digit,
    /// 원 문자 (①, ②, ③)
    CircledDigit,
    /// 로마 숫자 대문자 (I, II, III)
    RomanUpper,
    /// 로마 숫자 소문자 (i, ii, iii)
    RomanLower,
    /// 영문 대문자 (A, B, C)
    LatinUpper,
    /// 영문 소문자 (a, b, c)
    LatinLower,
    /// 원 영문 대문자 (Ⓐ, Ⓑ, Ⓒ)
    CircledLatinUpper,
    /// 원 영문 소문자 (ⓐ, ⓑ, ⓒ)
    CircledLatinLower,
    /// 한글 (가, 나, 다)
    HangulSyllable,
    /// 원 한글 (㉮, ㉯, ㉰)
    CircledHangul,
    /// 한글 자모 (ㄱ, ㄴ, ㄷ)
    HangulJamo,
    /// 원 한글 자모
    CircledHangulJamo,
    /// 한글 일이삼 (일, 이, 삼)
    HangulIdeograph,
    /// 한자 (一, 二, 三)
    Ideograph,
    /// 원 한자
    CircledIdeograph,
    /// 간지 (甲, 乙, 丙)
    Ganji,
}

/// 각주/미주 번호 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteNumberPosition {
    /// 위 첨자
    #[default]
    Superscript,
    /// 아래 첨자
    Subscript,
}

/// 각주/미주 번호 매김 방식
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteNumbering {
    /// 문서 전체 연속
    #[default]
    Continuous,
    /// 섹션마다 새로 시작
    RestartSection,
    /// 페이지마다 새로 시작
    RestartPage,
}

/// 각주 배치 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FootnotePlacement {
    /// 각 단마다 따로 배열
    #[default]
    EachColumn,
    /// 통단으로 배열
    MergedColumn,
    /// 가장 오른쪽 단에 배열
    RightMostColumn,
}

/// 미주 배치 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EndnotePlacement {
    /// 문서의 마지막
    #[default]
    EndOfDocument,
    /// 구역의 마지막
    EndOfSection,
}
