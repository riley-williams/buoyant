use core::cmp::max;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
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
}
