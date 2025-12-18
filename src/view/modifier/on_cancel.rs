use crate::{
    environment::LayoutEnvironment,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug)]
pub struct OnCancel<V, F>(V, F);

impl<V, F> OnCancel<V, F> {
    pub const fn new(value: V, func: F) -> Self {
        OnCancel(value, func)
    }
}

impl<V: ViewMarker, F> ViewMarker for OnCancel<V, F> {
    type Renderables = V::Renderables;
    type Transition = V::Transition;
}

impl<Captures: ?Sized, V: ViewLayout<Captures>, F: Fn(&mut Captures)> ViewLayout<Captures>
    for OnCancel<V, F>
{
    type Sublayout = V::Sublayout;
    type State = V::State;

    fn priority(&self) -> i8 {
        self.0.priority()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn transition(&self) -> Self::Transition {
        self.0.transition()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.0.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.0.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.0.render_tree(layout, origin, env, captures, state)
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> crate::event::EventResult {
        use crate::event::{Event, keyboard::KeyboardEventKind as Kind};

        let mut result = self.0.handle_event(event, context, tree, captures, state);

        if let Event::Keyboard(k) = event {
            if !result.handled && k.kind == Kind::Cancel {
                (self.1)(captures);
                result.handled = true;
            }
        }

        result
    }
}
