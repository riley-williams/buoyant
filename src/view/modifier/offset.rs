use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::Renderable,
};

/// Offsets the rendered position of a child view by a given point.
#[derive(Debug, Clone)]
pub struct Offset<T> {
    child: T,
    offset: Point,
}

impl<T> Offset<T> {
    pub const fn new(child: T, offset: Point) -> Self {
        Self { child, offset }
    }
}

impl<T: Layout> Layout for Offset<T> {
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
        self.child.is_empty()
    }
}

impl<T: Renderable> Renderable for Offset<T> {
    type Renderables = T::Renderables;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        let origin = origin + self.offset;
        self.child.render_tree(layout, origin, env)
    }
}
