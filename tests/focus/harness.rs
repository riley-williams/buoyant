//! Test harness for focus tests to reduce boilerplate

use std::time::Duration;

use buoyant::{
    environment::DefaultEnvironment,
    event::{Event, EventContext, EventResult},
    focus::{
        ContentShape, DefaultFocus, FocusAction, FocusDirection, FocusEvent, FocusStateChange, Role,
    },
    primitives::{Point, Size, geometry::Rectangle},
    render::Rect,
    view::ViewLayout,
};
use embedded_touch::{Tool, Touch};

/// A test harness that simplifies focus testing by managing all the boilerplate.
#[allow(dead_code)]
pub struct FocusTestHarness<V, S>
where
    V: ViewLayout<S>,
{
    pub state: S,
    view: V,
    view_state: V::State,
    render_tree: V::Renderables,
    focus_tree: V::FocusTree,
    env: DefaultEnvironment,
    event_context: EventContext,
}

impl<V, S> FocusTestHarness<V, S>
where
    V: ViewLayout<S>,
    V::FocusTree: DefaultFocus,
{
    /// Creates a new test harness with the given view, state, and layout size.
    pub fn new(view: V, state: S, size: Size) -> Self
    where
        S: 'static,
    {
        let mut state = state;
        let env = DefaultEnvironment::non_animated();
        let mut view_state = view.build_state(&mut state);
        let layout = view.layout(&size.into(), &env, &mut state, &mut view_state);
        let render_tree = view.render_tree(
            &layout.sublayouts,
            Point::zero(),
            &env,
            &mut state,
            &mut view_state,
        );
        let focus_tree = DefaultFocus::default_first();
        let event_context = EventContext::new(Duration::default());

        Self {
            state,
            view,
            view_state,
            render_tree,
            focus_tree,
            env,
            event_context,
        }
    }

    /// Sends a focus event and returns the result.
    fn send(&mut self, action: FocusAction) -> FocusStateChange {
        self.view.focus(
            &FocusEvent::new(action, Role::Button.mask()),
            &self.event_context,
            &mut self.render_tree,
            &mut self.state,
            &mut self.view_state,
            &mut self.focus_tree,
        )
    }

    /// Acquires focus searching forward (towards the end).
    pub fn focus_forward(&mut self) -> FocusStateChange {
        self.send(FocusAction::Focus(FocusDirection::Forward))
    }

    /// Acquires focus searching backward (towards the beginning).
    #[allow(dead_code)]
    pub fn focus_backward(&mut self) -> FocusStateChange {
        self.send(FocusAction::Focus(FocusDirection::Backward))
    }

    /// Moves focus to the next element.
    pub fn next(&mut self) -> FocusStateChange {
        self.send(FocusAction::Next)
    }

    /// Moves focus to the previous element.
    pub fn previous(&mut self) -> FocusStateChange {
        self.send(FocusAction::Previous)
    }

    /// Activates the currently focused element.
    pub fn select(&mut self) -> FocusStateChange {
        self.send(FocusAction::Select)
    }

    /// Blurs (exits) the current focus.
    #[allow(dead_code)]
    pub fn blur(&mut self) -> FocusStateChange {
        self.send(FocusAction::Blur)
    }

    /// Sends a tap (touch down + touch up) at the given point.
    #[allow(dead_code)]
    pub fn tap(&mut self, point: Point) -> EventResult {
        self.view.handle_event(
            &Event::Touch(Touch::new(
                0,
                point.into(),
                embedded_touch::Phase::Started,
                Tool::Finger,
            )),
            &self.event_context,
            &mut self.render_tree,
            &mut self.state,
            &mut self.view_state,
        );

        self.view.handle_event(
            &Event::Touch(Touch::new(
                0,
                point.into(),
                embedded_touch::Phase::Ended,
                Tool::Finger,
            )),
            &self.event_context,
            &mut self.render_tree,
            &mut self.state,
            &mut self.view_state,
        )
    }
}

/// Helper to create a `FocusStateChange::Focused` with a rectangle shape.
pub fn focused_rect(rect: Rectangle) -> FocusStateChange {
    FocusStateChange::Focused {
        shape: ContentShape::Rectangle(rect),
        result: EventResult::default(),
    }
}
