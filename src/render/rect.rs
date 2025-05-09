use crate::primitives::geometry::Rectangle;
use crate::primitives::Interpolate;
use crate::primitives::{Point, Size};
use crate::render::{AnimationDomain, Render, RenderTarget};
use crate::render_target::SolidBrush;

use super::AnimatedJoin;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    #[must_use]
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
}

impl AnimatedJoin for Rect {
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let origin = Point::interpolate(source.origin, target.origin, domain.factor);
        let size = Size::interpolate(source.size, target.size, domain.factor);
        Self::new(origin, size)
    }
}

impl<C: Copy> Render<C> for Rect {
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
            &Rectangle::new(self.origin, Size::new(self.size.width, self.size.height)),
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
        Render::render(
            &AnimatedJoin::join(source.clone(), target.clone(), domain),
            render_target,
            style,
            offset,
        );
    }
}
