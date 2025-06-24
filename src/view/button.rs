use core::marker::PhantomData;

use crate::{
    environment::LayoutEnvironment,
    layout::ResolvedLayout,
    primitives::{Frame, ProposedDimensions},
    render::Container,
    view::{Event, ViewLayout, ViewMarker},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    CaptivePressed,
    Captive,
    AtRest,
}

#[derive(Debug)]
pub struct Button<ViewFn, Inner, Action> {
    _inner_marker: PhantomData<Inner>,
    view: ViewFn,
    action: Action,
}

impl<ViewFn, Inner, Action> Button<ViewFn, Inner, Action> {
    pub fn new(action: Action, view: ViewFn) -> Self {
        Self {
            view,
            action,
            _inner_marker: PhantomData,
        }
    }
}

impl<ViewFn, Inner: ViewMarker, Action> ViewMarker for Button<ViewFn, Inner, Action> {
    type Renderables = Container<Inner::Renderables>;
}

impl<Captures, Inner, ViewFn, Action> ViewLayout<Captures> for Button<ViewFn, Inner, Action>
where
    Action: Fn(&mut Captures),
    Captures: ?Sized,
    Inner: ViewLayout<Captures>,
    ViewFn: Fn(bool) -> Inner,
{
    type State = (ButtonState, Inner::State);
    type Sublayout = Inner::Sublayout;

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        (
            ButtonState::AtRest,
            (self.view)(false).build_state(captures),
        )
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        match state.0 {
            ButtonState::CaptivePressed => {
                (self.view)(true).layout(offer, env, captures, &mut state.1)
            }
            ButtonState::AtRest | ButtonState::Captive => {
                (self.view)(false).layout(offer, env, captures, &mut state.1)
            }
        }
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        Container::new(
            Frame::new(origin, layout.resolved_size.into()),
            match state.0 {
                ButtonState::CaptivePressed => {
                    (self.view)(true).render_tree(layout, origin, env, captures, &mut state.1)
                }
                ButtonState::AtRest | ButtonState::Captive => {
                    (self.view)(false).render_tree(layout, origin, env, captures, &mut state.1)
                }
            },
        )
    }

    fn handle_event(
        &mut self,
        event: &Event,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> bool {
        match event {
            Event::TouchDown(point) => {
                if render_tree.frame.contains(point) {
                    state.0 = ButtonState::CaptivePressed;
                    true
                } else {
                    false
                }
            }
            Event::TouchUp(point) => {
                if render_tree.frame.contains(point) && state.0 == ButtonState::CaptivePressed {
                    (self.action)(captures);
                    state.0 = ButtonState::AtRest;
                    true
                } else {
                    state.0 = ButtonState::AtRest;
                    false
                }
            }
            Event::TouchMoved(point) => match (render_tree.frame.contains(point), state.0) {
                (true, ButtonState::Captive) => {
                    state.0 = ButtonState::CaptivePressed;
                    true
                }
                (false, ButtonState::CaptivePressed) => {
                    state.0 = ButtonState::Captive;
                    true
                }
                (true, ButtonState::CaptivePressed) | (false, ButtonState::Captive) => true,
                (_, ButtonState::AtRest) => false,
            },
            _ => false,
        }
    }
}
