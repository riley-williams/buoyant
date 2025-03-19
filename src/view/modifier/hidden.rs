use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::ProposedDimensions,
    render::Renderable,
};

/// A view that uses the layout of the child view, but renders nothing
#[derive(Debug, Clone)]
pub struct Hidden<T> {
    child: T,
}

impl<T> Hidden<T> {
    pub const fn new(child: T) -> Self {
        Self { child }
    }
}

impl<T: Layout> Layout for Hidden<T> {
    type Sublayout = T::Sublayout;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.child.layout(offer, env)
    }

    fn priority(&self) -> i8 {
        self.child.priority()
    }

    fn is_empty(&self) -> bool {
        // This is still useful because the empty property affects stack spacing
        self.child.is_empty()
    }
}

impl<T: Renderable> Renderable for Hidden<T> {
    // Render nothing
    type Renderables = ();

    fn render_tree(
        &self,
        _layout: &ResolvedLayout<Self::Sublayout>,
        _origin: crate::primitives::Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
    }
}
