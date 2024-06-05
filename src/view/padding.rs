use crate::{
    layout::{Environment, Layout, PreRender},
    primitives::{iint, uint, Point, Size},
    render::{Render, RenderProxy, RenderTarget},
};

pub struct Padding<T> {
    padding: uint,
    child: T,
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

impl<'a, V: Layout, P> Render<P> for PreRender<'_, Padding<V>, V::Cache<'a>>
where
    V: Render<P>,
{
    fn render(&self, target: &mut impl RenderTarget<P>, env: &impl Environment) {
        let mut proxy = RenderProxy::new(
            target,
            Point {
                x: self.source_view.padding as iint,
                y: self.source_view.padding as iint,
            },
        );
        self.source_view.child.render(&mut proxy, env);
    }
}
