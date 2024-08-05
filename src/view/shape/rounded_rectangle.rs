use crate::{
    layout::{Layout, ResolvedLayout},
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
};
use embedded_graphics::primitives::StyledDrawable;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct RoundedRectangle {
    corner_radius: u16,
    anti_aliasing: bool,
}

impl RoundedRectangle {
    pub fn new(corner_radius: u16) -> Self {
        Self {
            corner_radius,
            anti_aliasing: false,
        }
    }
    pub fn with_anti_aliasing(mut self, anti_aliasing: bool) -> Self {
        self.anti_aliasing = anti_aliasing;
        self
    }
}

impl Layout for RoundedRectangle {
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

impl<P: Copy> CharacterRender<P> for RoundedRectangle {
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
