use fixed::{traits::ToFixed, types::I18F14};
use fixed_macro::fixed;

use crate::primitives::{Interpolate, Point, geometry::Rectangle};

type FixedPoint = I18F14;

/// A normalized point in a view's coordinate space.
///
/// A `UnitPoint` of 1 represents the bottom or trailing edge of the view, while
/// 0 represents the leading or top edge. 0.5 represents the center of the view.
///
/// A unit point outside the range of 0 to 1 is valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitPoint {
    x: FixedPoint,
    y: FixedPoint,
}

impl UnitPoint {
    /// Creates a new `UnitPoint` with the given x and y coordinates.
    #[must_use]
    pub fn new(x: impl ToFixed, y: impl ToFixed) -> Self {
        Self {
            x: x.to_fixed(),
            y: y.to_fixed(),
        }
    }

    /// Converts `self` to a [`Point`] in the given frame's coordinate space.
    #[must_use]
    pub fn in_view_bounds(&self, frame: &Rectangle) -> Point {
        frame.origin
            + Point {
                x: (self.x * frame.size.width.to_fixed::<FixedPoint>()).to_num(),
                y: (self.y * frame.size.height.to_fixed::<FixedPoint>()).to_num(),
            }
    }

    #[must_use]
    pub const fn top_leading() -> Self {
        Self {
            x: fixed!(0.0: I18F14),
            y: fixed!(0.0: I18F14),
        }
    }

    #[must_use]
    pub const fn top() -> Self {
        Self {
            x: fixed!(0.5: I18F14),
            y: fixed!(0.0: I18F14),
        }
    }

    #[must_use]
    pub const fn top_trailing() -> Self {
        Self {
            x: fixed!(1.0: I18F14),
            y: fixed!(0.0: I18F14),
        }
    }

    #[must_use]
    pub const fn leading() -> Self {
        Self {
            x: fixed!(0.0: I18F14),
            y: fixed!(0.5: I18F14),
        }
    }

    #[must_use]
    pub const fn center() -> Self {
        Self {
            x: fixed!(0.5: I18F14),
            y: fixed!(0.5: I18F14),
        }
    }

    #[must_use]
    pub const fn trailing() -> Self {
        Self {
            x: fixed!(1.0: I18F14),
            y: fixed!(0.5: I18F14),
        }
    }

    #[must_use]
    pub const fn bottom_leading() -> Self {
        Self {
            x: fixed!(0.0: I18F14),
            y: fixed!(1.0: I18F14),
        }
    }

    #[must_use]
    pub const fn bottom() -> Self {
        Self {
            x: fixed!(0.5: I18F14),
            y: fixed!(1.0: I18F14),
        }
    }

    #[must_use]
    pub const fn bottom_trailing() -> Self {
        Self {
            x: fixed!(1.0: I18F14),
            y: fixed!(1.0: I18F14),
        }
    }
}

impl Interpolate for UnitPoint {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        Self {
            x: FixedPoint::interpolate(from.x, to.x, amount),
            y: FixedPoint::interpolate(from.y, to.y, amount),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Size;

    #[test]
    fn unit_point_in_view_bounds() {
        let frame = Rectangle::new(Point::new(5, 10), Size::new(12, 22));

        let test_points = [0.0, 0.2, 0.5, 0.8, 1.0];

        for x in test_points {
            for y in test_points {
                let unit_point = UnitPoint::new(x, y);
                let point = unit_point.in_view_bounds(&frame);
                assert!(
                    point.x >= frame.origin.x,
                    "Expected {} >= {}",
                    point.x,
                    frame.origin.x
                );
                assert!(
                    point.y >= frame.origin.y,
                    "Expected {} >= {}",
                    point.y,
                    frame.origin.y
                );
                assert!(
                    point.x <= frame.x_end(),
                    "Expected {} <= {}",
                    point.x,
                    frame.x_end()
                );
                assert!(
                    point.y <= frame.y_end(),
                    "Expected {} <= {}",
                    point.y,
                    frame.y_end()
                );
            }
        }
    }

    #[test]
    fn unit_point_convenience_inits() {
        let frame = Rectangle::new(Point::new(15, 25), Size::new(100, 80));
        let frame_center = frame.origin + Size::new(frame.size.width / 2, frame.size.height / 2);

        let point = UnitPoint::top_leading().in_view_bounds(&frame);
        assert_eq!(point, frame.origin);

        let point = UnitPoint::top().in_view_bounds(&frame);
        let expected = Point::new(frame_center.x, frame.origin.y);
        assert_eq!(point, expected);

        let point = UnitPoint::top_trailing().in_view_bounds(&frame);
        let expected = Point::new(frame.x_end(), frame.origin.y);
        assert_eq!(point, expected);

        let point = UnitPoint::leading().in_view_bounds(&frame);
        let expected = Point::new(frame.origin.x, frame_center.y);
        assert_eq!(point, expected);

        let point = UnitPoint::center().in_view_bounds(&frame);
        assert_eq!(point, frame_center);

        let point = UnitPoint::trailing().in_view_bounds(&frame);
        let expected = Point::new(frame.x_end(), frame_center.y);
        assert_eq!(point, expected);

        let point = UnitPoint::bottom_leading().in_view_bounds(&frame);
        let expected = Point::new(frame.origin.x, frame.y_end());
        assert_eq!(point, expected);

        let point = UnitPoint::bottom().in_view_bounds(&frame);
        let expected = Point::new(frame_center.x, frame.y_end());
        assert_eq!(point, expected);

        let point = UnitPoint::bottom_trailing().in_view_bounds(&frame);
        let expected = Point::new(frame.x_end(), frame.y_end());
        assert_eq!(point, expected);
    }

    #[test]
    fn interpolate_unit_point() {
        let from = UnitPoint::new(0.0, 0.0);
        let to = UnitPoint::new(1.0, 1.0);
        let interpolated = UnitPoint::interpolate(from, to, 0);
        assert_eq!(interpolated.x, 0.0.to_fixed::<FixedPoint>());
        assert_eq!(interpolated.y, 0.0.to_fixed::<FixedPoint>());

        // 0.5 is not exactly representable because of the u8 factor...
        let interpolated = UnitPoint::interpolate(from, to, 127);
        assert_eq!(interpolated.x, 0.498.to_fixed::<FixedPoint>());
        assert_eq!(interpolated.y, 0.498.to_fixed::<FixedPoint>());

        let interpolated = UnitPoint::interpolate(from, to, 255);
        assert_eq!(interpolated.x, 1.0.to_fixed::<FixedPoint>());
        assert_eq!(interpolated.y, 1.0.to_fixed::<FixedPoint>());
    }

    #[test]
    fn unit_point_interpolate_outside_0_to_1() {
        let from = UnitPoint::new(-1.5, 10.0);
        let to = UnitPoint::new(1.0, -20.0);
        let factor = 5;

        let interpolated = UnitPoint::interpolate(from, to, factor);

        let expected_x = f64::interpolate(from.x.to_num(), to.x.to_num(), factor);
        let expected_y = f64::interpolate(from.y.to_num(), to.y.to_num(), factor);
        assert!(interpolated.x.abs_diff(expected_x.to_fixed::<FixedPoint>()) < 0.001);
        assert!(interpolated.y.abs_diff(expected_y.to_fixed::<FixedPoint>()) < 0.001);
    }
}
