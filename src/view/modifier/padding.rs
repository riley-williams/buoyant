use crate::{
    environment::Environment,
    layout::{Layout, ResolvedLayout},
    pixel::RenderUnit,
    primitives::{Point, Size},
    render::Render,
    render_target::RenderTarget,
};

/// A view that adds padding around a child view.
/// When the space offered to the padding is less than 2* the padding, the padding will
/// not be truncated and will return a size larger than the offer.
pub struct Padding<T> {
    padding: u16,
    child: T,
}

impl<T> Padding<T> {
    pub fn new(padding: u16, child: T) -> Self {
        Self { padding, child }
    }
}

impl<T> PartialEq for Padding<T> {
    fn eq(&self, other: &Self) -> bool {
        self.padding == other.padding
    }
}

impl<V: Layout> Layout for Padding<V> {
    type Sublayout = ResolvedLayout<V::Sublayout>;

    fn layout(&self, offer: Size, env: &impl Environment) -> ResolvedLayout<Self::Sublayout> {
        let padded_offer = Size::new(
            offer.width.saturating_sub(2 * self.padding),
            offer.height.saturating_sub(2 * self.padding),
        );
        let child_layout = self.child.layout(padded_offer, env);
        let padding_size =
            child_layout.resolved_size + Size::new(2 * self.padding, 2 * self.padding);
        ResolvedLayout {
            sublayouts: child_layout,
            resolved_size: padding_size,
        }
    }
}

impl<Pixel, View: Layout> Render<Pixel, ResolvedLayout<View::Sublayout>> for Padding<View>
where
    View: Render<Pixel, View::Sublayout>,
    Pixel: RenderUnit,
{
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        layout: &ResolvedLayout<ResolvedLayout<View::Sublayout>>,
        env: &impl Environment,
    ) {
        let original_window = target.window();
        target.set_window_origin(
            original_window.origin + Point::new(self.padding as i16, self.padding as i16),
        );
        self.child.render(target, &layout.sublayouts, env);
        target.set_window(original_window);
    }
}
