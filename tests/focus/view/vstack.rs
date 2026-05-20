use buoyant::{
    app::{App, Harness as _},
    event::EventResult,
    focus::Role,
    primitives::Size,
    render::ContentShape,
    view::prelude::*,
};

struct State {
    a: u32,
    b: u32,
    c: u32,
}

/// Three different shapes to distinguish buttons
fn three_button_stack(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|s: &mut State| s.a += 1, |_| Circle),
        Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
        Button::new(|s: &mut State| s.c += 1, |_| RoundedRectangle::new(10)),
    ))
}

fn stack_with_unfocusable_first(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Rectangle,
        Button::new(|s: &mut State| s.a += 1, |_| Circle),
        Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
    ))
}

fn nested_stack_unfocusable_end(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        VStack::new((
            Button::new(|s: &mut State| s.a += 1, |_| Circle),
            Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
            Rectangle,
        )),
        Button::new(|s: &mut State| s.c += 1, |_| RoundedRectangle::new(10)),
        Rectangle,
    ))
}

fn stack_no_focusable(_: &State) -> impl View<(), State> + use<> {
    VStack::new((Rectangle, Rectangle, Rectangle))
}

#[test]
fn navigate_forward_through_stack() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), three_button_stack).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    harness.select();
    assert_eq!(harness.state().a, 1);

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    harness.select();
    assert_eq!(harness.state().b, 1);

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));

    harness.select();
    assert_eq!(harness.state().c, 1);
}

#[test]
fn navigate_backward_through_stack() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), three_button_stack).with_roles(Role::Button);

    harness.focus_forward();
    harness.next();
    harness.next();

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    harness.select();
    assert_eq!(harness.state().b, 1);
    assert_eq!(harness.state().c, 0);
}

#[test]
fn skips_unfocusable_first_element() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), stack_with_unfocusable_first).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn previous_into_container_with_unfocusable_end() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), nested_stack_unfocusable_end).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Rectangle(_))
    ));
}

#[test]
fn no_focusable_elements_returns_deferred() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), stack_no_focusable).with_roles(Role::Button);

    assert!(
        matches!(harness.focus_forward(), EventResult::Deferred { .. }),
        "No focusable elements should return Deferred"
    );
}
