//! 형식별 확장 기능
//!
//! HWP/HWPX 형식에 특화된 확장 기능을 제공합니다.

pub mod hwp;
pub mod hwpx;
pub mod track_change;

pub use hwp::HwpExtension;
pub use hwpx::HwpxExtension;
pub use track_change::{TrackChange, TrackChangeType};
