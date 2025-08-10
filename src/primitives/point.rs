use fixed::traits::ToFixed as _;

use crate::primitives::{
    transform::{CoordinateSpaceTransform, LinearTransform},
    Interpolate, Size,
};

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

impl core::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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

impl CoordinateSpaceTransform for Point {
    fn applying(&self, transform: &LinearTransform) -> Self {
        Self {
            x: (self.x * transform.scale.cast_signed()).to_num::<i32>() + transform.offset.x,
            y: (self.y * transform.scale.cast_signed()).to_num::<i32>() + transform.offset.y,
        }
    }

    fn applying_inverse(&self, transform: &LinearTransform) -> Self {
        let p = *self - transform.offset;
        Self {
            x: (p.x.to_fixed::<fixed::types::I18F14>() / transform.scale.cast_signed())
                .to_num::<i32>(),
            y: (p.y.to_fixed::<fixed::types::I18F14>() / transform.scale.cast_signed())
                .to_num::<i32>(),
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
