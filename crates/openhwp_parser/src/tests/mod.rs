use crate::*;
use std::path::Path;

const FILE_PATH: &str = "../../fixtures/한글문서파일형식_5.0_revision1.3.hwp";

#[test]
fn from_path() {
    if !Path::new(FILE_PATH).exists() {
        eprintln!("fixture not found, skipping snapshot");
        return;
    }
    // assert!(HwpDocument::from_path(FILE_PATH).is_ok());
    insta::assert_debug_snapshot!(HwpDocument::from_path(FILE_PATH));
}

#[test]
fn new() {
    let path = Path::new(FILE_PATH);
    if !path.exists() {
        eprintln!("fixture not found, skipping reader test");
        return;
    }
    let reader = HwpReader::from_path(path);
    assert!(reader.is_ok());

    let mut reader = reader.unwrap();
    assert!(HwpDocument::from_reader(&mut reader).is_ok());
}
