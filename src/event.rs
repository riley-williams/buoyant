use core::time::Duration;

use crate::primitives::Point;

/// An interaction event that can be handled by a view.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Touch(embedded_touch::Touch),
    /// A scroll event with the given offset.
    Scroll(Point),
    /// External state changed which may affect the view.
    ///
    /// The view should be recomputed in response to this event.
    External,
    /// The app should exit
    Exit,
}

impl Event {
    /// Returns a new event with the specified offset applied to any point-based data.
    #[must_use]
    pub fn offset(&self, offset: Point) -> Self {
        let mut event = self.clone();
        match &mut event {
            Self::Touch(touch) => {
                touch.location += offset.into();
            }
            Self::Scroll(_) | Self::External | Self::Exit => {}
        }
        event
    }
}

// FIXME: convert these flags to use a bitmask

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EventResult {
    /// Whether the event was handled by the view.
    pub handled: bool,
    /// Whether the view should be recomputed, and render trees joined.
    pub recompute_view: bool,
    /// This flag indicates the view should be redrawn even if no animations were reported as
    /// active.
    ///
    /// This should be set when a view directly modifies the render tree state
    /// without requesting a view recompute, e.g. scrollview dragging.
    pub redraw: bool,
}

impl EventResult {
    /// Creates a new `EventResult` with the specified handled state and recompute flag.
    #[must_use]
    pub const fn new(handled: bool, recompute_view: bool, redraw: bool) -> Self {
        Self {
            handled,
            recompute_view,
            redraw,
        }
    }

    /// merges another `EventResult` into this one.
    #[expect(clippy::needless_pass_by_value)]
    pub fn merge(&mut self, other: Self) {
        self.handled |= other.handled;
        self.recompute_view |= other.recompute_view;
        self.redraw |= other.redraw;
    }

    /// Returns the result of merging another `EventResult` into this one.
    #[must_use]
    #[expect(clippy::needless_pass_by_value)]
    pub fn merging(self, other: Self) -> Self {
        Self {
            handled: self.handled || other.handled,
            recompute_view: self.recompute_view || other.recompute_view,
            redraw: self.redraw || other.redraw,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventContext {
    pub app_time: Duration,
}

impl EventContext {
    /// Creates a new `EventContext` with the given application time.
    #[must_use]
    pub const fn new(app_time: Duration) -> Self {
        Self { app_time }
    }
}

#[cfg(feature = "embedded-graphics-simulator")]
pub mod simulator {
    use crate::{
        focus::{FocusAction, FocusEvent},
        primitives::Point,
    };

    use super::Event;
    use embedded_graphics_simulator::{SimulatorEvent, sdl2::Keycode};
    use embedded_touch::{Phase, PointerButton, Tool, Touch, TouchPoint};

    #[derive(Debug, Default)]
    pub struct MouseTracker {
        touch: Option<Touch>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum EventType {
        Event(Event),
        Focus(FocusAction),
    }

    impl From<Event> for EventType {
        fn from(event: Event) -> Self {
            Self::Event(event)
        }
    }

    impl From<FocusAction> for EventType {
        fn from(event: FocusAction) -> Self {
            Self::Focus(event)
        }
    }

    impl MouseTracker {
        #[must_use]
        pub fn new() -> Self {
            Self { touch: None }
        }

        pub fn process_event(&mut self, event: SimulatorEvent) -> Option<EventType> {
            match event {
                SimulatorEvent::MouseButtonDown { point, mouse_btn } => {
                    let touch = Touch {
                        id: 0,
                        location: TouchPoint::new(point.x, point.y),
                        phase: Phase::Started,
                        tool: Tool::Pointer {
                            button: map_button(mouse_btn),
                        },
                    };
                    self.touch = Some(touch.clone());
                    Some(Event::Touch(touch).into())
                }
                SimulatorEvent::MouseButtonUp { point, mouse_btn } => {
                    let touch = Touch {
                        id: 0,
                        location: TouchPoint::new(point.x, point.y),
                        phase: Phase::Ended,
                        tool: Tool::Pointer {
                            button: map_button(mouse_btn),
                        },
                    };

                    self.touch = None;
                    Some(Event::Touch(touch).into())
                }
                SimulatorEvent::MouseMove { point } => {
                    if let Some(touch) = &mut self.touch {
                        touch.location = TouchPoint::new(point.x, point.y);
                        touch.phase = Phase::Moved;
                        Some(Event::Touch(touch.clone()).into())
                    } else {
                        let touch = Touch {
                            id: 0,
                            location: TouchPoint::new(point.x, point.y),
                            phase: Phase::Hovering(None),
                            tool: Tool::Pointer {
                                button: PointerButton::None,
                            },
                        };

                        Some(Event::Touch(touch).into())
                    }
                }
                SimulatorEvent::MouseWheel {
                    scroll_delta,
                    direction,
                } => {
                    if direction == embedded_graphics_simulator::sdl2::MouseWheelDirection::Flipped
                    {
                        Some(
                            Event::Scroll(Point::new(scroll_delta.x * 4, scroll_delta.y * 4))
                                .into(),
                        )
                    } else {
                        Some(
                            Event::Scroll(Point::new(-scroll_delta.x * 4, -scroll_delta.y * 4))
                                .into(),
                        )
                    }
                }
                SimulatorEvent::Quit => Some(Event::Exit.into()),
                SimulatorEvent::KeyDown { .. } => None,
                SimulatorEvent::KeyUp {
                    keycode,
                    keymod: _,
                    repeat: _,
                } => match keycode {
                    Keycode::Return | Keycode::Space => Some(FocusAction::Select.into()),
                    Keycode::Backspace => Some(FocusAction::Blur.into()),
                    Keycode::Left | Keycode::Up => Some(FocusAction::Previous.into()),
                    Keycode::Right | Keycode::Down => Some(FocusAction::Next.into()),
                    _ => None,
                },
            }
        }
    }

    fn map_button(mouse_btn: embedded_graphics_simulator::sdl2::MouseButton) -> PointerButton {
        match mouse_btn {
            embedded_graphics_simulator::sdl2::MouseButton::Left => PointerButton::Primary,
            embedded_graphics_simulator::sdl2::MouseButton::Right => PointerButton::Secondary,
            embedded_graphics_simulator::sdl2::MouseButton::Middle => PointerButton::Tertiary,
            _ => PointerButton::None,
        }
    }
}
