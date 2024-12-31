use crate::{
    environment::RenderEnvironment,
    layout::{Layout, ResolvedLayout},
    pixel::Interpolate,
    primitives::{Dimension, Dimensions, Point, ProposedDimensions},
    render::AnimationConfiguration,
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Circle;

impl Circle {
    pub fn new() -> Self {
        Self
    }
}

impl Layout for Circle {
    type Sublayout = ();

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let minimum_dimension = offer.width.min(offer.height).resolve_most_flexible(0, 10);
        ResolvedLayout {
            sublayouts: (),
            resolved_size: Dimensions {
                width: minimum_dimension,
                height: minimum_dimension,
            },
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

impl<P: Copy> crate::render::CharacterRender<P> for Circle {
    fn render(
        &self,
        target: &mut impl crate::render_target::CharacterRenderTarget<Color = P>,
        layout: &ResolvedLayout<Self::Sublayout>,
        env: &impl crate::environment::RenderEnvironment<Color = P>,
    ) {
        let radius = u16::from(layout.resolved_size.width) as i32 / 2;
        let r2 = radius * radius;
        let color = env.foreground_color();
        for y in 0..layout.resolved_size.height.into() {
            for x in 0..layout.resolved_size.width.into() {
                let dx = x as i32 - radius;
                let dy = y as i32 - radius;
                if dx * dx + dy * dy <= r2 {
                    target.draw(layout.origin + Point::new(x, y), ' ', color);
                }
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::{draw_target::DrawTarget, primitives::StyledDrawable};

#[cfg(feature = "embedded-graphics")]
impl<P: embedded_graphics_core::pixelcolor::PixelColor + Interpolate> crate::render::PixelRender<P>
    for Circle
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
        _ = embedded_graphics::primitives::Circle::new(
            layout.origin.into(),
            layout.resolved_size.width.into(),
        )
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
        let width = Dimension::interpolate(
            source_layout.resolved_size.width,
            target_layout.resolved_size.width,
            config.factor,
        );

        let style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
            .fill_color(color)
            .build();
        _ = embedded_graphics::primitives::Circle::new(origin.into(), width.into())
            .draw_styled(&style, target);
    }
}
