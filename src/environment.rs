use crate::{
    layout::{Alignment, LayoutDirection},
    pixel::ColorValue,
    style::color_style::ColorStyle,
};

pub trait LayoutEnvironment {
    fn layout_direction(&self) -> LayoutDirection;
    fn alignment(&self) -> Alignment;
}

pub trait RenderEnvironment<C: ColorValue>: LayoutEnvironment {
    fn foreground_style(&self) -> impl ColorStyle<Color = C>;
}

pub struct DefaultEnvironment<Color: ColorValue> {
    foreground_color: Color,
}

impl<Color: ColorValue> DefaultEnvironment<Color> {
    pub fn new(foreground_color: Color) -> Self {
        Self { foreground_color }
    }
}

impl<Color: ColorValue> LayoutEnvironment for DefaultEnvironment<Color> {
    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::default()
    }

    fn alignment(&self) -> Alignment {
        Alignment::default()
    }
}

impl<C: ColorValue> RenderEnvironment<C> for DefaultEnvironment<C> {
    fn foreground_style(&self) -> impl ColorStyle<Color = C> {
        self.foreground_color
    }
}

#[cfg(test)]
pub(crate) mod mock {
    use crate::style::color_style::ColorStyle;

    use super::*;

    pub struct TestEnv<Color: ColorValue> {
        pub direction: LayoutDirection,
        pub alignment: Alignment,
        pub foreground_style: Color,
    }

    impl<Color: ColorValue> LayoutEnvironment for TestEnv<Color> {
        fn layout_direction(&self) -> LayoutDirection {
            self.direction
        }

        fn alignment(&self) -> Alignment {
            self.alignment
        }
    }

    impl<Color: ColorValue> RenderEnvironment<Color> for TestEnv<Color> {
        fn foreground_style(&self) -> impl ColorStyle<Color = Color> {
            self.foreground_style
        }
    }

    impl<C: ColorValue + Default> Default for TestEnv<C> {
        fn default() -> Self {
            Self {
                direction: LayoutDirection::Horizontal,
                alignment: Alignment::default(),
                foreground_style: C::default(),
            }
        }
    }

    impl<C: ColorValue> TestEnv<C> {
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
