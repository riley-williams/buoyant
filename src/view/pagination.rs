use core::num::NonZeroUsize;

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
    active_index: usize,
    inner: T,
}

/// Trait used for indexing into the pagination
pub trait PaginationIndex<'a> {
    /// The output type when indexing into the pagination
    type Output: 'a;

    /// Retuns the number of paginated items
    fn len(&self) -> NonZeroUsize;

    /// Returns the data needed to display page `i`
    fn index(&self, i: usize) -> Self::Output;
}

#[derive(Debug, Clone)]
pub struct PaginationView<'a, M, V, F> {
    items: M,
    build_view: F,
    direction: PaginationDirection,
    transparent_on_click: bool,
    groups: Groups,
    carousel: bool,
    _marker: core::marker::PhantomData<(V, &'a ())>,
}

impl Pagination {
    /// Constructs new horizontal [`Pagination`]
    pub fn new_horizontal<'a, M, V, F>(
        groups: impl Into<Groups>,
        items: M,
        build_view: F,
    ) -> PaginationView<'a, M, V, F>
    where
        M: PaginationIndex<'a>,
        F: Fn(M::Output) -> V,
    {
        PaginationView {
            items,
            build_view,
            direction: PaginationDirection::Horizontal,
            groups: groups.into(),
            transparent_on_click: false,
            carousel: false,
            _marker: core::marker::PhantomData,
        }
    }

    /// Constructs new vertical [`Pagination`]
    pub fn new_vertical<'a, M, V, F>(
        groups: impl Into<Groups>,
        items: M,
        build_view: F,
    ) -> PaginationView<'a, M, V, F>
    where
        M: PaginationIndex<'a>,
        F: Fn(M::Output) -> V,
    {
        PaginationView {
            items,
            build_view,
            direction: PaginationDirection::Vertical,
            groups: groups.into(),
            transparent_on_click: false,
            carousel: false,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<'a, M, V, F> PaginationView<'a, M, V, F> {
    /// Horizontal pagination will consume `Left` and `Right` keyboard events to
    /// navigate, vertical will consume `Up` and `Down`, will be transparent to clicks etc.
    /// With transparent on click enabled, first click will be consumed and then all buttons except
    /// `Cancel` will be passed to the child views. `Cancel` will "exit" from the navigation.
    pub fn with_transparent_on_click(mut self, transparent_on_click: bool) -> Self {
        self.transparent_on_click = transparent_on_click;
        self
    }
    pub fn with_carousel(mut self, carousel: bool) -> Self {
        self.carousel = carousel;
        self
    }
}

impl<'a, M, V, F> ViewMarker for PaginationView<'a, M, V, F>
where
    V: ViewMarker,
{
    type Renderables = V::Renderables;
    // TODO: slide between pages
    type Transition = Opacity;
}

impl<'a, M, V, F, Captures> ViewLayout<Captures> for PaginationView<'a, M, V, F>
where
    M: PaginationIndex<'a>,
    F: Fn(M::Output) -> V,
    V: ViewLayout<Captures>,
    Captures: ?Sized,
{
    type State = PaginationState<V::State>;
    type Sublayout = V::Sublayout;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        let view = (self.build_view)(self.items.index(0));
        PaginationState {
            observed_groups: Groups::default(),
            focus: FocusState::new(self.groups),
            active_index: 0,
            inner: view.build_state(captures),
        }
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let view = (self.build_view)(self.items.index(state.active_index));
        view.layout(offer, env, captures, &mut state.inner)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let view = (self.build_view)(self.items.index(state.active_index));
        view.render_tree(layout, origin, env, captures, &mut state.inner)
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

        // std::println!("Event!: {event:?}");

        if let Event::Keyboard(k) = event {
            match (self.direction, k.kind) {
                (D::Horizontal, K::Click)
                    if focus.should_focus(k.groups) && self.transparent_on_click =>
                {
                    context.input.focus(focus.focus(k.groups));
                    return handled;
                }
                (D::Horizontal, K::Cancel)
                    if focus.should_blur(k.groups) && self.transparent_on_click =>
                {
                    context.input.blur(focus.blur(k.groups));
                    return handled;
                }
                (D::Vertical, K::Up) | (D::Horizontal, K::Left)
                    if !focus.is_focused_any(k.groups) && focus.is_member_of_any(k.groups) =>
                {
                    let next_index = if state.active_index == 0 {
                        self.carousel.then_some(self.items.len().get() - 1)
                    } else {
                        Some(state.active_index - 1)
                    };
                    if let Some(i) = next_index {
                        context.input.blur(state.observed_groups);
                        state.active_index = i;
                        let view = (self.build_view)(self.items.index(state.active_index));
                        state.inner = view.build_state(captures);
                    }
                    return handled;
                }
                (D::Vertical, K::Down) | (D::Horizontal, K::Right)
                    if !focus.is_focused_any(k.groups) && focus.is_member_of_any(k.groups) =>
                {
                    let next_index = if state.active_index + 1 == self.items.len().get() {
                        self.carousel.then_some(0)
                    } else {
                        Some(state.active_index + 1)
                    };
                    if let Some(i) = next_index {
                        context.input.blur(state.observed_groups);
                        state.active_index = i;
                        let view = (self.build_view)(self.items.index(state.active_index));
                        state.inner = view.build_state(captures);
                    }
                    return handled;
                }
                _ => (),
            }
        }

        let view = (self.build_view)(self.items.index(state.active_index));
        let result = view.handle_event(event, context, render_tree, captures, &mut state.inner);
        if result.handled {
            state.observed_groups |= event.groups();
        }
        result
    }
}
