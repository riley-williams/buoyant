use buoyant::{
    app::{App, Harness as _},
    event::EventResult,
    focus::Role,
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

/// Focusable foreground under an unfocusable (empty) overlay.
fn focusable_foreground_empty_overlay(_: &State) -> impl View<(), State> + use<> {
    Button::new(|s: &mut State| s.foreground_tapped = true, |_| Circle)
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
    assert!(matches!(harness.next(), EventResult::Deferred { .. }));
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
    assert!(matches!(harness.previous(), EventResult::Deferred { .. }));
}

/// When focus leaves the foreground backward and the overlay can't take it, the
/// foreground's lost-focus signal must not be swallowed by the overlay's plain
/// deferral.
#[test]
fn lost_focus_propagates_when_leaving_foreground_backward() {
    let state = State {
        foreground_tapped: false,
        overlay_tapped: false,
    };
    let mut harness = App::new(
        state,
        Size::new(100, 100),
        focusable_foreground_empty_overlay,
    )
    .with_roles(Role::Button);

    // Focus lands on the foreground button (overlay is empty).
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    // Navigating backward leaves the foreground; the empty overlay can't take
    // focus, so the view as a whole loses focus.
    assert_eq!(harness.previous(), EventResult::deferred_lost_focus());
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
