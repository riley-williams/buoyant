use crate::{
    font::FontLayout,
    pixel::Interpolate,
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

    fn join(source: Self, mut target: Self, config: &AnimationDomain) -> Self {
        let x = i16::interpolate(source.origin.x, target.origin.x, config.factor);
        let y = i16::interpolate(source.origin.y, target.origin.y, config.factor);
        target.origin = Point::new(x, y);
        target
    }
}
