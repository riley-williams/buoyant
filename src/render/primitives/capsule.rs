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
pub struct Capsule {
    origin: Point,
    size: Size,
}

impl Capsule {
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
}

impl<C: PixelColor> EmbeddedGraphicsRender<C> for Capsule {
    fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &PrimitiveStyle<C>) {
        let radius = self.size.height.min(self.size.width) / 2;
        let rectangle = embedded_graphics::primitives::Rectangle {
            top_left: self.origin.into(),
            size: self.size.into(),
        };

        _ = embedded_graphics::primitives::RoundedRectangle::with_equal_corners(
            rectangle,
            embedded_graphics::prelude::Size {
                width: radius.into(),
                height: radius.into(),
            },
        )
        .draw_styled(style, render_target);
    }

    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
        let x = i16::interpolate(source.origin.x, target.origin.x, config.factor);
        let y = i16::interpolate(source.origin.y, target.origin.y, config.factor);
        let w = u16::interpolate(source.size.width, target.size.width, config.factor);
        let h = u16::interpolate(source.size.height, target.size.height, config.factor);
        Capsule::new(Point::new(x, y), Size::new(w, h))
    }
}
