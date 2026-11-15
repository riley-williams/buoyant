use fixed::traits::ToFixed as _;

use crate::primitives::{
    Angle, Point, Size,
    geometry::{PathEl, Rectangle, Shape, ShapePathIter},
    transform::{CoordinateSpaceTransform, LinearTransform, ScaleFactor},
};

/// An arc along the edge of a circle, from `start` to `end`.
///
/// Angles increase clockwise from the positive x-axis. If `end` is less than `start`,
/// the arc is drawn counterclockwise.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arc {
    /// Top left corner of the bounding box of the circle supporting the arc
    pub origin: Point,
    pub diameter: u32,
    pub start: Angle,
    pub end: Angle,
}

impl Arc {
    #[must_use]
    pub const fn new(origin: Point, diameter: u32, start: Angle, end: Angle) -> Self {
        Self {
            origin,
            diameter,
            start,
            end,
        }
    }

    /// Returns an arc offset by the given point.
    #[must_use]
    pub fn with_offset(mut self, offset: Point) -> Self {
        self.origin += offset;
        self
    }

    /// Offsets the arc by the given point
    pub fn offset(&mut self, offset: Point) {
        self.origin += offset;
    }
}

impl Shape for Arc {
    // 65 = MoveTo + 64 segments
    type PathElementsIter<'iter>
        = ShapePathIter<65>
    where
        Self: 'iter;

    #[expect(clippy::cast_precision_loss)]
    fn path_elements(&self, _tolerance: u16) -> Self::PathElementsIter<'_> {
        let radius = self.diameter as f32 / 2.0;
        let center_x = self.origin.x as f32 + radius;
        let center_y = self.origin.y as f32 + radius;

        // Repeated moves draw nothing, so unfilled elements are harmless
        let center = Point::new(center_x as i32, center_y as i32);
        let mut elements = [PathEl::MoveTo(center); 65];

        // FIXME: Like Circle, this needs trig functions which are only available with std
        #[cfg(feature = "std")]
        {
            let start = self.start.to_radians();
            let sweep = self.end.to_radians() - start;
            elements.iter_mut().enumerate().for_each(|(i, element)| {
                let angle = start + sweep * i as f32 / 64.0;
                let point = Point::new(
                    (center_x + radius * angle.cos()) as i32,
                    (center_y + radius * angle.sin()) as i32,
                );
                *element = if i == 0 {
                    PathEl::MoveTo(point)
                } else {
                    PathEl::LineTo(point)
                };
            });
        }

        ShapePathIter::new(elements)
    }

    // FIXME: This is the bounding box of the full circle, not the tight bounds of the arc
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.origin, Size::new(self.diameter, self.diameter))
    }

    fn as_arc(&self) -> Option<Arc> {
        Some(self.clone())
    }
}

impl CoordinateSpaceTransform for Arc {
    fn applying(&self, transform: &LinearTransform) -> Self {
        Self {
            origin: self.origin.applying(transform),
            diameter: (self.diameter * transform.scale).to_num(),
            start: self.start,
            end: self.end,
        }
    }

    fn applying_inverse(&self, transform: &LinearTransform) -> Self {
        Self {
            origin: self.origin.applying_inverse(transform),
            diameter: (self.diameter.to_fixed::<ScaleFactor>() / transform.scale).to_num(),
            start: self.start,
            end: self.end,
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<Arc> for embedded_graphics::primitives::Arc {
    fn from(value: Arc) -> Self {
        Self::new(
            value.origin.into(),
            value.diameter,
            value.start.into(),
            embedded_graphics::geometry::Angle::from_degrees(
                value.end.to_degrees() - value.start.to_degrees(),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::transform::LinearTransform;

    fn arc(origin: Point, diameter: u32) -> Arc {
        Arc::new(origin, diameter, Angle::degrees(-90), Angle::degrees(180))
    }

    #[test]
    fn applying_transform() {
        let transformed =
            arc(Point::new(10, 20), 40).applying(&LinearTransform::new(Point::new(5, 10), 2.0));

        assert_eq!(transformed.origin, Point::new(25, 50));
        assert_eq!(transformed.diameter, 80);
        assert_eq!(transformed.start, Angle::degrees(-90));
        assert_eq!(transformed.end, Angle::degrees(180));
    }

    #[test]
    fn applying_inverse_transform() {
        let inverse_transformed = arc(Point::new(50, 80), 80)
            .applying_inverse(&LinearTransform::new(Point::new(5, 10), 2.0));

        assert_eq!(inverse_transformed.origin, Point::new(22, 35));
        assert_eq!(inverse_transformed.diameter, 40);
        assert_eq!(inverse_transformed.start, Angle::degrees(-90));
        assert_eq!(inverse_transformed.end, Angle::degrees(180));
    }

    #[test]
    fn transform_roundtrip() {
        let original = arc(Point::new(16, 24), 32);
        let transform = LinearTransform::new(Point::new(4, 8), 2.0);

        let transformed = original.applying(&transform).applying_inverse(&transform);

        assert_eq!(transformed, original);
    }

    #[test]
    fn identity_transform() {
        let original = arc(Point::new(100, 200), 50);

        assert_eq!(original.applying(&LinearTransform::identity()), original);
    }

    #[test]
    fn bounding_box_is_full_circle() {
        let arc = Arc::new(Point::new(5, 6), 20, Angle::degrees(0), Angle::degrees(10));

        assert_eq!(
            arc.bounding_box(),
            Rectangle::new(Point::new(5, 6), Size::new(20, 20))
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn path_is_open_and_follows_the_arc() {
        let arc = Arc::new(Point::new(0, 0), 100, Angle::degrees(0), Angle::degrees(90));
        let elements: std::vec::Vec<_> = arc.path_elements(1).collect();

        assert_eq!(elements.len(), 65);
        assert_eq!(elements[0], PathEl::MoveTo(Point::new(100, 50)));
        // Points are truncated, so allow for a pixel of error
        let PathEl::LineTo(end) = elements[64] else {
            panic!("expected the path to end with a line");
        };
        assert!(
            end.x.abs_diff(50) <= 1 && end.y.abs_diff(100) <= 1,
            "{end:?}"
        );
        assert!(!elements.contains(&PathEl::ClosePath));
    }

    #[cfg(feature = "embedded-graphics")]
    #[test]
    fn converts_end_angle_to_sweep() {
        let arc = Arc::new(Point::new(1, 2), 30, Angle::degrees(90), Angle::degrees(45));
        let eg_arc: embedded_graphics::primitives::Arc = arc.into();

        assert_eq!(
            eg_arc.top_left,
            embedded_graphics::geometry::Point::new(1, 2)
        );
        assert_eq!(eg_arc.diameter, 30);
        assert_eq!(eg_arc.angle_start.to_degrees(), 90.0);
        assert_eq!(eg_arc.angle_sweep.to_degrees(), -45.0);
    }
}
