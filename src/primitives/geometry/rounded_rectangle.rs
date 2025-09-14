use fixed::traits::ToFixed as _;

use crate::primitives::{
    Point, Size,
    geometry::{PathEl, Rectangle, Shape, ShapePathIter},
    transform::{CoordinateSpaceTransform, LinearTransform, ScaleFactor},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoundedRectangle {
    pub origin: Point,
    pub size: Size,
    pub radius: u32,
}

impl RoundedRectangle {
    #[must_use]
    pub const fn new(origin: Point, size: Size, radius: u32) -> Self {
        Self {
            origin,
            size,
            radius,
        }
    }
}

impl CoordinateSpaceTransform for RoundedRectangle {
    fn applying(&self, transform: &LinearTransform) -> Self {
        Self {
            origin: self.origin.applying(transform),
            size: self.size.applying(transform),
            radius: (self.radius * transform.scale).to_num(),
        }
    }

    fn applying_inverse(&self, transform: &LinearTransform) -> Self {
        Self {
            origin: self.origin.applying_inverse(transform),
            size: self.size.applying_inverse(transform),
            radius: (self.radius.to_fixed::<ScaleFactor>() / transform.scale).to_num(),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<RoundedRectangle> for embedded_graphics::primitives::RoundedRectangle {
    fn from(value: RoundedRectangle) -> Self {
        use embedded_graphics::prelude::*;
        use embedded_graphics::primitives::CornerRadii;

        use crate::primitives::geometry::Rectangle;

        Self::new(
            Rectangle::new(value.origin, value.size).into(),
            CornerRadii::new(Size::new_equal(value.radius)),
        )
    }
}

impl Shape for RoundedRectangle {
    type PathElementsIter<'iter>
        = ShapePathIter<10>
    where
        Self: 'iter;

    fn path_elements(&self, _tolerance: u16) -> Self::PathElementsIter<'_> {
        let r = self.radius as i32;
        let width = self.size.width as i32;
        let height = self.size.height as i32;
        let x = self.origin.x;
        let y = self.origin.y;

        // FIXME: The quad points used here are suboptimal
        let elements = [
            // Starting point
            PathEl::MoveTo(Point::new(x + width - r, y)),
            // Top edge and top-left corner
            PathEl::LineTo(Point::new(x + r, y)),
            PathEl::QuadTo(Point::new(x, y), Point::new(x, y + r)),
            // Left side
            PathEl::LineTo(Point::new(x, y + height - r)),
            // Bottom-left corner
            PathEl::QuadTo(Point::new(x, y + height), Point::new(x + r, y + height)),
            // Bottom side
            PathEl::LineTo(Point::new(x + width - r, y + height)),
            // Bottom-right corner
            PathEl::QuadTo(
                Point::new(x + width, y + height),
                Point::new(x + width, y + height - r),
            ),
            // Right side
            PathEl::LineTo(Point::new(x + width, y + r)),
            // Top-right corner
            PathEl::QuadTo(Point::new(x + width, y), Point::new(x + width - r, y)),
            // Close the path
            PathEl::ClosePath,
        ];

        ShapePathIter::new(elements)
    }

    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.origin, self.size)
    }

    fn as_rounded_rect(&self) -> Option<RoundedRectangle> {
        Some(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::transform::LinearTransform;

    #[test]
    fn applying_transform() {
        let rounded_rect = RoundedRectangle::new(Point::new(10, 20), Size::new(40, 60), 8);
        let transform = LinearTransform::new(Point::new(5, 10), 2.0);

        let transformed = rounded_rect.applying(&transform);

        assert_eq!(transformed.origin, Point::new(25, 50));
        assert_eq!(transformed.size, Size::new(80, 120));
        assert_eq!(transformed.radius, 16);
    }

    #[test]
    fn applying_inverse_transform() {
        let rounded_rect = RoundedRectangle::new(Point::new(50, 80), Size::new(80, 120), 16);
        let transform = LinearTransform::new(Point::new(5, 10), 2.0);

        let inverse_transformed = rounded_rect.applying_inverse(&transform);

        assert_eq!(inverse_transformed.origin, Point::new(22, 35));
        assert_eq!(inverse_transformed.size, Size::new(40, 60));
        assert_eq!(inverse_transformed.radius, 8);
    }

    #[test]
    fn transform_roundtrip() {
        let original = RoundedRectangle::new(Point::new(16, 24), Size::new(32, 48), 8);
        let transform = LinearTransform::new(Point::new(4, 8), 2.0);

        let transformed = original.applying(&transform).applying_inverse(&transform);

        assert_eq!(transformed.origin, original.origin);
        assert_eq!(transformed.size, original.size);
        assert_eq!(transformed.radius, original.radius);
    }

    #[test]
    fn identity_transform() {
        let rounded_rect = RoundedRectangle::new(Point::new(100, 200), Size::new(50, 75), 10);
        let identity = LinearTransform::identity();

        let transformed = rounded_rect.applying(&identity);

        assert_eq!(transformed, rounded_rect);
    }
}
