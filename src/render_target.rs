#[cfg(feature = "crossterm")]
mod crossterm_render_target;

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_target;
#[cfg(feature = "embedded-graphics")]
pub use embedded_graphics_target::EmbeddedGraphicsRenderTarget;

use core::marker::PhantomData;

#[cfg(feature = "crossterm")]
pub use crossterm_render_target::CrosstermRenderTarget;

mod fixed_text_buffer;
pub use fixed_text_buffer::FixedTextBuffer;

use crate::{
    font::{self, FontMetrics as _, GlyphIndex},
    primitives::{geometry::Shape, Point, Size},
};

pub trait RenderTarget {
    type ColorFormat;

    /// Clears the target using the provided color
    fn clear(&mut self, color: Self::ColorFormat);

    /// Fills a shape using the specified style and brush.
    fn fill<C: Into<Self::ColorFormat>>(
        &mut self,
        transform_offset: Point,
        brush: &impl Brush<ColorFormat = C>,
        brush_offset: Option<Point>,
        shape: &impl Shape,
    );

    /// Strokes a shape using the specified style and brush.
    fn stroke<C: Into<Self::ColorFormat>>(
        &mut self,
        stroke: &Stroke,
        transform_offset: Point,
        brush: &impl Brush<ColorFormat = C>,
        brush_offset: Option<Point>,
        shape: &impl Shape,
    );

    /// Draws a series of glyphs using the specified style and brush.
    fn draw_glyphs<C: Into<Self::ColorFormat>>(
        &mut self,
        offset: Point,
        brush: &impl Brush<ColorFormat = C>,
        glyphs: impl Iterator<Item = Glyph>,
        font: &impl font::FontRender,
    );

    /// Draws a string using the specified style and brush.
    ///
    /// This performs the same operation as `draw_glyphs`, but also handles
    /// glyph indexing and positioning.
    fn draw_str<C: Into<Self::ColorFormat>>(
        &mut self,
        offset: Point,
        brush: &impl Brush<ColorFormat = C>,
        text: &str,
        font: &impl font::FontRender,
    ) {
        let metrics = font.metrics();
        let mut x = 0;
        self.draw_glyphs(
            offset,
            brush,
            text.chars().map(|c| {
                let index = font.glyph_index(c);
                let glyph = Glyph {
                    id: index,
                    character: c,
                    x: x.into(),
                    y: 0,
                };
                x += metrics.advance(glyph.id) as i16;
                glyph
            }),
            font,
        );
    }
}

pub trait DrawGlyphs<'a> {
    type ColorFormat;
    fn draw(
        &mut self,
        style: &impl Brush<ColorFormat = Self::ColorFormat>,
        glyphs: impl Iterator<Item = char>,
    );
}

/// Positioned glyph.
#[derive(Copy, Clone, Default, Debug)]
pub struct Glyph {
    /// Glyph identifier.
    pub id: GlyphIndex,
    /// The character represented by the glyph.
    pub character: char,
    /// X-offset in run, relative to transform.
    pub x: i32,
    /// Y-offset in run, relative to transform.
    pub y: i32,
}

pub trait ImageBrush: Brush {
    /// Blob containing the image data.
    fn data(&self) -> &[u8];
    /// Dimensions of the image.
    fn size(&self) -> Size;
    /// Iterator over the pixels of the image.
    fn pixel_iter(&self) -> impl Iterator<Item = Self::ColorFormat>;
}

/// Describes the color content of a filled or stroked shape.
pub trait Brush {
    type ColorFormat;

    fn color_at(&self, point: Point) -> Option<Self::ColorFormat>;

    /// Solid color brush.
    fn as_solid(&self) -> Option<Self::ColorFormat>;

    /// Image brush.
    fn as_image(&self) -> Option<&impl ImageBrush<ColorFormat = Self::ColorFormat>>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolidBrush<C> {
    color: C,
}

impl<C: Copy> SolidBrush<C> {
    #[must_use]
    pub const fn new(color: C) -> Self {
        Self { color }
    }
}

impl<C: Copy> Brush for SolidBrush<C> {
    type ColorFormat = C;

    fn color_at(&self, _point: Point) -> Option<Self::ColorFormat> {
        Some(self.color)
    }

    fn as_solid(&self) -> Option<Self::ColorFormat> {
        Some(self.color)
    }

    fn as_image(&self) -> Option<&impl ImageBrush<ColorFormat = Self::ColorFormat>> {
        Option::<&EmptyImage<Self::ColorFormat>>::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EmptyImage<C> {
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
    fn data(&self) -> &[u8] {
        &[]
    }

    fn size(&self) -> Size {
        Size::zero()
    }

    fn pixel_iter(&self) -> impl Iterator<Item = Self::ColorFormat> {
        core::iter::empty()
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Stroke {
    /// Width of the stroke.
    pub width: u32,
}

impl Stroke {
    #[must_use]
    pub const fn new(width: u32) -> Self {
        Self { width }
    }
}

// TODO: Move me
// #[cfg(feature = "embedded_graphics")]
mod image_impl {
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
        fn data(&self) -> &[u8] {
            self.as_raw().image_data()
        }

        fn size(&self) -> crate::primitives::Size {
            self.as_raw().size().into()
        }

        fn pixel_iter(&self) -> impl Iterator<Item = Self::ColorFormat> {
            self.pixels().map(|p| p.1)
        }
    }
}
