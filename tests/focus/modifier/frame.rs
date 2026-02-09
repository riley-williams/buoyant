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
fn frame_sets_shape_size() {
    fn view(_: &State) -> impl View<(), State> + use<> {
        Button::new(|s: &mut State| s.tapped = true, |_| Circle).frame_sized(50, 50)
    }

    let state = State { tapped: false };
    let mut harness = App::new(state, Size::new(100, 100), view).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());

    // Shape should reflect the explicit frame size
    if let Some(ContentShape::Circle(circle)) = result.shape() {
        assert_eq!(
            circle.diameter, 50,
            "Circle diameter should match frame width"
        );
    } else {
        panic!("Expected Circle shape");
    }
}

#[test]
fn frame_with_padding_accumulates() {
    fn view(_: &State) -> impl View<(), State> + use<> {
        Button::new(|s: &mut State| s.tapped = true, |_| Circle)
            .frame_sized(40, 40)
            .padding(Edges::All, 5)
    }

    let state = State { tapped: false };
    let mut harness = App::new(state, Size::new(100, 100), view).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());

    // Padding affects the outer layout but the inner shape remains the Circle
    if let Some(ContentShape::Circle(circle)) = result.shape() {
        // The button's shape reflects the inner content (40x40)
        assert_eq!(circle.diameter, 40, "Circle should match inner frame size");
    } else {
        panic!("Expected Circle shape");
    }
}
