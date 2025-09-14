pub mod helpers;

use std::time::Duration;

use buoyant::{
    environment::{DefaultEnvironment, LayoutEnvironment},
    event::Event,
    layout::{Alignment, LayoutDirection},
    primitives::{Point, ProposedDimensions, Size},
    render_target::FixedTextBuffer,
    view::View,
};
use embedded_touch::{Phase, PointerButton, Tool, Touch};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TestEnv {
    pub direction: LayoutDirection,
    pub alignment: Alignment,
    pub foreground_color: char,
    pub app_time: Duration,
}

impl LayoutEnvironment for TestEnv {
    fn layout_direction(&self) -> LayoutDirection {
        self.direction
    }

    fn app_time(&self) -> Duration {
        self.app_time
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Horizontal,
            alignment: Alignment::default(),
            foreground_color: 'x',
            app_time: Duration::default(),
        }
    }
}

#[allow(dead_code)]
impl TestEnv {
    #[must_use]
    pub fn with_direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    #[must_use]
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

#[allow(dead_code)]
#[must_use]
pub fn collect_text<const W: usize, const H: usize>(buffer: &FixedTextBuffer<W, H>) -> String {
    buffer
        .text
        .iter()
        .map(|chars| chars.iter().collect::<String>())
        .collect::<String>()
}

#[allow(dead_code)]
#[must_use]
pub fn make_render_tree<Color: Copy, Captures: ?Sized, V>(
    view: &V,
    size: Size,
    captures: &mut Captures,
) -> V::Renderables
where
    V: View<Color, Captures>,
{
    let env = DefaultEnvironment::default();
    let mut state = view.build_state(captures);
    let layout = view.layout(&size.into(), &env, captures, &mut state);
    view.render_tree(&layout, Point::zero(), &env, captures, &mut state)
}

#[allow(dead_code)]
pub fn touch_down(p: Point) -> Event {
    Event::Touch(Touch {
        id: 0,
        location: p.into(),
        phase: Phase::Started,
        tool: Tool::Pointer {
            button: PointerButton::Primary,
        },
    })
}

#[allow(dead_code)]
pub fn touch_up(p: Point) -> Event {
    Event::Touch(Touch {
        id: 0,
        location: p.into(),
        phase: Phase::Ended,
        tool: Tool::Pointer {
            button: PointerButton::Primary,
        },
    })
}

#[allow(dead_code)]
pub fn touch_move(p: Point) -> Event {
    Event::Touch(Touch {
        id: 0,
        location: p.into(),
        phase: Phase::Moved,
        tool: Tool::Pointer {
            button: PointerButton::Primary,
        },
    })
}

/// Tap at the given coordinates on the view.
#[allow(dead_code)]
pub fn tap<V: View<char, Data>, Data: ?Sized>(
    view: &V,
    captures: &mut Data,
    state: &mut V::State,
    size: impl Into<ProposedDimensions>,
    x: i32,
    y: i32,
) {
    let layout = view.layout(
        &size.into(),
        &DefaultEnvironment::default(),
        captures,
        state,
    );

    let mut tree = view.render_tree(
        &layout,
        Point::zero(),
        &DefaultEnvironment::default(),
        captures,
        state,
    );

    view.handle_event(
        &Event::Touch(Touch::new(
            0,
            Point::new(x, y).into(),
            embedded_touch::Phase::Started,
            Tool::Finger,
        )),
        &buoyant::event::EventContext::new(Duration::ZERO),
        &mut tree,
        captures,
        state,
    );

    view.handle_event(
        &Event::Touch(Touch::new(
            0,
            Point::new(x, y).into(),
            embedded_touch::Phase::Ended,
            Tool::Finger,
        )),
        &buoyant::event::EventContext::new(Duration::ZERO),
        &mut tree,
        captures,
        state,
    );
}
