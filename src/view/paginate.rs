use core::marker::PhantomData;

use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventContext, EventResult},
    focus::{ContentShape, DefaultFocus, FocusAction, FocusEvent, FocusStateChange},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

#[derive(Clone, Debug)]
pub struct Paginate<V, ViewFn, Action> {
    _view: PhantomData<V>,
    view_fn: ViewFn,
    action: Action,
}

#[derive(Clone, Debug)]
pub enum PageEvent {
    Focused,
    Next,
    Previous,
    Exit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageState {
    UnFocused,
    Focused,
    Captive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageFocus(bool);

impl PageFocus {
    fn is_captive(self) -> bool {
        self.0
    }
}

impl DefaultFocus for PageFocus {
    fn default_first() -> Self {
        Self(false)
    }

    fn default_last() -> Self {
        Self(false)
    }
}

impl<V: ViewMarker, ViewFn: Fn(&PageState) -> V, Action> Paginate<V, ViewFn, Action> {
    pub fn new<C>(action: Action, view_fn: ViewFn) -> Self
    where
        V: ViewLayout<C>,
        Action: Fn(&mut C, &PageEvent),
    {
        Self {
            _view: PhantomData,
            view_fn,
            action,
        }
    }
}

impl<V: ViewMarker, ViewFn, Action> ViewMarker for Paginate<V, ViewFn, Action> {
    type Renderables = V::Renderables;
    type Transition = Opacity;
}

impl<C, V, ViewFn, Action> ViewLayout<C> for Paginate<V, ViewFn, Action>
where
    V: ViewLayout<C>,
    ViewFn: Fn(&PageState) -> V,
    Action: Fn(&mut C, &PageEvent),
{
    // FIXME: Shouldn't have to sync here
    type State = (PageState, V::State);

    type Sublayout = V::Sublayout;

    type FocusTree = PageFocus;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut C) -> Self::State {
        let s = PageState::UnFocused;
        let view = (self.view_fn)(&s);
        (s, view.build_state(captures))
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut C,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        // FIXME: Pass focus in layout to avoid state sync?
        let view = (self.view_fn)(&state.0);
        view.layout(offer, env, captures, &mut state.1)
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut C,
        state: &mut Self::State,
    ) -> Self::Renderables {
        // FIXME: Pass focus in render to avoid state sync?
        let view = (self.view_fn)(&state.0);
        view.render_tree(layout, origin, env, captures, &mut state.1)
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut C,
        state: &mut Self::State,
    ) -> EventResult {
        let view = (self.view_fn)(&state.0);
        view.handle_event(event, context, render_tree, captures, &mut state.1)
    }

    fn focus(
        &self,
        event: &FocusEvent,
        _context: &EventContext,
        _render_tree: &mut Self::Renderables,
        captures: &mut C,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> FocusStateChange {
        // TODO: ContentShape for inner render tree
        let focused_result = FocusStateChange::Focused {
            shape: ContentShape::Empty,
            result: EventResult::new(true, true, true),
        };

        if focus.is_captive() {
            match event.action {
                FocusAction::Next => {
                    (self.action)(captures, &PageEvent::Next);
                    focused_result
                }
                FocusAction::Previous => {
                    (self.action)(captures, &PageEvent::Previous);
                    focused_result
                }
                FocusAction::Focus(_) => {
                    state.0 = PageState::Captive;
                    focused_result
                }
                FocusAction::Blur | FocusAction::Select => {
                    // FIXME: Customizable exit on select?
                    (self.action)(captures, &PageEvent::Exit);
                    focus.0 = false;
                    state.0 = PageState::Focused;
                    focused_result
                }
            }
        } else {
            match event.action {
                FocusAction::Next | FocusAction::Previous | FocusAction::Blur => {
                    state.0 = PageState::UnFocused;
                    // FIXME: Because this doesn't contain event result,
                    // we can't inform the render loop that state changed
                    FocusStateChange::Exhausted
                }
                FocusAction::Focus(_) => {
                    state.0 = PageState::Focused;
                    focused_result
                }
                FocusAction::Select => {
                    (self.action)(captures, &PageEvent::Focused);
                    focus.0 = true;
                    state.0 = PageState::Captive;
                    focused_result
                }
            }
        }
    }
}
