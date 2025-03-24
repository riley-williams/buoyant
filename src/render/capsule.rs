use crate::primitives::{Interpolate as _, Point, Size};
use crate::render_target::{geometry::RoundedRectangle, Brush, RenderTarget};

use super::{AnimatedJoin, AnimationDomain, Render};

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

impl<C: Copy> Render<C> for Capsule {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        style: &C,
        offset: Point,
    ) {
        let brush = Brush::Solid(*style);
        let radius = self.size.height.min(self.size.width) / 2;

        render_target.fill(
            offset.into(),
            brush,
            None,
            &RoundedRectangle::new(
                self.origin.into(),
                (self.size.width, self.size.height),
                radius,
            ),
        );
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
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
