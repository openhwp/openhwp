//! 채우기 관련 열거형

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 채우기 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FillType {
    /// 없음
    #[default]
    None,
    /// 단색
    Solid,
    /// 그라데이션
    Gradient,
    /// 이미지
    Image,
    /// 패턴
    Pattern,
}

/// 그라데이션 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GradientType {
    /// 선형
    #[default]
    Linear,
    /// 원형
    Radial,
    /// 사각형
    Square,
    /// 원뿔형
    Conical,
}

/// 패턴 종류 (해칭)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PatternType {
    /// 가로줄
    #[default]
    Horizontal,
    /// 세로줄
    Vertical,
    /// 역슬래시
    BackSlash,
    /// 슬래시
    Slash,
    /// 십자
    Cross,
    /// 대각선 십자
    CrossDiagonal,
}

/// 이미지 채우기 모드
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageFillMode {
    /// 바둑판식 배열 (전체)
    #[default]
    Tile,
    /// 바둑판식 배열 (가로/위)
    TileHorizontalTop,
    /// 바둑판식 배열 (가로/아래)
    TileHorizontalBottom,
    /// 바둑판식 배열 (세로/왼쪽)
    TileVerticalLeft,
    /// 바둑판식 배열 (세로/오른쪽)
    TileVerticalRight,
    /// 늘이기
    Stretch,
    /// 가운데
    Center,
    /// 가운데 위
    CenterTop,
    /// 가운데 아래
    CenterBottom,
    /// 가운데 왼쪽
    CenterLeft,
    /// 왼쪽 위
    TopLeft,
    /// 왼쪽 아래
    BottomLeft,
    /// 가운데 오른쪽
    CenterRight,
    /// 오른쪽 위
    TopRight,
    /// 오른쪽 아래
    BottomRight,
    /// 원본 크기
    Original,
}

/// 채우기 영역 종류 (페이지 테두리)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FillAreaType {
    /// 종이
    #[default]
    Paper,
    /// 쪽
    Page,
    /// 테두리
    Border,
}
