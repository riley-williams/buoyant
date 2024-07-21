#[cfg(feature = "crossterm")]
mod crossterm_render_target;
#[cfg(feature = "crossterm")]
pub use crossterm_render_target::CrosstermRenderTarget;

mod fixed_text_buffer;
pub use fixed_text_buffer::FixedTextBuffer;

use crate::{
    pixel::PixelColor,
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
pub trait RenderTarget<Color>
where
    Color: PixelColor,
{
    /// The size of the render target
    fn size(&self) -> Size;

    /// Clear the render target
    fn clear(&mut self, color: Color) {
        self.draw_solid(&Frame::new(Point::zero(), self.size()), color);
    }

    /// Draw a pixel to the render target
    fn draw(&mut self, point: Point, color: Color);

    fn draw_iter<I>(&mut self, pixels: I)
    where
        I: IntoIterator<Item = crate::pixel::Pixel<Color>>,
    {
        for pixel in pixels {
            self.draw(pixel.point, pixel.color);
        }
    }

    /// Fills a rectangle from left-right, top-bottom using the iterator
    fn draw_contiguous<I>(&mut self, area: &Frame, colors: I)
    where
        I: IntoIterator<Item = Color>,
    {
        let mut point = area.origin;
        for color in colors {
            self.draw(point, color);
            point.x += 1;
            if point.x >= area.origin.x + area.size.width as i16 {
                point.x = area.origin.x;
                point.y += 1;
            }
        }
    }

    /// Draws a solid rectangle
    fn draw_solid(&mut self, area: &Frame, color: Color) {
        for y in 0..area.size.height {
            for x in 0..area.size.width {
                self.draw(area.origin + Point::new(x as i16, y as i16), color);
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics_core::{draw_target::DrawTarget, primitives::Rectangle};

#[cfg(feature = "embedded-graphics")]
impl<D, C> RenderTarget<C> for D
where
    D: DrawTarget<Color = C>,
    C: PixelColor + embedded_graphics_core::pixelcolor::PixelColor,
{
    fn size(&self) -> Size {
        self.bounding_box().size.into()
    }

    fn clear(&mut self, color: C) {
        _ = self.clear(color);
    }

    fn draw(&mut self, point: Point, color: C) {
        _ = self.draw_iter(std::iter::once(crate::pixel::Pixel { point, color }.into()));
    }

    fn draw_iter<I>(&mut self, pixels: I)
    where
        I: IntoIterator<Item = crate::pixel::Pixel<C>>,
    {
        _ = self.draw_iter(pixels.into_iter().map(Into::into));
    }

    fn draw_contiguous<I>(&mut self, area: &Frame, colors: I)
    where
        I: IntoIterator<Item = C>,
    {
        let rect = Rectangle::from(*area);
        _ = self.fill_contiguous(&rect, colors);
    }

    fn draw_solid(&mut self, area: &Frame, color: C) {
        let rect = Rectangle::from(*area);
        _ = self.fill_solid(&rect, color);
    }
}
