use crate::{
    environment::RenderEnvironment,
    layout::{Layout, ResolvedLayout},
    pixel::Interpolate,
    primitives::{Dimensions, Point, ProposedDimensions},
    render::{AnimationConfiguration, CharacterRender},
    render_target::CharacterRenderTarget,
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Rectangle;

impl Rectangle {
    pub fn corner_radius(self, radius: u16) -> RoundedRectangle {
        RoundedRectangle::new(radius)
    }
}

impl Layout for Rectangle {
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
        _: &impl crate::environment::LayoutEnvironment,
    ) {
        layout.origin = origin;
    }
}

impl<P: Copy> CharacterRender<P> for Rectangle {
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = P>,
        layout: &ResolvedLayout<Self::Sublayout>,
        env: &impl RenderEnvironment<Color = P>,
    ) {
        let width = layout.resolved_size.width;
        let height = layout.resolved_size.height;
        let color = env.foreground_color();
        for y in 0..height.into() {
            for x in 0..width.into() {
                target.draw(layout.origin + Point::new(x, y), ' ', color);
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::primitives::StyledDrawable as _;

use super::RoundedRectangle;

#[cfg(feature = "embedded-graphics")]
impl<P: embedded_graphics_core::pixelcolor::PixelColor + Interpolate> crate::render::PixelRender<P>
    for Rectangle
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = P>,
        layout: &ResolvedLayout<Self::Sublayout>,
        env: &impl RenderEnvironment<Color = P>,
    ) {
        let color = env.foreground_color();

        let style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
            .fill_color(color)
            .build();
        _ = embedded_graphics::primitives::Rectangle {
            top_left: layout.origin.into(),
            size: layout.resolved_size.into(),
        }
        .draw_styled(&style, target);
    }

    fn render_animated(
        target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = P>,
        _source_view: &Self,
        source_layout: &ResolvedLayout<Self::Sublayout>,
        _target_view: &Self,
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

        let style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
            .fill_color(color)
            .build();
        _ = embedded_graphics::primitives::Rectangle {
            top_left: origin.into(),
            size: size.into(),
        }
        .draw_styled(&style, target);
    }
}
