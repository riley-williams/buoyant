use crate::pixel::Interpolate;
use crate::primitives::{Point, Size};
use crate::render::{AnimationDomain, CharacterRender, CharacterRenderTarget};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    #[must_use]
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use embedded_graphics::prelude::PixelColor;
    use embedded_graphics::primitives::{PrimitiveStyle, StyledDrawable as _};
    use embedded_graphics_core::draw_target::DrawTarget;

    use crate::pixel::Interpolate;
    use crate::primitives::{Point, Size};
    use crate::render::{AnimationDomain, EmbeddedGraphicsRender};

    use super::Rect;
    // TODO: not really ideal...reimplement later
    impl<C: PixelColor> EmbeddedGraphicsRender<C> for Rect {
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &C) {
            _ = embedded_graphics::primitives::Rectangle {
                top_left: self.origin.into(),
                size: self.size.into(),
            }
            .draw_styled(&PrimitiveStyle::with_fill(*style), render_target);
        }

        fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
            let x = i16::interpolate(source.origin.x, target.origin.x, config.factor);
            let y = i16::interpolate(source.origin.y, target.origin.y, config.factor);
            let w = u16::interpolate(source.size.width, target.size.width, config.factor);
            let h = u16::interpolate(source.size.height, target.size.height, config.factor);
            Rect::new(Point::new(x, y), Size::new(w, h))
        }
    }
}

impl<C> CharacterRender<C> for Rect {
    fn render(&self, render_target: &mut impl CharacterRenderTarget<Color = C>, style: &C) {
        // TODO: don't draw offscreen?
        for x in self.origin.x..self.origin.x + self.size.width as i16 {
            for y in self.origin.y..self.origin.y + self.size.height as i16 {
                render_target.draw_color(Point::new(x, y), style);
            }
        }
    }

    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
        let x = i16::interpolate(source.origin.x, target.origin.x, config.factor);
        let y = i16::interpolate(source.origin.y, target.origin.y, config.factor);
        let w = u16::interpolate(source.size.width, target.size.width, config.factor);
        let h = u16::interpolate(source.size.height, target.size.height, config.factor);
        Rect::new(Point::new(x, y), Size::new(w, h))
    }
}
