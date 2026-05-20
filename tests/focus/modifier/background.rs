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
    assert!(matches!(harness.next(), EventResult::Deferred { .. }));
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
    assert!(matches!(harness.previous(), EventResult::Deferred { .. }));
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
