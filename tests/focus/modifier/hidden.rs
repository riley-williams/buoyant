use buoyant::{
    app::{App, Harness as _},
    focus::Role,
    primitives::Size,
    render::ContentShape,
    view::prelude::*,
};

struct State {
    tapped: bool,
}

#[test]
fn hidden_view_is_skipped() {
    fn view(_: &State) -> impl View<(), State> + use<> {
        VStack::new((
            Button::new(|s: &mut State| s.tapped = true, |_| Circle).hidden(),
            Button::new(|s: &mut State| s.tapped = true, |_| Rectangle),
        ))
    }

    let state = State { tapped: false };
    let mut harness = App::new(state, Size::new(100, 100), view).with_roles(Role::Button);

    // Hidden button should be skipped - focus lands on Rectangle button
    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(
        matches!(result.shape(), Some(ContentShape::Rectangle(_))),
        "Should focus second button (Rectangle), not hidden first button (Circle)"
    );
}

#[test]
fn visible_view_can_receive_focus() {
    fn view(_: &State) -> impl View<(), State> + use<> {
        VStack::new((
            Button::new(|s: &mut State| s.tapped = true, |_| Circle),
            Button::new(|s: &mut State| s.tapped = true, |_| Rectangle),
        ))
    }

    let state = State { tapped: false };
    let mut harness = App::new(state, Size::new(100, 100), view).with_roles(Role::Button);

    // First button should be focusable
    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(
        matches!(result.shape(), Some(ContentShape::Circle(_))),
        "Should focus first button (Circle)"
    );
}
