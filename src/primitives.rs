use core::cmp::max;

// use 32 bit integers if the half_precision feature is disabled
// otherwise use 16 bit integers

#[allow(non_camel_case_types)]
#[cfg(not(feature = "reduced_precision"))]
pub type uint = u32;

#[allow(non_camel_case_types)]
#[cfg(not(feature = "reduced_precision"))]
pub type iint = i32;

#[allow(non_camel_case_types)]
#[cfg(feature = "reduced_precision")]
pub type uint = u16;

#[allow(non_camel_case_types)]
#[cfg(feature = "reduced_precision")]
pub type iint = i16;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Size {
    pub width: uint,
    pub height: uint,
}

impl Size {
    pub fn new(width: uint, height: uint) -> Self {
        Size { width, height }
    }

    pub fn union(&self, rhs: Size) -> Size {
        Size {
            width: max(self.width, rhs.width),
            height: max(self.height, rhs.height),
        }
    }
}

// implement addition operator for Size
impl core::ops::Add for Size {
    type Output = Size;
    fn add(self, rhs: Size) -> Size {
        Size {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: iint,
    pub y: iint,
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
    pub fn new(x: iint, y: iint) -> Self {
        Point { x, y }
    }
}
