use crate::primitives::{Interpolate, Point};
use crate::render::shape::{AsShapePrimitive, Inset};

use super::{AnimatedJoin, AnimationDomain};

/// A circle with the origin at the top-left corner
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circle {
    pub origin: Point,
    pub diameter: u32,
}

impl Circle {
    /// Creates a new circle with the given origin and diameter.
    #[must_use]
    pub const fn new(origin: Point, diameter: u32) -> Self {
        Self { origin, diameter }
    }
}

impl Inset for Circle {
    fn inset(mut self, amount: i32) -> Self {
        self.diameter = self.diameter.saturating_add_signed(-2 * amount);
        self.origin.x += amount;
        self.origin.y += amount;
        self
    }
}

impl AsShapePrimitive for Circle {
    type Primitive = crate::primitives::geometry::Circle;
    fn as_shape(&self) -> Self::Primitive {
        Self::Primitive::new(self.origin, self.diameter)
    }
}

impl AnimatedJoin for Circle {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        // Interpolating diameter can lead to jitter from lack of precision,
        // interpolate the bottom-right corner instead and derive diameter of the largest
        // fitting circle. Diameter drift is not noticeable, while drift in the leading/trailing
        // edges is.
        let bottom_right = Point::interpolate(
            source.origin + Point::new(source.diameter as i32, source.diameter as i32),
            self.origin + Point::new(self.diameter as i32, self.diameter as i32),
            domain.factor,
        );
        self.origin = Point::interpolate(source.origin, self.origin, domain.factor);
        self.diameter = bottom_right
            .x
            .abs_diff(self.origin.x)
            .max(bottom_right.y.abs_diff(self.origin.y));
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
        let mut target = Circle {
            origin: Point::new(100, 50),
            diameter: 30,
        };

        target.join_from(&source, &animation_domain(0));

        // At factor 0, should be identical to source
        assert_eq!(target.origin, source.origin);
        assert_eq!(target.diameter, source.diameter);
    }

    #[test]
    fn animated_join_at_end() {
        let source = Circle {
            origin: Point::new(0, 0),
            diameter: 10,
        };
        let original_target = Circle {
            origin: Point::new(100, 50),
            diameter: 30,
        };
        let mut target = original_target.clone();

        target.join_from(&source, &animation_domain(255));

        // At factor 255, should be identical to target
        assert_eq!(target.origin, original_target.origin);
        assert_eq!(target.diameter, original_target.diameter);
    }

    #[test]
    fn positive_inset_shrinks() {
        let circle = Circle {
            origin: Point::new(10, 20),
            diameter: 100,
        };
        let inset_circle = circle.inset(10);

        assert_eq!(inset_circle.origin, Point::new(20, 30));
        assert_eq!(inset_circle.diameter, 80);
    }

    #[test]
    fn negative_inset_grows() {
        let circle = Circle {
            origin: Point::new(10, 20),
            diameter: 100,
        };
        let inset_circle = circle.inset(-10);

        assert_eq!(inset_circle.origin, Point::new(0, 10));
        assert_eq!(inset_circle.diameter, 120);
    }

    #[test]
    fn overflowing_inset_saturates() {
        let circle = Circle {
            origin: Point::new(10, 20),
            diameter: 100,
        };
        let inset_circle = circle.inset(200);

        // Inset larger than diameter should not result in negative diameter
        assert_eq!(inset_circle.origin, Point::new(210, 220));
        assert_eq!(inset_circle.diameter, 0);
    }

    #[test]
    fn trailing_corner_does_not_jitter() {
        let source = Circle::new(Point::new(990, 990), 10);
        let original_target = Circle::new(Point::new(0, 0), 1000);

        for factor in 0..=255 {
            let mut target = original_target.clone();
            target.join_from(&source, &animation_domain(factor));
            assert_eq!(target.origin.x + target.diameter as i32, 1000);
            assert_eq!(target.origin.y + target.diameter as i32, 1000);
        }
    }
}
