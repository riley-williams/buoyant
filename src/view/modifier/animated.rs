use crate::{
    layout::Layout,
    render::{Animate, Renderable},
    Animation,
};

#[derive(Debug, Clone)]
pub struct Animated<View, Value> {
    view: View,
    animation: Animation,
    value: Value,
}

impl<View, Value: PartialEq + Clone> Animated<View, Value> {
    pub const fn new(view: View, animation: Animation, value: Value) -> Self {
        Animated {
            view,
            animation,
            value,
        }
    }
}

impl<T: Layout, U> Layout for Animated<T, U> {
    type Sublayout = T::Sublayout;

    fn layout(
        &self,
        offer: &crate::primitives::ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> crate::layout::ResolvedLayout<Self::Sublayout> {
        self.view.layout(offer, env)
    }
}

impl<T: Renderable<C>, C, U: PartialEq + Clone> Renderable<C> for Animated<T, U> {
    type Renderables = Animate<T::Renderables, U>;

    fn render_tree(
        &self,
        layout: &crate::layout::ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> Self::Renderables {
        Animate::new(
            self.view.render_tree(layout, origin, env),
            self.animation.clone(),
            env.app_time(),
            self.value.clone(),
        )
    }
}
