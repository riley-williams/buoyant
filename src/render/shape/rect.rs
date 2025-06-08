use crate::primitives::geometry::Rectangle;
use crate::primitives::Interpolate;
use crate::primitives::{Point, Size};
use crate::render::shape::{AsShapePrimitive, Inset};
use crate::render::AnimationDomain;

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

impl Inset for Rect {
    fn inset(mut self, amount: i32) -> Self {
        self.size.width = self.size.width.saturating_add_signed(-2 * amount);
        self.size.height = self.size.height.saturating_add_signed(-2 * amount);
        self.origin.x += amount;
        self.origin.y += amount;
        self
    }
}

impl AsShapePrimitive for Rect {
    type Primitive = Rectangle;
    fn as_shape(&self) -> Self::Primitive {
        Rectangle::new(self.origin, Size::new(self.size.width, self.size.height))
    }
}

impl AnimatedJoin for Rect {
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let origin = Point::interpolate(source.origin, target.origin, domain.factor);
        let size = Size::interpolate(source.size, target.size, domain.factor);
        Self::new(origin, size)
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

    #[test]
    fn positive_inset_shrinks() {
        let rect = Rect {
            origin: Point::new(10, 20),
            size: Size::new(100, 200),
        };
        let inset_rect = rect.inset(10);

        assert_eq!(inset_rect.origin, Point::new(20, 30));
        assert_eq!(inset_rect.size, Size::new(80, 180));
    }

    #[test]
    fn negative_inset_grows() {
        let rect = Rect {
            origin: Point::new(10, 20),
            size: Size::new(100, 200),
        };
        let inset_rect = rect.inset(-10);

        assert_eq!(inset_rect.origin, Point::new(0, 10));
        assert_eq!(inset_rect.size, Size::new(120, 220));
    }

    #[test]
    fn overflowing_inset_saturates() {
        let rect = Rect {
            origin: Point::new(10, 20),
            size: Size::new(100, 200),
        };
        let inset_rect = rect.inset(200);

        // Inset larger than size should not result in negative dimensions
        assert_eq!(inset_rect.origin, Point::new(210, 220));
        assert_eq!(inset_rect.size, Size::new(0, 0));
    }
}
