use crate::{
    primitives::{geometry, Interpolate, Point},
    render_target::{RenderTarget, SolidBrush},
};

use super::{AnimatedJoin, AnimationDomain, Render};

/// A circle with the origin at the top-left corner
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circle {
    pub origin: Point,
    pub diameter: u32,
}

impl AnimatedJoin for Circle {
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let origin = Point::interpolate(source.origin, target.origin, domain.factor);
        let diameter = u32::interpolate(source.diameter, target.diameter, domain.factor);
        Self { origin, diameter }
    }
}

impl<C: Copy> Render<C> for Circle {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        style: &C,
        offset: Point,
    ) {
        let brush = SolidBrush::new(*style);
        render_target.fill(
            offset,
            &brush,
            None,
            &geometry::Circle::new(self.origin, self.diameter),
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
