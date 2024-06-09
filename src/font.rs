/// A font that renders individual characters at a time.
/// Multi-character graphemes are not supported, making
/// this primarily useful for embedded devices.
pub trait CharacterFont {
    /// The height of a character in points
    fn line_height(&self) -> u16;
    /// The width of a character in points
    fn character_width(&self, character: char) -> u16;
}

/// A font capable of rendering unicode graphemes.
/// This is the preferred font type for most applications
#[cfg(feature = "unicode")]
pub trait UnicodeFont {
    /// The height of a character in points
    fn line_height(&self) -> u16;
    /// The width of a grapheme in points
    fn character_width(&self, grapheme: &str) -> u16;
}

/// A simple font for rendering non-unicode characters in a terminal.
/// The width and height of all characters is 1.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerminalChar();

impl CharacterFont for TerminalChar {
    #[inline]
    fn line_height(&self) -> u16 {
        1
    }
    #[inline]
    fn character_width(&self, _character: char) -> u16 {
        1
    }
}

/// A font for rendering unicode characters in a terminal.
/// The width and height of all characters is 1.
#[cfg(feature = "unicode")]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerminalUnicode();

#[cfg(feature = "unicode")]
impl UnicodeFont for TerminalUnicode {
    #[inline]
    fn line_height(&self) -> u16 {
        1
    }
    #[inline]
    fn character_width(&self, _grapheme: &str) -> u16 {
        1
    }
}
