use core::cmp::max;

use fixed::traits::ToFixed as _;

use crate::primitives::{
    Interpolate, Point,
    transform::{CoordinateSpaceTransform, ScaleFactor},
};

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

impl<T: Into<u32>> From<(T, T)> for Size {
    fn from(value: (T, T)) -> Self {
        Self {
            width: value.0.into(),
            height: value.1.into(),
        }
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

impl CoordinateSpaceTransform for Size {
    fn applying(&self, transform: &crate::primitives::transform::LinearTransform) -> Self {
        Self {
            width: (self.width * transform.scale).to_num(),
            height: (self.height * transform.scale).to_num(),
        }
    }

    fn applying_inverse(&self, transform: &crate::primitives::transform::LinearTransform) -> Self {
        Self {
            width: (self.width.to_fixed::<ScaleFactor>() / transform.scale).to_num(),
            height: (self.height.to_fixed::<ScaleFactor>() / transform.scale).to_num(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::transform::LinearTransform;

    #[test]
    fn zero() {
        let size = Size::zero();
        assert_eq!(size, Size::new(0, 0));
    }

    #[test]
    fn area() {
        let size = Size::new(10, 20);
        assert_eq!(size.area(), 200);

        let zero_size = Size::zero();
        assert_eq!(zero_size.area(), 0);
    }

    #[test]
    fn union() {
        let size1 = Size::new(10, 30);
        let size2 = Size::new(20, 15);
        let union = size1.union(size2);
        assert_eq!(union, Size::new(20, 30));
    }

    #[test]
    fn intersection() {
        let size1 = Size::new(10, 30);
        let size2 = Size::new(20, 15);
        let intersection = size1.intersection(size2);
        assert_eq!(intersection, Size::new(10, 15));
    }

    #[test]
    fn contains() {
        let size = Size::new(100, 200);

        // Points inside
        assert!(size.contains(Point::new(0, 0)));
        assert!(size.contains(Point::new(50, 100)));
        assert!(size.contains(Point::new(99, 199)));

        // Points outside
        assert!(!size.contains(Point::new(100, 50)));
        assert!(!size.contains(Point::new(50, 200)));
        assert!(!size.contains(Point::new(-1, 50)));
        assert!(!size.contains(Point::new(50, -1)));
        assert!(!size.contains(Point::new(100, 200)));
    }

    #[test]
    fn add() {
        let size1 = Size::new(10, 20);
        let size2 = Size::new(30, 40);
        let result = size1 + size2;
        assert_eq!(result, Size::new(40, 60));
    }

    #[test]
    fn interpolate() {
        let from = Size::new(10, 20);
        let to = Size::new(30, 60);

        let mid = Size::interpolate(from, to, 128); // 50%
        assert_eq!(mid, Size::new(20, 40));

        let quarter = Size::interpolate(from, to, 64); // 25%
        assert_eq!(quarter, Size::new(15, 30));
    }

    #[test]
    fn applying_transform() {
        let size = Size::new(40, 60);
        let transform = LinearTransform::new(Point::new(5, 10), 2.0);

        let transformed = size.applying(&transform);
        assert_eq!(transformed, Size::new(80, 120));
    }

    #[test]
    fn applying_inverse_transform() {
        let size = Size::new(80, 120);
        let transform = LinearTransform::new(Point::new(5, 10), 2.0);

        let inverse_transformed = size.applying_inverse(&transform);
        assert_eq!(inverse_transformed, Size::new(40, 60));
    }

    #[test]
    fn transform_roundtrip() {
        let original = Size::new(32, 48);
        let transform = LinearTransform::new(Point::new(4, 8), 2.0);

        let transformed = original.applying(&transform).applying_inverse(&transform);
        assert_eq!(transformed, original);
    }

    #[test]
    fn identity_transform() {
        let size = Size::new(100, 150);
        let identity = LinearTransform::identity();

        let transformed = size.applying(&identity);

        assert_eq!(transformed, size);
    }
}
