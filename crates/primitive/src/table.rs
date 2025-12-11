//! 표 관련 열거형

/// 슬래시 대각선 종류 (표 셀 대각선)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SlashDiagonalType {
    /// 없음
    #[default]
    None,
    /// 중심선 하나
    Center,
    /// 중심선 + 중심선 아래의 사선
    CenterBelow,
    /// 중심선 + 중심선 위의 사선
    CenterAbove,
    /// 중심선 + 중심선 아래의 사선 + 중심선 위의 사선
    All,
}

/// 중심선 종류 (표 셀 중심선)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CenterLineType {
    /// 없음
    #[default]
    None,
    /// 세로
    Vertical,
    /// 가로
    Horizontal,
    /// 교차
    Cross,
}
