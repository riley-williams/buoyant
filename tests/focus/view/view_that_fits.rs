use buoyant::{
    app::{App, Harness as _},
    event::EventResult,
    focus::Role,
    primitives::Size,
    render::ContentShape,
    view::{FitAxis, ViewThatFits, prelude::*},
};

#[derive(Default)]
struct State {
    a: u32,
    b: u32,
    c: u32,
}

fn two_choice_view(_: &State) -> impl View<(), State> + use<> {
    ViewThatFits::new(FitAxis::Vertical, {
        Button::new(
            |s: &mut State| s.a += 1,
            |_| Circle.flex_frame().with_min_height(50),
        )
    })
    .or(Button::new(
        |s: &mut State| s.b += 1,
        |_| Rectangle.flex_frame().with_min_height(20),
    ))
}

fn three_choice_view(_: &State) -> impl View<(), State> + use<> {
    ViewThatFits::new(FitAxis::Vertical, {
        Button::new(
            |s: &mut State| s.a += 1,
            |_| Circle.flex_frame().with_min_height(100),
        )
    })
    .or(Button::new(
        |s: &mut State| s.b += 1,
        |_| Rectangle.flex_frame().with_min_height(50),
    ))
    .or(Button::new(
        |s: &mut State| s.c += 1,
        |_| RoundedRectangle::new(10).flex_frame().with_min_height(10),
    ))
}

#[test]
fn two_choice_first_selected_is_focusable() {
    // Size 100x100 is large enough for first choice (min height 50)
    let state = State::default();
    let mut harness =
        App::new(state, Size::new(100, 100), two_choice_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn two_choice_first_selected_select_triggers_correct_action() {
    let state = State::default();
    let mut harness =
        App::new(state, Size::new(100, 100), two_choice_view).with_roles(Role::Button);

    harness.focus_forward();
    harness.select();

    assert_eq!(harness.state().a, 1);
    assert_eq!(harness.state().b, 0);
}

#[test]
fn two_choice_first_selected_next_returns_deferred() {
    let state = State::default();
    let mut harness =
        App::new(state, Size::new(100, 100), two_choice_view).with_roles(Role::Button);

    harness.focus_forward();
    let result = harness.next();
    assert_eq!(result, EventResult::Deferred);
}

#[test]
fn two_choice_second_selected_is_focusable() {
    // Size 30x30 is too small for first choice (min height 50) but fits second (min height 20)
    let state = State::default();
    let mut harness = App::new(state, Size::new(30, 30), two_choice_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));
}

#[test]
fn two_choice_second_selected_select_triggers_correct_action() {
    let state = State::default();
    let mut harness = App::new(state, Size::new(30, 30), two_choice_view).with_roles(Role::Button);

    harness.focus_forward();
    harness.select();

    assert_eq!(harness.state().a, 0,);
    assert_eq!(harness.state().b, 1);
}

#[test]
fn two_choice_second_selected_previous_returns_deferred() {
    let state = State::default();
    let mut harness = App::new(state, Size::new(30, 30), two_choice_view)
        .with_roles(Role::Button)
        .with_focus_at_end();

    harness.focus_backward();
    let result = harness.previous();
    assert_eq!(result, EventResult::Deferred);
}

#[test]
fn three_choice_first_selected() {
    // Size 200x200 fits first choice (min height 100)
    let state = State::default();
    let mut harness =
        App::new(state, Size::new(200, 200), three_choice_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    harness.select();
    assert_eq!(harness.state().a, 1);
}

#[test]
fn three_choice_second_selected() {
    // Size 60x60 is too small for first (min 100) but fits second (min 50)
    let state = State::default();
    let mut harness =
        App::new(state, Size::new(60, 60), three_choice_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    harness.select();
    assert_eq!(harness.state().b, 1);
}

#[test]
fn three_choice_third_selected() {
    // 20x20 is too small for first (min 100) and second (min 50) but fits third (min 10)
    let state = State::default();
    let mut harness =
        App::new(state, Size::new(20, 20), three_choice_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::RoundedRectangle { .. })
    ));

    harness.select();
    assert_eq!(harness.state().c, 1);
}

/// Nested inside a `VStack` with minimum sizes, mostly for sanity checking
fn stack_with_view_that_fits(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(
            |s: &mut State| s.a += 1,
            |_| Circle.flex_frame().with_min_height(10),
        ),
        ViewThatFits::new(FitAxis::Vertical, {
            Button::new(
                |s: &mut State| s.b += 1,
                |_| Rectangle.flex_frame().with_min_height(50),
            )
        })
        .or(Button::new(
            |s: &mut State| s.c += 1,
            |_| RoundedRectangle::new(5).flex_frame().with_min_height(10),
        )),
    ))
}

