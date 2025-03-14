use crate::primitives::{Interpolate as _, Point, Size};

use super::{AnimatedJoin, AnimationDomain};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capsule {
    pub origin: Point,
    pub size: Size,
}

impl Capsule {
    #[must_use]
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
}

impl AnimatedJoin for Capsule {
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let origin = Point::interpolate(source.origin, target.origin, domain.factor);
        let size = Size::interpolate(source.size, target.size, domain.factor);
        Self::new(origin, size)
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use crate::{primitives::Point, render::AnimatedJoin};
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
