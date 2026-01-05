//! A tappable button that can be pressed to trigger an action.

use core::marker::PhantomData;

use embedded_touch::Phase;

use crate::{
    environment::LayoutEnvironment,
    event::{
        EventContext, EventResult,
        input::{FocusState, Groups, InputRef, Interaction},
        keyboard::{KeyboardEvent, KeyboardEventKind},
    },
    layout::ResolvedLayout,
    primitives::{Frame, ProposedDimensions},
    render::Container,
    transition::Opacity,
    view::{Event, ViewLayout, ViewMarker},
};

/// A button state.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonState {
    /// A button interaction state.
    pub touch: ButtonTouchState,
    /// A button focus state.
    pub focus: FocusState,
}

/// A button interaction state.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ButtonTouchState {
    /// The button is pressed and the touch is still within the button area.
    CaptivePressed(u8),
    /// The button was pressed but the touch has moved outside the button area.
    Captive(u8),
    /// The button is not pressed, or the touch has been released.
    #[default]
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
///         |i| {
///             Text::new("Press me", &FONT_9X15)
///                 .foreground_color(Rgb888::WHITE)
///                 .padding(Edges::All, 10)
///                 .background_color(
///                     if i.is_pressed() {
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
    groups: Groups,
    view: ViewFn,
    action: Action,
}

// NOTE: I tried making Button accept both `fn(&mut Captures)` and `fn(&Input, &mut Captures)`,
//       and I succeeded, but it looks too ugly in the API and I don't think it's worth it.
//       It would confuse users more than help them.
impl<ViewFn: Fn(Interaction) -> Inner, Inner, Action> Button<ViewFn, Inner, Action> {
    #[allow(missing_docs)]
    pub fn new(action: Action, view: ViewFn) -> Self {
        Self {
            view,
            action,
            groups: Groups::default(),
            _inner_marker: PhantomData,
        }
    }

    #[allow(missing_docs)]
    pub fn new_with_groups(action: Action, groups: impl Into<Groups>, view: ViewFn) -> Self {
        Self {
            view,
            action,
            groups: groups.into(),
            _inner_marker: PhantomData,
        }
    }
}

impl<ViewFn, Inner, Action> Button<ViewFn, Inner, Action> {
    /// Assign the button to the set of input groups. It becomes focusable via
    /// any keyboard within these groups.
    #[must_use]
    pub fn groups(mut self, groups: Groups) -> Self {
        self.groups |= groups;
        self
    }

    fn interaction(&self, state: ButtonState, input: InputRef<'_>) -> Interaction {
        // todo: handle button press too, and click
        let pressed = matches!(state.touch, ButtonTouchState::CaptivePressed(_));
        let groups = input.active_groups() & self.groups;
        let focused = state.focus.is_focused_any(groups);
        Interaction::new()
            .with(pressed, Interaction::PRESSED)
            .with(focused, Interaction::FOCUSED)
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
    ViewFn: Fn(Interaction) -> Inner,
{
    type State = (ButtonState, Inner::State);
    type Sublayout = Inner::Sublayout;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        (
            ButtonState {
                touch: ButtonTouchState::default(),
                focus: FocusState::new(self.groups),
            },
            (self.view)(Interaction::new()).build_state(captures),
        )
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let interaction = self.interaction(state.0, env.input());

        (self.view)(interaction).layout(offer, env, captures, &mut state.1)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let interaction = self.interaction(state.0, env.input());

        Container::new(
            Frame::new(origin, layout.resolved_size.into()),
            (self.view)(interaction).render_tree(layout, origin, env, captures, &mut state.1),
        )
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        match event {
            Event::Touch(touch) => self.handle_touch(render_tree, captures, state, touch),
            Event::Keyboard(keyboard) => self.handle_keyboard(context, captures, state, keyboard),
            _ => EventResult::default(),
        }
    }
}

impl<Inner, ViewFn, Action> Button<ViewFn, Inner, Action>
where
    ViewFn: Fn(Interaction) -> Inner,
{
    fn handle_touch<Captures: ?Sized>(
        &self,
        render_tree: &<Self as ViewMarker>::Renderables,
        captures: &mut Captures,
        state: &mut <Self as ViewLayout<Captures>>::State,
        touch: &embedded_touch::Touch,
    ) -> EventResult
    where
        Inner: ViewLayout<Captures>,
        Action: Fn(&mut Captures),
    {
        let mut result = EventResult::default();
        // Only track the ID of the first touch that started within the button.
        if let ButtonTouchState::Captive(touch_id) | ButtonTouchState::CaptivePressed(touch_id) =
            state.0.touch
            && touch.id != touch_id
        {
            return result;
        }

        let point = touch.location.into();
        match touch.phase {
            Phase::Started => {
                if render_tree.frame.contains(&point) {
                    state.0.touch = ButtonTouchState::CaptivePressed(touch.id);
                    // TODO: I think we could maybe just recompute the tiny button render
                    // tree here and avoid recomputing the view.
                    // May require an internal animation render node?
                    result.recompute_view = true;
                    result.handled = true;
                }
            }
            Phase::Ended => {
                if state.0.touch != ButtonTouchState::AtRest {
                    if render_tree.frame.contains(&point) {
                        (self.action)(captures);
                    }
                    state.0.touch = ButtonTouchState::AtRest;
                    result.recompute_view = true;
                    result.handled = true;
                }
            }
            Phase::Moved => match (render_tree.frame.contains(&point), state.0.touch) {
                (true, ButtonTouchState::Captive(touch_id)) => {
                    state.0.touch = ButtonTouchState::CaptivePressed(touch_id);
                    // TODO: Same here...
                    result.recompute_view = true;
                    result.handled = true;
                }
                (false, ButtonTouchState::CaptivePressed(touch_id)) => {
                    state.0.touch = ButtonTouchState::Captive(touch_id);
                    // TODO: Same here...
                    result.recompute_view = true;
                    result.handled = true;
                }
                (true, ButtonTouchState::CaptivePressed(_))
                | (false, ButtonTouchState::Captive(_)) => {
                    result.handled = true;
                }
                (_, ButtonTouchState::AtRest) => (),
            },
            Phase::Cancelled => {
                if matches!(state.0.touch, ButtonTouchState::CaptivePressed(_)) {
                    // TODO: Same here...
                    result.recompute_view = true;
                }
                state.0.touch = ButtonTouchState::AtRest;
                result.handled = false;
            }
            Phase::Hovering(_) => {
                // Events are handled one-by-one, ignore irrelevant events, but don't modify the
                // state.
            }
        }
        result
    }
    fn handle_keyboard<Captures: ?Sized>(
        &self,
        context: &EventContext,
        captures: &mut Captures,
        state: &mut <Self as ViewLayout<Captures>>::State,
        event: &KeyboardEvent,
    ) -> EventResult
    where
        Inner: ViewLayout<Captures>,
        Action: Fn(&mut Captures),
    {
        if !state.0.focus.is_member_of_any(event.groups) {
            return EventResult::default();
        }

        match event.kind {
            k if k.is_movement() => context.input.leaf_move(&mut state.0.focus, event.groups),
            KeyboardEventKind::LongPress | KeyboardEventKind::Cancel
                if state.0.focus.is_focused_any(event.groups) =>
            {
                context.input.leaf_move(&mut state.0.focus, event.groups)
            }
            KeyboardEventKind::Click if state.0.focus.is_focused_any(event.groups) => {
                (self.action)(captures);

                EventResult::new(true, true, false)
            }
            _ => EventResult::default(),
        }
    }
}
