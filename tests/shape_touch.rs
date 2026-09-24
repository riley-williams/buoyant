use buoyant::{
    app::{App, Harness as _},
    focus::Role,
    primitives::{Point, Size},
    view::prelude::*,
};

#[derive(Clone, Default, PartialEq, Eq, Debug)]
struct State {
    taps: u32,
}

/// A solid (non-focusable) overlay should claim touches that start inside it, letting
/// touches that start outside fall through to the button behind it.
fn assert_solid_overlay<V, F>(view_fn: F)
where
    V: View<char, State>,
    V::FocusTree: buoyant::focus::DefaultFocus,
    V::Renderables: buoyant::render::AnimatedJoin,
    F: Fn(&State) -> V,
{
    let mut harness =
        App::new(State::default(), Size::new(10, 10), view_fn).with_roles(Role::Button);

    // Tap inside the overlay: the shape handles the Started touch, so the button
    // behind it never captures and the action never fires.
    harness.tap(Point::new(1, 1));
    assert_eq!(
        harness.state().taps,
        0,
        "touch starting inside the overlay should be claimed by the overlay"
    );

    // Tap outside the overlay but inside the button: the shape defers, the button
    // captures and fires on release.
    harness.tap(Point::new(8, 8));
    assert_eq!(
        harness.state().taps,
        1,
        "touch starting outside the overlay should reach the button"
    );

    // Drag from outside the overlay into it: the shape only handles Started touches
    // inside it, so a touch beginning outside is captured by the button and remains
    // captive even as it moves into the overlay's bounds.
    harness.drag(Point::new(8, 8), Point::new(1, 1));
    assert_eq!(
        harness.state().taps,
        2,
        "touch starting outside the overlay should remain with the button"
    );
}

#[test]
fn solid_shapes_handle_touch() {
    assert_solid_overlay(|_| {
        Button::new(|s: &mut State| s.taps += 1, |_| Rectangle)
            .overlay(Alignment::TopLeading, Rectangle.frame_sized(3, 3))
    });
    assert_solid_overlay(|_| {
        Button::new(|s: &mut State| s.taps += 1, |_| Rectangle)
            .overlay(Alignment::TopLeading, Circle.frame_sized(3, 3))
    });
    assert_solid_overlay(|_| {
        Button::new(|s: &mut State| s.taps += 1, |_| Rectangle)
            .overlay(Alignment::TopLeading, Capsule.frame_sized(3, 3))
    });
    assert_solid_overlay(|_| {
        Button::new(|s: &mut State| s.taps += 1, |_| Rectangle).overlay(
            Alignment::TopLeading,
            RoundedRectangle::new(3).frame_sized(3, 3),
        )
    });
}
