use crate::*;
use std::path::Path;

const FILE_PATH: &str = "../../fixtures/한글문서파일형식_5.0_revision1.3.hwp";

#[test]
fn from_path() {
    assert!(HwpDocument::from_path(FILE_PATH).is_ok());
}

#[test]
fn new() {
    let path = Path::new(FILE_PATH);
    let reader = HwpReader::from_path(path);
    assert!(reader.is_ok());

    let reader = reader.unwrap();
    assert!(HwpDocument::from_reader(reader).is_ok());
}
