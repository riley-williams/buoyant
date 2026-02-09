use embedded_touch::{Tool, Touch};

use crate::{
    event::{Event, EventResult},
    focus::{FocusAction, FocusDirection, FocusGroup},
    primitives::Point,
};

/// [`Harness`] provides a convenience interface for sending events
pub trait Harness {
    /// Sends an event and returns the result.
    fn send(&mut self, event: impl Into<Event>) -> EventResult;

    /// Acquires focus, searching forward.
    ///
    /// The event is sent with [`CommonGroup`](FocusGroup::common_group())
    fn focus_forward(&mut self) -> EventResult {
        self.send(FocusAction::Focus(FocusDirection::Forward))
    }

    /// Acquires focus searching forward in the specified group.
    fn focus_forward_group(&mut self, group: FocusGroup) -> EventResult {
        self.send(FocusAction::Focus(FocusDirection::Forward).into_event(group))
    }

    /// Acquires focus, searching backward.
    ///
    /// The event is sent with [`CommonGroup`](FocusGroup::common_group())
    fn focus_backward(&mut self) -> EventResult {
        self.send(FocusAction::Focus(FocusDirection::Backward))
    }

    /// Acquires focus searching backward in the specified group.
    fn focus_backward_group(&mut self, group: FocusGroup) -> EventResult {
        self.send(FocusAction::Focus(FocusDirection::Backward).into_event(group))
    }

    /// Moves focus to the next element.
    ///
    /// The event is sent with [`CommonGroup`](FocusGroup::common_group())
    fn next(&mut self) -> EventResult {
        self.send(FocusAction::Next)
    }

    /// Moves focus to the next element in the specified group.
    fn next_group(&mut self, group: FocusGroup) -> EventResult {
        self.send(FocusAction::Next.into_event(group))
    }

    /// Moves focus to the previous element.
    ///
    /// The event is sent with [`CommonGroup`](FocusGroup::common_group())
    fn previous(&mut self) -> EventResult {
        self.send(FocusAction::Previous)
    }

    /// Moves focus to the previous element in the specified group.
    fn previous_group(&mut self, group: FocusGroup) -> EventResult {
        self.send(FocusAction::Previous.into_event(group))
    }

    /// Activates the currently focused element.
    ///
    /// The event is sent with [`CommonGroup`](FocusGroup::common_group())
    fn select(&mut self) -> EventResult {
        self.send(FocusAction::Select)
    }

    /// Activates the currently focused element in the specified group.
    fn select_group(&mut self, group: FocusGroup) -> EventResult {
        self.send(FocusAction::Select.into_event(group))
    }

    /// Blurs (exits) the current focus.
    ///
    /// The event is sent with [`CommonGroup`](FocusGroup::common_group())
    fn blur(&mut self) -> EventResult {
        self.send(FocusAction::Blur)
    }

    /// Blurs (exits) the current focus in the specified group.
    fn blur_group(&mut self, group: FocusGroup) -> EventResult {
        self.send(FocusAction::Blur.into_event(group))
    }

    /// Sends a tap (touch down + touch up) at the given point.
    ///
    /// The result of the touch up event is returned.
    fn tap(&mut self, point: Point) -> EventResult {
        self.send(Event::Touch(Touch::new(
            0,
            point.into(),
            embedded_touch::Phase::Started,
            Tool::Finger,
        )));

        self.send(Event::Touch(Touch::new(
            0,
            point.into(),
            embedded_touch::Phase::Ended,
            Tool::Finger,
        )))
    }

    /// Sends a drag (down + move + up) from the start point to the end point.
    ///
    /// The result of the touch up event is returned.
    fn drag(&mut self, start: Point, end: Point) -> EventResult {
        self.send(Event::Touch(Touch::new(
            0,
            start.into(),
            embedded_touch::Phase::Started,
            Tool::Finger,
        )));

        self.send(Event::Touch(Touch::new(
            0,
            end.into(),
            embedded_touch::Phase::Moved,
            Tool::Finger,
        )));

        self.send(Event::Touch(Touch::new(
            0,
            end.into(),
            embedded_touch::Phase::Ended,
            Tool::Finger,
        )))
    }

    /// Sends a touch down event at the given point.
    ///
    /// Prefer [`Self::tap()`] or [`Self::drag()`] when possible to avoid leaving
    /// the view in an inconsistent state.
    fn touch_down(&mut self, point: Point) -> EventResult {
        self.send(Event::Touch(Touch::new(
            0,
            point.into(),
            embedded_touch::Phase::Started,
            Tool::Finger,
        )))
    }

    /// Sends a touch move event at the given point.
    ///
    /// Prefer [`Self::drag()`] when possible to avoid leaving the view in an
    /// inconsistent state.
    fn touch_move(&mut self, point: Point) -> EventResult {
        self.send(Event::Touch(Touch::new(
            0,
            point.into(),
            embedded_touch::Phase::Moved,
            Tool::Finger,
        )))
    }

    /// Sends a touch up event at the given point.
    ///
    /// Prefer [`Self::tap()`] or [`Self::drag()`] when possible to avoid leaving
    /// the view in an inconsistent state.
    fn touch_up(&mut self, point: Point) -> EventResult {
        self.send(Event::Touch(Touch::new(
            0,
            point.into(),
            embedded_touch::Phase::Ended,
            Tool::Finger,
        )))
    }
}
