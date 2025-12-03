use std::num::{NonZeroU8, NonZeroU32};

/// "xs:boolean"
pub type Boolean = bool;

/// "xs:nonNegativeInteger"
pub type NonNegativeInteger32 = u32;

/// 0 <= "xs:nonNegativeInteger" <= 255
pub type NonNegativeInteger8 = u8;

/// "xs:positiveInteger"
pub type PositiveInteger32 = NonZeroU32;

/// 0 < "xs:positiveInteger" <= 255
pub type PositiveInteger8 = NonZeroU8;

/// "xs:integer"
pub type Integer32 = i32;

/// 0 <= "xs:integer" <= 65535
pub type Integer16 = u16;

/// 0 <= "xs:integer" <= 255
pub type Integer8 = u8;

/// "xs:unsignedByte"
pub type UnsignedByte = u8;

/// "xs:float"
pub type Float32 = f32;

/// "xs:unsignedInt"
pub type UnsignedInt32 = u32;

/// "xs:unsignedShort"
pub type UnsignedShort = u16;

/// "xs:dateTime"
// TODO: Change to proper DateTime later
pub type DateTime = std::string::String;

/// "xs:byte"
pub type Byte = i8;

/// "xs:base64Binary"
// TODO: Change to Vec<u8> later
pub type Base64Binary = std::string::String;

/// "xs:string"
pub type String = std::string::String;
