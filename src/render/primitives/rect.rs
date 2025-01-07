use crate::pixel::Interpolate;
use crate::render::AnimationDomain;
use crate::{
    primitives::{Point, Size},
    render::Render,
};
use embedded_graphics::prelude::PixelColor;
use embedded_graphics::primitives::{PrimitiveStyle, StyledDrawable as _};
use embedded_graphics_core::draw_target::DrawTarget;

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
impl<C: PixelColor> Render<C> for Rect {
    fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &PrimitiveStyle<C>) {
        _ = embedded_graphics::primitives::Rectangle {
            top_left: self.origin.into(),
            size: self.size.into(),
        }
        .draw_styled(style, render_target);
    }

    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
        let x = i16::interpolate(source.origin.x, target.origin.x, config.factor);
        let y = i16::interpolate(source.origin.y, target.origin.y, config.factor);
        let w = u16::interpolate(source.size.width, target.size.width, config.factor);
        let h = u16::interpolate(source.size.height, target.size.height, config.factor);
        Rect::new(Point::new(x, y), Size::new(w, h))
    }
}
