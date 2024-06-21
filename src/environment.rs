use crate::{
    layout::{Alignment, LayoutDirection},
    primitives::Size,
};

pub trait Environment {
    fn layout_direction(&self) -> LayoutDirection;
    fn alignment(&self) -> Alignment;

    fn foreground_style(&self) -> impl ColorStyle;
    fn background_style(&self) -> impl ColorStyle;
}

pub trait ColorStyle: Clone + Copy {
    /// Shade a pixel at the given relative coordinates
    fn shade_pixel(&self, x: u16, y: u16, in_bounds: Size) -> rgb::RGB8;
}

impl ColorStyle for rgb::RGB8 {
    fn shade_pixel(&self, _: u16, _: u16, _: Size) -> rgb::RGB8 {
        *self
    }
}

pub struct DefaultEnvironment;

impl Environment for DefaultEnvironment {
    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::default()
    }

    fn alignment(&self) -> Alignment {
        Alignment::default()
    }

    fn foreground_style(&self) -> impl ColorStyle {
        rgb::RGB8::new(255, 255, 255)
    }

    fn background_style(&self) -> impl ColorStyle {
        rgb::RGB8::new(0, 0, 0)
    }
}

#[cfg(test)]
pub(crate) mod mock {
    use super::*;

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
}
