mod crossterm_target;
pub use crossterm_target::CrosstermRenderTarget;

mod fixed_text_buffer;
pub use fixed_text_buffer::FixedTextBuffer;

use crate::primitives::{Point, Size};

/// A target that accepts pixels
pub trait RenderTarget<C> {
    /// The size of the render target
    fn size(&self) -> Size;

    /// Clear the render target
    fn clear(&mut self);

    /// Draw a pixel to the render target
    fn draw(&mut self, point: Point, item: C);
}
