use super::HwpChar;

#[test]
fn char_controls() {
    let char_controls: Vec<_> = [
        10u8, 0, // LineBreak
        13, 0, // ParagraphBreak
        24, 0, // Hyphen
        25, 0, // Reserved
        26, 0, // Reserved
        27, 0, // Reserved
        28, 0, // Reserved
        29, 0, // Reserved
        30, 0, // GroupSpace
        31, 0, // FixedWidthSpace
    ]
    .chunks_exact(2)
    .map(HwpChar::from_buf)
    .collect();
    insta::assert_debug_snapshot!(char_controls);
}

#[test]
fn inline_controls() {
    let inline_controls: Vec<_> = [
        4u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, // FieldEnd
        5u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, // Reserved
        6u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, // Reserved
        7u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, // Reserved
        19u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, // Reserved
        20u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, // Reserved
        8u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, // TitleMark
        9u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, // Tab
    ]
    .chunks_exact(16)
    .map(HwpChar::from_buf)
    .collect();
    insta::assert_debug_snapshot!(inline_controls);
}

#[test]
fn extend_controls() {
    let extend_controls: Vec<_> = [
        1u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, // Reserved
        12u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, // Reserved
        14u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, // Reserved
        2u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, // PageOrColumn
        3u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, // FieldStart
        11u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, // DrawingOrTable
        15u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, // HiddenComment
        16u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, // HeaderOrFooter
        17u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, // FootnoteOrEndnote
        18u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, // AutoNumber
        21u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, // PageControl
        22u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, // BookmarkOrIndexMark
        23u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, // RubyOrOverlapping
    ]
    .chunks_exact(16)
    .map(HwpChar::from_buf)
    .collect();
    insta::assert_debug_snapshot!(extend_controls);
}
