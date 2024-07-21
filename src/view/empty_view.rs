use crate::{
    layout::{Layout, ResolvedLayout},
    pixel::PixelColor,
    primitives::{Point, Size},
    render::Render,
    render_target::RenderTarget,
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

impl<Pixel: PixelColor> Render<Pixel> for EmptyView {
    fn render(
        &self,
        _: &mut impl RenderTarget<Pixel>,
        _: &ResolvedLayout<Self::Sublayout>,
        _: Point,
        _: &impl crate::environment::RenderEnvironment<Pixel>,
    ) {
    }
}
