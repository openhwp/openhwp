//! HWP용 Spacing 타입
//!
//! HWP 파일의 테이블/셀 패딩을 위한 Spacing 타입입니다.

use crate::error::Result;
use crate::util::ByteReader;

use super::HwpUnit16;

/// HWP 4방향 Spacing (HwpUnit16 기반)
///
/// 표와 셀의 패딩에 사용됩니다.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Spacing {
    /// 왼쪽
    pub left: HwpUnit16,
    /// 오른쪽
    pub right: HwpUnit16,
    /// 위쪽
    pub top: HwpUnit16,
    /// 아래쪽
    pub bottom: HwpUnit16,
}

impl Spacing {
    /// 새 Spacing 생성
    pub const fn new(left: HwpUnit16, right: HwpUnit16, top: HwpUnit16, bottom: HwpUnit16) -> Self {
        Self { left, right, top, bottom }
    }

    /// ByteReader에서 파싱 (4 x HwpUnit16).
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self> {
        Ok(Self {
            left: reader.read_hwp_unit16()?,
            right: reader.read_hwp_unit16()?,
            top: reader.read_hwp_unit16()?,
            bottom: reader.read_hwp_unit16()?,
        })
    }

    /// primitive::Spacing<HwpUnit>으로 변환
    pub fn to_hwp_unit_spacing(self) -> primitive::Spacing<primitive::HwpUnit> {
        primitive::Spacing {
            left: self.left.to_hwp_unit(),
            right: self.right.to_hwp_unit(),
            top: self.top.to_hwp_unit(),
            bottom: self.bottom.to_hwp_unit(),
        }
    }
}

/// 표 내부 패딩 (하위 호환용 타입 별칭)
pub type TablePadding = Spacing;

/// 셀 패딩 (하위 호환용 타입 별칭)
pub type CellPadding = Spacing;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_aliases() {
        let table_padding: TablePadding = Spacing::default();
        let cell_padding: CellPadding = Spacing::default();
        assert_eq!(table_padding, cell_padding);
    }
}
