use embedded_graphics::prelude::{PixelColor, Point as EgPoint};
use u8g2_fonts::{
    FontRenderer,
    types::{FontColor, VerticalPosition},
};

use crate::primitives::{Point, Size};
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
    fn rendered_size(&self, character: char) -> crate::primitives::Size {
        self.get_rendered_dimensions(
            character,
            EgPoint::zero(),
            u8g2_fonts::types::VerticalPosition::Top,
        )
        .map_or(Size::zero(), |d| {
            d.bounding_box.unwrap_or_default().size.into()
        })
    }

    fn default_line_height(&self) -> u32 {
        self.get_default_line_height()
    }

    fn advance(&self, character: char) -> u32 {
        self.get_rendered_dimensions(
            character,
            EgPoint::zero(),
            u8g2_fonts::types::VerticalPosition::Top,
        )
        .map_or(0, |d| d.advance.x as u32)
    }

    fn baseline(&self) -> u32 {
        self.get_ascent() as u32 // FIXME: This is wrong...
    }

    fn str_width(&self, text: &str) -> u32 {
        text.chars().map(|c| self.advance(c)).sum()
    }
}
