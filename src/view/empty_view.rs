use crate::{
    layout::{Layout, ResolvedLayout},
    pixel::RenderUnit,
    primitives::Size,
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
        _: &impl crate::environment::Environment,
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

impl<Pixel: RenderUnit> Render<Pixel, ()> for EmptyView {
    fn render(
        &self,
        _: &mut impl RenderTarget<Pixel>,
        _: &ResolvedLayout<()>,
        _: &impl crate::environment::Environment,
    ) {
    }
}
