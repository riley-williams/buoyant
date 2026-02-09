use buoyant::{
    app::{App, Harness as _},
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

fn nested_hstack_view(_: &State) -> impl View<(), State> + use<> {
    HStack::new((
        HStack::new((
            Button::new(|s: &mut State| s.a += 1, |_| Circle),
            Rectangle,
            Button::new(|s: &mut State| s.b += 1, |_| RoundedRectangle::new(10)),
        )),
        Button::new(|s: &mut State| s.c += 1, |_| Rectangle),
        Rectangle,
    ))
}

#[test]
fn navigate_forward_skips_unfocusable() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), nested_hstack_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    let result = harness.select();
    assert!(matches!(result.shape(), Some(ContentShape::Rectangle(_))));
    assert_eq!(harness.state().c, 1);
}

#[test]
fn navigate_backward_skips_unfocusable() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), nested_hstack_view).with_roles(Role::Button);

    harness.focus_forward();
    harness.next();
    harness.next();

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn select_triggers_action() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), nested_hstack_view).with_roles(Role::Button);

    harness.focus_forward();
    assert_eq!(harness.state().a, 0);

    harness.select();
    assert_eq!(harness.state().a, 1);
    assert_eq!(harness.state().b, 0);
    assert_eq!(harness.state().c, 0);

    harness.next();
    harness.select();
    assert_eq!(harness.state().a, 1);
    assert_eq!(harness.state().b, 1);
    assert_eq!(harness.state().c, 0);
}

fn hstack_with_leading_unfocusable(_: &State) -> impl View<(), State> + use<> {
    HStack::new((
        HStack::new((
            Rectangle,
            Button::new(|s: &mut State| s.a += 1, |_| Circle),
            Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
        )),
        Button::new(|s: &mut State| s.c += 1, |_| RoundedRectangle::new(10)),
        Rectangle,
    ))
}

#[test]
fn focus_skips_leading_unfocusable() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), hstack_with_leading_unfocusable)
        .with_roles(Role::Button);

    // Should skip leading Rectangle and focus Circle button
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
}
