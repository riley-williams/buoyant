use crate::primitives::{Angle, Interpolate, Point};
use crate::render::shape::{AsShapePrimitive, Inset};

use super::{AnimatedJoin, AnimationDomain};

/// An arc with the origin at the top-left corner of its supporting circle
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arc {
    pub origin: Point,
    pub diameter: u32,
    pub start: Angle,
    pub end: Angle,
}

impl Arc {
    /// Creates a new arc with the given origin, diameter, and start and end angles.
    #[must_use]
    pub const fn new(origin: Point, diameter: u32, start: Angle, end: Angle) -> Self {
        Self {
            origin,
            diameter,
            start,
            end,
        }
    }
}

impl Inset for Arc {
    fn inset(mut self, amount: i32) -> Self {
        self.diameter = self.diameter.saturating_add_signed(-2 * amount);
        self.origin.x += amount;
        self.origin.y += amount;
        self
    }
}

impl AsShapePrimitive for Arc {
    type Primitive = crate::primitives::geometry::Arc;
    fn as_shape(&self) -> Self::Primitive {
        Self::Primitive::new(self.origin, self.diameter, self.start, self.end)
    }
}

impl AnimatedJoin for Arc {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        // Interpolate the bottom-right corner rather than the diameter to prevent jitter,
        // same as the circle renderer
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
        self.start = Angle::interpolate(source.start, self.start, domain.factor);
        self.end = Angle::interpolate(source.end, self.end, domain.factor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;

    fn animation_domain(factor: u8) -> AnimationDomain {
        AnimationDomain::new(factor, Duration::from_millis(100))
    }

    fn arc(origin: Point, diameter: u32, start: i32, end: i32) -> Arc {
        Arc::new(origin, diameter, Angle::degrees(start), Angle::degrees(end))
    }

    #[test]
    fn animated_join_at_start() {
        let source = arc(Point::new(0, 0), 10, 0, 90);
        let mut target = arc(Point::new(100, 50), 30, -90, 270);

        target.join_from(&source, &animation_domain(0));

        assert_eq!(target, source);
    }

    #[test]
    fn animated_join_at_end() {
        let source = arc(Point::new(0, 0), 10, 0, 90);
        let original_target = arc(Point::new(100, 50), 30, -90, 270);
        let mut target = original_target.clone();

        target.join_from(&source, &animation_domain(255));

        assert_eq!(target, original_target);
    }

    #[test]
    fn animated_join_interpolates_angles() {
        let source = arc(Point::new(0, 0), 10, 0, 0);
        let mut target = arc(Point::new(0, 0), 10, -90, 360);

        target.join_from(&source, &animation_domain(128));

        assert_eq!(
            target.start,
            Angle::interpolate(Angle::degrees(0), Angle::degrees(-90), 128)
        );
        assert_eq!(
            target.end,
            Angle::interpolate(Angle::degrees(0), Angle::degrees(360), 128)
        );
    }

    #[test]
    fn positive_inset_shrinks() {
        let inset_arc = arc(Point::new(10, 20), 100, 0, 90).inset(10);

        assert_eq!(inset_arc, arc(Point::new(20, 30), 80, 0, 90));
    }

    #[test]
    fn negative_inset_grows() {
        let inset_arc = arc(Point::new(10, 20), 100, 0, 90).inset(-10);

        assert_eq!(inset_arc, arc(Point::new(0, 10), 120, 0, 90));
    }

    #[test]
    fn overflowing_inset_saturates() {
        let inset_arc = arc(Point::new(10, 20), 100, 0, 90).inset(200);

        assert_eq!(inset_arc.origin, Point::new(210, 220));
        assert_eq!(inset_arc.diameter, 0);
    }

    #[test]
    fn trailing_corner_does_not_jitter() {
        let source = arc(Point::new(990, 990), 10, 0, 90);
        let original_target = arc(Point::new(0, 0), 1000, 0, 90);

        for factor in 0..=255 {
            let mut target = original_target.clone();
            target.join_from(&source, &animation_domain(factor));
            assert_eq!(target.origin.x + target.diameter as i32, 1000);
            assert_eq!(target.origin.y + target.diameter as i32, 1000);
        }
    }
}
