use crate::{
    primitives::{geometry::RoundedRectangle, Interpolate, Point, Size},
    render::{
        shape::{AsShapePrimitive, Inset},
        AnimatedJoin, AnimationDomain,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct RoundedRect {
    pub origin: Point,
    pub size: Size,
    pub corner_radius: u16,
}

impl RoundedRect {
    #[must_use]
    pub const fn new(origin: Point, size: Size, corner_radius: u16) -> Self {
        Self {
            origin,
            size,
            corner_radius,
        }
    }
}

impl Inset for RoundedRect {
    fn inset(mut self, amount: i32) -> Self {
        self.size.width = self.size.width.saturating_add_signed(-2 * amount);
        self.size.height = self.size.height.saturating_add_signed(-2 * amount);
        self.corner_radius = self.corner_radius.saturating_add_signed(-amount as i16);
        self.origin.x += amount;
        self.origin.y += amount;
        self
    }
}

impl AsShapePrimitive for RoundedRect {
    type Primitive = RoundedRectangle;
    fn as_shape(&self) -> Self::Primitive {
        RoundedRectangle::new(
            self.origin,
            Size::new(self.size.width, self.size.height),
            self.corner_radius.into(),
        )
    }
}

impl AnimatedJoin for RoundedRect {
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
        self.corner_radius =
            u16::interpolate(source.corner_radius, self.corner_radius, domain.factor);
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
        let source = RoundedRect {
            origin: Point::new(0, 0),
            size: Size::new(20, 30),
            corner_radius: 5,
        };
        let mut target = RoundedRect {
            origin: Point::new(10, 15),
            size: Size::new(40, 60),
            corner_radius: 15,
        };

        target.join_from(&source, &animation_domain(0));

        assert_eq!(target.origin, source.origin);
        assert_eq!(target.size, source.size);
        assert_eq!(target.corner_radius, source.corner_radius);
    }

    #[test]
    fn animated_join_at_end() {
        let source = RoundedRect {
            origin: Point::new(0, 0),
            size: Size::new(20, 30),
            corner_radius: 5,
        };
        let original_target = RoundedRect {
            origin: Point::new(10, 15),
            size: Size::new(40, 60),
            corner_radius: 15,
        };
        let mut target = original_target.clone();

        target.join_from(&source, &animation_domain(255));

        assert_eq!(target.origin, original_target.origin);
        assert_eq!(target.size, original_target.size);
        assert_eq!(target.corner_radius, original_target.corner_radius);
    }

    #[test]
    fn positive_inset_shrinks() {
        let rect = RoundedRect {
            origin: Point::new(10, 20),
            size: Size::new(100, 200),
            corner_radius: 30,
        };
        let inset_rect = rect.inset(10);

        assert_eq!(inset_rect.origin, Point::new(20, 30));
        assert_eq!(inset_rect.size, Size::new(80, 180));
        assert_eq!(inset_rect.corner_radius, 20);
    }

    #[test]
    fn negative_inset_grows() {
        let rect = RoundedRect {
            origin: Point::new(10, 20),
            size: Size::new(100, 200),
            corner_radius: 30,
        };
        let inset_rect = rect.inset(-10);

        assert_eq!(inset_rect.origin, Point::new(0, 10));
        assert_eq!(inset_rect.size, Size::new(120, 220));
        assert_eq!(inset_rect.corner_radius, 40);
    }

    #[test]
    fn overflowing_inset_saturates() {
        let rect = RoundedRect {
            origin: Point::new(10, 20),
            size: Size::new(100, 200),
            corner_radius: 30,
        };
        let inset_rect = rect.inset(200);

        // Inset larger than size should not result in negative dimensions
        assert_eq!(inset_rect.origin, Point::new(210, 220));
        assert_eq!(inset_rect.size, Size::new(0, 0));
        assert_eq!(inset_rect.corner_radius, 0);
    }

    #[test]
    fn trailing_corner_does_not_jitter() {
        let source = RoundedRect::new(Point::new(990, 990), Size::new(10, 10), 5);
        let original_target = RoundedRect::new(Point::new(0, 0), Size::new(1000, 1000), 70);

        for factor in 0..=255 {
            let mut target = original_target.clone();
            target.join_from(&source, &animation_domain(factor));
            assert_eq!(target.origin + target.size, Point::new(1000, 1000));
        }
    }
}
