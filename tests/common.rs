use buoyant::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Alignment, LayoutDirection},
    style::color_style::ColorStyle,
};

pub struct TestEnv {
    pub direction: LayoutDirection,
    pub alignment: Alignment,
    pub foreground_style: char,
}

impl LayoutEnvironment for TestEnv {
    fn layout_direction(&self) -> LayoutDirection {
        self.direction
    }

    fn alignment(&self) -> Alignment {
        self.alignment
    }
}

impl RenderEnvironment<char> for TestEnv {
    fn foreground_style(&self) -> impl ColorStyle<Color = char> {
        self.foreground_style
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Horizontal,
            alignment: Alignment::default(),
            foreground_style: ' ',
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
