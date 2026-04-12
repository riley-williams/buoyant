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

fn geometry_reader_single_button(_: &State) -> impl View<(), State> + use<> {
    GeometryReader::new(|_size| Button::new(|s: &mut State| s.a += 1, |_| Circle))
}

fn geometry_reader_with_stack(_: &State) -> impl View<(), State> + use<> {
    GeometryReader::new(|_size| {
        VStack::new((
            Button::new(|s: &mut State| s.a += 1, |_| Circle),
            Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
            Button::new(|s: &mut State| s.c += 1, |_| RoundedRectangle::new(10)),
        ))
    })
}

fn geometry_reader_no_focusable(_: &State) -> impl View<(), State> + use<> {
    GeometryReader::new(|_size| Rectangle)
}

fn geometry_reader_in_stack(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|s: &mut State| s.a += 1, |_| Circle),
        GeometryReader::new(|_size| Button::new(|s: &mut State| s.b += 1, |_| Rectangle)),
        Button::new(|s: &mut State| s.c += 1, |_| RoundedRectangle::new(10)),
    ))
}

fn nested_geometry_readers(_: &State) -> impl View<(), State> + use<> {
    GeometryReader::new(|_outer_size| {
        GeometryReader::new(|_inner_size| Button::new(|s: &mut State| s.a += 1, |_| Circle))
    })
}

#[test]
fn focus_reaches_inner_view() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), geometry_reader_single_button)
        .with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));
}

#[test]
fn select_triggers_inner_action() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), geometry_reader_single_button)
        .with_roles(Role::Button);

    harness.focus_forward();
    assert_eq!(harness.state().a, 0);

    harness.select();
    assert_eq!(harness.state().a, 1);
}

#[test]
fn navigate_through_inner_stack() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), geometry_reader_with_stack).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));

    let result = harness.next();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Rectangle(_))));

    let result = harness.next();
    assert!(result.requested_focus());
    assert!(matches!(
        result.shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));

    harness.select();
    assert_eq!(harness.state().c, 1);
}

#[test]
fn backward_navigation_in_inner_stack() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), geometry_reader_with_stack).with_roles(Role::Button);

    harness.focus_forward();
    harness.next();
    harness.next();

    let result = harness.previous();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Rectangle(_))));

    let result = harness.previous();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));
}

#[test]
fn no_focusable_returns_deferred() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), geometry_reader_no_focusable).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert_eq!(result, EventResult::Deferred);
}

#[test]
fn geometry_reader_in_stack_forward() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), geometry_reader_in_stack).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));

    let result = harness.next();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Rectangle(_))));

    let result = harness.next();
    assert!(result.requested_focus());
    assert!(matches!(
        result.shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));
}

#[test]
fn geometry_reader_in_stack_backward() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), geometry_reader_in_stack)
        .with_roles(Role::Button)
        .with_focus_at_end();

    let result = harness.focus_backward();
    assert!(matches!(
        result.shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));

    let result = harness.previous();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Rectangle(_))));

    let result = harness.previous();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));
}

#[test]
fn nested_geometry_readers_focus() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), nested_geometry_readers).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));

    harness.select();
    assert_eq!(harness.state().a, 1);
}

#[test]
fn focus_backward_from_end() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), geometry_reader_with_stack)
        .with_roles(Role::Button)
        .with_focus_at_end();

    // Focus backward should start at last element
    let result = harness.focus_backward();
    assert!(result.requested_focus());
    assert!(matches!(
        result.shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));
}
