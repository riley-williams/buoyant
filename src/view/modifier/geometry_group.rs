use crate::{
    layout::Layout,
    primitives::Point,
    render::{Offset, Renderable},
};

pub struct GeometryGroup<View> {
    view: View,
}

impl<View> GeometryGroup<View> {
    pub fn new(view: View) -> Self {
        Self { view }
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
        self.view.layout(offer, env)
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
        Offset::new(origin, self.view.render_tree(layout, Point::zero(), env))
    }
}
