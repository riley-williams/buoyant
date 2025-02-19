use crate::primitives::{Point, Size};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capsule {
    pub origin: Point,
    pub size: Size,
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
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &C, offset: Point) {
            let top_left = (self.origin + offset).into();
            let radius = self.size.height.min(self.size.width) / 2;
            let rectangle = embedded_graphics::primitives::Rectangle {
                top_left,
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

        fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
            let x = i16::interpolate(source.origin.x, target.origin.x, domain.factor);
            let y = i16::interpolate(source.origin.y, target.origin.y, domain.factor);
            let w = u16::interpolate(source.size.width, target.size.width, domain.factor);
            let h = u16::interpolate(source.size.height, target.size.height, domain.factor);
            Capsule::new(Point::new(x, y), Size::new(w, h))
        }
    }
}
