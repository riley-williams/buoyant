use crate::{
    environment::LayoutEnvironment,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// A view that uses the layout of the child view, but renders nothing
#[derive(Debug, Clone)]
pub struct Hidden<T> {
    child: T,
}

impl<T: ViewMarker> Hidden<T> {
    pub const fn new(child: T) -> Self {
        Self { child }
    }
}

impl<T> ViewMarker for Hidden<T>
where
    T: ViewMarker,
{
    type Renderables = (); // Render nothing
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T> ViewLayout<Captures> for Hidden<T>
where
    T: ViewLayout<Captures>,
{
    type Sublayout = T::Sublayout;
    type State = T::State;

    fn priority(&self) -> i8 {
        self.child.priority()
    }

    fn is_empty(&self) -> bool {
        // This is still useful because the empty property affects stack spacing
        self.child.is_empty()
    }

    fn transition(&self) -> Self::Transition {
        self.child.transition()
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
        self.child.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        _layout: &ResolvedLayout<Self::Sublayout>,
        _origin: Point,
        _env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> Self::Renderables {
    }
}
