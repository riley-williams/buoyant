use buoyant::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Alignment, LayoutDirection},
    pixel::PixelColor,
    style::color_style::ColorStyle,
};

pub struct TestEnv<Color> {
    pub direction: LayoutDirection,
    pub alignment: Alignment,
    pub foreground_style: Color,
}

impl<Color> LayoutEnvironment for TestEnv<Color> {
    fn layout_direction(&self) -> LayoutDirection {
        self.direction
    }

    fn alignment(&self) -> Alignment {
        self.alignment
    }
}

impl<Color: PixelColor> RenderEnvironment<Color> for TestEnv<Color> {
    fn foreground_style(&self) -> impl ColorStyle<Color = Color> {
        self.foreground_style
    }
}

impl<C: Default> Default for TestEnv<C> {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Horizontal,
            alignment: Alignment::default(),
            foreground_style: C::default(),
        }
    }
}

impl TestEnv<()> {
    pub fn colorless() -> Self {
        Self {
            direction: LayoutDirection::Horizontal,
            alignment: Alignment::default(),
            foreground_style: (),
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
