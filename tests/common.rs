use buoyant::{
    environment::{ColorStyle, Environment},
    layout::{Alignment, LayoutDirection},
};

pub struct TestEnv {
    pub direction: LayoutDirection,
    pub alignment: Alignment,
    pub foreground_style: rgb::RGB8,
    pub background_style: rgb::RGB8,
}

impl Environment for TestEnv {
    fn layout_direction(&self) -> LayoutDirection {
        self.direction
    }

    fn alignment(&self) -> Alignment {
        self.alignment
    }

    fn foreground_style(&self) -> impl ColorStyle {
        self.foreground_style
    }

    fn background_style(&self) -> impl ColorStyle {
        self.background_style
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Horizontal,
            alignment: Alignment::default(),
            foreground_style: rgb::RGB8::new(255, 255, 255),
            background_style: rgb::RGB8::new(0, 0, 0),
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
