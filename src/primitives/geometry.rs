use super::{Point, Size};

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

#[cfg(feature = "embedded-graphics")]
impl From<Rectangle> for embedded_graphics_core::primitives::Rectangle {
    fn from(value: Rectangle) -> Self {
        Self {
            top_left: value.origin.into(),
            size: value.size.into(),
        }
    }
}

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
