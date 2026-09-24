use buoyant::{app::Harness as _, view::prelude::*};

mod common;
use common::app;

#[derive(Clone, Default, PartialEq, Eq, Debug)]
struct State {
    taps: u32,
}

fn clipped_button_view(_: &State) -> impl View<char, State> + use<> {
    Button::new(|s: &mut State| s.taps += 1, |_| Rectangle)
        .frame_sized(5, 5)
        .clipped()
}

/// `clipped` only culls the *start* of a touch that begins outside the clip rect; drags
/// that begin outside never capture the button even as they move inside.
#[test]
fn clipped_culls_touches_starting_outside() {
    let mut harness = app(State::default(), clipped_button_view);

    // The button is clipped to a 5x5 frame at the top-left. (1,1) is inside the clip rect.
    harness.tap(buoyant::primitives::Point::new(1, 1));
    assert_eq!(
        harness.state().taps,
        1,
        "tap inside the clip rect should fire the button"
    );

    // (8,8) is outside the clip rect; the Started touch is culled so the button never
    // captures and the subsequent Ended touch does nothing.
    harness.tap(buoyant::primitives::Point::new(8, 8));
    assert_eq!(
        harness.state().taps,
        1,
        "tap starting outside the clip rect should be culled"
    );

    // A drag starting outside the clip and moving in: the Started touch is culled, so the
    // button never captures and the Moved/Ended touches (which pass through) find it at rest.
    harness.drag(
        buoyant::primitives::Point::new(8, 8),
        buoyant::primitives::Point::new(1, 1),
    );
    assert_eq!(
        harness.state().taps,
        1,
        "drag starting outside the clip rect should not fire the button"
    );
}
