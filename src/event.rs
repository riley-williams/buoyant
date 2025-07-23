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
