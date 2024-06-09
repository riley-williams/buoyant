use crate::font::UnicodeFont;

use super::{HorizontalTextAlignment, Text};

impl<'a, F: UnicodeFont> Text<'a, F> {
    pub fn uni(text: &'a str, font: &'a F) -> Text<'a, F> {
        Text {
            text,
            font,
            alignment: HorizontalTextAlignment::default(),
        }
    }
}

// TODO: ...smarter layout and alignment for graphemes
