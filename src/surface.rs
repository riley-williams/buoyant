use crate::primitives::{geometry::Rectangle, Pixel, Point, Size};

/// This trait is a less restrictive version of `embedded_graphics::DrawTarget`.
///
/// It is mostly used to enable support for existing crates that require
/// `DrawTarget`. The surface may be a temporary buffer (glyph cache) or in
/// some cases may render directly to a display device.
pub trait Surface {
    type Color: Copy;

    fn size(&self) -> Size;

    fn draw_iter<I>(&mut self, pixels: I)
    where
        I: IntoIterator<Item = Pixel<Self::Color>>;

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I)
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let x_end = area.origin.x + area.size.width as i32;
        let y_end = area.origin.y + area.size.height as i32;
        let points = (area.origin.y..y_end)
            .flat_map(move |y| (area.origin.x..x_end).map(move |x| Point::new(x, y)));
        self.draw_iter(
            points
                .zip(colors)
                .map(|(point, color)| Pixel { color, point }),
        );
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) {
        self.fill_contiguous(area, core::iter::repeat(color));
    }
}

/// A surface which draws with a specified offset.
#[derive(Debug)]
pub struct OffsetSurface<'a, S> {
    surface: &'a mut S,
    offset: Point,
}

impl<'a, S: Surface> OffsetSurface<'a, S> {
    pub fn new(surface: &'a mut S, offset: Point) -> Self {
        Self { surface, offset }
    }
}

impl<S: Surface> Surface for OffsetSurface<'_, S> {
    type Color = S::Color;

    fn size(&self) -> Size {
        // TODO: Is this really the correct / expected behavior?
        let mut size = self.surface.size();
        size.width -= self.offset.x as u32;
        size.height -= self.offset.y as u32;
        size
    }

    fn draw_iter<I>(&mut self, pixels: I)
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.surface.draw_iter(pixels.into_iter().map(|mut p| {
            p.point += self.offset;
            p
        }));
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I)
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let origin = area.origin + self.offset;
        let area = Rectangle::new(origin, area.size);
        self.surface.fill_contiguous(&area, colors);
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) {
        let origin = area.origin + self.offset;
        let area = Rectangle::new(origin, area.size);
        self.surface.fill_solid(&area, color);
    }
}

/// A surface which can be used as an embedded-graphics draw target
#[cfg(feature = "embedded-graphics")]
#[derive(Debug)]
pub struct EmbeddedGraphicsSurface<T>(pub T);

#[cfg(feature = "embedded-graphics")]
pub trait AsDrawTarget: Surface {
    /// Produces a surface conforming to `embedded_graphics::DrawTarget`
    fn draw_target(
        &mut self,
    ) -> impl embedded_graphics::prelude::DrawTarget<Color = Self::Color, Error = ()> + '_;
}

#[cfg(feature = "embedded-graphics")]
impl<T: Surface<Color = P>, P: embedded_graphics::prelude::PixelColor> AsDrawTarget for T {
    fn draw_target(
        &mut self,
    ) -> impl embedded_graphics::draw_target::DrawTarget<Color = <Self as Surface>::Color, Error = ()> + '_
    {
        EmbeddedGraphicsSurface(self)
    }
}

#[cfg(feature = "embedded-graphics")]
impl<T: Surface> embedded_graphics::prelude::Dimensions for EmbeddedGraphicsSurface<&mut T> {
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        embedded_graphics::primitives::Rectangle::new(
            embedded_graphics::prelude::Point::zero(),
            self.0.size().into(),
        )
    }
}

#[cfg(feature = "embedded-graphics")]
impl<T: Surface> embedded_graphics::prelude::DrawTarget for EmbeddedGraphicsSurface<&mut T>
where
    T::Color: embedded_graphics::prelude::PixelColor,
{
    type Color = T::Color;

    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        self.0.draw_iter(pixels.into_iter().map(|p| Pixel {
            point: p.0.into(),
            color: p.1,
        }));
        Ok(())
    }

    fn fill_contiguous<I>(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        colors: I,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.0.draw_iter(
            embedded_graphics::prelude::PointsIter::points(area)
                .zip(colors)
                .map(|(pos, color)| Pixel {
                    point: pos.into(),
                    color,
                }),
        );
        Ok(())
    }

    fn fill_solid(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        color: Self::Color,
    ) -> Result<(), Self::Error> {
        let area: Rectangle = (*area).into();
        self.0.fill_contiguous(&area, core::iter::repeat(color));
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        let bounding_box = Rectangle::new(Point::zero(), self.0.size());
        self.0.fill_solid(&bounding_box, color);
        Ok(())
    }
}
