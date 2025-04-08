use std::time::Duration;

use buoyant::{
    environment::LayoutEnvironment,
    layout::{Alignment, LayoutDirection},
    render_target::FixedTextBuffer,
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
