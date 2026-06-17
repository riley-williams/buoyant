use buoyant::{
    app::{App, Harness as _},
    focus::{self, Role},
    primitives::{Point, Size, geometry},
    render::ContentShape,
    view::prelude::*,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct State {
    a: u32,
    b: u32,
    c: u32,
}

fn three_buttons_with_focus_touches(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|s: &mut State| s.a += 1, |_| Circle).frame_sized(50, 50),
        Button::new(|s: &mut State| s.b += 1, |_| Rectangle).frame_sized(50, 50),
        RoundedRectangle::new(10).frame_sized(50, 50),
        Button::new(|s: &mut State| s.c += 1, |_| RoundedRectangle::new(5)).frame_sized(50, 50),
    ))
    .focus_touches()
}

#[test]
fn touch_moves_focus() {
    let mut harness = App::new(
        State::default(),
        Size::new(50, 200),
        three_buttons_with_focus_touches,
    )
    .with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    harness.tap(Point::new(25, 175));
    assert!(*harness.state() == State { a: 0, b: 0, c: 1 });
    assert!(matches!(
        harness.select().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));
    assert!(*harness.state() == State { a: 0, b: 0, c: 2 });

    // Still on the circle
    harness.tap(Point::new(25, 25));
    assert!(*harness.state() == State { a: 1, b: 0, c: 2 });
    assert!(matches!(
        harness.select().shape(),
        Some(ContentShape::Circle(_))
    ));
    assert!(*harness.state() == State { a: 2, b: 0, c: 2 });

    // Tap in the non-button rectangle, Circle should still be focused
    harness.tap(Point::new(25, 125));
    assert!(*harness.state() == State { a: 2, b: 0, c: 2 });
    assert!(matches!(
        harness.select().shape(),
        Some(ContentShape::Circle(_))
    ));
    assert!(*harness.state() == State { a: 3, b: 0, c: 2 });
}

/// A focusable button above an unfocusable button. The unfocusable button still
/// handles taps (running its action) but never takes focus, so a tap on it
/// returns `Handled { request_focus: false }`.
fn focusable_and_unfocusable_sibling(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|s: &mut State| s.a += 1, |_| Circle).frame_sized(50, 50),
        Button::new(|s: &mut State| s.b += 1, |_| Rectangle)
            .frame_sized(50, 50)
            .unfocusable(),
    ))
    .focus_touches()
}

/// Tapping a sibling that handles the touch without taking focus should blur the
/// currently focused button, and that loss must reach the caller (so wrappers
/// like `Paginate` can stop tracking the child as focused).
///
/// Today this is unrepresentable: the tap returns `Handled { request_focus:
/// false }` and `focus_lost` lives only on `EventResult::Deferred`, so the blur
/// is invisible to the parent. Reaching green needs two coupled changes:
///   1. carry `focus_lost` on `EventResult::Handled` (so `lost_focus()` can be
///      true for a handled-but-unfocused result), and
///   2. have the tap path actually blur the previously focused element when a
///      touch is handled without acquiring focus (today `focus_touches` only
///      tears down old focus on `request_focus: true`, and
///      `focus_touches::touch_moves_focus` asserts the focus is *retained*).
#[test]
#[ignore = "needs focus_lost on EventResult::Handled + tap-away blur behavior"]
fn tap_unfocusable_sibling_blurs_focused_button() {
    let mut harness = App::new(
        State::default(),
        Size::new(50, 100),
        focusable_and_unfocusable_sibling,
    )
    .with_roles(Role::Button);

    // Focus the top button.
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    // Tap the unfocusable sibling: it handles the tap (its action runs) but
    // takes no focus.
    let result = harness.tap(Point::new(25, 75));
    assert_eq!(harness.state().b, 1, "the tap should have been handled");
    assert!(result.is_handled());
    assert!(!result.requested_focus());

    // The focused button should have been blurred, and the caller must be told.
    assert!(
        result.lost_focus(),
        "tapping a handled, unfocusable sibling should report the focus loss"
    );
}

fn grouped_buttons(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|s: &mut State| s.a += 1, |_| Circle)
            .frame_sized(50, 50)
            .exclusive_focus(focus::GROUP_0),
        Button::new(|s: &mut State| s.b += 1, |_| Rectangle)
            .frame_sized(50, 50)
            .exclusive_focus(focus::GROUP_1),
        RoundedRectangle::new(10).frame_sized(50, 50),
        Button::new(|s: &mut State| s.c += 1, |_| RoundedRectangle::new(5))
            .frame_sized(50, 50)
            .unfocusable(),
    ))
    .multiplex_focus([focus::GROUP_0.into(), focus::GROUP_1.into()])
    .focus_touches()
}

#[test]
fn touch_moves_focus_within_groups() {
    let mut harness =
        App::new(State::default(), Size::new(50, 200), grouped_buttons).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward_group(focus::GROUP_0).shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.focus_backward_group(focus::GROUP_1).shape(),
        Some(ContentShape::Rectangle(_))
    ));

    // Tap obtains focus within groups 1 & 2
    harness.tap(Point::new(25, 25));
    assert!(*harness.state() == State { a: 1, b: 0, c: 0 });
    assert!(matches!(
        harness.select_group(focus::GROUP_0).shape(),
        Some(ContentShape::Circle(_))
    ));
    assert!(*harness.state() == State { a: 2, b: 0, c: 0 });

    harness.tap(Point::new(25, 75));
    assert!(*harness.state() == State { a: 2, b: 1, c: 0 });
    assert!(matches!(
        harness.select_group(focus::GROUP_1).shape(),
        Some(ContentShape::Rectangle(_))
    ));
    assert!(*harness.state() == State { a: 2, b: 2, c: 0 });

    // Tap in the non-button rectangle
    harness.tap(Point::new(25, 125));
    assert!(*harness.state() == State { a: 2, b: 2, c: 0 });

    // Tap the unfocusable button
    harness.tap(Point::new(25, 175));
    assert!(*harness.state() == State { a: 2, b: 2, c: 1 });

    // Focus in the groups should be retained
    assert_eq!(
        harness.select_group(focus::GROUP_0).shape(),
        Some(&ContentShape::Circle(geometry::Circle::new(
            Point::new(0, 0),
            50
        )))
    );
    assert!(*harness.state() == State { a: 3, b: 2, c: 1 });

    assert!(matches!(
        harness.select_group(focus::GROUP_1).shape(),
        Some(ContentShape::Rectangle(_))
    ));
    assert!(*harness.state() == State { a: 3, b: 3, c: 1 });
}
