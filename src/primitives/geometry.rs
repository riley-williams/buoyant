mod circle;
mod line;
mod rectangle;
mod rounded_rectangle;

pub use circle::Circle;
pub use line::Line;
pub use rectangle::Rectangle;
pub use rounded_rectangle::RoundedRectangle;

use super::Point;

/// The element of a Bézier path.
///
/// A valid path has `MoveTo` at the beginning of each subpath.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathEl {
    /// Move directly to the point without drawing anything, starting a new
    /// subpath.
    MoveTo(Point),
    /// Draw a line from the current location to the point.
    LineTo(Point),
    /// Draw a quadratic bezier using the current location and the two points.
    QuadTo(Point, Point),
    /// Draw a cubic bezier using the current location and the three points.
    CurveTo(Point, Point, Point),
    /// Close off the path.
    ClosePath,
}

pub trait Shape {
    type PathElementsIter<'iter>: Iterator<Item = PathEl> + 'iter
    where
        Self: 'iter;

    fn path_elements(&self, tolerance: u16) -> Self::PathElementsIter<'_>;

    /// The smallest rectangle that encloses the shape.
    fn bounding_box(&self) -> Rectangle;

    /// If the shape is a line, make it available.
    fn as_line(&self) -> Option<Line> {
        None
    }

    /// If the shape is a rectangle, make it available.
    fn as_rect(&self) -> Option<Rectangle> {
        None
    }

    /// If the shape is a rounded rectangle, make it available.
    fn as_rounded_rect(&self) -> Option<RoundedRectangle> {
        None
    }

    /// If the shape is a circle, make it available.
    fn as_circle(&self) -> Option<Circle> {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShapePathIter<const N: usize> {
    elements: [PathEl; N],
    index: usize,
}

impl<const N: usize> Iterator for ShapePathIter<N> {
    type Item = PathEl;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < N {
            let element = self.elements[self.index];
            self.index += 1;
            Some(element)
        } else {
            None
        }
    }
}

impl<const N: usize> ShapePathIter<N> {
    #[must_use]
    pub const fn new(elements: [PathEl; N]) -> Self {
        Self { elements, index: 0 }
    }
}

/// Describes the relationship between two rectangles.
#[allow(dead_code, reason = "unused with some feature combinations")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Intersection {
    /// The other rectangle is completely inside this rectangle.
    Contains,
    /// The other rectangle partially overlaps with this rectangle.
    Overlaps,
    /// The other rectangle does not intersect with this rectangle.
    NonIntersecting,
}
