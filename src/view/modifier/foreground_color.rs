use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::{primitives::ShadeSubtree, shade::ShadeSolid, Renderable},
};

/// Sets a foreground style
#[derive(Debug, PartialEq)]
pub struct ForegroundStyle<V, S> {
    inner: V,
    shader: S,
}

impl<V, S> ForegroundStyle<V, S> {
    pub fn new(color: S, inner: V) -> Self {
        Self {
            shader: color,
            inner,
        }
    }
}

impl<Inner: Layout, Color> Layout for ForegroundStyle<Inner, Color> {
    type Sublayout = Inner::Sublayout;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.inner.layout(offer, env)
    }
}

impl<C: Clone + ShadeSolid, V: Renderable<C>> Renderable<C> for ForegroundStyle<V, C> {
    type Renderables = ShadeSubtree<C, V::Renderables>;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        ShadeSubtree::new(
            self.shader.clone(),
            self.inner.render_tree(layout, origin, env),
        )
    }
}
