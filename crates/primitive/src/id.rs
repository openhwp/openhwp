//! ID 참조 타입
//!
//! 문서 내 요소 간 참조를 위한 타입 안전한 ID 시스템입니다.
//! HWP와 HWPX에서 공통으로 사용됩니다.

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 숫자 ID 타입 정의 매크로
macro_rules! define_numeric_id {
    (
        $(#[$meta:meta])*
        $name:ident, $display:literal
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        pub struct $name(pub u32);

        impl $name {
            /// ID 생성
            pub const fn new(id: u32) -> Self {
                Self(id)
            }

            /// 내부 값 반환
            pub const fn value(self) -> u32 {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!($display, "({})"), self.0)
            }
        }

        impl From<u32> for $name {
            fn from(id: u32) -> Self {
                Self(id)
            }
        }

        impl From<$name> for u32 {
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}

// ============================================================================
// 문서 스타일 관련 ID
// ============================================================================

define_numeric_id!(
    /// 글자 모양 ID
    CharShapeId, "CharShape"
);

define_numeric_id!(
    /// 문단 모양 ID
    ParaShapeId, "ParaShape"
);

define_numeric_id!(
    /// 스타일 ID
    StyleId, "Style"
);

define_numeric_id!(
    /// 테두리/채우기 ID
    BorderFillId, "BorderFill"
);

define_numeric_id!(
    /// 폰트 ID
    FontId, "Font"
);

define_numeric_id!(
    /// 탭 정의 ID
    TabDefId, "TabDef"
);

define_numeric_id!(
    /// 번호 정의 ID
    NumberingId, "Numbering"
);

// ============================================================================
// 도형/객체 관련 ID
// ============================================================================

define_numeric_id!(
    /// 외곽선 모양 ID
    OutlineShapeId, "OutlineShape"
);

define_numeric_id!(
    /// 메모 모양 ID
    MemoShapeId, "MemoShape"
);

define_numeric_id!(
    /// 연결 목록 ID
    LinkListId, "LinkList"
);

define_numeric_id!(
    /// 시작 ID
    BeginId, "Begin"
);

define_numeric_id!(
    /// 주체 ID (그룹 도형 등)
    SubjectId, "Subject"
);

define_numeric_id!(
    /// 속성 ID
    PropertyId, "Property"
);

define_numeric_id!(
    /// 테두리 타입 ID (양식 컨트롤 등)
    BorderTypeId, "BorderType"
);

// ============================================================================
// 문자열 기반 ID
// ============================================================================

/// 바이너리 데이터 ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BinaryDataId(pub String);

impl BinaryDataId {
    /// ID 생성
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// 내부 값 반환
    pub fn value(&self) -> &str {
        &self.0
    }

    /// 숫자 ID에서 생성 (HWP용)
    pub fn from_numeric(id: u16) -> Self {
        Self(format!("BIN{:04X}", id))
    }
}

impl fmt::Display for BinaryDataId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Binary({})", self.0)
    }
}

impl From<String> for BinaryDataId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for BinaryDataId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

/// 파일 ID (문자열)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct FileId(pub String);

impl FileId {
    /// ID 생성
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// 내부 값 반환
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File({})", self.0)
    }
}

impl From<String> for FileId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for FileId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

/// 이미지 ID (문자열)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ImageId(pub String);

impl ImageId {
    /// ID 생성
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// 내부 값 반환
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ImageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Image({})", self.0)
    }
}

impl From<String> for ImageId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for ImageId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

/// 마스터 페이지 ID (문자열)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct MasterPageId(pub String);

impl MasterPageId {
    /// ID 생성
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// 내부 값 반환
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MasterPageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MasterPage({})", self.0)
    }
}

impl From<String> for MasterPageId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for MasterPageId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_id() {
        let id = CharShapeId::new(42);
        assert_eq!(id.value(), 42);
        assert_eq!(format!("{}", id), "CharShape(42)");

        let id2: CharShapeId = 100.into();
        assert_eq!(id2.value(), 100);
    }

    #[test]
    fn test_string_id() {
        let id = BinaryDataId::new("test_binary");
        assert_eq!(id.value(), "test_binary");

        let id2 = BinaryDataId::from_numeric(0x1234);
        assert_eq!(id2.value(), "BIN1234");
    }
}
