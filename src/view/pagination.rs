use crate::{
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

/// A pagination component
#[expect(missing_debug_implementations)]
pub struct Pagination;

/// Direction of pagination
#[derive(Default, Debug, Clone, Copy)]
pub enum PaginationDirection {
    /// Horizonaly paginate with `Left` and `Right` buttons
    #[default]
    Horizontal,
    /// Verticaly paginate with `Up` and `Down` buttons
    Vertical,
}

#[derive(Default, Debug)]
pub struct PaginationState<T> {
    observed_groups: Groups,
    focus: FocusState,
    entered: FocusState,
    inner: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaginationAction {
    Next,
    Previous,
    Enter,
    Submit,
    Escape,
}

#[derive(Debug, Clone)]
pub struct PaginationView<V, Action> {
    view: V,
    action: Action,
    direction: PaginationDirection,
    click_to_enter: bool,
    groups: Groups,
}

impl Pagination {
    /// Constructs new horizontal [`Pagination`]
    pub fn new_horizontal<V, Action, Captures>(
        groups: impl Into<Groups>,
        on_action: Action,
        view: V,
    ) -> PaginationView<V, Action> {
        PaginationView {
            action: on_action,
            direction: PaginationDirection::Horizontal,
            groups: groups.into(),
            click_to_enter: false,
            view,
        }
    }

    /// Constructs new vertical [`Pagination`]
    pub fn new_vertical<V, Action, Captures>(
        groups: impl Into<Groups>,
        on_action: Action,
        view: V,
    ) -> PaginationView<V, Action> {
        PaginationView {
            action: on_action,
            direction: PaginationDirection::Vertical,
            groups: groups.into(),
            click_to_enter: false,
            view,
        }
    }
}

impl<V, Action> PaginationView<V, Action> {
    pub fn with_click_to_enter(mut self, click_to_enter: bool) -> Self {
        self.click_to_enter = click_to_enter;
        self
    }
}

impl<V, Action> ViewMarker for PaginationView<V, Action>
where
    V: ViewMarker,
{
    type Renderables = V::Renderables;
    // TODO: slide between pages
    type Transition = Opacity;
}

impl<V, Action, Captures> ViewLayout<Captures> for PaginationView<V, Action>
where
    Action: Fn(PaginationAction, &mut Captures),
    V: ViewLayout<Captures>,
    Captures: ?Sized,
{
    type State = PaginationState<V::State>;
    type Sublayout = V::Sublayout;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        PaginationState {
            observed_groups: Groups::default(),
            focus: FocusState::new(self.groups),
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
        use crate::event::keyboard::KeyboardEventKind as K;
        use PaginationDirection as D;

        let handled = EventResult::new(true, false);

        let focus = &mut state.focus;
        let entered = &mut state.entered;

        if let Event::Keyboard(k) = event {
            if self.click_to_enter && !entered.is_focused_any(k.groups) {
                return if k.kind.is_movement() {
                    context.input.leaf_move(focus, k.groups)
                } else if k.kind == K::Click {
                    (self.action)(PaginationAction::Enter, captures);
                    entered.focus(k.groups);
                    EventResult::new(true, true)
                } else {
                    let result = self.view.handle_event(
                        event,
                        context,
                        render_tree,
                        captures,
                        &mut state.inner,
                    );
                    if result.handled {
                        state.observed_groups |= event.groups();
                    }
                    result
                };
            }

            // Match assumes that we already entered.

            match (self.direction, k.kind) {
                (_, K::Cancel | K::LongPress) if self.click_to_enter => {
                    (self.action)(PaginationAction::Escape, captures);
                    let result = self.view.handle_event(
                        event,
                        context,
                        render_tree,
                        captures,
                        &mut state.inner,
                    );
                    return if result.handled {
                        result
                    } else {
                        entered.blur(k.groups);
                        handled
                    };
                }
                (D::Vertical, K::Up) | (D::Horizontal, K::Left) => {
                    context.input.blur(state.observed_groups);
                    state.observed_groups = Groups::default();
                    (self.action)(PaginationAction::Previous, captures);
                    return handled;
                }
                (D::Vertical, K::Down) | (D::Horizontal, K::Right) => {
                    context.input.blur(state.observed_groups);
                    state.observed_groups = Groups::default();
                    (self.action)(PaginationAction::Next, captures);
                    return handled;
                }
                _ => (),
            }
        }

        let result =
            self.view
                .handle_event(event, context, render_tree, captures, &mut state.inner);
        if result.handled {
            state.observed_groups |= event.groups();
        }

        if let Event::Keyboard(k) = event && k.kind == K::Click {
            (self.action)(PaginationAction::Submit, captures);
        }

        result
    }
}
