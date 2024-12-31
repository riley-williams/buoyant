mod dimension;
pub use dimension::*;

use core::cmp::max;

use crate::pixel::Interpolate;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Size { width, height }
    }

    /// Returns the smallest size that contains both sizes.
    pub fn union(&self, rhs: Size) -> Size {
        Size {
            width: max(self.width, rhs.width),
            height: max(self.height, rhs.height),
        }
    }

    /// Returns the overlapping area of the two sizes, which is the min of the two dimensions.
    pub fn intersection(&self, rhs: Size) -> Size {
        Size {
            width: self.width.min(rhs.width),
            height: self.height.min(rhs.height),
        }
    }

    pub fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }

    /// Returns true if the point is non-negative and within the bounds of the size.
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width as i16 && point.y < self.height as i16
    }

    #[inline]
    pub fn area(&self) -> u16 {
        self.width * self.height
    }
}

impl core::ops::Add for Size {
    type Output = Size;
    fn add(self, rhs: Size) -> Size {
        Size {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<embedded_graphics_core::geometry::Size> for Size {
    fn from(value: embedded_graphics_core::geometry::Size) -> Self {
        Size {
            width: value.width as u16,
            height: value.height as u16,
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<Size> for embedded_graphics_core::geometry::Size {
    fn from(value: Size) -> Self {
        embedded_graphics_core::geometry::Size::new(value.width as u32, value.height as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl core::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    pub fn new(x: i16, y: i16) -> Self {
        Point { x, y }
    }

    pub fn zero() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl Interpolate for Point {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        Point {
            x: (((amount as i32 * to.x as i32) + ((255 - amount) as i32 * from.x as i32)) / 255)
                as i16,
            y: (((amount as i32 * to.y as i32) + ((255 - amount) as i32 * from.y as i32)) / 255)
                as i16,
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<Point> for embedded_graphics_core::geometry::Point {
    fn from(value: Point) -> Self {
        embedded_graphics_core::geometry::Point::new(value.x as i32, value.y as i32)
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<embedded_graphics_core::geometry::Point> for Point {
    fn from(value: embedded_graphics_core::geometry::Point) -> Self {
        Point {
            x: value.x as i16,
            y: value.y as i16,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Frame {
    pub size: Size,
    pub origin: Point,
}

impl Frame {
    pub fn new(origin: Point, size: Size) -> Self {
        Frame { origin, size }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<Frame> for embedded_graphics_core::primitives::Rectangle {
    fn from(value: Frame) -> Self {
        embedded_graphics_core::primitives::Rectangle::new(value.origin.into(), value.size.into())
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<embedded_graphics_core::primitives::Rectangle> for Frame {
    fn from(value: embedded_graphics_core::primitives::Rectangle) -> Self {
        Frame {
            origin: value.top_left.into(),
            size: value.size.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pixel::Interpolate as _;

    use super::Point;

    #[test]
    fn interpolate_point() {
        let from = Point::new(10, 0);
        let to = Point::new(-10, 10000);
        assert_eq!(Point::interpolate(from, to, 0), from);
        assert_eq!(Point::interpolate(from, to, 255), to);
        assert_eq!(Point::interpolate(from, to, 128), Point::new(0, 5019)); // imperfect resolution
    }
}
