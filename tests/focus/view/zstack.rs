use buoyant::{
    app::{App, Harness as _},
    event::{Event, EventResult, Key},
    focus::{FocusAction, Role},
    primitives::Size,
    render::ContentShape,
    view::{map_event::Mapping, prelude::*},
};

struct State {
    a: u32,
    b: u32,
    c: u32,
}

fn three_button_stack(_: &State) -> impl View<(), State> + use<> {
    ZStack::new((
        Button::new(|s: &mut State| s.a += 1, |_| Circle),
        Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
        Button::new(|s: &mut State| s.c += 1, |_| RoundedRectangle::new(10)),
    ))
}

fn stack_with_unfocusable_back(_: &State) -> impl View<(), State> + use<> {
    ZStack::new((
        Rectangle,
        Button::new(|s: &mut State| s.a += 1, |_| Circle),
        Button::new(|s: &mut State| s.b += 1, |_| RoundedRectangle::new(10)),
    ))
}

fn stack_with_unfocusable_middle(_: &State) -> impl View<(), State> + use<> {
    ZStack::new((
        Button::new(|s: &mut State| s.a += 1, |_| Circle),
        Rectangle,
        Button::new(|s: &mut State| s.b += 1, |_| RoundedRectangle::new(10)),
    ))
}

fn stack_with_unfocusable_front(_: &State) -> impl View<(), State> + use<> {
    ZStack::new((
        Button::new(|s: &mut State| s.a += 1, |_| Circle),
        Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
        Rectangle,
    ))
}

fn stack_no_focusable(_: &State) -> impl View<(), State> + use<> {
    ZStack::new((Rectangle, Rectangle, Rectangle))
}

fn nested_zstack(_: &State) -> impl View<(), State> + use<> {
    ZStack::new((
        ZStack::new((Button::new(|s: &mut State| s.a += 1, |_| Circle), Rectangle)),
        Button::new(|s: &mut State| s.b += 1, |_| RoundedRectangle::new(10)),
    ))
}

#[test]
fn navigate_forward_through_entire_stack() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), three_button_stack).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));

    harness.select();
    assert_eq!(harness.state().a, 1);

    let result = harness.next();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Rectangle(_))));

    harness.select();
    assert_eq!(harness.state().b, 1);

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
fn navigate_backward_through_entire_stack() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), three_button_stack)
        .with_roles(Role::Button)
        .with_focus_at_end();

    let result = harness.focus_backward();
    assert!(result.requested_focus());
    assert!(matches!(
        result.shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));

    harness.select();
    assert_eq!(harness.state().c, 1);

    let result = harness.previous();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Rectangle(_))));

    harness.select();
    assert_eq!(harness.state().b, 1);

    let result = harness.previous();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));

    harness.select();
    assert_eq!(harness.state().a, 1);
}

#[test]
fn skips_unfocusable_back_layer() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), stack_with_unfocusable_back).with_roles(Role::Button);

    // Should skip unfocusable Rectangle and focus first button (Circle)
    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));
}

#[test]
fn skips_unfocusable_middle_layer() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), stack_with_unfocusable_middle)
        .with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));

    // Next should skip Rectangle and focus RoundedRectangle
    let result = harness.next();
    assert!(result.requested_focus());
    assert!(matches!(
        result.shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));
}

#[test]
fn skips_unfocusable_front_layer() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), stack_with_unfocusable_front).with_roles(Role::Button);

    harness.focus_forward();

    let result = harness.next();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Rectangle(_))));

    let result = harness.next();
    assert_eq!(result, EventResult::Deferred);
}

#[test]
fn backward_skips_unfocusable_front_layer() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), stack_with_unfocusable_front)
        .with_roles(Role::Button)
        .with_focus_at_end();

    let result = harness.focus_backward();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Rectangle(_))));
}

#[test]
fn no_focusable_elements_returns_deferred() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), stack_no_focusable).with_roles(Role::Button);

    assert_eq!(harness.focus_forward(), EventResult::Deferred);
}

#[test]
fn nested_zstack_forward_navigation() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), nested_zstack).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));

    let result = harness.next();
    assert!(result.requested_focus());
    assert!(matches!(
        result.shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));
}

#[test]
fn nested_zstack_backward_navigation() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = App::new(state, Size::new(100, 100), nested_zstack).with_roles(Role::Button);

    harness.focus_forward();
    harness.next();

    harness.select();
    assert_eq!(harness.state().b, 1);

    // Since inner ZStack has unfocusable at end, should focus Circle
    let result = harness.previous();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));
}

#[test]
fn previous_from_first_returns_deferred() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), three_button_stack).with_roles(Role::Button);

    harness.focus_forward();

    let result = harness.previous();
    assert_eq!(result, EventResult::Deferred);
}

#[test]
fn next_from_last_returns_deferred() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), three_button_stack).with_roles(Role::Button);

    harness.focus_forward();
    harness.next();
    harness.next();

    let result = harness.next();
    assert_eq!(result, EventResult::Deferred);
}

fn single_item_stack(_: &State) -> impl View<(), State> + use<> {
    ZStack::new((Button::new(|s: &mut State| s.a += 1, |_| Circle),))
}

/// Single-item `ZStack` has its own manual impl
#[test]
fn single_item_focus() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), single_item_stack).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));

    harness.select();
    assert_eq!(harness.state().a, 1);

    // Next from only item should return Deferred
    let result = harness.next();
    assert_eq!(result, EventResult::Deferred);
}

fn key_activatable_button<S>(
    action: impl Fn(&mut State) + 'static,
    shape: impl Fn() -> S + 'static,
) -> impl View<(), State> + 'static
where
    S: View<(), State> + 'static,
{
    Button::new(action, move |_| shape()).map_event(move |event, _: &mut State| match event {
        Event::KeyDown(Key::Character('\n')) => Mapping::Replace(Event::from(FocusAction::Select)),
        Event::KeyUp(_) => Mapping::Defer,
        _ => Mapping::Passthrough,
    })
}

fn key_aware_stack(_: &State) -> impl View<(), State> + use<> {
    ZStack::new((
        key_activatable_button(|s: &mut State| s.a += 1, || Circle),
        key_activatable_button(|s: &mut State| s.b += 1, || Rectangle),
        key_activatable_button(|s: &mut State| s.c += 1, || RoundedRectangle::new(10)),
    ))
}

#[test]
fn key_down_routes_only_to_focused_child() {
    let state = State { a: 0, b: 0, c: 0 };
    let mut harness =
        App::new(state, Size::new(100, 100), key_aware_stack).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
    harness.key_down(Key::Character('\n'));
    assert_eq!(harness.state().a, 1);
    assert_eq!(harness.state().b, 0);
    assert_eq!(harness.state().c, 0);

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Rectangle(_))
    ));
    harness.key_down(Key::Character('\n'));
    assert_eq!(harness.state().a, 1);
    assert_eq!(harness.state().b, 1);
    assert_eq!(harness.state().c, 0);
}
