use crate::{
    environment::LayoutEnvironment,
    event::{
        Event, EventContext, EventResult,
        input::{FocusState, Groups, Interaction},
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
    /// Vertically paginate with `Up` and `Down` buttons
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
    /// When `click_to_exit` then when emitted before successfully exiting the pagination.
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
    reroute_navigation: bool,
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
            reroute_navigation: false,
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
            reroute_navigation: false,
            view,
        }
    }
}

impl<ViewFn, Action> PaginationView<ViewFn, Action> {
    pub fn click_to_enter(mut self, click_to_enter: bool) -> Self {
        self.click_to_enter = click_to_enter;
        self
    }
    pub fn click_to_exit(mut self, click_to_exit: bool) -> Self {
        self.click_to_exit = click_to_exit;
        self
    }
    pub fn reroute_navigation(mut self, reroute: bool) -> Self {
        self.reroute_navigation = reroute;
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
    Action: Fn(PaginationAction, &mut Captures),
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

    #[allow(clippy::too_many_lines)]
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

        let mut result = EventResult::default();

        let handled = EventResult::new(true, false);
        let interaction = self.interaction(state);
        let focus = &mut state.focus;
        let entered = &mut state.entered;
        let groups = event.groups() & context.input.active_groups();
        let should_handle = entered.is_member_of_any(groups);
        let is_click = match event {
            Event::Keyboard(k) => k.kind == K::Click,
            Event::Touch(_t) => todo!("Behave like a button"),
            _ => false,
        };

        let is_entered = entered.is_focused_any(groups);

        if let Event::Keyboard(k) = event
            && k.kind.is_movement()
            && should_handle
            && !is_entered
        {
            let is_across = matches!(
                (self.direction, k.kind),
                (D::Vertical, K::Right | K::Left) | (D::Horizontal, K::Up | K::Down),
            );
            let is_along = matches!(
                (self.direction, k.kind),
                (D::Vertical, K::Up | K::Down) | (D::Horizontal, K::Left | K::Right),
            );

            assert!(
                is_across != is_along,
                "Movement can be either across or along pagination direction"
            );

            if !self.click_to_enter {
                entered.focus(groups);
                if is_along && !self.reroute_navigation {
                    result.merge(context.input.leaf_move(focus, k.groups));
                }
            }

            if self.click_to_enter || (is_across && !self.reroute_navigation) {
                return context.input.leaf_move(focus, k.groups);
            }
        }

        let is_entered = entered.is_focused_any(groups);

        if is_click && self.click_to_enter && !is_entered && should_handle {
            (self.action)(PaginationAction::Enter, captures);
            entered.focus(groups);
            // Like, that enter couldn't unfocus previous focus and would've anyway
            // went there, so if blurred, then focus there.
            if !context.input.is_focused_any(self.groups) {
                result.merge(context.input.leaf_move(focus, event.groups()));
            }
            return result.merging(EventResult::new(true, true));
        }

        if let Event::Keyboard(k) = event
            && (!self.click_to_enter || is_entered)
            && should_handle
        {
            match (self.direction, k.kind) {
                (D::Vertical, K::Up) | (D::Horizontal, K::Left) => {
                    (self.action)(PaginationAction::Previous, captures);
                    context.input.blur(state.observed_groups);
                    state.observed_groups = Groups::default();
                    return handled.handled();
                }
                (D::Vertical, K::Down) | (D::Horizontal, K::Right) => {
                    (self.action)(PaginationAction::Next, captures);
                    context.input.blur(state.observed_groups);
                    state.observed_groups = Groups::default();
                    return handled.handled();
                }
                (D::Vertical, K::Right | K::Left) | (D::Horizontal, K::Up | K::Down)
                    if focus.is_focused_any(groups) && self.reroute_navigation =>
                {
                    _ = context.input.leaf_move(focus, groups);
                }
                _ => (),
            }
        }

        let view = (self.view)(interaction);
        let mut result = view.handle_event(event, context, render_tree, captures, &mut state.inner);
        if result.handled {
            state.observed_groups |= groups;
        }

        if let Event::Keyboard(k) = event
            && !self.click_to_enter
            && !result.handled
            && should_handle
            && matches!(
                (self.direction, k.kind),
                (D::Vertical, K::Left | K::Right) | (D::Horizontal, K::Up | K::Down)
            )
        {
            entered.blur(groups);
            return result.merging(context.input.leaf_move(focus, k.groups));
        }

        if is_click && is_entered && !result.handled && self.click_to_exit && should_handle {
            (self.action)(PaginationAction::Submit, captures);
            entered.blur(groups);
            context.input.blur(state.observed_groups);
            state.observed_groups = Groups::default();
            return result.handled();
        }

        if result.handled {
            return result;
        }

        if let Event::Keyboard(k) = event
            && (!self.click_to_enter || is_entered)
            && should_handle
        {
            match (self.direction, k.kind) {
                (_, K::Cancel | K::LongPress) if self.click_to_enter => {
                    (self.action)(PaginationAction::Escape, captures);
                    entered.blur(groups);
                    context.input.blur(state.observed_groups);
                    state.observed_groups = Groups::default();
                    return result.handled();
                }
                _ => (),
            }
        }

        if let Event::Keyboard(k) = event
            && k.kind.is_movement()
            && self.click_to_enter
            && should_handle
        {
            result.handled = true;
        }

        result
    }
}

// TODO: This need a lot of tests, there are a lot of edge cases.
