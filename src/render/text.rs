use crate::{
    font::{CharacterBufferFont, FontLayout as _},
    primitives::{Interpolate, Point, Size},
    render::{AnimatedJoin, AnimationDomain, CharacterRender, CharacterRenderTarget},
    view::{HorizontalTextAlignment, WhitespaceWrap},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Text<'a, T, F> {
    pub origin: Point,
    pub size: Size,
    pub font: &'a F,
    pub text: T,
    pub alignment: HorizontalTextAlignment,
}

impl<T: AsRef<str>, F> AnimatedJoin for Text<'_, T, F> {
    fn join(source: Self, mut target: Self, domain: &AnimationDomain) -> Self {
        target.origin = Interpolate::interpolate(source.origin, target.origin, domain.factor);
        target.size = Interpolate::interpolate(source.size, target.size, domain.factor);
        target
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use super::{Point, Text};
    use crate::primitives::Interpolate;
    use crate::render::EmbeddedGraphicsRender;
    use crate::view::WhitespaceWrap;
    use crate::{font::FontLayout as _, render::AnimationDomain};
    use embedded_graphics_core::Drawable;

    use embedded_graphics::{
        mono_font::{MonoFont, MonoTextStyle},
        prelude::PixelColor,
    };
    use embedded_graphics_core::draw_target::DrawTarget;

    impl<C: PixelColor, T: AsRef<str>> EmbeddedGraphicsRender<C> for Text<'_, T, MonoFont<'_>> {
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &C, offset: Point) {
            if self.size.area() == 0 {
                return;
            }

            let origin = self.origin + offset;
            let line_height = self.font.line_height() as i16;

            let baseline = self.font.baseline() as i16;
            // TODO: add default?
            let style = MonoTextStyle::new(self.font, *style);
            let mut height = 0;
            let wrap = WhitespaceWrap::new(self.text.as_ref(), self.size.width, self.font);
            for line in wrap {
                // TODO: WhitespaceWrap should also return the width of the line
                let width = self.font.str_width(line);

                let x = self.alignment.align(self.size.width as i16, width as i16);
                // embedded_graphics draws text at the baseline
                let txt_start = origin + Point::new(x, height + baseline);
                _ = embedded_graphics::text::Text::new(line, txt_start.into(), style)
                    .draw(render_target);

                height += line_height;
                if height >= self.size.height as i16 {
                    break;
                }
            }
        }
        fn render_animated(
            render_target: &mut impl DrawTarget<Color = C>,
            source: &Self,
            target: &Self,
            style: &C,
            offset: Point,
            domain: &AnimationDomain,
        ) {
            let origin = Interpolate::interpolate(source.origin, target.origin, domain.factor);
            let size = Interpolate::interpolate(source.size, target.size, domain.factor);
            Text {
                text: target.text.as_ref(),
                origin,
                size,
                font: target.font,
                alignment: target.alignment,
            }
            .render(render_target, style, offset);
        }
    }
}

impl<C, T: AsRef<str>> CharacterRender<C> for Text<'_, T, CharacterBufferFont> {
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
        let wrap = WhitespaceWrap::new(self.text.as_ref(), self.size.width, self.font);
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

    fn render_animated(
        render_target: &mut impl CharacterRenderTarget<Color = C>,
        source: &Self,
        target: &Self,
        style: &C,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        let origin = Interpolate::interpolate(source.origin, target.origin, domain.factor);
        let size = Interpolate::interpolate(source.size, target.size, domain.factor);
        Text {
            text: target.text.as_ref(),
            origin,
            size,
            font: target.font,
            alignment: target.alignment,
        }
        .render(render_target, style, offset);
    }
}
