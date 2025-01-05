use crate::{
    primitives::{Point, Size},
    render::shade::Shader,
};

#[cfg(feature = "crossterm")]
mod crossterm_render_target;

#[cfg(feature = "crossterm")]
pub use crossterm_render_target::CrosstermRenderTarget;

mod fixed_text_buffer;
pub use fixed_text_buffer::FixedTextBuffer;
pub use fixed_text_buffer::TxtColor;

pub trait RenderTarget {
    type Color;

    fn size(&self) -> Size;
    fn draw(&mut self, point: Point, color: Self::Color);
    fn draw_text(&mut self, text: &str, position: Point, shader: &impl Shader<Color = Self::Color>);

    fn draw_rect(
        &mut self,
        position: Point,
        size: Size,
        shader: &impl Shader<Color = Self::Color>,
    ) {
        for x in 0..size.width as i16 {
            for y in 0..size.height as i16 {
                self.draw(position + Point::new(x, y), shader.shade(Point::new(x, y)));
            }
        }
    }
}
