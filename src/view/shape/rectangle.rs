use crate::{
    layout::{Layout, ResolvedLayout},
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Rectangle;

impl Layout for Rectangle {
    type Sublayout = ();

    fn layout(
        &self,
        offer: Size,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: offer,
        }
    }
}

impl<P: Copy> CharacterRender<P> for Rectangle {
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = P>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::RenderEnvironment<Color = P>,
    ) {
        let width = layout.resolved_size.width;
        let height = layout.resolved_size.height;
        let color = env.foreground_color();
        for y in 0..height as i16 {
            for x in 0..width as i16 {
                target.draw(origin + Point::new(x, y), ' ', color);
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<P: embedded_graphics_core::pixelcolor::PixelColor> crate::render::PixelRender<P>
    for Rectangle
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = P>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::RenderEnvironment<Color = P>,
    ) {
        let width = layout.resolved_size.width;
        let height = layout.resolved_size.height;
        let color = env.foreground_color();
        for y in 0..height as i16 {
            for x in 0..width as i16 {
                let point = origin + Point::new(x, y);
                _ = target.draw_iter(core::iter::once(embedded_graphics::Pixel(
                    point.into(),
                    color,
                )));
            }
        }
    }
}
