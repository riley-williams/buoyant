use buoyant::{
    environment::LayoutEnvironment,
    event::{
        Event, EventContext, EventResult,
        input::{FocusState, Groups},
    },
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

#[derive(Default, Debug)]
pub struct State<T> {
    entered: FocusState,
    inner: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableProxyAction {
    Enter,
    Escape,
}

#[derive(Debug, Clone)]
pub struct TableMovementProxy<V, Action> {
    view: V,
    action: Action,
    groups: Groups,
}

impl<V, Action> TableMovementProxy<V, Action> {
    pub fn new(groups: impl Into<Groups>, on_action: Action, view: V) -> Self {
        Self {
            action: on_action,
            groups: groups.into(),
            view,
        }
    }
}

impl<V, Action> ViewMarker for TableMovementProxy<V, Action>
where
    V: ViewMarker,
{
    type Renderables = V::Renderables;
    type Transition = Opacity;
}

impl<V, Action, Captures> ViewLayout<Captures> for TableMovementProxy<V, Action>
where
    Action: Fn(TableProxyAction, &mut Captures),
    V: ViewLayout<Captures>,
    Captures: ?Sized,
{
    type State = State<V::State>;
    type Sublayout = V::Sublayout;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        State {
            entered: FocusState::new(self.groups),
            inner: self.view.build_state(captures),
        }
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.view.layout(offer, env, captures, &mut state.inner)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.view
            .render_tree(layout, origin, env, captures, &mut state.inner)
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        use buoyant::event::keyboard::KeyboardEventKind as K;

        let entered = &mut state.entered;
        let groups = event.groups() & context.input.active_groups();
        let should_handle = entered.is_member_of_any(groups);
        let is_click = match event {
            Event::Keyboard(k) => k.kind == K::Click,
            Event::Touch(_t) => todo!("Behave like a button"),
            _ => false,
        };
        let is_movement = match event {
            Event::Keyboard(k) => k.kind.is_movement(),
            _ => false,
        };

        let is_entered = entered.is_focused_any(self.groups);
        std::println!("Proxy. Entered: {is_entered}, groups: {groups:?}");

        if !is_entered && (is_click || is_movement) {
            (self.action)(TableProxyAction::Enter, captures);
            entered.focus(self.groups);
            return EventResult::new(true, true, false);
        }

        if let Event::Keyboard(k) = event
            && should_handle
            && (k.kind == K::Cancel || k.kind == K::LongPress)
        {
            let result =
                self.view
                    .handle_event(event, context, render_tree, captures, &mut state.inner);

            if result.handled {
                return result;
            }

            (self.action)(TableProxyAction::Escape, captures);
            entered.blur(groups);
            return result.handled();
        } else {
            return self
                .view
                .handle_event(event, context, render_tree, captures, &mut state.inner);
        }
    }
}
