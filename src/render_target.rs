#[cfg(feature = "crossterm")]
mod crossterm_render_target;

#[cfg(feature = "crossterm")]
pub use crossterm_render_target::CrosstermRenderTarget;

mod fixed_text_buffer;
pub use fixed_text_buffer::FixedTextBuffer;

use crate::primitives::{Point, Size};

/// A target that can render character pixels.
///
/// A pixel could be a character, a color, or a more complex structure
/// such as a a character with a foreground and background color, like what
/// you might render to a terminal.
pub trait CharacterRenderTarget {
    type Color: Copy;
    /// The size of the render target
    fn size(&self) -> Size;

    /// Clear the render target
    fn clear(&mut self, color: Self::Color) {
        for y in 0..self.size().height {
            for x in 0..self.size().width {
                self.draw(Point::new(x as i16, y as i16), ' ', color);
            }
        }
    }

    /// Draw a pixel to the render target
    fn draw(&mut self, point: Point, character: char, color: Self::Color);
}
