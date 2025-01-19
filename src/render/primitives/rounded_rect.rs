use crate::{
    pixel::Interpolate,
    primitives::{Point, Size},
    render::{AnimationDomain, EmbeddedGraphicsRender},
};

use embedded_graphics::{
    prelude::PixelColor,
    primitives::{PrimitiveStyle, StyledDrawable as _},
};
use embedded_graphics_core::draw_target::DrawTarget;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundedRect {
    pub origin: Point,
    pub size: Size,
    pub corner_radius: u16,
}

impl<C: PixelColor> EmbeddedGraphicsRender<C> for RoundedRect {
    fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &PrimitiveStyle<C>) {
        _ = embedded_graphics::primitives::RoundedRectangle::new(
            embedded_graphics::primitives::Rectangle {
                top_left: self.origin.into(),
                size: self.size.into(),
            },
            embedded_graphics::primitives::CornerRadii::new(
                (self.corner_radius as u32, self.corner_radius as u32).into(),
            ),
        )
        .draw_styled(style, render_target);
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
