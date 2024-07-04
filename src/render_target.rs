#[cfg(feature = "crossterm")]
mod crossterm_render_target;
#[cfg(feature = "crossterm")]
pub use crossterm_render_target::CrosstermRenderTarget;

mod fixed_text_buffer;
pub use fixed_text_buffer::FixedTextBuffer;

use crate::{
    pixel::RenderUnit,
    primitives::{Frame, Point, Size},
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
    Pixel: RenderUnit,
{
    /// The size of the render target
    fn size(&self) -> Size;

    /// Clear the render target
    fn clear(&mut self);

    /// Draw a pixel to the render target
    fn draw(&mut self, point: Point, item: Pixel);

    /// Set the window frame. Draw commands will be drawn inside this frame
    fn set_window(&mut self, frame: Frame);

    /// Get the current window frame
    fn window(&self) -> Frame;

    /// Sets the origin of the window frame. The window size will not be changed.
    fn set_window_origin(&mut self, origin: Point) {
        let parent_frame = self.window();
        self.set_window(Frame {
            origin,
            size: parent_frame.size,
        });
    }
}
