use crate::layout::{Alignment, LayoutDirection};

pub trait LayoutEnvironment {
    fn layout_direction(&self) -> LayoutDirection;
    fn alignment(&self) -> Alignment;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultEnvironment;

impl LayoutEnvironment for DefaultEnvironment {
    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::default()
    }

    fn alignment(&self) -> Alignment {
        Alignment::default()
    }
}

#[cfg(test)]
pub(crate) mod mock {
    use super::*;

    pub struct TestEnv {
        pub direction: LayoutDirection,
        pub alignment: Alignment,
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
            }
        }
    }

    impl TestEnv {
        pub fn with_direction(mut self, direction: LayoutDirection) -> Self {
            self.direction = direction;
            self
        }
    }
}
