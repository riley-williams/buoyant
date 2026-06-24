//! Focus handling tests for `MatchView` (aka `OneOf`) views
//!
//! Tests focus behavior when the active variant changes

use buoyant::{
    app::{App, Harness as _},
    event::EventResult,
    focus::Role,
    match_view,
    primitives::Size,
    render::ContentShape,
    view::prelude::*,
};

#[derive(Clone)]
struct State {
    variant: Variant,
    a: u32,
    b: u32,
    c: u32,
}

#[derive(Clone, Copy, PartialEq)]
enum Variant {
    First,
    Second,
    Third,
}

/// `MatchView` with two different button shapes
fn two_branch_view(state: &State) -> impl View<(), State> + use<> {
    match_view!(state.variant, {
        Variant::First => Button::new(|s: &mut State| s.a += 1, |_| Circle),
        _ => Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
    })
}

/// `MatchView` with three different button shapes
fn three_branch_view(state: &State) -> impl View<(), State> + use<> {
    match_view!(state.variant, {
        Variant::First => Button::new(|s: &mut State| s.a += 1, |_| Circle),
        Variant::Second => Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
        Variant::Third => Button::new(|s: &mut State| s.c += 1, |_| RoundedRectangle::new(10)),
    })
}

/// `MatchView` where one branch has no focusable content
fn view_with_unfocusable_branch(state: &State) -> impl View<(), State> + use<> {
    match_view!(state.variant, {
        Variant::First => Button::new(|s: &mut State| s.a += 1, |_| Circle),
        _ => Rectangle,
    })
}

/// `VStack` containing a `MatchView` to test navigation across variant changes
fn stack_with_match_view(state: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|s: &mut State| s.a += 1, |_| Circle),
        match_view!(state.variant, {
            Variant::First => Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
            _ => Button::new(|s: &mut State| s.c += 1, |_| RoundedRectangle::new(10)),
        }),
    ))
}

#[test]
fn first_variant_is_focusable() {
    let state = State {
        variant: Variant::First,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), two_branch_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn second_variant_is_focusable() {
    let state = State {
        variant: Variant::Second,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), two_branch_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));
}

#[test]
fn select_triggers_action_on_correct_variant() {
    let state = State {
        variant: Variant::Second,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), three_branch_view).with_roles(Role::Button);

    harness.focus_forward();
    harness.select();

    assert_eq!(
        harness.state().a,
        0,
        "First variant action should not trigger"
    );
    assert_eq!(
        harness.state().b,
        1,
        "Second variant action should trigger once"
    );
    assert_eq!(
        harness.state().c,
        0,
        "Third variant action should not trigger"
    );
}

#[test]
fn unfocusable_variant_returns_deferred() {
    let state = State {
        variant: Variant::Second, // This maps to Rectangle (not a button)
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), view_with_unfocusable_branch).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(
        matches!(result, EventResult::Deferred),
        "Unfocusable variant should return Deferred"
    );
}

#[test]
fn next_from_match_view_returns_deferred() {
    // MatchView has only one active branch, so Next should return Deferred
    let state = State {
        variant: Variant::First,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), two_branch_view).with_roles(Role::Button);

    harness.focus_forward();
    let result = harness.next();
    assert!(
        matches!(result, EventResult::Deferred),
        "Next from single-element match view should return Deferred"
    );
}

#[test]
fn previous_from_match_view_returns_deferred() {
    // MatchView has only one active branch, so Previous should return Deferred
    let state = State {
        variant: Variant::First,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), two_branch_view).with_roles(Role::Button);

    harness.focus_forward();
    let result = harness.previous();
    assert!(
        matches!(result, EventResult::Deferred),
        "Previous from single-element match view should return Deferred"
    );
}

#[test]
fn stack_navigation_through_match_view() {
    let state = State {
        variant: Variant::First,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), stack_with_match_view).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    harness.select();
    assert_eq!(
        harness.state().b,
        1,
        "Rectangle button action should trigger"
    );
}

