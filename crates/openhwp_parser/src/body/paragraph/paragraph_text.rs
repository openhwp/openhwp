use crate::{BodyIter, HwpCharControl, HwpTag, HwpText};

#[derive(Debug, Default)]
pub struct ParagraphText {
    pub text: HwpText,
}

impl<'hwp> BodyIter<'hwp> {
    pub fn paragraph_text(&mut self, size: usize) -> ParagraphText {
        match self.expect(HwpTag::HWPTAG_PARA_TEXT) {
            Ok(record) => ParagraphText {
                text: HwpText::from_buf(record.payload, size),
            },
            Err(_) => ParagraphText::default(),
        }
    }
}

impl ParagraphText {
    #[inline]
    pub fn to_string(&self) -> String {
        self.text.to_string()
    }

    #[inline]
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.text.chars()
    }

    #[inline]
    pub fn controls(&self) -> impl Iterator<Item = &HwpCharControl> + '_ {
        self.text.controls()
    }

    #[inline]
    pub fn control_count(&self) -> usize {
        self.text
            .controls()
            .filter(|control| matches!(control, HwpCharControl::Extend(_)))
            .count()
    }
}
