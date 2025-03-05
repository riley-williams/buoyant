use crate::{
    layout::Layout,
    primitives::Point,
    render::{Offset, Renderable},
};

#[derive(Debug, Clone)]
pub struct GeometryGroup<View> {
    inner: View,
}

impl<View> GeometryGroup<View> {
    pub const fn new(view: View) -> Self {
        Self { inner: view }
    }
}

// Transparent layout
impl<T: Layout> Layout for GeometryGroup<T> {
    type Sublayout = T::Sublayout;

    fn layout(
        &self,
        offer: &crate::primitives::ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> crate::layout::ResolvedLayout<Self::Sublayout> {
        self.inner.layout(offer, env)
    }

    fn priority(&self) -> i8 {
        self.inner.priority()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T: Renderable<C>, C> Renderable<C> for GeometryGroup<T> {
    type Renderables = Offset<T::Renderables>;

    fn render_tree(
        &self,
        layout: &crate::layout::ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> Self::Renderables {
        // Store the offset, and render subtrees from zero
        Offset::new(origin, self.inner.render_tree(layout, Point::zero(), env))
    }
}
