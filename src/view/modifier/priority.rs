use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::Renderable,
};

/// Sets the priority of the view layout.
///
/// Stack subviews will be laid out in groups of equal priority, with higher priority views being
/// laid out first. All the remaining space will be offered, meaning greedy views will leave
/// no space at all for lower priority views. Lower priority views which have a non-zero minimum
/// size in this scenario will cause the stack children to lay out outside the stack frame.
#[derive(Debug, Clone)]
pub struct Priority<T> {
    priority: i8,
    child: T,
}

impl<T> Priority<T> {
    pub const fn new(priority: i8, child: T) -> Self {
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

    fn priority(&self) -> i8 {
        self.priority
    }

    fn is_empty(&self) -> bool {
        self.child.is_empty()
    }
}

impl<T: Renderable> Renderable for Priority<T> {
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
