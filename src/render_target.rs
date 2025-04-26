#[cfg(feature = "crossterm")]
mod crossterm;

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics;
#[cfg(feature = "embedded-graphics")]
pub use embedded_graphics::EmbeddedGraphicsRenderTarget;

#[cfg(feature = "crossterm")]
pub use crossterm::CrosstermRenderTarget;

mod fixed_text_buffer;
pub use fixed_text_buffer::FixedTextBuffer;

use crate::{
    font::{self, FontMetrics as _},
    image::EmptyImage,
    primitives::{geometry::Shape, Point, Size},
    surface::Surface,
};

pub trait RenderTarget {
    type ColorFormat;

    /// The drawable size of the target
    fn size(&self) -> Size;

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
        font: &impl font::FontRender<Self::ColorFormat>,
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
        font: &impl font::FontRender<Self::ColorFormat>,
    ) {
        let metrics = font.metrics();
        let mut x = 0;
        self.draw_glyphs(
            offset,
            brush,
            text.chars().map(|c| {
                let glyph = Glyph {
                    character: c,
                    offset: Point::new(x, 0),
                };
                x += metrics.advance(glyph.character) as i32;
                glyph
            }),
            font,
        );
    }

    /// Obtain a raw surface to directly write pixels.
    ///
    /// This is most often useful for bridging `embedded_graphics` types
    /// that are designed to render to a `DrawTarget`.
    ///
    /// ```
    /// # use buoyant::primitives::Size;
    /// # use buoyant::render_target::RenderTarget;
    /// # use buoyant::render_target::EmbeddedGraphicsRenderTarget;
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::pixelcolor::Rgb888;
    /// # use embedded_graphics::mock_display::MockDisplay;
    /// use tinytga::Tga;
    /// use crate::buoyant::surface::AsDrawTarget;
    ///
    /// # let mut display = MockDisplay::<Rgb888>::new();
    /// # let mut target = EmbeddedGraphicsRenderTarget::new(display);
    /// // let mut target = EmbeddedGraphicsRenderTarget::new(...);
    /// # let data = include_bytes!("../tests/assets/rhombic-dodecahedron.tga");
    ///
    /// let img: Tga<Rgb888> = Tga::from_slice(data).unwrap();
    ///
    /// img.draw(&mut target.raw_surface().draw_target());
    /// ```
    fn raw_surface(&mut self) -> &mut impl Surface<Color = Self::ColorFormat>;
}

/// Positioned glyph.
#[derive(Copy, Clone, Default, Debug)]
pub struct Glyph {
    /// The character represented by the glyph.
    pub character: char,
    /// Offset in run, relative to transform.
    pub offset: Point,
}

/// Describes the color content of a filled or stroked shape.
pub trait Brush {
    type ColorFormat;

    /// Computes the color at a specific point
    fn color_at(&self, point: Point) -> Option<Self::ColorFormat>;

    /// Solid color brush.
    fn as_solid(&self) -> Option<Self::ColorFormat>;

    /// Image brush.
    fn as_image(&self) -> Option<&impl ImageBrush<ColorFormat = Self::ColorFormat>>;
}

pub trait ImageBrush: Brush {
    /// Dimensions of the image.
    fn size(&self) -> Size;
    /// Iterator over the contiguous pixels of the image.
    fn color_iter(&self) -> impl Iterator<Item = Self::ColorFormat>;
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
