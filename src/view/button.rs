//! A tappable button that can be pressed to trigger an action.

use core::marker::PhantomData;

use embedded_touch::Phase;

use crate::{
    environment::LayoutEnvironment,
    event::{EventContext, EventResult},
    layout::ResolvedLayout,
    primitives::{Frame, ProposedDimensions},
    render::Container,
    transition::Opacity,
    view::{Event, ViewLayout, ViewMarker},
};

/// A button interaction state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ButtonState {
    /// The button is pressed and the touch is still within the button area.
    CaptivePressed(u8),
    /// The button was pressed but the touch has moved outside the button area.
    Captive(u8),
    /// The button is not pressed, or the touch has been released.
    AtRest,
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
/// The boolean passed to the view function can be used to alter the pressed appearance:
///
/// ```
/// use buoyant::view::prelude::*;
/// use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
/// use embedded_graphics::mono_font::ascii::FONT_9X15;
///
/// fn highlight_button() -> impl View<Rgb888, i32> {
///     Button::new(
///         |c: &mut i32| { *c += 1; },
///         |is_pressed| {
///             Text::new("Press me", &FONT_9X15)
///                 .foreground_color(Rgb888::WHITE)
///                 .padding(Edges::All, 10)
///                 .background_color(
///                     if is_pressed {
///                         Rgb888::BLUE
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

impl<ViewFn, Inner, Action> Button<ViewFn, Inner, Action> {
    #[allow(missing_docs)]
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
    type Transition = Opacity;
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

    fn transition(&self) -> Self::Transition {
        Opacity
    }

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
            ButtonState::CaptivePressed(_) => {
                (self.view)(true).layout(offer, env, captures, &mut state.1)
            }
            ButtonState::AtRest | ButtonState::Captive(_) => {
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
                ButtonState::CaptivePressed(_) => {
                    (self.view)(true).render_tree(layout, origin, env, captures, &mut state.1)
                }
                ButtonState::AtRest | ButtonState::Captive(_) => {
                    (self.view)(false).render_tree(layout, origin, env, captures, &mut state.1)
                }
            },
        )
    }

    fn handle_event(
        &self,
        event: &Event,
        _context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        let mut result = EventResult::default();
        let Event::Touch(touch) = event else {
            return result;
        };
        // Only track the ID of the first touch that started within the button.
        if let ButtonState::Captive(touch_id) | ButtonState::CaptivePressed(touch_id) = state.0
            && touch.id != touch_id
        {
            return result;
        }

        let point = touch.location.into();
        match touch.phase {
            Phase::Started => {
                if render_tree.frame.contains(&point) {
                    state.0 = ButtonState::CaptivePressed(touch.id);
                    // TODO: I think we could maybe just recompute the tiny button render
                    // tree here and avoid recomputing the view.
                    // May require an internal animation render node?
                    result.recompute_view = true;
                    result.handled = true;
                }
            }
            Phase::Ended => {
                if state.0 != ButtonState::AtRest {
                    if render_tree.frame.contains(&point) {
                        (self.action)(captures);
                    }
                    state.0 = ButtonState::AtRest;
                    result.recompute_view = true;
                    result.handled = true;
                }
            }
            Phase::Moved => match (render_tree.frame.contains(&point), state.0) {
                (true, ButtonState::Captive(touch_id)) => {
                    state.0 = ButtonState::CaptivePressed(touch_id);
                    // TODO: Same here...
                    result.recompute_view = true;
                    result.handled = true;
                }
                (false, ButtonState::CaptivePressed(touch_id)) => {
                    state.0 = ButtonState::Captive(touch_id);
                    // TODO: Same here...
                    result.recompute_view = true;
                    result.handled = true;
                }
                (true, ButtonState::CaptivePressed(_)) | (false, ButtonState::Captive(_)) => {
                    result.handled = true;
                }
                (_, ButtonState::AtRest) => (),
            },
            Phase::Cancelled => {
                if matches!(state.0, ButtonState::CaptivePressed(_)) {
                    // TODO: Same here...
                    result.recompute_view = true;
                }
                state.0 = ButtonState::AtRest;
                result.handled = false;
            }
            Phase::Hovering(_) => {
                // Events are handled one-by-one, ignore irrelevant events, but don't modify the
                // state.
            }
        }
        result
    }
}
