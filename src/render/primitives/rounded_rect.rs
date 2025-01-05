use crate::{
    pixel::Interpolate,
    primitives::{Point, Size},
    render::{shade::Shader, AnimationDomain, Render},
    render_target::RenderTarget,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundedRect {
    pub origin: Point,
    pub size: Size,
    pub corner_radius: u16,
}

// TODO: This draws a rectangle
impl<C> Render<C> for RoundedRect {
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

    fn render_animated(
        render_target: &mut impl RenderTarget<Color = C>,
        source: &Self,
        source_shader: &impl Shader<Color = C>,
        target: &Self,
        target_shader: &impl Shader<Color = C>,
        config: &AnimationDomain,
    ) {
        let min_x = source.origin.x.min(target.origin.x);
        let max_x = (source.origin.x + source.size.width as i16)
            .max(target.origin.x + target.size.width as i16);
        let min_y = source.origin.y.min(target.origin.y);
        let max_y = (source.origin.y + source.size.height as i16)
            .max(target.origin.y + target.size.height as i16);

        for y in min_y..max_y {
            for x in min_x..max_x {
                let p = Point::new(x, y);
                let color = if config.factor < 128 {
                    source_shader.shade(p)
                } else {
                    target_shader.shade(p)
                };
                render_target.draw(p, color);
            }
        }
    }

    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
        let x = i16::interpolate(source.origin.x, target.origin.x, config.factor);
        let y = i16::interpolate(source.origin.y, target.origin.y, config.factor);
        let w = u16::interpolate(source.size.width, target.size.width, config.factor);
        let h = u16::interpolate(source.size.height, target.size.height, config.factor);
        let r = u16::interpolate(source.corner_radius, target.corner_radius, config.factor);
        RoundedRect {
            origin: Point::new(x, y),
            size: Size::new(w, h),
            corner_radius: r,
        }
    }
}
