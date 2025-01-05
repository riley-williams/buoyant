use buoyant::{
    environment::LayoutEnvironment,
    layout::{Alignment, LayoutDirection},
    render_target::{FixedTextBuffer, TxtColor},
};

pub struct TestEnv {
    pub direction: LayoutDirection,
    pub alignment: Alignment,
    pub foreground_color: TxtColor,
}

impl LayoutEnvironment for TestEnv {
    fn layout_direction(&self) -> LayoutDirection {
        self.direction
    }

    fn alignment(&self) -> Alignment {
        self.alignment
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Horizontal,
            alignment: Alignment::default(),
            foreground_color: TxtColor::default(),
        }
    }
}

impl TestEnv {
    pub fn with_direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

pub fn collect_text<const W: usize, const H: usize>(buffer: &FixedTextBuffer<W, H>) -> String {
    buffer
        .text
        .iter()
        .map(|chars| chars.iter().collect::<String>())
        .collect::<String>()
}
