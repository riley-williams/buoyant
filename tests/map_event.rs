use buoyant::{
    app::{App, Harness},
    event::{Event, EventResult, Key},
    focus::{FocusAction, FocusGroup},
    primitives::{Point, Size, geometry},
    view::{map_event::Mapping, prelude::*},
};
use embedded_graphics::pixelcolor::Rgb888;

#[derive(Clone, Default, PartialEq, Eq, Debug)]
struct State {
    taps: u8,
    location: Point,
}

fn arrow_pointer() -> impl View<Rgb888, State> + use<> {
    Circle.map_event(|event, state: &mut State| match event {
        Event::KeyDown { key, .. } => {
            match key {
                Key::UpArrow => state.location.y += 1,
                Key::DownArrow => state.location.y -= 1,
                Key::LeftArrow => state.location.x -= 1,
                Key::RightArrow => state.location.x += 1,
                _ => (),
            }

            if geometry::Rectangle::new(Point::new(-2, -2), Size::new(5, 5))
                .contains(&state.location)
            {
                // "handle" events in the rect
                Mapping::Handled
            } else {
                Mapping::Defer
            }
        }
        Event::Focus { action, .. } => match action {
            FocusAction::Focus(_) => Mapping::Handled,
            _ => Mapping::Defer,
        },
        _ => Mapping::Passthrough,
    })
}

fn view(_s: &State) -> impl View<Rgb888, State> + use<> {
    VStack::new((
        Button::new(|s: &mut State| s.taps += 1, |_| Rectangle),
        arrow_pointer(),
    ))
    .map_event(|event, _: &mut State| match event {
        Event::KeyDown { key, .. } => match key {
            Key::DownArrow | Key::RightArrow => Mapping::Fallback(Event::Focus {
                action: FocusAction::Next,
                group: FocusGroup::default(),
            }),
            Key::UpArrow | Key::LeftArrow => Mapping::Fallback(Event::Focus {
                action: FocusAction::Previous,
                group: FocusGroup::default(),
            }),
            _ => Mapping::Passthrough,
        },
        _ => Mapping::Passthrough,
    })
}

#[test]
fn map_events() {
    let mut app = App::new(State::default(), Size::new(10, 10), view);
    app.focus_forward();
    app.select();
    assert_eq!(app.state().taps, 1);
    // move focus off button
    app.key_down(Key::DownArrow);

    app.key_down(Key::DownArrow);
    app.key_down(Key::RightArrow);
    app.key_down(Key::RightArrow);
    assert_eq!(app.state().location, Point::new(2, -1));

    // should do nothing, second view still focused
    app.select();
    assert_eq!(app.state().taps, 1);

    // move focus to edge of valid rect
    app.key_down(Key::UpArrow);
    app.key_down(Key::UpArrow);
    app.key_down(Key::UpArrow);
    assert_eq!(app.state().location, Point::new(2, 2));

    // should do nothing, second view still focused
    app.select();
    assert_eq!(app.state().taps, 1);

    // moves outside rect, should fallback to outer focus movement
    app.key_down(Key::UpArrow);
    app.select();
    assert_eq!(app.state().taps, 2);
}

fn replace_view(_s: &State) -> impl View<Rgb888, State> + use<> {
    Button::new(|s: &mut State| s.taps += 1, |_| Rectangle).map_event(|event, _: &mut State| {
        match event {
            Event::KeyDown {
                key: Key::Character('\n'),
                ..
            } => Mapping::Replace(Event::from(FocusAction::Select)),
            _ => Mapping::Passthrough,
        }
    })
}

#[test]
fn replace_delivers_mapped_event() {
    let mut app = App::new(State::default(), Size::new(10, 10), replace_view);
    app.focus_forward();
    // The mapped '\n' is replaced with Select, activating the focused button.
    app.key_down(Key::Character('\n'));
    assert_eq!(app.state().taps, 1);
    // An unmapped key passes through; the button does not handle it.
    app.key_down(Key::UpArrow);
    assert_eq!(app.state().taps, 1);
}

fn defer_view(_s: &State) -> impl View<Rgb888, State> + use<> {
    Button::new(|s: &mut State| s.taps += 1, |_| Rectangle).map_event(|event, _: &mut State| {
        match event {
            Event::Focus {
                action: FocusAction::Select,
                ..
            } => Mapping::Defer,
            _ => Mapping::Passthrough,
        }
    })
}

#[test]
fn defer_withholds_event_from_view() {
    let mut app = App::new(State::default(), Size::new(10, 10), defer_view);
    app.focus_forward();
    // Select is deferred: it never reaches the button and the result is Deferred.
    let result = app.select();
    assert_eq!(result, EventResult::Deferred);
    assert_eq!(app.state().taps, 0);
}

/// Focus events are reported as handled. You can never leave.
fn evil_focus_black_hole(_s: &State) -> impl View<Rgb888, State> + use<> {
    Button::new(|s: &mut State| s.taps += 1, |_| Rectangle).map_event(|event, _: &mut State| {
        match event {
            Event::Focus {
                action: FocusAction::Select,
                ..
            } => Mapping::Handled,
            _ => Mapping::Passthrough,
        }
    })
}

#[test]
fn handled_reports_handled_without_reaching_view() {
    let mut app = App::new(State::default(), Size::new(10, 10), evil_focus_black_hole);
    app.focus_forward();
    // Select is reported as handled but never reaches the button.
    let result = app.select();
    assert!(result.is_handled());
    assert_eq!(app.state().taps, 0);
}

fn stateful_view(_s: &State) -> impl View<Rgb888, State> + use<> {
    Rectangle.map_stateful_event(|event, state: &mut State, count: &mut u8| match event {
        Event::KeyDown { .. } => {
            *count += 1;
            state.taps = *count;
            Mapping::Handled
        }
        _ => Mapping::Passthrough,
    })
}
/// Verifies that the private persisted state `I` accumulates across events
/// rather than resetting.
#[test]
fn map_stateful_event_persists_internal_state() {
    let mut app = App::new(State::default(), Size::new(10, 10), stateful_view);
    app.focus_forward();
    app.key_down(Key::UpArrow);
    assert_eq!(app.state().taps, 1);
    app.key_down(Key::DownArrow);
    assert_eq!(app.state().taps, 2);
    app.key_down(Key::LeftArrow);
    assert_eq!(app.state().taps, 3);
}
