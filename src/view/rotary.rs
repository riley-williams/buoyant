//! A view that captures focus on select and modifies state in response to
//! focus navigation events while captive.

use core::marker::PhantomData;

use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventContext, EventResult},
    focus::{DefaultFocus, FocusAction, Role},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    render::IntrinsicShape,
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

/// A view that captures focus on select and modifies state in response to
/// focus navigation events while captive.
///
/// This view is classified as a [`Button`][`crate::focus::Role::Button`].
///
/// Touch interaction with this view is limited to moving focus to the element.
/// When the Rotary is in the captive state, tapping off of it will trigger an
/// [`Exit`][`RotaryEvent::Exit`] event.
///
/// Examples
///
/// A view that displays a count and increments or decrements it in response to rotary
/// events. The count border changes color based on the state of focus:
///
/// ```
/// use embedded_graphics::pixelcolor::Rgb888;
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::mono_font::ascii::FONT_6X10;
/// use buoyant::view::prelude::*;
/// use buoyant::view::rotary::{RotaryEvent, RotaryState};
///
/// fn counter(count: u32) -> impl View<Rgb888, u32> + use<> {
///     Rotary::new(
///         |count: &mut u32, event: RotaryEvent| match event {
///             RotaryEvent::Next => *count += 1,
///             RotaryEvent::Previous => *count= 1,
///             RotaryEvent::Select | RotaryEvent::Exit => {}
///         },
///         move |rotary_state| {
///             Text::new_fmt::<12>(format_args!("{count}"), &FONT_6X10)
///                 .padding(Edges::All, 4)
///                 .background(Alignment::Center, {
///                     let color = match rotary_state {
///                         RotaryState::UnFocused => Rgb888::BLACK,
///                         RotaryState::Focused => Rgb888::WHITE,
///                         RotaryState::Captive => Rgb888::GREEN,
///                     };
///                     RoundedRectangle::new(4)
///                         .stroked(2)
///                         .foreground_color(color)
///                 })
///         },
///     )
/// }
/// ```

#[derive(Clone, Debug)]
pub struct Rotary<V, ViewFn, Action> {
    _view: PhantomData<V>,
    view_fn: ViewFn,
    action: Action,
}

/// An event emitted by a [`Rotary`] when it captive.
#[derive(Clone, Copy, Debug)]
pub enum RotaryEvent {
    /// A [`Next`][`FocusAction::Next`] event occurred while the rotary was captive
    /// focused.
    Next,
    /// A [`Previous`][`FocusAction::Previous`] event occurred while the rotary was captive
    /// focused.
    Previous,
    /// A [`Select`][`FocusAction::Select`] event occurred while the rotary was captive focused.
    /// The rotary will stay focused but will no longer be captive after this event.
    Select,
    /// A [`Blur`][`FocusAction::Blur`] event occurred while focused, typically from a "back" or "menu" button press.
    /// The rotary will stay focused but will no longer be captive after this event.
    Exit,
}

#[expect(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RotaryState {
    UnFocused,
    Focused,
    Captive,
}

#[expect(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RotaryFocus(bool);

impl RotaryFocus {
    fn is_captive(self) -> bool {
        self.0
    }
}

impl DefaultFocus for RotaryFocus {
    fn default_first() -> Self {
        Self(false)
    }

    fn default_last() -> Self {
        Self(false)
    }
}

impl<V: ViewMarker, ViewFn: Fn(RotaryState) -> V, Action> Rotary<V, ViewFn, Action> {
    #[expect(missing_docs)]
    pub fn new<C>(action: Action, view_fn: ViewFn) -> Self
    where
        V: ViewLayout<C>,
        Action: Fn(&mut C, RotaryEvent),
    {
        Self {
            _view: PhantomData,
            view_fn,
            action,
        }
    }
}

impl<V: ViewMarker, ViewFn, Action> ViewMarker for Rotary<V, ViewFn, Action> {
    type Renderables = V::Renderables;
    type Transition = Opacity;
}

