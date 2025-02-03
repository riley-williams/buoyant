use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::Renderable,
};

/// A view that adds padding around a child view.
/// When the space offered to the padding is less than 2* the padding, the padding will
/// not be truncated and will return a size larger than the offer.
pub struct Priority<T> {
    priority: u16,
    child: T,
}

impl<T> Priority<T> {
    pub fn new(priority: u16, child: T) -> Self {
        Self { priority, child }
    }
}

impl<T> PartialEq for Priority<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<V: Layout> Layout for Priority<V> {
    type Sublayout = V::Sublayout;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.child.layout(offer, env)
    }
}

impl<T: Renderable<C>, C> Renderable<C> for Priority<T> {
    type Renderables = T::Renderables;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        self.child.render_tree(layout, origin, env)
    }
}
