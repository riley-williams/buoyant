use crate::{
    primitives::{Point, Size, geometry::Rectangle},
    render_target::Surface,
};

use super::{Font, FontMetrics, FontRender};

/// A simple font for rendering non-Unicode characters in a text buffer
/// The width and height of all characters is 1.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharacterBufferFont;

impl Font for CharacterBufferFont {
    type Attributes = ();
    fn metrics(&self, _attributes: &Self::Attributes) -> impl FontMetrics {
        CharacterBufferFontMetrics
    }
}

impl crate::font::Sealed for CharacterBufferFont {}

impl<C> FontRender<C> for CharacterBufferFont {
    fn draw(
        &self,
        _character: char,
        _offset: Point,
        _foreground: C,
        _background_color: Option<C>,
        _attributes: &Self::Attributes,
        _surface: &mut impl Surface<Color = C>,
    ) {
    }
}

struct CharacterBufferFontMetrics;
impl FontMetrics for CharacterBufferFontMetrics {
    fn rendered_size(&self, _: char) -> Option<Rectangle> {
        Some(Rectangle::new(Point::zero(), Size::new(1, 1)))
    }

    fn vertical_metrics(&self) -> crate::font::VMetrics {
        crate::font::VMetrics {
            ascent: 1,
            descent: 0,
            line_spacing: 0,
        }
    }

    fn advance(&self, _: char) -> u32 {
        1
    }
}
