use buoyant::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Alignment, LayoutDirection},
};

pub struct TestEnv<Color> {
    pub direction: LayoutDirection,
    pub alignment: Alignment,
    pub foreground_color: Color,
}

impl<Color> LayoutEnvironment for TestEnv<Color> {
    fn layout_direction(&self) -> LayoutDirection {
        self.direction
    }

    fn alignment(&self) -> Alignment {
        self.alignment
    }
}

impl<Color: Copy> RenderEnvironment for TestEnv<Color> {
    type Color = Color;
    fn foreground_color(&self) -> Color {
        self.foreground_color
    }
}

impl<C: Default> Default for TestEnv<C> {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Horizontal,
            alignment: Alignment::default(),
            foreground_color: C::default(),
        }
    }
}

impl TestEnv<()> {
    pub fn colorless() -> Self {
        Self {
            direction: LayoutDirection::Horizontal,
            alignment: Alignment::default(),
            foreground_color: (),
        }
    }
}

impl<C> TestEnv<C> {
    pub fn with_direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}
