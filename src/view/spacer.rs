use crate::{
    layout::{Environment, Layout, LayoutDirection, PreRender},
    primitives::{uint, Size},
    render::{Render, RenderTarget},
};

#[derive(Default)]
pub struct Spacer {
    min_length: uint,
}

impl Layout for Spacer {
    type Cache<'a> = ();
    fn layout(&self, offer: Size, env: &dyn Environment) -> PreRender<'_, Self, ()> {
        let size = match env.layout_direction() {
            LayoutDirection::Horizontal => {
                Size::new(core::cmp::max(offer.width, self.min_length), 0)
            }
            LayoutDirection::Vertical => Size::new(0, offer.height),
        };
        PreRender {
            source_view: self,
            layout_cache: (),
            resolved_size: size,
        }
    }

    fn priority(&self) -> i8 {
        // This view should take all the remaining space after other siblings have been laid out
        i8::MIN
    }
}

impl<I> Render<I> for PreRender<'_, Spacer, ()> {
    fn render(&self, _target: &mut impl RenderTarget<I>, _env: &impl Environment) {}
}
