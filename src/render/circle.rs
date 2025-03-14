use crate::primitives::{Interpolate, Point};

use super::{AnimatedJoin, AnimationDomain};

/// A circle with the origin at the top-left corner
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circle {
    pub origin: Point,
    pub diameter: u16,
}

impl AnimatedJoin for Circle {
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let origin = Point::interpolate(source.origin, target.origin, domain.factor);
        let diameter = u16::interpolate(source.diameter, target.diameter, domain.factor);
        Self { origin, diameter }
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use crate::primitives::Point;
    use crate::render::{AnimatedJoin, AnimationDomain, EmbeddedGraphicsRender};

    use super::Circle;
    use embedded_graphics::{
        prelude::PixelColor,
        primitives::{PrimitiveStyle, StyledDrawable as _},
    };
    use embedded_graphics_core::draw_target::DrawTarget;

    impl<C: PixelColor> EmbeddedGraphicsRender<C> for Circle {
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &C, offset: Point) {
            let center = self.origin + offset;
            _ = embedded_graphics::primitives::Circle::new(center.into(), self.diameter.into())
                .draw_styled(&PrimitiveStyle::with_fill(*style), render_target);
        }

        fn render_animated(
            render_target: &mut impl DrawTarget<Color = C>,
            source: &Self,
            target: &Self,
            style: &C,
            offset: Point,
            domain: &AnimationDomain,
        ) {
            // TODO: expecting these clones to be optimized away, check
            AnimatedJoin::join(source.clone(), target.clone(), domain).render(
                render_target,
                style,
                offset,
            );
        }
    }
}
