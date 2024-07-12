use embedded_graphics::{draw_target::DrawTarget, primitives::Rectangle};

use crate::{
    pixel::ColorValue,
    primitives::{Frame, Point, Size},
};

use super::RenderTarget;

pub struct EmbeddedDisplayRenderTarget<D> {
    display: D,
    window: Frame,
}

impl<D, Pixel> RenderTarget<Pixel> for EmbeddedDisplayRenderTarget<D>
where
    D: DrawTarget<Color = Pixel>,
    Pixel: ColorValue,
{
    fn size(&self) -> Size {
        self.window.size
    }

    fn clear(&mut self) {
        todo!()
    }

    fn draw(&mut self, point: Point, item: Pixel) {
        _ = self.display.fill_solid(
            &Rectangle::new(
                embedded_graphics::geometry::Point {
                    x: point.x as i32,
                    y: point.y as i32,
                },
                embedded_graphics::geometry::Size {
                    width: 1,
                    height: 1,
                },
            ),
            item,
        );
    }

    fn set_window(&mut self, frame: Frame) {
        self.window = frame;
    }

    fn window(&self) -> Frame {
        self.window
    }
}
