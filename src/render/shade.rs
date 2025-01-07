use crate::primitives::Point;

pub trait style {
    type Color;
    fn shade(&self, point: Point) -> Self::Color;
}
