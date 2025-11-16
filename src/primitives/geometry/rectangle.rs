use crate::primitives::{
    Point, Size,
    transform::{CoordinateSpaceTransform, LinearTransform},
};

use super::{PathEl, Shape, ShapePathIter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rectangle {
    pub origin: Point,
    pub size: Size,
}

impl Rectangle {
    #[must_use]
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    #[must_use]
    pub const fn intersects(&self, other: &Self) -> bool {
        let self_right = self.origin.x + self.size.width as i32;
        let self_bottom = self.origin.y + self.size.height as i32;
        let other_right = other.origin.x + other.size.width as i32;
        let other_bottom = other.origin.y + other.size.height as i32;

        !(self.origin.x >= other_right
            || self_right <= other.origin.x
            || self.origin.y >= other_bottom
            || self_bottom <= other.origin.y)
    }

    #[must_use]
    pub const fn contains(&self, point: &Point) -> bool {
        self.origin.x <= point.x
            && self.origin.y <= point.y
            && point.x < (self.origin.x + self.size.width as i32)
            && point.y < (self.origin.y + self.size.height as i32)
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        }

        let x1 = self.origin.x.max(other.origin.x);
        let y1 = self.origin.y.max(other.origin.y);
        let x2 =
            (self.origin.x + self.size.width as i32).min(other.origin.x + other.size.width as i32);
        let y2 = (self.origin.y + self.size.height as i32)
            .min(other.origin.y + other.size.height as i32);

        Some(Self {
            origin: Point::new(x1, y1),
            size: Size::new((x2 - x1) as u32, (y2 - y1) as u32),
        })
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let x1 = self.origin.x.min(other.origin.x);
        let y1 = self.origin.y.min(other.origin.y);
        let x2 =
            (self.origin.x + self.size.width as i32).max(other.origin.x + other.size.width as i32);
        let y2 = (self.origin.y + self.size.height as i32)
            .max(other.origin.y + other.size.height as i32);

        Self {
            origin: Point::new(x1, y1),
            size: Size::new((x2 - x1) as u32, (y2 - y1) as u32),
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            origin: Point::new(0, 0),
            size: Size::new(0, 0),
        }
    }
}

impl CoordinateSpaceTransform for Rectangle {
    fn applying(&self, transform: &LinearTransform) -> Self {
        Self {
            origin: self.origin.applying(transform),
            size: self.size.applying(transform),
        }
    }

