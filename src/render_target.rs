mod crossterm_target;
pub use crossterm_target::CrosstermRenderTarget;

mod fixed_text_buffer;
pub use fixed_text_buffer::FixedTextBuffer;

use crate::primitives::{Point, Size};

/// A target that can render pixels.
///
/// A pixel could be a character, a color, or a more complex structure
/// such as a a character with a foreground and background color, like what
/// you might render to a terminal.
///
/// The built-in views that primarily perform layout (stacks, padding, etc.)
/// do not render pixels, so their default Render impl is sufficient.
/// For other types such as Text, Divider, etc., you will need to implement
/// Render for your target Pixel type.
pub trait RenderTarget<Pixel> {
    /// The size of the render target
    fn size(&self) -> Size;

    /// Clear the render target
    fn clear(&mut self);

    /// Draw a pixel to the render target
    fn draw(&mut self, point: Point, item: Pixel);
}

#[derive(Debug, PartialEq)]
pub struct Proxy<'a, T> {
    target: &'a mut T,
    pub origin: Point,
}

impl<'a, T> Proxy<'a, T> {
    pub fn new(target: &'a mut T, origin: Point) -> Self {
        Proxy { target, origin }
    }
}

impl<'a, T: RenderTarget<I>, I> RenderTarget<I> for Proxy<'a, T> {
    fn size(&self) -> Size {
        self.target.size()
    }
    fn clear(&mut self) {
        self.target.clear()
    }

    fn draw(&mut self, point: Point, item: I) {
        self.target.draw(point + self.origin, item)
    }
}

pub struct ClippingProxy<'a, T> {
    target: &'a mut T,
    pub origin: Point,
    pub size: Size,
}

impl<'a, T> ClippingProxy<'a, T> {
    pub fn new(target: &'a mut T, origin: Point, size: Size) -> Self {
        Self {
            target,
            origin,
            size,
        }
    }
}

impl<'a, T: RenderTarget<I>, I> RenderTarget<I> for ClippingProxy<'a, T> {
    fn size(&self) -> Size {
        self.target.size()
    }
    fn clear(&mut self) {
        self.target.clear()
    }

    fn draw(&mut self, point: Point, item: I) {
        if self.size.contains(point) {
            self.target.draw(point + self.origin, item)
        }
    }
}
