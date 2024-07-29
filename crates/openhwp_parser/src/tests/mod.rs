use crate::*;

const FILE_PATH: &str = "../../fixtures/한글문서파일형식_5.0_revision1.3.hwp";

#[test]
fn test_open() -> Result<(), Box<dyn std::error::Error>> {
    let _ = HwpDocument::from_path(FILE_PATH)?;

    Ok(())
}
