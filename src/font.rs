pub trait CharacterFont {
    /// The height of a character in points
    fn line_height(&self) -> u16;
    /// The width of a character in points
    fn character_width(&self, character: char) -> u16;
}

#[cfg(feature = "unicode")]
pub trait UnicodeFont {
    /// The height of a character in points
    fn line_height(&self) -> u16;
    /// The width of a grapheme in points
    fn character_width(&self, grapheme: &str) -> u16;
}

#[derive(Default)]
pub struct CharMonospace();

impl CharacterFont for CharMonospace {
    fn line_height(&self) -> u16 {
        1
    }
    fn character_width(&self, _character: char) -> u16 {
        1
    }
}

#[cfg(feature = "unicode")]
#[derive(Default)]
pub struct UnicodeMonospace();

#[cfg(feature = "unicode")]
impl UnicodeFont for UnicodeMonospace {
    fn line_height(&self) -> u16 {
        1
    }
    fn character_width(&self, _grapheme: &str) -> u16 {
        1
    }
}
