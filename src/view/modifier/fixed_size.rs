use crate::{
    environment::LayoutEnvironment,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimension, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// Proposes ``ProposedDimension::Compact``, resulting in the child view rendering at its ideal
/// size along the specified axis.
#[derive(Debug, Clone)]
pub struct FixedSize<T> {
    horizontal: bool,
    vertical: bool,
    child: T,
}

impl<T> FixedSize<T> {
    pub const fn new(horizontal: bool, vertical: bool, child: T) -> Self {
        Self {
            horizontal,
            vertical,
            child,
        }
    }
}

impl<V> ViewMarker for FixedSize<V>
where
    V: ViewMarker,
{
    type Renderables = V::Renderables;
}

impl<Captures: ?Sized, V> ViewLayout<Captures> for FixedSize<V>
where
    V: ViewLayout<Captures>,
{
    type Sublayout = V::Sublayout;
    type State = V::State;

    fn priority(&self) -> i8 {
        self.child.priority()
    }

    fn is_empty(&self) -> bool {
        self.child.is_empty()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.child.build_state(captures)
    }
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
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

        let child_offer = ProposedDimensions {
            width: proposed_width,
            height: proposed_height,
        };

        self.child.layout(&child_offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.child.render_tree(layout, origin, env, captures, state)
    }

    fn handle_event(
        &mut self,
        event: &crate::view::Event,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> bool {
        self.child.handle_event(event, render_tree, captures, state)
    }
}
