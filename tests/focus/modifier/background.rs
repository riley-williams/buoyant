use buoyant::{
    app::{App, Harness as _},
    event::{Event, EventResult, Key},
    focus::{FocusAction, Role},
    layout::Alignment,
    primitives::Size,
    render::ContentShape,
    view::{map_event::Mapping, prelude::*},
};

struct State {
    foreground_tapped: bool,
    background_tapped: bool,
}

/// Both background and foreground have focusable buttons
fn view_with_both_focusable(_: &State) -> impl View<(), State> + use<> {
    Button::new(|s: &mut State| s.foreground_tapped = true, |_| Circle).background(
        Alignment::Center,
        Button::new(|s: &mut State| s.background_tapped = true, |_| Rectangle),
    )
}

fn view_with_empty_background(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|s: &mut State| s.foreground_tapped = true, |_| Circle),
        Rectangle,
    ))
    .background(Alignment::Center, EmptyView)
}

/// Focusable foreground over an unfocusable (empty) background.
fn focusable_foreground_empty_background(_: &State) -> impl View<(), State> + use<> {
    Button::new(|s: &mut State| s.foreground_tapped = true, |_| Circle)
        .background(Alignment::Center, EmptyView)
}

#[test]
fn navigate_forward_through_background() {
    let state = State {
        foreground_tapped: false,
        background_tapped: false,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), view_with_both_focusable).with_roles(Role::Button);

    // Background receives focus first
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    harness.select();
    assert!(harness.state().background_tapped);
    assert!(!harness.state().foreground_tapped);

    // Next moves to foreground
    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Circle(_))
    ));

    harness.select();
    assert!(harness.state().foreground_tapped);

    // Past foreground returns deferred
    assert!(matches!(harness.next(), EventResult::Deferred));
}

#[test]
fn navigate_backward_through_background() {
    let state = State {
        foreground_tapped: false,
        background_tapped: false,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), view_with_both_focusable).with_roles(Role::Button);

    // Navigate to the end
    harness.focus_forward();
    harness.next();

    // Previous moves from foreground to background
    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    harness.select();
    assert!(harness.state().background_tapped);
    assert!(!harness.state().foreground_tapped);

    // Past background returns deferred
    assert!(matches!(harness.previous(), EventResult::Deferred));
}

/// When focus leaves the foreground backward and the background can't take it,
/// the foreground's lost-focus signal must not be swallowed by the background's
/// plain deferral.
#[test]
fn lost_focus_propagates_when_leaving_foreground_backward() {
    let state = State {
        foreground_tapped: false,
        background_tapped: false,
    };
    let mut harness = App::new(
        state,
        Size::new(100, 100),
        focusable_foreground_empty_background,
    )
    .with_roles(Role::Button);

    // Focus lands on the foreground button (background is empty).
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    // Navigating backward leaves the foreground; the empty background can't take
    // focus, so the view as a whole loses focus.
    assert_eq!(harness.previous(), EventResult::Deferred);
}

#[test]
fn empty_background_skips_to_foreground() {
    let state = State {
        foreground_tapped: false,
        background_tapped: false,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), view_with_empty_background).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
}

/// A button which can be selected with the newline key down
fn key_activatable_button<S, F: Fn() -> S, A: Fn(&mut State)>(
    action: A,
    shape: F,
) -> impl View<(), State> + use<S, F, A>
where
    S: View<(), State>,
{
    Button::new(action, move |_| shape()).map_event(move |event, _: &mut State| match event {
        Event::KeyDown {
            key: Key::Character('\n'),
            ..
        } => Mapping::Replace(Event::from(FocusAction::Select)),
        Event::KeyUp { .. } => Mapping::Defer,
        _ => Mapping::Passthrough,
    })
}

fn key_aware_background_view(_: &State) -> impl View<(), State> + use<> {
    key_activatable_button(|s: &mut State| s.foreground_tapped = true, || Circle).background(
        Alignment::Center,
        key_activatable_button(|s: &mut State| s.background_tapped = true, || Rectangle),
    )
}

#[test]
fn key_down_routes_to_focused_background_child() {
    let state = State {
        foreground_tapped: false,
        background_tapped: false,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), key_aware_background_view).with_roles(Role::Button);

    // Background receives focus first.
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));
    // A key event reaches the focused background button and not the foreground.
    harness.key_down(Key::Character('\n'));
    assert!(harness.state().background_tapped);
    assert!(!harness.state().foreground_tapped);

    // Move to foreground; key now activates foreground only.
    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Circle(_))
    ));
    harness.key_down(Key::Character('\n'));
    assert!(harness.state().foreground_tapped);
}
