use crate::{
    font::{FontMetrics, FontRender},
    primitives::{Interpolate, Point, Size},
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::{self, Glyph, RenderTarget, SolidBrush},
    view::{HorizontalTextAlignment, WhitespaceWrap},
};

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl<C: Copy, T: AsRef<str>, F: FontRender> Render<C> for Text<'_, T, F> {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        style: &C,
        offset: Point,
    ) {
        if self.size.area() == 0 {
            return;
        }

        let brush = SolidBrush::new(*style);

        let origin = self.origin + offset;
        let line_height = self.font.line_height() as i16;

        let mut height = 0;
        let wrap = WhitespaceWrap::new(self.text.as_ref(), self.size.width, self.font);

        let metrics = self.font.metrics();

        for line in wrap {
            // TODO: WhitespaceWrap should also return the width of the line
            let width = self.font.str_width(line);

            let line_x = self.alignment.align(self.size.width as i16, width as i16) + origin.x;

            let mut x = 0;
            render_target.draw_glyphs(
                render_target::geometry::Point::new(line_x.into(), (origin.y + height).into()),
                &brush,
                line.chars().map(|c| {
                    let index = self.font.glyph_index(c);
                    let glyph = Glyph {
                        id: index,
                        character: c,
                        x: x.into(),
                        y: 0,
                    };
                    x += metrics.advance(glyph.id) as i16;
                    glyph
                }),
                self.font,
            );

            height += line_height;
            if height >= self.size.height as i16 {
                break;
            }
        }
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
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
