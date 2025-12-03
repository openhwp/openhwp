#[derive(Debug, Error)]
pub enum Error {
    #[error("XML parsing error: {0}")]
    Xml(#[from] quick_xml::Error),
    #[error("Attribute decoding error: {0}")]
    Attribute(#[from] quick_xml::events::attributes::AttrError),
    #[error("Name decoding error: {0}")]
    Name(#[from] quick_xml::encoding::EncodingError),
    #[error("Invalid UTF-8 sequence: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Unexpected element: expected {expected:?}, found {found:?}")]
    UnexpectedElement {
        expected: crate::any_element::ElementName,
        found: crate::any_element::ElementName,
    },
    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Parse float error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("Missing required element: {0}")]
    MissingElement(String),
    #[error("Missing required attribute: {0}")]
    MissingAttribute(String),
    #[error("Invalid value for {on}: {variant}")]
    InvalidVariant { on: &'static str, variant: String },
    #[error("{0}")]
    Unknown(String),
}

#[macro_export]
macro_rules! unknown {
    ($($tt:tt)*) => {
        return Err(crate::error::Error::Unknown(format!($($tt)*).into()))
    };
}

#[macro_export]
macro_rules! missing_element {
    ($($tt:tt)*) => {
        return Err(crate::error::Error::MissingElement(format!($($tt)*).into()))
    };
}

#[macro_export]
macro_rules! missing_attribute {
    ($($tt:tt)*) => {
        return Err(crate::error::Error::MissingAttribute(format!($($tt)*).into()))
    };
}

#[macro_export]
macro_rules! invalid_variant {
    ($on:expr, $variant:expr) => {
        return Err(crate::error::Error::InvalidVariant {
            on: $on,
            variant: $variant.into(),
        })
    };
}
