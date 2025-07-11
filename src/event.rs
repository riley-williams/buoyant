use crate::primitives::Point;

/// An interaction event that can be handled by a view.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// A touch or click which started at the specified point.
    ///
    /// This event is triggered when the user begins a touch interaction
    /// or presses a mouse button at the given coordinates.
    TouchDown(Point),

    /// A touch or click ended at the specified point.
    ///
    /// This event is triggered when the user ends a touch interaction
    /// or releases a mouse button at the given coordinates.
    TouchUp(Point),

    /// A touch or mouse cursor moved to the specified point.
    ///
    /// This event is triggered when the user moves their finger during
    /// a touch interaction or moves the mouse cursor while pressed.
    TouchMoved(Point),
    Scroll(Point),
    Exit,
}

#[cfg(feature = "simulator")]
pub mod impl_eg {
    use crate::primitives::Point;

    use super::Event;
    use embedded_graphics_simulator::SimulatorEvent;

    impl TryFrom<SimulatorEvent> for Event {
        type Error = ();

        fn try_from(event: SimulatorEvent) -> Result<Self, Self::Error> {
            match event {
                SimulatorEvent::Quit => Ok(Self::Exit),
                SimulatorEvent::MouseButtonDown {
                    mouse_btn: _,
                    point,
                } => Ok(Self::TouchDown(Point::new(point.x, point.y))),
                SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point,
                } => Ok(Self::TouchUp(Point::new(point.x, point.y))),
                SimulatorEvent::MouseMove { point } => {
                    Ok(Self::TouchMoved(Point::new(point.x, point.y)))
                }
                SimulatorEvent::MouseWheel {
                    scroll_delta,
                    direction,
                } => {
                    if direction == embedded_graphics_simulator::sdl2::MouseWheelDirection::Normal {
                        Ok(Self::Scroll(Point::new(scroll_delta.x, scroll_delta.y)))
                    } else {
                        Ok(Self::Scroll(Point::new(-scroll_delta.x, -scroll_delta.y)))
                    }
                }
                SimulatorEvent::KeyDown {
                    keycode: _,
                    keymod: _,
                    repeat: _,
                }
                | SimulatorEvent::KeyUp {
                    keycode: _,
                    keymod: _,
                    repeat: _,
                } => Err(()),
            }
        }
    }
}
