use core::time::Duration;

use crate::{
    event::input::{Input, InputRef},
    layout::LayoutDirection,
};

pub trait LayoutEnvironment {
    fn layout_direction(&self) -> LayoutDirection;
    /// The duration since the application started.
    /// This is used to drive animations.
    fn app_time(&self) -> Duration;
    fn input(&self) -> InputRef<'_>;
}

#[derive(Default, Debug, Clone, Copy)]
pub struct DefaultEnvironment<'a> {
    pub app_time: Duration,
    pub input: InputRef<'a>,
}

impl<'a> DefaultEnvironment<'a> {
    #[must_use]
    pub const fn new(app_time: Duration) -> Self {
        Self {
            app_time,
            input: InputRef::DUMMY,
        }
    }

    #[must_use]
    pub const fn input(self, input: &'a Input<'a>) -> Self {
        let input = input.as_ref();
        Self { input, ..self }
    }

    // Input should always be provided, `as_drawable` is probably fine to not blur subtrees?
    #[must_use]
    pub const fn non_animated() -> Self {
        Self {
            app_time: Duration::new(0, 0),
            input: InputRef::DUMMY,
        }
    }
}

impl LayoutEnvironment for DefaultEnvironment<'_> {
    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::default()
    }

    fn app_time(&self) -> Duration {
        self.app_time
    }

    fn input(&self) -> InputRef<'_> {
        self.input
    }
}
