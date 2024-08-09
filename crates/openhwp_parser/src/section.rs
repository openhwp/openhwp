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
    pub fn from_vec(_buf: Vec<u8>) -> Result<Self, SectionError> {
        std::todo!();
    }
}
