use buoyant::{
    app::{App, Harness},
    focus::{BoundaryBehavior, Role},
    font::CharacterBufferFont,
    primitives::Point,
    render_target::FixedTextBuffer,
    view::{prelude::*, rotary::RotaryState},
};

use buoyant::view::rotary::{Rotary, RotaryEvent};

use crate::assert_str_grid_eq;

#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "This is just what the harness wants"
)]
fn rotary_view(count: &u32) -> impl View<char, u32> + use<> {
    let count = *count;
    HStack::new((
        Rotary::new(
            |state: &mut u32, event: RotaryEvent| match event {
                RotaryEvent::Next => *state += 1,
                RotaryEvent::Previous => *state -= 1,
                RotaryEvent::Exit => *state = 0,
                RotaryEvent::Select => {}
            },
            move |state| {
                let c = match state {
                    RotaryState::UnFocused => '-',
                    RotaryState::Focused => '~',
                    RotaryState::Captive => '|',
                };
                Text::new_fmt::<12>(format_args!("{c}{count}{c}"), &CharacterBufferFont)
                // .content_shape(RoundedRectangle::new(2))
            },
        )
        .priority(1),
        Button::new(|_: &mut u32| {}, |_| Rectangle).foreground_color('.'),
    ))
    .bound_focus(BoundaryBehavior::Wrap)
    .focus_touches()
}

#[test]
fn actions_and_state() {
    let mut target = FixedTextBuffer::<6, 1>::default();
    let mut harness = App::new(0, target.size(), rotary_view).with_roles(Role::Button);

    harness.focus_forward();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["~0~...",], &target.text);

    harness.next();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["-0-...",], &target.text);

    harness.next();
    harness.select();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["|0|...",], &target.text);

    (0..10).for_each(|_| _ = harness.next());
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["|10|..",], &target.text);

    harness.select();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["~10~..",], &target.text);

    harness.select();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["|10|..",], &target.text);

    harness.blur();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["~0~...",], &target.text);
}

#[test]
fn touch_moves_focus() {
    let mut target = FixedTextBuffer::<6, 1>::default();
    let mut harness = App::new(0, target.size(), rotary_view).with_roles(Role::Button);

    // Initial focus on right ... button
    harness.next();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["-0-...",], &target.text);

    harness.tap(Point::new(1, 0));
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["~0~...",], &target.text);

    // Focus moves away when not captive
    harness.tap(Point::new(5, 0));
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["-0-...",], &target.text);

    harness.tap(Point::new(1, 0));
    harness.select();
    harness.next();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["|1|...",], &target.text);

    // Focus moves away when captive, and triggers Exit action
    harness.tap(Point::new(5, 0));
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["-0-...",], &target.text);
}

#[test]
fn tap_on_captive_doesnt_exit() {
    let mut target = FixedTextBuffer::<6, 1>::default();
    let mut harness = App::new(0, target.size(), rotary_view).with_roles(Role::Button);

    harness.select();
    harness.next();
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["|1|...",], &target.text);

    // Exit should not trigger
    harness.tap(Point::new(2, 0));
    harness.render_animated(&mut target, &' ');
    assert_str_grid_eq!(["|1|...",], &target.text);
}
