use crate::primitives::uint;

pub trait Font {
    /// The height of a character in points
    fn line_height(&self) -> uint;
    /// The width of a character in points
    fn character_width(&self, character: char) -> uint;
}

#[derive(Default)]
pub struct TextBufferFont();
impl Font for TextBufferFont {
    fn line_height(&self) -> uint {
        1
    }
    fn character_width(&self, _character: char) -> uint {
        1
    }
}
