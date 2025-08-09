//! A tappable button that can be pressed to trigger an action.

use core::marker::PhantomData;

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
pub enum ButtonState {
    /// The button is pressed and the touch is still within the button area.
    CaptivePressed,
    /// The button was pressed but the touch has moved outside the button area.
    Captive,
    /// The button is not pressed, or the touch has been released.
    AtRest,
}

/// A tappable button that can be pressed to trigger an action.
///
/// The action is executed upon releasing if the tap starts and ends within the button's area.
///
/// Note the signature of the action: `|data: &mut Seal<MyData>| { ... }`
///
/// [`Seal`] is a wrapper around the captures that tracks mutation of the inner data
/// in order to intelligently recompute the view body. Obtaining a mutable reference
/// to the data will break the seal, but it can also be manually broken by calling
/// [`Seal::break_seal`]. This is useful if you want to ensure that the view is recomputed
/// after the action is executed, even if the data has not changed.
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
///             |c: &mut Seal<i32>| { **c += 1; },
///             |_| Text::new("Increment", &FONT_9X15),
///         ),
///         Button::new(
///             |c: &mut Seal<i32>| { **c -= 1; },
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
///         |c: &mut Seal<i32>| { **c += 1; },
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
    type Transition = Opacity;
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
        _context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
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

use core::ops::{Deref, DerefMut};

/// A [`Seal`] is a wrapper around a mutable reference that can be "broken" to allow mutation.
/// This is used to determine if a view tree needs to be re-computed due to changes in the underlying data.
///
/// # Examples
///
/// Reading doesn't break the seal:
///
/// ```
/// # use buoyant::view::button::Seal;
/// fn read_only_operation(value: &mut Seal<i32>) {
///     println!("Value: {}", *value);
/// }
/// ```
///
/// Writing breaks the seal:
///
/// ```
/// # use buoyant::view::button::Seal;
/// fn mutating_operation(value: &mut Seal<i32>) {
///     *value.as_mut() += 10;
/// }
/// ```
///
/// Conditional operations may avoid breaking the seal if no mutation occurs:
///
/// ```
/// # use buoyant::view::button::Seal;
/// fn conditional_operation(value: &mut Seal<i32>, should_modify: bool) {
///     if should_modify {
///         *value.as_mut() = 100;
///     } else {
///         println!("Value: {}", *value);
///     }
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Seal<'a, T: ?Sized> {
    value: &'a mut T,
    is_broken: bool,
}

impl<T> core::fmt::Display for Seal<'_, T>
where
    T: ?Sized + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl<'a, T: ?Sized> Seal<'a, T> {
    #[must_use]
    pub(crate) const fn new(value: &'a mut T) -> Self {
        Self {
            value,
            is_broken: false,
        }
    }

    /// Mark the seal as broken, triggering a re-computation of the view tree.
    ///
    /// This may be necessary if the underlying data uses interior mutability or
    /// if the the view state is not a pure function of the data.
    pub const fn break_seal(&mut self) {
        self.is_broken = true;
    }

    /// Check if the seal was broken.
    #[must_use]
    pub const fn is_broken(&self) -> bool {
        self.is_broken
    }
}

impl<T: ?Sized> AsRef<T> for Seal<'_, T> {
    fn as_ref(&self) -> &T {
        self.value
    }
}

impl<T: ?Sized> AsMut<T> for Seal<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        self.is_broken = true;
        self.value
    }
}

impl<T: ?Sized> Deref for Seal<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T: ?Sized> DerefMut for Seal<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.is_broken = true;
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::Seal;

    #[test]
    fn test_ref_seal() {
        let mut value = 42;
        let mut seal = Seal::new(&mut value);
        assert_eq!(seal.as_ref(), &42);
        assert!(!seal.is_broken);
        *(seal.as_mut()) = 43;
        assert_eq!(seal.value, &43);
        assert!(seal.is_broken);
    }

    #[test]
    fn test_deref_seal() {
        let mut value = 42;
        let mut seal = Seal::new(&mut value);
        assert_eq!(*seal, 42);
        assert!(!seal.is_broken);
        *seal = 43;
        assert_eq!(seal.value, &43);
        assert!(seal.is_broken);
    }

    #[test]
    fn test_manually_break_seal() {
        let mut value = 42;
        let mut seal = Seal::new(&mut value);
        assert!(!seal.is_broken);
        seal.break_seal();
        assert!(seal.is_broken);
    }
}
