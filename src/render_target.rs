#[cfg(feature = "crossterm")]
mod crossterm_render_target;
mod embedded_graphics_target;
pub use embedded_graphics_target::EmbeddedGraphicsRenderTarget;

use core::marker::PhantomData;

#[cfg(feature = "crossterm")]
pub use crossterm_render_target::CrosstermRenderTarget;

mod fixed_text_buffer;
pub use fixed_text_buffer::FixedTextBuffer;
pub mod geometry;

use geometry::Point;

/// The element of a Bézier path.
///
/// A valid path has `MoveTo` at the beginning of each subpath.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathEl {
    /// Move directly to the point without drawing anything, starting a new
    /// subpath.
    MoveTo(Point),
    /// Draw a line from the current location to the point.
    LineTo(Point),
    /// Draw a quadratic bezier using the current location and the two points.
    QuadTo(Point, Point),
    /// Draw a cubic bezier using the current location and the three points.
    CurveTo(Point, Point, Point),
    /// Close off the path.
    ClosePath,
}

pub trait Shape {
    type PathElementsIter<'iter>: Iterator<Item = PathEl> + 'iter
    where
        Self: 'iter;

    fn path_elements(&self, tolerance: u16) -> Self::PathElementsIter<'_>;

    /// The smallest rectangle that encloses the shape.
    fn bounding_box(&self) -> geometry::Rectangle;

    /// If the shape is a line, make it available.
    fn as_line(&self) -> Option<geometry::Line> {
        None
    }

    /// If the shape is a rectangle, make it available.
    fn as_rect(&self) -> Option<geometry::Rectangle> {
        None
    }

    /// If the shape is a rounded rectangle, make it available.
    fn as_rounded_rect(&self) -> Option<geometry::RoundedRectangle> {
        None
    }

    /// If the shape is a circle, make it available.
    fn as_circle(&self) -> Option<geometry::Circle> {
        None
    }
}

pub trait RenderTarget {
    type Layer;
    type ColorFormat;

    /// clears the target
    fn reset(&mut self);

    fn push_layer(&mut self) -> Self::Layer;

    /// Pops the current layer.
    fn pop_layer(&mut self, layer: Self::Layer);

    /// Fills a shape using the specified style and brush.
    fn fill(
        &mut self,
        transform_offset: Point,
        brush: Brush<'_, impl Into<Self::ColorFormat>>,
        brush_offset: Option<Point>,
        shape: &impl Shape,
    );

    /// Strokes a shape using the specified style and brush.
    fn stroke(
        &mut self,
        stroke: &Stroke,
        transform_offset: Point,
        brush: Brush<'_, impl Into<Self::ColorFormat>>,
        brush_offset: Option<Point>,
        shape: &impl Shape,
    );

    // /// Draws an image at its natural size with the given transform.
    // fn draw_image(&mut self, image: &Image, transform: Affine) { }

    fn draw_glyphs(
        &mut self,
        offset: Point,
        brush: Brush<'_, impl Into<Self::ColorFormat>>,
        text: &str,
    );
}

pub trait DrawGlyphs<'a> {
    type ColorFormat;
    fn draw(&mut self, style: Brush<'a, Self::ColorFormat>, glyphs: impl Iterator<Item = char>);
}

/// Positioned glyph.
#[derive(Copy, Clone, Default, Debug)]
pub struct Glyph {
    /// Glyph identifier.
    pub id: u32,
    /// X-offset in run, relative to transform.
    pub x: i32,
    /// Y-offset in run, relative to transform.
    pub y: i32,
}

#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub struct Image<'a, ColorFormat> {
    /// Blob containing the image data.
    pub data: &'a [u8],
    /// Pixel format of the image.
    pub format: PhantomData<ColorFormat>,
    /// Width of the image.
    pub width: u32,
    /// Height of the image.
    pub height: u32,
}

impl<'a, ColorFormat> Image<'a, ColorFormat> {
    #[must_use]
    pub const fn new(data: &'a [u8], width: u32, height: u32) -> Self {
        Self {
            data,
            format: PhantomData,
            width,
            height,
        }
    }
}

/// Describes the color content of a filled or stroked shape.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Brush<'a, ColorFormat> {
    /// Solid color brush.
    Solid(ColorFormat),
    // /// Gradient brush.
    // Gradient(Gradient<ColorFormat>),
    /// Image brush.
    Image(Image<'a, ColorFormat>),
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
