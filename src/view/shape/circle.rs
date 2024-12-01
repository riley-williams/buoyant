use crate::{
    layout::{Layout, ResolvedLayout},
    primitives::{Point, Size},
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Circle;

impl Layout for Circle {
    type Sublayout = ();

    fn layout(
        &self,
        offer: Size,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let minimum_dimension = offer.width.min(offer.height);
        ResolvedLayout {
            sublayouts: (),
            resolved_size: Size {
                width: minimum_dimension,
                height: minimum_dimension,
            },
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::{draw_target::DrawTarget, primitives::StyledDrawable};

#[cfg(feature = "embedded-graphics")]
impl<P: embedded_graphics_core::pixelcolor::PixelColor> crate::render::PixelRender<P> for Circle {
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = P>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::RenderEnvironment<Color = P>,
    ) {
        let color = env.foreground_color();
        let style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
            .fill_color(color)
            .build();
        _ = embedded_graphics::primitives::Circle::new(
            origin.into(),
            layout.resolved_size.width as u32,
        )
        .draw_styled(&style, target);
    }
}
