use crate::{
    font::FontLayout,
    primitives::{Point, Size},
    render::{shade::Shader, AnimationDomain, Render},
    render_target::RenderTarget,
    view::{HorizontalTextAlignment, WhitespaceWrap},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedText<const N: usize, F> {
    pub origin: Point,
    pub size: Size,
    pub font: F,
    pub text: heapless::String<N>,
    pub alignment: HorizontalTextAlignment,
}

impl<C, const N: usize, F: FontLayout> Render<C> for OwnedText<N, &F> {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<Color = C>,
        shader: &impl Shader<Color = C>,
    ) {
        if self.size.area() == 0 {
            return;
        }

        let line_height = self.font.line_height() as i16;

        let mut height = 0;
        let wrap = WhitespaceWrap::new(&self.text, self.size.width, self.font);
        for line in wrap {
            // TODO: WhitespaceWrap should also return the width of the line
            let width = self.font.str_width(line);

            let x = self.alignment.align(self.size.width as i16, width as i16);

            render_target.draw_text(line, self.origin + Point::new(x, height), shader);

            height += line_height;
            if height >= self.size.height as i16 {
                break;
            }
        }
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<Color = C>,
        _source: &Self,
        _: &impl Shader<Color = C>,
        target: &Self,
        target_shader: &impl Shader<Color = C>,
        _config: &AnimationDomain,
    ) {
        target.render(render_target, target_shader);
    }

    fn join(_source: Self, target: Self, _config: &AnimationDomain) -> Self {
        target
    }
}
