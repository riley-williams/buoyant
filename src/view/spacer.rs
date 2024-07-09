use crate::{
    environment::Environment,
    layout::{Layout, LayoutDirection, ResolvedLayout},
    pixel::RenderUnit,
    primitives::Size,
    render::Render,
    render_target::RenderTarget,
};

#[derive(Default, PartialEq)]
pub struct Spacer {
    min_length: u16,
}

impl Layout for Spacer {
    type Sublayout = ();
    fn layout(&self, offer: Size, env: &impl Environment) -> ResolvedLayout<()> {
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

impl<Pixel: RenderUnit, Sublayout: Clone> Render<Pixel, Sublayout> for Spacer {
    fn render(
        &self,
        _target: &mut impl RenderTarget<Pixel>,
        _layout: &ResolvedLayout<Sublayout>,
        _env: &impl Environment,
    ) {
    }
}
