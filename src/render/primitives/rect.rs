use crate::pixel::Interpolate;
use crate::render::shade::Shader;
use crate::render::AnimationDomain;
use crate::render_target::RenderTarget;
use crate::{
    primitives::{Point, Size},
    render::Render,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
}

// TODO: not really ideal...reimplement later
impl<C> Render<C> for Rect {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<Color = C>,
        shader: &impl Shader<Color = C>,
    ) {
        for y in self.origin.y..(self.origin.y + self.size.height as i16) {
            for x in self.origin.x..(self.origin.x + self.size.width as i16) {
                let p = Point::new(x, y);
                render_target.draw(p, shader.shade(p));
            }
        }
    }

    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
        let x = i16::interpolate(source.origin.x, target.origin.x, config.factor);
        let y = i16::interpolate(source.origin.y, target.origin.y, config.factor);
        let w = u16::interpolate(source.size.width, target.size.width, config.factor);
        let h = u16::interpolate(source.size.height, target.size.height, config.factor);
        Rect::new(Point::new(x, y), Size::new(w, h))
    }
}