    fn applying_inverse(&self, transform: &LinearTransform) -> Self {
        Self {
            origin: self.origin.applying_inverse(transform),
            size: self.size.applying_inverse(transform),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<embedded_graphics_core::primitives::Rectangle> for Rectangle {
    fn from(value: embedded_graphics_core::primitives::Rectangle) -> Self {
        Self {
            origin: value.top_left.into(),
            size: value.size.into(),
        }
    }
}
impl Shape for Rectangle {
    type PathElementsIter<'iter>
        = ShapePathIter<5>
    where
        Self: 'iter;

    fn path_elements(&self, _tolerance: u16) -> Self::PathElementsIter<'_> {
        let top_left = self.origin;
        let top_right = Point::new(self.origin.x + self.size.width as i32, self.origin.y);
        let bottom_right = Point::new(
            self.origin.x + self.size.width as i32,
            self.origin.y + self.size.height as i32,
        );
        let bottom_left = Point::new(self.origin.x, self.origin.y + self.size.height as i32);

        let elements = [
            PathEl::MoveTo(top_left),
            PathEl::LineTo(top_right),
            PathEl::LineTo(bottom_right),
            PathEl::LineTo(bottom_left),
            PathEl::ClosePath,
        ];
        ShapePathIter::new(elements)
    }

    fn bounding_box(&self) -> Rectangle {
        self.clone()
    }

    fn as_rect(&self) -> Option<Rectangle> {
        Some(self.clone())
    }
}
#[cfg(feature = "embedded-graphics")]
impl From<Rectangle> for embedded_graphics_core::primitives::Rectangle {
    fn from(value: Rectangle) -> Self {
        Self {
            top_left: value.origin.into(),
            size: value.size.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::transform::LinearTransform;

    #[test]
    fn applying_transform() {
        let rect = Rectangle::new(Point::new(10, 20), Size::new(40, 60));
        let transform = LinearTransform::new(Point::new(5, 10), 2.0);

        let transformed = rect.applying(&transform);

        assert_eq!(transformed.origin, Point::new(25, 50));
        assert_eq!(transformed.size, Size::new(80, 120));
    }

    #[test]
    fn applying_inverse_transform() {
        let rect = Rectangle::new(Point::new(50, 80), Size::new(80, 120));
        let transform = LinearTransform::new(Point::new(5, 10), 2.0);

        let inverse_transformed = rect.applying_inverse(&transform);

        assert_eq!(inverse_transformed.origin, Point::new(22, 35));
        assert_eq!(inverse_transformed.size, Size::new(40, 60));
    }

    #[test]
    fn transform_roundtrip() {
        let original = Rectangle::new(Point::new(16, 24), Size::new(32, 48));
        let transform = LinearTransform::new(Point::new(4, 8), 2.0);

        let transformed = original.applying(&transform).applying_inverse(&transform);

        assert_eq!(transformed.origin, original.origin);
        assert_eq!(transformed.size, original.size);
    }

    #[test]
    fn identity_transform() {
        let rect = Rectangle::new(Point::new(100, 200), Size::new(50, 75));
        let identity = LinearTransform::identity();

        let transformed = rect.applying(&identity);

        assert_eq!(transformed, rect);
    }

    #[test]
    fn intersection_overlapping_rectangles() {
        let rect1 = Rectangle::new(Point::new(10, 10), Size::new(20, 20));
        let rect2 = Rectangle::new(Point::new(20, 20), Size::new(20, 20));

        let intersection = rect1.intersection(&rect2);

        assert!(intersection.is_some());
        let intersection = intersection.unwrap();
        assert_eq!(intersection.origin, Point::new(20, 20));
        assert_eq!(intersection.size, Size::new(10, 10));
    }

    #[test]
    fn intersection_non_overlapping_rectangles() {
        let rect1 = Rectangle::new(Point::new(0, 0), Size::new(10, 10));
        let rect2 = Rectangle::new(Point::new(20, 20), Size::new(10, 10));

        let intersection = rect1.intersection(&rect2);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersection_identical_rectangles() {
        let rect1 = Rectangle::new(Point::new(5, 5), Size::new(15, 25));
        let rect2 = Rectangle::new(Point::new(5, 5), Size::new(15, 25));

        let intersection = rect1.intersection(&rect2);

        assert!(intersection.is_some());
        let intersection = intersection.unwrap();
        assert_eq!(intersection, rect1);
    }

    #[test]
    fn intersection_fully_inside() {
        let outer = Rectangle::new(Point::new(0, 0), Size::new(100, 100));
        let inner = Rectangle::new(Point::new(25, 25), Size::new(50, 50));

        let intersection0 = outer.intersection(&inner);
        let intersection1 = inner.intersection(&outer);

        assert_eq!(intersection0, intersection1);
        assert!(intersection0.is_some());
        let intersection = intersection0.unwrap();
        assert_eq!(intersection, inner);
    }

    #[test]
    fn intersection_touching_edges() {
        let rect1 = Rectangle::new(Point::new(0, 0), Size::new(10, 10));
        let rect2 = Rectangle::new(Point::new(10, 0), Size::new(10, 10));

        let intersection1 = rect1.intersection(&rect2);
        let intersection2 = rect2.intersection(&rect1);

        assert!(intersection1.is_none());
        assert!(intersection2.is_none());
    }

    #[test]
    fn intersection_touching_corners() {
        let rect1 = Rectangle::new(Point::new(0, 0), Size::new(10, 10));
        let rect2 = Rectangle::new(Point::new(10, 10), Size::new(10, 10));

        let intersection = rect1.intersection(&rect2);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersection_partial_overlap_horizontal() {
        let rect1 = Rectangle::new(Point::new(0, 10), Size::new(20, 20));
        let rect2 = Rectangle::new(Point::new(10, 10), Size::new(20, 20));

        let intersection = rect1.intersection(&rect2);

        assert!(intersection.is_some());
        let intersection = intersection.unwrap();
        assert_eq!(intersection.origin, Point::new(10, 10));
        assert_eq!(intersection.size, Size::new(10, 20));
    }

    #[test]
    fn intersection_partial_overlap_vertical() {
        let rect1 = Rectangle::new(Point::new(10, 0), Size::new(20, 20));
        let rect2 = Rectangle::new(Point::new(10, 10), Size::new(20, 20));

        let intersection = rect1.intersection(&rect2);

        assert!(intersection.is_some());
        let intersection = intersection.unwrap();
        assert_eq!(intersection.origin, Point::new(10, 10));
        assert_eq!(intersection.size, Size::new(20, 10));
    }

    #[test]
    fn intersection_with_negative_coordinates() {
        let rect1 = Rectangle::new(Point::new(-10, -10), Size::new(20, 20));
        let rect2 = Rectangle::new(Point::new(-5, -5), Size::new(20, 20));

        let intersection = rect1.intersection(&rect2);

        assert!(intersection.is_some());
        let intersection = intersection.unwrap();
        assert_eq!(intersection.origin, Point::new(-5, -5));
        assert_eq!(intersection.size, Size::new(15, 15));
    }

    #[test]
    fn union_overlapping_rectangles() {
        let rect1 = Rectangle::new(Point::new(10, 10), Size::new(20, 20));
        let rect2 = Rectangle::new(Point::new(20, 20), Size::new(20, 20));

        let union = rect1.union(&rect2);

        assert_eq!(union.origin, Point::new(10, 10));
        assert_eq!(union.size, Size::new(30, 30));
    }

    #[test]
    fn union_non_overlapping_rectangles() {
        let rect1 = Rectangle::new(Point::new(0, 0), Size::new(10, 10));
        let rect2 = Rectangle::new(Point::new(20, 20), Size::new(10, 10));

        let union = rect1.union(&rect2);

        assert_eq!(union.origin, Point::new(0, 0));
        assert_eq!(union.size, Size::new(30, 30));
    }

    #[test]
    fn union_identical_rectangles() {
        let rect1 = Rectangle::new(Point::new(5, 5), Size::new(15, 25));
        let rect2 = Rectangle::new(Point::new(5, 5), Size::new(15, 25));

        let union = rect1.union(&rect2);

        assert_eq!(union, rect1);
    }

    #[test]
    fn union_fully_inside() {
        let outer = Rectangle::new(Point::new(0, 0), Size::new(100, 100));
        let inner = Rectangle::new(Point::new(25, 25), Size::new(50, 50));

        let union0 = outer.union(&inner);
        let union1 = inner.union(&outer);

        assert_eq!(union0, union1);
        assert_eq!(union0, outer);
    }

    #[test]
    fn union_touching_corners() {
        let rect1 = Rectangle::new(Point::new(0, 0), Size::new(10, 10));
        let rect2 = Rectangle::new(Point::new(10, 10), Size::new(10, 10));

        let union = rect1.union(&rect2);

        assert_eq!(union.origin, Point::new(0, 0));
        assert_eq!(union.size, Size::new(20, 20));
    }

    #[test]
    fn union_partial_overlap_horizontal() {
        let rect1 = Rectangle::new(Point::new(0, 10), Size::new(20, 20));
        let rect2 = Rectangle::new(Point::new(10, 10), Size::new(20, 20));

        let union = rect1.union(&rect2);

        assert_eq!(union.origin, Point::new(0, 10));
        assert_eq!(union.size, Size::new(30, 20));
    }

    #[test]
    fn union_partial_overlap_vertical() {
        let rect1 = Rectangle::new(Point::new(10, 0), Size::new(20, 20));
        let rect2 = Rectangle::new(Point::new(10, 10), Size::new(20, 20));

        let union = rect1.union(&rect2);

        assert_eq!(union.origin, Point::new(10, 0));
        assert_eq!(union.size, Size::new(20, 30));
    }

    #[test]
    fn union_with_negative_coordinates() {
        let rect1 = Rectangle::new(Point::new(-10, -10), Size::new(20, 20));
        let rect2 = Rectangle::new(Point::new(-5, -5), Size::new(20, 20));

        let union = rect1.union(&rect2);

        assert_eq!(union.origin, Point::new(-10, -10));
        assert_eq!(union.size, Size::new(25, 25));
    }

    #[test]
    fn union_is_commutative() {
        let rect1 = Rectangle::new(Point::new(5, 10), Size::new(15, 20));
        let rect2 = Rectangle::new(Point::new(12, 8), Size::new(10, 25));

        let union1 = rect1.union(&rect2);
        let union2 = rect2.union(&rect1);

        assert_eq!(union1, union2);
    }
}
