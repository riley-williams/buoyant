use crate::layout::{Alignment, LayoutDirection};

pub trait Environment {
    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::default()
    }
    fn alignment(&self) -> Alignment {
        Alignment::default()
    }
}
