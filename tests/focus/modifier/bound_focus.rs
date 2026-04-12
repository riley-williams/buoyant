//! Focus handling tests for `bound_focus` modifier
//!
//! Tests focus boundary behavior with Wrap and Stop modes

use buoyant::{
    app::{App, Harness as _},
    event::EventResult,
    focus::{BoundaryBehavior, Role},
    primitives::Size,
    render::ContentShape,
    view::prelude::*,
};

#[derive(Clone)]
struct State;

/// Three buttons in a `VStack` with Wrap behavior
fn three_buttons_wrap(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|_: &mut State| {}, |_| Circle).frame_sized(50, 50),
        Button::new(|_: &mut State| {}, |_| Rectangle).frame_sized(50, 50),
        Button::new(|_: &mut State| {}, |_| RoundedRectangle::new(5)).frame_sized(50, 50),
    ))
    .bound_focus(BoundaryBehavior::Wrap)
}

/// Three buttons in a `VStack` with Stop behavior
fn three_buttons_stop(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|_: &mut State| {}, |_| Circle).frame_sized(50, 50),
        Button::new(|_: &mut State| {}, |_| Rectangle).frame_sized(50, 50),
        Button::new(|_: &mut State| {}, |_| RoundedRectangle::new(5)).frame_sized(50, 50),
    ))
    .bound_focus(BoundaryBehavior::Stop)
}

/// No focusable elements
fn no_focusable_elements(_: &State) -> impl View<(), State> + use<> {
    VStack::new((Rectangle, Rectangle, Rectangle)).bound_focus(BoundaryBehavior::Wrap)
}

/// Single focusable element
fn single_focusable(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Rectangle,
        Button::new(|_: &mut State| {}, |_| Circle).frame_sized(50, 50),
        Rectangle,
    ))
    .bound_focus(BoundaryBehavior::Wrap)
}

#[test]
fn wrap_forward_navigation() {
    let mut harness =
        App::new(State, Size::new(100, 200), three_buttons_wrap).with_roles(Role::Button);

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
        harness.next().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn wrap_backward_navigation() {
    let mut harness =
        App::new(State, Size::new(100, 200), three_buttons_wrap).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));
}

#[test]
fn stop_forward_navigation() {
    let mut harness =
        App::new(State, Size::new(100, 200), three_buttons_stop).with_roles(Role::Button);

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
        harness.next().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));
}

#[test]
fn stop_backward_navigation() {
    let mut harness =
        App::new(State, Size::new(100, 200), three_buttons_stop).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn no_focusable_elements_returns_deferred() {
    let mut harness =
        App::new(State, Size::new(100, 200), no_focusable_elements).with_roles(Role::Button);

    assert!(matches!(harness.focus_forward(), EventResult::Deferred));
}

#[test]
fn single_element_wrap_stays_on_element() {
    let mut harness =
        App::new(State, Size::new(100, 200), single_focusable).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    // Navigate forward - should wrap back to same element
    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Circle(_))
    ));

    // Navigate backward - should wrap back to same element
    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn blur_passes_through() {
    let mut harness =
        App::new(State, Size::new(100, 200), three_buttons_wrap).with_roles(Role::Button);

    harness.focus_forward();
    assert!(matches!(harness.blur(), EventResult::Deferred));
}

#[test]
fn select_passes_through_when_not_handled() {
    let mut harness =
        App::new(State, Size::new(100, 200), no_focusable_elements).with_roles(Role::Button);

    assert!(matches!(harness.select(), EventResult::Deferred));
}
