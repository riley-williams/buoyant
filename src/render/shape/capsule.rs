use crate::primitives::{Interpolate as _, Point, Size, geometry::RoundedRectangle};
use crate::render::shape::{AsShapePrimitive, Inset};

use super::{AnimatedJoin, AnimationDomain};

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

impl Inset for Capsule {
    fn inset(mut self, amount: i32) -> Self {
        self.size.width = self.size.width.saturating_add_signed(-2 * amount);
        self.size.height = self.size.height.saturating_add_signed(-2 * amount);
        self.origin.x += amount;
        self.origin.y += amount;
        self
    }
}

impl AsShapePrimitive for Capsule {
    type Primitive = crate::primitives::geometry::RoundedRectangle;
    fn as_shape(&self) -> Self::Primitive {
        let radius = self.size.height.min(self.size.width) / 2;
        RoundedRectangle::new(
            self.origin,
            Size::new(self.size.width, self.size.height),
            radius,
        )
    }
}

impl AnimatedJoin for Capsule {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        // Avoid directly interpolating the size as it can lead to jitter from lack of precision
        let bottom_right = Point::interpolate(
            source.origin + source.size,
            self.origin + self.size,
            domain.factor,
        );
        self.origin = Point::interpolate(source.origin, self.origin, domain.factor);
        self.size = Size::new(
            bottom_right.x.abs_diff(self.origin.x),
            bottom_right.y.abs_diff(self.origin.y),
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
        let mut target = Capsule::new(Point::new(15, 25), Size::new(40, 50));

        target.join_from(&source, &animation_domain(0));

        assert_eq!(source.origin, target.origin);
        assert_eq!(source.size, target.size);
    }

    #[test]
    fn animated_join_at_end() {
        let source = Capsule::new(Point::new(5, 10), Size::new(20, 30));
        let original_target = Capsule::new(Point::new(15, 25), Size::new(40, 50));
        let mut target = original_target.clone();

        target.join_from(&source, &animation_domain(255));

        assert_eq!(target.origin, original_target.origin);
        assert_eq!(target.size, original_target.size);
    }

    #[test]
    fn positive_inset_shrinks() {
        let capsule = Capsule {
            origin: Point::new(10, 20),
            size: Size::new(100, 200),
        };
        let inset_capsule = capsule.inset(10);

        assert_eq!(inset_capsule.origin, Point::new(20, 30));
        assert_eq!(inset_capsule.size, Size::new(80, 180));
    }

    #[test]
    fn negative_inset_grows() {
        let capsule = Capsule {
            origin: Point::new(10, 20),
            size: Size::new(100, 200),
        };
        let inset_capsule = capsule.inset(-10);

        assert_eq!(inset_capsule.origin, Point::new(0, 10));
        assert_eq!(inset_capsule.size, Size::new(120, 220));
    }

    #[test]
    fn overflowing_inset_saturates() {
        let capsule = Capsule {
            origin: Point::new(10, 20),
            size: Size::new(100, 200),
        };
        let inset_capsule = capsule.inset(200);

        // Inset larger than size should not result in negative dimensions
        assert_eq!(inset_capsule.origin, Point::new(210, 220));
        assert_eq!(inset_capsule.size, Size::new(0, 0));
    }

    #[test]
    fn trailing_corner_does_not_jitter() {
        let source = Capsule::new(Point::new(990, 990), Size::new(10, 10));
        let original_target = Capsule::new(Point::new(0, 0), Size::new(1000, 1000));

        for factor in 0..=255 {
            let mut target = original_target.clone();
            target.join_from(&source, &animation_domain(factor));
            assert_eq!(target.origin + target.size, Point::new(1000, 1000));
        }
    }
}
