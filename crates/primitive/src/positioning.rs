//! 개체 위치 관련 열거형

/// 개체 수직 기준
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalRelativeTo {
    /// 종이
    #[default]
    Paper,
    /// 쪽
    Page,
    /// 문단
    Paragraph,
}

/// 개체 수평 기준
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HorizontalRelativeTo {
    /// 종이
    #[default]
    Paper,
    /// 쪽
    Page,
    /// 단
    Column,
    /// 문단
    Paragraph,
}

/// 개체 너비 기준
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WidthRelativeTo {
    /// 종이
    Paper,
    /// 쪽
    Page,
    /// 단
    Column,
    /// 문단
    Paragraph,
    /// 절대값 (기본값)
    #[default]
    Absolute,
}

/// 개체 높이 기준
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HeightRelativeTo {
    /// 종이
    Paper,
    /// 쪽
    Page,
    /// 절대값 (기본값)
    #[default]
    Absolute,
}
