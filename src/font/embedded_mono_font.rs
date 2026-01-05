use crate::font;
use crate::primitives::geometry::Rectangle;
use crate::surface::AsDrawTarget as _;
use embedded_graphics::Drawable;
use embedded_graphics::mono_font::MonoFont;
use embedded_graphics::{mono_font::MonoTextStyleBuilder, prelude::PixelColor, text::Text};

use crate::primitives::{Point, Size};
use crate::surface::Surface;

use super::{Font, FontMetrics, FontRender};

impl Font for MonoFont<'_> {
    type Attributes = ();
    fn metrics(&self, _attributes: &Self::Attributes) -> impl FontMetrics {
        let size = self.character_size.into();
        let v_metrics = font::VMetrics {
            ascent: self.baseline as i32,
            descent: self.baseline as i32 - self.character_size.height as i32,
            line_spacing: 0,
        };
        MonoFontMetrics {
            size,
            v_metrics,
            advance: self.character_spacing + self.character_size.width,
        }
    }
}

impl font::Sealed for MonoFont<'_> {}

impl<C: PixelColor> FontRender<C> for MonoFont<'_> {
    fn draw(
        &self,
        character: char,
        mut offset: Point,
        color: C,
        _background_color: Option<C>,
        _attributes: &Self::Attributes,
        surface: &mut impl Surface<Color = C>,
    ) {
        let mut s = heapless::String::<1>::new();
        _ = s.push(character);
        let style = MonoTextStyleBuilder::new()
            .font(self)
            .text_color(color)
            .build();
        offset.y += self.baseline as i32;
        _ = Text::new(&s, offset.into(), style).draw(&mut surface.draw_target());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MonoFontMetrics {
    size: Size,
    v_metrics: font::VMetrics,
    advance: u32,
}

impl FontMetrics for MonoFontMetrics {
    fn rendered_size(&self, _: char) -> Option<Rectangle> {
        Some(Rectangle::new(Point::zero(), self.size))
    }

    fn vertical_metrics(&self) -> font::VMetrics {
        self.v_metrics
    }

    fn advance(&self, _: char) -> u32 {
        self.advance
    }
}
