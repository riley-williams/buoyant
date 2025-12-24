use embedded_graphics::prelude::{Dimensions, PixelColor, Point as EgPoint};
use u8g2_fonts::{
    FontRenderer,
    types::{FontColor, VerticalPosition},
};

use crate::primitives::{Point, Size, geometry::Rectangle};
use crate::surface::AsDrawTarget;

use super::{Font, FontMetrics, FontRender};

impl Font for FontRenderer {
    fn metrics(&self) -> impl FontMetrics {
        self
    }
}

impl crate::font::Sealed for FontRenderer {}

impl<C: PixelColor> FontRender<C> for FontRenderer {
    fn draw(
        &self,
        character: char,
        color: C,
        _background_color: Option<C>,
        surface: &mut impl crate::surface::Surface<Color = C>,
    ) {
        let font_color = FontColor::Transparent(color);
        let mut draw_target = surface.draw_target();
        _ = self.render(
            character,
            Point::zero().into(),
            VerticalPosition::Top,
            font_color,
            &mut draw_target,
        );
    }
}

impl FontMetrics for FontRenderer {
    fn rendered_size(&self, character: char) -> Option<Rectangle> {
        self.get_rendered_dimensions_aligned(
            character,
            EgPoint::zero(),
            VerticalPosition::Top,
            u8g2_fonts::types::HorizontalAlignment::Left,
        )
        .map_or(None, |d| d.map(Into::into))
    }

    fn default_line_height(&self) -> u32 {
        self.get_default_line_height()
    }

    fn advance(&self, character: char) -> u32 {
        self.get_rendered_dimensions(character, EgPoint::zero(), VerticalPosition::Top)
            .map_or(0, |d| d.advance.x as u32)
    }

    fn maximum_character_size(&self) -> Size {
        self.get_font_bounding_box(VerticalPosition::Top)
            .bounding_box()
            .size
            .into()
    }
}
