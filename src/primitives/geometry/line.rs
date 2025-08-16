use crate::primitives::{
    geometry::{PathEl, Rectangle, Shape, ShapePathIter},
    transform::{CoordinateSpaceTransform, LinearTransform},
    Point, Size,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    #[must_use]
    pub const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

impl Shape for Line {
    type PathElementsIter<'iter>
        = ShapePathIter<2>
    where
        Self: 'iter;

    fn path_elements(&self, _tolerance: u16) -> Self::PathElementsIter<'_> {
        let elements = [PathEl::MoveTo(self.start), PathEl::LineTo(self.end)];
        ShapePathIter::new(elements)
    }

    fn bounding_box(&self) -> Rectangle {
        let min_x = self.start.x.min(self.end.x);
        let min_y = self.start.y.min(self.end.y);

        Rectangle::new(
            Point::new(min_x, min_y),
            Size::new(
                self.start.x.abs_diff(self.end.x),
                self.start.y.abs_diff(self.end.y),
            ),
        )
    }

    fn as_line(&self) -> Option<Line> {
        Some(self.clone())
    }
}

impl CoordinateSpaceTransform for Line {
    fn applying(&self, transform: &LinearTransform) -> Self {
        Self {
            start: self.start.applying(transform),
            end: self.end.applying(transform),
        }
    }

    fn applying_inverse(&self, transform: &LinearTransform) -> Self {
        Self {
            start: self.start.applying_inverse(transform),
            end: self.end.applying_inverse(transform),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<Line> for embedded_graphics::primitives::Line {
    fn from(value: Line) -> Self {
        Self::new(value.start.into(), value.end.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::transform::LinearTransform;

    #[test]
    fn applying_transform() {
        let line = Line::new(Point::new(10, 20), Point::new(30, 40));
        let transform = LinearTransform::new(Point::new(5, 10), 2.0);

        let transformed = line.applying(&transform);

        assert_eq!(transformed.start, Point::new(25, 50));
        assert_eq!(transformed.end, Point::new(65, 90));
    }

    #[test]
    fn applying_inverse_transform() {
        let line = Line::new(Point::new(50, 80), Point::new(70, 100));
        let transform = LinearTransform::new(Point::new(5, 10), 2.0);

        let inverse_transformed = line.applying_inverse(&transform);

        assert_eq!(inverse_transformed.start, Point::new(22, 35));
        assert_eq!(inverse_transformed.end, Point::new(32, 45));
    }

    #[test]
    fn transform_roundtrip() {
        let original = Line::new(Point::new(16, 24), Point::new(32, 48));
        let transform = LinearTransform::new(Point::new(4, 8), 2.0);

        let transformed = original.applying(&transform).applying_inverse(&transform);

        assert_eq!(transformed.start, original.start);
        assert_eq!(transformed.end, original.end);
    }

    #[test]
    fn identity_transform() {
        let line = Line::new(Point::new(100, 200), Point::new(150, 250));
        let identity = LinearTransform::identity();

        let transformed = line.applying(&identity);

        assert_eq!(transformed, line);
    }
}
