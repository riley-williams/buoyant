//! A tappable button that can be pressed to trigger an action.

use core::marker::PhantomData;

use embedded_touch::Phase;

use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventContext, EventResult},
    focus::{FocusAction, Role},
    layout::ResolvedLayout,
    primitives::ProposedDimensions,
    render::IntrinsicShape,
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

/// The touch interaction state.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
enum ButtonTouchState {
    /// The button is pressed and the touch is still within the button area.
    CaptivePressed(u8),
    /// The button was pressed but the touch has moved outside the button area.
    Captive(u8),
    /// The button is not pressed, or the touch has been released.
    #[default]
    AtRest,
}

/// The current interaction state of the button
#[derive(Default,Debug, Clone, PartialEq, Eq)]
pub struct ButtonState {
    /// The current state of a touch interaction with the button.
    touch: ButtonTouchState,
    /// Whether the button is focused.
    is_focused: bool,
}

impl ButtonState {
    /// Whether the button is currently focused.
    #[must_use]
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    /// Whether the button is currently pressed.
    #[must_use]
    pub fn is_pressed(&self) -> bool {
        matches!(self.touch, ButtonTouchState::CaptivePressed(_))
    }
}

/// A tappable button that can be pressed to trigger an action.
///
/// The action is executed upon releasing if the tap starts and ends within the button's area.
///
/// # Examples
///
/// A counter with buttons to increment and decrement the count:
///
/// ```
/// use buoyant::view::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb888;
/// use embedded_graphics::mono_font::ascii::FONT_9X15;
///
/// fn count_view(count: i32) -> impl View<Rgb888, i32> {
///     VStack::new((
///         Text::new_fmt::<32>(
///             format_args!("count: {count}"),
///             &FONT_9X15,
///         ),
///         Button::new(
///             |c: &mut i32| { *c += 1; },
///             |_| Text::new("Increment", &FONT_9X15),
///         ),
///         Button::new(
///             |c: &mut i32| { *c -= 1; },
///             |_| Text::new("Decrement", &FONT_9X15),
///         ),
///     ))
/// }
/// ```
///
/// The [`ButtonState`] passed to the view function can be used to alter the pressed
/// and focused appearances:
///
/// ```
/// use buoyant::view::prelude::*;
/// use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
/// use embedded_graphics::mono_font::ascii::FONT_9X15;
///
/// fn highlight_button() -> impl View<Rgb888, i32> {
///     Button::new(
///         |c: &mut i32| { *c += 1; },
///         |state| {
///             Text::new("Press me", &FONT_9X15)
///                 .foreground_color(Rgb888::WHITE)
///                 .padding(Edges::All, 10)
///                 .background_color(
///                     if state.is_pressed() {
///                         Rgb888::BLUE
///                     } else if state.is_focused() {
///                         Rgb888::CYAN
///                     } else {
///                         Rgb888::GREEN
///                     },
///                     RoundedRectangle::new(10)
///                 )
///         },
///     )
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Button<ViewFn, Inner, Action> {
    _inner_marker: PhantomData<Inner>,
    view: ViewFn,
    action: Action,
}

impl<ViewFn, Inner: ViewMarker, Action> Button<ViewFn, Inner, Action> {
    #[allow(missing_docs)]
    pub fn new(action: Action, view: ViewFn) -> Self
    where
        ViewFn: Fn(ButtonState) -> Inner,
    {
        Self {
            view,
            action,
            _inner_marker: PhantomData,
        }
    }
}

impl<ViewFn, Inner: ViewMarker, Action> ViewMarker for Button<ViewFn, Inner, Action>
where
    Inner::Renderables: IntrinsicShape,
{
    type Renderables = Inner::Renderables;
    type Transition = Opacity;
}

