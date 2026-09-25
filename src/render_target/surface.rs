use crate::primitives::{
    Pixel, Point, Size,
    geometry::{Intersection, Rectangle},
};

mod offset_surface;
pub use offset_surface::OffsetSurface;

mod clipped_surface;
pub use clipped_surface::ClippedSurface;

mod bounded_surface;
pub use bounded_surface::BoundedSurface;

/// How a rectangle relates to the region a surface permits drawing in.
///
/// Obtained from [`Surface::visibility_of`]. The rectangles involved are all
/// in the coordinate space of the surface that produced the [`Visibility`],
/// which is the same space as the points passed to [`Surface::draw_iter`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    /// The rectangle lies entirely within the permitted region and may be
    /// drawn without clipping.
    Full,
    /// The rectangle is partially permitted. Drawing must be confined to this
    /// sub-rectangle.
    Clipped(Rectangle),
    /// The rectangle lies entirely outside the permitted region. Nothing
    /// should be drawn.
    Hidden,
}

impl Visibility {
    /// Classifies `rect` against the region `bounds`, both in the same
    /// coordinate space.
    #[must_use]
    pub fn classify(bounds: &Rectangle, rect: &Rectangle) -> Self {
        match bounds.intersection_with(rect) {
            Intersection::Contains => Self::Full,
            Intersection::Overlaps => bounds
                .intersection(rect)
                .map_or(Self::Hidden, Self::Clipped),
            Intersection::NonIntersecting => Self::Hidden,
        }
    }
}

#[cfg(feature = "embedded-graphics")]
mod draw_target_surface;
#[cfg(feature = "embedded-graphics")]
pub use draw_target_surface::DrawTargetSurface;

/// This trait is a less restrictive version of `embedded_graphics::DrawTarget`.
///
/// It is mostly used to enable support for existing crates that require
/// `DrawTarget`. The surface may be a temporary buffer (glyph cache) or in
/// some cases may render directly to a display device.
pub trait Surface {
    type Color: Copy;

    /// The region this surface permits drawing in.
    ///
    /// The origin is not necessarily zero.
    fn bounding_box(&self) -> Rectangle;

    /// The size of the region this surface permits drawing in.
    fn size(&self) -> Size {
        self.bounding_box().size
    }

    /// Classifies `rect` against the region this surface permits drawing in.
    ///
    /// Surfaces which do not enforce their permitted region rely on callers
    /// consulting this before drawing.
    fn visibility_of(&self, rect: &Rectangle) -> Visibility {
        Visibility::classify(&self.bounding_box(), rect)
    }

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

    fn clear(&mut self, color: Self::Color) {
        self.fill_solid(&self.bounding_box(), color);
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
        self.0.bounding_box().into()
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
        self.0.fill_contiguous(&(*area).into(), colors);
        Ok(())
    }

    fn fill_solid(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        color: Self::Color,
    ) -> Result<(), Self::Error> {
        let area: Rectangle = (*area).into();
        self.0.fill_solid(&area, color);
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.0.clear(color);
        Ok(())
    }
}

impl<T: Surface> Surface for &mut T {
    type Color = T::Color;

    fn bounding_box(&self) -> Rectangle {
        (**self).bounding_box()
    }

    fn size(&self) -> Size {
        (**self).size()
    }

    fn visibility_of(&self, rect: &Rectangle) -> Visibility {
        (**self).visibility_of(rect)
    }

    fn draw_iter<I>(&mut self, pixels: I)
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        (**self).draw_iter(pixels);
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I)
    where
        I: IntoIterator<Item = Self::Color>,
    {
        (**self).fill_contiguous(area, colors);
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) {
        (**self).fill_solid(area, color);
    }

    fn clear(&mut self, color: Self::Color) {
        (**self).clear(color);
    }
}
