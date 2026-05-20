use crate::assert_str_grid_eq;
use buoyant::{
    app::{App, Harness as _},
    event::EventResult,
    focus::{BoundaryBehavior, Role},
    primitives::{Point, Size},
    render::ContentShape,
    render_target::FixedTextBuffer,
    view::prelude::*,
};

struct State {
    tapped: bool,
}

fn single_button_view(_: &State) -> impl View<(), State> + use<> {
    Button::new(|s: &mut State| s.tapped = true, |_| Circle)
}

fn indicator_button() -> impl View<char, State> + use<> {
    Button::new(
        |_s: &mut State| {},
        |a| {
            let color = match (a.is_focused(), a.is_pressed()) {
                (true, true) => 'x',
                (true, false) => 'f',
                (false, true) => 'p',
                (false, false) => 'o',
            };
            Rectangle.foreground_color(color)
        },
    )
}

fn multi_button(_: &State) -> impl View<char, State> + use<> {
    HStack::new((indicator_button(), indicator_button(), Rectangle))
        .bound_focus(BoundaryBehavior::Stop)
        // .multiplex_focus::<1>()
        .focus_touches()
}

#[test]
fn single_button_focus() {
    let state = State { tapped: false };
    let mut harness =
        App::new(state, Size::new(100, 100), single_button_view).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(
        result.requested_focus(),
        "Single button should be focusable"
    );
    assert!(matches!(result.shape(), Some(ContentShape::Circle(_))));
}

#[test]
fn single_button_next_returns_deferred() {
    let state = State { tapped: false };
    let mut harness =
        App::new(state, Size::new(100, 100), single_button_view).with_roles(Role::Button);

    // Focus the button
    harness.focus_forward();

    // Next on single element should return Deferred
    let result = harness.next();
    assert_eq!(result, EventResult::deferred_lost_focus());
}

#[test]
fn select_triggers_action() {
    let state = State { tapped: false };
    let mut harness =
        App::new(state, Size::new(100, 100), single_button_view).with_roles(Role::Button);

    harness.focus_forward();
    assert!(!harness.state().tapped);

    harness.select();
    assert!(
        harness.state().tapped,
        "Select should trigger button action"
    );
}

#[test]
fn tap_first_moves_focus() {
    let mut target = FixedTextBuffer::<6, 1>::default();
    let state = State { tapped: false };
    let mut harness = App::new(state, target.size(), multi_button).with_roles(Role::Button);

    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["oooo  ",], &target.text);

    harness.focus_forward();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ffoo  ",], &target.text);

    harness.next();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ooff  ",], &target.text);

    harness.next();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ooff  ",], &target.text);

    harness.touch_down(Point::zero());
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ppff  ",], &target.text);

    harness.touch_up(Point::zero());
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ffoo  ",], &target.text);
}

#[test]
fn tap_second_moves_focus() {
    let mut target = FixedTextBuffer::<6, 1>::default();
    let state = State { tapped: false };
    let mut harness = App::new(state, target.size(), multi_button).with_roles(Role::Button);

    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["oooo  ",], &target.text);

    harness.focus_forward();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ffoo  ",], &target.text);

    harness.touch_down(Point::new(3, 0));
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ffpp  ",], &target.text);

    harness.touch_up(Point::new(3, 0));
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ooff  ",], &target.text);
}

#[test]
fn tap_cancel_doesnt_move_focus() {
    let mut target = FixedTextBuffer::<6, 1>::default();
    let state = State { tapped: false };
    let mut harness = App::new(state, target.size(), multi_button).with_roles(Role::Button);

    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["oooo  ",], &target.text);

    harness.focus_forward();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ffoo  ",], &target.text);

    harness.touch_down(Point::new(3, 0));
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ffpp  ",], &target.text);

    // move off the button before lifting up
    harness.touch_up(Point::new(0, 0));
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ffoo  ",], &target.text);
}

#[test]
fn tap_nowhere_doesnt_move_focus() {
    let mut target = FixedTextBuffer::<6, 1>::default();
    let state = State { tapped: false };
    let mut harness = App::new(state, target.size(), multi_button).with_roles(Role::Button);

    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["oooo  ",], &target.text);

    harness.focus_forward();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ffoo  ",], &target.text);

    harness.touch_down(Point::new(5, 0));
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ffoo  ",], &target.text);

    harness.touch_up(Point::zero());
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["ffoo  ",], &target.text);
}
