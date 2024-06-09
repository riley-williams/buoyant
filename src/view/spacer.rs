use crate::{
    layout::{Environment, Layout, LayoutDirection, ResolvedLayout},
    primitives::Size,
    render::Render,
    render_target::RenderTarget,
};

#[derive(Default)]
pub struct Spacer {
    min_length: u16,
}

impl Layout for Spacer {
    type Sublayout<'a> = ();
    fn layout(&self, offer: Size, env: &dyn Environment) -> ResolvedLayout<()> {
        let size = match env.layout_direction() {
            LayoutDirection::Horizontal => {
                Size::new(core::cmp::max(offer.width, self.min_length), 0)
            }
            LayoutDirection::Vertical => {
                Size::new(0, core::cmp::max(offer.height, self.min_length))
            }
        };
        ResolvedLayout {
            sublayouts: (),
            resolved_size: size,
        }
    }

    fn priority(&self) -> i8 {
        // This view should take all the remaining space after other siblings have been laid out
        i8::MIN
    }
}

impl<Pixel, Sublayout> Render<Pixel, Sublayout> for Spacer {
    fn render(
        &self,
        _target: &mut impl RenderTarget<Pixel>,
        _layout: &ResolvedLayout<Sublayout>,
        _env: &dyn Environment,
    ) {
    }
}
