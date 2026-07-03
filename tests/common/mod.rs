pub mod helpers;

use std::time::Duration;

use buoyant::{
    environment::{DefaultEnvironment, LayoutEnvironment},
    focus::DefaultFocus,
    layout::{Alignment, LayoutDirection},
    primitives::{Point, ProposedDimensions, Size},
    render::AnimatedJoin,
    render_target::FixedTextBuffer,
    view::View,
};
use embedded_touch::{Phase, PointerButton, Tool, Touch, TouchPoint};

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
    view.render_tree(
        &layout.sublayouts,
        Point::zero(),
        &env,
        captures,
        &mut state,
    )
}

#[allow(dead_code)]
pub fn touch_down(x: i32, y: i32) -> Touch {
    Touch {
        id: 0,
        location: TouchPoint::new(x, y),
        phase: Phase::Started,
        tool: Tool::Pointer {
            button: PointerButton::Primary,
        },
    }
}

#[allow(dead_code)]
pub fn touch_up(x: i32, y: i32) -> Touch {
    Touch {
        id: 0,
        location: TouchPoint::new(x, y),
        phase: Phase::Ended,
        tool: Tool::Pointer {
            button: PointerButton::Primary,
        },
    }
}

#[allow(dead_code)]
pub fn touch_move(x: i32, y: i32) -> Touch {
    Touch {
        id: 0,
        location: TouchPoint::new(x, y),
        phase: Phase::Moved,
        tool: Tool::Pointer {
            button: PointerButton::Primary,
        },
    }
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
        &layout.sublayouts,
        Point::zero(),
        &DefaultEnvironment::default(),
        captures,
        state,
    );

    view.handle_touch(
        &Touch::new(
            0,
            Point::new(x, y).into(),
            embedded_touch::Phase::Started,
            Tool::Finger,
        ),
        &buoyant::event::EventContext::new(Duration::ZERO),
        &mut tree,
        captures,
        state,
    );

    view.handle_touch(
        &Touch::new(
            0,
            Point::new(x, y).into(),
            embedded_touch::Phase::Ended,
            Tool::Finger,
        ),
        &buoyant::event::EventContext::new(Duration::ZERO),
        &mut tree,
        captures,
        state,
    );
}

/// Builds an [`buoyant::app::App`] over a `10x10` `char` display, routing the view function
/// through a generic boundary so the closure's higher-ranked `Fn(&S) -> V` signature is
/// resolved correctly. The app is configured with [`buoyant::focus::Role::Button`].
#[allow(dead_code)]
pub fn app<V, S, F>(state: S, view_fn: F) -> buoyant::app::App<V, S, F>
where
    V: View<char, S>,
    V::FocusTree: DefaultFocus,
    V::Renderables: AnimatedJoin,
    S: 'static,
    F: Fn(&S) -> V,
{
    buoyant::app::App::new(state, Size::new(10, 10), view_fn)
        .with_roles(buoyant::focus::Role::Button)
}
