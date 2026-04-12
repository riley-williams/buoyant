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
fn clipped_view_can_receive_focus() {
    fn view(_: &State) -> impl View<(), State> + use<> {
        Button::new(|s: &mut State| s.tapped = true, |_| Circle)
            .frame_sized(100, 100)
            .clipped()
    }

    let state = State { tapped: false };
    let mut harness = App::new(state, Size::new(100, 100), view).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(
        matches!(result.shape(), Some(ContentShape::Circle(_))),
        "Clipped button should still be focusable"
    );
}
