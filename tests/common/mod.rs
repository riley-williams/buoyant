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

// #[allow(dead_code)]
// pub fn collect_lines<const W: usize, const H: usize>(
//     buffer: &FixedTextBuffer<W, H>,
// ) -> Vec<String> {
//     let vec = heapless::Vec::new();
//     buffer
//         .text
//         .iter()
//         .map(|chars| {
//             chars.iter().collect::<String>();
//         })
//         .collect::<Vec<_>>();
// }

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
