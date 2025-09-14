use crate::surface::AsDrawTarget as _;
use embedded_graphics::Drawable;
use embedded_graphics::mono_font::MonoFont;
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::{PixelColor, Point},
    text::Text,
};

use crate::primitives::Size;
use crate::surface::Surface;

use super::{Font, FontMetrics, FontRender};

impl Font for MonoFont<'_> {
    fn metrics(&self) -> impl FontMetrics {
        MonoFontMetrics {
            size: self.character_size.into(),
            baseline: self.baseline,
            advance: self.character_spacing + self.character_size.width,
        }
    }
}

impl crate::font::Sealed for MonoFont<'_> {}

impl<C: PixelColor> FontRender<C> for MonoFont<'_> {
    fn draw(&self, character: char, color: C, surface: &mut impl Surface<Color = C>) {
        let mut s = heapless::String::<1>::new();
        _ = s.push(character);
        let style = MonoTextStyleBuilder::new()
            .font(self)
            .text_color(color)
            .build();
        let mut point = Point::zero();
        point.y += self.baseline as i32;
        _ = Text::new(&s, point, style).draw(&mut surface.draw_target());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MonoFontMetrics {
    size: Size,
    baseline: u32,
    advance: u32,
}

impl FontMetrics for MonoFontMetrics {
    fn advance(&self, _: char) -> u32 {
        self.advance
    }

    fn rendered_size(&self, _: char) -> Size {
        self.size
    }

    fn default_line_height(&self) -> u32 {
        self.size.height
    }

    fn baseline(&self) -> u32 {
        self.baseline
    }
}
