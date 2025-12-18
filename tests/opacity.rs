use core::time::Duration;

use embedded_touch::{Tool, Touch};

use buoyant::{
    environment::DefaultEnvironment,
    event::Event,
    primitives::{Point, Size},
    view::prelude::*,
};

#[test]
fn nonzero_opacity_hands_off_event() {
    let view = Button::new(|x: &mut u32| *x += 1, |_| Rectangle).opacity(1);
    let mut x = 0;
    let mut state = view.build_state(&mut x);
    let layout = view.layout(
        &Size::new(100, 20).into(),
        &DefaultEnvironment::default(),
        &mut x,
        &mut state,
    );
    let mut tree = view.render_tree(
        &layout,
        Point::zero(),
        &DefaultEnvironment::default(),
        &mut x,
        &mut state,
    );
    let input = buoyant::event::input::Input::new();

    view.handle_event(
        &Event::Touch(Touch::new(
            0,
            Point::zero().into(),
            embedded_touch::Phase::Started,
            Tool::Finger,
        )),
        &buoyant::event::EventContext::new(Duration::ZERO, &input),
        &mut tree,
        &mut x,
        &mut state,
    );
    view.handle_event(
        &Event::Touch(Touch::new(
            0,
            Point::zero().into(),
            embedded_touch::Phase::Ended,
            Tool::Finger,
        )),
        &buoyant::event::EventContext::new(Duration::ZERO, &input),
        &mut tree,
        &mut x,
        &mut state,
    );

    assert_eq!(x, 1);
}

#[test]
fn zero_opacity_skips_event_handling() {
    let view = Button::new(|x: &mut u32| *x += 1, |_| Rectangle).opacity(0);
    let mut x = 0;
    let mut state = view.build_state(&mut x);
    let layout = view.layout(
        &Size::new(100, 20).into(),
        &DefaultEnvironment::default(),
        &mut x,
        &mut state,
    );
    let mut tree = view.render_tree(
        &layout,
        Point::zero(),
        &DefaultEnvironment::default(),
        &mut x,
        &mut state,
    );

    let input = buoyant::event::input::Input::new();

    view.handle_event(
        &Event::Touch(Touch::new(
            0,
            Point::zero().into(),
            embedded_touch::Phase::Started,
            Tool::Finger,
        )),
        &buoyant::event::EventContext::new(Duration::ZERO, &input),
        &mut tree,
        &mut x,
        &mut state,
    );
    view.handle_event(
        &Event::Touch(Touch::new(
            0,
            Point::zero().into(),
            embedded_touch::Phase::Ended,
            Tool::Finger,
        )),
        &buoyant::event::EventContext::new(Duration::ZERO, &input),
        &mut tree,
        &mut x,
        &mut state,
    );

    assert_eq!(x, 0);
}
