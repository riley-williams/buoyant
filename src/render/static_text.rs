use crate::{
    font::{CharacterBufferFont, FontLayout},
    primitives::Interpolate,
    primitives::{Point, Size},
    render::{AnimationDomain, CharacterRender, CharacterRenderTarget},
    view::{HorizontalTextAlignment, WhitespaceWrap},
};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticText<'a, F> {
    pub origin: Point,
    pub size: Size,
    pub font: &'a F,
    pub text: &'a str,
    pub alignment: HorizontalTextAlignment,
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use super::{AnimationDomain, FontLayout, Interpolate, Point, StaticText, WhitespaceWrap};

    use embedded_graphics_core::Drawable;

    use embedded_graphics::{
        mono_font::{MonoFont, MonoTextStyle},
        prelude::PixelColor,
    };
    use embedded_graphics_core::draw_target::DrawTarget;

    use crate::render::EmbeddedGraphicsRender;

    impl<C: PixelColor> EmbeddedGraphicsRender<C> for StaticText<'_, MonoFont<'_>> {
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &C, offset: Point) {
            if self.size.area() == 0 {
                return;
            }

            let line_height = self.font.line_height() as i16;

            let baseline = self.font.baseline() as i16;
            // TODO: add default?
            let style = MonoTextStyle::new(self.font, *style);
            let mut height = 0;
            let wrap = WhitespaceWrap::new(self.text, self.size.width, self.font);
            for line in wrap {
                // TODO: WhitespaceWrap should also return the width of the line
                let width = self.font.str_width(line);

                let x = self.alignment.align(self.size.width as i16, width as i16);
                // embedded_graphics draws text at the baseline
                let txt_start = self.origin + offset + Point::new(x, height + baseline);
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
}

impl<C> CharacterRender<C> for StaticText<'_, CharacterBufferFont> {
    fn render(
        &self,
        render_target: &mut impl CharacterRenderTarget<Color = C>,
        style: &C,
        offset: Point,
    ) {
        if self.size.area() == 0 {
            return;
        }

        let origin = self.origin + offset;
        let line_height = self.font.line_height() as i16;

        let mut height = 0;
        let wrap = WhitespaceWrap::new(self.text, self.size.width, self.font);
        for line in wrap {
            // TODO: WhitespaceWrap should also return the width of the line
            let width = self.font.str_width(line);

            let x = self.alignment.align(self.size.width as i16, width as i16);
            // embedded_graphics draws text at the baseline
            let txt_start = origin + Point::new(x, height);
            render_target.draw_string(txt_start, line, style);
            height += line_height;
            if height >= self.size.height as i16 {
                break;
            }
        }
    }

    fn join(source: Self, mut target: Self, domain: &AnimationDomain) -> Self {
        let x = i16::interpolate(source.origin.x, target.origin.x, domain.factor);
        let y = i16::interpolate(source.origin.y, target.origin.y, domain.factor);
        target.origin = Point::new(x, y);
        target
    }
}
