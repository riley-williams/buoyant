use crate::{
    layout::{Layout, ProposedDimensions, ResolvedLayout},
    primitives::{Dimensions, Point},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
};

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyView;

impl Layout for EmptyView {
    type Sublayout = ();
    fn layout(
        &self,
        _: ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: Dimensions::zero(),
        }
    }

    fn priority(&self) -> i8 {
        i8::MIN
    }

    fn is_empty(&self) -> bool {
        true
    }
}

impl<Pixel: Copy> CharacterRender<Pixel> for EmptyView {
    fn render(
        &self,
        _: &mut impl CharacterRenderTarget<Color = Pixel>,
        _: &ResolvedLayout<Self::Sublayout>,
        _: Point,
        _: &impl crate::environment::RenderEnvironment<Color = Pixel>,
    ) {
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<Pixel: embedded_graphics_core::pixelcolor::PixelColor> crate::render::PixelRender<Pixel>
    for EmptyView
{
    fn render(
        &self,
        _: &mut impl DrawTarget<Color = Pixel>,
        _: &ResolvedLayout<Self::Sublayout>,
        _: Point,
        _: &impl crate::environment::RenderEnvironment<Color = Pixel>,
    ) {
    }
}
