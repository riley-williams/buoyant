use crate::{
    font::FontLayout,
    pixel::Interpolate,
    primitives::{Point, Size},
    render::{AnimationDomain, Render},
    view::{HorizontalTextAlignment, WhitespaceWrap},
};
use embedded_graphics_core::Drawable;

use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::PixelColor,
    primitives::PrimitiveStyle,
};
use embedded_graphics_core::draw_target::DrawTarget;

#[derive(Debug, Clone, PartialEq)]
pub struct StaticText<'a> {
    pub origin: Point,
    pub size: Size,
    pub font: &'a MonoFont<'a>,
    pub text: &'a str,
    pub alignment: HorizontalTextAlignment,
}

impl<C: PixelColor> Render<C> for StaticText<'_> {
    fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &PrimitiveStyle<C>) {
        if self.size.area() == 0 {
            return;
        }

        let line_height = self.font.line_height() as i16;

        let mut origin: embedded_graphics_core::geometry::Point = self.origin.into();
        origin.y += self.font.baseline() as i32;
        // TODO: add default?
        let style = MonoTextStyle::new(self.font, style.fill_color.unwrap());
        let mut height = 0;
        let wrap = WhitespaceWrap::new(self.text, self.size.width, self.font);
        for line in wrap {
            // TODO: WhitespaceWrap should also return the width of the line
            let width = self.font.str_width(line);

            let x = self.alignment.align(self.size.width as i16, width as i16);
            let txt_start = self.origin + Point::new(x, height);
            _ = embedded_graphics::text::Text::new(self.text, txt_start.into(), style)
                .draw(render_target);

            height += line_height;
            if height >= self.size.height as i16 {
                break;
            }
        }
    }

    fn join(source: Self, mut target: Self, config: &AnimationDomain) -> Self {
        let x = i16::interpolate(source.origin.x, target.origin.x, config.factor);
        let y = i16::interpolate(source.origin.y, target.origin.y, config.factor);
        target.origin = Point::new(x, y);
        target
    }
}
