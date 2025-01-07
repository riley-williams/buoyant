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
pub struct OwnedText<'a, const N: usize> {
    pub origin: Point,
    pub size: Size,
    pub font: &'a MonoFont<'a>,
    pub text: heapless::String<N>,
    pub alignment: HorizontalTextAlignment,
}

impl<C: PixelColor, const N: usize> Render<C> for OwnedText<'_, N> {
    fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &PrimitiveStyle<C>) {
        if self.size.area() == 0 {
            return;
        }

        let line_height = self.font.line_height() as i16;

        let baseline = self.font.baseline() as i16;
        // TODO: add default?
        let style = MonoTextStyle::new(self.font, style.fill_color.unwrap());
        let mut height = 0;
        let wrap = WhitespaceWrap::new(&self.text, self.size.width, self.font);
        for line in wrap {
            // TODO: WhitespaceWrap should also return the width of the line
            let width = self.font.str_width(line);

            let x = self.alignment.align(self.size.width as i16, width as i16);
            // embedded_graphics draws text at the baseline
            let txt_start = self.origin + Point::new(x, height + baseline);
            _ = embedded_graphics::text::Text::new(line, txt_start.into(), style)
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
