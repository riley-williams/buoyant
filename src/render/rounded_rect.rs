use crate::{
    primitives::{Interpolate, Point, Size},
    render::{AnimatedJoin, AnimationDomain},
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct RoundedRect {
    pub origin: Point,
    pub size: Size,
    pub corner_radius: u16,
}

impl AnimatedJoin for RoundedRect {
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let origin = Point::interpolate(source.origin, target.origin, domain.factor);
        let size = Size::interpolate(source.size, target.size, domain.factor);
        let r = u16::interpolate(source.corner_radius, target.corner_radius, domain.factor);
        Self {
            origin,
            size,
            corner_radius: r,
        }
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use embedded_graphics::{
        prelude::PixelColor,
        primitives::{PrimitiveStyle, StyledDrawable as _},
    };
    use embedded_graphics_core::draw_target::DrawTarget;

    use crate::render::{AnimatedJoin, EmbeddedGraphicsRender};

    use super::{Point, RoundedRect};

    impl<C: PixelColor> EmbeddedGraphicsRender<C> for RoundedRect {
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &C, offset: Point) {
            let top_left = (self.origin + offset).into();
            _ = embedded_graphics::primitives::RoundedRectangle::new(
                embedded_graphics::primitives::Rectangle {
                    top_left,
                    size: self.size.into(),
                },
                embedded_graphics::primitives::CornerRadii::new(
                    (u32::from(self.corner_radius), u32::from(self.corner_radius)).into(),
                ),
            )
            .draw_styled(&PrimitiveStyle::with_fill(*style), render_target);
        }

        fn render_animated(
            render_target: &mut impl DrawTarget<Color = C>,
            source: &Self,
            target: &Self,
            style: &C,
            offset: Point,
            domain: &crate::render::AnimationDomain,
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
