use core::{marker::PhantomData, time::Duration};

use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Frame, ProposedDimensions, Seal},
    render::Container,
    view::{Event, ViewLayout, ViewMarker},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    CaptivePressed,
    Captive,
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
///                 .padding(Edges::All, 10)
///                 .background(Alignment::default(), RoundedRectangle::new(10).stroked(2))
///                 .foreground_color(
///                     if is_pressed {
///                         Rgb888::BLUE
///                     } else {
///                         Rgb888::GREEN
///                     }
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
}

impl<Captures, Inner, ViewFn, Action> ViewLayout<Captures> for Button<ViewFn, Inner, Action>
where
    Action: Fn(&mut Seal<Captures>),
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
        &self,
        event: &Event,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        app_time: Duration,
    ) -> EventResult {
        let mut result = EventResult::default();
        match event {
            Event::TouchDown(point) => {
                if render_tree.frame.contains(point) {
                    state.0 = ButtonState::CaptivePressed;
                    // TODO: I think we could maybe just recompute the tiny button render
                    // tree here and avoid recomputing the view.
                    // May require an internal animation render node?
                    result.recompute_view = true;
                    result.handled = true;
                }
            }
            Event::TouchUp(point) => {
                if render_tree.frame.contains(point) && state.0 != ButtonState::AtRest {
                    let mut seal = Seal::new(captures);
                    (self.action)(&mut seal);
                    state.0 = ButtonState::AtRest;
                    // TODO: Same here, if the seal isn't broken?
                    result.recompute_view = seal.is_broken();
                    result.handled = true;
                }
            }
            Event::TouchMoved(point) => match (render_tree.frame.contains(point), state.0) {
                (true, ButtonState::Captive) => {
                    state.0 = ButtonState::CaptivePressed;
                    // TODO: Same here...
                    result.recompute_view = true;
                    result.handled = true;
                }
                (false, ButtonState::CaptivePressed) => {
                    state.0 = ButtonState::Captive;
                    // TODO: Same here...
                    result.recompute_view = true;
                    result.handled = true;
                }
                (true, ButtonState::CaptivePressed) | (false, ButtonState::Captive) => {
                    result.handled = true;
                }
                (_, ButtonState::AtRest) => (),
            },
            Event::TouchCancelled => {
                state.0 = ButtonState::AtRest;
                if state.0 == ButtonState::CaptivePressed {
                    // TODO: Same here...
                    result.recompute_view = true;
                }
                result.handled = false;
            }
            _ => (),
        }
        result
    }
}
