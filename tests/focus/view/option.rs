use buoyant::{
    app::{App, Harness as _},
    focus::Role,
    primitives::Size,
    render::ContentShape,
    view::prelude::*,
};

struct State {
    tapped_a: bool,
    tapped_b: bool,
}

fn view_with_some(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Some(Button::new(|s: &mut State| s.tapped_a = true, |_| Circle)),
        Button::new(|s: &mut State| s.tapped_b = true, |_| Rectangle),
    ))
}

fn view_with_none(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        None::<Button<fn(ButtonState) -> Circle, Circle, fn(&mut State)>>,
        Button::new(|s: &mut State| s.tapped_b = true, |_| Rectangle),
    ))
}

/// Only a None variant - no focusable content
fn view_only_none(_: &State) -> impl View<(), State> + use<> {
    None::<Button<fn(ButtonState) -> Circle, Circle, fn(&mut State)>>
}

#[test]
fn some_is_focusable() {
    let state = State {
        tapped_a: false,
        tapped_b: false,
    };
    let mut harness = App::new(state, Size::new(100, 100), view_with_some).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn none_skips_to_next() {
    let state = State {
        tapped_a: false,
        tapped_b: false,
    };
    let mut harness = App::new(state, Size::new(100, 100), view_with_none).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));
}

#[test]
fn none_returns_deferred() {
    let state = State {
        tapped_a: false,
        tapped_b: false,
    };
    let mut harness = App::new(state, Size::new(100, 100), view_only_none).with_roles(Role::Button);

    assert!(
        matches!(
            harness.focus_forward(),
            buoyant::event::EventResult::Deferred { .. }
        ),
        "None should return Deferred"
    );
}
