#[cfg(feature = "crossterm")]
mod crossterm_render_target;

// #[cfg(feature = "crossterm")]
// pub use crossterm_render_target::CrosstermRenderTarget;

mod fixed_text_buffer;
pub use fixed_text_buffer::CharColor;
pub use fixed_text_buffer::FixedTextBuffer;