impl<C, V, ViewFn, Action> ViewLayout<C> for Rotary<V, ViewFn, Action>
where
    V: ViewLayout<C, Renderables: IntrinsicShape>,
    ViewFn: Fn(RotaryState) -> V,
    Action: Fn(&mut C, RotaryEvent),
{
    // FIXME: Shouldn't have to sync here
    type State = (RotaryState, V::State);

    type Sublayout = V::Sublayout;

    type FocusTree = RotaryFocus;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut C) -> Self::State {
        let s = RotaryState::UnFocused;
        let view = (self.view_fn)(s);
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
        let view = (self.view_fn)(state.0);
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
        let view = (self.view_fn)(state.0);
        view.render_tree(layout, origin, env, captures, &mut state.1)
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut C,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> EventResult {
        if let Event::Focus {
            action: focus_event,
            ..
        } = event
        {
            if !context.roles.contains(Role::Button) {
                return EventResult::Deferred;
            }
            let focused_shape = render_tree.content_shape();

            return if focus.is_captive() {
                match focus_event {
                    FocusAction::Next => {
                        (self.action)(captures, RotaryEvent::Next);
                        // If in the future we track state changes more granularly we could maybe
                        // avoid rebuilding the view on every rotary event. But likely most events
                        // change state anyways.
                        context.request_view_rebuild();
                        EventResult::handled_focused(focused_shape)
                    }
                    FocusAction::Previous => {
                        (self.action)(captures, RotaryEvent::Previous);
                        context.request_view_rebuild();
                        EventResult::handled_focused(focused_shape)
                    }
                    FocusAction::Focus(_) => {
                        state.0 = RotaryState::Captive;
                        EventResult::handled_focused(focused_shape)
                    }
                    FocusAction::Teardown => {
                        (self.action)(captures, RotaryEvent::Exit);
                        focus.0 = false;
                        state.0 = RotaryState::UnFocused;
                        context.request_view_rebuild();
                        EventResult::handled_focused(focused_shape)
                    }
                    FocusAction::Select => {
                        (self.action)(captures, RotaryEvent::Select);
                        focus.0 = false;
                        state.0 = RotaryState::Focused;
                        context.request_view_rebuild();
                        EventResult::handled_focused(focused_shape)
                    }
                    FocusAction::Blur => {
                        (self.action)(captures, RotaryEvent::Exit);
                        focus.0 = false;
                        state.0 = RotaryState::Focused;
                        context.request_view_rebuild();
                        EventResult::handled_focused(focused_shape)
                    }
                }
            } else {
                match focus_event {
                    FocusAction::Next
                    | FocusAction::Previous
                    | FocusAction::Blur
                    | FocusAction::Teardown => {
                        state.0 = RotaryState::UnFocused;
                        context.request_view_rebuild();
                        EventResult::Deferred
                    }
                    FocusAction::Focus(_) => {
                        if state.0 != RotaryState::Focused {
                            state.0 = RotaryState::Focused;
                            context.request_view_rebuild();
                        }
                        EventResult::handled_focused(focused_shape)
                    }
                    FocusAction::Select => {
                        focus.0 = true;
                        state.0 = RotaryState::Captive;
                        context.request_view_rebuild();
                        EventResult::handled_focused(focused_shape)
                    }
                }
            };
        } else if let Event::Touch(touch) = event
            && render_tree.content_shape().contains(touch.location.into())
        {
            // Just move focus to this element on touch for now, but we could maybe
            // also support dragging or scroll events.
            match state.0 {
                RotaryState::UnFocused => {
                    context.request_view_rebuild();
                    state.0 = RotaryState::Focused;
                    return EventResult::handled_focused(render_tree.content_shape());
                }
                RotaryState::Captive | RotaryState::Focused => {
                    // This prevents focus_touches from sending a `Terminate` event and
                    // swapping to the new touch focus tree.
                    return EventResult::handled_unfocused();
                }
            }
        }

        EventResult::Deferred
    }
}