#[test]
fn third_variant_is_focusable() {
    let state = State {
        variant: Variant::Third,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), three_branch_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));
}

#[test]
fn focus_backward_on_match_view() {
    let state = State {
        variant: Variant::First,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness = App::new(state, Size::new(100, 100), two_branch_view)
        .with_roles(Role::Button)
        .with_focus_at_end();

    // Focus backward should still find the button
    assert!(matches!(
        harness.focus_backward().shape(),
        Some(ContentShape::Circle(_))
    ));
}

// =============================================================================
// Variant change tests
// =============================================================================

#[test]
fn variant_change_while_focused_first_to_second() {
    let state = State {
        variant: Variant::First,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), two_branch_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    // Change variant and rebuild
    harness.state_mut().variant = Variant::Second;
    harness.force_rebuild();

    let result = harness.focus_forward();
    assert!(matches!(result.shape(), Some(ContentShape::Rectangle(_))));

    harness.select();
    assert_eq!(harness.state().b, 1, "Second variant action should trigger");
}

#[test]
fn variant_change_while_focused_second_to_first() {
    let state = State {
        variant: Variant::Second,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), two_branch_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    harness.state_mut().variant = Variant::First;
    harness.force_rebuild();

    let result = harness.focus_forward();
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));

    harness.select();
    assert_eq!(harness.state().a, 1, "First variant action should trigger");
}

#[test]
fn variant_change_cycles_through_all_three() {
    let state = State {
        variant: Variant::First,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), three_branch_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
    harness.select();
    assert_eq!(harness.state().a, 1);

    harness.state_mut().variant = Variant::Second;
    harness.force_rebuild();

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));
    harness.select();
    assert_eq!(harness.state().b, 1);

    harness.state_mut().variant = Variant::Third;
    harness.force_rebuild();

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));

    harness.select();
    assert_eq!(harness.state().c, 1);

    harness.state_mut().variant = Variant::First;
    harness.force_rebuild();
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
    harness.select();
    assert_eq!(harness.state().a, 2);
}

#[test]
fn variant_change_to_unfocusable_branch() {
    let state = State {
        variant: Variant::First,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), view_with_unfocusable_branch).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    // Change to unfocusable variant
    harness.state_mut().variant = Variant::Second;
    harness.force_rebuild();

    // Focus should return Deferred since Rectangle is not a button
    let result = harness.focus_forward();
    assert!(
        matches!(result, EventResult::Deferred),
        "Unfocusable variant should return Deferred"
    );
}

#[test]
fn variant_change_from_unfocusable_to_focusable() {
    let state = State {
        variant: Variant::Second, // Rectangle (not a button)
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), view_with_unfocusable_branch).with_roles(Role::Button);

    // No focus available initially
    let result = harness.focus_forward();
    assert!(matches!(result, EventResult::Deferred));

    // Change to focusable variant
    harness.state_mut().variant = Variant::First;
    harness.force_rebuild();

    // Now focus should work
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
}

/// Focus in stacks can be weird, this is mostly for sanity
#[test]
fn stack_variant_change_while_focused_on_match_view() {
    let state = State {
        variant: Variant::First,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut harness =
        App::new(state, Size::new(100, 100), stack_with_match_view).with_roles(Role::Button);

    // Focus the first button, then navigate to match_view
    harness.focus_forward();
    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    // Change variant while focused on match_view
    harness.state_mut().variant = Variant::Second;
    harness.force_rebuild();

    // After rebuild, focus_forward tries to acquire focus at current position.
    // The focus tree still points to the match_view (second element in VStack),
    // which now contains a RoundedRectangle button.
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));

    // Verify correct action triggers
    harness.select();
    assert_eq!(
        harness.state().c,
        1,
        "RoundedRectangle button action should trigger"
    );
}
