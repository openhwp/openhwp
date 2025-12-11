//! 색상 타입
//!
//! 문서에서 사용되는 색상을 표현합니다.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// ARGB 색상
///
/// 알파, 빨강, 초록, 파랑 각 8비트로 구성됩니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color {
    /// 알파 (0 = 투명, 255 = 불투명)
    pub alpha: u8,
    /// 빨강
    pub red: u8,
    /// 초록
    pub green: u8,
    /// 파랑
    pub blue: u8,
}

impl Color {
    /// RGB 색상 생성 (알파 = 255)
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            alpha: 255,
            red,
            green,
            blue,
        }
    }

    /// ARGB 색상 생성
    pub const fn argb(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        Self {
            alpha,
            red,
            green,
            blue,
        }
    }

    /// u32에서 생성 (0xAARRGGBB 형식)
    pub const fn from_argb_u32(value: u32) -> Self {
        Self {
            alpha: ((value >> 24) & 0xFF) as u8,
            red: ((value >> 16) & 0xFF) as u8,
            green: ((value >> 8) & 0xFF) as u8,
            blue: (value & 0xFF) as u8,
        }
    }

    /// u32에서 생성 (0x00RRGGBB 형식, 알파 = 255)
    pub const fn from_rgb_u32(value: u32) -> Self {
        Self {
            alpha: 255,
            red: ((value >> 16) & 0xFF) as u8,
            green: ((value >> 8) & 0xFF) as u8,
            blue: (value & 0xFF) as u8,
        }
    }

    /// u32로 변환 (0xAARRGGBB 형식)
    pub const fn to_argb_u32(self) -> u32 {
        ((self.alpha as u32) << 24)
            | ((self.red as u32) << 16)
            | ((self.green as u32) << 8)
            | (self.blue as u32)
    }

    /// u32로 변환 (0x00RRGGBB 형식)
    pub const fn to_rgb_u32(self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }

    /// hex 문자열에서 파싱 (#RRGGBB 또는 #AARRGGBB)
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);

        match hex.len() {
            6 => {
                let value = u32::from_str_radix(hex, 16).ok()?;
                Some(Self::from_rgb_u32(value))
            }
            8 => {
                let value = u32::from_str_radix(hex, 16).ok()?;
                Some(Self::from_argb_u32(value))
            }
            _ => None,
        }
    }

    /// hex 문자열로 변환 (#RRGGBB)
    pub fn to_hex_rgb(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    /// hex 문자열로 변환 (#AARRGGBB)
    pub fn to_hex_argb(self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            self.alpha, self.red, self.green, self.blue
        )
    }

    // 기본 색상 상수
    /// 검정
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    /// 흰색
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    /// 빨강
    pub const RED: Self = Self::rgb(255, 0, 0);
    /// 초록
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    /// 파랑
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    /// 투명
    pub const TRANSPARENT: Self = Self::argb(0, 0, 0, 0);
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self::from_argb_u32(value)
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        color.to_argb_u32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_rgb() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
        assert_eq!(color.alpha, 255);
    }

    #[test]
    fn test_color_from_u32() {
        let color = Color::from_argb_u32(0x80FF8040);
        assert_eq!(color.alpha, 128);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
    }

    #[test]
    fn test_color_hex() {
        let color = Color::from_hex("#FF8040").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);

        assert_eq!(color.to_hex_rgb(), "#FF8040");
    }

    #[test]
    fn test_color_hex_with_alpha() {
        let color = Color::from_hex("#80FF8040").unwrap();
        assert_eq!(color.alpha, 128);
        assert_eq!(color.to_hex_argb(), "#80FF8040");
    }
}
