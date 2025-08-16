use crate::primitives::{Interpolate, Point, Size};

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

    #[must_use]
    pub const fn x_end(&self) -> i32 {
        self.origin.x + self.size.width as i32
    }

    #[must_use]
    pub const fn y_end(&self) -> i32 {
        self.origin.y + self.size.height as i32
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
