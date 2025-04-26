use core::marker::PhantomData;

use crate::{
    primitives::{Point, Size},
    render_target::{Brush, ImageBrush},
};

/// A zero-sized image
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyImage<C> {
    _marker: PhantomData<C>,
}

impl<C> Brush for EmptyImage<C> {
    type ColorFormat = C;

    fn color_at(&self, _point: Point) -> Option<Self::ColorFormat> {
        None
    }

    fn as_solid(&self) -> Option<Self::ColorFormat> {
        None
    }

    fn as_image(&self) -> Option<&impl ImageBrush<ColorFormat = Self::ColorFormat>> {
        Some(self)
    }
}

impl<C> ImageBrush for EmptyImage<C> {
    fn size(&self) -> Size {
        Size::zero()
    }

    fn color_iter(&self) -> impl Iterator<Item = Self::ColorFormat> {
        core::iter::empty()
    }
}
