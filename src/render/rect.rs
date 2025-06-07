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
        Render::render(
            &AnimatedJoin::join(source.clone(), target.clone(), domain),
            render_target,
            style,
            offset,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;

    fn animation_domain(factor: u8) -> AnimationDomain {
        AnimationDomain::new(factor, Duration::from_millis(100))
    }

    #[test]
    fn animated_join_at_start() {
        let source = Rect::new(Point::new(0, 0), Size::new(10, 20));
        let target = Rect::new(Point::new(50, 30), Size::new(40, 60));

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(0));

        assert_eq!(result.origin, source.origin);
        assert_eq!(result.size, source.size);
    }

    #[test]
    fn animated_join_at_end() {
        let source = Rect::new(Point::new(0, 0), Size::new(10, 20));
        let target = Rect::new(Point::new(50, 30), Size::new(40, 60));

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(255));

        assert_eq!(result.origin, target.origin);
        assert_eq!(result.size, target.size);
    }
}
