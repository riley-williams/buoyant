use buoyant::{
    app::{App, Harness as _},
    event::{Event, EventResult, Key},
    focus::{FocusAction, Role},
    layout::Alignment,
    primitives::Size,
    render::ContentShape,
    view::prelude::*,
};

struct State {
    foreground_tapped: bool,
    overlay_tapped: bool,
}

/// Both overlay and foreground have focusable buttons
fn view_with_both_focusable(_: &State) -> impl View<(), State> + use<> {
    Button::new(|s: &mut State| s.foreground_tapped = true, |_| Circle).overlay(
        Alignment::Center,
        Button::new(|s: &mut State| s.overlay_tapped = true, |_| Rectangle),
    )
}

fn view_with_empty_overlay(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|s: &mut State| s.foreground_tapped = true, |_| Circle),
        Rectangle,
    ))
    .overlay(Alignment::Center, EmptyView)
}

#[test]
fn navigate_forward_through_overlay() {
    let state = State {
        foreground_tapped: false,
        overlay_tapped: false,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), view_with_both_focusable).with_roles(Role::Button);

    // Overlay receives focus first (higher z-order)
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    harness.select();
    assert!(harness.state().overlay_tapped);
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
fn navigate_backward_through_overlay() {
    let state = State {
        foreground_tapped: false,
        overlay_tapped: false,
    };
    let mut harness = App::new(state, Size::new(100, 100), view_with_both_focusable)
        .with_roles(Role::Button)
        .with_focus_at_end();

    // Previous moves from foreground to overlay
    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    harness.select();
    assert!(harness.state().overlay_tapped);
    assert!(!harness.state().foreground_tapped);

    // Past overlay returns deferred
    assert!(matches!(harness.previous(), EventResult::Deferred));
}

#[test]
fn empty_overlay_skips_to_foreground() {
    let state = State {
        foreground_tapped: false,
        overlay_tapped: false,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), view_with_empty_overlay).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
}

fn key_activatable_button<S>(
    action: impl Fn(&mut State) + 'static,
    shape: impl Fn() -> S + 'static,
) -> impl View<(), State> + 'static
where
    S: View<(), State> + 'static,
{
    Button::new(action, move |_| shape()).map_event::<(), _>(move |event, ()| match event {
        Event::KeyDown(Key::Character('\n')) => Some(Event::from(FocusAction::Select)),
        Event::KeyUp(_) => None,
        _ => Some(event.clone()),
    })
}

fn key_aware_overlay_view(_: &State) -> impl View<(), State> + use<> {
    key_activatable_button(|s: &mut State| s.foreground_tapped = true, || Circle).overlay(
        Alignment::Center,
        key_activatable_button(|s: &mut State| s.overlay_tapped = true, || Rectangle),
    )
}

#[test]
fn key_down_routes_to_focused_overlay_child() {
    let state = State {
        foreground_tapped: false,
        overlay_tapped: false,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), key_aware_overlay_view).with_roles(Role::Button);

    // Overlay receives focus first.
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));
    harness.key_down(Key::Character('\n'));
    assert!(harness.state().overlay_tapped);
    assert!(!harness.state().foreground_tapped);

    // Move to foreground, key now activates foreground only.
    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Circle(_))
    ));
    harness.key_down(Key::Character('\n'));
    assert!(harness.state().foreground_tapped);
}
