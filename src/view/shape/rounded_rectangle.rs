use crate::{
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct RoundedRectangle {
    corner_radius: u16,
}

impl RoundedRectangle {
    pub fn new(corner_radius: u16) -> Self {
        Self { corner_radius }
    }
}

impl Layout for RoundedRectangle {
    type Sublayout = ();

    fn layout(
        &self,
        offer: ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: offer.resolve_most_flexible(0, 10),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::{draw_target::DrawTarget, primitives::StyledDrawable};

#[cfg(feature = "embedded-graphics")]
impl<P: embedded_graphics_core::pixelcolor::PixelColor> crate::render::PixelRender<P>
    for RoundedRectangle
{
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
        _ = embedded_graphics::primitives::RoundedRectangle::new(
            embedded_graphics::primitives::Rectangle {
                top_left: origin.into(),
                size: layout.resolved_size.into(),
            },
            embedded_graphics::primitives::CornerRadii::new(
                (self.corner_radius as u32, self.corner_radius as u32).into(),
            ),
        )
        .draw_styled(&style, target);
    }
}
