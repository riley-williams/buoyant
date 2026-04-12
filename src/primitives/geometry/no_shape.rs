use crate::primitives::{
    Point, Size,
    transform::{CoordinateSpaceTransform, LinearTransform},
};

use super::{Rectangle, Shape, ShapePathIter};

/// A zero-sized type representing the absence of a shape.
///
/// This is useful for `ContentShape` implementations that have no intrinsic shape,
/// avoiding the overhead of `Option<WastedBytesHere>`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NoShape;

impl CoordinateSpaceTransform for NoShape {
    fn applying(&self, _transform: &LinearTransform) -> Self {
        Self
    }

    fn applying_inverse(&self, _transform: &LinearTransform) -> Self {
        Self
    }
}

impl Shape for NoShape {
    type PathElementsIter<'iter> = ShapePathIter<0>;

    fn path_elements(&self, _tolerance: u16) -> Self::PathElementsIter<'_> {
        ShapePathIter::new([])
    }

    fn bounding_box(&self) -> Rectangle {
        // This is sort of meaningless...
        Rectangle::new(Point::new(0, 0), Size::new(0, 0))
    }
}
