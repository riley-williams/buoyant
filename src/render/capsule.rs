use crate::primitives::{Point, Size};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capsule {
    origin: Point,
    size: Size,
}

impl Capsule {
    #[must_use]
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use crate::{
        pixel::Interpolate,
        primitives::{Point, Size},
    };
    use embedded_graphics::{
        prelude::PixelColor,
        primitives::{PrimitiveStyle, StyledDrawable as _},
    };
    use embedded_graphics_core::draw_target::DrawTarget;

    use crate::render::{AnimationDomain, EmbeddedGraphicsRender};

    use super::Capsule;
    impl<C: PixelColor> EmbeddedGraphicsRender<C> for Capsule {
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &C) {
            let radius = self.size.height.min(self.size.width) / 2;
            let rectangle = embedded_graphics::primitives::Rectangle {
                top_left: self.origin.into(),
                size: self.size.into(),
            };

            _ = embedded_graphics::primitives::RoundedRectangle::with_equal_corners(
                rectangle,
                embedded_graphics::prelude::Size {
                    width: radius.into(),
                    height: radius.into(),
                },
            )
            .draw_styled(&PrimitiveStyle::with_fill(*style), render_target);
        }

        fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
            let x = i16::interpolate(source.origin.x, target.origin.x, config.factor);
            let y = i16::interpolate(source.origin.y, target.origin.y, config.factor);
            let w = u16::interpolate(source.size.width, target.size.width, config.factor);
            let h = u16::interpolate(source.size.height, target.size.height, config.factor);
            Capsule::new(Point::new(x, y), Size::new(w, h))
        }
    }
}
