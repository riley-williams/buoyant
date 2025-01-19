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
