pub trait Font {
    /// The height of a character in points
    fn line_height(&self) -> u16;
    /// The width of a character in points
    fn character_width(&self, character: char) -> u16;
}

#[derive(Default)]
pub struct TextBufferFont();
impl Font for TextBufferFont {
    fn line_height(&self) -> u16 {
        1
    }
    fn character_width(&self, _character: char) -> u16 {
        1
    }
}
