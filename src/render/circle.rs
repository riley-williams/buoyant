use crate::primitives::Point;

/// A circle with the origin at the top-left corner
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circle {
    pub origin: Point,
    pub diameter: u16,
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use crate::pixel::Interpolate;
    use crate::primitives::Point;
    use crate::render::{AnimationDomain, EmbeddedGraphicsRender};

    use super::Circle;
    use embedded_graphics::{
        prelude::PixelColor,
        primitives::{PrimitiveStyle, StyledDrawable as _},
    };
    use embedded_graphics_core::draw_target::DrawTarget;

    impl<C: PixelColor> EmbeddedGraphicsRender<C> for Circle {
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &C) {
            _ = embedded_graphics::primitives::Circle::new(
                self.origin.into(),
                self.diameter.into(),
            )
            .draw_styled(&PrimitiveStyle::with_fill(*style), render_target);
        }

        fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
            let x = i16::interpolate(source.origin.x, target.origin.x, config.factor);
            let y = i16::interpolate(source.origin.y, target.origin.y, config.factor);
            let diameter = u16::interpolate(source.diameter, target.diameter, config.factor);
            Circle {
                origin: Point::new(x, y),
                diameter,
            }
        }
    }
}