#[test]
fn stack_navigation_first_vtf_choice() {
    // Large space: ViewThatFits selects first choice (Rectangle, min height 50)
    let state = State::default();
    let mut harness =
        App::new(state, Size::new(100, 100), stack_with_view_that_fits).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Rectangle(_))
    ),);

    harness.select();
    assert_eq!(harness.state().b, 1);
}

#[test]
fn stack_navigation_second_vtf_choice() {
    // Constrained space: ViewThatFits selects second choice (RoundedRectangle, min height 10)
    // With height 30, VStack has ~20 for ViewThatFits after Circle's min 10
    let state = State::default();
    let mut harness =
        App::new(state, Size::new(100, 30), stack_with_view_that_fits).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::RoundedRectangle { .. })
    ));

    harness.select();
    assert_eq!(harness.state().c, 1);
}

#[test]
fn stack_backward_navigation() {
    let state = State::default();
    let mut harness =
        App::new(state, Size::new(100, 100), stack_with_view_that_fits).with_roles(Role::Button);

    harness.focus_forward();
    harness.next();

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn two_choice_focus_backward_first_selected() {
    let state = State::default();
    let mut harness = App::new(state, Size::new(100, 100), two_choice_view)
        .with_roles(Role::Button)
        .with_focus_at_end();

    assert!(matches!(
        harness.focus_backward().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn two_choice_focus_backward_second_selected() {
    let state = State::default();
    let mut harness = App::new(state, Size::new(30, 30), two_choice_view)
        .with_roles(Role::Button)
        .with_focus_at_end();

    assert!(matches!(
        harness.focus_backward().shape(),
        Some(ContentShape::Rectangle(_))
    ));
}

#[test]
fn three_choice_focus_backward_third_selected() {
    let state = State::default();
    let mut harness = App::new(state, Size::new(20, 20), three_choice_view)
        .with_roles(Role::Button)
        .with_focus_at_end();

    assert!(matches!(
        harness.focus_backward().shape(),
        Some(ContentShape::RoundedRectangle { .. })
    ));
}

#[test]
fn resize_from_first_to_second_choice_while_focused() {
    // Start with first choice selected (large space)
    let state = State::default();
    let mut harness =
        App::new(state, Size::new(100, 100), two_choice_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    harness.resize(Size::new(30, 30));
    harness.finalize_view();

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    harness.select();
    assert_eq!(harness.state().b, 1);
}

#[test]
fn resize_from_second_to_first_choice_while_focused() {
    // Start with second choice selected (constrained space)
    let state = State::default();
    let mut harness = App::new(state, Size::new(30, 30), two_choice_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    // Expand the space so first choice fits again
    harness.resize(Size::new(100, 100));
    harness.finalize_view();

    // Focus should now be on first choice
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    harness.select();
    assert_eq!(harness.state().a, 1);
}

#[test]
fn resize_three_choice_cycles_through_all() {
    let state = State::default();
    let mut harness =
        App::new(state, Size::new(200, 200), three_choice_view).with_roles(Role::Button);

    // Large space: first choice
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
    harness.select();
    assert_eq!(harness.state().a, 1);

    // Medium space: second choice
    harness.resize(Size::new(60, 60));
    harness.finalize_view();
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));
    harness.select();
    assert_eq!(harness.state().b, 1);

    // Small space: third choice
    harness.resize(Size::new(20, 20));
    harness.finalize_view();
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::RoundedRectangle { .. })
    ));
    harness.select();
    assert_eq!(harness.state().c, 1);

    // Back to large space: first choice again
    harness.resize(Size::new(200, 200));
    harness.finalize_view();
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
    harness.select();
    assert_eq!(harness.state().a, 2);
}
