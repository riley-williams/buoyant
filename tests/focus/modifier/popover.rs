use buoyant::{
    app::{App, Harness as _},
    event::{Event, Key},
    focus::{FocusAction, Role},
    primitives::Size,
    render::ContentShape,
    view::{popover::Dismissal, prelude::*},
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

fn key_activatable_button<S>(
    action: impl Fn(&mut State) + 'static,
    shape: impl Fn() -> S + 'static,
) -> impl View<(), State> + 'static
where
    S: View<(), State> + 'static,
{
    Button::new(action, move |_| shape()).map_event::<(), _>(move |event, ()| match event {
        Event::KeyDown(Key::Character('\n')) => Some(Event::from(FocusAction::Select)),
        Event::KeyUp(_) => None,
        _ => Some(event.clone()),
    })
}

fn key_aware_popover_view(state: &State) -> impl View<(), State> + use<> {
    key_activatable_button(|s: &mut State| s.main_tapped = true, || Rectangle).popover(
        state.popover_visible.as_ref(),
        |()| {
            VStack::new((
                Rectangle,
                key_activatable_button(|s: &mut State| s.popover_a_tapped = true, || Circle)
                    .frame_sized(50, 50),
                key_activatable_button(
                    |s: &mut State| s.popover_b_tapped = true,
                    || RoundedRectangle::new(5),
                )
                .frame_sized(50, 50),
            ))
        },
    )
}

#[test]
fn key_routes_to_overlay_when_present() {
    let state = State::default().with_popover();
    let mut harness =
        App::new(state, Size::new(100, 100), key_aware_popover_view).with_roles(Role::Button);

    // Focus enters the overlay which is initially visible
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
    // A key event must reach the overlay's focused button, not the inner view.
    harness.key_down(Key::Character('\n'));
    assert!(harness.state().popover_a_tapped);
    assert!(!harness.state().main_tapped);
}

#[test]
fn key_routes_to_inner_when_overlay_absent() {
    let state = State::default();
    let mut harness =
        App::new(state, Size::new(100, 100), key_aware_popover_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));
    harness.key_down(Key::Character('\n'));
    assert!(harness.state().main_tapped);
    assert!(!harness.state().popover_a_tapped);
}

fn dismissible_popover_view(state: &State) -> impl View<(), State> + use<> {
    Button::new(|s: &mut State| s.main_tapped = true, |_| Rectangle)
        .popover(state.popover_visible.as_ref(), |()| {
            VStack::new((
                Rectangle,
                Button::new(|s: &mut State| s.popover_a_tapped = true, |_| Circle)
                    .frame_sized(50, 50),
            ))
        })
        .on_blur(|s: &mut State| {
            s.popover_visible = None;
            Dismissal::Dismiss
        })
}

/// When the overlay's focused child defers a `Blur`, the popover consults the
/// `on_blur` handler. Returning [`Dismissal::Dismiss`] drops the overlay's
/// focus subtree and requests a view rebuild; the callback clears the state
/// driving the overlay's visibility, so the next focus acquisition lands on
/// the inner view.
#[test]
fn blur_releases_overlay_via_on_dismiss() {
    let state = State::default().with_popover();
    let mut harness =
        App::new(state, Size::new(100, 100), dismissible_popover_view).with_roles(Role::Button);

    // Overlay is active; focus is on the first overlay button.
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    // Blurring should dismiss the overlay. The result is handled with focus
    // returned to the inner view (Rectangle).
    let result = harness.blur();
    assert!(result.is_handled());
    assert!(
        matches!(result.shape(), Some(ContentShape::Rectangle(_))),
        "Focus should return to the inner view (Rectangle), got {:?}",
        result.shape()
    );

    // The dismiss callback cleared the overlay visibility flag.
    assert!(
        harness.state().popover_visible.is_none(),
        "on_blur should have cleared the overlay visibility flag"
    );

    // The view rebuild is pending (requested by the dismiss). Finalizing it
    // rebuilds the view without the overlay and re-acquires focus, which must
    // land on the inner view (Rectangle).
    harness.finalize_view();
    assert!(
        matches!(harness.focus_shape(), ContentShape::Rectangle(_)),
        "Focus should return to the inner view (Rectangle) after rebuild, got {:?}",
        harness.focus_shape()
    );
}

/// `Teardown` must not wrap focus within the overlay nor invoke the dismiss
/// callback. The overlay focus subtree is simply dropped along with the rest
/// of the stale focus tree.
#[test]
fn teardown_does_not_dismiss() {
    let state = State::default().with_popover();
    let mut harness =
        App::new(state, Size::new(100, 100), dismissible_popover_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    // Send a Teardown event. The focused Button handles it by returning
    // handled_unfocused (resetting its focused state); the popover proxies
    // that result without wrapping or invoking on_blur.
    let result = harness.send(FocusAction::Teardown);
    assert!(
        result.is_handled(),
        "Teardown proxied to the focused child should be handled"
    );
    assert!(
        harness.state().popover_visible.is_some(),
        "on_blur must not fire for Teardown"
    );
}

/// Without an `on_blur` handler, the default [`NoDismiss`] returns
/// [`Dismissal::Retain`]: blur does not tear down the overlay.
#[test]
fn blur_retains_overlay_by_default() {
    let state = State::default().with_popover();
    let mut harness =
        App::new(state, Size::new(100, 100), key_aware_popover_view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    let result = harness.blur();
    assert!(result.is_handled());
    // No content shape for a retained blur.
    assert!(
        matches!(result.shape(), Some(ContentShape::Empty)),
        "Retained blur should report an empty content shape, got {:?}",
        result.shape()
    );
    assert!(
        harness.state().popover_visible.is_some(),
        "default NoDismiss should retain the overlay"
    );
}

/// An explicit `on_blur` handler returning [`Dismissal::Retain`] keeps the
/// overlay active
#[test]
fn blur_retains_overlay_via_on_blur_retain() {
    let state = State::default().with_popover();
    let view = |state: &State| {
        Button::new(|s: &mut State| s.main_tapped = true, |_| Rectangle)
            .popover(state.popover_visible.as_ref(), |()| {
                VStack::new((
                    Rectangle,
                    Button::new(|s: &mut State| s.popover_a_tapped = true, |_| Circle)
                        .frame_sized(50, 50),
                ))
            })
            .on_blur(|_s: &mut State| Dismissal::Retain)
    };
    let mut harness = App::new(state, Size::new(100, 100), view).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    let result = harness.blur();
    assert!(result.is_handled());
    assert!(
        matches!(result.shape(), Some(ContentShape::Empty)),
        "Retained blur should report an empty content shape, got {:?}",
        result.shape()
    );
    assert!(
        harness.state().popover_visible.is_some(),
        "Retain should keep the overlay mounted"
    );
}
