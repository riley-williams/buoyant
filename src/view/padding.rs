use crate::{
    layout::{Environment, Layout, ResolvedLayout},
    primitives::{Point, Size},
    render::{Render, RenderProxy},
    render_target::RenderTarget,
};

pub struct Padding<T> {
    padding: u16,
    child: T,
}

impl<T> Padding<T> {
    pub fn new(padding: u16, child: T) -> Self {
        Self { padding, child }
    }
}

impl<V: Layout> Layout for Padding<V> {
    type Sublayout<'a> = ResolvedLayout<V::Sublayout<'a>> where V: 'a;

    fn layout(&self, offer: Size, env: &dyn Environment) -> ResolvedLayout<Self::Sublayout<'_>> {
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

impl<'a, Pixel, View: Layout> Render<Pixel, ResolvedLayout<View::Sublayout<'a>>> for Padding<View>
where
    View: Render<Pixel, View::Sublayout<'a>>,
{
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        layout: &ResolvedLayout<ResolvedLayout<View::Sublayout<'a>>>,
        env: &dyn Environment,
    ) {
        let mut proxy = RenderProxy::new(
            target,
            Point {
                x: self.padding as i16,
                y: self.padding as i16,
            },
        );
        self.child.render(&mut proxy, &layout.sublayouts, env);
    }
}
