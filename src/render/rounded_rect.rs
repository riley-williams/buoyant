use crate::primitives::{Point, Size};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundedRect {
    pub origin: Point,
    pub size: Size,
    pub corner_radius: u16,
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use embedded_graphics::{
        prelude::PixelColor,
        primitives::{PrimitiveStyle, StyledDrawable as _},
    };
    use embedded_graphics_core::draw_target::DrawTarget;

    use crate::render::EmbeddedGraphicsRender;
    use crate::{pixel::Interpolate, render::AnimationDomain};

    use super::{Point, RoundedRect, Size};

    impl<C: PixelColor> EmbeddedGraphicsRender<C> for RoundedRect {
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &C) {
            _ = embedded_graphics::primitives::RoundedRectangle::new(
                embedded_graphics::primitives::Rectangle {
                    top_left: self.origin.into(),
                    size: self.size.into(),
                },
                embedded_graphics::primitives::CornerRadii::new(
                    (u32::from(self.corner_radius), u32::from(self.corner_radius)).into(),
                ),
            )
            .draw_styled(&PrimitiveStyle::with_fill(*style), render_target);
        }

        #[allow(clippy::many_single_char_names)]
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
}
