use embedded_graphics::prelude::{PixelColor, Point as EgPoint};
use u8g2_fonts::{
    FontRenderer,
    types::{FontColor, VerticalPosition},
};

use crate::surface::AsDrawTarget;
use crate::{
    font,
    primitives::{Point, geometry::Rectangle},
};

use super::{Font, FontMetrics, FontRender};

impl Font for FontRenderer {
    type Attributes = ();
    fn metrics(&self, _attributes: &Self::Attributes) -> impl FontMetrics {
        self
    }
}

impl font::Sealed for FontRenderer {}

impl<C: PixelColor> FontRender<C> for FontRenderer {
    fn draw(
        &self,
        character: char,
        offset: Point,
        color: C,
        _background_color: Option<C>,
        _attributes: &Self::Attributes,
        surface: &mut impl crate::surface::Surface<Color = C>,
    ) {
        let font_color = FontColor::Transparent(color);
        let mut draw_target = surface.draw_target();
        _ = self.render(
            character,
            offset.into(),
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

    fn vertical_metrics(&self) -> font::VMetrics {
        let ascent = self.get_ascent() as i32;
        let descent = -(self.get_descent() as i32);
        font::VMetrics {
            ascent,
            descent,
            line_spacing: self.get_default_line_height() as i32 - ascent + descent,
        }
    }

    fn advance(&self, character: char) -> u32 {
        self.get_rendered_dimensions(character, EgPoint::zero(), VerticalPosition::Top)
            .map_or(0, |d| d.advance.x as u32)
    }
}
