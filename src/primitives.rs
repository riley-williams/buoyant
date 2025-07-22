mod dimension;
pub mod geometry;
mod interpolate;
mod seal;

pub use dimension::*;
pub use interpolate::Interpolate;
pub use seal::Seal;

use core::cmp::max;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Returns the smallest size that contains both sizes.
    #[must_use]
    pub fn union(&self, rhs: Self) -> Self {
        Self {
            width: max(self.width, rhs.width),
            height: max(self.height, rhs.height),
        }
    }

    /// Returns the overlapping area of the two sizes, which is the min of the two dimensions.
    #[must_use]
    pub fn intersection(&self, rhs: Self) -> Self {
        Self {
            width: self.width.min(rhs.width),
            height: self.height.min(rhs.height),
        }
    }

    #[must_use]
    pub const fn zero() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    /// Returns true if the point is non-negative and within the bounds of the size.
    #[must_use]
    pub const fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width as i32 && point.y < self.height as i32
    }

    #[inline]
    #[must_use]
    pub const fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl core::ops::Add for Size {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<embedded_graphics_core::geometry::Size> for Size {
    fn from(value: embedded_graphics_core::geometry::Size) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<Size> for embedded_graphics_core::geometry::Size {
    fn from(value: Size) -> Self {
        Self::new(value.width, value.height)
    }
}

impl Interpolate for Size {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        Self {
            width: Interpolate::interpolate(from.width, to.width, amount),
            height: Interpolate::interpolate(from.height, to.height, amount),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl core::ops::Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl core::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl core::ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl core::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl core::ops::Add<Size> for Point {
    type Output = Self;
    fn add(self, rhs: Size) -> Self {
        Self {
            x: self.x + rhs.width as i32,
            y: self.y + rhs.height as i32,
        }
    }
}

impl Point {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Interpolate for Point {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        Self {
            x: ((i32::from(amount) * to.x) + (i32::from(255 - amount) * from.x)) / 255,
            y: ((i32::from(amount) * to.y) + (i32::from(255 - amount) * from.y)) / 255,
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<Point> for embedded_graphics_core::geometry::Point {
    fn from(value: Point) -> Self {
        Self::new(value.x, value.y)
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<embedded_graphics_core::geometry::Point> for Point {
    fn from(value: embedded_graphics_core::geometry::Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Frame {
    pub size: Size,
    pub origin: Point,
}

impl Frame {
    #[must_use]
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { size, origin }
    }

    #[must_use]
    pub const fn contains(&self, point: &Point) -> bool {
        self.origin.x <= point.x
            && self.origin.y <= point.y
            && point.x < (self.origin.x + self.size.width as i32)
            && point.y < (self.origin.y + self.size.height as i32)
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<Frame> for embedded_graphics_core::primitives::Rectangle {
    fn from(value: Frame) -> Self {
        Self::new(value.origin.into(), value.size.into())
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<embedded_graphics_core::primitives::Rectangle> for Frame {
    fn from(value: embedded_graphics_core::primitives::Rectangle) -> Self {
        Self {
            origin: value.top_left.into(),
            size: value.size.into(),
        }
    }
}

impl Interpolate for Frame {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        Self {
            origin: Interpolate::interpolate(from.origin, to.origin, amount),
            size: Interpolate::interpolate(from.size, to.size, amount),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pixel<C> {
    pub color: C,
    pub point: Point,
}

#[cfg(feature = "embedded-graphics")]
impl<C: embedded_graphics::pixelcolor::PixelColor> From<embedded_graphics::Pixel<C>> for Pixel<C> {
    fn from(value: embedded_graphics::Pixel<C>) -> Self {
        Self {
            color: value.1,
            point: value.0.into(),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl<C: embedded_graphics::pixelcolor::PixelColor> From<Pixel<C>> for embedded_graphics::Pixel<C> {
    fn from(value: Pixel<C>) -> Self {
        Self(value.point.into(), value.color)
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::Interpolate as _;

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
