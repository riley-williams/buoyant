use super::{PathEl, Shape};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<crate::primitives::Point> for Point {
    fn from(value: crate::primitives::Point) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
        }
    }
}

impl core::ops::Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
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
    // Top left corner of the bounding box
    pub origin: Point,
    pub diameter: i32,
}

impl Circle {
    #[must_use]
    pub const fn new(origin: Point, diameter: i32) -> Self {
        Self { origin, diameter }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rectangle {
    pub origin: Point,
    pub size: (u32, u32),
}

impl Rectangle {
    #[must_use]
    pub const fn new(origin: Point, size: (u32, u32)) -> Self {
        Self {
            origin,
            size: (size.0, size.1),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<embedded_graphics_core::primitives::Rectangle> for Rectangle {
    fn from(value: embedded_graphics_core::primitives::Rectangle) -> Self {
        Self {
            origin: Point::new(value.top_left.x, value.top_left.y),
            size: (value.size.width, value.size.height),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoundedRectangle {
    pub origin: Point,
    pub size: (i32, i32),
    pub radius: i32,
}

impl RoundedRectangle {
    #[must_use]
    pub const fn new(origin: Point, size: (u16, u16), radius: u16) -> Self {
        Self {
            origin,
            size: (size.0 as i32, size.1 as i32),
            radius: radius as i32,
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
        let max_x = self.start.x.max(self.end.x);
        let max_y = self.start.y.max(self.end.y);

        Rectangle::new(
            Point::new(min_x, min_y),
            ((max_x - min_x) as u32, (max_y - min_y) as u32),
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
        let top_right = Point::new(self.origin.x + self.size.0 as i32, self.origin.y);
        let bottom_right = Point::new(
            self.origin.x + self.size.0 as i32,
            self.origin.y + self.size.1 as i32,
        );
        let bottom_left = Point::new(self.origin.x, self.origin.y + self.size.1 as i32);

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
        let radius = self.diameter as f32 / 2.0;
        let center_x = self.origin.x as f32 + radius;
        let center_y = self.origin.y as f32 + radius;

        let mut elements = [PathEl::ClosePath; 66];

        // First point (MoveTo)
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
        Rectangle::new(self.origin, (self.diameter as u32, self.diameter as u32))
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
        let r = self.radius;
        let width = self.size.0;
        let height = self.size.1;
        let x = self.origin.x;
        let y = self.origin.y;

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
        Rectangle::new(self.origin, (self.size.0 as u32, self.size.1 as u32))
    }

    fn as_rounded_rect(&self) -> Option<RoundedRectangle> {
        Some(self.clone())
    }
}
