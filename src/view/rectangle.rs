use crate::{
    layout::{Layout, ResolvedLayout},
    pixel::PixelColor,
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
    style::color_style::ColorStyle,
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

impl<P: PixelColor> CharacterRender<P> for Rectangle {
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = P>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::RenderEnvironment<P>,
    ) {
        let width = layout.resolved_size.width;
        let height = layout.resolved_size.height;
        for y in 0..height as i16 {
            for x in 0..width as i16 {
                let foreground_color =
                    env.foreground_style()
                        .shade_pixel(x as u16, y as u16, layout.resolved_size);
                target.draw(origin + Point::new(x, y), ' ', foreground_color);
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<P: PixelColor + embedded_graphics_core::pixelcolor::PixelColor>
    crate::render::EmbeddedRender<P> for Rectangle
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = P>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::RenderEnvironment<P>,
    ) {
        let width = layout.resolved_size.width;
        let height = layout.resolved_size.height;
        for y in 0..height as i16 {
            for x in 0..width as i16 {
                let foreground_color =
                    env.foreground_style()
                        .shade_pixel(x as u16, y as u16, layout.resolved_size);
                let point = origin + Point::new(x, y);
                _ = target.draw_iter(core::iter::once(embedded_graphics::Pixel(
                    point.into(),
                    foreground_color,
                )));
            }
        }
    }
}
