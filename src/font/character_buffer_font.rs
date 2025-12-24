use crate::{
    primitives::{Point, Size, geometry::Rectangle},
    surface::Surface,
};

use super::{Font, FontMetrics, FontRender};

/// A simple font for rendering non-Unicode characters in a text buffer
/// The width and height of all characters is 1.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharacterBufferFont;

impl Font for CharacterBufferFont {
    fn metrics(&self) -> impl FontMetrics {
        CharacterBufferFontMetrics
    }
}

impl crate::font::Sealed for CharacterBufferFont {}

impl<C> FontRender<C> for CharacterBufferFont {
    fn draw(
        &self,
        _character: char,
        _foreground: C,
        _background_color: Option<C>,
        _surface: &mut impl Surface<Color = C>,
    ) {
    }
}

struct CharacterBufferFontMetrics;
impl FontMetrics for CharacterBufferFontMetrics {
    fn rendered_size(&self, _: char) -> Option<Rectangle> {
        Some(Rectangle::new(Point::zero(), Size::new(1, 1)))
    }

    fn default_line_height(&self) -> u32 {
        1
    }

    fn advance(&self, _: char) -> u32 {
        1
    }

    fn maximum_character_size(&self) -> Size {
        Size::new(1, 1)
    }
}
