use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::{Renderable, ShadeSubtree},
};

/// Sets a foreground style
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForegroundStyle<V, S> {
    inner: V,
    style: S,
}

impl<V, S> ForegroundStyle<V, S> {
    pub const fn new(style: S, inner: V) -> Self {
        Self { inner, style }
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

    fn priority(&self) -> i8 {
        self.inner.priority()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<C: Clone, V: Renderable> Renderable for ForegroundStyle<V, C> {
    type Renderables = ShadeSubtree<C, V::Renderables>;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        ShadeSubtree::new(
            self.style.clone(),
            self.inner.render_tree(layout, origin, env),
        )
    }
}
