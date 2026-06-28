use buoyant::{
    app::{App, Harness as _},
    focus::Role,
    primitives::{Point, Size},
    render::ContentShape,
    view::prelude::*,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct State {
    a: u32,
    b: u32,
    c: u32,
}

fn three_buttons_with_focus_touches(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|s: &mut State| s.a += 1, |_| Circle).frame_sized(50, 50),
        Button::new(|s: &mut State| s.b += 1, |_| Rectangle).frame_sized(50, 50),
        RoundedRectangle::new(10).frame_sized(50, 50),
        Button::new(|s: &mut State| s.c += 1, |_| RoundedRectangle::new(5)).frame_sized(50, 50),
    ))
}

#[test]
fn touch_moves_focus() {
    let mut harness = App::new(
        State::default(),
        Size::new(50, 200),
        three_buttons_with_focus_touches,
    )
    .with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    harness.tap(Point::new(25, 175));
    assert!(*harness.state() == State { a: 0, b: 0, c: 1 });
    assert!(matches!(
        harness.select().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));
    assert!(*harness.state() == State { a: 0, b: 0, c: 2 });

    // Still on the circle
    harness.tap(Point::new(25, 25));
    assert!(*harness.state() == State { a: 1, b: 0, c: 2 });
    assert!(matches!(
        harness.select().shape(),
        Some(ContentShape::Circle(_))
    ));
    assert!(*harness.state() == State { a: 2, b: 0, c: 2 });

    // Tap in the non-button rectangle, Circle should still be focused
    harness.tap(Point::new(25, 125));
    assert!(*harness.state() == State { a: 2, b: 0, c: 2 });
    assert!(matches!(
        harness.select().shape(),
        Some(ContentShape::Circle(_))
    ));
    assert!(*harness.state() == State { a: 3, b: 0, c: 2 });
}
