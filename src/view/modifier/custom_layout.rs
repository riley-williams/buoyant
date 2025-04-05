use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::ProposedDimensions,
    render::Renderable,
};

#[derive(Debug, Clone)]
pub struct CustomLayout<T, F> {
    child: T,
    layout_fn: F,
}

impl<T, F: Fn(ProposedDimensions) -> ProposedDimensions> CustomLayout<T, F> {
    pub const fn new(child: T, layout_fn: F) -> Self {
        Self { child, layout_fn }
    }
}

impl<T: Layout, F: Fn(ProposedDimensions) -> ProposedDimensions> Layout for CustomLayout<T, F> {
    type Sublayout = T::Sublayout;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let modified_offer = (self.layout_fn)(*offer);
        self.child.layout(&modified_offer, env)
    }

    fn priority(&self) -> i8 {
        self.child.priority()
    }

    fn is_empty(&self) -> bool {
        self.child.is_empty()
    }
}
impl<T: Renderable, F: Fn(ProposedDimensions) -> ProposedDimensions> Renderable
    for CustomLayout<T, F>
{
    type Renderables = T::Renderables;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        self.child.render_tree(layout, origin, env)
    }
}
