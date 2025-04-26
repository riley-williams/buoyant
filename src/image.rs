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

#[cfg(feature = "embedded-graphics")]
mod tga {
    use embedded_graphics::{
        pixelcolor::{Gray8, Rgb555, Rgb888},
        prelude::PixelColor,
    };
    use tinytga::Tga;

    use super::{Brush, ImageBrush};

    impl<C> Brush for Tga<'_, C>
    where
        C: PixelColor + From<Rgb888> + From<Rgb555> + From<Gray8>,
    {
        type ColorFormat = C;

        fn color_at(&self, _point: crate::primitives::Point) -> Option<Self::ColorFormat> {
            // FIXME: I don't see any Tga API to do this...
            None
        }

        fn as_solid(&self) -> Option<Self::ColorFormat> {
            None
        }

        fn as_image(&self) -> Option<&'_ impl super::ImageBrush<ColorFormat = Self::ColorFormat>> {
            Some(self)
        }
    }

    impl<C> ImageBrush for Tga<'_, C>
    where
        C: PixelColor + From<Rgb888> + From<Rgb555> + From<Gray8>,
    {
        fn size(&self) -> crate::primitives::Size {
            self.as_raw().size().into()
        }

        fn color_iter(&self) -> impl Iterator<Item = Self::ColorFormat> {
            self.pixels().map(|p| p.1)
        }
    }
}
