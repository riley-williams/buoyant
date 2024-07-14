use embedded_graphics::{draw_target::DrawTarget, geometry::Dimensions, primitives::Rectangle};

use crate::{
    pixel::ColorValue,
    primitives::{Point, Size},
};

use super::RenderTarget;

impl<D, Pixel> RenderTarget<Pixel> for D
where
    D: DrawTarget<Color = Pixel> + Dimensions,
    Pixel: ColorValue,
{
    fn size(&self) -> Size {
        self.bounding_box().size.into()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn draw(&mut self, point: Point, item: Pixel) {
        _ = self.fill_solid(
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
}
