/// A font that renders individual characters at a time.
/// Multi-character graphemes are not supported, making
/// this primarily useful for embedded devices.
pub trait FontLayout {
    /// The height of a character in points
    fn line_height(&self) -> u16;

    /// The width of a character in points
    fn character_width(&self, character: char) -> u16;

    /// The distance from the top of the character to the baseline
    fn baseline(&self) -> u16 {
        self.line_height()
    }

    fn str_width(&self, text: &str) -> u16 {
        text.chars().map(|c| self.character_width(c)).sum()
    }
}

/// A simple font for rendering non-unicode characters in a text buffer
/// The width and height of all characters is 1.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharacterBufferFont;

impl FontLayout for CharacterBufferFont {
    #[inline]
    fn line_height(&self) -> u16 {
        1
    }

    #[inline]
    fn character_width(&self, _: char) -> u16 {
        1
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_fonts {
    use super::FontLayout;

    impl FontLayout for embedded_graphics::mono_font::MonoFont<'_> {
        #[inline]
        fn line_height(&self) -> u16 {
            self.character_size.height as u16
        }

        #[inline]
        fn character_width(&self, _: char) -> u16 {
            self.character_size.width as u16 + self.character_spacing as u16
        }

        #[inline]
        fn baseline(&self) -> u16 {
            self.baseline as u16
        }
    }
}
