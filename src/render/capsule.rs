use crate::primitives::geometry::RoundedRectangle;
use crate::primitives::{Interpolate as _, Point, Size};
use crate::render_target::RenderTarget;
use crate::render_target::SolidBrush;

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
        let brush = SolidBrush::new(*style);
        let radius = self.size.height.min(self.size.width) / 2;

        render_target.fill(
            offset,
            &brush,
            None,
            &RoundedRectangle::new(
                self.origin,
                Size::new(self.size.width, self.size.height),
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
        AnimatedJoin::join(source.clone(), target.clone(), domain).render(
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
        let source = Capsule::new(Point::new(5, 10), Size::new(20, 30));
        let target = Capsule::new(Point::new(15, 25), Size::new(40, 50));

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(0));

        assert_eq!(result.origin, source.origin);
        assert_eq!(result.size, source.size);
    }

    #[test]
    fn animated_join_at_end() {
        let source = Capsule::new(Point::new(5, 10), Size::new(20, 30));
        let target = Capsule::new(Point::new(15, 25), Size::new(40, 50));

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(255));

        assert_eq!(result.origin, target.origin);
        assert_eq!(result.size, target.size);
    }
}