impl<Captures, Inner, ViewFn, Action> ViewLayout<Captures> for Button<ViewFn, Inner, Action>
where
    Action: Fn(&mut Captures),
    Captures: ?Sized,
    Inner: ViewLayout<Captures>,
    Inner::Renderables: IntrinsicShape,
    ViewFn: Fn(ButtonState) -> Inner,
{
    type State = (ButtonState, Inner::State);
    type Sublayout = ResolvedLayout<Inner::Sublayout>;
    type FocusTree = Inner::FocusTree;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        let initial_state = ButtonState {
            touch: ButtonTouchState::AtRest,
            is_focused: false,
        };
        let inner_state = (self.view)(initial_state.clone()).build_state(captures);
        (initial_state, inner_state)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let inner_layout = (self.view)(state.0.clone()).layout(offer, env, captures, &mut state.1);
        ResolvedLayout {
            resolved_size: inner_layout.resolved_size,
            sublayouts: inner_layout,
        }
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: crate::primitives::Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        (self.view)(state.0.clone()).render_tree(
            &layout.sublayouts,
            origin,
            env,
            captures,
            &mut state.1,
        )
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        _focus: &mut Self::FocusTree,
    ) -> EventResult {
        match event {
            Event::Touch(touch) => {
                // Only track the ID of the first touch that started within the button.
                if let ButtonTouchState::Captive(touch_id)
                | ButtonTouchState::CaptivePressed(touch_id) = state.0.touch
                    && touch.id != touch_id
                {
                    return EventResult::deferred();
                }

                let point = touch.location.into();
                match touch.phase {
                    Phase::Started => {
                        if render_tree.content_shape().contains(point) {
                            state.0.touch = ButtonTouchState::CaptivePressed(touch.id);
                            // TODO: I think we could maybe just recompute the tiny button render
                            // tree here and avoid recomputing the view.
                            // May require an internal animation render node?
                            context.request_view_rebuild();
                            return EventResult::handled_unfocused();
                        }
                    }
                    Phase::Ended => {
                        if state.0.touch != ButtonTouchState::AtRest {
                            state.0.touch = ButtonTouchState::AtRest;
                            context.request_view_rebuild();
                            let content_shape = render_tree.content_shape();
                            if content_shape.contains(point) {
                                (self.action)(captures);
                                return EventResult::handled_focused(content_shape);
                            }
                            return EventResult::handled_unfocused();
                        }
                    }
                    Phase::Moved => {
                        match (render_tree.content_shape().contains(point), state.0.touch) {
                            (true, ButtonTouchState::Captive(touch_id)) => {
                                state.0.touch = ButtonTouchState::CaptivePressed(touch_id);
                                // TODO: Same here...
                                context.request_view_rebuild();
                                return EventResult::handled_unfocused();
                            }
                            (false, ButtonTouchState::CaptivePressed(touch_id)) => {
                                state.0.touch = ButtonTouchState::Captive(touch_id);
                                // TODO: Same here...
                                context.request_view_rebuild();
                                return EventResult::handled_unfocused();
                            }
                            (true, ButtonTouchState::CaptivePressed(_))
                            | (false, ButtonTouchState::Captive(_)) => {
                                return EventResult::handled_unfocused();
                            }
                            (_, ButtonTouchState::AtRest) => (),
                        }
                    }
                    Phase::Cancelled => {
                        let was_pressed =
                            matches!(state.0.touch, ButtonTouchState::CaptivePressed(_));
                        state.0.touch = ButtonTouchState::AtRest;
                        state.0.is_focused = false;
                        if was_pressed {
                            // TODO: Same here...
                            context.request_view_rebuild();
                            return EventResult::deferred_lost_focus();
                        }
                        return EventResult::deferred();
                    }
                    Phase::Hovering(_) => {}
                }
                EventResult::deferred()
            }
            Event::Focus {
                action: focus_event,
                ..
            } => {
                if !context.roles.contains(Role::Button) {
                    return EventResult::deferred();
                }
                context.request_redraw();
                // FIXME: Every time we encounter a button, view is forced to be rebuilt...
                // Maybe save both states in the render tree instead
                match focus_event {
                    FocusAction::Teardown => {
                        state.0.is_focused = false;
                        context.request_view_rebuild();
                        EventResult::deferred_lost_focus()
                    }
                    FocusAction::Blur | FocusAction::Next | FocusAction::Previous => {
                        state.0.is_focused = false;
                        context.request_view_rebuild();
                        EventResult::deferred_lost_focus()
                    }
                    FocusAction::Focus(_) => {
                        if !state.0.is_focused {
                            context.request_view_rebuild();
                        }
                        state.0.is_focused = true;
                        EventResult::handled_focused(render_tree.content_shape())
                    }
                    FocusAction::Select => {
                        (self.action)(captures);
                        state.0.is_focused = true;
                        context.request_view_rebuild();
                        EventResult::handled_focused(render_tree.content_shape())
                    }
                }
            }
            _ => EventResult::deferred(),
        }
    }
}
