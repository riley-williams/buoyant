use buoyant::{
    environment::DefaultEnvironment,
    event::{EventContext, TouchResult},
    primitives::{Point, Size},
    view::prelude::*,
};

mod common;
use common::{touch_down, touch_move};

/// `background_color` claims a touch that starts inside the background shape when the
/// foreground defers; it does not claim non-Started touches or touches starting outside.
#[allow(clippy::let_unit_value)]
#[test]
fn background_color_claims_started_touch_in_background_shape() {
    // The foreground is empty (always defers); the background is a 5x5 rectangle filling the
    // foreground's frame at (0,0)-(5,5).
    let view = EmptyView.frame_sized(5, 5).background_color(' ', Rectangle);
    let mut state = view.build_state(&mut ());
    let layout = view.layout(
        &Size::new(10, 10).into(),
        &DefaultEnvironment::default(),
        &mut (),
        &mut state,
    );
    let mut tree = view.render_tree(
        &layout.sublayouts,
        Point::zero(),
        &DefaultEnvironment::default(),
        &mut (),
        &mut state,
    );
    let ctx = EventContext::new(core::time::Duration::ZERO);

    // A touch starting inside the background rectangle is handled.
    let result = view.handle_touch(&touch_down(1, 1), &ctx, &mut tree, &mut (), &mut state);
    assert_eq!(
        result,
        TouchResult::Handled,
        "started touch inside the background should be handled"
    );

    // A touch starting outside the background rectangle is not handled.
    let result = view.handle_touch(&touch_down(8, 8), &ctx, &mut tree, &mut (), &mut state);
    assert_eq!(
        result,
        TouchResult::Deferred,
        "started touch outside the background should defer"
    );

    // A non-Started touch inside the background is not handled (only Started touches are claimed).
    let result = view.handle_touch(&touch_move(1, 1), &ctx, &mut tree, &mut (), &mut state);
    assert_eq!(
        result,
        TouchResult::Deferred,
        "moved touch inside the background should defer"
    );
}
