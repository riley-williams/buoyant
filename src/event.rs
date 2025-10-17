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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EventResult {
    /// Whether the event was handled by the view.
    pub handled: bool,
    /// Whether the view should be recomputed, and render trees joined.
    pub recompute_view: bool,
}

impl EventResult {
    /// Creates a new `EventResult` with the specified handled state and recompute flag.
    #[must_use]
    pub const fn new(handled: bool, recompute_view: bool) -> Self {
        Self {
            handled,
            recompute_view,
        }
    }

    /// merges another `EventResult` into this one.
    #[expect(clippy::needless_pass_by_value)]
    pub fn merge(&mut self, other: Self) {
        self.handled |= other.handled;
        self.recompute_view |= other.recompute_view;
    }

    /// Returns the result of merging another `EventResult` into this one.
    #[must_use]
    #[expect(clippy::needless_pass_by_value)]
    pub fn merging(self, other: Self) -> Self {
        Self {
            handled: self.handled || other.handled,
            recompute_view: self.recompute_view || other.recompute_view,
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
    use crate::primitives::Point;

    use super::Event;
    use embedded_graphics_simulator::SimulatorEvent;
    use embedded_touch::{Phase, PointerButton, Tool, Touch, TouchPoint};

    #[derive(Debug, Default)]
    pub struct MouseTracker {
        touch: Option<Touch>,
    }

    impl MouseTracker {
        #[must_use]
        pub fn new() -> Self {
            Self { touch: None }
        }

        pub fn process_event(&mut self, event: SimulatorEvent) -> Option<Event> {
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
                    Some(Event::Touch(touch))
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
                    Some(Event::Touch(touch))
                }
                SimulatorEvent::MouseMove { point } => {
                    if let Some(touch) = &mut self.touch {
                        touch.location = TouchPoint::new(point.x, point.y);
                        touch.phase = Phase::Moved;
                        Some(Event::Touch(touch.clone()))
                    } else {
                        let touch = Touch {
                            id: 0,
                            location: TouchPoint::new(point.x, point.y),
                            phase: Phase::Hovering(None),
                            tool: Tool::Pointer {
                                button: PointerButton::None,
                            },
                        };

                        Some(Event::Touch(touch))
                    }
                }
                SimulatorEvent::MouseWheel {
                    scroll_delta,
                    direction,
                } => {
                    if direction == embedded_graphics_simulator::sdl2::MouseWheelDirection::Flipped
                    {
                        Some(Event::Scroll(Point::new(
                            scroll_delta.x * 4,
                            scroll_delta.y * 4,
                        )))
                    } else {
                        Some(Event::Scroll(Point::new(
                            -scroll_delta.x * 4,
                            -scroll_delta.y * 4,
                        )))
                    }
                }
                SimulatorEvent::Quit => Some(Event::Exit),
                SimulatorEvent::KeyDown { .. } | SimulatorEvent::KeyUp { .. } => None,
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
