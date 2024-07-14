#[cfg(feature = "crossterm")]
mod crossterm_render_target;
#[cfg(feature = "crossterm")]
pub use crossterm_render_target::CrosstermRenderTarget;

#[cfg(feature = "embedded-graphics")]
pub mod embedded_display_render_target;

mod fixed_text_buffer;
pub use fixed_text_buffer::FixedTextBuffer;

use crate::{
    pixel::ColorValue,
    primitives::{Point, Size},
};

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
pub trait RenderTarget<Pixel>
where
    Pixel: ColorValue,
{
    /// The size of the render target
    fn size(&self) -> Size;

    /// Clear the render target
    fn clear(&mut self);

    /// Draw a pixel to the render target
    fn draw(&mut self, point: Point, item: Pixel);
}
