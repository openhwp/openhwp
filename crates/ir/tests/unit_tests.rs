//! IR 크레이트 단위 테스트

use ir::*;

#[test]
fn test_hwp_unit_conversions() {
    // 1pt = 100 HwpUnit
    let unit = HwpUnit::from_pt(10.0);
    assert_eq!(unit.value(), 1000);
    assert!((unit.to_pt() - 10.0).abs() < 0.001);

    // mm 변환 테스트
    let unit_mm = HwpUnit::from_mm(25.4); // 1 inch = 25.4mm = 7200 HwpUnit
    assert_eq!(unit_mm.value(), 7200);

    // inch 변환 테스트
    let unit_inch = HwpUnit::from_inch(1.0);
    assert_eq!(unit_inch.value(), 7200);
}

#[test]
fn test_hwp_unit_constants() {
    let zero = HwpUnit::ZERO;
    assert_eq!(zero.value(), 0);

    let unit = HwpUnit::new(100);
    assert_eq!(unit.value(), 100);
}

#[test]
fn test_color_creation() {
    let black = Color::BLACK;
    assert_eq!(black.red, 0);
    assert_eq!(black.green, 0);
    assert_eq!(black.blue, 0);

    let white = Color::WHITE;
    assert_eq!(white.red, 255);
    assert_eq!(white.green, 255);
    assert_eq!(white.blue, 255);

    let custom = Color::rgb(128, 64, 32);
    assert_eq!(custom.red, 128);
    assert_eq!(custom.green, 64);
    assert_eq!(custom.blue, 32);
}

#[test]
fn test_color_hex() {
    let color = Color::from_hex("#FF8040").unwrap();
    assert_eq!(color.red, 255);
    assert_eq!(color.green, 128);
    assert_eq!(color.blue, 64);
    assert_eq!(color.to_hex_rgb(), "#FF8040");
}

#[test]
fn test_document_creation() {
    let doc = Document::new();

    assert!(doc.metadata.title.is_none());
    assert!(doc.sections.is_empty());
}

#[test]
fn test_paragraph_creation() {
    let para = paragraph::Paragraph::new();
    assert!(para.runs.is_empty());
    assert!(para.is_empty());

    let para_with_text = paragraph::Paragraph::with_text("Hello, World!");
    assert!(!para_with_text.is_empty());
    assert_eq!(para_with_text.to_plain_text(), "Hello, World!");
}

#[test]
fn test_run_creation() {
    let run = paragraph::Run::text("Test text");
    assert_eq!(run.contents.len(), 1);

    if let paragraph::RunContent::Text(text) = &run.contents[0] {
        assert_eq!(text.text, "Test text");
    } else {
        panic!("Expected Text content");
    }
}

#[test]
fn test_section_with_paragraphs() {
    let mut section = Section::default();
    section
        .paragraphs
        .push(paragraph::Paragraph::with_text("First paragraph"));
    section
        .paragraphs
        .push(paragraph::Paragraph::with_text("Second paragraph"));

    assert_eq!(section.paragraphs.len(), 2);
}

#[test]
fn test_metadata() {
    let meta = Metadata::new()
        .with_title("Test Document")
        .with_author("Test Author");

    assert_eq!(meta.title, Some("Test Document".to_string()));
    assert_eq!(meta.author, Some("Test Author".to_string()));
}

#[test]
fn test_document_version() {
    let version = DocumentVersion::new(5, 1, 0, 0);
    assert_eq!(version.major, 5);
    assert_eq!(version.minor, 1);
    assert_eq!(version.patch, 0);
    assert_eq!(version.build, 0);
    assert_eq!(version.to_string(), "5.1.0.0");
}

#[test]
fn test_style_store() {
    let store = StyleStore::new();
    assert!(store.fonts.is_empty());
    assert!(store.char_shapes.is_empty());
    assert!(store.para_shapes.is_empty());
}

#[test]
fn test_binary_data_store() {
    let mut store = BinaryDataStore::new();
    let id = BinaryDataId::new("image1.png");
    let data = binary::BinaryData::new(BinaryFormat::Png, vec![0x89, b'P', b'N', b'G']);

    store.add(id.clone(), data);

    assert!(store.get(&id).is_some());
    assert_eq!(store.get(&id).unwrap().format, BinaryFormat::Png);
}

#[test]
fn test_reference_ids() {
    let char_shape_id = CharShapeId::new(1);
    assert_eq!(char_shape_id.value(), 1);

    let para_shape_id = ParaShapeId::new(2);
    assert_eq!(para_shape_id.value(), 2);

    let style_id = StyleId::new(3);
    assert_eq!(style_id.value(), 3);
}

#[test]
fn test_conversion_error() {
    let error = ConversionError::unsupported("Test feature");
    assert!(error.message.contains("Test feature"));
}

#[test]
fn test_warning_collector() {
    let mut collector = WarningCollector::new();
    collector.data_loss("Some data was lost");
    collector.default_substituted("Value was substituted");

    let result: ConversionResult<i32> = collector.into_result(100);
    assert_eq!(result.warnings.len(), 2);
    assert!(result.has_warnings());
}

#[test]
fn test_extensions() {
    let ext = Extensions::new();
    assert!(ext.hwp.is_none());
    assert!(ext.hwpx.is_none());

    let ext_with_hwp = Extensions::new().with_hwp(extensions::HwpExtensions::default());
    assert!(ext_with_hwp.hwp.is_some());
}

#[test]
fn test_size_and_point() {
    let size = Size::new(HwpUnit(1000), HwpUnit(2000));
    assert_eq!(size.width.value(), 1000);
    assert_eq!(size.height.value(), 2000);

    let point = Point::new(HwpUnit(100), HwpUnit(200));
    assert_eq!(point.x.value(), 100);
    assert_eq!(point.y.value(), 200);
}

#[test]
fn test_insets() {
    let insets = Insets::all(HwpUnit(100));
    assert_eq!(insets.left, insets.right);
    assert_eq!(insets.top, insets.bottom);

    let custom = Insets::new(HwpUnit(10), HwpUnit(20), HwpUnit(30), HwpUnit(40));
    assert_eq!(custom.left.value(), 10);
    assert_eq!(custom.right.value(), 20);
    assert_eq!(custom.top.value(), 30);
    assert_eq!(custom.bottom.value(), 40);
}
