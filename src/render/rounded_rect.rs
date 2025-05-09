use crate::{
    primitives::{geometry::RoundedRectangle, Interpolate, Point, Size},
    render::{AnimatedJoin, AnimationDomain},
    render_target::{RenderTarget, SolidBrush},
};

use super::Render;

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

impl<C: Copy> Render<C> for RoundedRect {
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
            &RoundedRectangle::new(
                self.origin,
                Size::new(self.size.width, self.size.height),
                self.corner_radius.into(),
            ),
        );
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
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
