use fixed::traits::ToFixed as _;

use crate::primitives::{
    Point, Size,
    geometry::{PathEl, Rectangle, Shape, ShapePathIter},
    transform::{CoordinateSpaceTransform, LinearTransform, ScaleFactor},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Circle {
    /// Top left corner of the bounding box
    pub origin: Point,
    pub diameter: u32,
}

impl Circle {
    #[must_use]
    pub const fn new(origin: Point, diameter: u32) -> Self {
        Self { origin, diameter }
    }
}

impl Shape for Circle {
    // Use a const generic with enough capacity for most cases
    // 66 = MoveTo + 64 segments + ClosePath
    type PathElementsIter<'iter>
        = ShapePathIter<66>
    where
        Self: 'iter;

    #[expect(clippy::cast_precision_loss)]
    fn path_elements(&self, _tolerance: u16) -> Self::PathElementsIter<'_> {
        // FIXME: This can be approximated quite well with a 4x cubic bezier
        let radius = self.diameter as f32 / 2.0;
        let center_x = self.origin.x as f32 + radius;
        let center_y = self.origin.y as f32 + radius;

        let mut elements = [PathEl::ClosePath; 66];

        let first_point = Point::new((center_x + radius) as i32, center_y as i32);
        elements[0] = PathEl::MoveTo(first_point);

        // FIXME: This is lazy, need to implement actual circle
        #[cfg(feature = "std")]
        (1..=64).for_each(|i| {
            let angle = (i as f32 * 2.0 * core::f32::consts::PI) / 64.0;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            elements[i] = PathEl::LineTo(Point::new(x as i32, y as i32));
        });

        // Close the path (already initialized with ClosePath)

        ShapePathIter::new(elements)
    }

    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.origin, Size::new(self.diameter, self.diameter))
    }

    fn as_circle(&self) -> Option<Circle> {
        Some(self.clone())
    }
}

impl CoordinateSpaceTransform for Circle {
    fn applying(&self, transform: &LinearTransform) -> Self {
        Self {
            origin: self.origin.applying(transform),
            diameter: (self.diameter * transform.scale).to_num(),
        }
    }

    fn applying_inverse(&self, transform: &LinearTransform) -> Self {
        Self {
            origin: self.origin.applying_inverse(transform),
            diameter: (self.diameter.to_fixed::<ScaleFactor>() / transform.scale).to_num(),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<Circle> for embedded_graphics::primitives::Circle {
    fn from(value: Circle) -> Self {
        Self::new(value.origin.into(), value.diameter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::transform::LinearTransform;

    #[test]
    fn applying_transform() {
        let circle = Circle::new(Point::new(10, 20), 40);
        let transform = LinearTransform::new(Point::new(5, 10), 2.0);

        let transformed = circle.applying(&transform);

        assert_eq!(transformed.origin, Point::new(25, 50));
        assert_eq!(transformed.diameter, 80);
    }

    #[test]
    fn applying_inverse_transform() {
        let circle = Circle::new(Point::new(50, 80), 80);
        let transform = LinearTransform::new(Point::new(5, 10), 2.0);

        let inverse_transformed = circle.applying_inverse(&transform);

        // Origin should be inverse transformed: ((50 - 5) / 2, (80 - 10) / 2) = (22, 35)
        assert_eq!(inverse_transformed.origin, Point::new(22, 35));
        // Diameter should be inverse scaled: 80 / 2 = 40
        assert_eq!(inverse_transformed.diameter, 40);
    }

    #[test]
    fn transform_roundtrip() {
        let original = Circle::new(Point::new(16, 24), 32);
        let transform = LinearTransform::new(Point::new(4, 8), 2.0);

        let transformed = original.applying(&transform).applying_inverse(&transform);

        assert_eq!(transformed.origin, original.origin);
        assert_eq!(transformed.diameter, original.diameter);
    }

    #[test]
    fn identity_transform() {
        let circle = Circle::new(Point::new(100, 200), 50);
        let identity = LinearTransform::identity();

        let transformed = circle.applying(&identity);

        assert_eq!(transformed, circle);
    }
}
