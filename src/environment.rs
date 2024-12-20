use crate::layout::{Alignment, LayoutDirection};

pub trait LayoutEnvironment {
    fn layout_direction(&self) -> LayoutDirection;
    fn alignment(&self) -> Alignment;
}

pub trait RenderEnvironment: LayoutEnvironment {
    type Color;
    fn foreground_color(&self) -> Self::Color;
}

pub struct DefaultEnvironment<Color> {
    foreground_color: Color,
}

impl<Color: Copy> DefaultEnvironment<Color> {
    pub fn new(foreground_color: Color) -> Self {
        Self { foreground_color }
    }
}

impl<Color> LayoutEnvironment for DefaultEnvironment<Color> {
    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::default()
    }

    fn alignment(&self) -> Alignment {
        Alignment::default()
    }
}

impl<C: Copy> RenderEnvironment for DefaultEnvironment<C> {
    type Color = C;
    fn foreground_color(&self) -> C {
        self.foreground_color
    }
}

#[cfg(test)]
pub(crate) mod mock {
    use super::*;

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

    impl<C: Copy + PartialEq + Default> Default for TestEnv<C> {
        fn default() -> Self {
            Self {
                direction: LayoutDirection::Horizontal,
                alignment: Alignment::default(),
                foreground_color: C::default(),
            }
        }
    }

    impl<C> TestEnv<C> {
        pub fn with_direction(mut self, direction: LayoutDirection) -> Self {
            self.direction = direction;
            self
        }
    }
}
