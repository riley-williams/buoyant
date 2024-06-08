use crate::{
    layout::{Environment, Layout, PreRender},
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
    type Cache<'a> = V::Cache<'a> where V: 'a;

    fn layout(&self, offer: Size, env: &dyn Environment) -> PreRender<'_, Self, Self::Cache<'_>> {
        let padded_offer = Size::new(
            offer.width.saturating_sub(2 * self.padding),
            offer.height.saturating_sub(2 * self.padding),
        );
        let child_pre_render = self.child.layout(padded_offer, env);
        PreRender {
            source_view: self,
            layout_cache: child_pre_render.layout_cache,
            resolved_size: offer,
        }
    }
}

impl<Pixel, View, Cache> Render<Pixel, Cache> for Padding<View>
where
    View: Render<Pixel, Cache>,
{
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        cache: &Cache,
        resolved_size: Size,
        env: &dyn Environment,
    ) {
        let mut proxy = RenderProxy::new(
            target,
            Point {
                x: self.padding as i16,
                y: self.padding as i16,
            },
        );
        // TODO: inset view
        self.child.render(&mut proxy, cache, resolved_size, env);
    }
}
