use crate::{
    layout::{Layout, ResolvedLayout},
    pixel::PixelColor,
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
};

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyView;

impl Layout for EmptyView {
    type Sublayout = ();
    fn layout(
        &self,
        _: Size,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: Size::default(),
        }
    }

    fn priority(&self) -> i8 {
        i8::MIN
    }
}

impl<Pixel: PixelColor> CharacterRender<Pixel> for EmptyView {
    fn render(
        &self,
        _: &mut impl CharacterRenderTarget<Color = Pixel>,
        _: &ResolvedLayout<Self::Sublayout>,
        _: Point,
        _: &impl crate::environment::RenderEnvironment<Pixel>,
    ) {
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<Pixel: PixelColor> crate::render::EmbeddedRender<Pixel> for EmptyView {
    fn render(
        &self,
        _: &mut impl DrawTarget<Color = Pixel>,
        _: &ResolvedLayout<Self::Sublayout>,
        _: Point,
        _: &impl crate::environment::RenderEnvironment<Pixel>,
    ) {
    }
}
