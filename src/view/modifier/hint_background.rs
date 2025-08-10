use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    render,
    view::{ViewLayout, ViewMarker},
};

/// A view modifier that adds a background color hint for fast simulated blending
#[derive(Debug, Clone)]
pub struct HintBackground<T, C> {
    foreground: T,
    background_color: C,
}

impl<T, C> HintBackground<T, C> {
    pub const fn new(foreground: T, background_color: C) -> Self {
        Self {
            foreground,
            background_color,
        }
    }
}

impl<T, C> ViewMarker for HintBackground<T, C>
where
    T: ViewMarker,
    C: Clone,
{
    type Renderables = render::HintBackground<T::Renderables, C>;
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T, C> ViewLayout<Captures> for HintBackground<T, C>
where
    T: ViewLayout<Captures>,
    C: Clone,
{
    type Sublayout = T::Sublayout;
    type State = T::State;

    fn priority(&self) -> i8 {
        self.foreground.priority()
    }

    fn is_empty(&self) -> bool {
        self.foreground.is_empty()
    }

    fn transition(&self) -> Self::Transition {
        self.foreground.transition()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.foreground.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.foreground.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        render::HintBackground::new(
            self.foreground
                .render_tree(layout, origin, env, captures, state),
            self.background_color.clone(),
        )
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        // Background can't handle events
        self.foreground
            .handle_event(event, context, &mut render_tree.subtree, captures, state)
    }
}
