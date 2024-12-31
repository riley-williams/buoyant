use crate::{
    environment::RenderEnvironment,
    layout::{Layout, ResolvedLayout},
    pixel::Interpolate,
    primitives::{Dimensions, Point, ProposedDimensions},
    render::AnimationConfiguration,
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
        offer: &ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: offer.resolve_most_flexible(0, 10),
            origin: Point::zero(),
        }
    }

    fn place_subviews(
        &self,
        layout: &mut ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl crate::environment::LayoutEnvironment,
    ) {
        layout.origin = origin;
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::{draw_target::DrawTarget, primitives::StyledDrawable};

#[cfg(feature = "embedded-graphics")]
impl<P: embedded_graphics_core::pixelcolor::PixelColor + Interpolate> crate::render::PixelRender<P>
    for RoundedRectangle
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = P>,
        layout: &ResolvedLayout<Self::Sublayout>,
        env: &impl crate::environment::RenderEnvironment<Color = P>,
    ) {
        let color = env.foreground_color();
        let style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
            .fill_color(color)
            .build();
        _ = embedded_graphics::primitives::RoundedRectangle::new(
            embedded_graphics::primitives::Rectangle {
                top_left: layout.origin.into(),
                size: layout.resolved_size.into(),
            },
            embedded_graphics::primitives::CornerRadii::new(
                (self.corner_radius as u32, self.corner_radius as u32).into(),
            ),
        )
        .draw_styled(&style, target);
    }
    fn render_animated(
        target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = P>,
        source_view: &Self,
        source_layout: &ResolvedLayout<Self::Sublayout>,
        target_view: &Self,
        target_layout: &ResolvedLayout<Self::Sublayout>,
        source_env: &impl RenderEnvironment<Color = P>,
        target_env: &impl RenderEnvironment<Color = P>,
        config: &AnimationConfiguration,
    ) {
        let color = P::interpolate(
            source_env.foreground_color(),
            target_env.foreground_color(),
            config.factor,
        );

        let origin = Point::interpolate(source_layout.origin, target_layout.origin, config.factor);
        let size = Dimensions::interpolate(
            source_layout.resolved_size,
            target_layout.resolved_size,
            config.factor,
        );

        let corner_radius = u16::interpolate(
            source_view.corner_radius,
            target_view.corner_radius,
            config.factor,
        ) as u32;

        let style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
            .fill_color(color)
            .build();
        _ = embedded_graphics::primitives::RoundedRectangle::new(
            embedded_graphics::primitives::Rectangle {
                top_left: origin.into(),
                size: size.into(),
            },
            embedded_graphics::primitives::CornerRadii::new((corner_radius, corner_radius).into()),
        )
        .draw_styled(&style, target);
    }
}
