use std::time::Duration;

use buoyant::{
    environment::{DefaultEnvironment, LayoutEnvironment},
    layout::{Alignment, LayoutDirection},
    primitives::{Point, Size},
    render_target::FixedTextBuffer,
    view::View,
};

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
pub fn make_render_tree<C, V>(view: &V, size: Size) -> V::Renderables
where
    V: View<C>,
{
    let env = DefaultEnvironment::default();
    let layout = view.layout(&size.into(), &env);
    view.render_tree(&layout, Point::zero(), &env)
}
