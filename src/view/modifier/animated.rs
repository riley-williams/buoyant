use crate::{
    animation::Animation,
    layout::Layout,
    render::{Animate, Renderable},
};

#[derive(Debug, Clone)]
pub struct Animated<View, Value> {
    inner: View,
    animation: Animation,
    value: Value,
}

impl<View, Value: PartialEq + Clone> Animated<View, Value> {
    pub const fn new(inner: View, animation: Animation, value: Value) -> Self {
        Animated {
            inner,
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
        self.inner.layout(offer, env)
    }

    fn priority(&self) -> i8 {
        self.inner.priority()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
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
            self.inner.render_tree(layout, origin, env),
            self.animation.clone(),
            env.app_time(),
            self.value.clone(),
        )
    }
}
