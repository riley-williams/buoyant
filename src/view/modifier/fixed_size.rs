use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimension, ProposedDimensions},
    render::Renderable,
};

/// Proposes ``ProposedDimension::Compact``, resulting in the child view rendering at its ideal
/// size along the specified axis.
pub struct FixedSize<T> {
    horizontal: bool,
    vertical: bool,
    child: T,
}

impl<T> FixedSize<T> {
    pub fn new(horizontal: bool, vertical: bool, child: T) -> Self {
        Self {
            horizontal,
            vertical,
            child,
        }
    }
}

impl<V: Layout> Layout for FixedSize<V> {
    type Sublayout = V::Sublayout;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let proposed_width = if self.horizontal {
            ProposedDimension::Compact
        } else {
            offer.width
        };

        let proposed_height = if self.vertical {
            ProposedDimension::Compact
        } else {
            offer.height
        };

        let offer = ProposedDimensions {
            width: proposed_width,
            height: proposed_height,
        };

        self.child.layout(&offer, env)
    }
}

impl<T: Renderable<C>, C> Renderable<C> for FixedSize<T> {
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
