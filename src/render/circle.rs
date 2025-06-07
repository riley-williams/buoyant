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
        let source = Circle {
            origin: Point::new(0, 0),
            diameter: 10,
        };
        let target = Circle {
            origin: Point::new(100, 50),
            diameter: 30,
        };

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(0));

        // At factor 0, should be identical to source
        assert_eq!(result.origin, source.origin);
        assert_eq!(result.diameter, source.diameter);
    }

    #[test]
    fn animated_join_at_end() {
        let source = Circle {
            origin: Point::new(0, 0),
            diameter: 10,
        };
        let target = Circle {
            origin: Point::new(100, 50),
            diameter: 30,
        };

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(255));

        // At factor 255, should be identical to target
        assert_eq!(result.origin, target.origin);
        assert_eq!(result.diameter, target.diameter);
    }
}
