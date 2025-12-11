//! 선 스타일 정의
//!
//! 테두리 및 밑줄에 사용되는 선의 시각적 스타일을 정의합니다.

/// 테두리 및 밑줄의 선 스타일
///
/// 실선, 점선, 파선 등 선의 시각적 패턴을 나타냅니다.
/// HWP 형식의 `BorderLineStyle`과 `UnderlineShape`를 통합한 타입입니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum LineStyle {
    /// 실선
    #[default]
    Solid = 0,
    /// 긴 파선
    LongDash = 1,
    /// 짧은 파선
    Dash = 2,
    /// 파선-점
    DashDot = 3,
    /// 파선-점-점
    DashDotDot = 4,
    /// 긴 파선 (대체)
    LongDashAlt = 5,
    /// 원형
    Circle = 6,
    /// 이중선
    Double = 7,
    /// 얇은-두꺼운
    ThinThick = 8,
    /// 두꺼운-얇은
    ThickThin = 9,
    /// 얇은-두꺼운-얇은
    ThinThickThin = 10,
    /// 물결
    Wave = 11,
    /// 이중 물결
    DoubleWave = 12,
    /// 두꺼운 3D
    Thick3D = 13,
    /// 두꺼운 3D 반전
    Thick3DReversed = 14,
    /// 단일 3D
    Single3D = 15,
    /// 단일 3D 반전
    Single3DReversed = 16,
}

impl LineStyle {
    /// raw 값에서 생성
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Solid,
            1 => Self::LongDash,
            2 => Self::Dash,
            3 => Self::DashDot,
            4 => Self::DashDotDot,
            5 => Self::LongDashAlt,
            6 => Self::Circle,
            7 => Self::Double,
            8 => Self::ThinThick,
            9 => Self::ThickThin,
            10 => Self::ThinThickThin,
            11 => Self::Wave,
            12 => Self::DoubleWave,
            13 => Self::Thick3D,
            14 => Self::Thick3DReversed,
            15 => Self::Single3D,
            16 => Self::Single3DReversed,
            _ => Self::Solid,
        }
    }

    /// raw 값으로 반환
    pub const fn as_raw(self) -> u8 {
        self as u8
    }
}

/// 테두리 선 스타일 (하위 호환용 타입 별칭)
pub type BorderLineStyle = LineStyle;

/// 밑줄 모양 (하위 호환용 타입 별칭)
pub type UnderlineShape = LineStyle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_raw() {
        assert_eq!(LineStyle::from_raw(0), LineStyle::Solid);
        assert_eq!(LineStyle::from_raw(7), LineStyle::Double);
        assert_eq!(LineStyle::from_raw(16), LineStyle::Single3DReversed);
        assert_eq!(LineStyle::from_raw(255), LineStyle::Solid); // fallback
    }

    #[test]
    fn test_as_raw() {
        assert_eq!(LineStyle::Solid.as_raw(), 0);
        assert_eq!(LineStyle::Double.as_raw(), 7);
        assert_eq!(LineStyle::Single3DReversed.as_raw(), 16);
    }

    #[test]
    fn test_type_aliases() {
        let border: BorderLineStyle = LineStyle::Dash;
        let underline: UnderlineShape = LineStyle::Wave;
        assert_eq!(border, LineStyle::Dash);
        assert_eq!(underline, LineStyle::Wave);
    }
}
