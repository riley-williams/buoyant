use buoyant::{
    app::{App, Harness as _},
    focus::Role,
    primitives::Size,
    render::ContentShape,
    view::prelude::*,
};

fn test_view(state: &State) -> impl View<(), State> + use<> {
    Button::new(|s: &mut State| s.main_tapped = true, |_| Rectangle).popover(
        state.popover_visible.as_ref(),
        |()| {
            VStack::new((
                Rectangle,
                Button::new(|s: &mut State| s.popover_a_tapped = true, |_| Circle)
                    .frame_sized(50, 50),
                Button::new(
                    |s: &mut State| s.popover_b_tapped = true,
                    |_| RoundedRectangle::new(5),
                )
                .frame_sized(50, 50),
            ))
        },
    )
}

#[derive(Clone, Default)]
struct State {
    main_tapped: bool,
    popover_a_tapped: bool,
    popover_b_tapped: bool,
    popover_visible: Option<()>,
}

impl State {
    fn with_popover(mut self) -> Self {
        self.popover_visible = Some(());
        self
    }
}

#[test]
fn popover_shown_receives_initial_focus() {
    let state = State::default().with_popover();

    let mut harness = App::new(state, Size::new(100, 100), test_view).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(
        matches!(result.shape(), Some(ContentShape::Circle(_))),
        "Popover content (Circle) should receive focus"
    );
}

#[test]
fn popover_hidden_shows_main_view() {
    let state = State::default();
    let mut harness = App::new(state, Size::new(100, 100), test_view).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(
        matches!(result.shape(), Some(ContentShape::Rectangle(_))),
        "Main button (Rectangle) should be focusable when popover is hidden"
    );
}

#[test]
fn popover_wraps_focus_forward() {
    let state = State::default().with_popover();
    let mut harness = App::new(state, Size::new(100, 100), test_view).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(
        matches!(result.shape(), Some(ContentShape::Circle(_))),
        "First element should be Circle"
    );

    let result = harness.next();
    assert!(result.requested_focus());
    assert!(
        matches!(result.shape(), Some(ContentShape::RoundedRectangle(_))),
        "Second element should be RoundedRectangle, got {:?}",
        result.shape()
    );

    let result = harness.next();
    assert!(result.requested_focus());
    assert!(
        matches!(result.shape(), Some(ContentShape::Circle(_))),
        "Should wrap to first element (Circle) when moving forward past end"
    );
}

#[test]
fn popover_wraps_focus_backward() {
    let state = State::default().with_popover();
    let mut harness = App::new(state, Size::new(100, 100), test_view).with_roles(Role::Button);

    let result = harness.focus_forward();
    assert!(result.requested_focus());
    assert!(
        matches!(result.shape(), Some(ContentShape::Circle(_))),
        "First element should be Circle"
    );

    let result = harness.previous();
    assert!(result.requested_focus());
    assert!(
        matches!(result.shape(), Some(ContentShape::RoundedRectangle(_))),
        "Should wrap to last element (RoundedRectangle) when moving backward from start, got {:?}",
        result.shape()
    );
}
