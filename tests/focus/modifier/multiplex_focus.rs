use buoyant::{
    app::{App, Harness as _},
    event::EventResult,
    focus,
    primitives::Size,
    render::ContentShape,
    view::prelude::*,
};

struct State {
    a: u32,
    b: u32,
    c: u32,
}

/// A stack with three buttons, the third being ungated
fn three_button_stack(_state: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|s: &mut State| s.a += 1, |_| Circle).exclusive_focus(focus::GROUP_0),
        Button::new(|s: &mut State| s.b += 1, |_| Rectangle).exclusive_focus(focus::GROUP_1),
        Button::new(|s: &mut State| s.c += 1, |_| RoundedRectangle::new(10)),
    ))
    .multiplex_focus([focus::GROUP_0.into(), focus::GROUP_1.into()])
}

#[test]
fn groups_are_independent() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), three_button_stack);

    harness.focus_forward_group(focus::GROUP_0);
    assert!(matches!(
        harness.focus_forward_group(focus::GROUP_0).shape(),
        Some(ContentShape::Circle(_))
    ),);
    assert!(matches!(
        harness.focus_forward_group(focus::GROUP_1).shape(),
        Some(ContentShape::Rectangle(_))
    ),);
    assert!(matches!(
        harness.next_group(focus::GROUP_0).shape(),
        Some(ContentShape::RoundedRectangle(_))
    ),);
    assert!(matches!(
        harness.next_group(focus::GROUP_1).shape(),
        Some(ContentShape::RoundedRectangle(_))
    ),);

    assert_eq!(harness.next_group(focus::GROUP_0), EventResult::Deferred);
    assert_eq!(harness.next_group(focus::GROUP_1), EventResult::Deferred);
}

#[test]
fn unfocused_behavior() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), three_button_stack);

    assert!(
        harness
            .focus_forward_group(focus::GROUP_0)
            .requested_focus()
    );
    assert!(!harness.blur_group(focus::GROUP_0).requested_focus());
    assert!(harness.select_group(focus::GROUP_0).requested_focus());

    // Programmer error to blur/select when no focus was obtained
    assert_eq!(harness.blur_group(focus::GROUP_1), EventResult::Deferred);
    assert_eq!(harness.select_group(focus::GROUP_1), EventResult::Deferred);
}

#[test]
fn select_across_groups() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), three_button_stack);

    harness.focus_forward_group(focus::GROUP_0);
    harness.select_group(focus::GROUP_0);
    assert_eq!(harness.state().a, 1);

    harness.focus_forward_group(focus::GROUP_1);
    harness.select_group(focus::GROUP_1);
    assert_eq!(harness.state().b, 1);

    harness.next_group(focus::GROUP_0);
    harness.select_group(focus::GROUP_0);
    assert_eq!(harness.state().c, 1);

    harness.previous_group(focus::GROUP_0);
    harness.select_group(focus::GROUP_0);
    assert_eq!(harness.state().a, 2);
}

#[test]
fn focus_from_opposite_ends() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), three_button_stack);

    assert!(matches!(
        harness.focus_forward_group(focus::GROUP_0).shape(),
        Some(ContentShape::Circle(_))
    ),);
    assert!(matches!(
        harness.focus_backward_group(focus::GROUP_1).shape(),
        Some(ContentShape::RoundedRectangle(_))
    ),);
    assert!(matches!(
        harness.next_group(focus::GROUP_0).shape(),
        Some(ContentShape::RoundedRectangle(_))
    ),);
    assert!(matches!(
        harness.previous_group(focus::GROUP_1).shape(),
        Some(ContentShape::Rectangle(_))
    ),);
    assert_eq!(harness.next_group(focus::GROUP_0), EventResult::Deferred);
    assert_eq!(
        harness.previous_group(focus::GROUP_1),
        EventResult::Deferred
    );
}
