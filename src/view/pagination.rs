use crate::{
    environment::LayoutEnvironment,
    event::{
        Event, EventContext, EventResult,
        input::{FocusState, Groups, Input, Interaction},
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

/// Represents semantic action made by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaginationAction {
    /// Request to paginate "Forward".
    Next,
    /// Request to paginate "Backwards".
    Previous,
    /// Emitted when focus gets captured inside the pagination, when
    /// `click_to_enter` is enabled.
    Enter,
    /// When `click_to_exit` then when emitted before exiting the pagination.
    /// When not enabled, emitted when user clicks inside the pagination.
    Submit,
    /// Emitted when user cancels pagination, e.g. by issuing `Cancel` event.
    Escape,
}

#[derive(Debug, Clone)]
pub struct PaginationView<ViewFn, Action> {
    view: ViewFn,
    action: Action,
    direction: PaginationDirection,
    click_to_enter: bool,
    click_to_exit: bool,
    groups: Groups,
}

impl Pagination {
    /// Constructs new horizontal [`Pagination`]
    pub fn new_horizontal<ViewFn, Action, Captures>(
        groups: impl Into<Groups>,
        on_action: Action,
        view: ViewFn,
    ) -> PaginationView<ViewFn, Action> {
        PaginationView {
            action: on_action,
            direction: PaginationDirection::Horizontal,
            groups: groups.into(),
            click_to_enter: false,
            click_to_exit: false,
            view,
        }
    }

    /// Constructs new vertical [`Pagination`]
    pub fn new_vertical<ViewFn, Action, Captures>(
        groups: impl Into<Groups>,
        on_action: Action,
        view: ViewFn,
    ) -> PaginationView<ViewFn, Action> {
        PaginationView {
            action: on_action,
            direction: PaginationDirection::Vertical,
            groups: groups.into(),
            click_to_enter: false,
            click_to_exit: false,
            view,
        }
    }
}

impl<ViewFn, Action> PaginationView<ViewFn, Action> {
    pub fn with_click_to_enter(mut self, click_to_enter: bool) -> Self {
        self.click_to_enter = click_to_enter;
        self
    }
    pub fn with_click_to_exit(mut self, click_to_exit: bool) -> Self {
        self.click_to_exit = click_to_exit;
        self
    }
    fn interaction<T>(&self, state: &PaginationState<T>) -> Interaction {
        let focused = state.focus.is_focused_any(self.groups);
        Interaction::new().with(focused, Interaction::FOCUSED)
    }
}

impl<V, ViewFn, Action> ViewMarker for PaginationView<ViewFn, Action>
where
    ViewFn: Fn(Interaction) -> V,
    V: ViewMarker,
{
    type Renderables = V::Renderables;
    // TODO: slide between pages
    type Transition = Opacity;
}

impl<V, ViewFn, Action, Captures> ViewLayout<Captures> for PaginationView<ViewFn, Action>
where
    Action: Fn(PaginationAction, &Input<'_>, &mut Captures),
    ViewFn: Fn(Interaction) -> V,
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
            inner: (self.view)(Interaction::new()).build_state(captures),
        }
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let interaction = self.interaction(state);
        (self.view)(interaction).layout(offer, env, captures, &mut state.inner)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let interaction = self.interaction(state);
        (self.view)(interaction).render_tree(layout, origin, env, captures, &mut state.inner)
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

        let interaction = self.interaction(&state);
        let focus = &mut state.focus;
        let entered = &mut state.entered;
        let input = context.input;

        if let Event::Keyboard(k) = event {
            if self.click_to_enter && !entered.is_focused_any(k.groups) {
                return if k.kind.is_movement() {
                    context.input.leaf_move(focus, k.groups)
                } else if k.kind == K::Click {
                    (self.action)(PaginationAction::Enter, input, captures);
                    entered.focus(k.groups);
                    EventResult::new(true, true)
                } else {
                    // Not movement nor click, be transparent
                    let view = (self.view)(interaction);
                    view.handle_event(event, context, render_tree, captures, &mut state.inner)
                };
            }

            // Assumption: this is keyboard event and we already entered.

            match (self.direction, k.kind) {
                (_, K::Click) if self.click_to_exit => {
                    (self.action)(PaginationAction::Submit, input, captures);
                    entered.blur(k.groups);
                    context.input.blur(state.observed_groups);
                    state.observed_groups = Groups::default();
                    return handled;
                }
                (_, K::Cancel | K::LongPress) if self.click_to_enter => {
                    (self.action)(PaginationAction::Escape, input, captures);
                    let view = (self.view)(interaction);
                    let result =
                        view.handle_event(event, context, render_tree, captures, &mut state.inner);
                    return if result.handled {
                        result
                    } else {
                        entered.blur(k.groups);
                        context.input.blur(state.observed_groups);
                        state.observed_groups = Groups::default();
                        handled
                    };
                }
                (D::Vertical, K::Up) | (D::Horizontal, K::Left) => {
                    context.input.blur(state.observed_groups);
                    state.observed_groups = Groups::default();
                    (self.action)(PaginationAction::Previous, input, captures);
                    return handled;
                }
                (D::Vertical, K::Down) | (D::Horizontal, K::Right) => {
                    context.input.blur(state.observed_groups);
                    state.observed_groups = Groups::default();
                    (self.action)(PaginationAction::Next, input, captures);
                    return handled;
                }
                _ => (),
                /* fallthrough */
            }
        }

        // Assumption: it is either not keyboard event, or we are already entered into one.

        let view = (self.view)(interaction);
        let mut result = view.handle_event(event, context, render_tree, captures, &mut state.inner);
        if result.handled {
            state.observed_groups |= event.groups();
        }

        if let Event::Keyboard(k) = event {
            if k.kind == K::Click {
                (self.action)(PaginationAction::Submit, input, captures);
            }
            if k.kind.is_movement() {
                result.handled = true;
            }
        }

        result
    }
}
