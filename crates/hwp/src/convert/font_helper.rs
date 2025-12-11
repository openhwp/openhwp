//! Font extraction helper functions.
//!
//! Provides utility functions for extracting font-related data from IR FontSet.

use ir::char_shape::{FontRef, FontSet};

/// Extracts a field from each language's font reference into an array.
///
/// This helper function reduces code duplication when extracting font data
/// for all 7 language types (Korean, English, Hanja, Japanese, Other, Symbol, User).
///
/// # Arguments
///
/// * `fonts` - The FontSet containing optional font references for each language
/// * `extractor` - A closure that extracts the desired value from a FontRef
/// * `default` - The default value to use when no font reference exists
///
/// # Returns
///
/// An array of 7 values, one for each language type in order.
///
/// # Example
///
/// ```ignore
/// let font_ids = extract_font_field(&shape.fonts, |f| f.id.value() as u16, 0);
/// ```
pub fn extract_font_field<T, F>(fonts: &FontSet, extractor: F, default: T) -> [T; 7]
where
    F: Fn(&FontRef) -> T,
    T: Copy,
{
    [
        fonts.korean.as_ref().map(&extractor).unwrap_or(default),
        fonts.english.as_ref().map(&extractor).unwrap_or(default),
        fonts.hanja.as_ref().map(&extractor).unwrap_or(default),
        fonts.japanese.as_ref().map(&extractor).unwrap_or(default),
        fonts.other.as_ref().map(&extractor).unwrap_or(default),
        fonts.symbol.as_ref().map(&extractor).unwrap_or(default),
        fonts.user.as_ref().map(&extractor).unwrap_or(default),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::char_shape::FontSet;

    #[test]
    fn test_extract_font_field_with_no_fonts() {
        let fonts = FontSet::default();

        let ids = extract_font_field(&fonts, |f| f.id.value() as u16, 0);
        assert_eq!(ids, [0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_extract_font_field_with_default() {
        let fonts = FontSet::default();

        // All fonts are None, so all values should be the default
        let ratios = extract_font_field(&fonts, |f| f.width_ratio.0 as u8, 100);
        assert_eq!(ratios, [100, 100, 100, 100, 100, 100, 100]);
    }
}
