#[derive(Debug)]
pub struct Section {
    //
}

#[derive(Debug, Error)]
pub enum SectionError {
    #[error("Invalid section")]
    InvalidSection,
}

impl Section {
    pub fn from_bytes(_bytes: &[u8]) -> Result<Self, SectionError> {
        std::todo!();
    }
}
