use crate::primitives::{Interpolate, Point};
use crate::render::shape::{AsShapePrimitive, Inset};

use super::{AnimatedJoin, AnimationDomain};

/// A circle with the origin at the top-left corner
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circle {
    pub origin: Point,
    pub diameter: u32,
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
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let origin = Point::interpolate(source.origin, target.origin, domain.factor);
        let diameter = u32::interpolate(source.diameter, target.diameter, domain.factor);
        Self { origin, diameter }
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
}
