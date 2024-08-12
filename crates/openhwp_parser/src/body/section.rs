use crate::Record;

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
    pub fn from_vec(buf: Vec<u8>) -> Result<Self, SectionError> {
        let mut stream = Record::iter(&buf);

        Ok(Self {})
    }
}
