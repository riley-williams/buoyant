use crate::{
    environment::RenderEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::CharacterRender,
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

use super::RoundedRectangle;

#[cfg(feature = "embedded-graphics")]
impl<P: embedded_graphics_core::pixelcolor::PixelColor> crate::render::PixelRender<P>
    for Rectangle
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = P>,
        layout: &ResolvedLayout<Self::Sublayout>,
        env: &impl RenderEnvironment<Color = P>,
    ) {
        let width = layout.resolved_size.width;
        let height = layout.resolved_size.height;
        let color = env.foreground_color();
        for y in 0..height.into() {
            for x in 0..width.into() {
                let point = layout.origin + Point::new(x, y);
                _ = target.draw_iter(core::iter::once(embedded_graphics::Pixel(
                    point.into(),
                    color,
                )));
            }
        }
    }
}
