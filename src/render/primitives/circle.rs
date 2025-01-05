use crate::{
    pixel::Interpolate,
    primitives::Point,
    render::{shade::Shader, AnimationDomain, Render},
    render_target::RenderTarget,
};

/// A circle with the origin at the top-left corner
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circle {
    pub origin: Point,
    pub diameter: u16,
}

// TODO: This draws a rectangle
impl<C> Render<C> for Circle {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<Color = C>,
        shader: &impl Shader<Color = C>,
    ) {
        for y in self.origin.y..(self.origin.y + self.diameter as i16) {
            for x in self.origin.x..(self.origin.x + self.diameter as i16) {
                let p = Point::new(x, y);
                render_target.draw(p, shader.shade(p));
            }
        }
    }

    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
        let x = i16::interpolate(source.origin.x, target.origin.x, config.factor);
        let y = i16::interpolate(source.origin.y, target.origin.y, config.factor);
        let diameter = u16::interpolate(source.diameter, target.diameter, config.factor);
        Circle {
            origin: Point::new(x, y),
            diameter,
        }
    }
}
