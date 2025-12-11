//! HWPX ↔ IR 변환 모듈
//!
//! HWPX 문서와 IR 간의 양방향 변환을 제공합니다.

mod from_ir;
mod to_ir;

pub use from_ir::IrToHwpx;
pub use to_ir::HwpxToIr;

use ir::WarningCollector;

/// HWPX → IR 변환 컨텍스트
pub struct ToIrContext {
    /// 경고 수집기
    pub warnings: WarningCollector,
}

impl ToIrContext {
    /// 새 컨텍스트 생성
    pub fn new() -> Self {
        Self {
            warnings: WarningCollector::new(),
        }
    }
}

impl Default for ToIrContext {
    fn default() -> Self {
        Self::new()
    }
}

/// IR → HWPX 변환 컨텍스트
pub struct FromIrContext {
    /// 경고 수집기
    pub warnings: WarningCollector,
}

impl FromIrContext {
    /// 새 컨텍스트 생성
    pub fn new() -> Self {
        Self {
            warnings: WarningCollector::new(),
        }
    }
}

impl Default for FromIrContext {
    fn default() -> Self {
        Self::new()
    }
}
